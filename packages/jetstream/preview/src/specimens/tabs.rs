//! Tabs specimen — tab bars with different selected states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::tabs::js_tabs;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{TabDefinition, TabsSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    let items = vec![
        TabDefinition::new("overview", "Overview"),
        TabDefinition::new("details", "Details"),
        TabDefinition::new("settings", "Settings"),
    ];

    div().flex_col().gap(24.0)
        // Default (first tab selected)
        .child(group("Default", secondary,
            js_tabs(&TabsSpec::new(items.clone()).with_value("overview"), theme)
        ))
        // Second tab selected
        .child(group("Second selected", secondary,
            js_tabs(&TabsSpec::new(items.clone()).with_value("details"), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
