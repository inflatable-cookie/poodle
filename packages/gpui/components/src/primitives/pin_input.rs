//! PinInput — real GPUI component backed by PinInputSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::PinInputSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI pin/OTP input component with fixed-length digit cells backed by `PinInputSpec`.
pub struct PinInput {
    spec: PinInputSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for PinInput {
    type Target = PinInputSpec;
    fn deref(&self) -> &PinInputSpec { &self.spec }
}

impl PinInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: PinInputSpec::default(), theme: theme.clone() }
    }

    pub fn from_spec(spec: PinInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn length(mut self, v: usize) -> Self { self.spec.length = v; self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = v.into(); self }
    pub fn masked(mut self, v: bool) -> Self { self.spec.is_masked = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

}

impl IntoElement for PinInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let control_height = resolve_px(theme, "semantic.size.control.height");
        // Cell width slightly wider than height for pin cells
        let cell_width = resolve_px(theme, "semantic.size.control.minWidth");

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let chars: Vec<char> = spec.value.chars().collect();

        let mut row = div().flex().gap(inline_gap);

        for i in 0..spec.length {
            let ch = chars.get(i).copied();
            let display = match ch {
                Some(_) if spec.is_masked => "\u{2022}".to_string(),
                Some(c) => c.to_string(),
                None => String::new(),
            };

            let cell = div()
                .w(cell_width)
                .h(control_height)
                .rounded(control_radius)
                .bg(surface_bg)
                .border_1()
                .border_color(border)
                .flex()
                .items_center()
                .justify_center()
                .text_lg()
                .text_color(text_primary)
                .child(display);

            row = row.child(cell);
        }

        let mut wrapper = div().flex().flex_col().child(row);

        if spec.is_disabled {
            wrapper = wrapper.opacity(disabled_opacity);
        }

        wrapper.into_any_element()
    }
}
