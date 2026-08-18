use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, Field, TextInput};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, EyebrowSpec, FieldSpec, TextInputSpec, ValidationState,
};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div().flex().flex_col().gap(px(24.0)).max_w(px(384.0)) // 24rem
        // --- Label + control (plain) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Label + control"), theme))
                .child(
                    Field::from_spec(FieldSpec::new("field-plain", "Workspace name"), theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-plain")
                                .with_placeholder("Acme Inc.")
                                .with_aria_label("Workspace name"),
                            theme,
                        )),
                ),
        )
        // --- Default with description (info icon) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default with description"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-name", "Display name")
                            .with_description("This is how your name appears to other users."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-name")
                                .with_placeholder("Enter your name")
                                .with_aria_label("Display name"),
                            theme,
                        )
                    )
                )
        )
        // --- Required ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Required"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-email", "Email address")
                            .with_required(true),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-email")
                                .with_placeholder("you@example.com")
                                .with_aria_label("Email address"),
                            theme,
                        )
                    )
                )
        )
        // --- Optional ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Optional"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-phone", "Phone number")
                            .with_optional_label("optional"),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-phone")
                                .with_placeholder("+1 (555) 000-0000")
                                .with_aria_label("Phone number"),
                            theme,
                        )
                    )
                )
        )
        // --- Optional with description (info icon + optional marker) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Optional with description"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-fax", "Fax number")
                            .with_optional_label("optional")
                            .with_description("Include country code."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-fax")
                                .with_placeholder("+1 (555) 000-0000")
                                .with_aria_label("Fax number"),
                            theme,
                        )
                    )
                )
        )
        // --- With error (invalid) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With error"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-user", "Username")
                            .with_error("This username is already taken.")
                            .with_validation_state(ValidationState::Invalid),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-user")
                                .with_value("admin")
                                .with_validation_state(ValidationState::Invalid)
                                .with_aria_label("Username"),
                            theme,
                        )
                    )
                )
        )
        // --- Pending ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Pending"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-handle", "Handle")
                            .with_pending_message("Checking availability\u{2026}")
                            .with_validation_state(ValidationState::Pending),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-handle")
                                .with_value("newuser42")
                                .with_validation_state(ValidationState::Pending)
                                .with_aria_label("Handle"),
                            theme,
                        )
                    )
                )
        )
        // --- Valid ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Valid"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-pass", "Password")
                            .with_validation_state(ValidationState::Valid)
                            .with_description("Must be at least 8 characters."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-pass")
                                .with_value("\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}")
                                .with_validation_state(ValidationState::Valid)
                                .with_aria_label("Password"),
                            theme,
                        )
                    )
                )
        )
        // --- With info icon (description + required) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With info icon"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("field-api", "API Key")
                            .with_description("Your personal API key for authentication.")
                            .with_required(true),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("field-api")
                                .with_placeholder("sk_live_\u{2026}")
                                .with_aria_label("API Key"),
                            theme,
                        )
                    )
                )
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "field",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| size_row(theme, size).into_any_element())
            .with_densities(|density, theme: &GpuiThemeProvider| {
                density_row(theme, density).into_any_element()
            }),
    )
}

fn size_row(theme: &poodle_gpui::GpuiThemeProvider, size: ControlSize) -> Div {
    let id = format!("field-size-{}", size_key(size));
    let label = size_key(size).to_uppercase();
    div().flex().flex_col().gap(px(4.0)).child(
        Field::from_spec(
            FieldSpec::new(id.clone(), "Display name")
                .with_description("This is how your name appears to other users.")
                .with_size(size),
            theme,
        )
        .with_control(TextInput::from_spec(
            TextInputSpec::new()
                .with_id(id)
                .with_placeholder(label)
                .with_size(size)
                .with_aria_label("Display name"),
            theme,
        )),
    )
}

fn density_row(theme: &poodle_gpui::GpuiThemeProvider, density: ControlDensity) -> Div {
    let id = format!("field-density-{}", density_key(density));
    let label = density_key(density).to_uppercase();
    div().flex().flex_col().gap(px(4.0)).child(
        Field::from_spec(
            FieldSpec::new(id.clone(), "Display name")
                .with_description("This is how your name appears to other users.")
                .with_density(density),
            theme,
        )
        .with_control(TextInput::from_spec(
            TextInputSpec::new()
                .with_id(id)
                .with_placeholder(label)
                .with_density(density)
                .with_aria_label("Display name"),
            theme,
        )),
    )
}
