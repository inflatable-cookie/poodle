//! ToastHost specimen — positioned toast container.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use crate::compat::js_toast_host;
use poodle_specs::{Toast, ToastHostPlacement, ToastHostSpec, ToastStackSpec, ToastTone};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.default");
    let panel = resolve_color(theme, "color.background.panel");

    // Contract §12 "Default (Bottom-End)": success + danger (sticky) toasts.
    let bottom_end = ToastStackSpec::new().with_toasts(vec![
        Toast::new("be-1", "Saved")
            .with_tone(ToastTone::Success)
            .with_message("Your changes were saved."),
        Toast::new("be-2", "Publishing failed")
            .with_tone(ToastTone::Danger)
            .with_message("This one stays until you dismiss it."),
    ]);

    // Remaining anchors so all four corners are demonstrated.
    let bottom_start = ToastStackSpec::new().with_toasts(vec![
        Toast::new("bs-1", "Sync delayed")
            .with_tone(ToastTone::Warning)
            .with_message("Background sync is running behind."),
    ]);
    let top_end = ToastStackSpec::new().with_toasts(vec![
        Toast::new("te-1", "Update applied")
            .with_tone(ToastTone::Success)
            .with_message("You are on the latest version."),
    ]);
    // Contract §12 "Top-Start Placement": single info toast.
    let top_start = ToastStackSpec::new().with_toasts(vec![
        Toast::new("ts-1", "Heads up")
            .with_tone(ToastTone::Info)
            .with_message("Anchored to the top-left corner."),
    ]);

    // Contract §12 "With Action": toast carrying an action affordance.
    let with_action = ToastStackSpec::new().with_toasts(vec![
        Toast::new("act-1", "Publishing failed")
            .with_tone(ToastTone::Danger)
            .with_message("We could not publish your changes.")
            .with_action_label("Retry"),
    ]);

    let anchored = |placement: ToastHostPlacement, stack: &ToastStackSpec| {
        // `position: fixed` is not representable in a specimen; the host's
        // absolute() corner placement is demonstrated relative to this bordered
        // surface instead. (note)
        surface(border, panel).child(js_toast_host(
            &ToastHostSpec::new().with_placement(placement),
            theme,
            stack,
        ))
    };

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Bottom-end (sticky danger)",
            secondary,
            anchored(ToastHostPlacement::BottomEnd, &bottom_end),
        ))
        .child(group(
            "Bottom-start",
            secondary,
            anchored(ToastHostPlacement::BottomStart, &bottom_start),
        ))
        .child(group(
            "Top-end",
            secondary,
            anchored(ToastHostPlacement::TopEnd, &top_end),
        ))
        .child(group(
            "Top-start",
            secondary,
            anchored(ToastHostPlacement::TopStart, &top_start),
        ))
        .child(group(
            "With action",
            secondary,
            anchored(ToastHostPlacement::BottomEnd, &with_action),
        ))
}

/// Relative bordered surface a host anchors inside.
fn surface(border: ColorValue, panel: ColorValue) -> El {
    div()
        .relative()
        .min_h(220.0)
        .border_1()
        .border_color(border)
        .rounded(8.0)
        .bg(panel)
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
