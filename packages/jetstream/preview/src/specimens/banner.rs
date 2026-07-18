//! Banner specimen — persistent status banners.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::banner::js_banner;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{BannerSpec, StatusTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Info (default)", secondary,
            js_banner(
                &BannerSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_title("New features available")
                    .with_message("A new version of the application is ready to install."),
                theme,
            )
        ))
        .child(group("Success", secondary,
            js_banner(
                &BannerSpec::new()
                    .with_tone(StatusTone::Success)
                    .with_message("Your changes have been saved successfully."),
                theme,
            )
        ))
        .child(group("Warning with dismiss", secondary,
            js_banner(
                &BannerSpec::new()
                    .with_tone(StatusTone::Warning)
                    .with_title("Unsaved changes")
                    .with_message("You have unsaved changes that will be lost on navigation.")
                    .with_dismissible(true),
                theme,
            )
        ))
        .child(group("Danger", secondary,
            js_banner(
                &BannerSpec::new()
                    .with_tone(StatusTone::Danger)
                    .with_title("Action required")
                    .with_message("Validation errors must be resolved before you can publish."),
                theme,
            )
        ))
        .child(group("No icon", secondary,
            js_banner(
                &BannerSpec::new()
                    .with_tone(StatusTone::Neutral)
                    .with_icon(false)
                    .with_message("Review is pending approval from two more reviewers."),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
