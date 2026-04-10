use gpui::*;
use poodle_primitives::{ControlDensity, ControlSize, SwitchSpec, SwitchTone, EyebrowSpec};
use poodle_gpui_components::{Switch, Eyebrow};
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
                        ("switch-dark-mode", "Dark mode", true),
                        ("switch-auto-save", "Auto-save drafts", false),
                        ("switch-compact", "Compact view", true),
                    ];
                    let mut col = div().flex().flex_col().gap(px(10.0));
                    for &(key, label, default) in items {
                        let init_key = format!("{key}__init");
                        let is_on = if !state.specimens.is_on(&init_key) {
                            default
                        } else {
                            state.specimens.is_on(key)
                        };
                        let key_owned = key.to_string();
                        let init_key_owned = init_key.clone();
                        let mut spec = SwitchSpec::new().with_checked(is_on);
                        spec.label = Some(label.to_string());
                        col = col.child(
                            Switch::from_spec(spec, theme)
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
                        .child({
                            let mut spec = SwitchSpec::new().with_checked(true).with_size(ControlSize::Xs);
                            spec.label = Some("Xs".to_string());
                            Switch::from_spec(spec, theme).with_id("switch-size-xs")
                        })
                        .child({
                            let mut spec = SwitchSpec::new().with_checked(true).with_size(ControlSize::Sm);
                            spec.label = Some("Sm".to_string());
                            Switch::from_spec(spec, theme).with_id("switch-size-sm")
                        })
                        .child({
                            let mut spec = SwitchSpec::new().with_checked(true).with_size(ControlSize::Md);
                            spec.label = Some("Md".to_string());
                            Switch::from_spec(spec, theme).with_id("switch-size-md")
                        })
                        .child({
                            let mut spec = SwitchSpec::new().with_checked(true).with_size(ControlSize::Lg);
                            spec.label = Some("Lg".to_string());
                            Switch::from_spec(spec, theme).with_id("switch-size-lg")
                        })
                        .child({
                            let mut spec = SwitchSpec::new().with_checked(true).with_size(ControlSize::Xl);
                            spec.label = Some("Xl".to_string());
                            Switch::from_spec(spec, theme).with_id("switch-size-xl")
                        })
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child({
                            let mut spec = SwitchSpec::new().with_checked(true).with_density(ControlDensity::Compact);
                            spec.label = Some("compact: Option".to_string());
                            Switch::from_spec(spec, theme).with_id("switch-density-compact")
                        })
                        .child({
                            let mut spec = SwitchSpec::new().with_checked(true).with_density(ControlDensity::Default);
                            spec.label = Some("default: Option".to_string());
                            Switch::from_spec(spec, theme).with_id("switch-density-default")
                        })
                        .child({
                            let mut spec = SwitchSpec::new().with_checked(true).with_density(ControlDensity::Comfortable);
                            spec.label = Some("comfortable: Option".to_string());
                            Switch::from_spec(spec, theme).with_id("switch-density-comfortable")
                        })
                )
        )
        // --- States ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("States"), theme))
                .child({
                    let mut col = div().flex().flex_col().gap(px(10.0));

                    // Disabled off
                    let mut spec = SwitchSpec::new().with_checked(false);
                    spec.label = Some("Disabled off".to_string());
                    spec.is_disabled = true;
                    col = col.child(Switch::from_spec(spec, theme).with_id("sw-disabled-off"));

                    // Disabled on
                    let mut spec = SwitchSpec::new().with_checked(true);
                    spec.label = Some("Disabled on".to_string());
                    spec.is_disabled = true;
                    col = col.child(Switch::from_spec(spec, theme).with_id("sw-disabled-on"));

                    // Read-only on
                    let mut spec = SwitchSpec::new().with_checked(true);
                    spec.label = Some("Read-only on".to_string());
                    spec.is_read_only = true;
                    col = col.child(Switch::from_spec(spec, theme).with_id("sw-readonly-on"));

                    col
                })
        )
        // --- Custom colors ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom colors"), theme))
                .child({
                    let mut col = div().flex().flex_col().gap(px(10.0));

                    let mut spec = SwitchSpec::new()
                        .with_checked(true)
                        .with_on_color("#22c55e")
                        .with_off_color("#cbd5e1");
                    spec.label = Some("Billing alerts".to_string());
                    col = col.child(Switch::from_spec(spec, theme).with_id("sw-custom-green"));

                    let mut spec = SwitchSpec::new()
                        .with_checked(false)
                        .with_on_color("#f59e0b")
                        .with_off_color("#94a3b8");
                    spec.label = Some("Quiet mode".to_string());
                    col = col.child(Switch::from_spec(spec, theme).with_id("sw-custom-amber"));

                    col
                })
        )
        // --- Dual labels and tones ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Dual labels and tones"), theme))
                .child({
                    let mut col = div().flex().flex_col().gap(px(10.0));

                    col = col.child(
                        Switch::from_spec(
                            SwitchSpec::new()
                                .with_default_checked(true)
                                .with_left_label("Draft")
                                .with_right_label("Live")
                                .with_left_tone(SwitchTone::Danger)
                                .with_right_tone(SwitchTone::Success)
                                .with_aria_label("Publication status"),
                            theme,
                        )
                        .with_id("sw-dual-publish")
                    );

                    col = col.child(
                        Switch::from_spec(
                            SwitchSpec::new()
                                .with_default_checked(false)
                                .with_left_label("Restricted")
                                .with_right_label("Free")
                                .with_left_tone(SwitchTone::Warning)
                                .with_right_tone(SwitchTone::Success)
                                .with_aria_label("Access status"),
                            theme,
                        )
                        .with_id("sw-dual-access")
                    );

                    col
                })
        )
}
