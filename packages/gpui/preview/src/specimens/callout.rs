use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CallOutSpec, StatusTone};
use pug_gpui_components::Callout;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Tones ---
        .child(section_label("TONES", text_secondary))
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
        // --- Message prop ---
        .child(section_label("MESSAGE PROP", text_secondary))
        .child(
            Callout::from_spec(
                CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_title("Information")
                    .with_content("This is an informational callout using the message prop instead of slot content."),
                theme,
            )
        )
        // --- Dismissible ---
        .child(section_label("DISMISSIBLE", text_secondary))
        .child(
            Callout::from_spec(
                CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_title("Dismissible callout")
                    .with_content("This callout can be dismissed by the user."),
                theme,
            )
        )
        // --- Without title ---
        .child(section_label("WITHOUT TITLE", text_secondary))
        .child(
            Callout::from_spec(
                CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_content("A simple inline callout without a title for brief contextual notes."),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
