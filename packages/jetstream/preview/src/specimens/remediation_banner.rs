//! RemediationBanner specimen — dismissible fix suggestion banner.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::remediation_banner::js_remediation_banner;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{RemediationBannerSpec, RemediationAction};
use poodle_components::StatusTone;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Info", secondary,
            js_remediation_banner(
                &RemediationBannerSpec::new(
                    "Tip",
                    "Add a description to improve search visibility.",
                )
                .with_tone(StatusTone::Info)
                .with_primary_action(RemediationAction::new("add", "Add description"))
                .with_dismissible(true),
                theme,
            )
        ))
        .child(group("Warning with actions", secondary,
            js_remediation_banner(
                &RemediationBannerSpec::new(
                    "Review attention needed",
                    "Resolve the blocking validation before publishing.",
                )
                .with_tone(StatusTone::Warning)
                .with_primary_action(RemediationAction::new("resolve", "Resolve"))
                .with_secondary_action(RemediationAction::new("inspect", "Inspect"))
                .with_dismissible(true),
                theme,
            )
        ))
        .child(group("Danger", secondary,
            js_remediation_banner(
                &RemediationBannerSpec::new(
                    "Critical error",
                    "The upload pipeline has failed and requires intervention.",
                )
                .with_tone(StatusTone::Danger),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
