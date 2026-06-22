//! ContextMenu specimen — context menu item surface.
//!
//! Mirrors the GPUI specimen (`packages/gpui/preview/src/specimens/context_menu.rs`)
//! against the contract (`docs/contracts/components/context-menu.md`). The menu
//! surface renders open (open/close + anchor positioning are preview-event-loop
//! bound). Covers the full item surface: actions with shortcut labels, separators,
//! a disabled item, a danger (destructive) item, and checkbox/radio items with a
//! leading check indicator — plus the size and density matrices. All real
//! `js_context_menu` instances.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::context_menu::js_context_menu;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ContextMenuSpec, ControlDensity, ControlSize, MenuEntry, MenuItemKind};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    // ── 1. Full editing menu — items, shortcuts, separators, disabled ───
    let editing_items = vec![
        MenuEntry::new("cut", "Cut").with_shortcut_label("\u{2318}X"),
        MenuEntry::new("copy", "Copy").with_shortcut_label("\u{2318}C"),
        MenuEntry::new("paste", "Paste").with_shortcut_label("\u{2318}V"),
        MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("select-all", "Select all").with_shortcut_label("\u{2318}A"),
        MenuEntry::new("sep2", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("delete", "Delete").with_disabled(true),
    ];

    // ── 2. Danger + checkbox/radio surface ──────────────────────────────
    let mixed_items = vec![
        MenuEntry::new("show-grid", "Show grid")
            .with_kind(MenuItemKind::Checkbox)
            .with_checked(true),
        MenuEntry::new("snap", "Snap to pixels").with_kind(MenuItemKind::Checkbox),
        MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("layout-grid", "Grid layout")
            .with_kind(MenuItemKind::Radio)
            .with_checked(true),
        MenuEntry::new("layout-list", "List layout").with_kind(MenuItemKind::Radio),
        MenuEntry::new("sep2", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("delete", "Delete")
            .with_destructive(true)
            .with_shortcut_label("\u{232B}"),
    ];

    // ── SIZE MATRIX ─────────────────────────────────────────────────────
    let matrix_items = || {
        vec![
            MenuEntry::new("cut", "Cut").with_shortcut_label("\u{2318}X"),
            MenuEntry::new("copy", "Copy").with_shortcut_label("\u{2318}C"),
            MenuEntry::new("paste", "Paste").with_shortcut_label("\u{2318}V"),
        ]
    };
    let sizes = [
        ("xs", ControlSize::Xs),
        ("sm", ControlSize::Sm),
        ("md", ControlSize::Md),
        ("lg", ControlSize::Lg),
        ("xl", ControlSize::Xl),
    ];
    let mut size_row = div().flex_row().flex_wrap().gap(16.0);
    for (name, size) in sizes {
        size_row = size_row.child(
            div().flex_col().gap(4.0)
                .child(label(name).text_color(secondary).text_size(11.0))
                .child(js_context_menu(
                    &ContextMenuSpec::new(matrix_items()).with_size(size),
                    theme,
                )),
        );
    }

    // ── DENSITY MATRIX ──────────────────────────────────────────────────
    let densities = [
        ("compact", ControlDensity::Compact),
        ("default", ControlDensity::Default),
        ("comfortable", ControlDensity::Comfortable),
    ];
    let mut density_row = div().flex_row().flex_wrap().gap(16.0);
    for (name, density) in densities {
        density_row = density_row.child(
            div().flex_col().gap(4.0)
                .child(label(name).text_color(secondary).text_size(11.0))
                .child(js_context_menu(
                    &ContextMenuSpec::new(matrix_items()).with_density(density),
                    theme,
                )),
        );
    }

    div().flex_col().gap(24.0)
        .child(group("Editing menu — shortcuts, separators, disabled", secondary,
            js_context_menu(&ContextMenuSpec::new(editing_items), theme)
        ))
        .child(group("Danger + checkbox/radio items", secondary,
            js_context_menu(&ContextMenuSpec::new(mixed_items), theme)
        ))
        .child(group("Sizes (xs → xl)", secondary, size_row))
        .child(group("Densities (compact / default / comfortable)", secondary, density_row))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
