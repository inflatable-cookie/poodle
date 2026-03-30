//! InlineRemediation specimen — inline fix suggestion display.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::inline_remediation::js_inline_remediation;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::{InlineRemediationSpec, RemediationAction};
use poodle_primitives::StatusTone;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Info suggestion", secondary,
            js_inline_remediation(
                &InlineRemediationSpec::new("Consider adding a description for better discoverability.")
                    .with_tone(StatusTone::Info)
                    .with_title("Suggestion"),
                theme,
            )
        ))
        .child(group("Warning with action", secondary,
            js_inline_remediation(
                &InlineRemediationSpec::new("This field has a validation issue that should be resolved.")
                    .with_tone(StatusTone::Warning)
                    .with_title("Validation warning")
                    .with_referenced_field_ids(vec![String::from("title"), String::from("slug")])
                    .with_action(RemediationAction::new("fix", "Auto-fix")),
                theme,
            )
        ))
        .child(group("Danger", secondary,
            js_inline_remediation(
                &InlineRemediationSpec::new("Required field is empty.")
                    .with_tone(StatusTone::Danger)
                    .with_title("Missing required field"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
