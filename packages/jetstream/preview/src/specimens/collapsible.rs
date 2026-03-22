//! Collapsible specimen — collapsible sections in open and closed states.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::collapsible::js_collapsible;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::CollapsibleSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");
    let text_muted = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Open
        .child(group("Open", secondary,
            div().w(300.0).child(
                js_collapsible(
                    &CollapsibleSpec::new().with_title("Section Title").with_default_open(true),
                    theme,
                    Some(label("This is the collapsible content that is visible when open.").text_color(text_muted).text_size(13.0)),
                )
            )
        ))
        // Closed
        .child(group("Closed", secondary,
            div().w(300.0).child(
                js_collapsible(
                    &CollapsibleSpec::new().with_title("Collapsed Section").with_default_open(false),
                    theme,
                    Some(label("Hidden content").text_color(text_muted).text_size(13.0)),
                )
            )
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(300.0).child(
                js_collapsible(
                    &CollapsibleSpec::new().with_title("Disabled Section").with_default_open(false).with_disabled(true),
                    theme,
                    Some(label("Cannot toggle").text_color(text_muted).text_size(13.0)),
                )
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
