//! Menu specimen — vertical menus with different item sets.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::menu::js_menu;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{MenuEntry, MenuItemKind, MenuSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let basic_items = vec![
        MenuEntry::new("new", "New File"),
        MenuEntry::new("open", "Open"),
        MenuEntry::new("save", "Save"),
    ];

    let extended_items = vec![
        MenuEntry::new("profile", "Profile"),
        MenuEntry::new("settings", "Settings"),
        MenuEntry::new("billing", "Billing"),
        MenuEntry::new("logout", "Log Out"),
    ];

    // Open state exercises every item kind, because the panel is the only place
    // `menuitem`, `menuitemcheckbox`, `menuitemradio` and `separator` exist —
    // a closed menu renders a trigger and nothing else, so none of those roles
    // were verifiable until this specimen existed.
    let open_items = vec![
        MenuEntry::new("cut", "Cut"),
        MenuEntry::new("copy", "Copy"),
        MenuEntry::new("sep-1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("wrap", "Word Wrap")
            .with_kind(MenuItemKind::Checkbox)
            .with_checked(true),
        MenuEntry::new("theme-dark", "Dark Theme")
            .with_kind(MenuItemKind::Radio)
            .with_checked(true),
        MenuEntry::new("theme-light", "Light Theme").with_kind(MenuItemKind::Radio),
    ];

    div().flex_col().gap(24.0)
        // Basic menu
        .child(group("Basic", secondary,
            js_menu(&MenuSpec::new(basic_items), theme)
        ))
        // Extended menu
        .child(group("Extended items", secondary,
            js_menu(&MenuSpec::new(extended_items), theme)
        ))
        // Open state — the panel and its items only exist here, so this is the
        // only specimen that can show, or verify, the menu's actual content.
        .child(group("Open state", secondary,
            js_menu(&MenuSpec::new(open_items).with_open(true), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
