//! Stack specimen — vertical stack layouts with different gaps and alignment.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::stack::js_stack;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{Alignment, ControlSize, LayoutJustify, PaddingScale, StackDirection, StackSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");
    let accent = resolve_color(theme, "color.accent.base");

    let item = |text: &str| label(text).text_color(text_primary).text_size(body_font)
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
        .child(group("Row direction", secondary,
            js_stack(&StackSpec::new().with_direction(StackDirection::Row), theme,
                vec![item("Left"), item("Middle"), item("Right")])
        ))
        .child(group("Row justify=between", secondary,
            js_stack(
                &StackSpec::new()
                    .with_direction(StackDirection::Row)
                    .with_justify(LayoutJustify::SpaceBetween),
                theme,
                vec![item("Start"), item("End")],
            )
        ))
        .child(group("Row wrapping", secondary,
            js_stack(
                &StackSpec::new()
                    .with_direction(StackDirection::Row)
                    .with_wrap(true),
                theme,
                vec![item("One"), item("Two"), item("Three"), item("Four"), item("Five"), item("Six")],
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
