//! ValidationSummary — real GPUI composite backed by ValidationSummarySpec.
//!
//! Contract: `docs/contracts/components/validation-summary.md`
//! Reference: no standalone Svelte component (contract self-declares "not yet
//! built"); the contract is the sole authority. Anatomy is built directly from
//! the contract §2 table.
//!
//! Anatomy: Root surface (border = danger when blocking, accent when
//! pending-only — `spec.border_token()`) → optional Title → List of Entry rows
//! (each: tone indicator + text column with field Label + Message). All
//! spacing/sizing/radius/typography resolve from `ValidationSummarySpec` token
//! methods.
//!
//! Accessibility (contract §5/§8 known deltas): GPUI 0.2 has no ARIA surface,
//! so `role`/`aria-live` from `announce_mode` are not emitted. The web-only
//! `<a href="#field-id">` focus-jump is not wired here — entries are static
//! (host can emulate focus imperatively in a later pass).

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::ValidationSummarySpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI validation-summary component backed by `ValidationSummarySpec`.
pub struct ValidationSummary {
    spec: ValidationSummarySpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for ValidationSummary {
    type Target = ValidationSummarySpec;
    fn deref(&self) -> &ValidationSummarySpec {
        &self.spec
    }
}

impl ValidationSummary {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ValidationSummarySpec::default(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: ValidationSummarySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = Some(v.into());
        self
    }
    pub fn include_pending(mut self, v: bool) -> Self {
        self.spec.include_pending = v;
        self
    }
}

impl IntoElement for ValidationSummary {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let entries = spec.active_entries();

        // Empty state (contract §4) — render nothing substantive.
        if entries.is_empty() {
            return div().into_any_element();
        }

        let border = resolve_color(theme, spec.border_token());
        let fill = resolve_color(theme, spec.fill_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let danger = resolve_color(theme, "color.status.danger");
        let accent = resolve_color(theme, "color.accent.base");

        let title_size = resolve_px(theme, spec.title_size_token());
        let entry_size = resolve_px(theme, spec.entry_text_size_token());
        let pad_x = resolve_px(theme, spec.padding_x_token());
        let pad_y = resolve_px(theme, spec.padding_y_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let list_gap = resolve_px(theme, spec.list_gap_token());
        let entry_gap = resolve_px(theme, spec.entry_gap_token());
        let entry_text_gap = resolve_px(theme, spec.entry_text_gap_token());

        // Tone indicator dot — half the entry text size (enrichment, not a
        // contract-anatomy part; see module note).
        let dot_size = entry_size * 0.5;

        let mut surface = div()
            .flex()
            .flex_col()
            .gap(list_gap)
            .px(pad_x)
            .py(pad_y)
            .rounded(radius)
            .bg(fill)
            .border_1()
            .border_color(border);

        // Title (optional)
        if let Some(ref title) = spec.title {
            surface = surface.child(
                div()
                    .text_size(title_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // List → Entry rows
        let mut list = div().flex().flex_col().gap(list_gap);
        for entry in &entries {
            let tone = if entry.is_blocking() { danger } else { accent };

            let row = div()
                .flex()
                .flex_row()
                .items_start()
                .gap(entry_gap)
                // Leading tone indicator dot.
                .child(
                    div()
                        .w(dot_size)
                        .h(dot_size)
                        .rounded(dot_size * 0.5)
                        .bg(tone)
                        .flex_shrink_0(),
                )
                // Text column: Label + Message.
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(entry_text_gap)
                        .child(
                            div()
                                .text_size(entry_size)
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(text_primary)
                                .child(entry.label.clone()),
                        )
                        .child(
                            div()
                                .text_size(entry_size)
                                .text_color(text_secondary)
                                .child(entry.message.clone()),
                        ),
                );

            list = list.child(row);
        }
        surface = surface.child(list);

        surface.into_any_element()
    }
}
