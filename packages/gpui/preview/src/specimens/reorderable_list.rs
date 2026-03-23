use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::ReorderableListSpec;
use pug_gpui_components::ReorderableList;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let border = theme.resolve_color("semantic.color.border.subtle");

    div().flex().flex_col().gap(px(24.0)).max_w(px(400.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            ReorderableList::from_spec(ReorderableListSpec::new(), theme)
                .with_child(list_item("Design tokens", text_primary, border))
                .with_child(list_item("Typography", text_primary, border))
                .with_child(list_item("Colors", text_primary, border))
                .with_child(list_item("Spacing", text_primary, border))
                .with_child(list_item("Shadows", text_primary, border))
        )
        // --- Short list ---
        .child(section_label("SHORT LIST", text_secondary))
        .child(
            ReorderableList::from_spec(ReorderableListSpec::new(), theme)
                .with_child(list_item("First", text_primary, border))
                .with_child(list_item("Second", text_primary, border))
                .with_child(list_item("Third", text_primary, border))
        )
}

fn list_item(
    label: &str,
    text_color: pug_tokens::typed::ColorValue,
    border_color: pug_tokens::typed::ColorValue,
) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(8.0))
        .px(px(12.0))
        .py(px(8.0))
        .border_b_1()
        .border_color(color_to_hsla(border_color).opacity(0.5))
        .child(
            div()
                .text_sm()
                .text_color(color_to_hsla(text_color))
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
