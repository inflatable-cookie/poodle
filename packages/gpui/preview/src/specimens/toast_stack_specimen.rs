use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, ToastStack};
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec, SemanticControlSizeRole};
use poodle_specs::{Toast, ToastPosition, ToastStackSpec, ToastTone};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let mixed_toasts = vec![
        Toast::new("saved", "Changes saved")
            .with_tone(ToastTone::Success)
            .with_message("Your theme and density settings were stored."),
        Toast::new("sync", "Sync delayed")
            .with_tone(ToastTone::Warning)
            .with_message("Background sync will retry in 30 seconds.")
            .with_action_label("Retry now"),
        Toast::new("publish", "Publish failed")
            .with_tone(ToastTone::Danger)
            .with_message("Check validation errors before publishing again.")
            .with_action_label("View errors"),
    ];

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Mixed tones"),
                    theme,
                ))
                .child(
                    div()
                        .relative()
                        .h(px(220.0))
                        .w_full()
                        .overflow_hidden()
                        .child(ToastStack::from_spec(
                            ToastStackSpec::new().with_toasts(mixed_toasts.clone()),
                            theme,
                        )),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Position variants"),
                    theme,
                ))
                .child(
                    div()
                        .grid()
                        .grid_cols(2)
                        .gap(px(12.0))
                        .child(
                            div().relative().h(px(180.0)).overflow_hidden().child(
                                ToastStack::from_spec(
                                    ToastStackSpec::new()
                                        .with_toasts(vec![Toast::new("top-left", "Queued")
                                            .with_message("Two uploads are still processing.")])
                                        .with_position(ToastPosition::TopLeft),
                                    theme,
                                ),
                            ),
                        )
                        .child(
                            div().relative().h(px(180.0)).overflow_hidden().child(
                                ToastStack::from_spec(
                                    ToastStackSpec::new()
                                        .with_toasts(vec![Toast::new(
                                            "top-right",
                                            "New version available",
                                        )
                                        .with_action_label("Update")])
                                        .with_position(ToastPosition::TopRight),
                                    theme,
                                ),
                            ),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Semantic presentation"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(
                            div().relative().h(px(180.0)).overflow_hidden().child(
                                ToastStack::from_spec(
                                    ToastStackSpec::new()
                                        .with_toasts(vec![Toast::new("compact", "Saved")
                                            .with_tone(ToastTone::Success)
                                            .with_message("Draft updated.")])
                                        .with_size(ControlSize::Sm)
                                        .with_density(ControlDensity::Compact),
                                    theme,
                                ),
                            ),
                        )
                        .child(
                            div().relative().h(px(180.0)).overflow_hidden().child(
                                ToastStack::from_spec(
                                    ToastStackSpec::new()
                                        .with_toasts(mixed_toasts)
                                        .with_size_role(SemanticControlSizeRole::Prominent),
                                    theme,
                                ),
                            ),
                        ),
                ),
        )
}
