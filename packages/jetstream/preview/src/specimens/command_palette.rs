//! CommandPalette specimen — command palettes with actions.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::command_palette::js_command_palette;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{CommandActionItem, CommandPaletteSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let actions = vec![
        CommandActionItem::new("open-file", "Open File"),
        CommandActionItem::new("save", "Save"),
        CommandActionItem::new("find", "Find in Files"),
        CommandActionItem::new("replace", "Find and Replace"),
        CommandActionItem::new("goto-line", "Go to Line"),
        CommandActionItem::new("toggle-terminal", "Toggle Terminal"),
    ];

    div().flex_col().gap(24.0)
        // Default
        .child(group("Default", secondary,
            js_command_palette(&CommandPaletteSpec::new(actions.clone()), theme)
        ))
        // With query
        .child(group("With query", secondary,
            js_command_palette(
                &CommandPaletteSpec::new(actions.clone()).with_query("find"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
