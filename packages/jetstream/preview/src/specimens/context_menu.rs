//! ContextMenu specimen — context menus with different item sets.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::context_menu::js_context_menu;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::{ContextMenuSpec, MenuEntry};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let items = vec![
        MenuEntry::new("cut", "Cut"),
        MenuEntry::new("copy", "Copy"),
        MenuEntry::new("paste", "Paste"),
    ];

    let extended_items = vec![
        MenuEntry::new("undo", "Undo"),
        MenuEntry::new("redo", "Redo"),
        MenuEntry::new("cut", "Cut"),
        MenuEntry::new("copy", "Copy"),
        MenuEntry::new("paste", "Paste"),
        MenuEntry::new("delete", "Delete"),
    ];

    div().flex_col().gap(24.0)
        // Default
        .child(group("Default", secondary,
            js_context_menu(&ContextMenuSpec::new(items), theme)
        ))
        // Extended items
        .child(group("Extended items", secondary,
            js_context_menu(&ContextMenuSpec::new(extended_items), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
