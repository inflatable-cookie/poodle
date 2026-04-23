//! NavigationMenu specimen — horizontal nav menus with active and disabled states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::navigation_menu::js_navigation_menu;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{NavigationMenuEntry, NavigationMenuSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let items = vec![
        NavigationMenuEntry::new("docs", "Docs"),
        NavigationMenuEntry::new("contracts", "Contracts"),
        NavigationMenuEntry::new("tokens", "Tokens"),
        NavigationMenuEntry::new("changelog", "Changelog"),
    ];

    let items_with_disabled = vec![
        NavigationMenuEntry::new("docs", "Docs"),
        NavigationMenuEntry::new("contracts", "Contracts"),
        NavigationMenuEntry::new("tokens", "Tokens"),
        NavigationMenuEntry::new("disabled", "Coming Soon").with_disabled(true),
    ];

    div().flex_col().gap(24.0)
        // Default (first item auto-selected)
        .child(group("Default (first active)", secondary,
            js_navigation_menu(&NavigationMenuSpec::new(items.clone()), theme)
        ))
        // Explicit active item
        .child(group("Active: Contracts", secondary,
            js_navigation_menu(
                &NavigationMenuSpec::new(items.clone()).with_default_value("contracts"),
                theme,
            )
        ))
        // With disabled entry
        .child(group("With disabled entry", secondary,
            js_navigation_menu(&NavigationMenuSpec::new(items_with_disabled), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
