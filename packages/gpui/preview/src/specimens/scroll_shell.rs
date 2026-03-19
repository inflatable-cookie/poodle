use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{ScrollShellSpec, Direction};
use pug_gpui_components::PugScrollShell;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let border = theme.resolve_color("semantic.color.border.default");

    let surface_row = |label: &str| {
        div()
            .h(px(24.0))
            .rounded(px(3.0))
            .border_1()
            .border_color(color_to_hsla(border))
            .px(px(8.0))
            .flex()
            .items_center()
            .child(
                div().text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child(label.to_string()),
            )
    };

    div().flex().flex_col().gap(px(16.0))
        // --- Vertical scroll ---
        .child(section_label("VERTICAL SCROLL", text_secondary))
        .child(
            div().h(px(160.0)).child(
                PugScrollShell::new(
                    ScrollShellSpec::new()
                        .with_direction(Direction::Vertical)
                        .with_label("Scrollable content"),
                    theme,
                )
                .with_child(surface_row("Item 1"))
                .with_child(surface_row("Item 2"))
                .with_child(surface_row("Item 3"))
                .with_child(surface_row("Item 4"))
                .with_child(surface_row("Item 5"))
                .with_child(surface_row("Item 6"))
                .with_child(surface_row("Item 7"))
                .with_child(surface_row("Item 8"))
                .with_child(surface_row("Item 9"))
                .with_child(surface_row("Item 10"))
                .with_child(surface_row("Item 11"))
                .with_child(surface_row("Item 12"))
            )
        )
        // --- Horizontal scroll ---
        .child(section_label("HORIZONTAL SCROLL", text_secondary))
        .child(
            div().h(px(40.0)).child(
                PugScrollShell::new(
                    ScrollShellSpec::new()
                        .with_direction(Direction::Horizontal)
                        .with_label("Horizontal items"),
                    theme,
                )
                .with_child(
                    div().flex().gap(px(4.0))
                        .child(column_item("Column 1", border, text_secondary))
                        .child(column_item("Column 2", border, text_secondary))
                        .child(column_item("Column 3", border, text_secondary))
                        .child(column_item("Column 4", border, text_secondary))
                        .child(column_item("Column 5", border, text_secondary))
                        .child(column_item("Column 6", border, text_secondary))
                        .child(column_item("Column 7", border, text_secondary))
                        .child(column_item("Column 8", border, text_secondary))
                        .child(column_item("Column 9", border, text_secondary))
                        .child(column_item("Column 10", border, text_secondary))
                )
            )
        )
}

fn column_item(
    label: &str,
    border: pug_tokens::typed::ColorValue,
    text: pug_tokens::typed::ColorValue,
) -> Div {
    div()
        .h(px(28.0))
        .px(px(12.0))
        .rounded(px(3.0))
        .border_1()
        .border_color(color_to_hsla(border))
        .flex()
        .items_center()
        .flex_shrink_0()
        .child(
            div().text_xs()
                .text_color(color_to_hsla(text))
                .whitespace_nowrap()
                .child(label.to_string()),
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
