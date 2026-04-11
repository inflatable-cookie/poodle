//! FilterToolbar specimen — layout container for filter controls.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::filter_toolbar::js_filter_toolbar;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::FilterToolbarSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    // Stand-in "control" chips so the specimen has something visible in
    // each grid slot without depending on TextInput/Select primitives.
    let chip = |label_text: &str| {
        label(label_text)
            .text_color(text_primary)
            .text_size(13.0)
            .p(8.0)
    };

    div().flex_col().gap(24.0)
        .child(group("Expanded with summary", secondary,
            js_filter_toolbar(
                &FilterToolbarSpec::new()
                    .with_summary_text("Showing 24 of 156 items")
                    .with_collapsible(false),
                theme,
                vec![chip("Search"), chip("Status"), chip("Type"), chip("Owner")],
                None,
                None,
            )
        ))
        .child(group("Collapsed with actions", secondary,
            js_filter_toolbar(
                &FilterToolbarSpec::new()
                    .with_summary_text("3 filters active")
                    .with_collapsible(true)
                    .with_collapsed(true),
                theme,
                vec![chip("Search"), chip("Status")],
                Some(label("Refresh").text_color(secondary).text_size(11.0)),
                None,
            )
        ))
        .child(group("With secondary slot", secondary,
            js_filter_toolbar(
                &FilterToolbarSpec::new()
                    .with_aria_label("Project filters")
                    .with_collapsible(false)
                    .with_columns(3),
                theme,
                vec![chip("Filter"), chip("Status"), chip("Type")],
                None,
                Some(label("Reset all").text_color(secondary).text_size(13.0)),
            )
        ))
        .child(group("Empty", secondary,
            js_filter_toolbar(&FilterToolbarSpec::new(), theme, vec![], None, None)
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
