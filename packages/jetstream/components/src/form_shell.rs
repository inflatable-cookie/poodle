//! FormShell — Jetstream form container with sections and submission backed by FormShellSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::FormShellSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_form_shell(
    spec: &FormShellSpec,
    theme: &JetstreamThemeProvider,
    section_slots: Vec<Option<JsEl>>,
    actions_slot: Option<JsEl>,
) -> JsEl {
    let stack_gap = resolve_px(theme, spec.stack_gap_token());
    let section_gap = resolve_px(theme, spec.section_gap_token());
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let danger_color = resolve_color(theme, "semantic.color.status.danger");
    let pending_color = resolve_color(theme, "semantic.color.accent.base");

    let title_size = rem_to_px(1.125);
    let desc_size = rem_to_px(0.8125);
    let status_size = rem_to_px(0.75);

    let mut el = ui_element::div()
        .flex_col().gap(stack_gap);

    // Title and description
    if spec.title.is_some() || spec.description.is_some() {
        let mut header = ui_element::div().flex_col().gap(rem_to_px(0.25));

        if let Some(ref title) = spec.title {
            header = header.child(
                ui_element::label(title)
                    .text_color(text_primary).text_size(title_size).text_weight(600)
            );
        }

        if let Some(ref desc) = spec.description {
            header = header.child(
                ui_element::label(desc)
                    .text_color(text_secondary).text_size(desc_size)
            );
        }

        el = el.child(header);
    }

    // Status summary (validation counts)
    let invalid = spec.invalid_field_count();
    let pending = spec.pending_field_count();

    if invalid > 0 || pending > 0 {
        let mut status = ui_element::div().flex_row().items_center().gap(rem_to_px(0.5));

        if invalid > 0 {
            let msg = format!("{} invalid field{}", invalid, if invalid == 1 { "" } else { "s" });
            status = status.child(
                ui_element::label(&msg).text_color(danger_color).text_size(status_size)
            );
        }

        if pending > 0 {
            let msg = format!("{} pending", pending);
            status = status.child(
                ui_element::label(&msg).text_color(pending_color).text_size(status_size)
            );
        }

        el = el.child(status);
    }

    // Sections
    let mut sections_container = ui_element::div().flex_col().gap(section_gap);

    for (idx, section_spec) in spec.sections.iter().enumerate() {
        let mut section = ui_element::div().flex_col().gap(rem_to_px(0.75));

        section = section.child(
            ui_element::label(&section_spec.title)
                .text_color(text_primary).text_size(rem_to_px(0.875)).text_weight(600)
        );

        if let Some(slot) = section_slots.get(idx).and_then(|s| s.as_ref()) {
            // Slot content would be injected here; for layout we clone the structure
            let _ = slot;
        }

        sections_container = sections_container.child(section);
    }

    el = el.child(sections_container);

    // Actions slot
    if let Some(actions) = actions_slot {
        el = el.child(actions);
    }

    if spec.is_disabled {
        el = el.opacity(0.5);
    }

    el
}
