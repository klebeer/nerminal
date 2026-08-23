use std::path::PathBuf;

use about_page::AboutPageView;
use appearance_page::{AppearancePageAction, AppearanceSettingsPageView};
use environments_page::EnvironmentsPageView;
use itertools::Itertools as _;
use keybindings::KeybindingsView;
use mcp_servers_page::MCPServersSettingsPageView;
use pathfinder_geometry::vector::Vector2F;
use privacy_page::PrivacySettingsPageView;
use scripting_page::ScriptingSettingsPageView;
use settings_file_footer::{SettingsFooterKind, SettingsFooterMouseStates, render_footer};
use settings_page::{
    HEADER_PADDING, MatchData, SettingsPage, SettingsPageEvent, SettingsPageMeta,
    SettingsPageViewHandle,
};
use shell_integration_page::{ShellIntegrationPageAction, ShellIntegrationPageView};
use warp_core::channel::ChannelState;
use warp_core::context_flag::ContextFlag;
use warp_core::features::FeatureFlag;
use warp_core::send_telemetry_from_ctx;
use warp_core::settings::ToggleableSetting as _;
use warp_core::ui::theme::color::internal_colors;
use warp_editor::editor::NavigationKey;
use warpui::elements::{
    Align, Border, ChildAnchor, ChildView, Clipped, ClippedScrollStateHandle, ClippedScrollable,
    ConstrainedBox, Container, CornerRadius, CrossAxisAlignment, DispatchEventResult, Empty,
    EventHandler, Expanded, Fill, Flex, MainAxisSize, OffsetPositioning, ParentAnchor,
    ParentElement, ParentOffsetBounds, Radius, SavePosition, ScrollbarWidth, Shrinkable, Stack,
    Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::keymap::{ContextPredicate, EnabledPredicate, FixedBinding};
use warpui::{
    Action, AppContext, Element, Entity, ModelHandle, SingletonEntity, TypedActionView,
    UpdateView as _, View, ViewContext, ViewHandle, id,
};

use self::telemetry::SettingsTelemetryEvent;
use crate::ai::custom_model_routers::CustomModelRouter;
use crate::ai::execution_profiles::ExecutionProfileId;
use crate::appearance::Appearance;
use crate::editor::{
    EditorView, Event as EditorEvent, PropagateAndNoOpNavigationKeys, SingleLineEditorOptions,
    TextColors, TextOptions,
};
use crate::menu::{self, Menu, MenuItem, MenuItemFields};
use crate::pane_group::focus_state::PaneFocusHandle;
use crate::pane_group::pane::view;
use crate::pane_group::{BackingView, Direction, PaneConfiguration, PaneEvent, SplitPaneState};
use crate::server::telemetry::MCPServerCollectionPaneEntrypoint;
use crate::settings::{AISettings, BlockVisibilitySettings, SettingsFileError};
use crate::settings_view::mcp_servers_page::{MCPServersSettingsPage, MCPServersSettingsPageEvent};
use crate::terminal::SizeInfo;
use crate::terminal::model::blockgrid::BlockGrid;
use crate::ui_components::icons;
use crate::util::bindings::{BindingGroup, CustomAction, keybinding_name_to_display_string};
use crate::view_components::ToastFlavor;
use crate::workspace::WorkspaceAction;
use crate::{GlobalResourceHandlesProvider, TelemetryEvent};

mod about_page;
mod agent_assisted_environment_modal;
mod appearance_page;
mod delete_environment_confirmation_dialog;
mod directory_color_add_picker;
pub(crate) mod environments_page;
pub(crate) mod handoff_environment_creation_modal;
pub mod keybindings;
pub mod mcp_servers;
pub mod mcp_servers_page;
pub mod pane_manager;
mod privacy_page;
mod scripting_page;
mod settings_file_footer;
pub(crate) mod settings_page;
mod shell_integration_page;
mod telemetry;
pub mod update_environment_form;

pub use settings_page::{
    AdditionalInfo, InputListItem, LocalOnlyIconState, ToggleState, render_body_item_label,
    render_info_icon, render_input_list, render_separator,
};

/// Original sidebar width used when the settings-file footer is not
/// enabled. Preserved for Preview/Stable until `FeatureFlag::SettingsFile`
/// is promoted.
const SIDEBAR_WIDTH_DEFAULT: f32 = 200.;

/// Wider sidebar used when the settings-file footer is enabled. Sized to
/// match Figma's settings nav rail (223px alert + 12px horizontal padding
/// on each side + 1px right border), giving the error-alert footer enough
/// room to render its "Open file" and "Fix with the agent" buttons side-by-side
/// with the designed 24px indent and 8px internal padding.
const SIDEBAR_WIDTH_WITH_FOOTER: f32 = 248.;

/// Returns the sidebar width, widened only when the settings-file footer
/// is enabled. This keeps the wider layout gated with the footer itself so
/// Preview/Stable users don't see an unexplained 48px width bump before
/// the feature ships.
fn sidebar_width() -> f32 {
    if FeatureFlag::SettingsFile.is_enabled() {
        SIDEBAR_WIDTH_WITH_FOOTER
    } else {
        SIDEBAR_WIDTH_DEFAULT
    }
}

/// Width of the borders for the header and the sidebar.
const SECTION_BORDER_WIDTH: f32 = 1.;

const POSITION_ID: &str = "settings_pane";

/// Saved-position id for the settings search input.
pub const SEARCH_EDITOR_POSITION_ID: &str = "settings_search_editor";

/// Saved-position id for a top-level sidebar row.
///
/// Nav-row position ids are derived from the [`SettingsSection`] variant
/// rather than its display label so that changing user-facing copy does not
/// break the integration tests that click these rows.
///
/// Nav rows cache their position for a single frame, so a row's presence in
/// the position cache means it was painted in the most recent frame. That is
/// what lets integration tests assert sidebar visibility against what was
/// actually drawn instead of re-deriving the filter rules.
pub fn nav_page_position_id(section: SettingsSection) -> String {
    format!("settings_nav_page:{section:?}")
}

/// Saved-position id for a collapsible umbrella header row.
pub fn nav_umbrella_position_id(label: &str) -> String {
    format!("settings_nav_umbrella:{label}")
}

/// Saved-position id for a subpage row nested under an umbrella.
pub fn nav_subpage_position_id(section: SettingsSection) -> String {
    format!("settings_nav_subpage:{section:?}")
}

pub(super) fn editor_text_colors(appearance: &Appearance) -> TextColors {
    let theme = appearance.theme();
    TextColors {
        default_color: theme.active_ui_text_color(),
        disabled_color: theme.disabled_ui_text_color(),
        hint_color: theme.disabled_ui_text_color(),
    }
}

#[derive(PartialEq)]
pub enum SettingsViewEvent {
    Pane(PaneEvent),
    StartResize,
    CheckForUpdate,
    LaunchNetworkLogging,
    OpenWarpDrive,
    SignupAnonymousUser,
    ShowToast {
        message: String,
        flavor: ToastFlavor,
    },
    OpenAIFactCollection,
    OpenMCPServerCollection,
    OpenCustomRouterEditor(Option<CustomModelRouter>),
    OpenCustomRouterFile(PathBuf),
    OpenExecutionProfileEditor(ExecutionProfileId),
    OpenLspLogs {
        log_path: PathBuf,
    },
    OpenProjectRulesPane {
        rule_paths: Vec<PathBuf>,
    },
}

/// Different navigation sections within the settings view
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum SettingsSection {
    About,
    Appearance,
    #[default]
    Keybindings,
    Privacy,
    Scripting,
    ShellIntegration,
    AgentMCPServers,
    CloudEnvironments,
}

use std::fmt::{self, Display};

use crate::util::bindings::custom_tag_to_keystroke;

impl Display for SettingsSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingsSection::Appearance => write!(f, "Appearance"),
            SettingsSection::Keybindings => write!(f, "Keyboard shortcuts"),
            SettingsSection::Scripting => write!(f, "Scripting"),
            SettingsSection::ShellIntegration => write!(f, "Shell integration"),
            SettingsSection::AgentMCPServers => write!(f, "MCP servers"),
            SettingsSection::CloudEnvironments => write!(f, "Environments"),
            _ => write!(f, "{self:?}"),
        }
    }
}

impl SettingsSection {
    /// Stable identifier for this section, used everywhere the section leaves
    /// the process: the SQLite session-restore key and the
    /// `surface.settings.open --page` warpctrl vocabulary.
    ///
    /// These strings are a compatibility contract — changing one breaks
    /// session restore for existing users and a public CLI argument. They were
    /// seeded from the [`Display`] labels that previously did this job, so no
    /// migration is needed. [`Display`] is now purely the user-facing sidebar
    /// label and is free to change without touching anything here.
    ///
    /// Deeplinks are deliberately *not* on this vocabulary: `warp://settings`
    /// uses its own snake_cased allowlist (see
    /// `settings_section_for_simple_subpage`).
    pub fn slug(self) -> &'static str {
        match self {
            Self::About => "About",
            Self::Appearance => "Appearance",
            Self::Keybindings => "Keyboard shortcuts",
            Self::Privacy => "Privacy",
            Self::Scripting => "Scripting",
            Self::ShellIntegration => "shell_integration",
            Self::AgentMCPServers => "MCP servers",
            Self::CloudEnvironments => "Environments",
        }
    }

    /// Parses a [`Self::slug`], also accepting the legacy spellings that
    /// persisted sessions and existing warpctrl callers may still be using.
    ///
    /// Legacy names for pages that no longer exist under that name resolve
    /// here, at the boundary, rather than becoming sections of their own. That
    /// keeps every `SettingsSection` value a real nav target, so no caller has
    /// to remember to normalize one before navigating.
    pub fn from_slug(slug: &str) -> Option<Self> {
        let section = match slug {
            "About" => Self::About,
            "Appearance" => Self::Appearance,
            "Keyboard shortcuts" => Self::Keybindings,
            "Privacy" => Self::Privacy,
            "Scripting" => Self::Scripting,
            "shell_integration" | "Warpify" => Self::ShellIntegration,
            // "MCP Servers" named the standalone page before it moved under the
            // Agents umbrella; it differs from the current slug only by casing.
            "MCP servers" | "MCP Servers" | "AgentMCPServers" => Self::AgentMCPServers,
            // "Code" named the combined page before it split in two.
            "Environments" | "CloudEnvironments" => Self::CloudEnvironments,
            _ => return None,
        };
        Some(section)
    }
}

