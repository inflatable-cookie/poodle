//! PugPanelHeader — real GPUI component backed by PanelHeaderSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_workstation::PanelHeaderSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI panel header bar backed by `PanelHeaderSpec`.
///
/// Renders a panel title bar with optional collapse toggle,
/// title, tabs slot, and utility actions slot.
pub struct PugPanelHeader {
    spec: PanelHeaderSpec,
    theme: GpuiThemeProvider,
    /// Slot for tab elements within the header.
    tabs: Option<AnyElement>,
    /// Slot for utility action buttons (e.g., close, settings).
    utility_actions: Option<AnyElement>,
    on_collapse_toggle: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl PugPanelHeader {
    pub fn new(spec: PanelHeaderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            tabs: None,
            utility_actions: None,
            on_collapse_toggle: None,
        }
    }

    pub fn with_tabs(mut self, tabs: impl IntoElement) -> Self {
        self.tabs = Some(tabs.into_any_element());
        self
    }

    pub fn with_utility_actions(mut self, actions: impl IntoElement) -> Self {
        self.utility_actions = Some(actions.into_any_element());
        self
    }

    pub fn on_collapse_toggle(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_collapse_toggle = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugPanelHeader {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let bg = resolve_color(theme, spec.background_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let mut header = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(32.0))
            .px(px(8.0))
            .bg(bg)
            .border_b_1()
            .border_color(border);

        // Left section: collapse toggle + title + tabs
        let mut left = div().flex().items_center().gap(px(6.0)).flex_1().min_w_0();

        // Collapse chevron
        if spec.is_collapsible {
            let chevron_id = SharedString::from("pug-panel-header-collapse");
            left = left.child(
                div()
                    .id(chevron_id)
                    .text_xs()
                    .text_color(text_secondary)
                    .cursor_pointer()
                    .hover(|s| s.text_color(text_primary))
                    .child("\u{25B6}"),
            );
        }

        // Title
        if let Some(ref title) = spec.title {
            left = left.child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(if spec.is_active {
                        text_primary
                    } else {
                        text_secondary
                    })
                    .overflow_x_hidden()
                    .whitespace_nowrap()
                    .child(title.clone()),
            );
        }

        // Tabs slot
        if let Some(tabs) = self.tabs {
            left = left.child(tabs);
        }

        header = header.child(left);

        // Right section: utility actions
        if let Some(utility_actions) = self.utility_actions {
            header = header.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .flex_shrink_0()
                    .child(utility_actions),
            );
        }

        header.into_any_element()
    }
}
