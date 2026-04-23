//! FormLayout specimen — responsive form grid with status messages.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::form_layout::js_form_layout;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, FormLayoutSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.default");

    // Fake field placeholder — a labeled input-like box
    let field = |lbl: &str| -> JsEl {
        div().flex_col().gap(rem_to_px(0.25))
            .child(label(lbl).text_color(secondary).text_size(11.0))
            .child(
                div()
                    .h(rem_to_px(2.0))
                    .border_1().border_color(border)
                    .rounded(rem_to_px(0.25))
                    .px(rem_to_px(0.5))
                    .items_center()
            )
    };

    let actions = div().flex_row().justify_end().gap(8.0)
        .child(button("Cancel").text_size(rem_to_px(size_font_rem(ControlSize::Md))).text_color(secondary))
        .child(button("Save").text_size(rem_to_px(size_font_rem(ControlSize::Md))).text_color(secondary));

    div().flex_col().gap(32.0)
        .child(group("Single-column layout", secondary,
            js_form_layout(
                &FormLayoutSpec::new().with_columns(1),
                theme,
                vec![
                    field("Project name"),
                    field("Description"),
                ],
                Some(actions.clone()),
            )
        ))
        .child(group("Two-column layout", secondary,
            js_form_layout(
                &FormLayoutSpec::new().with_columns(2),
                theme,
                vec![
                    field("First name"),
                    field("Last name"),
                    field("Email"),
                    field("Phone"),
                ],
                None,
            )
        ))
        .child(group("With description", secondary,
            js_form_layout(
                &FormLayoutSpec::new()
                    .with_description("Fill in the track metadata before publishing.")
                    .with_columns(1),
                theme,
                vec![field("Track title"), field("Artist")],
                None,
            )
        ))
        .child(group("With error message", secondary,
            js_form_layout(
                &FormLayoutSpec::new()
                    .with_error("Please fix the highlighted fields before continuing.")
                    .with_columns(1),
                theme,
                vec![field("Email"), field("Password")],
                None,
            )
        ))
        .child(group("With success message", secondary,
            js_form_layout(
                &FormLayoutSpec::new()
                    .with_success("Profile saved successfully.")
                    .with_columns(1),
                theme,
                vec![field("Display name")],
                None,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