/// Resolves a stable, friendly deeplink slug (used by
/// `warp://settings?widget=<slug>`) to the settings page and `&'static str`
/// widget id it should scroll to.
///
/// Only allowlisted widgets are linkable, so the public URL contract stays
/// stable and internal widget identifiers (Rust type names) are not exposed.
/// Add an entry here to make a new widget deep-linkable.
pub fn settings_widget_deeplink_target(_slug: &str) -> Option<(SettingsSection, &'static str)> {
    None
}

pub struct DisplayCount(pub usize);

impl Entity for DisplayCount {
    type Event = ();
}

impl SingletonEntity for DisplayCount {}

impl DisplayCount {
    pub fn num_displays(&self) -> usize {
        self.0
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self(1)
    }
}

pub mod flags {
    // The following are context flags to determine if the enable or disable binding is shown.
    pub const COPY_ON_SELECT_CONTEXT_FLAG: &str = "Copy_On_Select";

    pub const LINUX_SELECTION_CLIPBOARD_FLAG: &str = "Linux_Selection_Clipboard";
    pub const RESTORE_SESSION_CONTEXT_FLAG: &str = "Restore_Sessions";
    pub const HONOR_PS1_CONTEXT_FLAG: &str = "Honor_PS1";
    pub const GIT_PROMPT_CONTEXT_FLAG: &str = "Git_Prompt";
    pub const AUTOCOMPLETE_SYMBOLS_CONTEXT_FLAG: &str = "Autocomplete_Symbols";
    pub const QUAKE_MODE_ENABLED_CONTEXT_FLAG: &str = "Quake_Mode_Editor";
    pub const QUAKE_WINDOW_OPEN_FLAG: &str = "Quake_Window_Open";
    pub const EXTRA_META_KEYS_RIGHT_CONTEXT_FLAG: &str = "Extra_Meta_Keys_Right";
    pub const EXTRA_META_KEYS_LEFT_CONTEXT_FLAG: &str = "Extra_Meta_Keys_Left";
    pub const MOUSE_REPORTING_CONTEXT_FLAG: &str = "Mouse_Reporting";
    pub const SCROLL_REPORTING_CONTEXT_FLAG: &str = "Scroll_Reporting";
    pub const FOCUS_REPORTING_CONTEXT_FLAG: &str = "Focus_Reporting";
    pub const SSH_REUSE_CONTROL_MASTER_CONTEXT_FLAG: &str = "SSH_Reuse_Control_Master";
    pub const SSH_WARPIFICATION_CONTEXT_FLAG: &str = "SSH_Warpification";
    pub const NOTIFICATIONS_CONTEXT_FLAG: &str = "Notifications_Enabled";
    pub const LONG_RUNNING_NOTIFICATIONS_FLAG: &str = "Long_Running_Notifications";
    pub const AGENT_TASK_COMPLETED_NOTIFICATIONS_FLAG: &str = "Agent_Task_Completed_Notifications";
    pub const NEEDS_ATTENTION_NOTIFICATIONS_FLAG: &str = "Needs_Attention_Notifications";
    pub const NOTIFICATION_SOUND_FLAG: &str = "Notification_Sound";
    pub const AGENT_IN_APP_NOTIFICATIONS_FLAG: &str = "Agent_In_App_Notifications";
    pub const LINK_TOOLTIP_CONTEXT_FLAG: &str = "Link_Tooltip";
    pub const COMPACT_MODE_CONTEXT_FLAG: &str = "Compact_Mode_Enabled";
    pub const CURSOR_BLINK_CONTEXT_FLAG: &str = "Cursor_Blink_Enabled";
    pub const VIM_MODE_CONTEXT_FLAG: &str = "Vim_Mode_Enabled";
    pub const VIM_UNNAMED_SYSTEM_CLIPBOARD: &str = "Vim_Unnamed_System_Clipboard";
    pub const VIM_SHOW_STATUS_BAR: &str = "Vim_Show_Status_Bar";
    pub const JUMP_TO_BOTTOM_OF_BLOCK_BUTTON_CONTEXT_FLAG: &str =
        "Jump_To_Bottom_Of_Block_Button_Enabled";
    pub const RESPECT_SYSTEM_THEME_CONTEXT_FLAG: &str = "Respect_System_Theme";
    pub const COMPLETIONS_OPEN_WHILE_TYPING_CONTEXT_FLAG: &str = "Completions_Open_While_Typing";
    pub const COMMAND_CORRECTIONS_CONTEXT_FLAG: &str = "Command_Corrections";
    pub const ERROR_UNDERLINING_FLAG: &str = "error_underlining";
    pub const SYNTAX_HIGHLIGHTING_FLAG: &str = "syntax_highlighting";
    pub const SAME_LINE_PROMPT: &str = "Same_Line_Prompt_Enabled";
    pub const TELEMETRY_FLAG: &str = "telemetry";
    pub const SETTINGS_SYNC_FLAG: &str = "settings_sync";
    pub const SAFE_MODE_FLAG: &str = "safe_mode";
    pub const CRASH_REPORTING_FLAG: &str = "crash_reporting";
    pub const CLOUD_CONVERSATION_STORAGE_FLAG: &str = "Cloud_Conversation_Storage_Enabled";
    pub const CLOUD_CONVERSATION_STORAGE_EDITABLE_FLAG: &str =
        "Cloud_Conversation_Storage_Editable";
    pub const DIM_INACTIVE_PANES_FLAG: &str = "Dim_Inactive_Panes";
    pub const OPEN_WINDOWS_AT_CUSTOM_SIZE_FLAG: &str = "Open_Windows_At_Custom_Size";
    pub const WINDOW_BLUR_TEXTURE_FLAG: &str = "Window_Blur_Texture";
    pub const LEFT_PANEL_VISIBILITY_ACROSS_TABS_FLAG: &str = "Left_Panel_Visibility_Across_Tabs";
    pub const MATCH_AI_FONT_TO_TERMINAL_FONT_FLAG: &str = "Match_AI_Font_To_Terminal_Font";
    pub const MATCH_NOTEBOOK_FONT_SIZE_TO_TERMINAL_FONT_SIZE_FLAG: &str =
        "Match_Notebook_Font_Size_To_Terminal_Font_Size";
    pub const QUIT_WARNING_MODAL: &str = "Quit_Warning_Modal";
    pub const BLOCK_DIVIDERS_CONTEXT_FLAG: &str = "Block_Dividers_Enabled";

