//! Tabs specimen — all four variants, icon+count tabs, disabled tabs.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::tabs::js_tabs;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{TabDefinition, TabVariant, TabsSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Basic tabs — used for most variant groups.
    let items = vec![
        TabDefinition::new("overview", "Overview"),
        TabDefinition::new("details", "Details"),
        TabDefinition::new("settings", "Settings"),
    ];

    // Tabs with icons.
    let items_icons = vec![
        TabDefinition::new("overview", "Overview").with_icon("layout"),
        TabDefinition::new("activity", "Activity").with_icon("activity"),
        TabDefinition::new("settings", "Settings").with_icon("settings"),
    ];

    // Tabs with count badges.
    let items_counts = vec![
        TabDefinition::new("inbox", "Inbox").with_count(12),
        TabDefinition::new("sent", "Sent").with_count(3),
        TabDefinition::new("drafts", "Drafts").with_count(0),
    ];

    // Tabs with a disabled entry.
    let items_disabled = vec![
        TabDefinition::new("active", "Active"),
        TabDefinition::new("review", "Under Review").with_disabled(true),
        TabDefinition::new("archived", "Archived"),
    ];

    div().flex_col().gap(32.0)
        // ── Underline (default) ──────────────────────────────────────────
        .child(group("Underline — first selected", secondary,
            js_tabs(&TabsSpec::new(items.clone()).with_value("overview"), theme)
        ))
        .child(group("Underline — second selected", secondary,
            js_tabs(&TabsSpec::new(items.clone()).with_value("details"), theme)
        ))
        .child(group("Underline — no bottom border", secondary,
            js_tabs(
                &TabsSpec::new(items.clone())
                    .with_value("overview")
                    .with_bordered(false),
                theme,
            )
        ))

        // ── Card ─────────────────────────────────────────────────────────
        .child(group("Card — first selected", secondary,
            js_tabs(
                &TabsSpec::new(items.clone())
                    .with_variant(TabVariant::Card)
                    .with_value("overview"),
                theme,
            )
        ))
        .child(group("Card — second selected", secondary,
            js_tabs(
                &TabsSpec::new(items.clone())
                    .with_variant(TabVariant::Card)
                    .with_value("details"),
                theme,
            )
        ))

        // ── Pill ─────────────────────────────────────────────────────────
        .child(group("Pill — first selected", secondary,
            js_tabs(
                &TabsSpec::new(items.clone())
                    .with_variant(TabVariant::Pill)
                    .with_value("overview"),
                theme,
            )
        ))
        .child(group("Pill — second selected", secondary,
            js_tabs(
                &TabsSpec::new(items.clone())
                    .with_variant(TabVariant::Pill)
                    .with_value("settings"),
                theme,
            )
        ))

        // ── Block ─────────────────────────────────────────────────────────
        .child(group("Block — first selected", secondary,
            js_tabs(
                &TabsSpec::new(items.clone())
                    .with_variant(TabVariant::Block)
                    .with_value("overview"),
                theme,
            )
        ))
        .child(group("Block — second selected", secondary,
            js_tabs(
                &TabsSpec::new(items.clone())
                    .with_variant(TabVariant::Block)
                    .with_value("details"),
                theme,
            )
        ))

        // ── Decorations ──────────────────────────────────────────────────
        .child(group("With icons (Underline)", secondary,
            js_tabs(
                &TabsSpec::new(items_icons.clone()).with_value("overview"),
                theme,
            )
        ))
        .child(group("With icons (Pill)", secondary,
            js_tabs(
                &TabsSpec::new(items_icons)
                    .with_variant(TabVariant::Pill)
                    .with_value("activity"),
                theme,
            )
        ))
        .child(group("With count badges", secondary,
            js_tabs(
                &TabsSpec::new(items_counts).with_value("inbox"),
                theme,
            )
        ))

        // ── Disabled tab ─────────────────────────────────────────────────
        .child(group("With disabled tab (Underline)", secondary,
            js_tabs(&TabsSpec::new(items_disabled.clone()).with_value("active"), theme)
        ))
        .child(group("With disabled tab (Pill)", secondary,
            js_tabs(
                &TabsSpec::new(items_disabled)
                    .with_variant(TabVariant::Pill)
                    .with_value("active"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
