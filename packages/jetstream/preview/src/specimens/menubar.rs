//! Menubar specimen — horizontal menu bars with open and disabled states.
//!
//! Mirrors the GPUI specimen (`gpui/preview/src/specimens/menubar.rs`) and
//! contract §13 + §4: File/Edit/View triggers; an open overlay (selected via
//! `with_value`) rendering items, separators and shortcut meta; checkbox + radio
//! rows in the View menu; a disabled item (Paste Special in Edit); a disabled
//! top-level trigger (Window); plus size and density ladders. The open overlay,
//! checkmarks and shortcut meta are all rendered by `js_menubar` from the spec.

use crate::compat::js_menubar;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ControlDensity, ControlSize, MenuEntry, MenuItemKind, MenubarEntry, MenubarSpec,
};

/// The full File/Edit/View/Window item set, exercising shortcuts, separators,
/// checkbox + radio rows, a disabled item and a disabled trigger (mirrors GPUI).
fn full_items() -> Vec<MenubarEntry> {
    vec![
        MenubarEntry::new(
            "file",
            "File",
            vec![
                MenuEntry::new("new", "New").with_shortcut_label("\u{2318}N"),
                MenuEntry::new("open", "Open\u{2026}").with_shortcut_label("\u{2318}O"),
                MenuEntry::new("save", "Save").with_shortcut_label("\u{2318}S"),
                MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
                MenuEntry::new("quit", "Quit").with_shortcut_label("\u{2318}Q"),
            ],
        ),
        MenubarEntry::new(
            "edit",
            "Edit",
            vec![
                MenuEntry::new("undo", "Undo").with_shortcut_label("\u{2318}Z"),
                MenuEntry::new("redo", "Redo").with_shortcut_label("\u{21E7}\u{2318}Z"),
                MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
                MenuEntry::new("cut", "Cut").with_shortcut_label("\u{2318}X"),
                MenuEntry::new("copy", "Copy").with_shortcut_label("\u{2318}C"),
                MenuEntry::new("paste", "Paste").with_shortcut_label("\u{2318}V"),
                // Disabled item (contract §4 disabled item state).
                MenuEntry::new("paste-special", "Paste Special\u{2026}").with_disabled(true),
            ],
        ),
        // View menu: checkbox rows + a radio group (menuitemcheckbox /
        // menuitemradio roles, contract §2 / §6).
        MenubarEntry::new(
            "view",
            "View",
            vec![
                MenuEntry::new("show-sidebar", "Show Sidebar")
                    .with_kind(MenuItemKind::Checkbox)
                    .with_checked(true)
                    .with_shortcut_label("\u{2318}0"),
                MenuEntry::new("show-statusbar", "Show Status Bar")
                    .with_kind(MenuItemKind::Checkbox)
                    .with_checked(false),
                MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
                MenuEntry::new("zoom-100", "Actual Size")
                    .with_kind(MenuItemKind::Radio)
                    .with_checked(true)
                    .with_shortcut_label("\u{2318}0"),
                MenuEntry::new("zoom-125", "Zoom 125%")
                    .with_kind(MenuItemKind::Radio)
                    .with_checked(false),
                MenuEntry::new("zoom-150", "Zoom 150%")
                    .with_kind(MenuItemKind::Radio)
                    .with_checked(false),
                MenuEntry::new("sep2", "").with_kind(MenuItemKind::Separator),
                MenuEntry::new("fullscreen", "Full screen")
                    .with_shortcut_label("\u{2303}\u{2318}F"),
            ],
        ),
        // Disabled top-level trigger (contract §4 disabled trigger state).
        MenubarEntry::new(
            "window",
            "Window",
            vec![MenuEntry::new("minimize", "Minimize")],
        )
        .with_disabled(true),
    ]
}

/// A small two-trigger item set used by the size / density ladders.
fn ladder_items() -> Vec<MenubarEntry> {
    vec![
        MenubarEntry::new(
            "file",
            "File",
            vec![
                MenuEntry::new("new", "New"),
                MenuEntry::new("open", "Open\u{2026}"),
            ],
        ),
        MenubarEntry::new(
            "edit",
            "Edit",
            vec![
                MenuEntry::new("undo", "Undo"),
                MenuEntry::new("redo", "Redo"),
            ],
        ),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Application menu bar — File open: triggers + open overlay with
        // shortcuts + separators (the File menu is the open one here).
        .child(group(
            "Application menu bar (File open)",
            secondary,
            js_menubar(
                &MenubarSpec::new(full_items())
                    .with_value("file")
                    .with_aria_label("Application menu"),
                theme,
            ),
        ))
        // Edit open — disabled "Paste Special" item visible in the overlay.
        .child(group(
            "Edit open (disabled item)",
            secondary,
            js_menubar(
                &MenubarSpec::new(full_items())
                    .with_value("edit")
                    .with_aria_label("Application menu"),
                theme,
            ),
        ))
        // View open — checkbox + radio rows render checkmarks for checked state;
        // the disabled Window trigger is also visible in the trigger strip.
        .child(group(
            "View open (checkbox + radio) + disabled trigger",
            secondary,
            js_menubar(
                &MenubarSpec::new(full_items())
                    .with_value("view")
                    .with_aria_label("Application menu"),
                theme,
            ),
        ))
        // Size ladder — trigger height + font scale xs..xl.
        .child(group(
            "Size ladder",
            secondary,
            div()
                .flex_col()
                .gap(16.0)
                .child(ladder(
                    theme,
                    "XS",
                    MenubarSpec::new(ladder_items()).with_size(ControlSize::Xs),
                ))
                .child(ladder(
                    theme,
                    "SM",
                    MenubarSpec::new(ladder_items()).with_size(ControlSize::Sm),
                ))
                .child(ladder(
                    theme,
                    "MD",
                    MenubarSpec::new(ladder_items()).with_size(ControlSize::Md),
                ))
                .child(ladder(
                    theme,
                    "LG",
                    MenubarSpec::new(ladder_items()).with_size(ControlSize::Lg),
                ))
                .child(ladder(
                    theme,
                    "XL",
                    MenubarSpec::new(ladder_items()).with_size(ControlSize::Xl),
                )),
        ))
        // Density ladder — trigger padding tightens/loosens, height unchanged.
        .child(group(
            "Density ladder",
            secondary,
            div()
                .flex_col()
                .gap(16.0)
                .child(ladder(
                    theme,
                    "COMPACT",
                    MenubarSpec::new(ladder_items()).with_density(ControlDensity::Compact),
                ))
                .child(ladder(
                    theme,
                    "DEFAULT",
                    MenubarSpec::new(ladder_items()).with_density(ControlDensity::Default),
                ))
                .child(ladder(
                    theme,
                    "COMFORTABLE",
                    MenubarSpec::new(ladder_items()).with_density(ControlDensity::Comfortable),
                )),
        ))
}

fn ladder(theme: &JetstreamThemeProvider, lbl: &str, spec: MenubarSpec) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    div()
        .flex_col()
        .gap(8.0)
        .child(
            label(lbl)
                .text_color(secondary)
                .text_size(11.0)
                .text_weight(700),
        )
        .child(js_menubar(&spec.with_aria_label("Menubar"), theme))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
