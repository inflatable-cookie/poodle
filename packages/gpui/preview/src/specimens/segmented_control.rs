use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{SegmentedControlSpec, ChoiceOption};
use pug_gpui_components::SegmentedControl;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- Default: Grid / List / Table, value="grid" ---
    let view_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("grid", "Grid"),
        ChoiceOption::new("list", "List"),
        ChoiceOption::new("table", "Table"),
    ];

    let selected = state.specimens.selected("segmented");
    let selected_value = match selected {
        0 => "grid",
        1 => "list",
        2 => "table",
        _ => "grid",
    };

    let default_spec = SegmentedControlSpec::new(view_options.clone())
        .with_default_value(selected_value);

    // --- With disabled option: All / Active / Archived / Draft (disabled), defaultValue="all" ---
    let status_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("all", "All"),
        ChoiceOption::new("active", "Active"),
        ChoiceOption::new("archived", "Archived"),
        ChoiceOption::new("draft", "Draft").with_disabled(true),
    ];

    let disabled_opt_spec = SegmentedControlSpec::new(status_options)
        .with_default_value("all");

    // --- Fully disabled: Grid / List / Table, defaultValue="list", isDisabled ---
    let mut fully_disabled_spec = SegmentedControlSpec::new(view_options)
        .with_default_value("list");
    fully_disabled_spec.is_disabled = true;

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("DEFAULT", text_secondary))
                .child(
                    SegmentedControl::from_spec(default_spec, theme)
                        .with_id("seg-default")
                        .on_change(cx.listener(|this, value: &str, _w, cx| {
                            let idx = match value {
                                "grid" => 0,
                                "list" => 1,
                                "table" => 2,
                                _ => 0,
                            };
                            this.state.specimens.select("segmented", idx);
                            cx.notify();
                        }))
                )
        )
        // --- With disabled option ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("WITH DISABLED OPTION", text_secondary))
                .child(
                    SegmentedControl::from_spec(disabled_opt_spec, theme)
                        .with_id("seg-disabled-opt")
                )
        )
        // --- Fully disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(section_label("FULLY DISABLED", text_secondary))
                .child(
                    SegmentedControl::from_spec(fully_disabled_spec, theme)
                        .with_id("seg-fully-disabled")
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
