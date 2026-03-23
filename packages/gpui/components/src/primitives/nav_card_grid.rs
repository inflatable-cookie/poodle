//! NavCardGrid — real GPUI component backed by NavCardGridSpec.
//!
//! A responsive grid layout container for NavCard items.
//! Since gpui lacks native CSS grid, this groups children into rows
//! of equal-width columns using nested flex containers.

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::NavCardGridSpec;

use crate::theme_ext::resolve_px;

/// A navigation grid layout for NavCard children.
pub struct NavCardGrid {
    spec: NavCardGridSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for NavCardGrid {
    type Target = NavCardGridSpec;
    fn deref(&self) -> &NavCardGridSpec { &self.spec }
}

impl NavCardGrid {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: NavCardGridSpec::default(), theme: theme.clone(), children: Vec::new() }
    }

    pub fn from_spec(spec: NavCardGridSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────


    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for NavCardGrid {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let gap = resolve_px(theme, spec.gap_token());
        let columns = spec.columns.max(1) as usize;

        // Build a vertical stack of rows, each row containing `columns` items.
        let mut root = div()
            .flex()
            .flex_col()
            .gap(gap)
            .w_full();

        // Group children into rows of `columns` items each
        let mut row = div().flex().gap(gap);
        let mut col_idx = 0;

        for child in self.children {
            let wrapper = div()
                .flex_1()
                .min_w(px(0.0))
                .child(child);
            row = row.child(wrapper);
            col_idx += 1;

            if col_idx == columns {
                root = root.child(row);
                row = div().flex().gap(gap);
                col_idx = 0;
            }
        }

        // Handle the last incomplete row — pad with empty spacers to keep
        // items the same width as items in full rows.
        if col_idx > 0 {
            for _ in col_idx..columns {
                row = row.child(div().flex_1().min_w(px(0.0)));
            }
            root = root.child(row);
        }

        root.into_any_element()
    }
}
