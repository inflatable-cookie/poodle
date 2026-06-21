//! EditableLabel specimen — display + editing modes, variants, sizes, densities.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::editable_label::js_editable_label;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ControlDensity, ControlSize, EditableLabelActivation, EditableLabelSpec, EditableLabelVariant,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Display mode: value + pencil edit-icon
        .child(group(
            "Display mode (value + edit icon)",
            secondary,
            row(js_editable_label(
                &EditableLabelSpec::new()
                    .with_value("My project title")
                    .with_show_edit_icon(true),
                theme,
            )),
        ))
        // Editing mode: composed input shown (seeded with value)
        .child(group(
            "Editing mode (input shown)",
            secondary,
            row(js_editable_label(
                &EditableLabelSpec::new()
                    .with_value("My project title")
                    .with_editing(true),
                theme,
            )),
        ))
        // Empty state: emptyText placeholder
        .child(group(
            "Empty state",
            secondary,
            row(js_editable_label(
                &EditableLabelSpec::new()
                    .with_activation_mode(EditableLabelActivation::EnterOrSpace)
                    .with_empty_text("Add a description…")
                    .with_placeholder("Add a description…"),
                theme,
            )),
        ))
        // Flush variant (display): no padding/border, inline text + icon
        .child(group(
            "Flush variant (display)",
            secondary,
            row(js_editable_label(
                &EditableLabelSpec::new()
                    .with_value("Inline heading")
                    .with_variant(EditableLabelVariant::Flush)
                    .with_show_edit_icon(true),
                theme,
            )),
        ))
        // Flush variant (editing): bottom accent border only
        .child(group(
            "Flush variant (editing)",
            secondary,
            row(js_editable_label(
                &EditableLabelSpec::new()
                    .with_value("Inline heading")
                    .with_variant(EditableLabelVariant::Flush)
                    .with_editing(true),
                theme,
            )),
        ))
        // With max length: maxLength + placeholder (editing)
        .child(group(
            "With max length (20)",
            secondary,
            row(js_editable_label(
                &EditableLabelSpec::new()
                    .with_value("Short text")
                    .with_max_length(20)
                    .with_placeholder("Enter text…")
                    .with_editing(true),
                theme,
            )),
        ))
        // Disabled: reduced opacity, no edit affordance
        .child(group(
            "Disabled",
            secondary,
            row(js_editable_label(
                &EditableLabelSpec::new()
                    .with_value("Read-only value")
                    .with_disabled(true),
                theme,
            )),
        ))
        // Sizes sweep (xs → xl)
        .child(group("Sizes", secondary, sizes_row(theme)))
        // Densities sweep (compact / default / comfortable)
        .child(group("Densities", secondary, densities_row(theme)))
}

const SIZES: &[(ControlSize, &str)] = &[
    (ControlSize::Xs, "xs"),
    (ControlSize::Sm, "sm"),
    (ControlSize::Md, "md"),
    (ControlSize::Lg, "lg"),
    (ControlSize::Xl, "xl"),
];

const DENSITIES: &[(ControlDensity, &str)] = &[
    (ControlDensity::Compact, "compact"),
    (ControlDensity::Default, "default"),
    (ControlDensity::Comfortable, "comfortable"),
];

fn sizes_row(theme: &JetstreamThemeProvider) -> JsEl {
    let mut col = div().flex_col().gap(12.0);
    for (size, label_text) in SIZES {
        col = col.child(
            div().flex_col().gap(8.0).w(300.0).child(
                js_editable_label(
                    &EditableLabelSpec::new()
                        .with_value(label_text.to_uppercase())
                        .with_size(*size),
                    theme,
                ),
            ),
        );
    }
    col
}

fn densities_row(theme: &JetstreamThemeProvider) -> JsEl {
    let mut col = div().flex_col().gap(12.0);
    for (density, _label_text) in DENSITIES {
        col = col.child(
            div().flex_col().gap(8.0).w(300.0).child(
                js_editable_label(
                    &EditableLabelSpec::new()
                        .with_value("Edit me")
                        .with_density(*density),
                    theme,
                ),
            ),
        );
    }
    col
}

fn row(content: JsEl) -> JsEl {
    div().w(300.0).child(content)
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
