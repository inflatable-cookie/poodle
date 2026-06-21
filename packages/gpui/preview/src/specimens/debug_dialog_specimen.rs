use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{DebugDialog, Eyebrow};
use poodle_specs::{ButtonVariant, ControlSize, DebugDialogSpec, EyebrowSpec};

/// JSON payload mirrored from the Svelte specimen so both targets dump the same
/// shape (object with id/status plus a nested array).
fn sample_value() -> &'static str {
    "{\n  \"id\": \"asset_42\",\n  \"status\": \"ready\",\n  \"checks\": [\n    \"metadata\",\n    \"thumbnail\",\n    \"permissions\"\n  ]\n}"
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(640.0))
        // --- With debug value (ghost trigger + lg dialog) ---
        .child(group(
            "With debug value",
            theme,
            DebugDialog::from_spec(
                DebugDialogSpec::new()
                    .with_title("Asset payload")
                    .with_trigger_label("Inspect payload")
                    .with_value(sample_value()),
                theme,
            ),
        ))
        // --- Custom trigger (secondary variant, xs size, tighter max-height) ---
        .child(group(
            "Custom trigger",
            theme,
            DebugDialog::from_spec(
                DebugDialogSpec::new()
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
            theme,
            DebugDialog::from_spec(DebugDialogSpec::new(), theme),
        ))
}

fn group(label: &'static str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}
