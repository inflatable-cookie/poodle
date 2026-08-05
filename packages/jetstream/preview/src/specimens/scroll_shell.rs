//! ScrollShell specimen — scrollable container (viewport → content → children).
//!
//! Mirrors the GPUI specimen: a vertical scroll with enough rows to overflow,
//! and a horizontal scroll with a wide column row. Both render the real
//! three-layer `js_scroll_shell` (viewport owns overflow, content wraps,
//! children slot in) with direction + label from `ScrollShellSpec`.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_scroll_shell;
use crate::compat::{rem_to_px, size_font_rem};

use poodle_specs::{ControlSize, Direction, ScrollShellSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.default");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));

    // Token-styled scroll rows for the vertical example.
    let row = move |text: String| -> El {
        div().h(24.0).px(8.0).rounded(3.0)
            .border_1().border_color(border)
            .flex_row().items_center()
            .child(label(text).text_color(secondary).text_size(body_font))
    };
    // Non-shrinking column items for the horizontal example.
    let column = move |text: String| -> El {
        div().h(28.0).px(12.0).rounded(3.0)
            .border_1().border_color(border)
            .flex_row().items_center().flex_shrink_0()
            .child(label(text).text_color(secondary).text_size(body_font).whitespace_nowrap())
    };

    let vertical_rows: Vec<El> = (1..=12).map(|i| row(format!("Item {i}"))).collect();

    // Horizontal content is a single wide row of columns (the content layer
    // sizes to its children so the viewport scrolls on the x axis).
    let column_row = {
        let mut r = div().flex_row().gap(4.0);
        for i in 1..=10 {
            r = r.child(column(format!("Column {i}")));
        }
        r
    };

    div().flex_col().gap(24.0)
        .child(group("Vertical scroll", secondary,
            div().h(160.0).child(js_scroll_shell(
                &ScrollShellSpec::new()
                    .with_direction(Direction::Vertical)
                    .with_label("Scrollable content"),
                theme,
                vertical_rows,
            ))
        ))
        .child(group("Horizontal scroll", secondary,
            div().h(48.0).child(js_scroll_shell(
                &ScrollShellSpec::new()
                    .with_direction(Direction::Horizontal)
                    .with_label("Horizontal items"),
                theme,
                vec![column_row],
            ))
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
