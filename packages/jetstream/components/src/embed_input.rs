//! EmbedInput — Jetstream URL input with parsed embed preview pills backed by EmbedInputSpec.
//!
//! Contract: `docs/contracts/components/embed-input.md`
//! Reference: Svelte `EmbedInput.svelte` (parity authority); GPUI `composites/embed_input.rs`.
//!
//! Composes the real `js_text_input` primitive (multiline, rows=3) and the real
//! `js_pill` provider chip — no hand-styled fakes. Debounced parse / onValueChange
//! / onParse are preview-loop concerns (the spec pre-resolves parse state here).
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{EmbedInputSpec, PillSize, PillSpec, PillTone, TextInputSpec};

use crate::pill::js_pill;
use crate::presentation::rem_to_px;
use crate::text_input::js_text_input;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_embed_input(spec: &EmbedInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Status colors split per contract: error = text-danger, success = text-success.
    let danger_color = resolve_color(theme, "color.status.danger");
    let success_color = resolve_color(theme, "color.status.success");
    let status_font = resolve_px(theme, "typography.label.size");

    // Contract §7 spacing. Root gap 0.25rem → space.inline.xs; status min-height
    // 1.25rem → space.stack.lg. Status gap 0.375rem has no exact named token —
    // exact rem (status font-size 0.75rem likewise has no token).
    let root_gap = resolve_px(theme, "space.inline.xs");
    let status_min_h = resolve_px(theme, "space.stack.lg");
    let status_gap = rem_to_px(0.375);

    let (parsed, error) = spec.resolved_parse_state();

    // Real multiline TextInput primitive (rows=3) — delegates input semantics,
    // sizing, token resolution, and disabled-opacity to the primitive.
    let placeholder = spec
        .placeholder
        .clone()
        .unwrap_or_else(|| String::from("Paste a URL or embed code..."));
    let text_input = js_text_input(
        &TextInputSpec::new()
            .with_id("embed-input")
            .with_input_type("multiline")
            .with_rows(3)
            .with_value(spec.value.clone())
            .with_placeholder(placeholder)
            .with_disabled(spec.is_disabled),
        theme,
    );

    let mut wrapper = ui_element::div().flex_col().gap(root_gap).child(text_input);

    if error.is_some() || parsed.is_some() {
        let mut status_row = ui_element::div()
            .flex_row()
            .items_center()
            .gap(status_gap)
            .min_h(status_min_h);

        if let Some(ref err) = error {
            // Error span — text-danger.
            status_row = status_row.child(
                ui_element::label(err)
                    .text_color(danger_color)
                    .text_size(status_font),
            );
        } else if let Some(ref parsed) = parsed {
            // Success: real Pill (tone=Success; size Sm = chrome resolved at
            // default presentation) + SuccessText (text-success).
            status_row = status_row
                .child(js_pill(
                    &PillSpec::new()
                        .with_label(parsed.provider.clone())
                        .with_tone(PillTone::Success)
                        .with_size(PillSize::Sm),
                    theme,
                ))
                .child(
                    ui_element::label("Embed detected")
                        .text_color(success_color)
                        .text_size(status_font),
                );
        }

        wrapper = wrapper.child(status_row);
    }

    wrapper
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_text_input_field_with_placeholder() {
        let th = theme();
        let spec = EmbedInputSpec::new().with_placeholder("Paste a YouTube URL...");
        let tree = probe(&js_embed_input(&spec, &th), 320.0, 120.0);
        // The composed TextInput primitive surfaces the placeholder text.
        assert!(
            tree.has_text("Paste a YouTube URL..."),
            "placeholder missing: {:?}",
            tree.texts()
        );
        // The composed field renders a filled panel (the TextInput body) wrapping
        // the placeholder label.
        assert!(
            tree.nodes.iter().any(|n| n.depth >= 1 && n.bg.is_some()),
            "no filled field panel: {}",
            tree.to_json()
        );
    }

    #[test]
    fn success_state_shows_provider_pill_and_success_text() {
        let th = theme();
        let spec = EmbedInputSpec::new()
            .with_value("https://youtu.be/dQw4w9WgXcQ")
            .with_detected_parse();
        let tree = probe(&js_embed_input(&spec, &th), 320.0, 120.0);
        assert!(tree.has_text("youtube"), "provider pill missing: {:?}", tree.texts());
        assert!(tree.has_text("Embed detected"), "success text missing: {:?}", tree.texts());
    }

    #[test]
    fn error_state_shows_only_error_message() {
        let th = theme();
        let spec = EmbedInputSpec::new().with_error("Could not parse embed source");
        let tree = probe(&js_embed_input(&spec, &th), 320.0, 120.0);
        assert!(
            tree.has_text("Could not parse embed source"),
            "error text missing: {:?}",
            tree.texts()
        );
        // Error and success are mutually exclusive — no success content.
        assert!(!tree.has_text("Embed detected"), "error state must not show success text");
        assert!(!tree.has_text("youtube"), "error state must not show a provider pill");
    }

    #[test]
    fn disabled_input_delegates_dimming_to_primitive() {
        let th = theme();
        let spec = EmbedInputSpec::new().with_value("x").with_disabled(true);
        let tree = probe(&js_embed_input(&spec, &th), 320.0, 120.0);
        // The composed TextInput primitive renders the value (it owns disabled
        // opacity via its own disabled_opacity_token — no hardcoded 0.5 here).
        assert!(tree.has_text("x"), "input value missing: {}", tree.to_json());
        assert!(
            tree.nodes.iter().any(|n| n.depth >= 1 && n.bg.is_some()),
            "no field panel: {}",
            tree.to_json()
        );
    }

    #[test]
    fn empty_value_has_no_status_row() {
        let th = theme();
        let spec = EmbedInputSpec::new();
        let tree = probe(&js_embed_input(&spec, &th), 320.0, 120.0);
        assert!(!tree.has_text("Embed detected"), "empty input must not show success text");
    }
}
