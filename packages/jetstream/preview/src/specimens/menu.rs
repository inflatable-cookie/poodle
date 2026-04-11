//! Menu specimen — vertical menus with different item sets.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::menu::js_menu;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{MenuEntry, MenuSpec};

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

    div().flex_col().gap(24.0)
        // Basic menu
        .child(group("Basic", secondary,
            js_menu(&MenuSpec::new(basic_items), theme)
        ))
        // Extended menu
        .child(group("Extended items", secondary,
            js_menu(&MenuSpec::new(extended_items), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
