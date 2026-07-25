//! Meter — real GPUI component backed by MeterSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, MeterShape, MeterSpec, MeterTone, SemanticControlSizeRole};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

/// A real GPUI meter component backed by `MeterSpec`.
pub struct Meter {
    spec: MeterSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Meter {
    type Target = MeterSpec;
    fn deref(&self) -> &MeterSpec {
        &self.spec
    }
}

impl Meter {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: MeterSpec::new(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: MeterSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self {
        self.spec.value = v;
        self
    }
    pub fn min(mut self, v: f64) -> Self {
        self.spec.min = v;
        self
    }
    pub fn max(mut self, v: f64) -> Self {
        self.spec.max = v;
        self
    }
    pub fn low(mut self, v: f64) -> Self {
        self.spec.low = Some(v);
        self
    }
    pub fn high(mut self, v: f64) -> Self {
        self.spec.high = Some(v);
        self
    }
    pub fn optimum(mut self, v: f64) -> Self {
        self.spec.optimum = Some(v);
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = Some(v);
        self
    }
    pub fn size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn shape(mut self, v: MeterShape) -> Self {
        self.spec.shape = v;
        self
    }
    pub fn tone(mut self, v: MeterTone) -> Self {
        self.spec.tone = v;
        self
    }
    pub fn show_value(mut self, v: bool) -> Self {
        self.spec.show_value = v;
        self
    }
    pub fn value_text(mut self, v: impl Into<String>) -> Self {
        self.spec.value_text = Some(v.into());
        self
    }
}

impl IntoElement for Meter {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Contract §8: fill base resolves from `tone`, escalated to warning by
        // the `high` threshold — the spec owns that decision.
        let success_color = resolve_color(theme, spec.fill_token());
        let surface_bg = resolve_color(theme, spec.track_fill_token());
        let text_primary = resolve_color(theme, spec.track_mix_token());
        let radius = resolve_radius(theme, "radius.pill");
        let progress = spec.normalized_progress();

        // Contract §8: track bg = color-mix(surface 96%, text-primary)
        let track_fill = color_mix(surface_bg, text_primary, spec.track_mix_ratio());
        // Status-success fill. Contract §8 specifies a 90deg gradient
        // (mix(success 82%, white) → success); GPUI has no gradient, so a flat
        // success fill is the accepted Tier-2 delta.
        let fill = success_color;

        // Contract §8 Size Variants: thickness/diameter resolve from the
        // effective size (size override → size_role against the inherited scale).
        let effective_size = resolve_semantic_size(
            spec.size.unwrap_or(ControlSize::Md),
            spec.size_role,
        );
        let track_height = px(rem_to_px(spec.track_thickness_rem(effective_size)));

        let fill_width_pct = (progress * 100.0) as f32;

        // Ring shape. Contract §12 accepted delta: GPUI has no conic gradient or
        // arc primitive, so the ring renders as a circular track stroked in the
        // level-resolved fill colour, with the value readout carrying the
        // proportion.
        if spec.shape == MeterShape::Ring {
            let diameter = px(rem_to_px(spec.ring_size_rem(effective_size)));
            let thickness = px(rem_to_px(spec.ring_thickness_rem(effective_size)));
            let ring_track = color_mix(surface_bg, text_primary, spec.ring_track_mix_ratio());
            let mut ring = div()
                .w(diameter)
                .h(diameter)
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .rounded_full()
                .border(thickness)
                .border_color(fill)
                .bg(ring_track);
            if spec.show_value {
                ring = ring.child(
                    div()
                        .text_color(resolve_color(theme, spec.value_color_token()))
                        .text_size(diameter * 0.34)
                        .child(spec.value_display_text()),
                );
            }
            return ring.into_any_element();
        }

        // Accessibility: meter semantics — bounded-value measurement display
        // GPUI equivalent of role="meter", aria-valuenow, aria-valuemin, aria-valuemax
        // Native accessibility should expose: value={spec.value}, min={spec.min},
        // max={spec.max}, low/high/optimum when set, aria-label from spec.aria_label
        div()
            .w_full()
            .h(track_height)
            .rounded(radius)
            .bg(track_fill)
            .overflow_hidden()
            .child(
                div()
                    .h_full()
                    .rounded(radius)
                    .bg(fill)
                    .w(relative(fill_width_pct / 100.0)),
            )
            .into_any_element()
    }
}
