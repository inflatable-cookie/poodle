//! Checkbox specimen.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::checkbox::js_checkbox;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::CheckboxSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    fn cb(label: &str) -> CheckboxSpec {
        let mut s = CheckboxSpec::new();
        s.label = Some(label.into());
        s
    }

    div().flex_col().gap(24.0)
        .child(group("States", secondary,
            div().flex_col().gap(12.0)
                .child(js_checkbox(&cb("Unchecked"), theme))
                .child(js_checkbox(&{ let mut s = cb("Checked"); s = s.with_checked(true); s }, theme))
                .child(js_checkbox(&{ let mut s = cb("Mixed / indeterminate"); s = s.with_mixed(true); s }, theme))
        ))
        .child(group("Disabled", secondary,
            div().flex_col().gap(12.0)
                .child(js_checkbox(&{ let mut s = cb("Disabled unchecked"); s = s.with_disabled(true); s }, theme))
                .child(js_checkbox(&{ let mut s = cb("Disabled checked"); s = s.with_checked(true).with_disabled(true); s }, theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
