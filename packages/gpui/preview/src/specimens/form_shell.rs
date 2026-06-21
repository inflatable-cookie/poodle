//! FormShell specimen — exercises the GPUI `FormShell` composite across the
//! contract's four lifecycle states (ready / blocked / busy / disabled).
//!
//! Every group builds a real `FormShell` from `FormShellSpec`: header
//! (title + description), sectioned layout whose field slots host real `Field`
//! + `TextInput` controls (the contract delegates field rendering to the host,
//! referenced via `field_ids`), a form-level `StatusSummary` rendered as a real
//! `Callout` (tone derived by `resolved_status_tone()`), and a footer
//! `FormActions` row of real `Button`s. No hand-rolled chrome — all spacing,
//! typography, and opacity resolve from the spec's token methods.

use crate::app_state::AppState;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, Eyebrow, Field, FormShell, TextInput};
use poodle_specs::{
    ButtonSpec, ButtonVariant, EyebrowSpec, FormActionAlign, FormFieldState, FormSectionSpec,
    FormShellSpec, StatusTone, TextInputSpec, ValidationState,
};

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .gap(px(32.0))
        .max_w(px(560.0))
        .child(group(
            theme,
            "Ready",
            ready_spec(),
            FieldStates::Ready,
        ))
        .child(group(
            theme,
            "Blocked (invalid field)",
            blocked_spec(),
            FieldStates::Invalid,
        ))
        .child(group(
            theme,
            "Busy (submitting)",
            busy_spec(),
            FieldStates::Ready,
        ))
        .child(group(
            theme,
            "Disabled",
            disabled_spec(),
            FieldStates::Ready,
        ))
}

/// Which validation state to paint on the email field's `Field` chrome so the
/// host slot visually matches the spec's tracked field state.
#[derive(Clone, Copy)]
enum FieldStates {
    Ready,
    Invalid,
}

/// A labelled specimen group: eyebrow + a fully-built `FormShell`.
fn group(theme: &GpuiThemeProvider, label: &str, spec: FormShellSpec, fields: FieldStates) -> Div {
    let disabled = spec.is_disabled;
    let busy = spec.is_busy;

    // ── Account section slot (real Field + TextInput) ────────
    let name_field = Field::new("fs-name", "Full name", theme).with_control(
        TextInput::from_spec(
            TextInputSpec::new()
                .with_value("Jane Doe")
                .with_disabled(disabled || busy),
            theme,
        )
        .with_id(format!("fs-name-{label}")),
    );

    let mut email_field = Field::new("fs-email", "Email", theme);
    let email_input = match fields {
        FieldStates::Ready => TextInputSpec::new()
            .with_value("jane@example.com")
            .with_disabled(disabled || busy),
        FieldStates::Invalid => {
            email_field = email_field
                .validation_state(ValidationState::Invalid)
                .error("Enter a valid email address");
            TextInputSpec::new()
                .with_value("jane@@example")
                .with_validation_state(ValidationState::Invalid)
                .with_disabled(disabled || busy)
        }
    };
    let email_field = email_field
        .with_control(TextInput::from_spec(email_input, theme).with_id(format!("fs-email-{label}")));

    let account_slot = div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(name_field)
        .child(email_field);

    // ── Profile section slot (real Field + TextInput) ────────
    let bio_slot = Field::new("fs-bio", "Bio", theme).with_control(
        TextInput::from_spec(
            TextInputSpec::new()
                .with_placeholder("A few words about yourself…")
                .with_rows(3)
                .with_disabled(disabled || busy),
            theme,
        )
        .with_id(format!("fs-bio-{label}")),
    );

    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(label), theme))
        .child(
            FormShell::from_spec(spec, theme)
                .with_section_slot(account_slot)
                .with_section_slot(bio_slot)
                .with_action(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label("Cancel")
                            .with_disabled(disabled || busy),
                        theme,
                    )
                    .with_id(format!("fs-cancel-{label}")),
                )
                .with_action(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Primary)
                            .with_label(if busy { "Saving…" } else { "Save changes" })
                            .with_loading(busy)
                            .with_disabled(disabled),
                        theme,
                    )
                    .with_id(format!("fs-save-{label}")),
                ),
        )
}

/// Two-section spec shared by every group; callers tweak field state / flags.
fn base_spec() -> FormShellSpec {
    FormShellSpec::new("fs-demo")
        .with_title("Account settings")
        .with_description("Update your account profile and contact details.")
        .with_sections(vec![
            FormSectionSpec::new(
                "account",
                "Account",
                vec!["fs-name".into(), "fs-email".into()],
            )
            .with_description("Your sign-in identity."),
            FormSectionSpec::new("profile", "Profile", vec!["fs-bio".into()]),
        ])
        .with_actions(FormActionAlign::End, 2)
}

fn ready_spec() -> FormShellSpec {
    base_spec().with_fields(vec![
        FormFieldState::new("fs-name", "Full name").with_validation_state(ValidationState::Valid),
        FormFieldState::new("fs-email", "Email").with_validation_state(ValidationState::Valid),
        FormFieldState::new("fs-bio", "Bio"),
    ])
}

fn blocked_spec() -> FormShellSpec {
    base_spec().with_fields(vec![
        FormFieldState::new("fs-name", "Full name").with_validation_state(ValidationState::Valid),
        FormFieldState::new("fs-email", "Email")
            .with_validation_state(ValidationState::Invalid)
            .with_message("Enter a valid email address"),
        FormFieldState::new("fs-bio", "Bio"),
    ])
}

fn busy_spec() -> FormShellSpec {
    base_spec()
        .with_busy(true)
        .with_status_summary(StatusTone::Pending, "Submitting…")
        .with_fields(vec![
            FormFieldState::new("fs-name", "Full name")
                .with_validation_state(ValidationState::Valid),
            FormFieldState::new("fs-email", "Email")
                .with_validation_state(ValidationState::Valid),
            FormFieldState::new("fs-bio", "Bio"),
        ])
}

fn disabled_spec() -> FormShellSpec {
    base_spec().with_disabled(true).with_fields(vec![
        FormFieldState::new("fs-name", "Full name").with_disabled(true),
        FormFieldState::new("fs-email", "Email").with_disabled(true),
        FormFieldState::new("fs-bio", "Bio").with_disabled(true),
    ])
}
