//! Banner specimen — status banners with different tones.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::banner::js_banner;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{BannerSpec, StatusTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Info", secondary,
            js_banner(&BannerSpec::new()
                .with_tone(StatusTone::Info)
                .with_message("System maintenance scheduled for tonight."), theme)
        ))
        .child(group("Success", secondary,
            js_banner(&BannerSpec::new()
                .with_tone(StatusTone::Success)
                .with_message("Changes saved successfully."), theme)
        ))
        .child(group("Warning", secondary,
            js_banner(&BannerSpec::new()
                .with_tone(StatusTone::Warning)
                .with_message("Your trial expires in 3 days."), theme)
        ))
        .child(group("Danger", secondary,
            js_banner(&BannerSpec::new()
                .with_tone(StatusTone::Danger)
                .with_message("Payment failed. Please update your billing info."), theme)
        ))
        .child(group("Dismissible", secondary,
            js_banner(&BannerSpec::new()
                .with_tone(StatusTone::Info)
                .with_message("You can dismiss this banner.")
                .with_dismissible(true), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
