//! ResizeHandle specimen — draggable resize divider.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::resize_handle::js_resize_handle;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{Orientation, ResizeHandleSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Horizontal", secondary,
            div().w(300.0).child(
                js_resize_handle(
                    &ResizeHandleSpec::new().with_orientation(Orientation::Horizontal),
                    theme,
                )
            )
        ))
        .child(group("Vertical", secondary,
            div().h(100.0).child(
                js_resize_handle(
                    &ResizeHandleSpec::new().with_orientation(Orientation::Vertical),
                    theme,
                )
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
