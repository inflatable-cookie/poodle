//! PasswordRequirements — real GPUI component backed by PasswordRequirementsSpec.
//!
//! Renders a live-feedback checklist showing password policy satisfaction
//! with pass/fail indicators, title, optional hint, and error message.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{PasswordRequirementsPolicy, PasswordRequirementsSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI password requirements checklist backed by `PasswordRequirementsSpec`.
pub struct PasswordRequirements {
    spec: PasswordRequirementsSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for PasswordRequirements {
    type Target = PasswordRequirementsSpec;
    fn deref(&self) -> &PasswordRequirementsSpec {
        &self.spec
    }
}

impl PasswordRequirements {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: PasswordRequirementsSpec::new(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: PasswordRequirementsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn password(mut self, v: impl Into<String>) -> Self {
        self.spec.password = v.into();
        self
    }
    pub fn requirements(mut self, v: PasswordRequirementsPolicy) -> Self {
        self.spec.requirements = Some(v);
        self
    }
    pub fn loading(mut self, v: bool) -> Self {
        self.spec.is_loading = v;
        self
    }
    pub fn error(mut self, v: impl Into<String>) -> Self {
        self.spec.error = Some(v.into());
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = v.into();
        self
    }
    pub fn hint(mut self, v: impl Into<String>) -> Self {
        self.spec.hint = Some(v.into());
        self
    }
    pub fn loading_label(mut self, v: impl Into<String>) -> Self {
        self.spec.loading_label = v.into();
        self
    }
}

impl IntoElement for PasswordRequirements {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Token resolution ──────────────────────────────────────
        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let title_color = resolve_color(theme, spec.title_color_token());
        let text_color = resolve_color(theme, spec.text_color_token());
        let met_color = resolve_color(theme, spec.met_color_token());
        let error_color = resolve_color(theme, spec.error_color_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let border_width = resolve_px(theme, spec.border_width_token());

        // ── Size ladder (contract §7) ─────────────────────────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let body_font = px(rem_to_px(PasswordRequirementsSpec::body_size_rem(
            effective_size,
        )));
        let title_font = px(rem_to_px(PasswordRequirementsSpec::title_size_rem(
            effective_size,
        )));
        let line_height = relative(1.5);
        let padding = px(rem_to_px(PasswordRequirementsSpec::padding_rem(
            effective_size,
        )));
        let title_mb = px(rem_to_px(PasswordRequirementsSpec::hint_gap_rem(
            effective_size,
        )));
        let description_gap = px(rem_to_px(PasswordRequirementsSpec::description_gap_rem(
            effective_size,
        )));
        let hint_gap = px(rem_to_px(PasswordRequirementsSpec::hint_gap_rem(
            effective_size,
        )));

        // ── Outer container ───────────────────────────────────────
        let mut container = div()
            .bg(fill)
            .border(border_width)
            .border_color(border)
            .rounded(radius)
            .p(padding)
            .flex()
            .flex_col();

        // ── Loading state ─────────────────────────────────────────
        if spec.is_loading {
            container = container.child(
                div()
                    .text_size(body_font)
                    .line_height(line_height)
                    .text_color(text_color)
                    .child(spec.loading_label.clone()),
            );
            return container.into_any_element();
        }

        // ── Requirements checklist ────────────────────────────────
        if let Some(ref policy) = spec.requirements {
            // Title
            container = container.child(
                div()
                    .text_size(title_font)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(title_color)
                    .mb(title_mb) // Svelte: margin-bottom (0.5rem at md, size-scaled)
                    .child(format!("{}:", spec.title)),
            );

            // Build requirement rules list
            let mut list = div().flex().flex_col().gap(px(rem_to_px(0.125)));

            // Min length requirement (always present)
            let length_met = spec.length_met();
            list = list.child(build_rule_row(
                &format!("At least {} characters", policy.min_length),
                length_met,
                met_color,
                text_color,
                body_font,
                line_height,
            ));

            // Mixed case
            if policy.require_mixed_case {
                let met = spec.mixed_case_met();
                list = list.child(build_rule_row(
                    "Mix of uppercase and lowercase letters",
                    met,
                    met_color,
                    text_color,
                    body_font,
                    line_height,
                ));
            }

            // Digit
            if policy.require_digit {
                let met = spec.digit_met();
                list = list.child(build_rule_row(
                    "At least one number",
                    met,
                    met_color,
                    text_color,
                    body_font,
                    line_height,
                ));
            }

            // Special character
            if policy.require_special {
                let met = spec.special_met();
                list = list.child(build_rule_row(
                    "At least one special character",
                    met,
                    met_color,
                    text_color,
                    body_font,
                    line_height,
                ));
            }

            container = container.child(list);

            // Description
            if let Some(ref description) = policy.description {
                container = container.child(
                    div()
                        .mt(description_gap)
                        .text_size(body_font)
                        .line_height(line_height)
                        .text_color(text_color)
                        .child(description.clone()),
                );
            }

            // Hint
            if let Some(ref hint) = spec.hint {
                container = container.child(
                    div()
                        .mt(hint_gap)
                        .text_size(body_font)
                        .line_height(line_height)
                        .text_color(text_color)
                        .child(hint.clone()),
                );
            }
        } else if let Some(ref error) = spec.error {
            // Error-only state (no requirements loaded)
            container = container.child(
                div()
                    .text_size(body_font)
                    .line_height(line_height)
                    .text_color(error_color)
                    .child(error.clone()),
            );
        }

        container.into_any_element()
    }
}

/// Build a single requirement rule row with a pass/fail indicator.
fn build_rule_row(
    label: &str,
    is_met: bool,
    met_color: Hsla,
    unmet_color: Hsla,
    font_size: Pixels,
    line_height: DefiniteLength,
) -> Div {
    // Contract §6: non-color indicator must supplement the color change —
    // checkmark for met, cross for not-met (mirrors Jetstream `check`/`x`).
    let indicator = if is_met { "\u{2713}" } else { "\u{2717}" }; // ✓ / ✗
    let color = if is_met { met_color } else { unmet_color };

    div()
        .flex()
        .flex_row()
        .items_baseline()
        .gap(px(rem_to_px(0.375)))
        .text_color(color)
        .text_size(font_size)
        .line_height(line_height)
        .child(
            div()
                .w(px(rem_to_px(0.875)))
                .text_size(px(rem_to_px(0.75)))
                .flex_shrink_0()
                .child(indicator),
        )
        .child(label.to_string())
}
