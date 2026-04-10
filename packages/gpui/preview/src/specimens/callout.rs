use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{CallOutSpec, ControlDensity, ControlSize, StatusTone, EyebrowSpec};
use poodle_gpui_components::{Callout, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let dismissed_info = state.specimens.is_on("callout-dismissed-info");
    let dismissed_warning = state.specimens.is_on("callout-dismissed-warning");

    div().flex().flex_col().gap(px(24.0))
        // --- Tones ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Tones"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Neutral)
                                .with_title("Neutral callout")
                                .with_content("This is a general informational message with no specific severity."),
                            theme,
                        ))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Info)
                                .with_title("Info")
                                .with_content("Your changes have been saved and will take effect on next deploy."),
                            theme,
                        ))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Success)
                                .with_title("Success")
                                .with_content("All tests passed. The build is ready for production."),
                            theme,
                        ))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Warning)
                                .with_title("Warning")
                                .with_content("This API key expires in 7 days. Rotate it to avoid service interruption."),
                            theme,
                        ))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Danger)
                                .with_title("Error")
                                .with_content("Unable to connect to the database. Check your credentials and try again."),
                            theme,
                        ))
                )
        )

        // --- Message prop ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Message prop"), theme))
                .child(Callout::from_spec(
                    CallOutSpec::new()
                        .with_tone(StatusTone::Info)
                        .with_title("Information")
                        .with_content("This is an informational callout using the message prop instead of slot content."),
                    theme,
                ))
        )

        // --- Dismissible ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Dismissible"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(if !dismissed_info {
                            Callout::from_spec(
                                CallOutSpec::new()
                                    .with_tone(StatusTone::Info)
                                    .with_title("Dismissible callout")
                                    .with_content("This callout can be dismissed by the user."),
                                theme,
                            )
                            .on_dismiss(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggles.insert("callout-dismissed-info".to_string(), true);
                                cx.notify();
                            }))
                            .into_any_element()
                        } else {
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child("Info callout dismissed.")
                                .into_any_element()
                        })
                        .child(if !dismissed_warning {
                            Callout::from_spec(
                                CallOutSpec::new()
                                    .with_tone(StatusTone::Warning)
                                    .with_title("Dismissible warning")
                                    .with_content("This warning can also be dismissed."),
                                theme,
                            )
                            .on_dismiss(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state.specimens.toggles.insert("callout-dismissed-warning".to_string(), true);
                                cx.notify();
                            }))
                            .into_any_element()
                        } else {
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child("Warning callout dismissed.")
                                .into_any_element()
                        })
                )
        )

        // --- Without title ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Without title"), theme))
                .child(
                    Callout::from_spec(
                        CallOutSpec::new()
                            .with_tone(StatusTone::Info)
                            .with_content("A simple inline callout without a title for brief contextual notes."),
                        theme,
                    )
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Info)
                                .with_title("Xs size")
                                .with_content("Extra small callout."),
                            theme,
                        ).size(ControlSize::Xs))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Info)
                                .with_title("Sm size")
                                .with_content("Small callout."),
                            theme,
                        ).size(ControlSize::Sm))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Info)
                                .with_title("Md size")
                                .with_content("Medium callout."),
                            theme,
                        ).size(ControlSize::Md))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Info)
                                .with_title("Lg size")
                                .with_content("Large callout."),
                            theme,
                        ).size(ControlSize::Lg))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Info)
                                .with_title("Xl size")
                                .with_content("Extra large callout."),
                            theme,
                        ).size(ControlSize::Xl))
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Info)
                                .with_title("Compact")
                                .with_content("Tighter internal spacing."),
                            theme,
                        ).with_density(ControlDensity::Compact))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Info)
                                .with_title("Default")
                                .with_content("Default internal spacing."),
                            theme,
                        ).with_density(ControlDensity::Default))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Info)
                                .with_title("Comfortable")
                                .with_content("Looser internal spacing."),
                            theme,
                        ).with_density(ControlDensity::Comfortable))
                )
        )
}
