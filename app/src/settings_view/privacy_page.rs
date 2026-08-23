//! Settings UI for privacy controls that belong to this machine.
use std::cell::RefCell;
use std::collections::HashMap;

use settings::{Setting as _, ToggleableSetting as _};
use warp_errors::report_if_error;
use warpui::elements::{Element, MouseStateHandle};
use warpui::ui_components::components::UiComponent;
use warpui::ui_components::switch::SwitchStateHandle;
use warpui::windowing::WindowManager;
use warpui::{AppContext, Entity, SingletonEntity, TypedActionView, View, ViewContext, ViewHandle};

use super::settings_page::{
    LocalOnlyIconState, MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle,
    SettingsWidget, render_body_item,
};
use super::{SettingsSection, ToggleState};
use crate::appearance::Appearance;
use crate::settings::{DevicePrivacySettings, SecureKeyboardEntry};

#[derive(Clone, Debug, PartialEq)]
pub enum PrivacySettingsPageAction {
    ToggleSecureKeyboardEntry,
}

pub struct PrivacySettingsPageView {
    page: PageType<Self>,
    local_only_icon_tooltip_states: RefCell<HashMap<String, MouseStateHandle>>,
}

impl PrivacySettingsPageView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        let widgets: Vec<Box<dyn SettingsWidget<View = Self>>> =
            vec![Box::new(SecureKeyboardEntryWidget::default())];

        Self {
            page: PageType::new_uncategorized(widgets, Some("Privacy")),
            local_only_icon_tooltip_states: RefCell::new(HashMap::new()),
        }
    }
}

impl Entity for PrivacySettingsPageView {
    type Event = ();
}

impl TypedActionView for PrivacySettingsPageView {
    type Action = PrivacySettingsPageAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            PrivacySettingsPageAction::ToggleSecureKeyboardEntry => {
                DevicePrivacySettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.secure_keyboard_entry.toggle_and_save_value(ctx));
                });
                // Nothing outside this process remembers the flag, so it has to move
                // now rather than on the next launch.
                let enabled = *DevicePrivacySettings::as_ref(ctx)
                    .secure_keyboard_entry
                    .value();
                WindowManager::as_ref(ctx).set_secure_keyboard_entry(enabled);
                ctx.notify();
            }
        }
    }
}

impl View for PrivacySettingsPageView {
    fn ui_name() -> &'static str {
        "PrivacySettingsPage"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

impl SettingsPageMeta for PrivacySettingsPageView {
    fn section() -> SettingsSection {
        SettingsSection::Privacy
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        cfg!(target_os = "macos")
    }

    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData {
        self.page.update_filter(query, ctx)
    }

    fn scroll_to_widget(&mut self, widget_id: &'static str) {
        self.page.scroll_to_widget(widget_id)
    }

    fn clear_highlighted_widget(&mut self) {
        self.page.clear_highlighted_widget();
    }
}

impl From<ViewHandle<PrivacySettingsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<PrivacySettingsPageView>) -> Self {
        SettingsPageViewHandle::Privacy(view_handle)
    }
}

#[derive(Default)]
struct SecureKeyboardEntryWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for SecureKeyboardEntryWidget {
    type View = PrivacySettingsPageView;

    fn search_terms(&self) -> &str {
        "secure keyboard entry keylogger password sudo input monitoring"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let enabled = DevicePrivacySettings::as_ref(app)
            .secure_keyboard_entry
            .value();

        render_body_item::<PrivacySettingsPageAction>(
            "Secure keyboard entry".into(),
            None,
            LocalOnlyIconState::for_setting(
                SecureKeyboardEntry::storage_key(),
                SecureKeyboardEntry::sync_to_cloud(),
                &mut view.local_only_icon_tooltip_states.borrow_mut(),
                app,
            ),
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*enabled)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(PrivacySettingsPageAction::ToggleSecureKeyboardEntry);
                })
                .finish(),
            Some(
                "Stops other apps from reading what you type here, the same protection \
                 Terminal and iTerm2 offer. While it is on, the emoji picker and any input \
                 method that runs outside this app stop working in this window."
                    .to_string(),
            ),
        )
    }
}
