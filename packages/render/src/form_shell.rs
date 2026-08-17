//! FormShell — sectioned form shell with status summary and actions region.
//!
//! Contract: `docs/contracts/components/form-shell.md`
//! Ported from: `packages/jetstream/components/src/form_shell.rs`.
//!
//! `section_slots[i]` is the host-rendered field content for
//! `spec.sections[i]`; `actions_slot` is the host-rendered action row. Submit
//! is host-driven by design (contract §3) — the shell renders no click
//! handler; the actions region dims when submission is blocked.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::{CallOutSpec, FormActionAlign, FormShellSpec, StatusTone};

use crate::callout::{callout, CalloutHandlers};

/// Semibold heading/label weight (typography constant, 600).
const SEMIBOLD: u16 = 600;

pub fn form_shell(
    spec: &FormShellSpec,
    theme: &dyn ThemeProvider,
    section_slots: Vec<Option<Node>>,
    actions_slot: Option<Node>,
) -> Node {
    let stack_gap = theme.resolve_space(spec.stack_gap_token());
    let section_gap = theme.resolve_space(spec.section_gap_token());
    let header_gap = theme.resolve_space(spec.header_gap_token());
    let section_inner_gap = theme.resolve_space(spec.section_internal_gap_token());

    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");

    let title_size = theme.resolve_space(spec.title_size_token());
    let desc_size = theme.resolve_space(spec.description_size_token());
    let section_title_size = theme.resolve_space(spec.section_title_size_token());

    let column = |gap: f32| -> Node {
        let mut n = Node::container();
        n.style.descriptor.layout.direction = LayoutDirection::Column;
        n.style.descriptor.layout.spacing.gap = gap;
        n
    };
    let text = |content: &str, color: poodle_node::ColorValue, size: f32, weight: Option<u16>| {
        let mut t = Node::text(content);
        t.style.descriptor.text_color = Some(color);
        t.style.text_size = Some(size);
        t.style.text_weight = weight;
        t
    };

    let mut el = column(stack_gap);

    // ── Header: Title + Description ───────────────────────────
    if spec.title.is_some() || spec.description.is_some() {
        let mut header = column(header_gap);
        if let Some(ref title) = spec.title {
            header = header.child(text(title, text_primary, title_size, Some(SEMIBOLD)));
        }
        if let Some(ref desc) = spec.description {
            header = header.child(text(desc, text_secondary, desc_size, None));
        }
        el = el.child(header);
    }

    // ── StatusSummary: a Callout driven by resolved_status_tone() ─
    let resolved_tone = spec.resolved_status_tone();
    if let Some(message) = status_message(spec, resolved_tone) {
        el = el.child(callout(
            &CallOutSpec::new()
                .with_tone(resolved_tone)
                .with_content(message),
            theme,
            CalloutHandlers::default(),
        ));
    }

    // ── Sections ─────────────────────────────────────────────
    if !spec.sections.is_empty() {
        let mut sections_container = column(section_gap);

        for (idx, section_spec) in spec.sections.iter().enumerate() {
            let mut section = column(section_inner_gap);

            // SectionTitle
            section = section.child(text(
                &section_spec.title,
                text_primary,
                section_title_size,
                Some(SEMIBOLD),
            ));

            // SectionDescription (optional)
            if let Some(ref desc) = section_spec.description {
                section = section.child(text(desc, text_secondary, desc_size, None));
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
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = match spec.actions.align {
                FormActionAlign::Start => MainAxisAlignment::Start,
                FormActionAlign::End => MainAxisAlignment::End,
                FormActionAlign::Between => MainAxisAlignment::SpaceBetween,
            };
            // Dim the actions when submission is blocked (busy / disabled /
            // invalid); the host wires the actual disabled buttons.
            if spec.blocks_submission() {
                s.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
            }
        }
        el = el.child(row.child(actions));
    }

    // ── Disabled: dim the whole shell (contract §4) ──────────
    if spec.is_disabled {
        el.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
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
