//! TriStateSwitch specimen — all three states, sizes, disabled.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::tri_state_switch::js_tri_state_switch;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{CheckState, ControlSize, TriStateSwitchSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // All three states
        .child(group("States", secondary,
            div().flex_col().gap(12.0)
                .child(js_tri_state_switch(&TriStateSwitchSpec::new().with_state(CheckState::Unchecked).with_label("Excluded"), theme))
                .child(js_tri_state_switch(&TriStateSwitchSpec::new().with_state(CheckState::Mixed).with_label("Default"), theme))
                .child(js_tri_state_switch(&TriStateSwitchSpec::new().with_state(CheckState::Checked).with_label("Included"), theme))
        ))
        // Sizes
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(js_tri_state_switch(&TriStateSwitchSpec::new().with_size(ControlSize::Sm).with_state(CheckState::Checked).with_label("Small"), theme))
                .child(js_tri_state_switch(&TriStateSwitchSpec::new().with_size(ControlSize::Md).with_state(CheckState::Checked).with_label("Medium"), theme))
                .child(js_tri_state_switch(&TriStateSwitchSpec::new().with_size(ControlSize::Lg).with_state(CheckState::Checked).with_label("Large"), theme))
        ))
        // Custom labels (Hide / All / Show)
        .child(group("Custom labels", secondary,
            div().flex_col().gap(12.0)
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_state(CheckState::Mixed)
                        .with_excluded_label("Hide")
                        .with_default_label("All")
                        .with_included_label("Show"),
                    theme,
                ))
        ))
        // Custom semantic colors (per-state hex overrides)
        .child(group("Custom semantic colors", secondary,
            div().flex_col().gap(12.0)
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_state(CheckState::Unchecked)
                        .with_excluded_color("#ef4444")
                        .with_default_color("#64748b")
                        .with_included_color("#22c55e"),
                    theme,
                ))
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_state(CheckState::Checked)
                        .with_excluded_color("#ef4444")
                        .with_default_color("#64748b")
                        .with_included_color("#22c55e"),
                    theme,
                ))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().flex_col().gap(12.0)
                .child(js_tri_state_switch(&TriStateSwitchSpec::new().with_state(CheckState::Unchecked).with_label("Disabled excluded").with_disabled(true), theme))
                .child(js_tri_state_switch(&TriStateSwitchSpec::new().with_state(CheckState::Checked).with_label("Disabled included").with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
