//! EmbedInput — URL input for embedding external content backed by EmbedInputSpec.

use crate::primitives::Pill;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::EmbedInputSpec;
use poodle_specs::{PillSize, PillSpec, PillTone};

pub struct EmbedInput {
    spec: EmbedInputSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for EmbedInput {
    type Target = EmbedInputSpec;
    fn deref(&self) -> &EmbedInputSpec {
        &self.spec
    }
}

impl EmbedInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: EmbedInputSpec::new(),
            theme: theme.clone(),
        }
    }
    pub fn from_spec(spec: EmbedInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for EmbedInput {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "radius.control");
        let text_color = resolve_color(theme, "color.text.primary");
        let placeholder_color = resolve_color(theme, "color.text.secondary");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let status_color = resolve_color(theme, spec.status_text_color_token());
        let body_size = resolve_px(theme, "typography.body.size");
        let label_size = resolve_px(theme, "typography.label.size");

        let display = if spec.value.is_empty() {
            spec.placeholder
                .as_deref()
                .unwrap_or("Paste URL or embed code...")
        } else {
            &spec.value
        };
        let color = if spec.value.is_empty() {
            placeholder_color
        } else {
            text_color
        };
        let (parsed, error) = spec.resolved_parse_state();

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
            .text_size(body_size)
            .line_height(relative(1.5))
            .text_color(color)
            .focus(move |s| s.border_color(focus_ring))
            .child(display.to_string());

        if spec.is_disabled {
            textarea = textarea
                .opacity(resolve_opacity(theme, "state.opacity.disabled"))
                .cursor_not_allowed();
        }

        let mut wrapper = div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .w_full()
            .child(textarea);

        if error.is_some() || parsed.is_some() {
            let status_area = div()
                .min_h(px(20.0))
                .px(px(4.0))
                .flex()
                .items_center()
                .gap(px(6.0))
                .text_size(label_size)
                .text_color(status_color)
                .when(parsed.is_some(), |el| {
                    el.child(Pill::from_spec(
                        PillSpec::new()
                            .with_label(parsed.clone().unwrap().provider)
                            .with_tone(PillTone::Success)
                            .with_size(PillSize::Sm),
                        theme,
                    ))
                })
                .when(error.is_some() || parsed.is_some(), |el| {
                    el.child(
                        error
                            .clone()
                            .unwrap_or_else(|| String::from("Embed detected")),
                    )
                });
            wrapper = wrapper.child(status_area);
        }

        wrapper.into_any_element()
    }
}
