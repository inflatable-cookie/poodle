//! CollapseToggle specimen — collapse toggles in collapsed and expanded states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::collapse_toggle::js_collapse_toggle;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::CollapseToggleSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Collapsed
        .child(group("Collapsed", secondary,
            js_collapse_toggle(&CollapseToggleSpec::new().with_collapsed(true), theme)
        ))
        // Expanded
        .child(group("Expanded", secondary,
            js_collapse_toggle(&CollapseToggleSpec::new().with_collapsed(false), theme)
        ))
        // Disabled
        .child(group("Disabled", secondary,
            js_collapse_toggle(&CollapseToggleSpec::new().with_collapsed(true).with_disabled(true), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
