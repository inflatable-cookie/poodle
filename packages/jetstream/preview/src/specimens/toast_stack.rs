//! ToastStack specimen — notification toast stack.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::theme_ext::*;
use poodle_jetstream_components::toast_stack::js_toast_stack;
use poodle_specs::{ControlDensity, ControlSize, Toast, ToastStackSpec, ToastTone};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

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
    // plus a danger toast with an action to exercise the assertive tone. ──
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

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Tones",
            secondary,
            js_toast_stack(&ToastStackSpec::new().with_toasts(tone_toasts), theme),
        ))
        .child(group(
            "Interactive stack",
            secondary,
            js_toast_stack(
                &ToastStackSpec::new().with_toasts(interactive_toasts),
                theme,
            ),
        ))
        .child(group("Sizes", secondary, sizes(theme)))
        .child(group("Densities", secondary, densities(theme)))
}

/// xs–xl size ladder, each an info toast + a success toast with an action
/// (mirrors the Svelte sizes snippet).
fn sizes(theme: &JetstreamThemeProvider) -> JsEl {
    let order = [
        (ControlSize::Xs, "xs"),
        (ControlSize::Sm, "sm"),
        (ControlSize::Md, "md"),
        (ControlSize::Lg, "lg"),
        (ControlSize::Xl, "xl"),
    ];
    let mut col = div().flex_col().gap(16.0);
    for (size, name) in order {
        let toasts = vec![
            Toast::new(format!("size-{name}-1"), format!("Toast at {name}"))
                .with_tone(ToastTone::Info)
                .with_message("Chrome scales with size."),
            Toast::new(format!("size-{name}-2"), "Action available")
                .with_tone(ToastTone::Success)
                .with_message("Dismiss and action controls follow the same ladder.")
                .with_action_label("View"),
        ];
        col = col.child(js_toast_stack(
            &ToastStackSpec::new().with_toasts(toasts).with_size(size),
            theme,
        ));
    }
    col
}

/// compact / default / comfortable density ladder.
fn densities(theme: &JetstreamThemeProvider) -> JsEl {
    let order = [
        (ControlDensity::Compact, "compact"),
        (ControlDensity::Default, "default"),
        (ControlDensity::Comfortable, "comfortable"),
    ];
    let mut col = div().flex_col().gap(16.0);
    for (density, name) in order {
        let toasts = vec![
            Toast::new(format!("density-{name}-1"), "Density example")
                .with_tone(ToastTone::Warning)
                .with_message("Spacing changes between compact, default, and comfortable."),
            Toast::new(format!("density-{name}-2"), "Retry failed")
                .with_tone(ToastTone::Danger)
                .with_message("Action row and body spacing should ladder correctly.")
                .with_action_label("Retry"),
        ];
        col = col.child(js_toast_stack(
            &ToastStackSpec::new()
                .with_toasts(toasts)
                .with_density(density),
            theme,
        ));
    }
    col
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
