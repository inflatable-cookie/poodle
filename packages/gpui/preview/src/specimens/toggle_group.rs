use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, ToggleGroup};
use poodle_specs::{EyebrowSpec, ToggleGroupOption, ToggleGroupSelectionMode};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let single_value = state
        .specimens
        .text
        .get("toggle-group-single")
        .cloned()
        .unwrap_or_else(|| "grid".to_string());
    let four_value = state
        .specimens
        .text
        .get("toggle-group-four")
        .cloned()
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

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Single selection"),
                    theme,
                ))
                .child(
                    ToggleGroup::new(single_options, theme)
                        .aria_label("View mode")
                        .default_value(vec![single_value.clone()])
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state
                                .specimens
                                .text
                                .insert("toggle-group-single".to_string(), val.to_string());
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("View: {}", single_value)),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Four options"),
                    theme,
                ))
                .child(
                    ToggleGroup::new(four_options, theme)
                        .aria_label("Text alignment")
                        .default_value(vec![four_value])
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state
                                .specimens
                                .text
                                .insert("toggle-group-four".to_string(), val.to_string());
                            cx.notify();
                        })),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Multiple selection"),
                    theme,
                ))
                .child(
                    ToggleGroup::new(multi_options, theme)
                        .aria_label("Filter tags")
                        .default_value(vec!["design".to_string(), "docs".to_string()])
                        .selection_mode(ToggleGroupSelectionMode::Multiple),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child("Selected: design, docs"),
                ),
        )
        // --- Allow deactivation (single mode clears on re-select) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Allow deactivation"),
                    theme,
                ))
                .child(
                    ToggleGroup::new(
                        vec![
                            ToggleGroupOption::new("grid", "Grid"),
                            ToggleGroupOption::new("list", "List"),
                            ToggleGroupOption::new("board", "Board"),
                        ],
                        theme,
                    )
                    .aria_label("Optional view mode")
                    .default_value(vec!["grid".to_string()])
                    .allow_deactivation(true),
                ),
        )
        // --- Disabled group ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(
                    ToggleGroup::new(disabled_options, theme)
                        .aria_label("Disabled toggle group")
                        .default_value(vec!["list".to_string()])
                        .disabled(true),
                ),
        )
        // --- Disabled item ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled item"),
                    theme,
                ))
                .child(
                    ToggleGroup::new(
                        vec![
                            ToggleGroupOption::new("grid", "Grid"),
                            ToggleGroupOption::new("list", "List").with_disabled(true),
                            ToggleGroupOption::new("board", "Board"),
                        ],
                        theme,
                    )
                    .aria_label("Toggle group with disabled item")
                    .default_value(vec!["grid".to_string()]),
                ),
        )
        .into_any_element();

    let make_opts = || {
        vec![
            ToggleGroupOption::new("grid", "Grid"),
            ToggleGroupOption::new("list", "List"),
            ToggleGroupOption::new("board", "Board"),
        ]
    };

    specimen_layout(
        state,
        cx,
        "toggle-group",
        examples,
        move |size, theme: &GpuiThemeProvider| {
            ToggleGroup::new(make_opts(), theme)
                .default_value(vec!["grid".to_string()])
                .size(size)
                .into_any_element()
        },
        move |density, theme: &GpuiThemeProvider| {
            ToggleGroup::new(make_opts(), theme)
                .default_value(vec!["grid".to_string()])
                .density(density)
                .into_any_element()
        },
    )
}
