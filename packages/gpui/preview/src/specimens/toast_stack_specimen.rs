use crate::app_state::AppState;
use crate::node_compat::{Button, Eyebrow, ToastStack};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, ButtonVariant, EyebrowSpec};
use poodle_specs::{Toast, ToastStackSpec, ToastTone};

/// One labelled group: eyebrow heading + content stacked beneath it.
fn group(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(content)
}

/// Positioned surface a single toast stack renders inside (toasts overlay a
/// relative container, mirroring how the stack is mounted in an app).
fn surface(stack: ToastStack, min_h: f32) -> Div {
    div().relative().min_h(px(min_h)).child(stack)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    // ── Tones: one toast per tone (info / success / warning / danger) ──
    // Contract §4 visual states + §8 tone custom-property table.
    let tone_toasts = vec![
        Toast::new("tone-info", "Heads up")
            .with_tone(ToastTone::Info)
            .with_message("Informational update for your awareness."),
        Toast::new("tone-success", "Changes saved")
            .with_tone(ToastTone::Success)
            .with_message("Your settings have been updated."),
        Toast::new("tone-warning", "Rate limit warning")
            .with_tone(ToastTone::Warning)
            .with_message("You are approaching your API limit."),
        Toast::new("tone-danger", "Publishing failed")
            .with_tone(ToastTone::Danger)
            .with_message("Check your connection and try again."),
    ];

    // ── Interactive stack: contract §12 trio (success, info+action, warning)
    // plus a danger toast to exercise the assertive tone in the same stack. ──
    let interactive_toasts = vec![
        Toast::new("1", "Changes saved")
            .with_tone(ToastTone::Success)
            .with_message("Your settings have been updated."),
        Toast::new("2", "New version available")
            .with_tone(ToastTone::Info)
            .with_message("Update to v2.1 for the latest features.")
            .with_action_label("Update"),
        Toast::new("3", "Rate limit warning")
            .with_tone(ToastTone::Warning)
            .with_message("You are approaching your API limit."),
        Toast::new("4", "Upload failed")
            .with_tone(ToastTone::Danger)
            .with_message("The file could not be uploaded.")
            .with_action_label("Retry"),
    ];
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "Tones",
            surface(
                ToastStack::from_spec(ToastStackSpec::new().with_toasts(tone_toasts), theme),
                260.0,
            ),
        ))
        .child(group(
            theme,
            "Interactive stack",
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Add toast"),
                    theme,
                ))
                .child(surface(
                    ToastStack::from_spec(
                        ToastStackSpec::new().with_toasts(interactive_toasts),
                        theme,
                    ),
                    260.0,
                )),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "toast-stack",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                surface(
                    ToastStack::from_spec(
                        ToastStackSpec::new().with_toasts(vec![
                            Toast::new(format!("size-{}-1", size_key(size)), "Toast")
                                .with_tone(ToastTone::Info)
                                .with_message("Chrome scales with size."),
                            Toast::new(format!("size-{}-2", size_key(size)), "Action available")
                                .with_tone(ToastTone::Success)
                                .with_message("Dismiss and action controls follow the same ladder.")
                                .with_action_label("View"),
                        ]),
                        theme,
                    )
                    .with_size(size),
                    120.0,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                surface(
                    ToastStack::from_spec(
                        ToastStackSpec::new().with_toasts(vec![
                            Toast::new(
                                format!("density-{}-1", density_key(density)),
                                "Density example",
                            )
                            .with_tone(ToastTone::Warning)
                            .with_message(
                                "Spacing changes between compact, default, and comfortable.",
                            ),
                            Toast::new(
                                format!("density-{}-2", density_key(density)),
                                "Retry failed",
                            )
                            .with_tone(ToastTone::Danger)
                            .with_message("Action row and body spacing follow the same ladder.")
                            .with_action_label("Retry"),
                        ]),
                        theme,
                    )
                    .with_density(density),
                    120.0,
                )
                .into_any_element()
            }),
    )
}
