use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Button, Callout, Eyebrow};
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, ButtonVariant, CallOutSpec, EyebrowSpec, StatusTone};
use std::sync::Arc;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let dismissed_info = state.specimens.is_on("callout-dismissed-info");
    let dismissed_warning = state.specimens.is_on("callout-dismissed-warning");
    let info_dismiss = Arc::new({
        let events = state.node_events.clone();
        move || {
            events.lock().unwrap().push(NodeSpecimenEvent::Toggle(
                "callout-dismissed-info".to_string(),
            ));
        }
    });
    let warning_dismiss = Arc::new({
        let events = state.node_events.clone();
        move || {
            events.lock().unwrap().push(NodeSpecimenEvent::Toggle(
                "callout-dismissed-warning".to_string(),
            ));
        }
    });

    let examples = div().flex().flex_col().gap(px(24.0))
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
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Pending)
                                .with_title("Pending")
                                .with_content("Provisioning resources. This may take a moment."),
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
                            .on_dismiss(info_dismiss.clone())
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
                            .on_dismiss(warning_dismiss.clone())
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

        // --- With action ---
        // The Callout primitive has no actions slot; pair it with a real Button.
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With action"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(Callout::from_spec(
                            CallOutSpec::new()
                                .with_tone(StatusTone::Warning)
                                .with_title("Quota warning")
                                .with_content("API usage is approaching the current workspace limit."),
                            theme,
                        ))
                        .child(
                            div().flex().justify_end().child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Secondary)
                                        .with_label("Review limits"),
                                    theme,
                                )
                                .with_id("callout-action-button"),
                            ),
                        )
                )
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "callout",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Callout::from_spec(
                CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_title("Callout")
                    .with_content("An informational callout."),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            Callout::from_spec(
                CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_title("Callout")
                    .with_content("An informational callout."),
                theme,
            )
            .with_density(density)
            .into_any_element()
        },
    )
}