    pub const LOG_OUT_WARNING_MODAL: &str = "Log_Out_Warning_Modal";
    pub const SMART_SELECT_FLAG: &str = "Smart_Select_Enabled";
    pub const ACTIVATION_HOTKEY_FLAG: &str = "Activation_Hotkey_Enabled";
    pub const TAB_INDICATORS_FLAG: &str = "Tab_Indicators_Enabled";
    pub const SHOW_CODE_REVIEW_BUTTON_FLAG: &str = "Show_Code_Review_Button_Enabled";
    pub const SHOW_CODE_REVIEW_DIFF_STATS_FLAG: &str = "Show_Code_Review_Diff_Stats_Enabled";
    pub const AUTO_OPEN_CODE_REVIEW_PANE_FLAG: &str = "Auto_Open_Code_Review_Pane_Enabled";
    pub const USE_VERTICAL_TABS_FLAG: &str = "Use_Vertical_Tabs";
    pub const PRESERVE_ACTIVE_TAB_COLOR_FLAG: &str = "Preserve_Active_Tab_Color";
    pub const SHOW_VERTICAL_TAB_PANEL_IN_RESTORED_WINDOWS_FLAG: &str =
        "Show_Vertical_Tab_Panel_In_Restored_Windows";
    pub const USE_LATEST_USER_PROMPT_AS_CONVERSATION_TITLE_IN_TAB_NAMES_FLAG: &str =
        "Use_Latest_User_Prompt_As_Conversation_Title_In_Tab_Names";
    pub const ALT_SCREEN_PADDING_FLAG: &str = "Alt_Screen_Padding";
    pub const SESSION_CONFIG_TAB_CONFIG_CHIP_OPEN: &str = "Session_Config_Tab_Config_Chip_Open";
    pub const FEATURE_INTRO_MODAL_OPEN: &str = "Feature_Intro_Modal_Open";
    pub const FOCUS_PANES_ON_HOVER_CONTEXT_FLAG: &str = "Focus_Panes_On_Hover";
    pub const HIDE_WORKSPACE_DECORATIONS_CONTEXT_FLAG: &str = "Hide_Workspace_Decorations";
    pub const ALIAS_EXPANSION_FLAG: &str = "Alias_Expansion_Enabled";
    pub const MIDDLE_CLICK_PASTE_FLAG: &str = "Middle_Click_Paste_Enabled";
    pub const CODE_AS_DEFAULT_EDITOR: &str = "Code_As_Default_Enabled";
    pub const SYNC_ALL_TABS_FLAG: &str = "Sync_All_Tabs_Enabled";
    pub const SYNC_ALL_PANES_IN_CURRENT_TAB: &str = "Sync_All_Panes_In_Current_Tab";
    pub const USE_AUDIBLE_BELL_CONTEXT_FLAG: &str = "Use_Audible_Terminal_Bell";
    pub const SHOW_INPUT_HINT_TEXT_CONTEXT_FLAG: &str = "Show_Input_Hint_text";
    pub const SHOW_AGENT_TIPS_FLAG: &str = "Show_Agent_Tips";
    pub const SHOW_OZ_UPDATES_IN_ZERO_STATE_FLAG: &str = "Show_Oz_Updates_In_Zero_State";
    pub const USE_AGENT_FOOTER_FLAG: &str = "Use_Agent_Footer";
    pub const THINKING_DISPLAY_SHOW_AND_COLLAPSE: &str = "Thinking_Display_ShowAndCollapse";
    pub const THINKING_DISPLAY_ALWAYS_SHOW: &str = "Thinking_Display_AlwaysShow";
    pub const THINKING_DISPLAY_NEVER_SHOW: &str = "Thinking_Display_NeverShow";
    pub const ORCHESTRATION_MESSAGE_DISPLAY_SHOW_AND_COLLAPSE: &str =
        "Orchestration_Message_Display_ShowAndCollapse";
    pub const ORCHESTRATION_MESSAGE_DISPLAY_ALWAYS_SHOW: &str =
        "Orchestration_Message_Display_AlwaysShow";
    pub const ORCHESTRATION_MESSAGE_DISPLAY_ALWAYS_COLLAPSE: &str =
        "Orchestration_Message_Display_AlwaysCollapse";
    pub const PROMPT_SUBMISSION_INTERRUPT: &str = "Prompt_Submission_Interrupt";
    pub const PROMPT_SUBMISSION_QUEUE: &str = "Prompt_Submission_Queue";
    pub const LRC_SUBMISSION_SEND_IMMEDIATELY: &str = "LRC_Submission_Send_Immediately";
    pub const LRC_SUBMISSION_QUEUE_UNTIL_COMMAND_COMPLETES: &str =
        "LRC_Submission_Queue_Until_Command_Completes";
    pub const SHOW_TERMINAL_INPUT_MESSAGE_LINE_FLAG: &str = "Show_Terminal_Input_Message_Line";
    pub const PRESERVE_INPUT_FOCUS_ON_BLOCK_SELECTION_FLAG: &str =
        "Preserve_Input_Focus_On_Block_Selection";
    pub const SLASH_COMMANDS_IN_TERMINAL_FLAG: &str = "Slash_Commands_In_Terminal";
    pub const AT_CONTEXT_MENU_IN_TERMINAL_FLAG: &str = "At_Context_Menu_In_Terminal";
    pub const OUTLINE_CODEBASE_SYMBOLS_FOR_AT_CONTEXT_MENU_FLAG: &str =
        "Outline_Codebase_Symbols_For_At_Context_Menu";
    pub const AUTOSUGGESTIONS_ENABLED_FLAG: &str = "Autosuggestions_Enabled";
    pub const AUTOSUGGESTION_KEYBINDING_HINT_FLAG: &str = "Hide_Autosuggestion_Keybinding_Hint";
    pub const SHOW_AUTOSUGGESTION_IGNORE_BUTTON_FLAG: &str = "Show_Autosuggestion_Ignore_Button";
    pub const SHOW_TERMINAL_ZERO_STATE_BLOCK_FLAG: &str = "Show_Terminal_Zero_State_Block";
    pub const GLOBAL_WORKFLOWS_IN_COMMAND_SEARCH_FLAG: &str = "Global_Workflows_In_Command_Search";
    pub const PREFER_LOW_POWER_GPU_FLAG: &str = "Prefer_Low_Power_GPU";
    pub const INITIALIZATION_BLOCK_FLAG: &str = "Initialization_Block_Visible";
    pub const IN_BAND_COMMAND_BLOCKS_FLAG: &str = "In_Band_Command_Blocks_Visible";
    pub const RECORDING_MODE_FLAG: &str = "Recording_Mode_Enabled";
    pub const IN_BAND_GENERATORS_FLAG: &str = "In_Band_Generators_Enabled";
    pub const WARP_SAME_LINE_PROMPT_FLAG: &str = "Warp_Same_Line_Prompt_Enabled";
    pub const DEBUG_NETWORK_ONLINE_FLAG: &str = "Network_Status_Online";
    pub const AI_INPUT_AUTODETECTION_FLAG: &str = "AI_Input_Autodetection";
    pub const NLD_IN_TERMINAL_FLAG: &str = "NLD_In_Terminal";
    pub const INTELLIGENT_AUTOSUGGESTIONS_FLAG: &str = "Intelligent_Autosuggestions";
    pub const PROMPT_SUGGESTIONS_FLAG: &str = "Prompt_Suggestions";
    pub const CODE_SUGGESTIONS_FLAG: &str = "Code_Suggestions";
    pub const NATURAL_LANGUAGE_AUTOSUGGESTIONS_FLAG: &str = "Natural_Language_Autosuggestions";
    pub const SHARED_BLOCK_TITLE_GENERATION_FLAG: &str = "Shared_Block_Title_Generation";
    pub const GIT_OPERATIONS_AUTOGEN_FLAG: &str = "Git_Operations_Autogen";
    pub const INCLUDE_AGENT_COMMANDS_IN_HISTORY_FLAG: &str = "Include_Agent_Commands_In_History";
    pub const AUTO_APPROVE_BYPASSES_COMMAND_DENYLIST_FLAG: &str =
        "Auto_Approve_Bypasses_Command_Denylist";
    pub const AI_RULES_FLAG: &str = "AI_Rules";
    pub const SUGGESTED_RULES_FLAG: &str = "Suggested_Rules";
    pub const WARP_DRIVE_CONTEXT_FLAG: &str = "Warp_Drive_Context";
    pub const FILE_BASED_MCP_FLAG: &str = "File_Based_MCP";
    pub const WARP_CREDIT_FALLBACK_FLAG: &str = "Warp_Credit_Fallback";
    pub const SHOW_BASE_MODEL_PICKER_IN_PROMPT_FLAG: &str = "Show_Base_Model_Picker_In_Prompt";
    pub const DEBUG_SHOW_MEMORY_STATS_FLAG: &str = "Debug_Memory_Statistics";
    pub const ALLOW_NATIVE_WAYLAND: &str = "Allow_Native_Wayland";
    pub const IS_ANY_AI_ENABLED: &str = "IsAnyAIEnabled";
    pub const IS_ACTIVE_AI_ENABLED: &str = "IsActiveAIEnabled";
    pub const IS_VOICE_INPUT_ENABLED: &str = "IsVoiceInputEnabled";
    pub const IS_BLOCK_AI_SUMMARIES_ENABLED: &str = "IsBlockAISummariesEnabled";
    pub const IS_CODEBASE_INDEXING_ENABLED: &str = "IsCodebaseIndexingEnabled";
    pub const IS_AUTOINDEXING_ENABLED: &str = "IsAutoIndexingEnabled";
    pub const LIGATURE_RENDERING_CONTEXT_FLAG: &str = "Ligature_Rendering_Enabled";
    pub const HAS_SETTINGS_TO_IMPORT_FLAG: &str = "HasSettingsToImport";
    /// The user's setting enabled UDI, but we may show a classic input (e.g. ssh/subshell warpification)
    pub const UNIVERSAL_DEVELOPER_INPUT_ENABLED: &str = "UniversalDeveloperInputEnabled";
    pub const AGENT_MODE_INPUT: &str = "InputAgentMode";
    pub const TERMINAL_MODE_INPUT: &str = "InputTerminalMode";
    pub const WARP_IS_DEFAULT_TERMINAL: &str = "WarpIsDefaultTerminal";
    pub const PASSIVE_CODE_DIFF_KEYBINDINGS_ENABLED: &str = "PassiveCodeDiffKeybindingsEnabled";
    /// When set, ctrl-enter should accept a prompt suggestion rather than insert a newline.
    /// This flag is set by the terminal Input when there's a pending passive code diff.
    pub const CTRL_ENTER_ACCEPTS_PROMPT_SUGGESTION: &str = "CtrlEnterAcceptsPromptSuggestion";
    /// When set, the terminal input owns Page Up / Page Down so the editor's fixed bindings
    /// should not match.
    pub const TERMINAL_INPUT_PAGE_KEYS_HANDLED_BY_INPUT: &str =
        "TerminalInputPageKeysHandledByInput";
    pub const HAS_PENDING_PROMPT_SUGGESTION: &str = "HasPendingPromptSuggestion";
    pub const ACTIVE_AGENT_VIEW: &str = "ActiveAgentView";
    pub const ACTIVE_INLINE_AGENT_VIEW: &str = "ActiveInlineAgentView";
    /// When set, ctrl-enter should be the active binding to enter agent view.
    ///
    /// This is true on linux and windows.
    pub const CTRL_ENTER_ENTERS_AGENT_VIEW: &str = "CtrlEnterEntersAgentView";
    pub const AGENT_VIEW_ENABLED: &str = "FeatureFlag.AgentView";
    pub const LOCKED_INPUT: &str = "LockedInput";
    pub const OPEN_INLINE_CONVERSATION_MENU: &str = "OpenInlineConversationMenu";
    pub const EMPTY_INPUT_BUFFER: &str = "EmptyInputBuffer";
    pub const CLI_AGENT_RICH_INPUT_OPEN: &str = "CLIAgentRichInputOpen";
    pub const CLI_AGENT_FOOTER_ENABLED: &str = "CLIAgentFooterEnabled";
    pub const CLI_AGENT_RICH_INPUT_CHIP_ENABLED: &str = "CLIAgentRichInputChipEnabled";
    pub const AUTO_TOGGLE_RICH_INPUT_FLAG: &str = "AutoToggleRichInput";
    pub const AUTO_OPEN_RICH_INPUT_ON_CLI_AGENT_START_FLAG: &str =
        "AutoOpenRichInputOnCLIAgentStart";
    pub const AUTO_DISMISS_RICH_INPUT_AFTER_SUBMIT_FLAG: &str = "AutoDismissRichInputAfterSubmit";
    pub const ENABLE_WARP_DRIVE: &str = "EnableWarpDrive";
    // Tools panel settings
    pub const SHOW_CONVERSATION_HISTORY: &str = "ShowConversationHistory";
    pub const SHOW_PROJECT_EXPLORER: &str = "ShowProjectExplorer";
    pub const SHOW_GLOBAL_SEARCH: &str = "ShowGlobalSearch";
    pub const SHOW_HIDDEN_FILES: &str = "ShowHiddenFiles";
}

