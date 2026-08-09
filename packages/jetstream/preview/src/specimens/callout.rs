//! Callout specimen — exercises the real `js_callout` component across the full
//! contract state set: all six tones (incl. neutral + pending spinner), message
//! prop, dismissible (with the x control), without-title, density, sizes, and a
//! real action button. Every visual resolves from `CallOutSpec` + tokens.

use crate::compat::js_button;
use crate::compat::js_callout;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ButtonSpec, ButtonVariant, CallOutSpec, ControlDensity, ControlSize, StatusTone,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // All six tones, stacked.
        .child(group("Tones", secondary,
            div().flex_col().gap(8.0)
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Neutral)
                    .with_title("Neutral callout")
                    .with_content("A general informational message with no specific severity."), theme))
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_title("Info")
                    .with_content("Your changes have been saved and will take effect on next deploy."), theme))
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Success)
                    .with_title("Success")
                    .with_content("All tests passed. The build is ready for production."), theme))
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Warning)
                    .with_title("Warning")
                    .with_content("This API key expires in 7 days. Rotate it to avoid service interruption."), theme))
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Danger)
                    .with_title("Error")
                    .with_content("Unable to connect to the database. Check your credentials and try again."), theme))
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Pending)
                    .with_title("Pending")
                    .with_content("Provisioning resources. This may take a moment."), theme))
        ))
        // Message prop — title + paragraph message.
        .child(group("Message prop", secondary,
            js_callout(&CallOutSpec::new()
                .with_tone(StatusTone::Info)
                .with_title("Information")
                .with_content("This is an informational callout using the message prop instead of slot content."), theme)
        ))
        // Dismissible — shows the x control.
        .child(group("Dismissible", secondary,
            js_callout(&CallOutSpec::new()
                .with_tone(StatusTone::Info)
                .with_title("Dismissible callout")
                .with_content("This callout can be dismissed by the user.")
                .dismissible(true), theme)
        ))
        // Without title — icon + body only.
        .child(group("Without title", secondary,
            js_callout(&CallOutSpec::new()
                .with_tone(StatusTone::Info)
                .with_content("A simple inline callout without a title for brief contextual notes."), theme)
        ))
        // With action — Callout has no actions slot; pair with a real Button.
        .child(group("With action", secondary,
            div().flex_col().gap(8.0)
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Warning)
                    .with_title("Quota warning")
                    .with_content("API usage is approaching the current workspace limit."), theme))
                .child(div().flex_row().justify_end()
                    .child(js_button(&ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Review limits"), theme)))
        ))
        // Sizes — chrome scales with the size prop.
        .child(group("Sizes", secondary,
            div().flex_col().gap(8.0)
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_size(ControlSize::Sm)
                    .with_title("Callout at sm")
                    .with_content("Text and icon chrome scale with the size prop."), theme))
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_size(ControlSize::Md)
                    .with_title("Callout at md")
                    .with_content("Text and icon chrome scale with the size prop."), theme))
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_size(ControlSize::Lg)
                    .with_title("Callout at lg")
                    .with_content("Text and icon chrome scale with the size prop."), theme))
        ))
        // Density — internal spacing adjusts.
        .child(group("Density", secondary,
            div().flex_col().gap(8.0)
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_density(ControlDensity::Compact)
                    .with_title("Compact density")
                    .with_content("Internal spacing adjusts with the density prop."), theme))
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_density(ControlDensity::Default)
                    .with_title("Default density")
                    .with_content("Internal spacing adjusts with the density prop."), theme))
                .child(js_callout(&CallOutSpec::new()
                    .with_tone(StatusTone::Info)
                    .with_density(ControlDensity::Comfortable)
                    .with_title("Comfortable density")
                    .with_content("Internal spacing adjusts with the density prop."), theme))
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
