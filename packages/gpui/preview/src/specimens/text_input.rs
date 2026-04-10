use gpui::*;
use poodle_primitives::{FieldSpec, TextInputSpec, ValidationState, EyebrowSpec};
use poodle_gpui_components::{Field, TextInput, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    // Track typed values
    let name_value = state.specimens.text.get("text-input-name").cloned()
        .unwrap_or_default();
    let email_value = state.specimens.text.get("text-input-email").cloned()
        .unwrap_or_else(|| "invalid-email".to_string());
    let email_is_valid = email_value.contains('@');
    let validation_state = if email_is_valid {
        ValidationState::Valid
    } else {
        ValidationState::Invalid
    };
    let workspace_value = state.specimens.text.get("text-input-workspace").cloned()
        .unwrap_or_else(|| "acme-admin".to_string());

    let examples = div().flex().flex_col().gap(px(24.0)).max_w(px(384.0)) // 24rem = Svelte specimen max-width
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0)) // 0.5rem
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("name-field", "Name")
                            .with_description("Enter your full name."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("name-field")
                                .with_placeholder("Jane Doe")
                                .with_value(&name_value),
                            theme,
                        )
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("text-input-name".to_string(), val.to_string());
                            cx.notify();
                        }))
                    )
                )
        )
        // --- With validation ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With validation"), theme))
                .child(
                    Field::from_spec({
                            let mut field = FieldSpec::new("email-field", "Email")
                                .with_description("A valid email address is required.")
                                .with_validation_state(validation_state);
                            if !email_is_valid {
                                field = field.with_error("Please enter a valid email address.");
                            }
                            field
                        },
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("email-field")
                                .with_placeholder("you@example.com")
                                .with_value(&email_value)
                                .with_validation_state(validation_state),
                            theme,
                        )
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("text-input-email".to_string(), val.to_string());
                            cx.notify();
                        }))
                    )
                )
        )
        // --- Async validation ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Async validation"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("workspace-field", "Workspace")
                            .with_description("Check whether the workspace handle is available.")
                            .with_validation_state(ValidationState::Pending)
                            .with_pending_message("Checking availability..."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("workspace-field")
                                .with_value(&workspace_value)
                                .with_validation_state(ValidationState::Pending),
                            theme,
                        )
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("text-input-workspace".to_string(), val.to_string());
                            cx.notify();
                        }))
                    )
                )
        )
        // --- Slug ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Slug"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("slug-field", "URL slug")
                            .with_description("Lowercase letters, numbers, and hyphens."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("slug-field")
                                .with_prefix("poodle.dev/")
                                .with_value("team-alpha")
                                .with_placeholder("your-workspace"),
                            theme,
                        )
                    )
                )
        )
        // --- Search ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Search"), theme))
                .child(
                    TextInput::from_spec(
                        TextInputSpec::new()
                            .with_id("search-field")
                            .with_input_type("search")
                            .with_placeholder("Search components…"),
                        theme,
                    )
                )
        )
        // --- Prefix and suffix ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Prefix and suffix"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("price-field", "Monthly cost"),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("price-field")
                                .with_prefix("$")
                                .with_suffix("/mo")
                                .with_value("29")
                                .with_input_type("number"),
                            theme,
                        )
                    )
                )
        )
        // --- Suffix only ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Suffix only"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("weight-field", "Weight"),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("weight-field")
                                .with_suffix("kg")
                                .with_value("72")
                                .with_input_type("number"),
                            theme,
                        )
                    )
                )
        )
        // --- Multiline (explicit type) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Multiline (explicit type)"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("bio-field", "Bio")
                            .with_description("Tell us a bit about yourself."),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("bio-field")
                                .with_input_type("multiline")
                                .with_placeholder("A few sentences…")
                                .with_rows(4),
                            theme,
                        )
                    )
                )
        )
        // --- Multiline (auto-detected from rows) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Multiline (auto-detected from rows)"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("notes-field", "Notes"),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("notes-field")
                                .with_placeholder("Scratch space…")
                                .with_rows(3),
                            theme,
                        )
                    )
                )
        )
        // --- Multiline with character count ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Multiline with character count"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("summary-field", "Summary"),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("summary-field")
                                .with_input_type("multiline")
                                .with_rows(3)
                                .with_max_length(140)
                                .with_placeholder("Up to 140 characters…"),
                            theme,
                        )
                    )
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("disabled-field", "API key"),
                        theme,
                    )
                    .with_control(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("disabled-field")
                                .with_value("sk-xxxx-xxxx-xxxx")
                                .with_disabled(true),
                            theme,
                        )
                    )
                )
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "text-input",
        examples,
        |size, theme: &GpuiThemeProvider| {
            TextInput::from_spec(
                TextInputSpec::new().with_placeholder("Text input"),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            TextInput::from_spec(
                TextInputSpec::new().with_placeholder("Text input"),
                theme,
            )
            .density(density)
            .into_any_element()
        },
    )
}
