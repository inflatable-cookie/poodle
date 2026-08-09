//! TabStrip specimen — standalone tablist primitive.
//!
//! Mirrors the GPUI specimen (`packages/gpui/preview/src/specimens/tab_strip.rs`)
//! against the contract (`docs/contracts/components/tab-strip.md`). Covers the
//! full static state surface: horizontal closable strip + host-owned panel,
//! disabled items (dimmed), vertical orientation (labels + close buttons stay
//! visible per §4), and the size + density matrices. Per contract §1 the add-tab
//! affordance, overflow chrome, and reorder handles are out of scope, so none are
//! drawn. Keyboard nav / selection commit are preview-event-loop bound.

use crate::compat::js_tab_strip;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlDensity, ControlSize, Orientation, TabStripItem, TabStripSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.default");
    let bg_surface = resolve_color(theme, "color.background.surface");
    let radius = resolve_radius(theme, "radius.surface");

    // ── 1. HORIZONTAL — closable file tabs + host-owned panel ───────────
    let horizontal_items = vec![
        TabStripItem::new("main.rs", "main.rs").with_closable(true),
        TabStripItem::new("lib.rs", "lib.rs").with_closable(true),
        TabStripItem::new("mod.rs", "mod.rs").with_closable(true),
        TabStripItem::new("Cargo.toml", "Cargo.toml"),
    ];
    let horizontal = js_tab_strip(
        &TabStripSpec::new(horizontal_items)
            .with_value("main.rs")
            .with_reorderable(true)
            .with_aria_label("File tabs"),
        theme,
    );
    let horizontal_panel = div()
        .rounded(radius)
        .border_1()
        .border_color(border)
        .flex_col()
        .child(horizontal)
        .child(
            div().p(16.0).bg(bg_surface).min_h(48.0).child(
                label("Host-owned panel: main.rs")
                    .text_size(13.0)
                    .text_color(secondary),
            ),
        );

    // ── 2. DISABLED ITEMS — dimmed, skipped by arrow nav ────────────────
    let disabled_items = vec![
        TabStripItem::new("active", "Active"),
        TabStripItem::new("disabled", "Disabled").with_disabled(true),
        TabStripItem::new("ready", "Ready"),
        TabStripItem::new("blocked", "Blocked").with_disabled(true),
    ];
    let disabled = js_tab_strip(
        &TabStripSpec::new(disabled_items)
            .with_value("active")
            .with_aria_label("Status tabs"),
        theme,
    );

    // ── 3. VERTICAL — labels + close buttons stay visible ───────────────
    let vertical_items = vec![
        TabStripItem::new("files", "Files").with_closable(true),
        TabStripItem::new("search", "Search"),
        TabStripItem::new("git", "Git").with_closable(true),
        TabStripItem::new("extensions", "Extensions"),
    ];
    let vertical = js_tab_strip(
        &TabStripSpec::new(vertical_items)
            .with_value("files")
            .with_orientation(Orientation::Vertical)
            .with_aria_label("Activity bar"),
        theme,
    );
    let vertical_panel = div()
        .flex_row()
        .rounded(radius)
        .border_1()
        .border_color(border)
        .min_h(160.0)
        .child(vertical)
        .child(
            div()
                .grow()
                .p(16.0)
                .bg(bg_surface)
                .child(label("Active: files").text_size(13.0).text_color(secondary)),
        );

    // ── SIZE MATRIX (xs → xl) ───────────────────────────────────────────
    let matrix_items = || {
        vec![
            TabStripItem::new("overview", "Overview"),
            TabStripItem::new("activity", "Activity").with_closable(true),
            TabStripItem::new("settings", "Settings"),
        ]
    };
    let sizes = [
        ("xs", ControlSize::Xs),
        ("sm", ControlSize::Sm),
        ("md", ControlSize::Md),
        ("lg", ControlSize::Lg),
        ("xl", ControlSize::Xl),
    ];
    let mut size_col = div().flex_col().gap(12.0);
    for (name, size) in sizes {
        size_col = size_col.child(
            div()
                .flex_col()
                .gap(4.0)
                .child(label(name).text_color(secondary).text_size(11.0))
                .child(js_tab_strip(
                    &TabStripSpec::new(matrix_items())
                        .with_value("overview")
                        .with_size(size)
                        .with_aria_label("Tab strip"),
                    theme,
                )),
        );
    }

    // ── DENSITY MATRIX (compact / default / comfortable) ────────────────
    let densities = [
        ("compact", ControlDensity::Compact),
        ("default", ControlDensity::Default),
        ("comfortable", ControlDensity::Comfortable),
    ];
    let mut density_col = div().flex_col().gap(12.0);
    for (name, density) in densities {
        density_col = density_col.child(
            div()
                .flex_col()
                .gap(4.0)
                .child(label(name).text_color(secondary).text_size(11.0))
                .child(js_tab_strip(
                    &TabStripSpec::new(matrix_items())
                        .with_value("overview")
                        .with_density(density)
                        .with_aria_label("Tab strip"),
                    theme,
                )),
        );
    }

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Horizontal — closable file tabs + host panel",
            secondary,
            horizontal_panel,
        ))
        .child(group(
            "Disabled items (dimmed, skipped by arrow keys)",
            secondary,
            disabled,
        ))
        .child(group(
            "Vertical — labels and close buttons stay visible",
            secondary,
            vertical_panel,
        ))
        .child(group("Sizes (xs → xl)", secondary, size_col))
        .child(group(
            "Densities (compact / default / comfortable)",
            secondary,
            density_col,
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
