//! Toolbar specimen — action bars with different alignments.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::button::js_button;
use flint_jetstream_components::toolbar::js_toolbar;
use flint_jetstream_components::theme_ext::*;
use flint_primitives::{Alignment, ButtonSpec, ButtonVariant, ToolbarSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let btn = |label: &str, variant: ButtonVariant| {
        js_button(&ButtonSpec::new().with_variant(variant).with_label(label), theme)
    };

    div().flex_col().gap(24.0)
        .child(group("Start aligned (default)", secondary,
            js_toolbar(&ToolbarSpec::new(), theme, vec![
                btn("Edit", ButtonVariant::Secondary),
                btn("Delete", ButtonVariant::Ghost),
            ])
        ))
        .child(group("End aligned", secondary,
            js_toolbar(&ToolbarSpec::new().with_alignment(Alignment::End), theme, vec![
                btn("Cancel", ButtonVariant::Ghost),
                btn("Save", ButtonVariant::Primary),
            ])
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
