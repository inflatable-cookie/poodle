//! DetailSectionGroup specimen — responsive grouping of detail sections.
//!
//! Covers the contract state surface: grid layout (default auto-fit), stack
//! layout (single column), column cap (`max_columns`), and the three density
//! variants. Sections are real `js_detail_section` composites holding real
//! `js_detail_item` rows — no hand-coded chrome.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::detail_item::js_detail_item;
use poodle_jetstream_components::detail_section::js_detail_section;
use poodle_jetstream_components::detail_section_group::js_detail_section_group;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ControlDensity, DetailItemLayout, DetailItemSpec, DetailSectionGroupLayout,
    DetailSectionGroupSpec, DetailSectionSpec,
};

/// A two-row detail section (stacked rows) used as a group child.
fn section(title: &str, a: (&str, &str), b: (&str, &str), theme: &JetstreamThemeProvider) -> JsEl {
    js_detail_section(
        &DetailSectionSpec::new().with_title(title).with_columns(2),
        theme,
        vec![
            js_detail_item(
                &DetailItemSpec::new(a.0)
                    .with_value(a.1)
                    .with_layout(DetailItemLayout::Stacked),
                theme,
            ),
            js_detail_item(
                &DetailItemSpec::new(b.0)
                    .with_value(b.1)
                    .with_layout(DetailItemLayout::Stacked),
                theme,
            ),
        ],
        None,
    )
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // --- Grid layout (default auto-fit) ---
        .child(group("Grid layout", secondary,
            js_detail_section_group(
                &DetailSectionGroupSpec::new().with_aria_label("Project metadata"),
                theme,
                vec![
                    section("General", ("Owner", "Platform"), ("Status", "Active"), theme),
                    section("Runtime", ("Region", "eu-west-1"), ("Tier", "Production"), theme),
                    section("Policy", ("Retention", "90 days"), ("Review", "Required"), theme),
                ],
            )
        ))
        // --- Stack layout (single column) ---
        .child(group("Stack layout", secondary,
            js_detail_section_group(
                &DetailSectionGroupSpec::new()
                    .with_layout(DetailSectionGroupLayout::Stack)
                    .with_item_min_column_width("10rem"),
                theme,
                vec![
                    section("Access", ("Role", "Editor"), ("Scope", "Workspace"), theme),
                    section("Billing", ("Plan", "Team"), ("Renewal", "Monthly"), theme),
                ],
            )
        ))
        // --- Column cap (maxColumns = 2) ---
        .child(group("Column cap (maxColumns = 2)", secondary,
            js_detail_section_group(
                &DetailSectionGroupSpec::new()
                    .with_min_column_width("10rem")
                    .with_max_columns(2),
                theme,
                vec![
                    cap_section("One", theme),
                    cap_section("Two", theme),
                    cap_section("Three", theme),
                    cap_section("Four", theme),
                ],
            )
        ))
        // --- Density variants ---
        .child(group("Density variants", secondary,
            div().flex_col().gap(16.0)
                .child(density_demo("Compact", ControlDensity::Compact, theme))
                .child(density_demo("Default", ControlDensity::Default, theme))
                .child(density_demo("Comfortable", ControlDensity::Comfortable, theme))
        ))
}

fn cap_section(title: &str, theme: &JetstreamThemeProvider) -> JsEl {
    js_detail_section(
        &DetailSectionSpec::new().with_title(title),
        theme,
        vec![js_detail_item(
            &DetailItemSpec::new("Value").with_value(title),
            theme,
        )],
        None,
    )
}

fn density_demo(name: &str, density: ControlDensity, theme: &JetstreamThemeProvider) -> JsEl {
    let muted = resolve_color(theme, "color.text.muted");
    div().flex_col().gap(4.0)
        .child(label(name).text_color(muted).text_size(11.0))
        .child(js_detail_section_group(
            &DetailSectionGroupSpec::new()
                .with_density(density)
                .with_min_column_width("12rem"),
            theme,
            vec![
                section("Density", ("Mode", "Set"), ("Inherited", "Yes"), theme),
                section("Spacing", ("Root", "Gap"), ("Body", "Gap"), theme),
            ],
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
