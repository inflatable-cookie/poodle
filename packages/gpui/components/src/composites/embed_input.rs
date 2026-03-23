//! EmbedInput — URL input for embedding external content backed by EmbedInputSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::EmbedInputSpec;
use poodle_primitives::ValidationState;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct EmbedInput {
    spec: EmbedInputSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for EmbedInput {
    type Target = EmbedInputSpec;
    fn deref(&self) -> &EmbedInputSpec { &self.spec }
}

impl EmbedInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: EmbedInputSpec::new(), theme: theme.clone() }
    }
    pub fn from_spec(spec: EmbedInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for EmbedInput {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.control");
        let text_color = resolve_color(theme, "semantic.color.text.primary");
        let placeholder_color = resolve_color(theme, "semantic.color.text.secondary");
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let danger_color = resolve_color(theme, "semantic.color.status.danger");
        let success_color = resolve_color(theme, "semantic.color.status.success");
        let warning_color = resolve_color(theme, "semantic.color.status.warning");

        let display = if spec.value.is_empty() {
            spec.placeholder.as_deref().unwrap_or("Paste URL or embed code...")
        } else {
            &spec.value
        };
        let color = if spec.value.is_empty() { placeholder_color } else { text_color };

        // Multi-line text area (min 3 rows ~72px) for URL / embed code
        let mut textarea = div()
            .id("poodle-embed-input")
            .focusable()
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .min_h(px(72.0))
            .w_full()
            .px(px(12.0))
            .py(px(8.0))
            .flex()
            .flex_col()
            .items_start()
            .overflow_hidden()
            .text_size(px(14.0))
            .line_height(relative(1.5))
            .text_color(color)
            .focus(move |s| s.border_color(focus_ring))
            .child(display.to_string());

        if spec.is_disabled {
            textarea = textarea
                .opacity(resolve_opacity(theme, "semantic.state.opacity.disabled"))
                .cursor_not_allowed();
        }

        // Status area below the textarea: error (red), success (green), pending (warning), or provider info
        let (status_color, status_text) = if spec.is_loading {
            (placeholder_color, Some("Resolving...".to_string()))
        } else {
            match spec.validation_state {
                ValidationState::Invalid => {
                    (danger_color, Some("Invalid URL or embed code".to_string()))
                }
                ValidationState::Valid => {
                    (success_color, Some("Valid embed".to_string()))
                }
                ValidationState::Pending => {
                    (warning_color, Some("Validating...".to_string()))
                }
                ValidationState::None => (placeholder_color, None),
            }
        };

        let mut wrapper = div().flex().flex_col().gap(px(4.0)).w_full().child(textarea);

        if let Some(text) = status_text {
            let status_indicator = match spec.validation_state {
                ValidationState::Invalid => "\u{2022} ",
                ValidationState::Valid => "\u{2713} ",
                ValidationState::Pending => "\u{2022} ",
                _ => "",
            };
            let status_area = div()
                .min_h(px(20.0))
                .px(px(4.0))
                .flex()
                .items_center()
                .gap(px(4.0))
                .text_size(px(12.0))
                .text_color(status_color)
                .child(format!("{}{}", status_indicator, text));
            wrapper = wrapper.child(status_area);
        }

        wrapper.into_any_element()
    }
}
