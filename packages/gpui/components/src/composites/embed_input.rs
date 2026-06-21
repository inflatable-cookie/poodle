//! EmbedInput — URL input for embedding external content backed by EmbedInputSpec.

use crate::presentation::rem_to_px;
use crate::primitives::{Pill, TextInput};
use crate::theme_ext::{resolve_color, resolve_px};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::EmbedInputSpec;
use poodle_specs::{PillSize, PillSpec, PillTone, TextInputSpec};

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

        // Status colors split per contract: error = text-danger, success = text-success.
        let danger_color = resolve_color(theme, "color.status.danger");
        let success_color = resolve_color(theme, "color.status.success");
        let status_size = resolve_px(theme, "typography.label.size");

        // Contract §7 spacing — root gap 0.25rem, status gap 0.375rem,
        // status min-height 1.25rem. Resolved from named space tokens where the
        // exact rem aligns (root → space.inline.xs 0.25rem; min-h → space.stack.lg
        // 1.25rem). Status gap 0.375rem has no exact named token — exact rem.
        let root_gap = resolve_px(theme, "space.inline.xs");
        let status_min_h = resolve_px(theme, "space.stack.lg");
        let status_gap = px(rem_to_px(0.375));

        let (parsed, error) = spec.resolved_parse_state();

        // Real TextInput primitive (rows=3 multiline) — delegates all input
        // semantics, sizing, and token resolution. Debounced parse / onValueChange
        // are preview-loop wiring (the spec pre-resolves parse state here).
        let placeholder = spec
            .placeholder
            .clone()
            .unwrap_or_else(|| String::from("Paste a URL or embed code..."));
        let text_input = TextInput::from_spec(
            TextInputSpec::new()
                .with_id("embed-input")
                .with_input_type("multiline")
                .with_rows(3)
                .with_value(spec.value.clone())
                .with_placeholder(placeholder)
                .with_disabled(spec.is_disabled),
            theme,
        );

        let mut wrapper = div()
            .flex()
            .flex_col()
            .gap(root_gap)
            .w_full()
            .child(text_input);

        if error.is_some() || parsed.is_some() {
            let mut status_area = div()
                .min_h(status_min_h)
                .flex()
                .items_center()
                .gap(status_gap)
                .text_size(status_size);

            // Error span — text-danger color (contract Error part).
            if let Some(err) = error.clone() {
                status_area = status_area.child(
                    div().text_color(danger_color).child(err),
                );
            } else if let Some(parsed) = parsed.clone() {
                // Success: ProviderPill (tone="success") + SuccessText.
                // Contract sizeRole="chrome": at default presentation chrome
                // resolves one stop down from Md → Sm. PillSpec has no size-role
                // field, so Sm is the faithful resolved value.
                status_area = status_area
                    .child(Pill::from_spec(
                        PillSpec::new()
                            .with_label(parsed.provider)
                            .with_tone(PillTone::Success)
                            .with_size(PillSize::Sm),
                        theme,
                    ))
                    .child(
                        div()
                            .text_color(success_color)
                            .child("Embed detected"),
                    );
            }

            wrapper = wrapper.child(status_area);
        }

        wrapper.into_any_element()
    }
}
