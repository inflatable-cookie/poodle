use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{SelectSpec, ChoiceOption, EyebrowSpec};
use poodle_gpui_components::{Select, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let fruit_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("apple", "Apple"),
        ChoiceOption::new("banana", "Banana"),
        ChoiceOption::new("cherry", "Cherry"),
        ChoiceOption::new("dragonfruit", "Dragonfruit"),
        ChoiceOption::new("elderberry", "Elderberry"),
    ];

    // Grouped options flattened (GPUI Select does not yet support option groups)
    let grouped_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("apple", "Apple"),
        ChoiceOption::new("banana", "Banana"),
        ChoiceOption::new("cherry", "Cherry"),
        ChoiceOption::new("carrot", "Carrot"),
        ChoiceOption::new("broccoli", "Broccoli"),
        {
            let mut opt = ChoiceOption::new("spinach", "Spinach");
            opt.is_disabled = true;
            opt
        },
        ChoiceOption::new("rice", "Rice"),
        ChoiceOption::new("wheat", "Wheat"),
    ];

    let disabled_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("apple", "Apple"),
        ChoiceOption::new("banana", "Banana"),
        ChoiceOption::new("cherry", "Cherry"),
        ChoiceOption::new("dragonfruit", "Dragonfruit"),
        ChoiceOption::new("elderberry", "Elderberry"),
    ];

    let default_open = state.specimens.is_on("select-default-open");
    let default_value = state.specimens.text.get("select-default-value").cloned();
    let grouped_open = state.specimens.is_on("select-grouped-open");
    let grouped_value = state.specimens.text.get("select-grouped-value").cloned();

    div().flex().flex_col().gap(px(24.0))
        // --- Default (flat options) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default (flat options)"), theme))
                .child({
                    let mut spec = SelectSpec::new(fruit_options)
                        .with_placeholder("Choose a fruit")
                        .with_open(default_open);
                    if let Some(ref val) = default_value {
                        spec = spec.with_value(val.as_str());
                    }

                    div().flex().flex_col().gap(px(6.0)).max_w(px(320.0))
                        .child(
                            Select::from_spec(spec, theme)
                                .with_id("select-default")
                                .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                                    this.state.specimens.toggle("select-default-open");
                                    cx.notify();
                                }))
                                .on_change(cx.listener(|this, val: &str, _w, cx| {
                                    this.state.specimens.text.insert("select-default-value".to_string(), val.to_string());
                                    this.state.specimens.toggles.insert("select-default-open".to_string(), false);
                                    cx.notify();
                                }))
                        )
                        .when(default_value.is_some(), |d| {
                            d.child(
                                div().text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", default_value.as_deref().unwrap_or("")))
                            )
                        })
                })
        )
        // --- Grouped options ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Grouped options"), theme))
                .child({
                    let mut spec = SelectSpec::new(grouped_options)
                        .with_placeholder("Choose a food")
                        .with_open(grouped_open);
                    if let Some(ref val) = grouped_value {
                        spec = spec.with_value(val.as_str());
                    }

                    div().flex().flex_col().gap(px(6.0)).max_w(px(320.0))
                        .child(
                            Select::from_spec(spec, theme)
                                .with_id("select-grouped")
                                .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
                                    this.state.specimens.toggle("select-grouped-open");
                                    cx.notify();
                                }))
                                .on_change(cx.listener(|this, val: &str, _w, cx| {
                                    this.state.specimens.text.insert("select-grouped-value".to_string(), val.to_string());
                                    this.state.specimens.toggles.insert("select-grouped-open".to_string(), false);
                                    cx.notify();
                                }))
                        )
                        .when(grouped_value.is_some(), |d| {
                            d.child(
                                div().text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", grouped_value.as_deref().unwrap_or("")))
                            )
                        })
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = SelectSpec::new(disabled_options)
                        .with_placeholder("Choose a fruit")
                        .with_value("banana");
                    spec.is_disabled = true;

                    div().max_w(px(320.0))
                        .child(
                            Select::from_spec(spec, theme)
                                .with_id("select-disabled")
                        )
                })
        )
}
