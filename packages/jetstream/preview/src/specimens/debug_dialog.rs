//! DebugDialog specimen — developer JSON disclosure (trigger + code dump).
//!
//! Mirrors the GPUI specimen group set: with debug value (ghost trigger + JSON
//! Code block), custom trigger (secondary/xs + tighter max-height), and
//! hidden-when-null (renders nothing). Composes the real `js_debug_dialog`,
//! which wraps the real `js_button` + `js_code` — no fakes. The JSON payload
//! mirrors the Svelte/GPUI specimens so all targets dump the same shape.

use crate::compat::js_debug_dialog;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ButtonVariant, ControlSize, DebugDialogSpec};

/// JSON payload mirrored from the Svelte/GPUI specimens (object with id/status
/// plus a nested array).
fn sample_value() -> &'static str {
    "{\n  \"id\": \"asset_42\",\n  \"status\": \"ready\",\n  \"checks\": [\n    \"metadata\",\n    \"thumbnail\",\n    \"permissions\"\n  ]\n}"
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .max_w(640.0)
        // --- With debug value (ghost trigger + lg dialog) ---
        .child(group(
            "With debug value",
            secondary,
            js_debug_dialog(
                &DebugDialogSpec::new()
                    .with_title("Asset payload")
                    .with_trigger_label("Inspect payload")
                    .with_value(sample_value()),
                theme,
            ),
        ))
        // --- Custom trigger (secondary variant, xs size, tighter max-height) ---
        .child(group(
            "Custom trigger",
            secondary,
            js_debug_dialog(
                &DebugDialogSpec::new()
                    .with_title("Compact payload")
                    .with_trigger_label("Debug")
                    .with_trigger_variant(ButtonVariant::Secondary)
                    .with_trigger_size(Some(ControlSize::Xs))
                    .with_max_height("18rem")
                    .with_value(sample_value()),
                theme,
            ),
        ))
        // --- Hidden when null (renders nothing; trigger only when value present) ---
        .child(group(
            "Hidden when null",
            secondary,
            js_debug_dialog(&DebugDialogSpec::new(), theme),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
