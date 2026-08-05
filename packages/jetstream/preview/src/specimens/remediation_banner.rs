//! RemediationBanner specimen — dismissible fix suggestion banner.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_remediation_banner;

use poodle_specs::{ButtonVariant, RemediationBannerSpec, RemediationAction};
use poodle_specs::{AnnouncementMode, StatusTone};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Info (polite, primary + dismiss)", secondary,
            js_remediation_banner(
                &RemediationBannerSpec::new(
                    "Tip",
                    "Add a description to improve search visibility.",
                )
                .with_tone(StatusTone::Info)
                .with_primary_action(
                    RemediationAction::new("add", "Add description")
                        .with_variant(ButtonVariant::Primary),
                )
                .with_dismissible(true),
                theme,
            )
        ))
        .child(group("Warning (assertive, two actions)", secondary,
            js_remediation_banner(
                &RemediationBannerSpec::new(
                    "Review attention needed",
                    "Resolve the blocking validation before publishing.",
                )
                .with_tone(StatusTone::Warning)
                .with_announce_mode(AnnouncementMode::Assertive)
                .with_primary_action(
                    RemediationAction::new("resolve", "Resolve")
                        .with_variant(ButtonVariant::Primary),
                )
                .with_secondary_action(RemediationAction::new("inspect", "Inspect"))
                .with_dismissible(true),
                theme,
            )
        ))
        .child(group("Danger (no actions)", secondary,
            js_remediation_banner(
                &RemediationBannerSpec::new(
                    "Critical error",
                    "The upload pipeline has failed and requires intervention.",
                )
                .with_tone(StatusTone::Danger),
                theme,
            )
        ))
        .child(group("Success (recovery confirmed)", secondary,
            js_remediation_banner(
                &RemediationBannerSpec::new(
                    "Recovered",
                    "The pipeline reconnected and resumed automatically.",
                )
                .with_tone(StatusTone::Success)
                .with_dismissible(true),
                theme,
            )
        ))
        .child(group("Pending (in-flight, disabled action, silent)", secondary,
            js_remediation_banner(
                &RemediationBannerSpec::new(
                    "Reconnecting",
                    "Attempting to restore the connection — retry is unavailable while in progress.",
                )
                .with_tone(StatusTone::Pending)
                .with_announce_mode(AnnouncementMode::None)
                .with_primary_action(
                    RemediationAction::new("retry", "Retry")
                        .with_variant(ButtonVariant::Secondary)
                        .with_disabled(true),
                ),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