pub fn init_actions_from_parent_view<T: Action + Clone>(
    app: &mut AppContext,
    context: &ContextPredicate,
    builder: fn(SettingsAction) -> T,
) {
    appearance_page::init_actions_from_parent_view(app, context, builder);
    shell_integration_page::init_actions_from_parent_view(app, context, builder);

    if ChannelState::enable_debug_features() || cfg!(windows) {
        ToggleSettingActionPair::add_toggle_setting_action_pairs_as_bindings(
            vec![
                ToggleSettingActionPair::custom(
                    SettingActionPairDescriptions::new(
                        "Show initialization block",
                        "Hide initialization block",
                    ),
                    builder(SettingsAction::Debug(
                        DebugSettingsAction::ToggleInitializationBlock,
                    )),
                    SettingActionPairContexts::new(
                        context.to_owned() & !id!(flags::INITIALIZATION_BLOCK_FLAG),
                        context.to_owned() & id!(flags::INITIALIZATION_BLOCK_FLAG),
                    ),
                    None,
                ),
                ToggleSettingActionPair::custom(
                    SettingActionPairDescriptions::new(
                        "Show in-band command blocks",
                        "Hide in-band command blocks",
                    ),
                    builder(SettingsAction::Debug(
                        DebugSettingsAction::ToggleInBandCommandBlocks,
                    )),
                    SettingActionPairContexts::new(
                        context.to_owned() & !id!(flags::IN_BAND_COMMAND_BLOCKS_FLAG),
                        context.to_owned() & id!(flags::IN_BAND_COMMAND_BLOCKS_FLAG),
                    ),
                    None,
                ),
            ],
            app,
        );
    }

    if FeatureFlag::DebugMode.is_enabled() {
        ToggleSettingActionPair::add_toggle_setting_action_pairs_as_bindings(
            vec![
                ToggleSettingActionPair::new(
                    "recording mode",
                    WorkspaceAction::ToggleRecordingMode,
                    &id!("Workspace"),
                    flags::RECORDING_MODE_FLAG,
                ),
                ToggleSettingActionPair::new(
                    "in-band generators for new sessions",
                    WorkspaceAction::ToggleInBandGenerators,
                    &id!("Workspace"),
                    flags::IN_BAND_GENERATORS_FLAG,
                ),
                ToggleSettingActionPair::new(
                    "debug network status",
                    WorkspaceAction::ToggleDebugNetworkStatus,
                    &id!("Workspace"),
                    flags::DEBUG_NETWORK_ONLINE_FLAG,
                ),
                ToggleSettingActionPair::new(
                    "memory statistics",
                    WorkspaceAction::ToggleShowMemoryStats,
                    &id!("Workspace"),
                    flags::DEBUG_SHOW_MEMORY_STATS_FLAG,
                ),
            ],
            app,
        );
    }

    let context = id!("SettingsViewInTab") & !id!("IMEOpen");
    app.register_fixed_bindings([
        FixedBinding::new("down", SettingsAction::Down, context.clone()),
        FixedBinding::new("up", SettingsAction::Up, context.clone()),
    ]);
}

/// The string the user will see when the action is enabled or disabled.
#[derive(Clone)]
pub struct SettingActionPairDescriptions {
    enable: String,
    disable: String,
}

impl SettingActionPairDescriptions {
    pub fn new(enable: &str, disable: &str) -> Self {
        Self {
            enable: enable.to_owned(),
            disable: disable.to_owned(),
        }
    }
}

/// The context to check to show the enable or disable
/// version of this action pair.
#[derive(Clone)]
pub struct SettingActionPairContexts {
    enable_predicate: ContextPredicate,
    disable_predicate: ContextPredicate,
}

impl SettingActionPairContexts {
    pub fn new(enable_predicate: ContextPredicate, disable_predicate: ContextPredicate) -> Self {
        Self {
            enable_predicate,
            disable_predicate,
        }
    }
}

/// Information needed to create a enable/disable action pair.
/// Note: The action pair doesn't actually need to update settings.
/// We should probably refactor this code to a different module.
#[derive(Clone)]
pub struct ToggleSettingActionPair<T: Action + Clone> {
    /// The user will actually read these strings.
    descriptions: SettingActionPairDescriptions,
    /// The actual action to toggle a setting on/off.
    toggle_action: T,
    /// We use our Context tree to determine where this setting should show up.
    /// Be sure to initialize all context strings you use
    /// in `fn keymap_context` in `impl View for Workspace`.
    contexts: SettingActionPairContexts,
    /// If Some(), custom_action is set as the Custom Trigger for the
    /// the toggle_action.
    /// This makes it possible to bind Mac menu items to the toggle_action.
    custom_action: Option<CustomAction>,
    /// Binding group for the set of actions produced by this pair. If not explicitly set, the
    /// `Settings` [`BindingGroup`] is applied.
    binding_group: BindingGroup,

    /// Predicate that determines if bindings corresponding to this pair are enabled.
    enabled_predicate: Option<EnabledPredicate>,

    /// Whether or not this pairing applies to the current platform (Mac, Linux, Web, etc.)
    supported_on_current_platform: bool,
}

impl<T: Action + Clone> ToggleSettingActionPair<T> {
    /// `description_suffix` will be visible to the user,
    /// e.g. `Enable {description_suffix}` or `Disable {description_suffix}`.
    /// We use contexts to decide if we show the user the enable or disable
    /// version of this action pair.
    /// `context_prefix` is logically ANDed with context_boolean_flag,
    /// like a prerequisite.
    /// `context_prefix` should be `Workspace` to have the action pair to
    /// display in the command palette.
    /// `context_boolean_flag` is will be in the context tree when the action
    /// is in the enabled state,
    /// and absent when the action is in the disabled state.
    pub fn new(
        description_suffix: &str,
        toggle_action: T,
        context_prefix: &ContextPredicate,
        context_boolean_flag: &'static str,
    ) -> Self {
        use warpui::keymap::macros::id;

        ToggleSettingActionPair {
            descriptions: SettingActionPairDescriptions {
                enable: format!("Enable {description_suffix}"),
                disable: format!("Disable {description_suffix}"),
            },
            contexts: SettingActionPairContexts {
                enable_predicate: context_prefix.to_owned() & !id!(context_boolean_flag),
                disable_predicate: context_prefix.to_owned() & id!(context_boolean_flag),
            },
            toggle_action,
            custom_action: None,
            binding_group: BindingGroup::Settings,
            supported_on_current_platform: true,
            enabled_predicate: None,
        }
    }

    pub fn custom(
        descriptions: SettingActionPairDescriptions,
        toggle_action: T,
        contexts: SettingActionPairContexts,
        custom_action: Option<CustomAction>,
    ) -> Self {
        ToggleSettingActionPair {
            toggle_action,
            contexts,
            descriptions,
            custom_action,
            binding_group: BindingGroup::Settings,
            supported_on_current_platform: true,
            enabled_predicate: None,
        }
    }

    pub fn with_group(mut self, group: BindingGroup) -> Self {
        self.binding_group = group;
        self
    }

    pub fn with_enabled(mut self, enabled_predicate: EnabledPredicate) -> Self {
        self.enabled_predicate = Some(enabled_predicate);
        self
    }

    pub fn is_supported_on_current_platform(mut self, value: bool) -> Self {
        self.supported_on_current_platform = value;
        self
    }

    /// Creates enable/disable bindings for a toggle feature, given a list of `ToggleSettingActionPair`'s.
    pub fn add_toggle_setting_action_pairs_as_bindings(
        action_pairs: Vec<ToggleSettingActionPair<T>>,
        app: &mut AppContext,
    ) {
        let (enable_bindings, disable_bindings): (Vec<FixedBinding>, Vec<FixedBinding>) =
            action_pairs
                .into_iter()
                .filter_map(|action_pair| {
                    let ToggleSettingActionPair {
                        toggle_action,
                        contexts,
                        descriptions,
                        custom_action,
                        binding_group,
                        supported_on_current_platform,
                        enabled_predicate,
                    } = action_pair;

                    if !supported_on_current_platform {
                        None
                    } else {
                        match custom_action {
                            Some(custom_action) => {
                                let mut enable_binding = FixedBinding::custom(
                                    custom_action,
                                    toggle_action.clone(),
                                    descriptions.enable,
                                    contexts.enable_predicate,
                                )
                                .with_group(binding_group.as_str());
                                let mut disable_binding = FixedBinding::custom(
                                    custom_action,
                                    toggle_action,
                                    descriptions.disable,
                                    contexts.disable_predicate,
                                )
                                .with_group(binding_group.as_str());

                                if let Some(enabled_predicate) = enabled_predicate {
                                    enable_binding = enable_binding.with_enabled(enabled_predicate);
                                    disable_binding =
                                        disable_binding.with_enabled(enabled_predicate);
                                }

                                Some((enable_binding, disable_binding))
                            }
                            None => {
                                let mut enable_binding = FixedBinding::empty(
                                    descriptions.enable,
                                    toggle_action.clone(),
                                    contexts.enable_predicate,
                                )
                                .with_group(binding_group.as_str());
                                let mut disable_binding = FixedBinding::empty(
                                    descriptions.disable,
                                    toggle_action,
                                    contexts.disable_predicate,
                                )
                                .with_group(binding_group.as_str());

                                if let Some(enabled_predicate) = enabled_predicate {
                                    enable_binding = enable_binding.with_enabled(enabled_predicate);
                                    disable_binding =
                                        disable_binding.with_enabled(enabled_predicate);
                                }

                                Some((enable_binding, disable_binding))
                            }
                        }
                    }
                })
                .unzip();

        app.register_fixed_bindings(enable_bindings);
        app.register_fixed_bindings(disable_bindings);
    }
}

