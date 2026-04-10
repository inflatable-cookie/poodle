//! ShellStatusBar specimen — bottom status bar with slots.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::shell_status_bar::js_shell_status_bar;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::ShellStatusBarSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_color = resolve_color(theme, "color.text.secondary");

    let leading = vec![
        label("main").text_color(text_color).text_size(11.0),
        label("UTF-8").text_color(text_color).text_size(11.0),
    ];
    let trailing = vec![
        label("Ln 42, Col 18").text_color(text_color).text_size(11.0),
        label("Rust").text_color(text_color).text_size(11.0),
    ];

    div().flex_col().gap(24.0)
        .child(group("With summary", secondary,
            js_shell_status_bar(
                &ShellStatusBarSpec::new()
                    .with_summary("Ready")
                    .with_leading_item_count(2)
                    .with_trailing_item_count(2),
                theme,
                leading,
                trailing,
            )
        ))
        .child(group("Minimal", secondary,
            js_shell_status_bar(
                &ShellStatusBarSpec::new()
                    .with_summary("Connected"),
                theme,
                vec![],
                vec![],
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
