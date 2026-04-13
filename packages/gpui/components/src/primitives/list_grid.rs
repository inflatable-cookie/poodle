//! ListGrid — responsive card/tile collection layout (`list-grid.md`).
//!
//! GPUI has no CSS Grid; `repeat(auto-fill, minmax(...))` is approximated with
//! `flex_wrap`, per-tile `min_w`, `flex_1`, and `flex_basis(0)`.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ListGridSpec, ListGridVariant};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_px;

/// List grid layout backed by [`ListGridSpec`].
pub struct ListGrid {
    spec: ListGridSpec,
    theme: GpuiThemeProvider,
    header: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for ListGrid {
    type Target = ListGridSpec;
    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}

impl ListGrid {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ListGridSpec::new(),
            theme: theme.clone(),
            header: None,
            children: Vec::new(),
        }
    }

    pub fn from_spec(spec: ListGridSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            header: None,
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, v: ListGridVariant) -> Self {
        self.spec.variant = v;
        self
    }

    pub fn min_item_width_em(mut self, em: f32) -> Self {
        self.spec.min_item_width_em = Some(em);
        self
    }

    pub fn with_header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for ListGrid {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let gap = resolve_px(theme, spec.gap_token());
        let min_w = if let Some(em) = spec.min_item_width_em {
            px(rem_to_px(em))
        } else {
            resolve_px(theme, spec.min_item_width_token())
        };

        let mut root = div().flex().flex_col().w_full();

        if let Some(header) = self.header {
            let header_gap = resolve_px(theme, ListGridSpec::header_actions_gap_token());
            let header_after = resolve_px(theme, spec.header_margin_bottom_token());
            root = root.child(
                div()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .items_center()
                    .gap(header_gap)
                    .pb(header_after)
                    .child(header),
            );
        }

        let mut grid = div().flex().w_full();

        match spec.variant {
            ListGridVariant::Compact => {
                grid = grid.flex_col().gap(gap);
                for child in self.children {
                    grid = grid.child(child);
                }
            }
            ListGridVariant::Default => {
                grid = grid.flex_wrap().gap(gap);
                for child in self.children {
                    let cell = div()
                        .min_w(min_w)
                        .flex_1()
                        .flex_basis(px(0.0))
                        .child(child);
                    grid = grid.child(cell);
                }
            }
        }

        root = root.child(grid);
        root.into_any_element()
    }
}
