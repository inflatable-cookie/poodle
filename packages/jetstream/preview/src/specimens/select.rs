//! Select specimen — dropdowns with placeholder, selected value, and disabled.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::select::js_select;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ChoiceOption, ControlSize, SelectSpec, SelectVariant, ValidationState};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let options = vec![
        ChoiceOption::new("apple", "Apple"),
        ChoiceOption::new("banana", "Banana"),
        ChoiceOption::new("cherry", "Cherry"),
    ];

    // Grouped options (Fruits / Vegetables / Grains), Spinach disabled.
    let grouped_options = vec![
        ChoiceOption::new("apple", "Apple").with_group("Fruits"),
        ChoiceOption::new("banana", "Banana").with_group("Fruits"),
        ChoiceOption::new("cherry", "Cherry").with_group("Fruits"),
        ChoiceOption::new("carrot", "Carrot").with_group("Vegetables"),
        ChoiceOption::new("broccoli", "Broccoli").with_group("Vegetables"),
        ChoiceOption::new("spinach", "Spinach")
            .with_group("Vegetables")
            .with_disabled(true),
        ChoiceOption::new("rice", "Rice").with_group("Grains"),
        ChoiceOption::new("wheat", "Wheat").with_group("Grains"),
    ];

    div().flex_col().gap(24.0)
        // Default (with placeholder)
        .child(group("Default", secondary,
            div().w(240.0)
                .child(js_select(&SelectSpec::new(options.clone()).with_aria_label("Fruit 1").with_placeholder("Choose fruit..."), theme))
        ))
        // With value
        .child(group("With value", secondary,
            div().w(240.0)
                .child(js_select(&SelectSpec::new(options.clone()).with_aria_label("Fruit 2").with_value("banana"), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = SelectSpec::new(options.clone()).with_aria_label("Fruit 3").with_value("apple");
            spec.is_disabled = true;
            div().w(240.0).child(js_select(&spec, theme))
        }))
        // Open state
        .child(group("Open state", secondary,
            div().w(240.0)
                .child(js_select(&SelectSpec::new(options.clone()).with_aria_label("Fruit 4").with_placeholder("Choose fruit...").with_open(true), theme))
        ))
        // Searchable
        .child(group("Searchable", secondary,
            div().w(240.0)
                .child(js_select(&SelectSpec::new(options.clone()).with_aria_label("Fruit 5").with_placeholder("Search...").with_searchable(true), theme))
        ))
        // Invalid validation
        .child(group("Invalid validation", secondary,
            div().w(240.0)
                .child(js_select(&SelectSpec::new(options.clone()).with_aria_label("Fruit 6").with_validation_state(ValidationState::Invalid).with_placeholder("Choose..."), theme))
        ))
        // Ghost variant
        .child(group("Ghost variant", secondary,
            div().w(240.0)
                .child(js_select(&SelectSpec::new(options.clone()).with_aria_label("Fruit 7").with_variant(SelectVariant::Ghost).with_value("cherry"), theme))
        ))
        // Grouped options (open — shows section headers + disabled option)
        .child(group("Grouped (open: headers + disabled option)", secondary,
            div().w(240.0)
                .child(js_select(
                    &SelectSpec::new(grouped_options.clone()).with_aria_label("Fruit 8")
                        .with_placeholder("Choose an item")
                        .with_open(true),
                    theme,
                ))
        ))
        // Clearable (with value — shows clear x in trigger)
        .child(group("Clearable (value selected → clear x)", secondary,
            div().w(240.0)
                .child(js_select(
                    &SelectSpec::new(options.clone()).with_aria_label("Fruit 9")
                        .with_placeholder("All fruits")
                        .with_clearable(true)
                        .with_value("banana"),
                    theme,
                ))
        ))
        // Selected + open (shows checkmark on the selected option)
        .child(group("Selected + open (checkmark indicator)", secondary,
            div().w(240.0)
                .child(js_select(
                    &SelectSpec::new(options.clone()).with_aria_label("Fruit 10")
                        .with_value("banana")
                        .with_open(true),
                    theme,
                ))
        ))
        // Sizes (xs → xl)
        .child(group("Sizes (xs → xl)", secondary,
            div().flex_col().gap(12.0)
                .child(div().w(240.0).child(js_select(&size_spec(&options, ControlSize::Xs), theme)))
                .child(div().w(240.0).child(js_select(&size_spec(&options, ControlSize::Sm), theme)))
                .child(div().w(240.0).child(js_select(&size_spec(&options, ControlSize::Md), theme)))
                .child(div().w(240.0).child(js_select(&size_spec(&options, ControlSize::Lg), theme)))
                .child(div().w(240.0).child(js_select(&size_spec(&options, ControlSize::Xl), theme)))
        ))
}

fn size_spec(options: &[ChoiceOption], size: ControlSize) -> SelectSpec {
    SelectSpec::new(options.to_vec()).with_aria_label("Fruit 11")
        .with_placeholder("Select...")
        .with_size(size)
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
