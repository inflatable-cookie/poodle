//! Spinner specimen — ring and grid variants, sizes, tones.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::spinner::js_spinner;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{SpinnerSpec, SpinnerVariant, SpinnerSize, SpinnerTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Variants
        .child(group("Variants", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_spinner(&SpinnerSpec::new().with_variant(SpinnerVariant::Ring), theme))
                .child(js_spinner(&SpinnerSpec::new().with_variant(SpinnerVariant::Grid), theme))
        ))
        // Sizes
        .child(group("Sizes", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_spinner(&SpinnerSpec::new().with_size(SpinnerSize::Xs), theme))
                .child(js_spinner(&SpinnerSpec::new().with_size(SpinnerSize::Sm), theme))
                .child(js_spinner(&SpinnerSpec::new().with_size(SpinnerSize::Md), theme))
                .child(js_spinner(&SpinnerSpec::new().with_size(SpinnerSize::Lg), theme))
                .child(js_spinner(&SpinnerSpec::new().with_size(SpinnerSize::Xl), theme))
        ))
        // Tones
        .child(group("Tones", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_spinner(&SpinnerSpec::new().with_tone(SpinnerTone::Current), theme))
                .child(js_spinner(&SpinnerSpec::new().with_tone(SpinnerTone::Accent), theme))
                .child(js_spinner(&SpinnerSpec::new().with_tone(SpinnerTone::Muted), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
