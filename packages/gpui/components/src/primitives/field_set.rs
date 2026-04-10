//! FieldSet — groups related form controls under an optional legend
//! with multi-column grid layout support.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::FieldSetSpec;

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI FieldSet component backed by `FieldSetSpec`.
pub struct FieldSet {
    spec: FieldSetSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for FieldSet {
    type Target = FieldSetSpec;
    fn deref(&self) -> &FieldSetSpec { &self.spec }
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

    pub fn legend(mut self, v: impl Into<String>) -> Self { self.spec.legend = Some(v.into()); self }
    pub fn columns(mut self, v: u8) -> Self { self.spec.columns = v; self }

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
        let gap = resolve_px(theme, spec.gap_token());
        let stack_sm = resolve_px(theme, "space.stack.sm");

        let mut root = div().flex().flex_col();

        // Legend (styled like Eyebrow: uppercase, small, semibold)
        if let Some(ref legend) = spec.legend {
            root = root.child(
                div()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(legend_color)
                    .mb(stack_sm)
                    .child(legend.to_uppercase()),
            );
        }

        // Fields grid
        let mut grid = div().flex().flex_col().gap(gap);

        // For multi-column, use flex-wrap with width constraints
        if spec.columns > 1 {
            grid = div().flex().flex_wrap().gap(gap);
        }

        for child in self.children {
            if spec.columns > 1 {
                // Each child takes up 1/columns of the width minus gap
                let col_width = match spec.columns {
                    2 => px(240.0), // approximate half-width
                    3 => px(160.0),
                    _ => px(120.0),
                };
                grid = grid.child(div().min_w(col_width).flex_1().child(child));
            } else {
                grid = grid.child(child);
            }
        }

        root = root.child(grid);
        root.into_any_element()
    }
}
