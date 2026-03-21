//! ResizeHandle — real GPUI component backed by ResizeHandleSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{Orientation, ResizeHandleSpec};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity};

/// A real GPUI resize handle component backed by `ResizeHandleSpec`.
///
/// Renders a hit target area containing a thin visual affordance line.
/// - Horizontal orientation: vertical line, col-resize cursor, drag left/right.
/// - Vertical orientation: horizontal line, row-resize cursor, drag up/down.
pub struct ResizeHandle {
    spec: ResizeHandleSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for ResizeHandle {
    type Target = ResizeHandleSpec;
    fn deref(&self) -> &ResizeHandleSpec { &self.spec }
}

impl ResizeHandle {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ResizeHandleSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: ResizeHandleSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn orientation(mut self, v: Orientation) -> Self { self.spec.orientation = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn aria_value_now(mut self, v: f32) -> Self { self.spec.aria_value_now = Some(v); self }
    pub fn aria_value_min(mut self, v: f32) -> Self { self.spec.aria_value_min = v; self }
    pub fn aria_value_max(mut self, v: f32) -> Self { self.spec.aria_value_max = v; self }

}

impl IntoElement for ResizeHandle {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Resolve tokens
        let border_color = resolve_color(theme, spec.border_color_token());
        let hover_color = resolve_color(theme, spec.hover_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        // Idle line color: 82% border-default mixed with transparent (Svelte: color-mix 82%)
        let idle_line_color = color_mix(border_color, hsla(0.0, 0.0, 0.0, 0.0), 0.82);

        let is_horizontal = spec.orientation == Orientation::Horizontal;

        // Hit target container: 8px perpendicular to resize direction.
        // Contains a centered 2px visual affordance line.
        // On hover, the container bg changes to a subtle accent to give feedback
        // across the full 8px hit area.
        let mut container = div()
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0();

        // Visual affordance: 2px line centered in hit target
        let line = if is_horizontal {
            div().w(px(2.0)).h_full().rounded(px(999.0)).bg(idle_line_color)
        } else {
            div().w_full().h(px(2.0)).rounded(px(999.0)).bg(idle_line_color)
        };

        if is_horizontal {
            container = container.w(px(8.0)).h_full().cursor_col_resize();
        } else {
            container = container.w_full().h(px(8.0)).cursor_row_resize();
        }

        // Disabled state: reduced opacity, default cursor, no hover
        if spec.is_disabled {
            container = container.opacity(disabled_opacity).cursor_default();
        } else {
            // Hover: accent color fills the hit target area as visual feedback.
            // The accent bg covers the 8px hit area with a subtle tint, and
            // the 2px idle line remains visible underneath.
            let accent_hover = hover_color.opacity(0.3);
            container = container.hover(move |s| s.bg(accent_hover));
        }

        container = container.child(line);

        container.into_any_element()
    }
}
