use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, Eyebrow, ToastStack};
use poodle_specs::{ButtonSpec, ButtonVariant, ControlDensity, ControlSize, EyebrowSpec};
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

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
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

    // ── Sizes: xs–xl, each an info toast + a success toast with an action,
    // mirroring the Svelte sizes snippet so the size ladder is visible. ──
    let control_sizes = [
        (ControlSize::Xs, "xs"),
        (ControlSize::Sm, "sm"),
        (ControlSize::Md, "md"),
        (ControlSize::Lg, "lg"),
        (ControlSize::Xl, "xl"),
    ];
    let mut size_stack = div().flex().flex_col().gap(px(16.0));
    for (size, size_label) in control_sizes {
        let toasts = vec![
            Toast::new(format!("size-{size_label}-1"), format!("Toast at {size_label}"))
                .with_tone(ToastTone::Info)
                .with_message("Chrome scales with size."),
            Toast::new(format!("size-{size_label}-2"), "Action available")
                .with_tone(ToastTone::Success)
                .with_message("Dismiss and action controls follow the same ladder.")
                .with_action_label("View"),
        ];
        size_stack = size_stack.child(surface(
            ToastStack::from_spec(ToastStackSpec::new().with_toasts(toasts), theme).with_size(size),
            120.0,
        ));
    }

    // ── Densities: compact / default / comfortable spacing ladder. ──
    let densities = [
        (ControlDensity::Compact, "compact"),
        (ControlDensity::Default, "default"),
        (ControlDensity::Comfortable, "comfortable"),
    ];
    let mut density_stack = div().flex().flex_col().gap(px(16.0));
    for (density, density_label) in densities {
        let toasts = vec![
            Toast::new(format!("density-{density_label}-1"), "Density example")
                .with_tone(ToastTone::Warning)
                .with_message("Spacing changes between compact, default, and comfortable."),
            Toast::new(format!("density-{density_label}-2"), "Retry failed")
                .with_tone(ToastTone::Danger)
                .with_message("Action row and body spacing should ladder correctly.")
                .with_action_label("Retry"),
        ];
        density_stack = density_stack.child(surface(
            ToastStack::from_spec(ToastStackSpec::new().with_toasts(toasts), theme)
                .with_density(density),
            120.0,
        ));
    }

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(theme, "Tones", surface(
            ToastStack::from_spec(ToastStackSpec::new().with_toasts(tone_toasts), theme),
            260.0,
        )))
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
        .child(group(theme, "Sizes", size_stack))
        .child(group(theme, "Densities", density_stack))
}
