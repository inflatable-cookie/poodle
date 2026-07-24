//! ValidationSummary — Jetstream grouped error surface backed by ValidationSummarySpec.
//!
//! Contract: `docs/contracts/components/validation-summary.md`
//! Reference: no standalone Svelte component (contract self-declares "not yet
//! built"); the contract is the sole authority. Anatomy is built directly from
//! the contract §2 table.
//!
//! Anatomy: Root `<aside>`-equivalent surface (border = danger when blocking,
//! accent when pending-only) → optional Title → List of Entry rows (each: a tone
//! indicator + a text column with the field Label and Message). All spacing,
//! sizing, radius, and typography resolve from `ValidationSummarySpec` token
//! methods.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ValidationSummarySpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Semibold label weight (typography constant; see `form_shell.rs::SEMIBOLD`).
const SEMIBOLD: u16 = 600;
const MEDIUM: u16 = 500;

/// Render a `ValidationSummary`.
///
/// Empty state (no active entries) renders nothing per contract §4 — callers
/// that want an explicit "clean" surface render it themselves. ARIA role from
/// `spec.accessibility_role()` is host-applied (contract §8 known delta — no
/// Jetstream a11y channel). The `<a href="#field-id">` focus-jump (contract §5)
/// is web-only; Jetstream carries the field id as an interaction id on the entry
/// so the host can emulate focus imperatively.
pub fn js_validation_summary(spec: &ValidationSummarySpec, theme: &JetstreamThemeProvider) -> JsEl {
    let entries = spec.active_entries();
    if entries.is_empty() {
        // Contract §4 empty state — render an empty surface placeholder so the
        // node tree is well-formed but visually nothing (no border, no padding).
        return ui_element::div();
    }

    let border = resolve_color(theme, spec.border_token());
    let fill = resolve_color(theme, spec.fill_token());
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let danger_color = resolve_color(theme, "color.status.danger");
    let accent_color = resolve_color(theme, "color.accent.base");

    let title_size = resolve_px(theme, spec.title_size_token());
    let entry_size = resolve_px(theme, spec.entry_text_size_token());
    let pad_x = resolve_px(theme, spec.padding_x_token());
    let pad_y = resolve_px(theme, spec.padding_y_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let list_gap = resolve_px(theme, spec.list_gap_token());
    let entry_gap = resolve_px(theme, spec.entry_gap_token());
    let entry_text_gap = resolve_px(theme, spec.entry_text_gap_token());

    // Tone indicator dot sizing derives from the entry text size (a leading
    // bullet, not a contract-anatomy part — see note below). Half-text size.
    let dot_size = entry_size * 0.5;

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .flex_col()
        .gap(list_gap);

    // ── Title (optional) ─────────────────────────────────────
    if let Some(ref title) = spec.title {
        el = el.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(title_size)
                .text_weight(SEMIBOLD),
        );
    }

    // ── List → Entry rows ────────────────────────────────────
    let mut list = ui_element::div().flex_col().gap(list_gap);
    for entry in &entries {
        let tone_color = if entry.is_blocking() {
            danger_color
        } else {
            accent_color
        };

        let mut row = ui_element::div()
            // Field id carried as the interaction id for host focus-jump emulation.
            .id(format!("validation-summary-entry:{}", entry.field_id))
            .flex_row()
            .items_start()
            .gap(entry_gap);

        // Leading tone indicator dot (NOT a contract-anatomy part — see module
        // note; kept as a visual enrichment, token-derived from entry text size).
        row = row.child(
            ui_element::div()
                .w(dot_size)
                .h(dot_size)
                .rounded(dot_size * 0.5)
                .bg(tone_color),
        );

        // Text column: Label + Message.
        let mut text_col = ui_element::div().flex_col().gap(entry_text_gap);
        text_col = text_col.child(
            ui_element::label(&entry.label)
                .text_color(text_primary)
                .text_size(entry_size)
                .text_weight(MEDIUM),
        );
        text_col = text_col.child(
            ui_element::label(&entry.message)
                .text_color(text_secondary)
                .text_size(entry_size),
        );

        row = row.child(text_col);
        list = list.child(row);
    }
    el = el.child(list);

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ValidationState, ValidationSummaryEntry};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn invalid_entries() -> Vec<ValidationSummaryEntry> {
        vec![
            ValidationSummaryEntry::new(
                "title",
                "Project title",
                "Title is required.",
                ValidationState::Invalid,
            ),
            ValidationSummaryEntry::new(
                "slug",
                "URL slug",
                "Slug contains invalid characters.",
                ValidationState::Invalid,
            ),
        ]
    }

    #[test]
    fn renders_title_and_entry_rows() {
        let spec = ValidationSummarySpec::new(invalid_entries())
            .with_title("Fix the following fields");
        let tree = probe(&js_validation_summary(&spec, &theme()), 400.0, 300.0);
        assert!(!tree.is_empty(), "summary produced no nodes");
        assert!(
            tree.has_text("Fix the following fields"),
            "title missing: {:?}",
            tree.texts()
        );
        // Both entries: labels + messages.
        assert!(tree.has_text("Project title"), "entry 1 label missing");
        assert!(tree.has_text("Title is required."), "entry 1 message missing");
        assert!(tree.has_text("URL slug"), "entry 2 label missing");
        // Field-id interaction ids present for host focus-jump emulation.
        assert!(
            tree.find_token("validation-summary-entry:title").is_some(),
            "entry field-id token missing"
        );
        assert!(
            tree.find_token("validation-summary-entry:slug").is_some(),
            "entry 2 field-id token missing"
        );
    }

    #[test]
    fn blocking_uses_danger_border_pending_only_uses_accent() {
        // Blocking (invalid) → danger border.
        let blocking = ValidationSummarySpec::new(invalid_entries());
        assert_eq!(blocking.border_token(), "color.status.danger");

        // Pending-only with include_pending → accent border (non-blocking).
        let pending = ValidationSummarySpec::new(vec![ValidationSummaryEntry::new(
            "search",
            "Asset search",
            "Checking references.",
            ValidationState::Pending,
        )])
        .with_include_pending(true);
        assert_eq!(pending.border_token(), "color.accent.base");

        // Both render their entry surfaces with distinct border colors.
        let b = probe(&js_validation_summary(&blocking, &theme()), 400.0, 300.0);
        let p = probe(&js_validation_summary(&pending, &theme()), 400.0, 300.0);
        assert!(p.has_text("Asset search"), "pending entry not rendered");
        assert!(!b.is_empty() && !p.is_empty());
    }

    #[test]
    fn pending_excluded_by_default() {
        // include_pending=false → a pending-only summary has no active entries
        // and renders the empty placeholder.
        let spec = ValidationSummarySpec::new(vec![ValidationSummaryEntry::new(
            "search",
            "Asset search",
            "Checking references.",
            ValidationState::Pending,
        )]);
        let tree = probe(&js_validation_summary(&spec, &theme()), 400.0, 300.0);
        assert!(
            !tree.has_text("Asset search"),
            "pending entry should be excluded by default: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn mixed_pending_and_invalid_keeps_danger_border() {
        let mut entries = invalid_entries();
        entries.push(ValidationSummaryEntry::new(
            "search",
            "Asset search",
            "Checking references.",
            ValidationState::Pending,
        ));
        let spec = ValidationSummarySpec::new(entries).with_include_pending(true);
        // Danger precedence over pending.
        assert_eq!(spec.border_token(), "color.status.danger");
        let tree = probe(&js_validation_summary(&spec, &theme()), 400.0, 300.0);
        assert!(tree.has_text("Asset search"), "pending entry missing in mixed");
        assert!(tree.has_text("Project title"), "invalid entry missing in mixed");
    }

    #[test]
    fn empty_renders_nothing_substantive() {
        let spec = ValidationSummarySpec::new(vec![]).with_title("Validation");
        let tree = probe(&js_validation_summary(&spec, &theme()), 400.0, 300.0);
        // Empty placeholder: no title, no entry rows.
        assert!(
            !tree.has_text("Validation"),
            "empty summary should render no title: {:?}",
            tree.texts()
        );
    }
}
