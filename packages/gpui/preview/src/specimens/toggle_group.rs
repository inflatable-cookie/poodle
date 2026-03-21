use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_components::{ToggleGroup, ToggleGroupItem};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- Single selection: Grid (selected) / List / Board ---
    let single_items = vec![
        ToggleGroupItem::new("Grid").with_pressed(true),
        ToggleGroupItem::new("List").with_pressed(false),
        ToggleGroupItem::new("Board").with_pressed(false),
    ];

    // --- Four options: Left (selected) / Center / Right / Justify ---
    let four_items = vec![
        ToggleGroupItem::new("Left").with_pressed(true),
        ToggleGroupItem::new("Center").with_pressed(false),
        ToggleGroupItem::new("Right").with_pressed(false),
        ToggleGroupItem::new("Justify").with_pressed(false),
    ];

    // --- Multiple selection: Design + Docs selected, Engineering not ---
    let multi_items = vec![
        ToggleGroupItem::new("Design").with_pressed(true),
        ToggleGroupItem::new("Engineering").with_pressed(false),
        ToggleGroupItem::new("Docs").with_pressed(true),
    ];

    // --- Disabled: List selected, all disabled ---
    let disabled_items = vec![
        ToggleGroupItem::new("Grid").with_pressed(false),
        ToggleGroupItem::new("List").with_pressed(true),
        ToggleGroupItem::new("Board").with_pressed(false),
    ];

    div().flex().flex_col().gap(px(24.0))
        // --- Single selection ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("SINGLE SELECTION", text_secondary))
                .child(ToggleGroup::new(single_items, theme))
        )
        // --- Four options ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("FOUR OPTIONS", text_secondary))
                .child(ToggleGroup::new(four_items, theme))
        )
        // --- Multiple selection ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("MULTIPLE SELECTION", text_secondary))
                .child(ToggleGroup::new(multi_items, theme))
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("DISABLED", text_secondary))
                .child(
                    div().opacity(0.48)
                        .child(ToggleGroup::new(disabled_items, theme))
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
