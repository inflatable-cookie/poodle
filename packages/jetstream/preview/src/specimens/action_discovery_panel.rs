//! ActionDiscoveryPanel specimen — keyboard shortcut discovery panel.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::action_discovery_panel::js_action_discovery_panel;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::{ActionDiscoveryPanelSpec, ActionDiscoverySection};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let sections = vec![
        ActionDiscoverySection::new("nav", "Navigation", vec![]),
        ActionDiscoverySection::new("edit", "Editing", vec![]),
    ];

    div().flex_col().gap(24.0)
        .child(group("Default", secondary,
            js_action_discovery_panel(&ActionDiscoveryPanelSpec::new(sections), theme)
        ))
        .child(group("Empty", secondary,
            js_action_discovery_panel(
                &ActionDiscoveryPanelSpec::new(vec![])
                    .with_empty_message("No shortcuts available"),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
