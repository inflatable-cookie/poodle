//! StateTile — real GPUI component backed by StateTileSpec.
//!
//! Contract: `docs/contracts/components/state-tile.md`
//! Reference: Svelte component NOT YET BUILT — the contract is the sole
//! authority (see `docs/parity/state-tile.md`). Jetstream mirrors the same
//! contract.
//!
//! A lightweight label-value tile: caption (`Label`), primary value (`Value`,
//! typography-heading), an optional trend row (decorative directional glyph +
//! trend label text), and an optional reserved sparkline slot the host fills.
//!
//! GPUI emits no ARIA (contract §7 keeps the root accessibility-neutral; GPUI
//! has no accessibility surface).

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::StateTileSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI state tile backed by `StateTileSpec`.
pub struct StateTile {
    spec: StateTileSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for StateTile {
    type Target = StateTileSpec;
    fn deref(&self) -> &StateTileSpec {
        &self.spec
    }
}

impl StateTile {
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        theme: &GpuiThemeProvider,
    ) -> Self {
        Self::from_spec(StateTileSpec::new(label, value), theme)
    }

    pub fn from_spec(spec: StateTileSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = v.into();
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = v.into();
        self
    }
    pub fn trend(mut self, v: impl Into<String>) -> Self {
        self.spec.trend = Some(v.into());
        self
    }
    pub fn trend_label(mut self, v: impl Into<String>) -> Self {
        self.spec.trend_label = Some(v.into());
        self
    }
    pub fn has_sparkline(mut self, v: bool) -> Self {
        self.spec.has_sparkline = v;
        self
    }
}

impl IntoElement for StateTile {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Colors (token-resolved via spec) ──
        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let label_color = resolve_color(theme, spec.label_color_token());
        let value_color = resolve_color(theme, spec.value_color_token());
        let trend_color = resolve_color(theme, spec.trend_color_token());

        // ── Dimensions ──
        let radius = resolve_radius(theme, spec.radius_token());
        let border_width = resolve_px(theme, spec.border_width_token());
        let pad_x = resolve_px(theme, "space.panel.x");
        let pad_y = resolve_px(theme, "space.panel.y");
        let gap = resolve_px(theme, "space.stack.sm");
        let trend_gap = resolve_px(theme, "space.inline.xs");
        let label_size = resolve_px(theme, spec.label_font_size_token());
        let value_size = resolve_px(theme, spec.value_font_size_token());
        let trend_size = resolve_px(theme, spec.trend_font_size_token());

        let mut tile = div()
            .flex()
            .flex_col()
            .gap(gap)
            .bg(fill)
            .rounded(radius)
            .border(border_width)
            .border_color(border)
            .px(pad_x)
            .py(pad_y)
            // Label
            .child(
                div()
                    .text_size(label_size)
                    .text_color(label_color)
                    .child(spec.label.clone()),
            )
            // Value
            .child(
                div()
                    .text_size(value_size)
                    .font_weight(FontWeight::BOLD)
                    .text_color(value_color)
                    .child(spec.value.clone()),
            );

        // ── Trend row (optional) — contract §7 keeps meaning in the label
        // text; the glyph is decorative. ──
        if spec.trend.is_some() || spec.trend_label.is_some() {
            let mut trend_row = div()
                .flex()
                .items_center()
                .gap(trend_gap)
                .text_size(trend_size)
                .text_color(trend_color);

            if spec.trend.is_some() {
                trend_row = trend_row.child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(spec.trend_glyph().to_string()),
                );
            }

            if let Some(ref trend_label) = spec.trend_label {
                trend_row = trend_row.child(div().child(trend_label.clone()));
            }

            tile = tile.child(trend_row);
        }

        // ── Sparkline slot (optional) — contract §1/§2/§3: host owns the chart;
        // the tile only reserves the slot (no synthetic data). ──
        if spec.has_sparkline {
            let slot_h = px(rem_to_px(2.0)); // note: contract slot height, no token
            let slot_bg = resolve_color(theme, spec.sparkline_slot_token());

            tile = tile.child(
                div()
                    .id("state-tile-sparkline")
                    .w_full()
                    .h(slot_h)
                    .rounded(radius)
                    .bg(slot_bg),
            );
        }

        tile.into_any_element()
    }
}
