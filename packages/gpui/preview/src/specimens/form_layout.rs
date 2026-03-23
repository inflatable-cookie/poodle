use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{ButtonSpec, ButtonVariant, TextInputSpec, TextAreaSpec, CheckboxSpec};
use pug_gpui_components::{Button, TextInput, TextArea, Checkbox, FormLayout, Field};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0)).max_w(px(520.0))
        // ── Two-column layout ─────────────────────────────────────
        .child(section_label("TWO-COLUMN LAYOUT", text_secondary))
        .child(
            FormLayout::new(theme)
                .description("Fill in the details below to create a new user account.")
                .columns(2)
                .with_child(
                    Field::new("fl-first", "First name", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Jane"),
                            theme,
                        ).with_id("fl-first"))
                )
                .with_child(
                    Field::new("fl-last", "Last name", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Doe"),
                            theme,
                        ).with_id("fl-last"))
                )
                .with_child(
                    Field::new("fl-email", "Email", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("jane@example.com"),
                            theme,
                        ).with_id("fl-email"))
                )
                .with_child(
                    Field::new("fl-phone", "Phone", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Phone number"),
                            theme,
                        ).with_id("fl-phone"))
                )
                .with_child(
                    Field::new("fl-notes", "Notes", theme)
                        .with_control(TextArea::from_spec(
                            TextAreaSpec::new().with_placeholder("Any additional notes..."),
                            theme,
                        ).with_id("fl-notes"))
                )
                .with_actions(
                    div().flex().gap(px(8.0)).justify_end()
                        .child(
                            Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Cancel"),
                                theme,
                            ).with_id("fl-cancel")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Create user"),
                                theme,
                            ).with_id("fl-create")
                        )
                )
        )

        // ── Three-column layout ──────────────────────────────────
        .child(section_label("THREE-COLUMN LAYOUT", text_secondary))
        .child(
            FormLayout::new(theme)
                .description("Three fields per row for compact entry forms.")
                .columns(3)
                .with_child(
                    Field::new("fl-mix-first", "First name", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Jane"),
                            theme,
                        ).with_id("fl-mix-first"))
                )
                .with_child(
                    Field::new("fl-mix-middle", "Middle name", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("M."),
                            theme,
                        ).with_id("fl-mix-middle"))
                )
                .with_child(
                    Field::new("fl-mix-last", "Last name", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Doe"),
                            theme,
                        ).with_id("fl-mix-last"))
                )
                .with_child(
                    Field::new("fl-mix-email", "Email", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("jane@example.com"),
                            theme,
                        ).with_id("fl-mix-email"))
                )
                .with_child(
                    Field::new("fl-mix-phone", "Phone", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("+1 555 0100"),
                            theme,
                        ).with_id("fl-mix-phone"))
                )
                .with_child(
                    Field::new("fl-mix-dept", "Department", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Engineering"),
                            theme,
                        ).with_id("fl-mix-dept"))
                )
                .with_actions(
                    div().flex().gap(px(8.0)).justify_end()
                        .child(
                            Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Cancel"),
                                theme,
                            ).with_id("fl-mix-cancel")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Save"),
                                theme,
                            ).with_id("fl-mix-save")
                        )
                )
        )

        // ── Single column ─────────────────────────────────────────
        .child(section_label("SINGLE COLUMN (COLUMNS=1)", text_secondary))
        .child(
            FormLayout::new(theme)
                .columns(1)
                .with_child(
                    Field::new("fl-display", "Display name", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Enter a name"),
                            theme,
                        ).with_id("fl-display"))
                )
                .with_child(
                    Field::new("fl-bio", "Bio", theme)
                        .with_control(TextArea::from_spec(
                            TextAreaSpec::new().with_placeholder("Tell us about yourself..."),
                            theme,
                        ).with_id("fl-bio"))
                )
                .with_child(
                    Checkbox::from_spec(
                        CheckboxSpec::new().with_label("I agree to the terms"),
                        theme,
                    )
                )
                .with_actions(
                    Button::from_spec(
                        ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Save profile"),
                        theme,
                    ).with_id("fl-save-profile")
                )
        )

        // ── With validation error ─────────────────────────────────
        .child(section_label("WITH ERROR AND FIELD ERRORS", text_secondary))
        .child(
            FormLayout::new(theme)
                .error("Unable to save. Please fix the errors below.")
                .columns(2)
                .with_child(
                    Field::new("fl-err-email", "Email", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_value("taken@example.com"),
                            theme,
                        ).with_id("fl-err-email"))
                )
                .with_child(
                    Field::new("fl-err-role", "Role", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Select a role..."),
                            theme,
                        ).with_id("fl-err-role"))
                )
                .with_actions(
                    Button::from_spec(
                        ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Retry"),
                        theme,
                    ).with_id("fl-retry")
                )
        )

        // ── With success message ──────────────────────────────────
        .child(section_label("WITH SUCCESS MESSAGE", text_secondary))
        .child(
            FormLayout::new(theme)
                .success("Settings saved successfully.")
                .columns(1)
                .with_child(
                    Field::new("fl-site", "Site name", theme)
                        .with_control(TextInput::from_spec(
                            TextInputSpec::new().with_value("My Project"),
                            theme,
                        ).with_id("fl-site"))
                )
                .with_actions(
                    Button::from_spec(
                        ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Save"),
                        theme,
                    ).with_id("fl-save-2")
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
