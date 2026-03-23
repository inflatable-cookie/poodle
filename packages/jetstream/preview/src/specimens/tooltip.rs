//! Tooltip specimen — tooltip panels with different text content.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::tooltip::js_tooltip;
use flint_jetstream_components::theme_ext::*;
use flint_primitives::TooltipSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Default tooltip
        .child(group("Default", secondary,
            js_tooltip(&TooltipSpec::new().with_content("Helpful tip"), theme)
        ))
        // Longer content
        .child(group("Longer content", secondary,
            js_tooltip(&TooltipSpec::new().with_content("This tooltip has a longer description to show wrapping behavior"), theme)
        ))
        // Short label
        .child(group("Short label", secondary,
            js_tooltip(&TooltipSpec::new().with_content("Save"), theme)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
