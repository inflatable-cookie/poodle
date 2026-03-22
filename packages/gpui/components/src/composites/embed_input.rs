//! EmbedInput — URL input for embedding external content backed by EmbedInputSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::EmbedInputSpec;
use pug_primitives::ValidationState;
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
        let display = if spec.value.is_empty() { spec.placeholder.as_deref().unwrap_or("Paste URL...") } else { &spec.value };
        let color = if spec.value.is_empty() { placeholder_color } else { text_color };

        // Multi-line text area (3 rows) instead of single-line input
        let mut textarea = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .min_h(px(72.0)).px(px(12.0)).py(px(8.0))
            .flex().items_start()
            .text_size(px(14.0)).text_color(color)
            .child(display.to_string());
        if spec.is_disabled {
            textarea = textarea.opacity(resolve_opacity(theme, "semantic.state.opacity.disabled"));
        }

        // Status area below the input
        let status_color = match spec.validation_state {
            ValidationState::Invalid => resolve_color(theme, "semantic.color.status.danger"),
            ValidationState::Valid => resolve_color(theme, "semantic.color.status.success"),
            ValidationState::Pending => resolve_color(theme, "semantic.color.status.warning"),
            ValidationState::None => placeholder_color,
        };
        let mut status_area = div()
            .h(px(20.0)).px(px(4.0))
            .flex().items_center().gap(px(6.0))
            .text_xs().text_color(status_color);
        if spec.is_loading {
            status_area = status_area.child("Loading...");
        } else {
            match spec.validation_state {
                ValidationState::Invalid => { status_area = status_area.child("Invalid URL"); }
                ValidationState::Valid => { status_area = status_area.child("Valid"); }
                ValidationState::Pending => { status_area = status_area.child("Validating..."); }
                ValidationState::None => {}
            }
        }

        div().flex().flex_col().gap(px(4.0))
            .child(textarea)
            .child(status_area)
            .into_any_element()
    }
}
