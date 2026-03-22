//! Callout — real GPUI component backed by CallOutSpec (contract: callout).

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CallOutSpec, StatusTone};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI call-out component backed by `CallOutSpec`.
pub struct Callout {
    spec: CallOutSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Callout {
    type Target = CallOutSpec;
    fn deref(&self) -> &CallOutSpec { &self.spec }
}

impl Callout {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CallOutSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: CallOutSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tone(mut self, v: StatusTone) -> Self { self.spec.tone = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn content(mut self, v: impl Into<String>) -> Self { self.spec.content = Some(v.into()); self }

}

impl IntoElement for Callout {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let panel_x = resolve_px(theme, "semantic.space.panel.x");
        let panel_y = resolve_px(theme, "semantic.space.panel.y");

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let radius = resolve_radius(theme, "semantic.radius.surface");

        // Contract: left border 3px, tone-colored bg at 10% opacity
        let mut el = div()
            .w_full()
            .px(panel_x)
            .py(panel_y)
            .rounded(radius)
            .bg(fill.opacity(0.1))
            .border_l(px(3.0))
            .border_color(border)
            .flex()
            .flex_col()
            .gap(px(4.0));

        // Contract: title font 0.875rem (14px), weight 600
        if let Some(ref title) = spec.title {
            el = el.child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Contract: content font 0.875rem (14px)
        if let Some(ref content) = spec.content {
            el = el.child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_secondary)
                    .child(content.clone()),
            );
        }

        el.into_any_element()
    }
}