#[derive(Clone, Debug)]
pub enum DebugSettingsAction {
    /// Whether or not the "bootstrap block" or "initialization block" is visible.
    ToggleInitializationBlock,
    /// Whether or not in-band generator commands are visible in the BlockList.
    ToggleInBandCommandBlocks,
}

#[derive(Debug, Clone)]
pub enum SettingsAction {
    SelectAndRefresh(SettingsSection),
    AppearancePageToggle(AppearancePageAction),
    ShellIntegrationPageToggle(ShellIntegrationPageAction),
    Tab,
    Split(Direction),
    ToggleMaximizePane,
    Close,
    OpenContextMenu(Vector2F),
    FocusSelf,
    Up,
    Down,
    /// For internal, debug-related settings which don't appear in the UI.
    Debug(DebugSettingsAction),
}

#[derive(Copy, Clone, Debug)]
enum CycleDirection {
    Up,
    Down,
}

/// A stop in the arrow-key navigation order over the sidebar.
///
/// A collapsed umbrella occupies a single stop rather than being skipped,
/// so arrow-key navigation auto-expands it and selects one of its visible
/// subpages instead of jumping over it. Which subpage is chosen depends
/// on the direction of cycling: navigating Down enters the umbrella at
/// its first visible subpage, while navigating Up enters at its last
/// visible subpage, matching the natural reading order the user was
fn visible_nav_sections<F>(nav_items: &[SettingsSection], is_visible: F) -> Vec<SettingsSection>
where
    F: Fn(SettingsSection) -> bool,
{
    nav_items
        .iter()
        .copied()
        .filter(|section| is_visible(*section))
        .collect()
}

fn next_stop_index(current: usize, len: usize, direction: CycleDirection) -> usize {
    debug_assert!(len > 0, "next_stop_index requires a non-empty stop list");
    match direction {
        CycleDirection::Up => {
            if current == 0 {
                len - 1
            } else {
                current - 1
            }
        }
        CycleDirection::Down => {
            if current + 1 >= len {
                0
            } else {
                current + 1
            }
        }
    }
}

macro_rules! update_page {
    ($handle:expr_2021, $update:expr_2021, $ctx:expr_2021) => {
        match $handle {
            SettingsPageViewHandle::Appearance(handle) => $ctx.update_view(handle, $update),
            SettingsPageViewHandle::Keybindings(handle) => $ctx.update_view(handle, $update),
            SettingsPageViewHandle::ShellIntegration(handle) => $ctx.update_view(handle, $update),
            SettingsPageViewHandle::Privacy(handle) => $ctx.update_view(handle, $update),
            SettingsPageViewHandle::Scripting(handle) => $ctx.update_view(handle, $update),
            SettingsPageViewHandle::CloudEnvironments(handle) => $ctx.update_view(handle, $update),
            SettingsPageViewHandle::About(handle) => $ctx.update_view(handle, $update),
            SettingsPageViewHandle::MCPServers(handle) => $ctx.update_view(handle, $update),
        }
    };
}

pub struct SettingsView {
    pane_configuration: ModelHandle<PaneConfiguration>,
    focus_handle: Option<PaneFocusHandle>,
    settings_pages: Vec<SettingsPage>,
    pages_filter: Vec<MatchData>,
    current_settings_page: SettingsSection,
    search_editor: ViewHandle<EditorView>,
    clipped_scroll_state: ClippedScrollStateHandle,
    context_menu: ViewHandle<Menu<SettingsAction>>,
    context_menu_state: Option<Vector2F>,
    environments_page_handle: ViewHandle<EnvironmentsPageView>,
    /// Sidebar navigation items (pages + umbrellas). This is the single source
    /// of truth for which sections sit under which umbrella.
    nav_items: Vec<SettingsSection>,
    /// Current settings.toml error, mirrored from `Workspace` via
    /// [`set_settings_error_state`]. Used by the sidebar footer to decide
    /// whether to show the inline error alert.
    settings_file_error: Option<SettingsFileError>,
    /// Whether the workspace-level settings-error banner has been dismissed.
    /// Mirrored from `Workspace` via [`set_settings_error_state`].
    settings_error_banner_dismissed: bool,
    /// Mouse state handles for the nav-rail footer buttons. Constructed once
    /// per `SettingsView` per `WARP.md`'s guidance that inline
    /// `MouseStateHandle::default()` breaks hover/click tracking.
    footer_mouse_states: SettingsFooterMouseStates,
}

impl SettingsView {
    pub fn new(page: Option<SettingsSection>, ctx: &mut ViewContext<Self>) -> Self {
        let pane_configuration = ctx.add_model(|_ctx| PaneConfiguration::new("Settings"));

        let _global_resource_handles = GlobalResourceHandlesProvider::as_ref(ctx).get().clone();
        // About page
        let about_page_handle = ctx.add_view(AboutPageView::new);

        // Privacy page
        let privacy_page_handle = ctx.add_typed_action_view(PrivacySettingsPageView::new);

        // Environments page
        let environments_page_handle = ctx.add_typed_action_view(EnvironmentsPageView::new);
        ctx.subscribe_to_view(&environments_page_handle, |me, _, event, ctx| {
            me.handle_environments_page_event(event, ctx);
        });

        // Appearance & themes page
        let appearance_page_handle = ctx.add_typed_action_view(AppearanceSettingsPageView::new);
        ctx.subscribe_to_view(&appearance_page_handle, |me, _, event, ctx| {
            me.handle_appearance_page_event(event, ctx);
        });

        // Keybindings page
        let keybindings_handle = ctx.add_typed_action_view(KeybindingsView::new);

        let shell_integration_page_handle =
            ctx.add_typed_action_view(ShellIntegrationPageView::new);
        ctx.subscribe_to_view(&shell_integration_page_handle, |me, _, event, ctx| {
            me.handle_shell_integration_page_event(event, ctx);
        });

        let scripting_page_handle = if FeatureFlag::WarpControlCli.is_enabled() {
            Some(ctx.add_typed_action_view(ScriptingSettingsPageView::new))
        } else {
            None
        };

        // MCP Servers page
        let mcp_servers_page_handle = ctx.add_typed_action_view(MCPServersSettingsPageView::new);
        ctx.subscribe_to_view(&mcp_servers_page_handle, |me, _, event, ctx| {
            me.handle_mcp_servers_page_event(event, ctx);
        });

        let font_family = Appearance::as_ref(ctx).ui_font_family();
        let search_editor = ctx.add_typed_action_view(|ctx| {
            let options = SingleLineEditorOptions {
                text: TextOptions {
                    font_family_override: Some(font_family),
                    ..Default::default()
                },
                // We want "up" and "down" to cycle settings pages.
                propagate_and_no_op_vertical_navigation_keys:
                    PropagateAndNoOpNavigationKeys::Always,
                ..Default::default()
            };
            let mut editor = EditorView::single_line(options, ctx);
            editor.set_placeholder_text("Search", ctx);
            editor
        });

        ctx.subscribe_to_view(&search_editor, Self::handle_search_editor_event);

        let context_menu = ctx.add_typed_action_view(|_| {
            Menu::new()
                .prevent_interaction_with_other_elements()
                .with_drop_shadow()
        });
        ctx.subscribe_to_view(&context_menu, move |me, _, event, ctx| {
            me.handle_menu_event(event, ctx);
        });

        let mut settings_pages = vec![
            SettingsPage::new(appearance_page_handle),
            SettingsPage::new(keybindings_handle),
            SettingsPage::new(privacy_page_handle),
            SettingsPage::new(shell_integration_page_handle),
        ];

        if let Some(scripting_page_handle) = scripting_page_handle {
            settings_pages.push(SettingsPage::new(scripting_page_handle));
        }

        settings_pages.extend(vec![
            SettingsPage::new(mcp_servers_page_handle),
            SettingsPage::new(environments_page_handle.clone()),
            SettingsPage::new(about_page_handle),
        ]);

        // The sidebar. Everything else is configured in `~/.nerminal/settings.toml`,
        // which hot-reloads; only the pages a file cannot replace are listed here.
        let mut nav_items = vec![SettingsSection::Appearance, SettingsSection::Keybindings];

        if cfg!(target_os = "macos") {
            nav_items.push(SettingsSection::Privacy);
        }

        if FeatureFlag::WarpControlCli.is_enabled() {
            nav_items.push(SettingsSection::Scripting);
        }
        nav_items.push(SettingsSection::About);

        // Landing page, and the fallback for a page this build does not have.
        let default_page = SettingsSection::Keybindings;
        let is_navigable = |section: SettingsSection| nav_items.contains(&section);
        let initial_page = match page {
            Some(SettingsSection::Scripting) if !FeatureFlag::WarpControlCli.is_enabled() => {
                default_page
            }
            Some(page) if is_navigable(page) => page,
            Some(_) => default_page,
            None => default_page,
        };

        Self {
            pages_filter: settings_pages
                .iter()
                .map(|_| MatchData::Uncounted(true))
                .collect(),
            settings_pages,
            current_settings_page: initial_page,
            pane_configuration,
            focus_handle: None,
            search_editor,
            clipped_scroll_state: Default::default(),
            context_menu,
            context_menu_state: Default::default(),
            environments_page_handle,
            nav_items,
            settings_file_error: None,
            settings_error_banner_dismissed: false,
            footer_mouse_states: SettingsFooterMouseStates::default(),
        }
    }

