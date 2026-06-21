//! NavCard specimen — navigation cards with icon, title, description, badge,
//! trailing arrow, density variants, and link/disabled states.
//!
//! `js_nav_card` renders the leading icon slot and trailing arrow internally
//! (icon glyph stands in for the host `icon()` snippet), so each specimen card
//! varies the spec rather than passing separate elements.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::nav_card::js_nav_card;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, NavCardSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // ── Default cards: icon + title + description, badge, disabled ──
        .child(group(
            "Navigation cards",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("Getting Started")
                        .with_description("Learn the basics of the component library."),
                    theme,
                ))
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("Components")
                        .with_description("Browse all available components.")
                        .with_badge("New"),
                    theme,
                ))
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("Tokens")
                        .with_description("Design tokens and theming system."),
                    theme,
                ))
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("API Reference")
                        .with_description("Complete component API documentation.")
                        .with_disabled(true),
                    theme,
                )),
        ))
        // ── Numeric badge / count ───────────────────────────────────────
        .child(group(
            "Numeric badge",
            secondary,
            js_nav_card(
                &NavCardSpec::new()
                    .with_title("Composites")
                    .with_description("Higher-order components built from primitives.")
                    .with_badge("12"),
                theme,
            ),
        ))
        // ── As link (href) ──────────────────────────────────────────────
        .child(group(
            "Single card (as link)",
            secondary,
            js_nav_card(
                &NavCardSpec::new()
                    .with_title("View Documentation")
                    .with_description("Open the full documentation site.")
                    .with_href("#"),
                theme,
            ),
        ))
        // ── Density variants: compact / default / comfortable ───────────
        .child(group(
            "Density variants (compact / default / comfortable)",
            secondary,
            div()
                .flex_col()
                .gap(8.0)
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("Components")
                        .with_description("Browse all available components.")
                        .with_badge("New")
                        .with_density(ControlDensity::Compact),
                    theme,
                ))
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("Components")
                        .with_description("Browse all available components.")
                        .with_badge("New")
                        .with_density(ControlDensity::Default),
                    theme,
                ))
                .child(js_nav_card(
                    &NavCardSpec::new()
                        .with_title("Components")
                        .with_description("Browse all available components.")
                        .with_badge("New")
                        .with_density(ControlDensity::Comfortable),
                    theme,
                )),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
