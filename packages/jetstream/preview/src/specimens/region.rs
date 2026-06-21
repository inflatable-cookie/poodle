//! Region specimen — presentational placeholder blocks with dashed borders.
//!
//! Mirrors the GPUI/Svelte specimen states: Default, Custom colors, Layout
//! composition. Region renders only its label (contract §3 forbids children).

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::region::js_region;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::RegionSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // --- Default ---
        .child(group("Default", secondary,
            js_region(&RegionSpec::new().with_label("Content area"), theme)
        ))
        // --- Custom colors ---
        .child(group("Custom colors", secondary,
            div().flex_col().gap(4.0)
                .child(js_region(&RegionSpec::new().with_label("Header").with_color("#5b9bd5").with_min_height(48.0), theme))
                .child(js_region(&RegionSpec::new().with_label("Sidebar").with_color("#70ad47").with_min_height(96.0), theme))
                .child(js_region(&RegionSpec::new().with_label("Main content").with_color("#ed7d31").with_min_height(128.0), theme))
                .child(js_region(&RegionSpec::new().with_label("Footer").with_color("#a855f7").with_min_height(48.0), theme))
        ))
        // --- Layout composition ---
        .child(group("Layout composition", secondary,
            div().flex_row().gap(8.0)
                .child(js_region(&RegionSpec::new().with_label("Nav").with_color("#5b9bd5").with_min_height(224.0), theme))
                .child(div().flex_col().gap(8.0)
                    .child(js_region(&RegionSpec::new().with_label("Toolbar").with_color("#70ad47").with_min_height(40.0), theme))
                    .child(js_region(&RegionSpec::new().with_label("Content").with_color("#ed7d31").with_min_height(160.0), theme))
                )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
