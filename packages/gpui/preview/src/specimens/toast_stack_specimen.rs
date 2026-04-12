use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, Eyebrow, ToastStack};
use poodle_specs::{ButtonSpec, ButtonVariant, ControlSize, EyebrowSpec};
use poodle_specs::{Toast, ToastStackSpec, ToastTone};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let control_sizes = [
        ControlSize::Xs,
        ControlSize::Sm,
        ControlSize::Md,
        ControlSize::Lg,
        ControlSize::Xl,
    ];

    let interactive_toasts = vec![
        Toast::new("1", "Changes saved")
            .with_tone(ToastTone::Success)
            .with_message("Your settings have been updated."),
        Toast::new("2", "New version available")
            .with_message("Update to v2.1 for the latest features.")
            .with_action_label("Update"),
        Toast::new("3", "Rate limit warning")
            .with_tone(ToastTone::Warning)
            .with_message("You are approaching your API limit."),
    ];

    let mut size_stack = div().flex().flex_col().gap(px(16.0));
    for size in control_sizes {
        let size_label = match size {
            ControlSize::Xs => "xs",
            ControlSize::Sm => "sm",
            ControlSize::Md => "md",
            ControlSize::Lg => "lg",
            ControlSize::Xl => "xl",
        };
        size_stack = size_stack.child(
            div().relative().min_h(px(120.0)).child(
                ToastStack::from_spec(
                    ToastStackSpec::new().with_toasts(vec![Toast::new(
                        format!("size-{size:?}"),
                        format!("Toast at {size_label}"),
                    )
                    .with_message("Chrome scales with size.")]),
                    theme,
                )
                .with_size(size),
            ),
        );
    }

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
                    EyebrowSpec::new().with_content("Sizes"),
                    theme,
                ))
                .child(size_stack),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Interactive stack"),
                    theme,
                ))
                .child(Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Add toast"),
                    theme,
                ))
                .child(
                    div()
                        .relative()
                        .min_h(px(192.0))
                        .child(ToastStack::from_spec(
                            ToastStackSpec::new().with_toasts(interactive_toasts),
                            theme,
                        )),
                ),
        )
}
