use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, Eyebrow, ToastHost};
use poodle_specs::{ButtonSpec, ButtonVariant, EyebrowSpec};
use poodle_specs::{Toast, ToastHostSpec, ToastTone};

use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let toasts = vec![
        Toast::new("1", "Saved")
            .with_tone(ToastTone::Success)
            .with_message("Your changes have been stored."),
        Toast::new("2", "Retry later")
            .with_tone(ToastTone::Warning)
            .with_message("Background sync is delayed."),
        Toast::new("3", "Publishing failed").with_message("Check your connection."),
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
        .child(
            div()
                .relative()
                .min_h(px(256.0))
                .border_1()
                .border_dashed()
                .border_color(color_to_hsla(theme.resolve_color("color.border.default")).opacity(0.82))
                .rounded(px(8.0))
                .bg(color_to_hsla(theme.resolve_color("color.background.panel")).opacity(0.96))
                .child(ToastHost::from_spec(ToastHostSpec::new(), theme).toasts(toasts)),
        )
}
