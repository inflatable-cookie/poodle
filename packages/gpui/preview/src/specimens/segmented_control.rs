use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{SegmentedControlSpec, ChoiceOption};
use pug_gpui_components::SegmentedControl;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
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

    let selected_value = state.specimens.text.get("segmented-value")
        .map(|s| s.as_str())
        .unwrap_or("grid")
        .to_string();

    let default_spec = SegmentedControlSpec::new(view_options.clone())
        .with_default_value(&selected_value);

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
                            this.state.specimens.text.insert("segmented-value".to_string(), value.to_string());
                            cx.notify();
                        }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Selected: {}", selected_value))
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
