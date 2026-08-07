use crate::node_compat::{Eyebrow, Field, FieldSet, TextInput};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, FieldSetSpec, FieldSpec, SpaceScale, TextInputSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(512.0))
        // --- Legend + description + grouped fields ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Legend + description"),
                    theme,
                ))
                .child(
                    FieldSet::from_spec(
                        FieldSetSpec::new()
                            .with_legend("Contact Information")
                            .with_description("We use this to reach you about your account."),
                        theme,
                    )
                    .with_child(
                        Field::from_spec(
                            FieldSpec::new("fs-name", "Full Name").with_required(true),
                            theme,
                        )
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("fs-name")
                                .with_placeholder("Jane Smith"),
                            theme,
                        )),
                    )
                    .with_child(
                        Field::from_spec(
                            FieldSpec::new("fs-email", "Email")
                                .with_required(true)
                                .with_description("We'll never share your email."),
                            theme,
                        )
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("fs-email")
                                .with_placeholder("jane@example.com"),
                            theme,
                        )),
                    )
                    .with_child(
                        Field::from_spec(
                            FieldSpec::new("fs-phone", "Phone").with_optional_label("Optional"),
                            theme,
                        )
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("fs-phone")
                                .with_placeholder("+1 (555) 000-0000"),
                            theme,
                        )),
                    ),
                ),
        )
        // --- Multi-column (two) with full-width span field ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Two columns + span"),
                    theme,
                ))
                .child(
                    FieldSet::from_spec(
                        FieldSetSpec::new().with_legend("Address").with_columns(2),
                        theme,
                    )
                    // `span="full"` is exercised on the spec; GPUI emits no
                    // CSS-grid placement (accepted layout delta, contract §12).
                    .with_child(
                        Field::from_spec(
                            FieldSpec::new("fs-street", "Street").with_span("full"),
                            theme,
                        )
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("fs-street")
                                .with_placeholder("123 Main St"),
                            theme,
                        )),
                    )
                    .with_child(
                        Field::from_spec(FieldSpec::new("fs-city", "City"), theme).with_control(
                            TextInput::from_spec(
                                TextInputSpec::new()
                                    .with_id("fs-city")
                                    .with_placeholder("Springfield"),
                                theme,
                            ),
                        ),
                    )
                    .with_child(
                        Field::from_spec(FieldSpec::new("fs-zip", "ZIP Code"), theme).with_control(
                            TextInput::from_spec(
                                TextInputSpec::new()
                                    .with_id("fs-zip")
                                    .with_placeholder("90210"),
                                theme,
                            ),
                        ),
                    ),
                ),
        )
        // --- Without legend ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Without legend"),
                    theme,
                ))
                .child(
                    FieldSet::from_spec(
                        FieldSetSpec::new().with_columns(2).with_gap(SpaceScale::Sm),
                        theme,
                    )
                    .with_child(
                        Field::from_spec(FieldSpec::new("fs-first", "First Name"), theme)
                            .with_control(TextInput::from_spec(
                                TextInputSpec::new()
                                    .with_id("fs-first")
                                    .with_placeholder("Jane"),
                                theme,
                            )),
                    )
                    .with_child(
                        Field::from_spec(FieldSpec::new("fs-last", "Last Name"), theme)
                            .with_control(TextInput::from_spec(
                                TextInputSpec::new()
                                    .with_id("fs-last")
                                    .with_placeholder("Smith"),
                                theme,
                            )),
                    ),
                ),
        )
        // --- Gap: none ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Gap: none"),
                    theme,
                ))
                .child(
                    FieldSet::from_spec(
                        FieldSetSpec::new()
                            .with_legend("Tightly stacked")
                            .with_gap(SpaceScale::None),
                        theme,
                    )
                    .with_child(
                        Field::from_spec(FieldSpec::new("fs-line1", "Line 1"), theme).with_control(
                            TextInput::from_spec(
                                TextInputSpec::new()
                                    .with_id("fs-line1")
                                    .with_placeholder("Apartment, suite, etc."),
                                theme,
                            ),
                        ),
                    )
                    .with_child(
                        Field::from_spec(FieldSpec::new("fs-line2", "Line 2"), theme).with_control(
                            TextInput::from_spec(
                                TextInputSpec::new()
                                    .with_id("fs-line2")
                                    .with_placeholder("Building, floor, etc."),
                                theme,
                            ),
                        ),
                    ),
                ),
        )
        // --- Multiple groups in a form ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Multiple groups in a form"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .child(
                            FieldSet::from_spec(
                                FieldSetSpec::new().with_legend("Personal").with_columns(2),
                                theme,
                            )
                            .with_child(
                                Field::from_spec(
                                    FieldSpec::new("fs2-first", "First Name").with_required(true),
                                    theme,
                                )
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_id("fs2-first"),
                                        theme,
                                    ),
                                ),
                            )
                            .with_child(
                                Field::from_spec(
                                    FieldSpec::new("fs2-last", "Last Name").with_required(true),
                                    theme,
                                )
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new().with_id("fs2-last"),
                                        theme,
                                    ),
                                ),
                            ),
                        )
                        .child(
                            FieldSet::from_spec(
                                FieldSetSpec::new().with_legend("Preferences"),
                                theme,
                            )
                            .with_child(
                                Field::from_spec(FieldSpec::new("fs2-lang", "Language"), theme)
                                    .with_control(TextInput::from_spec(
                                        TextInputSpec::new()
                                            .with_id("fs2-lang")
                                            .with_placeholder("Select language"),
                                        theme,
                                    )),
                            )
                            .with_child(
                                Field::from_spec(
                                    FieldSpec::new("fs2-tz", "Time Zone")
                                        .with_description("Used for scheduling and notifications."),
                                    theme,
                                )
                                .with_control(
                                    TextInput::from_spec(
                                        TextInputSpec::new()
                                            .with_id("fs2-tz")
                                            .with_placeholder("Select time zone"),
                                        theme,
                                    ),
                                ),
                            ),
                        ),
                ),
        )
}
