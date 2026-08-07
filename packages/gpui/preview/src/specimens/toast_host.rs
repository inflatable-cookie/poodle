use crate::node_compat::{Button, Eyebrow, ToastHost};
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, ButtonVariant, EyebrowSpec};
use poodle_specs::{Toast, ToastHostPlacement, ToastHostSpec, ToastTone};

use crate::style_bridge::color_to_hsla;

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

/// Dashed positioned surface a host anchors inside. `position: fixed` is not
/// representable in a specimen; the host's `absolute()` corner placement is
/// demonstrated relative to this container instead. (note)
fn surface(theme: &GpuiThemeProvider, host: ToastHost) -> Div {
    div()
        .relative()
        .min_h(px(220.0))
        .border_1()
        .border_dashed()
        .border_color(color_to_hsla(theme.resolve_color("color.border.default")).opacity(0.82))
        .rounded(px(8.0))
        .bg(color_to_hsla(theme.resolve_color("color.background.panel")).opacity(0.96))
        .child(host)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    // Contract §12 "Default (Bottom-End)": success toast + error (danger)
    // toast; the danger toast is sticky (matches default `stickyTones`).
    let bottom_end_toasts = vec![
        Toast::new("be-1", "Saved")
            .with_tone(ToastTone::Success)
            .with_message("Your changes were saved."),
        Toast::new("be-2", "Publishing failed")
            .with_tone(ToastTone::Danger)
            .with_message("This one stays until you dismiss it."),
    ];

    // Contract §12 "Top-Start Placement": a single info toast.
    let top_start_toasts = vec![Toast::new("ts-1", "Heads up")
        .with_tone(ToastTone::Info)
        .with_message("Anchored to the top-left corner.")];

    // The two remaining anchors so all four corners are demonstrated.
    let bottom_start_toasts = vec![Toast::new("bs-1", "Sync delayed")
        .with_tone(ToastTone::Warning)
        .with_message("Background sync is running behind.")];
    let top_end_toasts = vec![Toast::new("te-1", "Update applied")
        .with_tone(ToastTone::Success)
        .with_message("You are on the latest version.")];

    // Contract §12 "With Action": toast carrying an action affordance.
    let action_toasts = vec![Toast::new("act-1", "Publishing failed")
        .with_tone(ToastTone::Danger)
        .with_message("We could not publish your changes.")
        .with_action_label("Retry")];

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
                    EyebrowSpec::new().with_content("Runtime host"),
                    theme,
                ))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(theme.resolve_color("color.text.secondary")))
                        .child("The host owns timer policy and fixed positioning while ToastStack stays presentational."),
                )
                .child(Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Add toast"),
                    theme,
                )),
        )
        .child(group(
            theme,
            "Bottom-end (sticky danger)",
            surface(
                theme,
                ToastHost::from_spec(
                    ToastHostSpec::new().with_placement(ToastHostPlacement::BottomEnd),
                    theme,
                )
                .toasts(bottom_end_toasts),
            ),
        ))
        .child(group(
            theme,
            "Bottom-start",
            surface(
                theme,
                ToastHost::from_spec(
                    ToastHostSpec::new().with_placement(ToastHostPlacement::BottomStart),
                    theme,
                )
                .toasts(bottom_start_toasts),
            ),
        ))
        .child(group(
            theme,
            "Top-end",
            surface(
                theme,
                ToastHost::from_spec(
                    ToastHostSpec::new().with_placement(ToastHostPlacement::TopEnd),
                    theme,
                )
                .toasts(top_end_toasts),
            ),
        ))
        .child(group(
            theme,
            "Top-start",
            surface(
                theme,
                ToastHost::from_spec(
                    ToastHostSpec::new().with_placement(ToastHostPlacement::TopStart),
                    theme,
                )
                .toasts(top_start_toasts),
            ),
        ))
        .child(group(
            theme,
            "With action",
            surface(
                theme,
                ToastHost::from_spec(
                    ToastHostSpec::new().with_placement(ToastHostPlacement::BottomEnd),
                    theme,
                )
                .toasts(action_toasts),
            ),
        ))
}
