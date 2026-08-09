//! FieldSet specimen — legend + description, grouped real Field children,
//! multi-column, span, gap-none.

use crate::compat::js_field;
use crate::compat::js_field_set;
use crate::compat::js_text_input;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{FieldSetSpec, FieldSpec, SpaceScale, TextInputSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Legend + description + grouped real Field children
        .child(group(
            "Legend + description",
            secondary,
            div().w(400.0).child(js_field_set(
                &FieldSetSpec::new()
                    .with_legend("Personal Information")
                    .with_description("We use this to reach you about your account."),
                theme,
                vec![
                    field(theme, "fs-first", "First name", None, Some("Jane")),
                    field(theme, "fs-last", "Last name", None, Some("Smith")),
                    field(
                        theme,
                        "fs-email",
                        "Email",
                        Some("We'll never share your email."),
                        Some("jane@example.com"),
                    ),
                ],
            )),
        ))
        // Without legend
        .child(group(
            "Without legend",
            secondary,
            div().w(400.0).child(js_field_set(
                &FieldSetSpec::new(),
                theme,
                vec![
                    field(
                        theme,
                        "fs-street",
                        "Street address",
                        None,
                        Some("123 Main St"),
                    ),
                    field(theme, "fs-city", "City", None, Some("Springfield")),
                ],
            )),
        ))
        // Multi-column (two)
        .child(group(
            "Two columns",
            secondary,
            div().w(400.0).child(js_field_set(
                &FieldSetSpec::new().with_legend("Address").with_columns(2),
                theme,
                vec![
                    field(theme, "fs2-city", "City", None, Some("Springfield")),
                    field(theme, "fs2-state", "State", None, Some("IL")),
                    field(theme, "fs2-zip", "ZIP", None, Some("62704")),
                    field(theme, "fs2-country", "Country", None, Some("USA")),
                ],
            )),
        ))
        // Span — full-width field inside a two-column grid
        .child(group(
            "Span (full)",
            secondary,
            div().w(400.0).child(js_field_set(
                &FieldSetSpec::new().with_legend("Billing").with_columns(2),
                theme,
                vec![
                    // `span="full"` exercised on the spec; Jetstream emits no
                    // CSS-grid placement (accepted layout delta, contract §12).
                    js_field(
                        &FieldSpec::new("fs3-line", "Street").with_span("full"),
                        theme,
                        Some(js_text_input(
                            &TextInputSpec::new().with_placeholder("123 Main St"),
                            theme,
                        )),
                    ),
                    field(theme, "fs3-city", "City", None, Some("Springfield")),
                    field(theme, "fs3-zip", "ZIP", None, Some("62704")),
                ],
            )),
        ))
        // Gap: none
        .child(group(
            "Gap: none",
            secondary,
            div().w(400.0).child(js_field_set(
                &FieldSetSpec::new()
                    .with_legend("Tightly stacked")
                    .with_gap(SpaceScale::None),
                theme,
                vec![
                    field(
                        theme,
                        "fs4-line1",
                        "Line 1",
                        None,
                        Some("Apartment, suite, etc."),
                    ),
                    field(
                        theme,
                        "fs4-line2",
                        "Line 2",
                        None,
                        Some("Building, floor, etc."),
                    ),
                ],
            )),
        ))
}

/// A real `Field` (label + control) for use as a FieldSet child.
fn field(
    theme: &JetstreamThemeProvider,
    id: &str,
    label_text: &str,
    description: Option<&str>,
    value: Option<&str>,
) -> El {
    let mut spec = FieldSpec::new(id, label_text);
    if let Some(d) = description {
        spec = spec.with_description(d);
    }
    let mut input = TextInputSpec::new();
    if let Some(v) = value {
        input = input.with_value(v);
    }
    js_field(&spec, theme, Some(js_text_input(&input, theme)))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
