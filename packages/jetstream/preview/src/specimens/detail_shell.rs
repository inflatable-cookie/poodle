//! DetailShell specimen — detail page layout shell.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::detail_shell::js_detail_shell;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, DetailShellSpec, DetailState};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("With header and content", secondary,
            div().h(200.0).child(
                js_detail_shell(
                    &DetailShellSpec::new().with_title("Item Detail"),
                    theme,
                    Some(label("Header area").text_color(text_primary).text_size(body_font).text_weight(600).p(rem_to_px(0.75))),
                    Some(label("Content area").text_color(secondary).text_size(body_font).p(rem_to_px(0.75))),
                    None,
                )
            )
        ))
        .child(group("Loading state", secondary,
            div().h(160.0).child(
                js_detail_shell(
                    &DetailShellSpec::new().with_title("Loading").with_state(DetailState::Loading),
                    theme,
                    None,
                    None,
                    None,
                )
            )
        ))
        .child(group("Error state (custom title + message)", secondary,
            div().h(160.0).child(
                js_detail_shell(
                    &DetailShellSpec::new()
                        .with_title("Error")
                        .with_state(DetailState::Error)
                        .with_state_title("Failed to load")
                        .with_state_message("Something went wrong. Please try again."),
                    theme,
                    None,
                    None,
                    None,
                )
            )
        ))
        .child(group("Empty shell", secondary,
            div().h(120.0).child(
                js_detail_shell(&DetailShellSpec::new(), theme, None, None, None)
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
