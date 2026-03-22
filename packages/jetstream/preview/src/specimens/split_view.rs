//! SplitView specimen — two-pane split layout.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::split_view::js_split_view;
use pug_jetstream_components::theme_ext::*;
use pug_composites::{SplitOrientation, SplitViewSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let bg = resolve_color(theme, "semantic.color.background.surface");
    let border = resolve_color(theme, "semantic.color.border.subtle");

    div().flex_col().gap(24.0)
        .child(group("Horizontal split", secondary,
            div().h(150.0).child(
                js_split_view(
                    &SplitViewSpec::new(SplitOrientation::Horizontal),
                    theme,
                    Some(
                        div().grow().bg(bg).border_1().border_color(border).p(8.0)
                            .child(label("Primary").text_color(text_primary).text_size(13.0))
                    ),
                    Some(
                        div().grow().bg(bg).border_1().border_color(border).p(8.0)
                            .child(label("Secondary").text_color(secondary).text_size(13.0))
                    ),
                )
            )
        ))
        .child(group("Vertical split", secondary,
            div().h(200.0).child(
                js_split_view(
                    &SplitViewSpec::new(SplitOrientation::Vertical),
                    theme,
                    Some(
                        div().grow().bg(bg).border_1().border_color(border).p(8.0)
                            .child(label("Top").text_color(text_primary).text_size(13.0))
                    ),
                    Some(
                        div().grow().bg(bg).border_1().border_color(border).p(8.0)
                            .child(label("Bottom").text_color(secondary).text_size(13.0))
                    ),
                )
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
