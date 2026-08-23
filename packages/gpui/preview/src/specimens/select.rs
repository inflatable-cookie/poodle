//! Select specimen — the g12.019 Batch A pilot.
//!
//! Every Select below renders through the node tier: `poodle_render::select`
//! (`Spec + Context → Node`) interpreted by `poodle_gpui_node_backend::to_gpui`.
//! The old hand-written `poodle_gpui_components::Select` no longer renders
//! this specimen; everything around the selects (layout, Eyebrow headings,
//! captions) is unchanged.
//!
//! Node interaction closures are context-free (`Arc<dyn Fn() + Send + Sync>`),
//! so instead of `cx.listener` the handlers push `NodeSpecimenEvent`s onto a
//! queue the next render drains into specimen state (see `app_state.rs`).

use crate::node_compat::{Eyebrow, Select};
use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;

use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use poodle_gpui::GpuiThemeProvider;
use poodle_render::{RenderContext, select, SelectHandlers};
use poodle_specs::{ChoiceOption, EyebrowSpec, SelectMode, SelectSpec};

/// Build a node-tier Select with the specimen's toggle/change/clear wiring.
/// Mirrors the old specimen's behavior: toggle flips `{id}-open`, change
/// records `{id}-value` and closes, clear reports the default value ("").
fn node_select(id: &'static str, spec: SelectSpec, state: &AppState) -> AnyElement {
    let events = state.node_events.clone();

    let toggle_key = format!("{}-open", id);
    let toggle = {
        let events = events.clone();
        Arc::new(move || {
            events
                .lock()
                .unwrap()
                .push(NodeSpecimenEvent::Toggle(toggle_key.clone()));
        })
    };

    let change_open_key = format!("{}-open", id);
    let change_value_key = format!("{}-value", id);
    let change = {
        let events = events.clone();
        Arc::new(move |value: &str| {
            events.lock().unwrap().push(NodeSpecimenEvent::Change {
                open_key: change_open_key.clone(),
                value_key: change_value_key.clone(),
                value: value.to_string(),
            });
        })
    };

    let clear_open_key = format!("{}-open", id);
    let clear_value_key = format!("{}-value", id);
    let clear_value = spec.default_value.clone().unwrap_or_default();
    let clear = Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::Change {
            open_key: clear_open_key.clone(),
            value_key: clear_value_key.clone(),
            value: clear_value.clone(),
        });
    });

    let handlers = SelectHandlers {
        toggle: Some(toggle),
        change: Some(change),
        clear: Some(clear),
    };
    let node = select(&spec, &RenderContext::new(&state.theme), &handlers);
    poodle_gpui_node_backend::to_gpui(&node)
}

/// A node-tier Select with no handlers (disabled / validation / sizes).
fn node_select_static(spec: SelectSpec, state: &AppState) -> AnyElement {
    let node = select(&spec, &RenderContext::new(&state.theme), &SelectHandlers::default());
    poodle_gpui_node_backend::to_gpui(&node)
}

