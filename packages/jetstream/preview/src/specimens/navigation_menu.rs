//! NavigationMenu specimen — horizontal nav menu with entries.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::navigation_menu::js_navigation_menu;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{NavigationMenuEntry, NavigationMenuSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let items = vec![
        NavigationMenuEntry::new("docs", "Docs"),
        NavigationMenuEntry::new("contracts", "Contracts"),
        NavigationMenuEntry::new("tokens", "Tokens"),
    ];

    div().flex_col().gap(24.0)
        .child(group("Default", secondary,
            js_navigation_menu(&NavigationMenuSpec::new(items), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
