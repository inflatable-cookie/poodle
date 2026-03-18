//! Switch specimen.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::switch::js_switch;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::SwitchSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    fn sw(checked: bool, label: &str) -> SwitchSpec {
        let mut s = SwitchSpec::new();
        if checked { s = s.with_checked(true); }
        s.label = Some(label.into());
        s
    }

    fn sw_disabled(checked: bool, label: &str) -> SwitchSpec {
        let mut s = sw(checked, label);
        s.is_disabled = true;
        s
    }

    div().flex_col().gap(24.0)
        .child(group("States", secondary,
            div().flex_col().gap(12.0)
                .child(js_switch(&sw(false, "Off"), theme))
                .child(js_switch(&sw(true, "On"), theme))
        ))
        .child(group("Disabled", secondary,
            div().flex_col().gap(12.0)
                .child(js_switch(&sw_disabled(false, "Disabled off"), theme))
                .child(js_switch(&sw_disabled(true, "Disabled on"), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
