use gpui::*;
use poodle_primitives::{CheckboxSpec, ControlDensity, ControlSize, EyebrowSpec};
use poodle_gpui_components::{Checkbox, Eyebrow};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child({
                    let items: &[(&str, &str, bool)] = &[
                        ("checkbox-email", "Enable email notifications", true),
                        ("checkbox-marketing", "Subscribe to marketing emails", false),
                        ("checkbox-terms", "I agree to the terms and conditions", false),
                    ];
                    let mut col = div().flex().flex_col().gap(px(10.0));
                    for &(key, label, default) in items {
                        let init_key = format!("{key}__init");
                        let checked = if !state.specimens.is_on(&init_key) {
                            default
                        } else {
                            state.specimens.is_on(key)
                        };
                        let key_owned = key.to_string();
                        let init_key_owned = init_key.clone();
                        col = col.child(
                            Checkbox::from_spec(
                                CheckboxSpec::new()
                                    .with_checked(checked)
                                    .with_label(label),
                                theme,
                            )
                            .with_id(key)
                            .on_change(cx.listener(move |this, _checked: &bool, _w, cx| {
                                if !this.state.specimens.is_on(&init_key_owned) {
                                    this.state.specimens.toggle(&init_key_owned);
                                }
                                this.state.specimens.toggle(&key_owned);
                                cx.notify();
                            }))
                        );
                    }
                    col
                })
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().gap(px(16.0)).items_center()
                        .child(
                            Checkbox::from_spec(
                                CheckboxSpec::new().with_label("Accept terms"),
                                theme,
                            ).with_id("size-xs").size(ControlSize::Xs)
                        )
                        .child(
                            Checkbox::from_spec(
                                CheckboxSpec::new().with_label("Accept terms"),
                                theme,
                            ).with_id("size-sm").size(ControlSize::Sm)
                        )
                        .child(
                            Checkbox::from_spec(
                                CheckboxSpec::new().with_label("Accept terms"),
                                theme,
                            ).with_id("size-md").size(ControlSize::Md)
                        )
                        .child(
                            Checkbox::from_spec(
                                CheckboxSpec::new().with_label("Accept terms"),
                                theme,
                            ).with_id("size-lg").size(ControlSize::Lg)
                        )
                        .child(
                            Checkbox::from_spec(
                                CheckboxSpec::new().with_label("Accept terms"),
                                theme,
                            ).with_id("size-xl").size(ControlSize::Xl)
                        )
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            Checkbox::from_spec(
                                CheckboxSpec::new().with_label("compact: Option"),
                                theme,
                            ).with_id("density-compact").density(ControlDensity::Compact)
                        )
                        .child(
                            Checkbox::from_spec(
                                CheckboxSpec::new().with_label("default: Option"),
                                theme,
                            ).with_id("density-default").density(ControlDensity::Default)
                        )
                        .child(
                            Checkbox::from_spec(
                                CheckboxSpec::new().with_label("comfortable: Option"),
                                theme,
                            ).with_id("density-comfortable").density(ControlDensity::Comfortable)
                        )
                )
        )
        // --- States ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("States"), theme))
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_disabled(true)
                            .with_label("Disabled unchecked"),
                        theme,
                    ).with_id("cb-disabled-unchecked")
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_disabled(true)
                            .with_label("Disabled checked"),
                        theme,
                    ).with_id("cb-disabled-checked")
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_mixed(true)
                            .with_label("Mixed / indeterminate"),
                        theme,
                    ).with_id("cb-mixed")
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_read_only(true)
                            .with_label("Read-only checked"),
                        theme,
                    ).with_id("cb-readonly-checked")
                )
        )
        // --- Custom selected color ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom selected color"), theme))
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_label("Billable feature")
                            .with_selected_color("#22c55e"),
                        theme,
                    ).with_id("cb-custom-green")
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_label("Requires moderation")
                            .with_selected_color("#f59e0b"),
                        theme,
                    ).with_id("cb-custom-amber")
                )
        )
}
