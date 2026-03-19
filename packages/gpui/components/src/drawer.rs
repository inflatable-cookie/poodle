//! PugDrawer — real GPUI component backed by DrawerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{DrawerEdge, DrawerSpec};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI drawer component backed by `DrawerSpec`.
///
/// Renders a side panel. The parent controls the `open` state.
pub struct PugDrawer {
    spec: DrawerSpec,
    theme: GpuiThemeProvider,
    /// The content to show inside the drawer panel.
    content: Option<AnyElement>,
    /// The main area content (shown next to the drawer).
    main_content: Option<AnyElement>,
}

impl PugDrawer {
    pub fn new(spec: DrawerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            main_content: None,
        }
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn with_main_content(mut self, main: impl IntoElement) -> Self {
        self.main_content = Some(main.into_any_element());
        self
    }
}

impl IntoElement for PugDrawer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let panel_padding = resolve_px(theme, "semantic.space.panel.x");
        let surface_radius = resolve_radius(theme, "semantic.radius.surface");

        let surface_bg = resolve_color(theme, spec.surface_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let is_left = spec.edge == DrawerEdge::Left || spec.edge == DrawerEdge::Top;

        // Drawer panel — content-driven width via flex
        let mut panel = div()
            .min_w(px(200.0))
            .h_full()
            .bg(surface_bg)
            .rounded(surface_radius)
            .p(panel_padding)
            .flex()
            .flex_col()
            .gap(inline_gap);

        // Border on the side facing the main area
        if is_left {
            panel = panel.border_r_1().border_color(border);
        } else {
            panel = panel.border_l_1().border_color(border);
        }

        // Title
        if let Some(ref title) = spec.title {
            panel = panel.child(
                div()
                    .text_sm()
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Description
        if let Some(ref description) = spec.description {
            panel = panel.child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        // Content
        if let Some(content) = self.content {
            panel = panel.child(content);
        }

        // Main area
        let main = if let Some(main_content) = self.main_content {
            div().flex_1().flex().items_center().justify_center().child(main_content)
        } else {
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary)
                        .child("Main area"),
                )
        };

        let mut row = div().flex().h_full();

        if is_left {
            row = row.child(panel).child(main);
        } else {
            row = row.child(main).child(panel);
        }

        row.into_any_element()
    }
}
