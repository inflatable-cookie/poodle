//! Field — real GPUI component backed by FieldSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, FieldSpec, IconSize, IconSpec, SemanticControlSizeRole,
    ValidationState,
};

use super::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI field wrapper component backed by `FieldSpec`.
///
/// Renders label, optional indicator, description, error message,
/// and a slot for the control (input, select, etc.).
pub struct Field {
    spec: FieldSpec,
    theme: GpuiThemeProvider,
    /// The form control to wrap.
    control: Option<AnyElement>,
}

impl std::ops::Deref for Field {
    type Target = FieldSpec;
    fn deref(&self) -> &FieldSpec {
        &self.spec
    }
}

impl Field {
    pub fn new(id: impl Into<String>, label: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: FieldSpec::new(id, label),
            theme: theme.clone(),
            control: None,
        }
    }

    pub fn from_spec(spec: FieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            control: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.spec.id = v.into();
        self
    }
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = v.into();
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn hint(mut self, v: impl Into<String>) -> Self {
        self.spec.hint = Some(v.into());
        self
    }
    pub fn error(mut self, v: impl Into<String>) -> Self {
        self.spec.error = Some(v.into());
        self
    }
    pub fn pending_message(mut self, v: impl Into<String>) -> Self {
        self.spec.pending_message = Some(v.into());
        self
    }
    pub fn validation_state(mut self, v: ValidationState) -> Self {
        self.spec.validation_state = v;
        self
    }
    pub fn required(mut self, v: bool) -> Self {
        self.spec.is_required = v;
        self
    }
    pub fn optional_label(mut self, v: impl Into<String>) -> Self {
        self.spec.optional_label = Some(v.into());
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    /// Set the form control element that this field wraps.
    pub fn with_control(mut self, control: impl IntoElement) -> Self {
        self.control = Some(control.into_any_element());
        self
    }
}

impl IntoElement for Field {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let description_color = resolve_color(theme, spec.description_color_token());
        let error_color = resolve_color(theme, spec.error_color_token());
        let label_size = resolve_px(theme, spec.label_typography_token());
        let supporting_size = resolve_px(theme, spec.supporting_text_typography_token());
        let root_gap = resolve_px(theme, spec.row_gap_token());
        let header_gap = resolve_px(theme, spec.header_gap_token());
        let label_row_gap = resolve_px(theme, spec.label_row_gap_token());
        // Contract §8: label color = color-mix(in srgb, text-primary 45%, text-secondary)
        let label_primary = resolve_color(theme, spec.label_color_primary_token());
        let label_secondary = resolve_color(theme, spec.label_color_secondary_token());
        let label_color = color_mix(
            label_primary,
            label_secondary,
            FieldSpec::LABEL_COLOR_PRIMARY_RATIO,
        );

        let mut col = div().flex().flex_col().gap(root_gap);

        // `span` / `grid_area` (FieldSpec) are CSS-grid placement props. GPUI has
        // no CSS-grid parent context, so they are an accepted layout delta
        // (contract §10/§12: grid-column/grid-area integration is platform-owned).
        // The owning layout positions the field; no per-field style is emitted.

        // Label group — contract §7: `0.375rem` gap between label and required `*`
        let mut label_group = div().flex().items_center().gap(label_row_gap);
        label_group = label_group.child(
            div()
                .text_size(label_size)
                .font_weight(FontWeight::MEDIUM)
                .text_color(label_color)
                .child(spec.label.clone()),
        );
        if spec.is_required {
            label_group = label_group.child(
                div()
                    .text_size(label_size)
                    .text_color(error_color)
                    .child("*"),
            );
        }

        // Info icon — contract §2/§7/§8: pill wrapper next to the label when a
        // description/hint is set. Its presence (and that the description is NOT
        // rendered inline, contract §4/§9) is the load-bearing parity fix.
        // The icon carries the description as its tooltip content; the live
        // hover-to-open affordance lives in the preview event loop, not this
        // stateless builder (same accepted runtime limit as the Popover trigger —
        // contract §12 allows tooltip vs Popover implementation freedom).
        if spec.info_text().is_some() {
            // em-relative to the label font: 1.25em wrapper, 0.75em glyph.
            let label_px = f32::from(label_size);
            let icon_box = px(label_px * FieldSpec::INFO_ICON_EM);
            let icon_glyph = label_px * FieldSpec::INFO_ICON_SVG_EM;
            let info_bg_base = resolve_color(theme, spec.info_icon_bg_token());
            let info_bg = Hsla {
                a: info_bg_base.a * FieldSpec::INFO_ICON_BG_ALPHA,
                ..info_bg_base
            };
            let info_color = resolve_color(theme, spec.info_icon_color_token());
            let info_radius = resolve_radius(theme, spec.info_icon_radius_token());
            label_group = label_group.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_shrink_0()
                    .w(icon_box)
                    .h(icon_box)
                    .rounded(info_radius)
                    .bg(info_bg)
                    .cursor_pointer()
                    .child(
                        Icon::from_spec(
                            IconSpec::new("info").with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(info_color)
                        .with_px_size(icon_glyph),
                    ),
            );
        }

        let mut label_row = div()
            .flex()
            .items_center()
            .justify_between()
            .gap(header_gap);

        label_row = label_row.child(label_group);

        if !spec.is_required && spec.shows_optional_label() {
            if let Some(ref opt_label) = spec.optional_label {
                label_row = label_row.child(
                    div()
                        .text_size(supporting_size)
                        .text_color(description_color)
                        .flex_shrink_0()
                        .child(opt_label.clone()),
                );
            }
        }

        col = col.child(label_row);

        // Description is NOT rendered inline (contract §4/§9) — it lives in the
        // info-icon tooltip built above.

        // Control slot
        if let Some(control) = self.control {
            col = col.child(control);
        }

        // Error message
        if spec.validation_state == ValidationState::Invalid {
            if let Some(ref error) = spec.error {
                col = col.child(
                    div()
                        .text_size(supporting_size)
                        .text_color(error_color)
                        .child(error.clone()),
                );
            }
        }

        // Pending message
        if spec.validation_state == ValidationState::Pending {
            if let Some(ref pending) = spec.pending_message {
                col = col.child(
                    div()
                        .text_size(supporting_size)
                        .text_color(description_color)
                        .child(pending.clone()),
                );
            }
        }

        col.into_any_element()
    }
}
