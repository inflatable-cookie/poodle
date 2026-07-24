//! FormShell — Jetstream form container backed by FormShellSpec.
//!
//! Contract: `docs/contracts/components/form-shell.md`
//! Reference: no standalone Svelte component (Rust-only shared spec; contract is
//! the sole authority — see the parity doc). Anatomy is built directly from the
//! contract §2 table.
//!
//! Anatomy: Root stack → optional header (Title `<h2>` + Description `<p>`) →
//! optional StatusSummary `Callout` → Sections (each: SectionTitle `<h3>` +
//! optional SectionDescription + host-supplied field slots) → Actions row.
//! All spacing/typography/opacity resolve from `FormShellSpec` token methods.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CallOutSpec, FormShellSpec, StatusTone};

use crate::callout::js_callout;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// Semibold heading/label weight. Font weight is a typography constant (no px
/// dimension); mirrors the GPUI `FontWeight::SEMIBOLD` and the contract's
/// `typography.heading.weight` / `typography.label.weight` (600). No numeric
/// weight resolver exists on the theme, matching the established convention in
/// `callout.rs` / `relation_picker.rs`.
const SEMIBOLD: u16 = 600;

/// Render a `FormShell`.
///
/// `section_slots[i]` is the host-rendered field content for `spec.sections[i]`
/// (the contract delegates field rendering to the host, referenced via
/// `field_ids`). `actions_slot` is the host-rendered action row content (buttons);
/// the shell wraps it in the actions region honoring `actions.align`.
///
/// Submit/interaction is host-driven by design (contract §3) — the shell exposes
/// gating via `spec.blocks_submission()` and renders no click handler. The
/// actions region is dimmed when submission is blocked (preview-loop wires the
/// actual disabled buttons).
pub fn js_form_shell(
    spec: &FormShellSpec,
    theme: &JetstreamThemeProvider,
    section_slots: Vec<Option<JsEl>>,
    actions_slot: Option<JsEl>,
) -> JsEl {
    let stack_gap = resolve_px(theme, spec.stack_gap_token());
    let section_gap = resolve_px(theme, spec.section_gap_token());
    let header_gap = resolve_px(theme, spec.header_gap_token());
    let section_inner_gap = resolve_px(theme, spec.section_internal_gap_token());

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");

    let title_size = resolve_px(theme, spec.title_size_token());
    let desc_size = resolve_px(theme, spec.description_size_token());
    let section_title_size = resolve_px(theme, spec.section_title_size_token());

    let mut el = ui_element::div().flex_col().gap(stack_gap);

    // ── Header: Title (<h2>) + Description (<p>) ──────────────
    if spec.title.is_some() || spec.description.is_some() {
        let mut header = ui_element::div().flex_col().gap(header_gap);

        if let Some(ref title) = spec.title {
            header = header.child(
                ui_element::label(title)
                    .text_color(text_primary)
                    .text_size(title_size)
                    .text_weight(SEMIBOLD),
            );
        }

        if let Some(ref desc) = spec.description {
            header = header.child(
                ui_element::label(desc)
                    .text_color(text_secondary)
                    .text_size(desc_size),
            );
        }

        el = el.child(header);
    }

    // ── StatusSummary: a Callout driven by resolved_status_tone() ─
    // Contract §2/§5/§6: form-level status is a `Callout`. The effective tone is
    // derived from field state (danger when invalid, pending when busy/pending),
    // falling back to the explicit status_summary tone, then Neutral. The message
    // comes from the explicit status_summary when present, else a derived count.
    let resolved_tone = spec.resolved_status_tone();
    let status_message = status_message(spec, resolved_tone);
    if let Some(message) = status_message {
        let callout = js_callout(
            &CallOutSpec::new()
                .with_tone(resolved_tone)
                .with_content(message),
            theme,
        );
        el = el.child(callout);
    }

    // ── Sections ─────────────────────────────────────────────
    if !spec.sections.is_empty() {
        let mut sections_container = ui_element::div().flex_col().gap(section_gap);

        for (idx, section_spec) in spec.sections.iter().enumerate() {
            let mut section = ui_element::div().flex_col().gap(section_inner_gap);

            // SectionTitle (<h3>)
            section = section.child(
                ui_element::label(&section_spec.title)
                    .text_color(text_primary)
                    .text_size(section_title_size)
                    .text_weight(SEMIBOLD),
            );

            // SectionDescription (<p>, optional)
            if let Some(ref desc) = section_spec.description {
                section = section.child(
                    ui_element::label(desc)
                        .text_color(text_secondary)
                        .text_size(desc_size),
                );
            }

            // Fields — host-supplied slot referenced via field_ids.
            if let Some(slot) = section_slots.get(idx).and_then(|s| s.clone()) {
                section = section.child(slot);
            }

            sections_container = sections_container.child(section);
        }

        el = el.child(sections_container);
    }

    // ── Actions row (FormActionLayout align) ─────────────────
    if let Some(actions) = actions_slot {
        let mut row = ui_element::div().flex_row().items_center();
        // Honor actions.align (contract §2/§3). FormActionAlign lacks Center —
        // start/end/between only.
        use poodle_specs::FormActionAlign;
        row = match spec.actions.align {
            FormActionAlign::Start => row.justify_start(),
            FormActionAlign::End => row.justify_end(),
            FormActionAlign::Between => row.justify_between(),
        };
        row = row.child(actions);
        // Dim the actions when submission is blocked (busy / disabled / invalid).
        if spec.blocks_submission() {
            row = row.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
        }
        el = el.child(row);
    }

    // ── Disabled: dim the whole shell (contract §4) ──────────
    if spec.is_disabled {
        el = el.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    el
}

/// Derive the status-summary message. Returns `None` when there is nothing to
/// announce (ready state with no explicit summary).
fn status_message(spec: &FormShellSpec, resolved_tone: StatusTone) -> Option<String> {
    if let Some(ref summary) = spec.status_summary {
        return Some(summary.message.clone());
    }

    let invalid = spec.invalid_field_count();
    let pending = spec.pending_field_count();

    match resolved_tone {
        StatusTone::Danger if invalid > 0 => Some(format!(
            "{} field{} need{} attention.",
            invalid,
            if invalid == 1 { "" } else { "s" },
            if invalid == 1 { "s" } else { "" },
        )),
        StatusTone::Pending if spec.is_busy => Some("Submitting…".to_string()),
        StatusTone::Pending if pending > 0 => Some(format!(
            "{} field{} validating…",
            pending,
            if pending == 1 { "" } else { "s" },
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{
        FormActionAlign, FormFieldState, FormSectionSpec, ValidationState,
    };

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn sections() -> Vec<FormSectionSpec> {
        vec![
            FormSectionSpec::new("details", "Project Details", vec!["name".into()])
                .with_description("Core metadata."),
            FormSectionSpec::new("settings", "Settings", vec!["visibility".into()]),
        ]
    }

    #[test]
    fn renders_header_sections_and_actions() {
        let spec = FormShellSpec::new("project-form")
            .with_title("New Project")
            .with_description("Create a new project workspace.")
            .with_sections(sections())
            .with_actions(FormActionAlign::End, 2);
        let actions = ui_element::div()
            .child(ui_element::button("Cancel"))
            .child(ui_element::button("Save"));
        let tree = probe(
            &js_form_shell(&spec, &theme(), vec![None, None], Some(actions)),
            500.0,
            500.0,
        );
        assert!(!tree.is_empty(), "form shell produced no nodes");
        // Header
        assert!(tree.has_text("New Project"), "title missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Create a new project workspace."),
            "description missing: {:?}",
            tree.texts()
        );
        // Sections: titles + a section description
        assert!(tree.has_text("Project Details"), "section 1 title missing");
        assert!(tree.has_text("Settings"), "section 2 title missing");
        assert!(tree.has_text("Core metadata."), "section description missing");
        // Footer actions
        assert!(tree.has_text("Cancel"), "cancel action missing");
        assert!(tree.has_text("Save"), "save action missing");
    }

    #[test]
    fn renders_field_slot_per_section() {
        let spec = FormShellSpec::new("f").with_sections(sections());
        let slot0 = ui_element::label("FIELD-SLOT-NAME");
        let tree = probe(
            &js_form_shell(&spec, &theme(), vec![Some(slot0), None], None),
            400.0,
            400.0,
        );
        assert!(
            tree.has_text("FIELD-SLOT-NAME"),
            "section field slot not rendered: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn invalid_fields_render_danger_status_callout() {
        let fields = vec![
            FormFieldState::new("name", "Name")
                .with_validation_state(ValidationState::Invalid),
        ];
        let spec = FormShellSpec::new("f")
            .with_title("T")
            .with_fields(fields);
        let tree = probe(&js_form_shell(&spec, &theme(), vec![], None), 400.0, 400.0);
        // Callout renders an icon badge — danger tone uses the "circle-x" glyph.
        assert!(
            tree.count_kind("Icon") >= 1,
            "status callout icon badge missing: {:?}",
            tree.texts()
        );
        assert_eq!(spec.resolved_status_tone(), StatusTone::Danger);
        assert!(spec.blocks_submission(), "invalid field should block submission");
    }

    #[test]
    fn explicit_status_summary_message_renders() {
        let spec = FormShellSpec::new("f")
            .with_status_summary(StatusTone::Warning, "Heads up: draft not saved.");
        let tree = probe(&js_form_shell(&spec, &theme(), vec![], None), 400.0, 400.0);
        assert!(
            tree.has_text("Heads up: draft not saved."),
            "explicit status summary message missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn busy_state_renders_pending_status() {
        let spec = FormShellSpec::new("f").with_title("T").with_busy(true);
        let tree = probe(&js_form_shell(&spec, &theme(), vec![], None), 400.0, 400.0);
        assert_eq!(spec.resolved_status_tone(), StatusTone::Pending);
        // Pending callout renders a spinner badge (more than the bare header).
        assert!(tree.len() > 2, "busy status not rendered: {:?}", tree.texts());
        assert!(spec.blocks_submission(), "busy should block submission");
    }

    #[test]
    fn disabled_dims_the_shell() {
        let spec = FormShellSpec::new("f").with_title("T").with_disabled(true);
        let tree = probe(&js_form_shell(&spec, &theme(), vec![], None), 400.0, 400.0);
        assert!(!tree.is_empty(), "disabled form should still render");
        assert!(spec.blocks_submission(), "disabled should block submission");
    }

    #[test]
    fn ready_state_renders_no_status_callout() {
        // No fields, no explicit summary → no status region.
        let spec = FormShellSpec::new("f")
            .with_title("T")
            .with_sections(sections());
        let tree = probe(&js_form_shell(&spec, &theme(), vec![None, None], None), 400.0, 400.0);
        // No callout icon badge present in the ready state.
        assert_eq!(
            tree.count_kind("Icon"),
            0,
            "unexpected status callout in ready state: {:?}",
            tree.texts()
        );
    }
}
