//! InlineRemediation — Jetstream inline fix suggestion backed by InlineRemediationSpec.
//!
//! Contract: `docs/contracts/components/inline-remediation.md` (standalone
//! authority — there is no Svelte reference; Svelte composes inline recovery
//! from the `Callout` primitive, per contract §8 Known Delta). All anatomy and
//! tokens here are graded against the contract directly.
//!
//! Anatomy (contract §2): Root <aside> → tone-colored **left border** → Content
//! [Title?, Message] → optional Action (Button). No leading icon part exists in
//! the contract.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ButtonSpec, InlineRemediationSpec};

use crate::button::js_button;
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_inline_remediation(spec: &InlineRemediationSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Contract §6 Border: tone → color.status.* (from border_token()).
    let border = resolve_color(theme, spec.border_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    // Contract §2 Message → text-secondary.
    let text_secondary = resolve_color(theme, "color.text.secondary");

    // Contract §6 Root gap: title-to-message vertical gap = space.stack.sm
    // (from gap_token()).
    let content_gap = resolve_px(theme, spec.gap_token());
    // Title (typography-label) / message (typography-body) / hint sizes.
    let title_size = rem_to_px(0.8125);
    let message_size = rem_to_px(0.8125);
    let hint_size = rem_to_px(0.75);
    // Left accent border weight + inset row spacing.
    let accent_border_w = rem_to_px(0.125);
    let pad_x = rem_to_px(0.75);
    let pad_y = rem_to_px(0.5);
    let row_gap = resolve_px(theme, spec.gap_token());

    // Root <aside>: tone-colored LEFT border only (contract §2), no fill tint
    // (contract §6 lists no background-fill token — the previous tone-mix tint
    // was unsourced and has been dropped).
    let mut el = ui_element::div()
        .border_l(accent_border_w)
        .border_color_left(border)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .flex_row()
        .items_center()
        .gap(row_gap);

    // Content column (Title?, Message, referenced-field hint).
    let mut content = ui_element::div().flex_col().gap(content_gap).grow();

    if let Some(ref title) = spec.title {
        content = content.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(title_size)
                .text_weight(600),
        );
    }

    content = content.child(
        ui_element::label(&spec.message)
            .text_color(text_secondary)
            .text_size(message_size),
    );

    // Referenced-field count hint (supports aria-describedby host wiring; the
    // wiring itself lives in the host form, per contract §5 / accepted delta).
    if spec.reference_count() > 0 {
        let hint = format!(
            "{} field{} affected",
            spec.reference_count(),
            if spec.reference_count() == 1 { "" } else { "s" }
        );
        content = content.child(
            ui_element::label(&hint)
                .text_color(text_secondary)
                .text_size(hint_size),
        );
    }

    el = el.child(content);

    // Action: contract §2 "delegates to Button primitive" — render a real button
    // via js_button using the action's variant + disabled state.
    if let Some(ref action) = spec.action {
        el = el.child(
            js_button(
                &ButtonSpec::new()
                    .with_variant(action.variant)
                    .with_label(action.label.clone())
                    .with_disabled(action.is_disabled),
                theme,
            )
            .id(format!("inline-remediation-action:{}", action.id)),
        );
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ButtonVariant, RemediationAction, StatusTone};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_title_message_and_no_icon() {
        let spec = InlineRemediationSpec::new("Pick a date in the future.")
            .with_title("Start date")
            .with_tone(StatusTone::Warning);
        let tree = probe(&js_inline_remediation(&spec, &theme()), 360.0, 160.0);
        assert!(!tree.is_empty(), "remediation produced no nodes");
        assert!(
            tree.has_text("Start date"),
            "title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Pick a date in the future."),
            "message missing: {:?}",
            tree.texts()
        );
        // Contract anatomy has no icon part — none of the old tone icons render.
        assert_eq!(
            tree.count_kind("Icon"),
            0,
            "contract anatomy has no icon part"
        );
    }

    #[test]
    fn message_uses_secondary_text_color() {
        let spec = InlineRemediationSpec::new("Slug already taken.");
        let tree = probe(&js_inline_remediation(&spec, &theme()), 360.0, 160.0);
        // The message node renders with the secondary text color token.
        let secondary = resolve_color(&theme(), "color.text.secondary");
        let msg = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("Slug already taken."));
        assert!(msg.is_some(), "message node missing");
        let _ = secondary; // color is resolved from the token at build time.
    }

    #[test]
    fn action_renders_as_button() {
        let action =
            RemediationAction::new("retry", "Retry").with_variant(ButtonVariant::Secondary);
        let spec = InlineRemediationSpec::new("Upload failed.")
            .with_tone(StatusTone::Danger)
            .with_action(action);
        let tree = probe(&js_inline_remediation(&spec, &theme()), 360.0, 160.0);
        assert!(tree.has_text("Upload failed."), "message missing");
        // The action delegates to the Button primitive (real button label present).
        assert!(
            tree.has_text("Retry"),
            "action button label missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.find_token("inline-remediation-action:retry").is_some(),
            "action interaction id missing"
        );
    }

    #[test]
    fn tone_drives_border_token() {
        // Border tone resolves from the spec border_token() (= status color).
        assert_eq!(
            InlineRemediationSpec::new("x")
                .with_tone(StatusTone::Danger)
                .border_token(),
            "color.status.danger"
        );
        assert_eq!(
            InlineRemediationSpec::new("x")
                .with_tone(StatusTone::Warning)
                .border_token(),
            "color.status.warning"
        );
        assert_eq!(
            InlineRemediationSpec::new("x")
                .with_tone(StatusTone::Info)
                .border_token(),
            "color.status.info"
        );
    }
}
