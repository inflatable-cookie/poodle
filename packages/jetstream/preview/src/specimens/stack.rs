//! Stack specimen — vertical stack layouts with different gaps and alignment.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::stack::js_stack;
use flint_jetstream_components::theme_ext::*;
use flint_primitives::{Alignment, PaddingScale, StackSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let accent = resolve_color(theme, "semantic.color.accent.base");

    let item = |text: &str| label(text).text_color(text_primary).text_size(13.0)
        .px(8.0).py(4.0).bg(tint(accent, 0.12)).rounded(4.0);

    div().flex_col().gap(24.0)
        .child(group("Default gap", secondary,
            js_stack(&StackSpec::new(), theme, vec![item("Item 1"), item("Item 2"), item("Item 3")])
        ))
        .child(group("Small gap", secondary,
            js_stack(&StackSpec::new().with_gap(PaddingScale::Sm), theme, vec![item("A"), item("B"), item("C")])
        ))
        .child(group("Center aligned", secondary,
            js_stack(&StackSpec::new().with_align(Alignment::Center), theme, vec![item("Centered"), item("Items")])
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
