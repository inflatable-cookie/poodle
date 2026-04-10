//! CardRadioGroup specimen — radio selection across styled cards.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::card_radio_group::js_card_radio_group;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::CardRadioGroupSpec;
use poodle_primitives::ChoiceOption;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let options = vec![
        ChoiceOption::new("free", "Free").with_description("Basic access with limits."),
        ChoiceOption::new("pro", "Professional").with_description("Full access for individuals."),
        ChoiceOption::new("team", "Team").with_description("Collaboration for groups."),
    ];

    div().flex_col().gap(24.0)
        .child(group("With selection", secondary,
            js_card_radio_group(
                &CardRadioGroupSpec::new(options.clone())
                    .with_value("pro"),
                theme,
            )
        ))
        .child(group("No selection", secondary,
            js_card_radio_group(
                &CardRadioGroupSpec::new(options),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