    /// Pushes the current settings-file error state from `Workspace` into this
    /// view. Called by `Workspace` once at construction time and then again
    /// whenever the error state or banner dismissal changes. Triggers a
    /// re-render when anything actually changed.
    pub fn set_settings_error_state(
        &mut self,
        error: Option<SettingsFileError>,
        banner_dismissed: bool,
        ctx: &mut ViewContext<Self>,
    ) {
        let error_changed = self.settings_file_error != error;
        let dismissed_changed = self.settings_error_banner_dismissed != banner_dismissed;
        if !error_changed && !dismissed_changed {
            return;
        }
        self.settings_file_error = error;
        self.settings_error_banner_dismissed = banner_dismissed;
        ctx.notify();
    }

    pub fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus(&self.search_editor);
        ctx.emit(SettingsViewEvent::Pane(PaneEvent::FocusSelf));
    }

    fn filtered_pages<'a>(
        &'a self,
        app: &'a AppContext,
    ) -> impl Iterator<Item = (&'a SettingsPage, MatchData)> {
        self.settings_pages
            .iter()
            .zip(self.pages_filter.iter())
            .filter_map(move |(page, match_data)| {
                (self.should_render_page(page, app) && match_data.is_truthy())
                    .then_some((page, *match_data))
            })
    }

    fn handle_search_editor_event(
        &mut self,
        editor: ViewHandle<EditorView>,
        event: &EditorEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        match event {
            EditorEvent::Edited(_) => {
                let search_query = editor.as_ref(ctx).buffer_text(ctx);
                let is_search_active = !search_query.is_empty();

                // Every page owns its whole widget list, so one filter pass
                // covers all of them.
                for (i, page) in self.settings_pages.iter().enumerate() {
                    self.pages_filter[i] = update_page!(
                        &page.view_handle,
                        |view, ctx| {
                            let match_data = view.update_filter(&search_query, ctx);
                            ctx.notify();
                            match_data
                        },
                        ctx
                    );
                }

                // Auto-select: if the current page is no longer visible, jump
                // to the first visible subpage or page.
                let current_still_visible = self
                    .filtered_pages(ctx)
                    .any(|(page, _)| page.section == self.current_settings_page);

                if !current_still_visible {
                    // While searching, walk the sidebar order so a matching
                    // subpage is preferred over a later top-level page.
                    let first_visible = if is_search_active {
                        self.nav_items
                            .iter()
                            .copied()
                            .find(|section| self.section_passes_search_filter(*section))
                    } else {
                        self.filtered_pages(ctx)
                            .next()
                            .map(|(page, _)| page.section)
                    };

                    if let Some(new_section) = first_visible {
                        self.set_and_refresh_current_page_internal(
                            new_section,
                            false, /* should_clear_query */
                            false, /* allow_steal_focus */
                            ctx,
                        );
                    }
                }
                ctx.notify();
            }
            EditorEvent::Navigate(NavigationKey::Down) => self.key_down(ctx),
            EditorEvent::Navigate(NavigationKey::Up) => self.key_up(ctx),
            EditorEvent::Escape => ctx.focus_self(),
            _ => {}
        }
    }

    fn context_menu_items(&self, ctx: &mut ViewContext<Self>) -> Vec<MenuItem<SettingsAction>> {
        let mut items = vec![];

        if ContextFlag::CreateNewSession.is_enabled() {
            items.extend(vec![
                MenuItemFields::new("Split pane right")
                    .with_on_select_action(SettingsAction::Split(Direction::Right))
                    .with_key_shortcut_label(keybinding_name_to_display_string(
                        "pane_group:add_right",
                        ctx,
                    ))
                    .into_item(),
                MenuItemFields::new("Split pane left")
                    .with_on_select_action(SettingsAction::Split(Direction::Left))
                    .with_key_shortcut_label(keybinding_name_to_display_string(
                        "pane_group:add_left",
                        ctx,
                    ))
                    .into_item(),
                MenuItemFields::new("Split pane down")
                    .with_on_select_action(SettingsAction::Split(Direction::Down))
                    .with_key_shortcut_label(keybinding_name_to_display_string(
                        "pane_group:add_down",
                        ctx,
                    ))
                    .into_item(),
                MenuItemFields::new("Split pane up")
                    .with_on_select_action(SettingsAction::Split(Direction::Up))
                    .with_key_shortcut_label(keybinding_name_to_display_string(
                        "pane_group:add_up",
                        ctx,
                    ))
                    .into_item(),
            ]);
        }

        let split_pane_state = self
            .focus_handle
            .as_ref()
            .map(|h| h.split_pane_state(ctx))
            .unwrap_or(SplitPaneState::NotInSplitPane);

        if split_pane_state.is_in_split_pane() {
            let is_maximized = split_pane_state.is_maximized();
            items.push(
                MenuItemFields::toggle_pane_action(is_maximized)
                    .with_on_select_action(SettingsAction::ToggleMaximizePane)
                    .with_key_shortcut_label(keybinding_name_to_display_string(
                        "pane_group:toggle_maximize_pane",
                        ctx,
                    ))
                    .into_item(),
            );

            items.push(
                MenuItemFields::new("Close pane")
                    .with_on_select_action(SettingsAction::Close)
                    .with_key_shortcut_label(
                        custom_tag_to_keystroke(CustomAction::CloseCurrentSession.into())
                            .map(|keystroke| keystroke.displayed()),
                    )
                    .into_item(),
            );
        }

        items
    }

    fn handle_menu_event(&mut self, event: &menu::Event, ctx: &mut ViewContext<Self>) {
        if let menu::Event::Close { .. } = event {
            self.context_menu_state.take();
        }
        ctx.notify();
    }

    fn clear_search_query(&mut self, ctx: &mut ViewContext<Self>) {
        self.search_editor.update(ctx, |editor, ctx| {
            editor.clear_buffer(ctx);
        });
        self.pages_filter = self
            .settings_pages
            .iter()
            .map(|_| MatchData::Uncounted(true))
            .collect();
    }

    pub fn set_ps1_info(
        &mut self,
        ps1_grid_info: Option<(BlockGrid, SizeInfo)>,
        ctx: &mut ViewContext<Self>,
    ) {
        if let Some(appearance_page) = self.settings_page(SettingsSection::Appearance)
            && let SettingsPageViewHandle::Appearance(view) = &appearance_page.view_handle
        {
            view.update(ctx, |view, ctx| {
                view.set_ps1_info(ps1_grid_info, ctx);
            })
        }
    }

    pub fn get_ps1_info(&self, app: &AppContext) -> Option<(BlockGrid, SizeInfo)> {
        self.settings_page(SettingsSection::Appearance)
            .and_then(|appearance_page| {
                if let SettingsPageViewHandle::Appearance(view) = &appearance_page.view_handle {
                    view.read(app, |view, _| view.get_ps1_info().map(ToOwned::to_owned))
                } else {
                    None
                }
            })
    }

    fn handle_appearance_page_event(
        &mut self,
        event: &SettingsPageEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        match event {
            SettingsPageEvent::FocusModal => ctx.focus(&self.search_editor),
            SettingsPageEvent::Pane(_)
            | SettingsPageEvent::EnvironmentSetupModeSelectorToggled { .. }
            | SettingsPageEvent::AgentAssistedEnvironmentModalToggled { .. } => {
                // Only meaningful when the view is hosted inside a pane.
            }
        }
    }

    fn handle_environments_page_event(
        &mut self,
        event: &SettingsPageEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        match event {
            SettingsPageEvent::FocusModal => ctx.focus(&self.search_editor),
            SettingsPageEvent::EnvironmentSetupModeSelectorToggled { .. }
            | SettingsPageEvent::AgentAssistedEnvironmentModalToggled { .. } => {
                // Re-render so the modal overlay is shown/hidden.
                ctx.notify();
            }
            SettingsPageEvent::Pane(_) => {
                // Not applicable in standalone settings view.
            }
        }
    }

    fn handle_shell_integration_page_event(
        &mut self,
        event: &SettingsPageEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        match event {
            SettingsPageEvent::FocusModal => ctx.focus(&self.search_editor),
            SettingsPageEvent::Pane(_)
            | SettingsPageEvent::EnvironmentSetupModeSelectorToggled { .. }
            | SettingsPageEvent::AgentAssistedEnvironmentModalToggled { .. } => {
                // These events are not handled in standalone settings - only used
                // when the view is hosted inside a pane.
            }
        }
    }

    fn handle_mcp_servers_page_event(
        &mut self,
        event: &MCPServersSettingsPageEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        match event {
            MCPServersSettingsPageEvent::ShowModal => {
                // Modal rendering is handled in get_modal_content_for_page
                ctx.notify();
            }
            MCPServersSettingsPageEvent::HideModal => {
                // Modal rendering is handled in get_modal_content_for_page
                ctx.notify();
            }
        }
    }

    pub fn search_for_keybinding(&mut self, keybinding_name: &str, ctx: &mut ViewContext<Self>) {
        self.set_and_refresh_current_page(SettingsSection::Keybindings, ctx);

        if let Some(settings_page) = self.current_settings_page()
            && let SettingsPageViewHandle::Keybindings(view_handle) = &settings_page.view_handle
        {
            view_handle.update(ctx, |view, ctx| {
                view.search_for_binding(keybinding_name, ctx);
            })
        }
    }

    pub fn current_settings_section(&self) -> SettingsSection {
        self.current_settings_page
    }

    fn current_settings_page(&self) -> Option<&SettingsPage> {
        self.settings_pages
            .iter()
            .find(|page| page.section == self.current_settings_page)
    }

    fn settings_page(&self, section: SettingsSection) -> Option<&SettingsPage> {
        let settings_page = self
            .settings_pages
            .iter()
            .find(|page| page.section == section);
        if settings_page.is_none() {
            log::warn!("settings section {section:?} not found");
        }
        settings_page
    }

    /// Whether `section` is reachable from this build's sidebar.
    ///
    /// The backing page for every section is always constructed, so a section
    /// whose nav entry is omitted is still openable by anything that names it
    /// directly — session restore, a deeplink, `warpctrl`. Those land on the
    /// sidebar's first page instead.
    fn is_navigable(&self, section: SettingsSection) -> bool {
        self.nav_items.contains(&section)
    }

    fn landing_page(&self) -> SettingsSection {
        self.nav_items.first().copied().unwrap_or_default()
    }

    pub fn set_and_refresh_current_page_internal(
        &mut self,
        section: SettingsSection,
        should_clear_query: bool,
        allow_steal_focus: bool,
        ctx: &mut ViewContext<Self>,
    ) {
        let section = if self.is_navigable(section) {
            section
        } else {
            self.landing_page()
        };

        // Every nav target owns its backing page. Check it exists.
        if self.settings_page(section).is_none() {
            return;
        }
        let previous_section = self.current_settings_page;

        ctx.enable_key_bindings_dispatching();

        if let Some(current_page) = self.current_settings_page() {
            update_page!(
                &current_page.view_handle,
                |view, ctx| {
                    view.clear_highlighted_widget();
                    ctx.notify();
                },
                ctx
            );
        }

        if should_clear_query {
            self.clear_search_query(ctx);
        }
        self.current_settings_page = section;
        if previous_section != section && section == SettingsSection::CloudEnvironments {
            send_telemetry_from_ctx!(SettingsTelemetryEvent::EnvironmentsPageOpened, ctx);
        }

        #[cfg(feature = "crash_reporting")]
        {
            crate::crash_reporting::set_tag("warp.settings_page", section.to_string());
        }

        if let Some(settings_page) = self.current_settings_page() {
            update_page!(
                &settings_page.view_handle,
                |view, ctx| {
                    view.on_page_selected(allow_steal_focus, ctx);
                },
                ctx
            );
        }
        ctx.notify();
    }

    pub fn set_and_refresh_current_page(
        &mut self,
        section: SettingsSection,
        ctx: &mut ViewContext<Self>,
    ) {
        self.set_and_refresh_current_page_internal(section, true, true, ctx);
    }

    pub fn set_search_query(&mut self, query: &str, ctx: &mut ViewContext<Self>) {
        self.search_editor.update(ctx, |editor, ctx| {
            editor.set_buffer_text(query, ctx);
        });
    }

    fn should_render_page(&self, settings_page: &SettingsPage, app: &AppContext) -> bool {
        match &settings_page.view_handle {
            SettingsPageViewHandle::Appearance(v) => v.as_ref(app).should_render(app),
            SettingsPageViewHandle::Keybindings(v) => v.as_ref(app).should_render(app),
            SettingsPageViewHandle::About(v) => v.as_ref(app).should_render(app),
            SettingsPageViewHandle::ShellIntegration(v) => v.as_ref(app).should_render(app),
            SettingsPageViewHandle::Privacy(v) => v.as_ref(app).should_render(app),
            SettingsPageViewHandle::Scripting(v) => v.as_ref(app).should_render(app),
            SettingsPageViewHandle::CloudEnvironments(v) => v.as_ref(app).should_render(app),
            SettingsPageViewHandle::MCPServers(v) => v.as_ref(app).should_render(app),
        }
    }

    /// Open the invite section of the teams page, optionally with an email to invite.
    /// Open the MCP servers page, optionally to list page or edit page.
    /// If `autoinstall_gallery_title` is provided, triggers auto-install of the specified gallery MCP.
    pub fn open_mcp_servers_page(
        &mut self,
        page: MCPServersSettingsPage,
        autoinstall_gallery_title: Option<&str>,
        ctx: &mut ViewContext<Self>,
    ) {
        // Callers reach this through `Workspace::open_mcp_servers_page`, which
        // has already navigated to the page; only the sub-view selection is
        // left to do.
        if let Some(mcp_page) = self.settings_page(SettingsSection::AgentMCPServers)
            && let SettingsPageViewHandle::MCPServers(view) = &mcp_page.view_handle
        {
            view.update(ctx, |view, ctx| {
                view.update_page(page, ctx);
                if let Some(title) = autoinstall_gallery_title {
                    view.autoinstall_from_gallery(title, ctx);
                }
            })
        }
    }

    /// Updates the PS1 prompt that is shown on the Appearance page.
    fn key_up(&mut self, ctx: &mut ViewContext<Self>) {
        self.cycle_pages(CycleDirection::Up, ctx)
    }

    fn key_down(&mut self, ctx: &mut ViewContext<Self>) {
        self.cycle_pages(CycleDirection::Down, ctx)
    }

    /// Predicate for whether `section` is currently visible in the sidebar
    /// under the active search filter. Mirrors the inline filtering used
    /// when rendering sidebar items so arrow-key navigation stays in sync
    /// with what the user can actually see.
    fn section_passes_search_filter(&self, section: SettingsSection) -> bool {
        self.settings_pages
            .iter()
            .zip(self.pages_filter.iter())
            .any(|(page, match_data)| page.section == section && match_data.is_truthy())
    }

    fn cycle_pages(&mut self, direction: CycleDirection, ctx: &mut ViewContext<Self>) {
        let search_query = self.search_editor.as_ref(ctx).buffer_text(ctx);
        let is_search_active = !search_query.is_empty();

        let stops = visible_nav_sections(&self.nav_items, |section| {
            !is_search_active || self.section_passes_search_filter(section)
        });

        if stops.is_empty() {
            return;
        }

        let next_index = match stops.iter().position(|s| *s == self.current_settings_page) {
            Some(idx) => next_stop_index(idx, stops.len(), direction),
            // Current page is not in the visible nav order, so start over.
            None => 0,
        };
        let target_section = stops[next_index];

        self.set_and_refresh_current_page_internal(target_section, false, false, ctx);
    }

    fn input_tab(&mut self, ctx: &mut ViewContext<Self>) {
        if let Some(current_page) = self.current_settings_page()
            && let SettingsPageViewHandle::Keybindings(view_handle) = &current_page.view_handle
        {
            view_handle.update(ctx, |view, ctx| view.on_tab_pressed(ctx));
        }
    }

    pub fn scroll_to_settings_widget(
        &mut self,
        page: SettingsSection,
        widget_id: &'static str,
        ctx: &mut ViewContext<Self>,
    ) {
        self.set_and_refresh_current_page_internal(page, true, true, ctx);
        if let Some(current_page) = self.current_settings_page() {
            update_page!(
                &current_page.view_handle,
                |view, _| {
                    view.scroll_to_widget(widget_id);
                },
                ctx
            )
        }
    }

    fn debug_settings_action(&mut self, action: &DebugSettingsAction, ctx: &mut ViewContext<Self>) {
        match action {
            DebugSettingsAction::ToggleInitializationBlock => {
                BlockVisibilitySettings::handle(ctx).update(
                    ctx,
                    |block_visibility_settings, ctx| {
                        let _ = block_visibility_settings
                            .should_show_bootstrap_block
                            .toggle_and_save_value(ctx);
                    },
                );
            }
            DebugSettingsAction::ToggleInBandCommandBlocks => {
                BlockVisibilitySettings::handle(ctx).update(
                    ctx,
                    |block_visibility_settings, ctx| {
                        let _ = block_visibility_settings
                            .should_show_in_band_command_blocks
                            .toggle_and_save_value(ctx);
                    },
                );
            }
        }
    }

    fn get_modal_content_for_page(
        &self,
        page_handle: &SettingsPageViewHandle,
        app: &AppContext,
    ) -> Option<Box<dyn Element>> {
        match page_handle {
            SettingsPageViewHandle::MCPServers(view) => {
                view.read(app, |view, _| view.get_modal_content(app))
            }
            _ => None,
        }
    }

    fn render_search_editor(&self, appearance: &Appearance) -> Box<dyn Element> {
        SavePosition::new(
            Container::new(
                Flex::row()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_child(
                        Container::new(
                            ConstrainedBox::new(
                                icons::Icon::SearchSmall
                                    .to_warpui_icon(appearance.theme().active_ui_text_color())
                                    .finish(),
                            )
                            .with_width(16.)
                            .with_height(16.)
                            .finish(),
                        )
                        .with_uniform_margin(4.)
                        .with_margin_right(12.)
                        .finish(),
                    )
                    .with_child(
                        Shrinkable::new(
                            1.,
                            Clipped::new(ChildView::new(&self.search_editor).finish()).finish(),
                        )
                        .finish(),
                    )
                    .finish(),
            )
            .with_margin_left(16.)
            .with_margin_right(16.)
            .with_margin_bottom(8.)
            .finish(),
            SEARCH_EDITOR_POSITION_ID,
        )
        .for_single_frame()
        .finish()
    }

    fn render_search_zero_state(&self, appearance: &Appearance) -> Box<dyn Element> {
        let theme = appearance.theme();
        Container::new(
            Align::new(
                Flex::column()
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_children([
                        Text::new(
                            "No settings match your search.",
                            appearance.ui_font_family(),
                            appearance.ui_font_size(),
                        )
                        .with_style(Properties::default().weight(Weight::Medium))
                        .with_color(theme.sub_text_color(theme.background()).into_solid())
                        .finish(),
                        Text::new(
                            "You may want to try using different keywords or checking for any possible typos.",
                            appearance.ui_font_family(),
                            appearance.ui_font_size(),
                        )
                        .with_color(theme.disabled_ui_text_color().into_solid())
                        .finish(),
                    ])
                    .finish(),
            )
            .finish(),
        )
            .with_uniform_margin(16.)
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
        .with_background(internal_colors::fg_overlay_1(appearance.theme()))
        .finish()
    }
}

