//! DetailItem specimen — key-value pair rows.
//!
//! Covers the contract state surface: inline (default) + stacked layout,
//! simple vs surface presentation, empty/em-dash value, inline description,
//! a trailing action slot, a custom value slot (Pill), value truncation, and
//! the three density variants. Every element is a real component resolved from
//! tokens (`js_detail_item` / `js_detail_item_with_slots`, `js_button`,
//! `js_pill`) — no hand-coded chrome.

use crate::compat::js_button;
use crate::compat::js_pill;
use crate::compat::{js_detail_item, js_detail_item_with_slots};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, DetailItemLayout,
    DetailItemPresentation, DetailItemSpec, PillAppearance, PillSpec, PillTone,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // --- Inline layout (default) ---
        .child(group("Inline layout (default)", secondary,
            div().flex_col().gap(4.0)
                .child(js_detail_item(
                    &DetailItemSpec::new("Name").with_value("Jetstream Preview"),
                    theme,
                ))
                .child(js_detail_item(
                    &DetailItemSpec::new("Version").with_value("0.1.0"),
                    theme,
                ))
                .child(js_detail_item(
                    &DetailItemSpec::new("License").with_value("MIT"),
                    theme,
                ))
        ))
        // --- With description (inline, rendered under label) ---
        .child(group("With description", secondary,
            js_detail_item(
                &DetailItemSpec::new("API endpoint")
                    .with_value("https://api.example.com/v2")
                    .with_description("Base URL for all API requests.")
                    .with_truncate_value(true),
                theme,
            )
        ))
        // --- With action slot ---
        .child(group("With action slot", secondary,
            js_detail_item_with_slots(
                &DetailItemSpec::new("Email").with_value("clay@example.com"),
                theme,
                None,
                Some(js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(ControlSize::Sm)
                        .with_label("Change"),
                    theme,
                )),
            )
        ))
        // --- With value slot (custom content via Pill) ---
        .child(group("With value slot", secondary,
            js_detail_item_with_slots(
                &DetailItemSpec::new("Status"),
                theme,
                Some(js_pill(
                    &PillSpec::new()
                        .with_label("Active")
                        .with_tone(PillTone::Success)
                        .with_appearance(PillAppearance::Badge),
                    theme,
                )),
                None,
            )
        ))
        // --- Empty value (em-dash placeholder) ---
        .child(group("Empty value (em-dash)", secondary,
            js_detail_item(
                &DetailItemSpec::new("Owner"),
                theme,
            )
        ))
        // --- Stacked layout (truncated) ---
        .child(group("Stacked layout", secondary,
            js_detail_item(
                &DetailItemSpec::new("Arrangement")
                    .with_value("2CF8B3D0-F592-4D87-8F9F-74D6B42E0E7D:main:external:0:0:3440:1440:1000|37D8832A...")
                    .with_truncate_value(true)
                    .with_layout(DetailItemLayout::Stacked),
                theme,
            )
        ))
        // --- Simple vs surface presentation ---
        .child(group("Simple vs surface presentation", secondary,
            div().flex_col().gap(8.0)
                .child(js_detail_item(
                    &DetailItemSpec::new("Simple")
                        .with_value("Plain row, no chrome")
                        .with_presentation(DetailItemPresentation::Simple),
                    theme,
                ))
                .child(js_detail_item(
                    &DetailItemSpec::new("Surface")
                        .with_value("Elevated card row")
                        .with_presentation(DetailItemPresentation::Surface),
                    theme,
                ))
        ))
        // --- Density variants ---
        .child(group("Density variants", secondary,
            div().flex_col().gap(12.0)
                .child(density_demo("Compact", ControlDensity::Compact, theme))
                .child(density_demo("Default", ControlDensity::Default, theme))
                .child(density_demo("Comfortable", ControlDensity::Comfortable, theme))
        ))
}

fn density_demo(name: &str, density: ControlDensity, theme: &JetstreamThemeProvider) -> El {
    let muted = resolve_color(theme, "color.text.muted");
    div()
        .flex_col()
        .gap(4.0)
        .child(label(name).text_color(muted).text_size(11.0))
        .child(js_detail_item_with_slots(
            &DetailItemSpec::new("Storage")
                .with_value("84.2 GB")
                .with_description("Current usage for the active workspace.")
                .with_presentation(DetailItemPresentation::Surface)
                .with_density(density),
            theme,
            None,
            Some(js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_size(ControlSize::Sm)
                    .with_label("Manage"),
                theme,
            )),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
