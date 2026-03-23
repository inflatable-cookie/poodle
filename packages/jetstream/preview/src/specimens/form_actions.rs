//! FormActions specimen — form action bars.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::button::js_button;
use flint_jetstream_components::form_actions::js_form_actions;
use flint_jetstream_components::theme_ext::*;
use flint_primitives::{ButtonSpec, ButtonVariant, FormActionAlign, FormActionsSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let btn = |label: &str, variant: ButtonVariant| {
        js_button(&ButtonSpec::new().with_variant(variant).with_label(label), theme)
    };

    div().flex_col().gap(24.0)
        .child(group("End aligned (default)", secondary,
            js_form_actions(&FormActionsSpec::new(), theme, vec![
                btn("Cancel", ButtonVariant::Ghost),
                btn("Submit", ButtonVariant::Primary),
            ])
        ))
        .child(group("Start aligned", secondary,
            js_form_actions(&FormActionsSpec::new().with_align(FormActionAlign::Start), theme, vec![
                btn("Back", ButtonVariant::Secondary),
                btn("Next", ButtonVariant::Primary),
            ])
        ))
        .child(group("Between", secondary,
            js_form_actions(&FormActionsSpec::new().with_align(FormActionAlign::Between), theme, vec![
                btn("Reset", ButtonVariant::Ghost),
                btn("Save", ButtonVariant::Primary),
            ])
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
