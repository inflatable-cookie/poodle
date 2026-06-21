//! DetailSectionGroup — responsive layout container for grouped DetailSections.
//!
//! Contract: `docs/contracts/components/detail-section-group.md`
//! Reference: Svelte `DetailSectionGroup.svelte`, Jetstream
//! `js_detail_section_group`.
//!
//! `layout="grid"` flows sections across wrapping columns capped at
//! `max_columns`, each column at least `min_column_width` wide;
//! `layout="stack"` forces a single column. Inter-section gap is density-driven.
//! All dimensions resolve from the spec — no hardcoded px.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DetailSectionGroupLayout, DetailSectionGroupSpec};

use crate::presentation::rem_to_px;

pub struct DetailSectionGroup {
    spec: DetailSectionGroupSpec,
    children: Vec<AnyElement>,
}

impl DetailSectionGroup {
    pub fn from_spec(spec: DetailSectionGroupSpec, _theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
    pub fn layout(mut self, v: DetailSectionGroupLayout) -> Self {
        self.spec.layout = v;
        self
    }
    pub fn max_columns(mut self, v: u8) -> Self {
        self.spec = self.spec.with_max_columns(v);
        self
    }
    pub fn min_column_width(mut self, v: impl Into<String>) -> Self {
        self.spec.min_column_width = v.into();
        self
    }
    pub fn item_min_column_width(mut self, v: impl Into<String>) -> Self {
        self.spec.item_min_column_width = v.into();
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for DetailSectionGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let gap = px(rem_to_px(self.spec.gap_rem()));

        match self.spec.layout {
            DetailSectionGroupLayout::Stack => {
                // Single vertical column regardless of available width.
                let mut root = div().flex().flex_col().gap(gap);
                for child in self.children {
                    root = root.child(div().w_full().child(child));
                }
                root.into_any_element()
            }
            DetailSectionGroupLayout::Grid => {
                // Wrapping grid capped at `max_columns`. Each item's flex-basis
                // is a fraction of the row so at most N columns fit; `min_w`
                // keeps a column from collapsing below `min_column_width`.
                let columns = self.spec.max_columns.clamp(2, 5) as f32;
                let basis = relative(1.0 / columns - 0.01);
                let column_min = px(rem_to_px(self.spec.min_column_width_rem()));

                let mut root = div().flex().flex_row().flex_wrap().gap(gap);
                for child in self.children {
                    root = root.child(
                        div()
                            .flex_grow()
                            .flex_shrink_0()
                            .flex_basis(basis)
                            .min_w(column_min)
                            .child(child),
                    );
                }
                root.into_any_element()
            }
        }
    }
}
