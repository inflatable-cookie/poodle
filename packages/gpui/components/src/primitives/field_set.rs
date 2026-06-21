//! FieldSet — groups related form controls under an optional legend
//! with multi-column grid layout support.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::FieldSetSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI FieldSet component backed by `FieldSetSpec`.
pub struct FieldSet {
    spec: FieldSetSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for FieldSet {
    type Target = FieldSetSpec;
    fn deref(&self) -> &FieldSetSpec {
        &self.spec
    }
}

impl FieldSet {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: FieldSetSpec::new(),
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn from_spec(spec: FieldSetSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn legend(mut self, v: impl Into<String>) -> Self {
        self.spec.legend = Some(v.into());
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn columns(mut self, v: u8) -> Self {
        self.spec.columns = v;
        self
    }

    pub fn with_child(mut self, child: impl IntoElement + 'static) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for FieldSet {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let legend_color = resolve_color(theme, spec.legend_color_token());
        // Column-gap = scaleToSpace(gap); None → 0 (contract §6).
        let col_gap = spec
            .column_gap_token()
            .map(|t| resolve_px(theme, t))
            .unwrap_or_else(|| px(0.0));
        // Row-gap = column-gap + 0.5rem (asymmetric, contract §6).
        let row_gap = px(f32::from(col_gap) + rem_to_px(FieldSetSpec::ROW_GAP_EXTRA_REM));
        let legend_mb = resolve_px(theme, spec.legend_margin_bottom_token());

        let mut root = div().flex().flex_col();

        // `span` (FieldSetSpec) is a CSS-grid placement prop. GPUI has no
        // CSS-grid parent context, so it is an accepted layout delta — the
        // owning layout positions the fieldset; no per-fieldset style emitted.

        // Legend — eyebrow scale: fixed 0.6875rem (NOT typography-label-size,
        // contract §6), semibold, uppercase. letter-spacing 0.12em and
        // line-height 1.5 have no GPUI div API — accepted rendering delta.
        if let Some(ref legend) = spec.legend {
            root = root.child(
                div()
                    .text_size(px(rem_to_px(FieldSetSpec::LEGEND_SIZE_REM)))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(legend_color)
                    .mb(legend_mb)
                    .child(legend.to_uppercase()),
            );
        }

        // Description — <p> between legend and fields (contract §2).
        if let Some(ref description) = spec.description {
            let desc_color = resolve_color(theme, spec.description_color_token());
            let desc_size = resolve_px(theme, spec.description_size_token());
            let desc_mb = resolve_px(theme, spec.description_margin_bottom_token());
            root = root.child(
                div()
                    .text_size(desc_size)
                    .text_color(desc_color)
                    .mb(desc_mb)
                    .child(description.clone()),
            );
        }

        // Fields grid. Equal-fraction columns via flex_1 (mirrors CSS
        // repeat(columns, minmax(0, 1fr))) with asymmetric row/column gaps.
        let mut grid = if spec.columns > 1 {
            div().flex().flex_wrap()
        } else {
            div().flex().flex_col()
        };
        // Taffy/gpui flex has no separate row/column gap; the wrap row-gap and
        // the inline column-gap are both driven by `gap`. Use column-gap as the
        // single flex gap; the extra row-gap is an accepted flex-engine delta.
        grid = grid.gap(if spec.columns > 1 { col_gap } else { row_gap });

        for child in self.children {
            if spec.columns > 1 {
                // Equal fraction per column: flex_1 with min_w(0) so each child
                // shares remaining width evenly (≈ minmax(0, 1fr)).
                grid = grid.child(div().flex_1().min_w(px(0.0)).child(child));
            } else {
                grid = grid.child(child);
            }
        }

        root = root.child(grid);
        root.into_any_element()
    }
}
