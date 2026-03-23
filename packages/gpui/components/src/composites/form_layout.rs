//! FormLayout — form layout container with column/row gaps, title, and
//! callout-style error/success banners.
//! Uses semantic spacing tokens for consistent layout.

use gpui::prelude::FluentBuilder;
use gpui::*;
use flint_gpui::GpuiThemeProvider;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

pub struct FormLayout {
    theme: GpuiThemeProvider,
    title: Option<String>,
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
            title: None,
            description: None,
            error: None,
            success: None,
            children: Vec::new(),
            actions: None,
            columns: 1,
        }
    }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.title = Some(v.into()); self }
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

/// Builds a callout banner with a toned background for error/success messages.
fn callout_banner(
    theme: &GpuiThemeProvider,
    message: &str,
    tone_color: Hsla,
) -> impl IntoElement {
    let panel_bg = resolve_color(theme, "semantic.color.background.panel");
    let radius = resolve_radius(theme, "semantic.radius.control");
    let inline_pad = resolve_px(theme, "semantic.space.inline.md");
    let stack_pad = resolve_px(theme, "semantic.space.stack.sm");

    // Tone-colored background: 12% tone over panel
    let callout_bg = color_mix(tone_color, panel_bg, 0.12);
    // Tone-colored border: 30% tone over panel
    let callout_border = color_mix(tone_color, panel_bg, 0.30);

    div()
        .w_full()
        .px(inline_pad)
        .py(stack_pad)
        .rounded(radius)
        .bg(callout_bg)
        .border_1()
        .border_color(callout_border)
        .text_size(px(14.0))
        .text_color(tone_color)
        .child(message.to_string())
}

impl IntoElement for FormLayout {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let error_color = resolve_color(theme, "semantic.color.status.danger");
        let success_color = resolve_color(theme, "semantic.color.status.success");
        let row_gap = resolve_px(theme, "semantic.space.stack.md");
        let column_gap = resolve_px(theme, "semantic.space.inline.md");
        let section_gap = resolve_px(theme, "semantic.space.stack.lg");

        let mut el = div().flex().flex_col().gap(section_gap).w_full();

        // Title
        if let Some(ref title) = self.title {
            el = el.child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Description
        if let Some(ref desc) = self.description {
            el = el.child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_secondary)
                    .child(desc.clone()),
            );
        }

        // Error callout — tone-colored banner at top of form
        if let Some(ref error) = self.error {
            el = el.child(callout_banner(theme, error, error_color));
        }

        // Success callout — tone-colored banner at top of form
        if let Some(ref success) = self.success {
            el = el.child(callout_banner(theme, success, success_color));
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
