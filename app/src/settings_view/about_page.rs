use warpui::assets::asset_cache::AssetSource;
use warpui::elements::{
    Align, CacheOption, ConstrainedBox, Container, CrossAxisAlignment, Element, Flex, Image,
    MainAxisAlignment, MouseStateHandle, ParentElement, Wrap,
};
use warpui::ui_components::components::UiComponent;
use warpui::{AppContext, Entity, View, ViewContext, ViewHandle};

use super::SettingsSection;
use super::settings_page::{
    MatchData, PageType, SettingsPageEvent, SettingsPageMeta, SettingsPageViewHandle,
    SettingsWidget,
};
use crate::appearance::Appearance;
use crate::channel::ChannelState;
use crate::workspace::WorkspaceAction;

pub struct AboutPageView {
    page: PageType<Self>,
}

impl AboutPageView {
    pub fn new(_ctx: &mut ViewContext<AboutPageView>) -> Self {
        AboutPageView {
            page: PageType::new_monolith(AboutPageWidget::default(), None, false),
        }
    }
}

impl Entity for AboutPageView {
    type Event = SettingsPageEvent;
}

impl View for AboutPageView {
    fn ui_name() -> &'static str {
        "AboutPage"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

#[derive(Default)]
struct AboutPageWidget {
    copy_version_button_mouse_state: MouseStateHandle,
}

impl SettingsWidget for AboutPageWidget {
    type View = AboutPageView;

    fn search_terms(&self) -> &str {
        "about warp version"
    }

    fn render(
        &self,
        _view: &AboutPageView,
        appearance: &Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();

        let version = ChannelState::app_version().unwrap_or("dev build");
        // Releases are named as well as numbered, so show both when the build
        // carries a codename.
        let version_label = match ChannelState::app_codename() {
            Some(codename) => format!("{version}  \u{201c}{codename}\u{201d}"),
            None => version.to_string(),
        };

        let version_text = ui_builder
            .span(version_label)
            .with_soft_wrap()
            .build()
            .with_margin_top(16.)
            .finish();

        let copy_version_icon = appearance
            .ui_builder()
            .copy_button(16., self.copy_version_button_mouse_state.clone())
            .build()
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(WorkspaceAction::CopyVersion(version));
            })
            .finish();

        let version_row = Wrap::row()
            .with_main_axis_alignment(MainAxisAlignment::Center)
            .with_children([
                version_text,
                Container::new(copy_version_icon)
                    .with_margin_top(16.)
                    .with_padding_left(6.)
                    .finish(),
            ]);

        Align::new(
            Flex::column()
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(
                    ConstrainedBox::new(
                        Image::new(
                            AssetSource::Bundled {
                                path: "bundled/png/nerminal-icon.png",
                            },
                            CacheOption::BySize,
                        )
                        .finish(),
                    )
                    .with_max_height(96.)
                    .with_max_width(96.)
                    .finish(),
                )
                .with_child(
                    ui_builder
                        .span("Nerminal".to_string())
                        .build()
                        .with_margin_top(12.)
                        .finish(),
                )
                .with_child(version_row.finish())
                .with_child(
                    ui_builder
                        .span(
                            "A fork of Warp by Denver Technologies, Inc. Free software under AGPL-3.0-only.",
                        )
                        .build()
                        .with_margin_top(16.)
                        .finish(),
                )
                .finish(),
        )
        .finish()
    }
}

impl SettingsPageMeta for AboutPageView {
    fn section() -> SettingsSection {
        SettingsSection::About
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        true
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

impl From<ViewHandle<AboutPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<AboutPageView>) -> Self {
        SettingsPageViewHandle::About(view_handle)
    }
}
