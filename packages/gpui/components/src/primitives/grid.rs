//! Grid — real GPUI component backed by GridSpec.
//!
//! Note: gpui doesn't have native CSS grid support, so we approximate
//! using flex-wrap with equal-width children.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{Dimension, GridColumns, GridSpec, GridTrack, PaddingScale};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_px;

/// A grid layout approximated with flex-wrap.
pub struct Grid {
    spec: GridSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for Grid {
    type Target = GridSpec;
    fn deref(&self) -> &GridSpec {
        &self.spec
    }
}

impl Grid {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: GridSpec::new(),
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn from_spec(spec: GridSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn columns(mut self, v: Dimension) -> Self {
        self.spec.columns = v;
        self
    }
    pub fn rows(mut self, v: Dimension) -> Self {
        self.spec.rows = Some(v);
        self
    }
    pub fn gap(mut self, v: PaddingScale) -> Self {
        self.spec.gap = v;
        self
    }
    pub fn padding(mut self, v: PaddingScale) -> Self {
        self.spec.padding = v;
        self
    }
    pub fn role(mut self, v: impl Into<String>) -> Self {
        self.spec.role = Some(v.into());
        self
    }

    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for Grid {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        // GPUI/Taffy has no CSS grid, so the grid is approximated with
        // `flex().flex_wrap()`. Two-axis CSS `gap` maps to `gap_x`/`gap_y`
        // (same token, per contract §8). Column tracks drive per-child flex:
        //   - `Fr(w)` tracks → each child grows by its weight (`1fr 2fr` →
        //     1/3 + 2/3 split); cycles when there are more children than tracks.
        //   - `AutoFit { min_rem }` → each child has a min-width of `min_rem`
        //     and grows to fill the row, wrapping like `repeat(auto-fit, …)`.
        // DELTA vs CSS grid: explicit `rows` tracks are not honored (Taffy has
        // no row-track concept); rows emerge from flex-wrap. Fixed `Rem` tracks
        // size to their rem width but do not pin an exact column count.
        let theme = &self.theme;
        let spec = &self.spec;
        let padding = spec.resolved_padding();
        let columns = spec.parsed_columns();

        let mut el = div().flex().flex_wrap();

        // Column gap
        if let Some(gap_token) = spec.resolved_column_gap() {
            el = el.gap_x(resolve_px(theme, gap_token));
        }
        // Row gap — same token as column gap (CSS `gap` is one value).
        if let Some(row_gap_token) = spec.resolved_row_gap() {
            el = el.gap_y(resolve_px(theme, row_gap_token));
        }

        // Padding (uniform on both axes, contract §8 `padding: <value>`).
        if let Some(h) = padding.horizontal {
            el = el.px(resolve_px(theme, h));
        }
        if let Some(v) = padding.vertical {
            el = el.py(resolve_px(theme, v));
        }

        // Total fr-weight across the track list, used to turn a track's weight
        // into a relative flex-basis fraction (`1fr 2fr` → 1/3, 2/3). Rem tracks
        // contribute no fr-weight (they take a fixed width instead).
        let fr_total: f32 = match &columns {
            GridColumns::Tracks(tracks) => tracks
                .iter()
                .filter_map(|t| match t {
                    GridTrack::Fr(w) => Some(*w),
                    GridTrack::Rem(_) => None,
                })
                .sum::<f32>()
                .max(1.0),
            GridColumns::AutoFit { .. } => 1.0,
        };

        for (i, child) in self.children.into_iter().enumerate() {
            let wrapper = match &columns {
                // `repeat(auto-fit, minmax(min_rem, 1fr))`: each cell at least
                // `min_rem` wide, grows to fill, wraps to new rows.
                GridColumns::AutoFit { min_rem } => div()
                    .flex_grow()
                    .flex_basis(px(rem_to_px(*min_rem)))
                    .min_w(px(rem_to_px(*min_rem)))
                    .child(child),
                GridColumns::Tracks(tracks) if !tracks.is_empty() => {
                    match tracks[i % tracks.len()] {
                        // Weighted fr track → relative basis = weight / total.
                        GridTrack::Fr(weight) => div()
                            .flex_grow()
                            .flex_shrink_0()
                            .flex_basis(relative(weight / fr_total - 0.001))
                            .min_w(px(0.0))
                            .child(child),
                        // Fixed rem track → exact width, no grow/shrink.
                        GridTrack::Rem(rem) => {
                            div().w(px(rem_to_px(rem))).flex_shrink_0().child(child)
                        }
                    }
                }
                // Empty/degenerate track list → single equal column.
                GridColumns::Tracks(_) => div().flex_grow().min_w(px(0.0)).child(child),
            };
            el = el.child(wrapper);
        }

        el.into_any_element()
    }
}
