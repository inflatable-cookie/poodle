use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, TabStrip};
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, Orientation, TabStripItem, TabStripSpec};
use std::sync::Arc;

fn node_value_handler(state: &AppState, key: &'static str) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value: &str| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: value.to_string(),
        });
    })
}

/// TabStrip — the tablist-only primitive (contract: tab-strip.md).
///
/// Covers the full contract state surface: horizontal + vertical orientation,
/// selected/active indicator, disabled items (skipped by arrow nav), closable
/// items (per-item close button), reorderable flag, plus the size and density
/// matrices. The contract's §1 out-of-scope list excludes add-tab affordances,
/// overflow menus, and reorder handles, so those are intentionally absent.
pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border = theme.resolve_color("color.border.default");
    let bg_surface = theme.resolve_color("color.background.surface");

    // ── 1. HORIZONTAL — closable + reorderable file tabs ────────────
    let horizontal_items = vec![
        TabStripItem::new("main.rs", "main.rs").with_closable(true),
        TabStripItem::new("lib.rs", "lib.rs").with_closable(true),
        TabStripItem::new("mod.rs", "mod.rs").with_closable(true),
        TabStripItem::new("Cargo.toml", "Cargo.toml"),
    ];

    let horizontal_value = state
        .specimens
        .text
        .get("tab-strip-h-value")
        .map(|s| s.as_str())
        .unwrap_or("main.rs")
        .to_string();

    let last_closed = state
        .specimens
        .text
        .get("tab-strip-closed")
        .cloned()
        .unwrap_or_default();
    let last_reorder = state
        .specimens
        .text
        .get("tab-strip-reorder")
        .cloned()
        .unwrap_or_default();

    let horizontal_spec = TabStripSpec::new(horizontal_items)
        .with_value(&horizontal_value)
        .with_reorderable(true)
        .with_aria_label("File tabs");

    let horizontal_component = TabStrip::from_spec(horizontal_spec, theme)
        .with_id("tab-strip-horizontal")
        .on_change(node_value_handler(state, "tab-strip-h-value"))
        .on_close(node_value_handler(state, "tab-strip-closed"));

    // ── 2. DISABLED ITEM — skipped by arrow nav ─────────────────────
    let disabled_items = vec![
        TabStripItem::new("active", "Active"),
        TabStripItem::new("disabled", "Disabled").with_disabled(true),
        TabStripItem::new("ready", "Ready"),
        TabStripItem::new("blocked", "Blocked").with_disabled(true),
    ];
    let disabled_value = state
        .specimens
        .text
        .get("tab-strip-disabled-value")
        .map(|s| s.as_str())
        .unwrap_or("active")
        .to_string();
    let disabled_spec = TabStripSpec::new(disabled_items)
        .with_value(&disabled_value)
        .with_aria_label("Status tabs");
    let disabled_component = TabStrip::from_spec(disabled_spec, theme)
        .with_id("tab-strip-disabled")
        .on_change(node_value_handler(state, "tab-strip-disabled-value"));

    // ── 3. VERTICAL — labels + close buttons stay visible ───────────
    let vertical_items = vec![
        TabStripItem::new("files", "Files").with_closable(true),
        TabStripItem::new("search", "Search"),
        TabStripItem::new("git", "Git").with_closable(true),
        TabStripItem::new("extensions", "Extensions"),
    ];
    let vertical_value = state
        .specimens
        .text
        .get("tab-strip-vertical-value")
        .map(|s| s.as_str())
        .unwrap_or("files")
        .to_string();
    let vertical_spec = TabStripSpec::new(vertical_items)
        .with_value(&vertical_value)
        .with_orientation(Orientation::Vertical)
        .with_aria_label("Activity bar");
    let vertical_component = TabStrip::from_spec(vertical_spec, theme)
        .with_id("tab-strip-vertical")
        .on_change(node_value_handler(state, "tab-strip-vertical-value"))
        .on_close(node_value_handler(state, "tab-strip-closed"));

    // ── ASSEMBLE EXAMPLES ───────────────────────────────────────────
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // 1. Horizontal closable + reorderable
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Horizontal — closable, reorderable (Alt+Arrow)"),
                    theme,
                ))
                .child(
                    div()
                        .rounded(px(6.0))
                        .border_1()
                        .border_color(color_to_hsla(border))
                        .overflow_hidden()
                        .child(horizontal_component)
                        .child(
                            div()
                                .p(px(16.0))
                                .bg(color_to_hsla(bg_surface))
                                .min_h(px(48.0))
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("Host-owned panel: {horizontal_value}")),
                        ),
                )
                .when(!last_closed.is_empty(), |d| {
                    d.child(
                        div()
                            .text_sm()
                            .text_color(color_to_hsla(text_secondary))
                            .child(format!("Last closed: {last_closed}")),
                    )
                })
                .when(!last_reorder.is_empty(), |d| {
                    d.child(
                        div()
                            .text_sm()
                            .text_color(color_to_hsla(text_secondary))
                            .child(format!("Last reordered: {last_reorder}")),
                    )
                }),
        )
        // 2. Disabled items
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Disabled items (dimmed, skipped by arrow keys)"),
                    theme,
                ))
                .child(disabled_component),
        )
        // 3. Vertical
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Vertical — labels and close buttons stay visible"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .rounded(px(6.0))
                        .border_1()
                        .border_color(color_to_hsla(border))
                        .overflow_hidden()
                        .h(px(160.0))
                        .child(vertical_component)
                        .child(
                            div()
                                .flex_1()
                                .p(px(16.0))
                                .bg(color_to_hsla(bg_surface))
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("Active: {vertical_value}")),
                        ),
                ),
        )
        .into_any_element();

    // ── SIZE + DENSITY MATRICES (via specimen_layout) ───────────────
    let make_items = || {
        vec![
            TabStripItem::new("overview", "Overview"),
            TabStripItem::new("activity", "Activity").with_closable(true),
            TabStripItem::new("settings", "Settings"),
        ]
    };

    specimen_layout(
        state,
        cx,
        "tab-strip",
        examples,
        move |size, theme: &GpuiThemeProvider| {
            TabStrip::from_spec(
                TabStripSpec::new(make_items())
                    .with_value("overview")
                    .with_aria_label("Tab strip"),
                theme,
            )
            .with_id(format!("tab-strip-size-{size:?}"))
            .size(size)
            .into_any_element()
        },
        move |density, theme: &GpuiThemeProvider| {
            TabStrip::from_spec(
                TabStripSpec::new(make_items())
                    .with_value("overview")
                    .with_aria_label("Tab strip"),
                theme,
            )
            .with_id(format!("tab-strip-density-{density:?}"))
            .density(density)
            .into_any_element()
        },
    )
}
