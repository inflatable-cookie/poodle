use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Callout, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CallOutSpec, EyebrowSpec, StatusTone, ToneFill};
use std::sync::Arc;

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

fn tone_row(theme: &GpuiThemeProvider, tone: StatusTone, title: &str, content: &str) -> Callout {
    Callout::from_spec(
        CallOutSpec::new()
            .with_tone(tone)
            .with_title(title)
            .with_content(content),
        theme,
    )
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let dismissed = state.specimens.is_on("callout-dismissed");
    let events = state.node_events.clone();

    let dismissible = if dismissed {
        div().child("Dismissed.").into_any_element()
    } else {
        Callout::from_spec(
            CallOutSpec::new()
                .with_tone(StatusTone::Info)
                .with_title("Dismissible callout")
                .with_content("This callout can be dismissed by the user.")
                .dismissible(true),
            theme,
        )
        .on_dismiss(Arc::new(move || {
            events
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::Toggle("callout-dismissed".to_string()));
        }))
        .with_instance_id("live")
        .into_any_element()
    };

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Tones",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(tone_row(
                    theme,
                    StatusTone::Neutral,
                    "Neutral callout",
                    "A general informational message with no specific severity.",
                ))
                .child(tone_row(
                    theme,
                    StatusTone::Info,
                    "Info",
                    "Your changes have been saved and will take effect on next deploy.",
                ))
                .child(tone_row(
                    theme,
                    StatusTone::Success,
                    "Success",
                    "All tests passed. The build is ready for production.",
                ))
                .child(tone_row(
                    theme,
                    StatusTone::Warning,
                    "Warning",
                    "This API key expires in 7 days. Rotate it to avoid service interruption.",
                ))
                .child(tone_row(
                    theme,
                    StatusTone::Danger,
                    "Error",
                    "Unable to connect to the database. Check your credentials and try again.",
                ))
                .child(tone_row(
                    theme,
                    StatusTone::Pending,
                    "Pending",
                    "Provisioning resources. This may take a moment.",
                )),
        ))
        .child(group(
            "Message",
            theme,
            Callout::from_spec(
                CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_title("Information")
                    .with_content(
                        "This is an informational callout using the message field on the native spec.",
                    ),
                theme,
            ),
        ))
        .child(group(
            "Without title",
            theme,
            Callout::from_spec(
                CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_content(
                        "A simple inline callout without a title for brief contextual notes.",
                    ),
                theme,
            ),
        ))
        .child(group("Dismissible", theme, dismissible))
        .child(group(
            "Solid fills",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    Callout::from_spec(
                        CallOutSpec::new()
                            .with_tone(StatusTone::Neutral)
                            .with_fill(ToneFill::Solid)
                            .with_title("Solid neutral")
                            .with_content("One continuous neutral surface."),
                        theme,
                    ),
                )
                .child(
                    Callout::from_spec(
                        CallOutSpec::new()
                            .with_tone(StatusTone::Info)
                            .with_fill(ToneFill::Solid)
                            .with_title("Solid info")
                            .with_content("Tint-border colour promoted into the fill."),
                        theme,
                    ),
                )
                .child(
                    Callout::from_spec(
                        CallOutSpec::new()
                            .with_tone(StatusTone::Success)
                            .with_fill(ToneFill::Solid)
                            .with_title("Solid success")
                            .with_content("One continuous solid colour."),
                        theme,
                    ),
                )
                .child(
                    Callout::from_spec(
                        CallOutSpec::new()
                            .with_tone(StatusTone::Warning)
                            .with_fill(ToneFill::Solid)
                            .with_title("Solid warning")
                            .with_content("Tint-border colour promoted into the fill."),
                        theme,
                    ),
                )
                .child(
                    Callout::from_spec(
                        CallOutSpec::new()
                            .with_tone(StatusTone::Danger)
                            .with_fill(ToneFill::Solid)
                            .with_title("Solid danger")
                            .with_content("One continuous solid colour."),
                        theme,
                    ),
                )
                .child(
                    Callout::from_spec(
                        CallOutSpec::new()
                            .with_tone(StatusTone::Pending)
                            .with_fill(ToneFill::Solid)
                            .with_title("Solid pending")
                            .with_content("Pending spinner inherits the solid foreground."),
                        theme,
                    ),
                ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "callout",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                Callout::from_spec(
                    CallOutSpec::new()
                        .with_title("Neutral callout")
                        .with_content("A general informational message with no specific severity.")
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                Callout::from_spec(
                    CallOutSpec::new()
                        .with_title("Neutral callout")
                        .with_content("A general informational message with no specific severity.")
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
