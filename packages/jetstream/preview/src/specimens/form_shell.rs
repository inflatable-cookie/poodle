//! FormShell specimen — exercises the Jetstream `js_form_shell` composite across
//! the contract's four lifecycle states (ready / blocked / busy / disabled).
//!
//! Real components only (CLAUDE.md "no fakes"): every group builds a real
//! `js_form_shell` from `FormShellSpec`. The shell renders the header
//! (title + description), the sectioned layout whose field slots host real
//! `js_field` + `js_text_input` controls (the contract delegates field rendering
//! to the host, referenced via `field_ids`), the form-level StatusSummary as a
//! real `js_callout` (tone derived by `resolved_status_tone()`), and the footer
//! FormActions row of real `js_button`s. All spacing/typography/opacity resolve
//! from the spec's token methods inside the component — the specimen adds no
//! chrome of its own beyond the eyebrow group label.

use crate::compat::js_button;
use crate::compat::js_field;
use crate::compat::js_form_shell;
use crate::compat::js_text_input;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ButtonSpec, ButtonVariant, FieldSpec, FormActionAlign, FormFieldState, FormSectionSpec,
    FormShellSpec, StatusTone, TextInputSpec, ValidationState,
};

/// Which validation state to paint on the email field's `Field` chrome so the
/// host slot visually matches the spec's tracked field state.
#[derive(Clone, Copy)]
enum FieldStates {
    Ready,
    Invalid,
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(32.0)
        .w(560.0)
        .child(group(
            "Ready",
            secondary,
            shell(theme, "ready", ready_spec(), FieldStates::Ready),
        ))
        .child(group(
            "Blocked (invalid field)",
            secondary,
            shell(theme, "blocked", blocked_spec(), FieldStates::Invalid),
        ))
        .child(group(
            "Busy (submitting)",
            secondary,
            shell(theme, "busy", busy_spec(), FieldStates::Ready),
        ))
        .child(group(
            "Disabled",
            secondary,
            shell(theme, "disabled", disabled_spec(), FieldStates::Ready),
        ))
}

/// Build a fully-real `FormShell`: header + sections (host Field slots) +
/// StatusSummary callout + FormActions footer of real Buttons.
fn shell(
    theme: &JetstreamThemeProvider,
    key: &str,
    spec: FormShellSpec,
    fields: FieldStates,
) -> El {
    let disabled = spec.is_disabled;
    let busy = spec.is_busy;
    let inert = disabled || busy;

    // ── Account section slot (real Field + TextInput) ────────
    let name_slot = js_field(
        &FieldSpec::new("fs-name", "Full name"),
        theme,
        Some(js_text_input(
            &TextInputSpec::new()
                .with_id(format!("fs-name-{key}"))
                .with_value("Jane Doe")
                .with_disabled(inert),
            theme,
        )),
    );

    let (email_field_spec, email_input_spec) = match fields {
        FieldStates::Ready => (
            FieldSpec::new("fs-email", "Email"),
            TextInputSpec::new()
                .with_id(format!("fs-email-{key}"))
                .with_value("jane@example.com")
                .with_disabled(inert),
        ),
        FieldStates::Invalid => (
            FieldSpec::new("fs-email", "Email")
                .with_validation_state(ValidationState::Invalid)
                .with_error("Enter a valid email address"),
            TextInputSpec::new()
                .with_id(format!("fs-email-{key}"))
                .with_value("jane@@example")
                .with_validation_state(ValidationState::Invalid)
                .with_disabled(inert),
        ),
    };
    let email_slot = js_field(
        &email_field_spec,
        theme,
        Some(js_text_input(&email_input_spec, theme)),
    );

    let account_slot = div()
        .flex_col()
        .gap(12.0)
        .child(name_slot)
        .child(email_slot);

    // ── Profile section slot (real Field + TextInput) ────────
    let bio_slot = js_field(
        &FieldSpec::new("fs-bio", "Bio"),
        theme,
        Some(js_text_input(
            &TextInputSpec::new()
                .with_id(format!("fs-bio-{key}"))
                .with_placeholder("A few words about yourself…")
                .with_rows(3)
                .with_disabled(inert),
            theme,
        )),
    );

    // ── FormActions footer (real Buttons) ────────────────────
    let actions = div()
        .flex_row()
        .gap(8.0)
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label("Cancel")
                .with_disabled(inert),
            theme,
        ))
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_label(if busy { "Saving…" } else { "Save changes" })
                .with_loading(busy)
                .with_disabled(disabled),
            theme,
        ));

    js_form_shell(
        &spec,
        theme,
        vec![Some(account_slot), Some(bio_slot)],
        Some(actions),
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
            FormFieldState::new("fs-email", "Email").with_validation_state(ValidationState::Valid),
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

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
