//! Dialog — real GPUI component backed by DialogSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{DialogKind, DialogSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI dialog component backed by `DialogSpec`.
///
/// Renders the dialog surface (elevated card with title/description).
/// The parent is responsible for conditionally rendering based on `spec.current_open()`.
pub struct Dialog {
    spec: DialogSpec,
    theme: GpuiThemeProvider,
    /// Actions slot — typically buttons rendered by the parent.
    actions: Option<AnyElement>,
    /// Content slot — body content between description and actions.
    content: Option<AnyElement>,
}

impl std::ops::Deref for Dialog {
    type Target = DialogSpec;
    fn deref(&self) -> &DialogSpec { &self.spec }
}

impl Dialog {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DialogSpec::new(), theme: theme.clone(), actions: None, content: None }
    }

    pub fn from_spec(spec: DialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            actions: None,
            content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn kind(mut self, v: DialogKind) -> Self { self.spec.kind = v; self }
    pub fn dismiss_on_escape(mut self, v: bool) -> Self { self.spec.dismiss_on_escape = v; self }
    pub fn dismiss_on_backdrop(mut self, v: bool) -> Self { self.spec.dismiss_on_backdrop = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    /// Add body content between the description and actions.
    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    /// Add an actions row (e.g., Cancel + Confirm buttons).
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }
}

impl IntoElement for Dialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let actions_gap = resolve_px(theme, "semantic.space.inline.sm");
        let panel_x = resolve_px(theme, "semantic.space.panel.x");
        let panel_y = resolve_px(theme, "semantic.space.panel.y");

        let surface_bg = resolve_color(theme, spec.surface_fill_token());
        let panel = resolve_color(theme, "semantic.color.background.panel");
        let border_raw = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        // Contract: radius-surface
        let radius = resolve_radius(theme, "semantic.radius.surface");

        // Contract: border 72% border-default, bg 98% elevated
        let border = color_mix(border_raw, panel, 0.72);
        let bg = color_mix(surface_bg, panel, 0.98);

        let mut dialog = div()
            .px(panel_x)
            .py(panel_y)
            .rounded(radius)
            .bg(bg)
            .border_1()
            .border_color(border)
            .shadow_lg()
            .flex()
            .flex_col()
            // Contract: header gap 0.5rem (8px)
            .gap(px(8.0));

        // Contract: title font 1.125rem (18px), weight 600
        if let Some(ref title) = spec.title {
            dialog = dialog.child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Contract: description font 0.875rem (14px)
        if let Some(ref description) = spec.description {
            dialog = dialog.child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        // Content slot — body content between description and actions
        if let Some(content) = self.content {
            dialog = dialog.child(content);
        }

        // Actions slot — Contract: flex-wrap, justify-end
        if let Some(actions) = self.actions {
            dialog = dialog.child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(actions_gap)
                    .justify_end()
                    .pt(px(8.0)) // Visual separation from content
                    .child(actions),
            );
        }

        // Backdrop overlay — full-viewport scrim with centered dialog
        let backdrop = div()
            .id("pug-dialog-backdrop")
            .absolute()
            .inset_0()
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .flex()
            .items_center()
            .justify_center()
            .occlude()
            .child(dialog);

        backdrop.into_any_element()
    }
}
