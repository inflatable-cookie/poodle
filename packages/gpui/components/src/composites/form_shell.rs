//! FormShell — real GPUI composite backed by FormShellSpec.
//!
//! Contract: `docs/contracts/components/form-shell.md`
//! Reference: no standalone Svelte component (Rust-only shared spec; the
//! contract is the sole authority — see the parity doc). Anatomy is built
//! directly from the contract §2 table.
//!
//! Anatomy: Root stack → optional header (Title `<h2>` + Description `<p>`) →
//! optional StatusSummary `Callout` → Sections (each: SectionTitle `<h3>` +
//! optional SectionDescription + host-supplied field slots) → Actions row
//! (`FormActions`, honoring `actions.align`). All spacing/typography/opacity
//! resolve from `FormShellSpec` token methods.
//!
//! Accessibility (contract §5/§8 known delta): GPUI 0.2 has no ARIA surface, so
//! `aria-labelledby` / per-field `aria-describedby` are not emitted; the visual
//! structure still renders. Submit/interaction is host-driven (contract §3) —
//! the shell exposes gating via `spec.blocks_submission()` and renders no submit
//! handler; the actions row is dimmed when submission is blocked.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CallOutSpec, FormShellSpec, StatusTone};

use crate::primitives::{Callout, FormActions};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

/// A real GPUI form-shell composite backed by `FormShellSpec`.
///
/// `section_slots[i]` is the host-rendered field content for `spec.sections[i]`
/// (the contract delegates field rendering to the host, referenced via
/// `field_ids`). Action elements are added via `with_action` and rendered in the
/// footer `FormActions` row honoring `actions.align`.
pub struct FormShell {
    spec: FormShellSpec,
    theme: GpuiThemeProvider,
    section_slots: Vec<Option<AnyElement>>,
    actions: Vec<AnyElement>,
}

impl std::ops::Deref for FormShell {
    type Target = FormShellSpec;
    fn deref(&self) -> &FormShellSpec {
        &self.spec
    }
}

impl FormShell {
    pub fn new(theme: &GpuiThemeProvider, id: impl Into<String>) -> Self {
        Self {
            spec: FormShellSpec::new(id),
            theme: theme.clone(),
            section_slots: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn from_spec(spec: FormShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            section_slots: Vec::new(),
            actions: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = Some(v.into());
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn busy(mut self, v: bool) -> Self {
        self.spec.is_busy = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }

    /// Provide the host-rendered field slot for the next section in order.
    pub fn with_section_slot(mut self, slot: impl IntoElement) -> Self {
        self.section_slots.push(Some(slot.into_any_element()));
        self
    }

    /// Append an empty (no-content) section slot, keeping section index alignment.
    pub fn with_empty_section_slot(mut self) -> Self {
        self.section_slots.push(None);
        self
    }

    /// Add a footer action element (typically a Button).
    pub fn with_action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
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

impl IntoElement for FormShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let stack_gap = resolve_px(theme, spec.stack_gap_token());
        let section_gap = resolve_px(theme, spec.section_gap_token());
        let header_gap = resolve_px(theme, spec.header_gap_token());
        let section_inner_gap = resolve_px(theme, spec.section_internal_gap_token());

        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");

        let title_size = resolve_px(theme, spec.title_size_token());
        let desc_size = resolve_px(theme, spec.description_size_token());
        let section_title_size = resolve_px(theme, spec.section_title_size_token());

        let mut root = div().flex().flex_col().gap(stack_gap).w_full();

        // ── Header: Title (<h2>) + Description (<p>) ──────────
        if spec.title.is_some() || spec.description.is_some() {
            let mut header = div().flex().flex_col().gap(header_gap);
            if let Some(ref title) = spec.title {
                header = header.child(
                    div()
                        .text_size(title_size)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child(title.clone()),
                );
            }
            if let Some(ref desc) = spec.description {
                header = header.child(
                    div()
                        .text_size(desc_size)
                        .text_color(text_secondary)
                        .child(desc.clone()),
                );
            }
            root = root.child(header);
        }

        // ── StatusSummary: a Callout driven by resolved_status_tone() ─
        let resolved_tone = spec.resolved_status_tone();
        if let Some(message) = status_message(spec, resolved_tone) {
            root = root.child(Callout::from_spec(
                CallOutSpec::new()
                    .with_tone(resolved_tone)
                    .with_content(message),
                theme,
            ));
        }

        // ── Sections ─────────────────────────────────────────
        // Each: SectionTitle (<h3>) + optional SectionDescription + the
        // host-supplied field slot (referenced via field_ids). Slots are
        // `AnyElement` (not Clone), so the container is built in a helper that
        // takes ownership of them in order.
        if !spec.sections.is_empty() {
            root = root.child(render_sections(
                spec,
                self.section_slots,
                section_gap,
                section_inner_gap,
                desc_size,
                section_title_size,
                text_primary,
                text_secondary,
            ));
        }

        // ── Actions row (FormActions, honoring align) ────────
        if !self.actions.is_empty() {
            let mut actions = FormActions::new(theme).align(spec.actions.align);
            for action in self.actions {
                actions = actions.with_action(action);
            }
            let mut actions_el = div().w_full().child(actions);
            // Dim the actions when submission is blocked.
            if spec.blocks_submission() {
                actions_el =
                    actions_el.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
            }
            root = root.child(actions_el);
        }

        // ── Disabled: dim the whole shell (contract §4) ──────
        if spec.is_disabled {
            root = root.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
        }

        root.into_any_element()
    }
}

/// Build the sections container taking ownership of the host field slots in
/// order (slots are `AnyElement` and cannot be cloned).
#[allow(clippy::too_many_arguments)]
fn render_sections(
    spec: &FormShellSpec,
    mut section_slots: Vec<Option<AnyElement>>,
    section_gap: Pixels,
    section_inner_gap: Pixels,
    desc_size: Pixels,
    section_title_size: Pixels,
    text_primary: Hsla,
    text_secondary: Hsla,
) -> impl IntoElement {
    let mut container = div().flex().flex_col().gap(section_gap);
    for (idx, section_spec) in spec.sections.iter().enumerate() {
        let mut section = div().flex().flex_col().gap(section_inner_gap);

        section = section.child(
            div()
                .text_size(section_title_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(section_spec.title.clone()),
        );

        if let Some(ref desc) = section_spec.description {
            section = section.child(
                div()
                    .text_size(desc_size)
                    .text_color(text_secondary)
                    .child(desc.clone()),
            );
        }

        if let Some(slot) = section_slots.get_mut(idx).and_then(Option::take) {
            section = section.child(slot);
        }

        container = container.child(section);
    }
    container
}
