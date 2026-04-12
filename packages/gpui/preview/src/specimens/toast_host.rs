use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, Eyebrow, ToastHost};
use poodle_specs::{
    ButtonSpec, ButtonVariant, EyebrowSpec, Toast, ToastHostPlacement, ToastHostSpec, ToastTone,
};

use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let toasts = vec![
        Toast::new("1", "Saved")
            .with_tone(ToastTone::Success)
            .with_message("Your changes have been stored."),
        Toast::new("2", "Retry later")
            .with_tone(ToastTone::Warning)
            .with_message("Background sync is delayed."),
        Toast::new("3", "Publishing failed")
            .with_tone(ToastTone::Danger)
            .with_message("This one stays until you dismiss it."),
    ];

    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Runtime host"), theme))
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
        .child(
            div()
                .relative()
                .min_h(px(256.0))
                .border_1()
                .border_color(color_to_hsla(theme.resolve_color("color.border.default")))
                .rounded(px(8.0))
                .bg(color_to_hsla(theme.resolve_color("color.background.panel")))
                .child(
                    ToastHost::from_spec(
                        ToastHostSpec::new().with_placement(ToastHostPlacement::BottomEnd),
                        theme,
                    )
                    .toasts(toasts),
                ),
        )
}
