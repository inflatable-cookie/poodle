//! Drawer — real GPUI component backed by DrawerSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{DrawerEdge, DrawerSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_px};

/// A real GPUI drawer component backed by `DrawerSpec`.
///
/// Renders a side panel. The parent controls the `open` state.
pub struct Drawer {
    spec: DrawerSpec,
    theme: GpuiThemeProvider,
    /// The content to show inside the drawer panel.
    content: Option<AnyElement>,
    /// The main area content (shown next to the drawer).
    main_content: Option<AnyElement>,
}

impl std::ops::Deref for Drawer {
    type Target = DrawerSpec;
    fn deref(&self) -> &DrawerSpec { &self.spec }
}

impl Drawer {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DrawerSpec::new(), theme: theme.clone(), content: None, main_content: None }
    }

    pub fn from_spec(spec: DrawerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            main_content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn edge(mut self, v: DrawerEdge) -> Self { self.spec.edge = v; self }
    pub fn modal(mut self, v: bool) -> Self { self.spec.is_modal = v; self }
    pub fn dismiss_on_escape(mut self, v: bool) -> Self { self.spec.dismiss_on_escape = v; self }
    pub fn dismiss_on_backdrop(mut self, v: bool) -> Self { self.spec.dismiss_on_backdrop = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn with_main_content(mut self, main: impl IntoElement) -> Self {
        self.main_content = Some(main.into_any_element());
        self
    }
}

impl IntoElement for Drawer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");
        let panel_padding = resolve_px(theme, "semantic.space.panel.x");

        let surface_raw = resolve_color(theme, spec.surface_fill_token());
        let panel = resolve_color(theme, "semantic.color.background.panel");
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        // Contract: bg = color-mix 98% surface
        let surface_bg = color_mix(surface_raw, panel, 0.98);

        let is_left = spec.edge == DrawerEdge::Left || spec.edge == DrawerEdge::Top;

        // Contract: drawer radius = 0, min-width 200px, shadow
        let mut drawer_panel = div()
            .min_w(px(200.0))
            .h_full()
            .bg(surface_bg)
            .p(panel_padding)
            .flex()
            .flex_col()
            .gap(stack_gap)
            .shadow_md();

        // Contract: border on side facing main area only
        if is_left {
            drawer_panel = drawer_panel.border_r_1().border_color(border);
        } else {
            drawer_panel = drawer_panel.border_l_1().border_color(border);
        }

        // Contract: title font 1rem (16px), weight 600
        if let Some(ref title) = spec.title {
            drawer_panel = drawer_panel.child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Contract: description font 0.875rem (14px)
        if let Some(ref description) = spec.description {
            drawer_panel = drawer_panel.child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        // Content
        if let Some(content) = self.content {
            drawer_panel = drawer_panel.child(content);
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
                        .text_size(px(12.0))
                        .text_color(text_secondary)
                        .child("Main area"),
                )
        };

        let mut row = div().flex().h_full();

        if is_left {
            row = row.child(drawer_panel).child(main);
        } else {
            row = row.child(main).child(drawer_panel);
        }

        row.into_any_element()
    }
}