#[cfg(feature = "integration_tests")]
impl SettingsView {
    pub fn search_query(&self, app: &AppContext) -> String {
        self.search_editor.as_ref(app).buffer_text(app)
    }
}

impl Entity for SettingsView {
    type Event = SettingsViewEvent;
}

impl View for SettingsView {
    fn ui_name() -> &'static str {
        "SettingsViewInTab"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let settings_pages = self.filtered_pages(app).collect_vec();
        let appearance = Appearance::as_ref(app);

        let (page, current_page_handle) = if settings_pages.is_empty() {
            (self.render_search_zero_state(appearance), None)
        } else {
            match settings_pages
                .iter()
                .find(|(page, _)| page.section == self.current_settings_page)
            {
                None => (Empty::new().finish(), None),
                Some((page, _)) => (page.view_handle.child_view(), Some(&page.view_handle)),
            }
        };

        let theme = appearance.theme();

        let mut buttons = Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_child(self.render_search_editor(appearance));

        for section in self.nav_items.iter().copied() {
            let Some((page, match_data)) =
                settings_pages.iter().find(|(p, _)| p.section == section)
            else {
                continue;
            };
            let page_active = section == self.current_settings_page;
            buttons.add_child(
                SavePosition::new(
                    page.render_page_button(appearance, *match_data, page_active)
                        .on_click(move |ctx, _, _| {
                            ctx.dispatch_typed_action(SettingsAction::SelectAndRefresh(section));
                        })
                        .finish(),
                    &nav_page_position_id(section),
                )
                .for_single_frame()
                .finish(),
            );
        }
        // Footer: "Open settings file" button, or an inline error alert if
        // the workspace-level banner was dismissed. Rendered below the
        // scrollable nav list but inside the same sidebar column so it
        // shares the right-border and SIDEBAR_WIDTH constraint.
        let footer_kind = SettingsFooterKind::choose(
            FeatureFlag::SettingsFile.is_enabled(),
            self.settings_file_error.is_some(),
            self.settings_error_banner_dismissed,
        );
        let footer = render_footer(
            footer_kind,
            appearance,
            self.settings_file_error.as_ref(),
            AISettings::as_ref(app).is_any_ai_enabled(app),
            &self.footer_mouse_states,
        );

