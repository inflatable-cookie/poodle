//! DetailSection specimen — labeled content section within a detail view.
//!
//! Covers the contract state surface: title + description + body, trailing
//! header actions, description-only header (no title), empty/no-separator,
//! two-column body, and the three density variants. Body rows are real
//! `js_detail_item` components and the header action is a real `js_button` —
//! no hand-coded chrome.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::detail_item::js_detail_item;
use poodle_jetstream_components::detail_section::js_detail_section;
use poodle_jetstream_components::presentation::rem_to_px;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, DetailItemLayout, DetailItemSpec,
    DetailSectionSpec,
};

/// A stacked detail row sized so two land per row inside a `columns > 1`
/// flex-wrap body (mirrors the DetailSectionGroup column pattern).
fn col_item(label_text: &str, value: &str, theme: &JetstreamThemeProvider) -> JsEl {
    div().flex_1().min_w(rem_to_px(10.0)).child(js_detail_item(
        &DetailItemSpec::new(label_text)
            .with_value(value)
            .with_layout(DetailItemLayout::Stacked),
        theme,
    ))
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        // --- With title, description and rows ---
        .child(group("With title and rows", secondary,
            js_detail_section(
                &DetailSectionSpec::new()
                    .with_title("Project details")
                    .with_description("Core metadata for this project."),
                theme,
                vec![
                    js_detail_item(&DetailItemSpec::new("Name").with_value("Poodle Design System"), theme),
                    js_detail_item(&DetailItemSpec::new("Owner").with_value("Clay + Aura"), theme),
                    js_detail_item(&DetailItemSpec::new("Created").with_value("March 2025"), theme),
                    js_detail_item(&DetailItemSpec::new("Status").with_value("Active"), theme),
                ],
                None,
            )
        ))
        // --- With trailing header actions ---
        .child(group("With actions", secondary,
            js_detail_section(
                &DetailSectionSpec::new().with_title("Billing"),
                theme,
                vec![
                    js_detail_item(&DetailItemSpec::new("Plan").with_value("Pro"), theme),
                    js_detail_item(&DetailItemSpec::new("Billing cycle").with_value("Monthly"), theme),
                    js_detail_item(&DetailItemSpec::new("Next invoice").with_value("April 1, 2026"), theme),
                ],
                Some(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(ControlSize::Sm)
                        .with_label("Edit"),
                    theme,
                )),
            )
        ))
        // --- DetailItem with description ---
        .child(group("DetailItem with description", secondary,
            js_detail_section(
                &DetailSectionSpec::new().with_title("Configuration"),
                theme,
                vec![
                    js_detail_item(
                        &DetailItemSpec::new("API endpoint")
                            .with_value("https://api.example.com/v2")
                            .with_description("The base URL for all API requests.")
                            .with_truncate_value(true),
                        theme,
                    ),
                    js_detail_item(
                        &DetailItemSpec::new("Rate limit")
                            .with_value("1,000 req/min")
                            .with_description("Maximum requests per minute."),
                        theme,
                    ),
                ],
                None,
            )
        ))
        // --- Two-column body ---
        .child(group("Two-column details", secondary,
            js_detail_section(
                &DetailSectionSpec::new()
                    .with_title("Runtime summary")
                    .with_description("Compact layout for denser metadata surfaces.")
                    .with_columns(2),
                theme,
                vec![
                    col_item("Route", "local-brokered", theme),
                    col_item("Posture", "aura-local-brokered", theme),
                    col_item("Authority", "local", theme),
                    col_item("Displays", "2", theme),
                ],
                None,
            )
        ))
        // --- Description only (no title) ---
        .child(group("Description only (no title)", secondary,
            js_detail_section(
                &DetailSectionSpec::new()
                    .with_description("A section header carried by description text alone."),
                theme,
                vec![
                    js_detail_item(&DetailItemSpec::new("Region").with_value("eu-west-1"), theme),
                    js_detail_item(&DetailItemSpec::new("Zone").with_value("eu-west-1a"), theme),
                ],
                None,
            )
        ))
        // --- Empty body, no separator ---
        .child(group("Empty body, no separator", secondary,
            js_detail_section(
                &DetailSectionSpec::new()
                    .with_title("Empty Section")
                    .with_separated(false),
                theme,
                vec![label("No rows yet.").text_color(text_primary)],
                None,
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

fn density_demo(name: &str, density: ControlDensity, theme: &JetstreamThemeProvider) -> JsEl {
    let muted = resolve_color(theme, "color.text.muted");
    div().flex_col().gap(4.0)
        .child(label(name).text_color(muted).text_size(11.0))
        .child(js_detail_section(
            &DetailSectionSpec::new()
                .with_title("Workspace access")
                .with_description("Shared settings and runtime defaults.")
                .with_columns(2)
                .with_density(density),
            theme,
            vec![
                col_item("Default role", "Editor", theme),
                col_item("Approvals", "Required", theme),
                col_item("Region", "eu-west-1", theme),
                col_item("Retention", "30 days", theme),
            ],
            None,
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
