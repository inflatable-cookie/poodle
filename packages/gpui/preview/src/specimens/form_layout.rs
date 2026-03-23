use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{ButtonSpec, ButtonVariant, TextInputSpec};
use pug_gpui_components::{Button, TextInput, FormLayout};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0)).max_w(px(480.0))
        // --- Single column ---
        .child(section_label("SINGLE COLUMN", text_secondary))
        .child(
            FormLayout::new(theme)
                .title("Account Settings")
                .description("Update your account information below.")
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("Full name"),
                        theme,
                    ).with_id("fl-name")
                )
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("Email address"),
                        theme,
                    ).with_id("fl-email")
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
        // --- Two columns ---
        .child(section_label("TWO COLUMNS", text_secondary))
        .child(
            FormLayout::new(theme)
                .title("Billing Information")
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
                        TextInputSpec::new().with_placeholder("City"),
                        theme,
                    ).with_id("fl-city")
                )
                .with_child(
                    TextInput::from_spec(
                        TextInputSpec::new().with_placeholder("Zip code"),
                        theme,
                    ).with_id("fl-zip")
                )
        )
        // --- With validation ---
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
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
