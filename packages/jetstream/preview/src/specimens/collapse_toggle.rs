//! CollapseToggle specimen — collapse toggles in all directions and states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::collapse_toggle::js_collapse_toggle;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{CollapseDirection, CollapseToggleSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Left direction — collapsed vs expanded
        .child(group("Left: collapsed / expanded", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_collapsed(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Left).with_collapsed(false), theme))
        ))
        // Right direction
        .child(group("Right: collapsed / expanded", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Right).with_collapsed(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Right).with_collapsed(false), theme))
        ))
        // Up direction
        .child(group("Up: collapsed / expanded", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Up).with_collapsed(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Up).with_collapsed(false), theme))
        ))
        // Down direction
        .child(group("Down: collapsed / expanded", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Down).with_collapsed(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_direction(CollapseDirection::Down).with_collapsed(false), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().flex_row().gap(16.0).items_center()
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_collapsed(true).with_disabled(true), theme))
                .child(js_collapse_toggle(&CollapseToggleSpec::new().with_collapsed(false).with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
