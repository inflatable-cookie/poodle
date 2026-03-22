//! FormLayout — form layout container with column/row gaps and validation display.
//! Uses semantic spacing tokens for consistent layout.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use crate::theme_ext::{resolve_color, resolve_px};

pub struct FormLayout {
    theme: GpuiThemeProvider,
    description: Option<String>,
    error: Option<String>,
    success: Option<String>,
    children: Vec<AnyElement>,
    actions: Option<AnyElement>,
    /// Number of columns for the field grid (default 1).
    columns: usize,
}

impl FormLayout {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            description: None,
            error: None,
            success: None,
            children: Vec::new(),
            actions: None,
            columns: 1,
        }
    }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn error(mut self, v: impl Into<String>) -> Self { self.error = Some(v.into()); self }
    pub fn success(mut self, v: impl Into<String>) -> Self { self.success = Some(v.into()); self }
    pub fn columns(mut self, n: usize) -> Self { self.columns = n.max(1); self }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element()); self
    }
}

impl IntoElement for FormLayout {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let text_color = resolve_color(&self.theme, "semantic.color.text.primary");
        let desc_color = resolve_color(&self.theme, "semantic.color.text.secondary");
        let error_color = resolve_color(&self.theme, "semantic.color.status.danger");
        let success_color = resolve_color(&self.theme, "semantic.color.status.success");
        let row_gap = resolve_px(&self.theme, "semantic.space.stack.md");
        let column_gap = resolve_px(&self.theme, "semantic.space.inline.md");
        let section_gap = resolve_px(&self.theme, "semantic.space.stack.lg");

        let mut el = div().flex().flex_col().gap(section_gap);

        if let Some(ref desc) = self.description {
            el = el.child(div().text_size(px(14.0)).text_color(desc_color).child(desc.clone()));
        }
        if let Some(ref error) = self.error {
            el = el.child(div().text_size(px(14.0)).text_color(error_color).child(error.clone()));
        }
        if let Some(ref success) = self.success {
            el = el.child(div().text_size(px(14.0)).text_color(success_color).child(success.clone()));
        }

        // Form fields — grid-like layout with row_gap and column_gap.
        // When columns > 1, fields are laid out in rows with wrapping.
        if self.columns <= 1 {
            // Single-column: simple flex column with row gap
            let mut fields = div().flex().flex_col().gap(row_gap);
            for child in self.children {
                fields = fields.child(child);
            }
            el = el.child(fields);
        } else {
            // Multi-column: flex-wrap row with column_gap (horizontal) and row_gap (vertical)
            let mut fields = div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(column_gap)
                .gap_y(row_gap);

            // Each field gets a fractional basis to approximate columns.
            // basis = (100% - (columns-1)*column_gap) / columns
            // We approximate with a fixed percentage.
            let basis_pct = 100.0 / self.columns as f32;
            for child in self.children {
                let wrapper = div()
                    .flex_grow()
                    .flex_shrink_0()
                    .flex_basis(relative(basis_pct / 100.0 - 0.01))
                    .min_w(px(180.0))
                    .child(child);
                fields = fields.child(wrapper);
            }
            el = el.child(fields);
        }

        if let Some(actions) = self.actions {
            el = el.child(actions);
        }

        el.into_any_element()
    }
}