/// The option set both the Examples pane and the axis representatives use.
fn axis_options() -> Vec<ChoiceOption> {
    vec![
        ChoiceOption::new("apple", "Apple"),
        ChoiceOption::new("banana", "Banana"),
        ChoiceOption::new("cherry", "Cherry"),
        ChoiceOption::new("dragonfruit", "Dragonfruit"),
        ChoiceOption::new("elderberry", "Elderberry"),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let fruit_options: Vec<ChoiceOption> = axis_options();

    let rich_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("us", "United States").with_description("North America"),
        ChoiceOption::new("uk", "United Kingdom").with_description("Europe"),
        ChoiceOption::new("jp", "Japan").with_description("Asia"),
        ChoiceOption::new("au", "Australia").with_description("Oceania"),
        ChoiceOption::new("br", "Brazil").with_description("South America"),
    ];

    let framework_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("svelte", "Svelte"),
        ChoiceOption::new("react", "React"),
        ChoiceOption::new("vue", "Vue"),
        ChoiceOption::new("angular", "Angular"),
        ChoiceOption::new("solid", "SolidJS"),
        ChoiceOption::new("astro", "Astro"),
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
    ];

    // Helper for reading toggle/value state
    let get_open = |key: &str| state.specimens.is_on(key);
    let get_value = |key: &str| state.specimens.text.get(key).cloned();
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Native select ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Native select"),
                    theme,
                ))
                .child({
                    let value = get_value("select-native-value");
                    let mut spec = SelectSpec::new(fruit_options.clone())
                        .with_placeholder("Choose a fruit")
                        .with_open(get_open("select-native-open"));
                    if let Some(ref v) = value {
                        spec = spec.with_value(v.as_str());
                    }
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .max_w(px(320.0))
                        .child(node_select("select-native", spec, state))
                        .when(value.is_some(), |d| {
                            d.child(
                                div()
                                    .text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", value.as_deref().unwrap_or(""))),
                            )
                        })
                }),
        )
        // --- Custom dropdown (non-searchable) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Custom dropdown"),
                    theme,
                ))
                .child({
                    let value = get_value("select-custom-value");
                    let mut spec = SelectSpec::new(rich_options.clone())
                        .with_placeholder("Choose a country")
                        .with_open(get_open("select-custom-open"));
                    spec.mode = SelectMode::Custom;
                    if let Some(ref v) = value {
                        spec = spec.with_value(v.as_str());
                    }
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .max_w(px(320.0))
                        .child(node_select("select-custom", spec, state))
                        .when(value.is_some(), |d| {
                            d.child(
                                div()
                                    .text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", value.as_deref().unwrap_or(""))),
                            )
                        })
                }),
        )
        // --- Search and freeform entry ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Search and freeform entry"),
                    theme,
                ))
                .child({
                    let search_value = get_value("select-searchable-value");
                    let mut search = SelectSpec::new(framework_options.clone())
                        .with_placeholder("Search frameworks...")
                        .with_open(get_open("select-searchable-open"));
                    search.searchable = true;
                    if let Some(ref v) = search_value {
                        search = search.with_value(v.as_str());
                    }
                    let freeform_value = get_value("select-freeform-value");
                    let mut freeform = SelectSpec::new(framework_options.clone())
                        .with_placeholder("Type or select...")
                        .with_open(get_open("select-freeform-open"));
                    freeform.searchable = true;
                    freeform.freeform = true;
                    if let Some(ref v) = freeform_value {
                        freeform = freeform.with_value(v.as_str());
                    }
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .max_w(px(320.0))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(6.0))
                                .child(node_select("select-searchable", search, state))
                                .when(search_value.is_some(), |d| {
                                    d.child(
                                        div()
                                            .text_sm()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child(format!(
                                                "Selected: {}",
                                                search_value.as_deref().unwrap_or("")
                                            )),
                                    )
                                }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(6.0))
                                .child(node_select("select-freeform", freeform, state))
                                .when(freeform_value.is_some(), |d| {
                                    d.child(
                                        div()
                                            .text_sm()
                                            .text_color(color_to_hsla(text_secondary))
                                            .child(format!(
                                                "Value: {}",
                                                freeform_value.as_deref().unwrap_or("")
                                            )),
                                    )
                                }),
                        )
                }),
        )
        // --- Rich and grouped options ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Rich and grouped options"),
                    theme,
                ))
                .child({
                    let mut rich = SelectSpec::new(rich_options.clone())
                        .with_placeholder("Choose a country")
                        .with_open(get_open("select-rich-open"));
                    rich.mode = SelectMode::Custom;
                    if let Some(ref v) = get_value("select-rich-value") {
                        rich = rich.with_value(v.as_str());
                    }
                    let mut grouped = SelectSpec::new(grouped_options.clone())
                        .with_placeholder("Choose a food")
                        .with_open(get_open("select-native-grouped-open"));
                    if let Some(ref v) = get_value("select-native-grouped-value") {
                        grouped = grouped.with_value(v.as_str());
                    }
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .max_w(px(320.0))
                        .child(node_select("select-rich", rich, state))
                        .child(node_select("select-native-grouped", grouped, state))
                }),
        )
        // --- Clearable selection ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Clearable selection"),
                    theme,
                ))
                .child({
                    let mut spec = SelectSpec::new(fruit_options.clone())
                        .with_placeholder("All fruits")
                        .with_clearable(true)
                        .with_open(get_open("select-clearable-open"));
                    spec.mode = SelectMode::Custom;
                    if let Some(ref v) = get_value("select-clearable-value") {
                        spec = spec.with_value(v.as_str());
                    }
                    div()
                        .max_w(px(320.0))
                        .child(node_select("select-clearable", spec, state))
                }),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child({
                    let mut spec = SelectSpec::new(fruit_options.clone())
                        .with_placeholder("Choose a fruit")
                        .with_value("banana");
                    spec.is_disabled = true;

                    div()
                        .max_w(px(320.0))
                        .child(node_select_static(spec, state))
                }),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "select",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                Select::from_spec(
                    SelectSpec::new(axis_options())
                        .with_placeholder("Select...")
                        .with_aria_label(format!("{} select", size_key(size)))
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                Select::from_spec(
                    SelectSpec::new(axis_options())
                        .with_placeholder("Select...")
                        .with_aria_label(format!("{} select", density_key(density)))
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
