use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_specs::{ChoiceOption, EyebrowSpec, SelectMode, SelectSpec, ValidationState};
use poodle_gpui_components::{Select, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let fruit_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("apple", "Apple"),
        ChoiceOption::new("banana", "Banana"),
        ChoiceOption::new("cherry", "Cherry"),
        ChoiceOption::new("dragonfruit", "Dragonfruit"),
        ChoiceOption::new("elderberry", "Elderberry"),
    ];

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

    // Helper for building a Select with common toggle/change handlers
    let build_select = |id: &'static str,
                        spec: SelectSpec,
                        cx: &mut Context<PreviewRoot>|
     -> Select {
        let open_key = format!("{}-open", id);
        Select::from_spec(spec, theme)
            .with_id(id)
            .on_toggle(cx.listener(move |this, _open: &bool, _w, cx| {
                this.state.specimens.toggle(&open_key);
                cx.notify();
            }))
            .on_change(cx.listener(move |this, val: &str, _w, cx| {
                let open_key = format!("{}-open", id);
                let value_key = format!("{}-value", id);
                this.state.specimens.text.insert(value_key, val.to_string());
                this.state.specimens.toggles.insert(open_key, false);
                cx.notify();
            }))
    };

    div().flex().flex_col().gap(px(24.0))
        // --- Native (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Native (default)"), theme))
                .child({
                    let value = get_value("select-native-value");
                    let mut spec = SelectSpec::new(fruit_options.clone())
                        .with_placeholder("Choose a fruit")
                        .with_open(get_open("select-native-open"));
                    if let Some(ref v) = value {
                        spec = spec.with_value(v.as_str());
                    }
                    div().flex().flex_col().gap(px(6.0)).max_w(px(320.0))
                        .child(build_select("select-native", spec, cx))
                        .when(value.is_some(), |d| {
                            d.child(
                                div().text_sm().text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", value.as_deref().unwrap_or("")))
                            )
                        })
                })
        )
        // --- Custom dropdown (non-searchable) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom dropdown (non-searchable)"), theme))
                .child({
                    let value = get_value("select-custom-value");
                    let mut spec = SelectSpec::new(rich_options.clone())
                        .with_placeholder("Choose a country")
                        .with_open(get_open("select-custom-open"));
                    spec.mode = SelectMode::Custom;
                    if let Some(ref v) = value {
                        spec = spec.with_value(v.as_str());
                    }
                    div().flex().flex_col().gap(px(6.0)).max_w(px(320.0))
                        .child(build_select("select-custom", spec, cx))
                        .when(value.is_some(), |d| {
                            d.child(
                                div().text_sm().text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", value.as_deref().unwrap_or("")))
                            )
                        })
                })
        )
        // --- Searchable ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Searchable"), theme))
                .child({
                    let value = get_value("select-searchable-value");
                    let mut spec = SelectSpec::new(framework_options.clone())
                        .with_placeholder("Search frameworks...")
                        .with_open(get_open("select-searchable-open"));
                    spec.searchable = true;
                    if let Some(ref v) = value {
                        spec = spec.with_value(v.as_str());
                    }
                    div().flex().flex_col().gap(px(6.0)).max_w(px(320.0))
                        .child(build_select("select-searchable", spec, cx))
                        .when(value.is_some(), |d| {
                            d.child(
                                div().text_sm().text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", value.as_deref().unwrap_or("")))
                            )
                        })
                })
        )
        // --- Searchable with groups ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Searchable with groups"), theme))
                .child({
                    let value = get_value("select-searchable-grouped-value");
                    let mut spec = SelectSpec::new(grouped_options.clone())
                        .with_placeholder("Search food...")
                        .with_open(get_open("select-searchable-grouped-open"));
                    spec.searchable = true;
                    if let Some(ref v) = value {
                        spec = spec.with_value(v.as_str());
                    }
                    div().flex().flex_col().gap(px(6.0)).max_w(px(320.0))
                        .child(build_select("select-searchable-grouped", spec, cx))
                        .when(value.is_some(), |d| {
                            d.child(
                                div().text_sm().text_color(color_to_hsla(text_secondary))
                                    .child(format!("Selected: {}", value.as_deref().unwrap_or("")))
                            )
                        })
                })
        )
        // --- Freeform (autocomplete) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Freeform (autocomplete)"), theme))
                .child({
                    let value = get_value("select-freeform-value");
                    let mut spec = SelectSpec::new(framework_options.clone())
                        .with_placeholder("Type or select...")
                        .with_open(get_open("select-freeform-open"));
                    spec.searchable = true;
                    spec.freeform = true;
                    if let Some(ref v) = value {
                        spec = spec.with_value(v.as_str());
                    }
                    div().flex().flex_col().gap(px(6.0)).max_w(px(320.0))
                        .child(build_select("select-freeform", spec, cx))
                        .when(value.is_some(), |d| {
                            d.child(
                                div().text_sm().text_color(color_to_hsla(text_secondary))
                                    .child(format!("Value: {}", value.as_deref().unwrap_or("")))
                            )
                        })
                })
        )
        // --- Rich option rendering (custom slot) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Rich option rendering (custom slot)"), theme))
                .child({
                    let mut spec = SelectSpec::new(rich_options.clone())
                        .with_placeholder("Choose a country")
                        .with_open(get_open("select-rich-open"));
                    spec.mode = SelectMode::Custom;
                    if let Some(ref v) = get_value("select-rich-value") {
                        spec = spec.with_value(v.as_str());
                    }
                    div().max_w(px(320.0))
                        .child(build_select("select-rich", spec, cx))
                })
        )
        // --- Clearable (custom) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Clearable (custom)"), theme))
                .child({
                    let mut spec = SelectSpec::new(fruit_options.clone())
                        .with_placeholder("All fruits")
                        .with_clearable(true)
                        .with_open(get_open("select-clearable-open"));
                    spec.mode = SelectMode::Custom;
                    if let Some(ref v) = get_value("select-clearable-value") {
                        spec = spec.with_value(v.as_str());
                    }
                    div().max_w(px(320.0))
                        .child(build_select("select-clearable", spec, cx))
                })
        )
        // --- Native grouped ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Native grouped"), theme))
                .child({
                    let mut spec = SelectSpec::new(grouped_options.clone())
                        .with_placeholder("Choose a food")
                        .with_open(get_open("select-native-grouped-open"));
                    if let Some(ref v) = get_value("select-native-grouped-value") {
                        spec = spec.with_value(v.as_str());
                    }
                    div().max_w(px(320.0))
                        .child(build_select("select-native-grouped", spec, cx))
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = SelectSpec::new(fruit_options.clone())
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
        // --- Validation states ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Validation states"), theme))
                .child(
                    div().flex().flex_col().gap(px(12.0)).max_w(px(320.0))
                        .child(
                            Select::from_spec(
                                SelectSpec::new(fruit_options.clone())
                                    .with_placeholder("Pick one")
                                    .with_value("apple")
                                    .with_validation_state(ValidationState::Invalid),
                                theme,
                            )
                            .with_id("select-invalid")
                        )
                        .child(
                            Select::from_spec(
                                SelectSpec::new(fruit_options.clone())
                                    .with_placeholder("Pick one")
                                    .with_value("banana")
                                    .with_validation_state(ValidationState::Valid),
                                theme,
                            )
                            .with_id("select-valid")
                        )
                        .child(
                            Select::from_spec(
                                SelectSpec::new(fruit_options)
                                    .with_placeholder("Pick one")
                                    .with_value("cherry")
                                    .with_validation_state(ValidationState::Pending),
                                theme,
                            )
                            .with_id("select-pending")
                        )
                )
        )
}