        let scrollable = Container::new(
            ClippedScrollable::vertical(
                self.clipped_scroll_state.clone(),
                buttons.finish(),
                ScrollbarWidth::Auto,
                theme.nonactive_ui_detail().into(),
                theme.active_ui_detail().into(),
                Fill::None,
            )
            .finish(),
        )
        .with_padding_top(HEADER_PADDING)
        .finish();

        let sidebar = ConstrainedBox::new(
            Container::new(
                Flex::column()
                    .with_child(Expanded::new(1., scrollable).finish())
                    .with_child(footer)
                    .finish(),
            )
            .with_border(Border::right(SECTION_BORDER_WIDTH).with_border_fill(theme.outline()))
            .finish(),
        )
        .with_width(sidebar_width())
        .finish();

        let row = Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_main_axis_size(MainAxisSize::Max)
            .with_child(Shrinkable::new(1., sidebar).finish())
            .with_child(Shrinkable::new(1., page).finish())
            .finish();

        let mut stack = Stack::new().with_child(
            EventHandler::new(
                EventHandler::new(row)
                    .with_always_handle()
                    .on_left_mouse_down(|ctx, _app, _pos| {
                        ctx.dispatch_typed_action(SettingsAction::FocusSelf);
                        DispatchEventResult::PropagateToParent
                    })
                    .finish(),
            )
            .on_right_mouse_down(|event, _app, position| {
                let Some(parent_bounds) = event.element_position_by_id(POSITION_ID) else {
                    return DispatchEventResult::PropagateToParent;
                };
                let offset = position - parent_bounds.origin();
                event.dispatch_typed_action(SettingsAction::OpenContextMenu(offset));
                DispatchEventResult::StopPropagation
            })
            .finish(),
        );

        if let Some(position) = &self.context_menu_state {
            stack.add_positioned_overlay_child(
                ChildView::new(&self.context_menu).finish(),
                OffsetPositioning::offset_from_parent(
                    *position,
                    ParentOffsetBounds::WindowByPosition,
                    ParentAnchor::TopLeft,
                    ChildAnchor::TopLeft,
                ),
            );
        }

        if let Some(modal_content) =
            current_page_handle.and_then(|handle| self.get_modal_content_for_page(handle, app))
        {
            stack.add_positioned_overlay_child(
                modal_content,
                OffsetPositioning::offset_from_parent(
                    pathfinder_geometry::vector::vec2f(0., 0.),
                    ParentOffsetBounds::WindowByPosition,
                    ParentAnchor::Center,
                    ChildAnchor::Center,
                ),
            );
        }

        // Render environment setup mode selector overlay when open.
        if let Some(selector_handle) = self
            .environments_page_handle
            .as_ref(app)
            .environment_setup_mode_selector_handle()
        {
            stack.add_child(ChildView::new(selector_handle).finish());
        }

        // Render agent-assisted environment modal overlay when open.
        if let Some(modal_handle) = self
            .environments_page_handle
            .as_ref(app)
            .agent_assisted_environment_modal_handle(app)
        {
            stack.add_child(ChildView::new(modal_handle).finish());
        }

        SavePosition::new(stack.finish(), POSITION_ID).finish()
    }
}

impl TypedActionView for SettingsView {
    type Action = SettingsAction;

    fn handle_action(&mut self, action: &SettingsAction, ctx: &mut ViewContext<Self>) {
        match action {
            SettingsAction::SelectAndRefresh(section) => {
                self.set_and_refresh_current_page_internal(*section, false, true, ctx);

                if *section == SettingsSection::AgentMCPServers {
                    send_telemetry_from_ctx!(
                        TelemetryEvent::MCPServerCollectionPaneOpened {
                            entrypoint: MCPServerCollectionPaneEntrypoint::MCPSettingsTab,
                        },
                        ctx
                    );
                }
            }
            SettingsAction::AppearancePageToggle(appearance_action) => {
                if let Some(appearance_page) = self.settings_page(SettingsSection::Appearance)
                    && let SettingsPageViewHandle::Appearance(view) = &appearance_page.view_handle
                {
                    view.update(ctx, |view, ctx| {
                        view.handle_action(appearance_action, ctx);
                    })
                }
            }
            SettingsAction::ShellIntegrationPageToggle(shell_integration_action) => {
                if let Some(shell_integration_page) =
                    self.settings_page(SettingsSection::ShellIntegration)
                    && let SettingsPageViewHandle::ShellIntegration(view) =
                        &shell_integration_page.view_handle
                {
                    view.update(ctx, |view, ctx| {
                        view.handle_action(shell_integration_action, ctx);
                    })
                }
            }
            SettingsAction::Tab => self.input_tab(ctx),
            SettingsAction::Split(direction) => {
                let event = match direction {
                    Direction::Left => PaneEvent::SplitLeft(None),
                    Direction::Right => PaneEvent::SplitRight(None),
                    Direction::Up => PaneEvent::SplitUp(None),
                    Direction::Down => PaneEvent::SplitDown(None),
                };
                ctx.emit(SettingsViewEvent::Pane(event));
            }
            SettingsAction::ToggleMaximizePane => {
                ctx.emit(SettingsViewEvent::Pane(PaneEvent::ToggleMaximized))
            }
            SettingsAction::Close => ctx.emit(SettingsViewEvent::Pane(PaneEvent::Close)),
            SettingsAction::OpenContextMenu(position) => {
                self.context_menu_state = Some(*position);
                let menu_items = self.context_menu_items(ctx);
                self.context_menu.update(ctx, move |menu, ctx| {
                    menu.set_items(menu_items, ctx);
                    ctx.notify();
                });
                ctx.notify();
            }
            SettingsAction::FocusSelf => ctx.emit(SettingsViewEvent::Pane(PaneEvent::FocusSelf)),
            SettingsAction::Up => self.key_up(ctx),
            SettingsAction::Down => self.key_down(ctx),
            SettingsAction::Debug(action) => self.debug_settings_action(action, ctx),
        }
    }
}

impl BackingView for SettingsView {
    type PaneHeaderOverflowMenuAction = SettingsAction;
    type CustomAction = ();
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        action: &Self::PaneHeaderOverflowMenuAction,
        ctx: &mut warpui::ViewContext<Self>,
    ) {
        self.handle_action(action, ctx)
    }

    fn close(&mut self, ctx: &mut warpui::ViewContext<Self>) {
        ctx.emit(SettingsViewEvent::Pane(PaneEvent::Close));
    }

    fn focus_contents(&mut self, ctx: &mut warpui::ViewContext<Self>) {
        ctx.focus(&self.search_editor)
    }

    fn render_header_content(
        &self,
        _ctx: &view::HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> view::HeaderContent {
        view::HeaderContent::simple("Settings")
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        self.focus_handle = Some(focus_handle);
    }
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
