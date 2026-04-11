//! Menubar specimen — horizontal menu bars with dropdown entries.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::menubar::js_menubar;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{MenubarEntry, MenubarSpec, MenuEntry};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let items = vec![
        MenubarEntry::new("file", "File", vec![
            MenuEntry::new("new", "New"),
            MenuEntry::new("open", "Open"),
            MenuEntry::new("save", "Save"),
        ]),
        MenubarEntry::new("edit", "Edit", vec![
            MenuEntry::new("undo", "Undo"),
            MenuEntry::new("redo", "Redo"),
            MenuEntry::new("cut", "Cut"),
            MenuEntry::new("copy", "Copy"),
            MenuEntry::new("paste", "Paste"),
        ]),
        MenubarEntry::new("view", "View", vec![
            MenuEntry::new("zoom-in", "Zoom In"),
            MenuEntry::new("zoom-out", "Zoom Out"),
            MenuEntry::new("reset", "Reset Zoom"),
        ]),
    ];

    div().flex_col().gap(24.0)
        // Default
        .child(group("Default", secondary,
            js_menubar(&MenubarSpec::new(items), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
