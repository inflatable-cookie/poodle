//! Tabs specimen — all four variants, icon+count tabs, disabled tabs.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::tabs::js_tabs;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ControlDensity, ControlSize, Orientation, TabDefinition, TabVariant, TabsSpec,
};

use crate::app_state::AppState;

pub fn render(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
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

    // Closable card tabs (file-tab style with close x).
    let items_closable = vec![
        TabDefinition::new("index.ts", "index.ts"),
        TabDefinition::new("App.svelte", "App.svelte").with_closable(true),
        TabDefinition::new("utils.ts", "utils.ts").with_closable(true),
        TabDefinition::new("types.ts", "types.ts").with_closable(true),
    ];

    // Icon-only tabs for vertical orientation (label hidden in vertical mode).
    let items_vertical = vec![
        TabDefinition::new("files", "Explorer").with_icon("folder"),
        TabDefinition::new("search", "Search").with_icon("search"),
        TabDefinition::new("git", "Source Control").with_icon("git-branch"),
        TabDefinition::new("debug", "Debug").with_icon("terminal"),
    ];

    // Count-bearing tabs reused across full-width / size / density groups.
    let items_detail = vec![
        TabDefinition::new("details", "Details"),
        TabDefinition::new("usage", "Usage").with_count(12),
        TabDefinition::new("versions", "Versions").with_count(3),
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

        // ── Closable card tabs ───────────────────────────────────────────
        .child(group("Card — closable (file tabs with close x)", secondary,
            js_tabs(
                &TabsSpec::new(items_closable)
                    .with_variant(TabVariant::Card)
                    .with_value("App.svelte"),
                theme,
            )
        ))

        // ── Drag reorder states (drag-source + drop-target) ──────────────
        // Contract §4: drag-source = opacity 0.4; drop-target = inset
        // accent-base ring. This group is LIVE — it renders from AppState and
        // responds to a real pointer drag (see the MouseButton/MouseMoved
        // handlers in main.rs). Drag a tab to reorder; the dragged tab dims to
        // 0.4 and the tab under the cursor shows the accent ring.
        .child(group("Reorder — drag a tab to reorder (live)", secondary,
            js_tabs(&state.tabs_spec(), theme)
        ))
        // The remaining reorder group is a static snapshot of the states for
        // offscreen review (spec-set drag/drop values).
        .child(group("Reorder — dragging 'details', over 'settings' (Card)", secondary,
            js_tabs(
                &TabsSpec::new(items.clone())
                    .with_variant(TabVariant::Card)
                    .with_value("overview")
                    .with_reorderable(true)
                    .with_drag_value(Some("details".into()))
                    .with_drop_target_value(Some("settings".into())),
                theme,
            )
        ))

        // ── Full-width ───────────────────────────────────────────────────
        .child(group("Full-width (tabs flex to fill the row)", secondary,
            div().w(360.0).child(js_tabs(
                &TabsSpec::new(items_detail.clone())
                    .with_variant(TabVariant::Card)
                    .with_full_width(true)
                    .with_value("details"),
                theme,
            ))
        ))

        // ── Vertical orientation (icon-only) ─────────────────────────────
        .child(group("Vertical orientation (icon-only)", secondary,
            js_tabs(
                &TabsSpec::new(items_vertical.clone())
                    .with_variant(TabVariant::Underline)
                    .with_orientation(Orientation::Vertical)
                    .with_value("files"),
                theme,
            )
        ))
        .child(group("Vertical orientation (Block, icon-only)", secondary,
            js_tabs(
                &TabsSpec::new(items_vertical)
                    .with_variant(TabVariant::Block)
                    .with_orientation(Orientation::Vertical)
                    .with_value("files"),
                theme,
            )
        ))

        // ── Sizes (xs → xl) ──────────────────────────────────────────────
        .child(group("Sizes (xs → xl)", secondary,
            div().flex_col().gap(12.0)
                .child(js_tabs(&size_spec(&items_detail, ControlSize::Xs), theme))
                .child(js_tabs(&size_spec(&items_detail, ControlSize::Sm), theme))
                .child(js_tabs(&size_spec(&items_detail, ControlSize::Md), theme))
                .child(js_tabs(&size_spec(&items_detail, ControlSize::Lg), theme))
                .child(js_tabs(&size_spec(&items_detail, ControlSize::Xl), theme))
        ))

        // ── Densities (compact / default / comfortable) ──────────────────
        .child(group("Densities (compact / default / comfortable)", secondary,
            div().flex_col().gap(12.0)
                .child(js_tabs(&density_spec(&items_detail, ControlDensity::Compact), theme))
                .child(js_tabs(&density_spec(&items_detail, ControlDensity::Default), theme))
                .child(js_tabs(&density_spec(&items_detail, ControlDensity::Comfortable), theme))
        ))
}

fn size_spec(items: &[TabDefinition], size: ControlSize) -> TabsSpec {
    TabsSpec::new(items.to_vec())
        .with_variant(TabVariant::Card)
        .with_size(size)
        .with_value("details")
}

fn density_spec(items: &[TabDefinition], density: ControlDensity) -> TabsSpec {
    TabsSpec::new(items.to_vec())
        .with_variant(TabVariant::Card)
        .with_density(density)
        .with_value("details")
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
