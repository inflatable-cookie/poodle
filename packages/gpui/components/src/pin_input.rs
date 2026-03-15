//! PugPinInput — real GPUI component backed by PinInputSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::PinInputSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI pin/OTP input component with fixed-length digit cells backed by `PinInputSpec`.
pub struct PugPinInput {
    spec: PinInputSpec,
    theme: GpuiThemeProvider,
}

impl PugPinInput {
    pub fn new(spec: PinInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugPinInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");

        let chars: Vec<char> = spec.value.chars().collect();

        let mut row = div().flex().gap(px(8.0));

        for i in 0..spec.length {
            let ch = chars.get(i).copied();
            let display = match ch {
                Some(_) if spec.is_masked => "\u{2022}".to_string(),
                Some(c) => c.to_string(),
                None => String::new(),
            };

            let cell = div()
                .w(px(40.0))
                .h(px(44.0))
                .rounded(px(6.0))
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
            wrapper = wrapper.opacity(0.48);
        }

        wrapper.into_any_element()
    }
}
