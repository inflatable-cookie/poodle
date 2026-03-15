//! PugProjectHeader — real GPUI component backed by ProjectHeaderSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_workstation::ProjectHeaderSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI project header backed by `ProjectHeaderSpec`.
///
/// Renders the project name, optional subtitle, dirty-state indicator,
/// status items slot, and project-level actions slot.
pub struct PugProjectHeader {
    spec: ProjectHeaderSpec,
    theme: GpuiThemeProvider,
    /// Slot for action buttons (e.g., save, share).
    actions: Option<AnyElement>,
    /// Slot for status items (e.g., sync indicator).
    status_items: Option<AnyElement>,
}

impl PugProjectHeader {
    pub fn new(spec: ProjectHeaderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            actions: None,
            status_items: None,
        }
    }

    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }

    pub fn with_status_items(mut self, items: impl IntoElement) -> Self {
        self.status_items = Some(items.into_any_element());
        self
    }
}

impl IntoElement for PugProjectHeader {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let bg = resolve_color(theme, "semantic.color.background.panel");

        let mut header = div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(36.0))
            .px(px(12.0))
            .bg(bg)
            .border_b_1()
            .border_color(border);

        // Left section: title + dirty + subtitle
        let mut left = div().flex().items_center().gap(px(6.0)).flex_1().min_w_0();

        // Title with dirty indicator
        let mut title_el = div()
            .flex()
            .items_center()
            .gap(px(4.0));

        title_el = title_el.child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .overflow_x_hidden()
                .whitespace_nowrap()
                .child(spec.title.clone()),
        );

        if spec.is_dirty {
            let dirty_color = resolve_color(theme, spec.dirty_color_token());
            title_el = title_el.child(
                div()
                    .w(px(6.0))
                    .h(px(6.0))
                    .rounded_full()
                    .bg(dirty_color),
            );
        }

        left = left.child(title_el);

        // Subtitle
        if let Some(ref subtitle) = spec.subtitle {
            left = left.child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .overflow_x_hidden()
                    .whitespace_nowrap()
                    .child(subtitle.clone()),
            );
        }

        // Status items
        if let Some(status_items) = self.status_items {
            left = left.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(status_items),
            );
        }

        header = header.child(left);

        // Right section: actions
        if let Some(actions) = self.actions {
            header = header.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .flex_shrink_0()
                    .child(actions),
            );
        }

        header.into_any_element()
    }
}
