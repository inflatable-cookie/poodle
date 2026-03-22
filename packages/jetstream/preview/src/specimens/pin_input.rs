//! PinInput specimen — pin inputs empty, partial, full, and masked.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::pin_input::js_pin_input;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::PinInputSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Empty
        .child(group("Empty", secondary,
            div().w(300.0)
                .child(js_pin_input(&PinInputSpec::new(4), theme))
        ))
        // Partial fill
        .child(group("Partial (2 of 4)", secondary,
            div().w(300.0)
                .child(js_pin_input(&PinInputSpec::new(4).with_value("12"), theme))
        ))
        // Full
        .child(group("Full", secondary,
            div().w(300.0)
                .child(js_pin_input(&PinInputSpec::new(4).with_value("1234"), theme))
        ))
        // Masked
        .child(group("Masked", secondary,
            div().w(300.0)
                .child(js_pin_input(&PinInputSpec::new(4).with_value("1234").with_masked(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
