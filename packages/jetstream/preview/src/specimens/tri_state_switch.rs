//! TriStateSwitch specimen — all three states, sizes, disabled.

use crate::compat::js_tri_state_switch;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlSize, TriStateSwitchSpec, TriStateValue};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // All three states
        .child(group(
            "States",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Excluded)
                        .with_aria_label("Excluded"),
                    theme,
                    "jetstream-state-excluded",
                ))
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Default)
                        .with_aria_label("Default"),
                    theme,
                    "jetstream-state-default",
                ))
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Included)
                        .with_aria_label("Included"),
                    theme,
                    "jetstream-state-included",
                )),
        ))
        // Sizes
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_size(ControlSize::Sm)
                        .with_value(TriStateValue::Included)
                        .with_aria_label("Small"),
                    theme,
                    "jetstream-size-sm",
                ))
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_size(ControlSize::Md)
                        .with_value(TriStateValue::Included)
                        .with_aria_label("Medium"),
                    theme,
                    "jetstream-size-md",
                ))
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_size(ControlSize::Lg)
                        .with_value(TriStateValue::Included)
                        .with_aria_label("Large"),
                    theme,
                    "jetstream-size-lg",
                )),
        ))
        // Custom labels (Hide / All / Show)
        .child(group(
            "Custom labels",
            secondary,
            div().flex_col().gap(12.0).child(js_tri_state_switch(
                &TriStateSwitchSpec::new()
                    .with_value(TriStateValue::Default)
                    .with_excluded_label("Hide")
                    .with_default_label("All")
                    .with_included_label("Show")
                    .with_aria_label("Visibility filter"),
                theme,
                "jetstream-custom-labels",
            )),
        ))
        // Custom semantic colors (per-state hex overrides)
        .child(group(
            "Custom semantic colors",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Excluded)
                        .with_excluded_color("#ef4444")
                        .with_default_color("#64748b")
                        .with_included_color("#22c55e")
                        .with_aria_label("Custom colors excluded"),
                    theme,
                    "jetstream-custom-colors-excluded",
                ))
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Included)
                        .with_excluded_color("#ef4444")
                        .with_default_color("#64748b")
                        .with_included_color("#22c55e")
                        .with_aria_label("Custom colors included"),
                    theme,
                    "jetstream-custom-colors-included",
                )),
        ))
        // Disabled
        .child(group(
            "Disabled",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Excluded)
                        .with_aria_label("Disabled excluded")
                        .with_disabled(true),
                    theme,
                    "jetstream-disabled-excluded",
                ))
                .child(js_tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Included)
                        .with_aria_label("Disabled included")
                        .with_disabled(true),
                    theme,
                    "jetstream-disabled-included",
                )),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
