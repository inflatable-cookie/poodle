use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_components::ToggleGroup;
use pug_primitives::ToggleGroupOption;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let single_value = state.specimens.text.get("toggle-group-single").cloned()
        .unwrap_or_else(|| "grid".to_string());
    let four_value = state.specimens.text.get("toggle-group-four").cloned()
        .unwrap_or_else(|| "left".to_string());

    // --- Single selection: Grid / List / Board ---
    let single_options = vec![
        ToggleGroupOption::new("grid", "Grid"),
        ToggleGroupOption::new("list", "List"),
        ToggleGroupOption::new("board", "Board"),
    ];

    // --- Four options: Left / Center / Right / Justify ---
    let four_options = vec![
        ToggleGroupOption::new("left", "Left"),
        ToggleGroupOption::new("center", "Center"),
        ToggleGroupOption::new("right", "Right"),
        ToggleGroupOption::new("justify", "Justify"),
    ];

    // --- Multiple selection ---
    let multi_options = vec![
        ToggleGroupOption::new("design", "Design"),
        ToggleGroupOption::new("engineering", "Engineering"),
        ToggleGroupOption::new("docs", "Docs"),
    ];

    // --- Disabled ---
    let disabled_options = vec![
        ToggleGroupOption::new("grid", "Grid"),
        ToggleGroupOption::new("list", "List"),
        ToggleGroupOption::new("board", "Board"),
    ];

    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("SINGLE SELECTION", text_secondary))
                .child(
                    ToggleGroup::new(single_options, theme)
                        .default_value(vec![single_value])
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("toggle-group-single".to_string(), val.to_string());
                            cx.notify();
                        }))
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("FOUR OPTIONS", text_secondary))
                .child(
                    ToggleGroup::new(four_options, theme)
                        .default_value(vec![four_value])
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("toggle-group-four".to_string(), val.to_string());
                            cx.notify();
                        }))
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("MULTIPLE SELECTION", text_secondary))
                .child(
                    ToggleGroup::new(multi_options, theme)
                        .default_value(vec!["design".to_string(), "docs".to_string()])
                        .selection_mode(pug_primitives::ToggleGroupSelectionMode::Multiple)
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("DISABLED", text_secondary))
                .child(
                    ToggleGroup::new(disabled_options, theme)
                        .default_value(vec!["list".to_string()])
                        .disabled(true)
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
