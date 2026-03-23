use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{ButtonSpec, ButtonVariant, TextInputSpec, TextAreaSpec, CheckboxSpec};
use pug_gpui_components::{Button, TextInput, TextArea, Checkbox, FormLayout};
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
                .title("Account Settings")
                .description("Update your account information below.")
                .columns(2)
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("First name"),
                        theme,
                    ).with_id("fl-first")
                )
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("Last name"),
                        theme,
                    ).with_id("fl-last")
                )
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("Email address"),
                        theme,
                    ).with_id("fl-email")
                )
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("Phone number"),
                        theme,
                    ).with_id("fl-phone")
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
                                ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Save"),
                                theme,
                            ).with_id("fl-save")
                        )
                )
        )

        // ── Single column ─────────────────────────────────────────
        .child(section_label("SINGLE COLUMN (COLUMNS=1)", text_secondary))
        .child(
            FormLayout::new(theme)
                .title("Profile")
                .columns(1)
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("Display name"),
                        theme,
                    ).with_id("fl-display")
                )
                .with_child(
                    TextArea::from_spec(
                        TextAreaSpec::new().with_placeholder("Bio"),
                        theme,
                    ).with_id("fl-bio")
                )
                .with_child(
                    Checkbox::from_spec(
                        CheckboxSpec::new().with_label("Make profile public"),
                        theme,
                    )
                )
                .with_actions(
                    div().flex().gap(px(8.0)).justify_end()
                        .child(
                            Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Cancel"),
                                theme,
                            ).with_id("fl-cancel-2")
                        )
                        .child(
                            Button::from_spec(
                                ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Update Profile"),
                                theme,
                            ).with_id("fl-update")
                        )
                )
        )

        // ── With validation error ─────────────────────────────────
        .child(section_label("WITH VALIDATION ERROR", text_secondary))
        .child(
            FormLayout::new(theme)
                .title("Login")
                .error("Invalid credentials. Please try again.")
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("Username"),
                        theme,
                    ).with_id("fl-user")
                )
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("Password"),
                        theme,
                    ).with_id("fl-pass")
                )
                .with_actions(
                    Button::from_spec(
                        ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Sign in"),
                        theme,
                    ).with_id("fl-signin")
                )
        )

        // ── With success message ──────────────────────────────────
        .child(section_label("WITH SUCCESS MESSAGE", text_secondary))
        .child(
            FormLayout::new(theme)
                .title("Settings")
                .success("Your changes have been saved successfully.")
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new()
                            .with_placeholder("Organization name")
                            .with_value("Acme Corp"),
                        theme,
                    ).with_id("fl-org")
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
