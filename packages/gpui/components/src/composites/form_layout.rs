//! FormLayout — form layout container with column/row gaps and composed
//! Callout / FormActions primitives.
//!
//! Contract: `docs/contracts/components/form-layout.md`
//! Reference: `packages/svelte/components/src/FormLayout.svelte`
//!
//! Error/success messages delegate to the real `Callout` primitive; the action
//! row delegates to `FormActions` (contract §8 Composed Primitives). The
//! accessible field-errors summary (contract §2 / §6) renders a danger-toned
//! list. All spacing/colors resolve from tokens.

use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CallOutSpec, StatusTone};

use crate::primitives::{Callout, FormActions};

pub struct FormLayout {
    theme: GpuiThemeProvider,
    description: Option<String>,
    error: Option<String>,
    success: Option<String>,
    /// Ordered `(field-name, message)` field-error pairs for the accessible
    /// summary (contract §2 FieldErrors / §6 `role="alert"`).
    field_errors: Vec<(String, String)>,
    children: Vec<AnyElement>,
    actions: Option<AnyElement>,
    /// Number of columns for the field grid. Contract/Svelte default is 6.
    columns: usize,
}

impl FormLayout {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            description: None,
            error: None,
            success: None,
            field_errors: Vec::new(),
            children: Vec::new(),
            actions: None,
            columns: 6,
        }
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    pub fn error(mut self, v: impl Into<String>) -> Self {
        self.error = Some(v.into());
        self
    }
    pub fn success(mut self, v: impl Into<String>) -> Self {
        self.success = Some(v.into());
        self
    }
    /// Add a single field-error entry. Order is preserved.
    pub fn with_field_error(mut self, field: impl Into<String>, message: impl Into<String>) -> Self {
        self.field_errors.push((field.into(), message.into()));
        self
    }
    pub fn columns(mut self, n: usize) -> Self {
        self.columns = n.max(1);
        self
    }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }
}

/// Accessible field-errors summary (contract §2 FieldErrors / §6).
///
/// Background `color-mix(status-danger 8%, transparent)`, border
/// `color-mix(status-danger 40%, transparent)`, label-size text. GPUI has no
/// a11y channel, so `role="alert"`/`aria-live` are not emitted — the visual
/// summary still renders.
fn field_errors_summary(
    theme: &GpuiThemeProvider,
    errors: &[(String, String)],
) -> impl IntoElement {
    let danger = resolve_color(theme, "color.status.danger");
    let radius = resolve_radius(theme, "radius.surface");
    let pad_x = resolve_px(theme, "space.panel.x");
    let pad_y = resolve_px(theme, "space.panel.y");
    let font_size = resolve_px(theme, "typography.label.size");
    let text_color = resolve_color(theme, "color.text.primary");
    let gap = resolve_px(theme, "space.stack.sm");
    let transparent = Hsla { a: 0.0, ..danger };
    // color-mix toward transparent: 8% fill, 40% border (Svelte FormLayout:84-85).
    let fill = color_mix(danger, transparent, 0.08);
    let border = color_mix(danger, transparent, 0.40);

    let mut block = div()
        .w_full()
        .px(pad_x)
        .py(pad_y)
        .rounded(radius)
        .bg(fill)
        .border_1()
        .border_color(border)
        .flex()
        .flex_col()
        .gap(gap)
        .child(
            div()
                .text_size(font_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_color)
                .child("Please fix the following errors:"),
        );

    let mut list = div().flex().flex_col().gap(gap);
    for (field, message) in errors {
        list = list.child(
            div()
                .text_size(font_size)
                .text_color(text_color)
                .child(format!("{}: {}", field, message)),
        );
    }
    block = block.child(list);
    block
}

impl IntoElement for FormLayout {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let row_gap = resolve_px(theme, "space.stack.md");
        let column_gap = resolve_px(theme, "space.inline.md");
        let section_gap = resolve_px(theme, "space.stack.lg");
        let body_size = resolve_px(theme, "typography.body.size");
        // Per-column min-width for the flex approximation, from `size.select.minWidth`
        // (8rem) — the nearest field-control min-width semantic (was a raw 180px).
        let column_min_width = resolve_px(theme, "size.select.minWidth");

        let mut el = div().flex().flex_col().gap(section_gap).w_full();

        // Description
        if let Some(ref desc) = self.description {
            el = el.child(
                div()
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .child(desc.clone()),
            );
        }

        // Error/success delegate to the real Callout primitive (contract §8).
        if let Some(ref error) = self.error {
            el = el.child(Callout::from_spec(
                CallOutSpec::new().with_tone(StatusTone::Danger).with_content(error),
                theme,
            ));
        }
        if let Some(ref success) = self.success {
            el = el.child(Callout::from_spec(
                CallOutSpec::new().with_tone(StatusTone::Success).with_content(success),
                theme,
            ));
        }

        // Accessible field-errors summary (contract §2 / §6).
        if !self.field_errors.is_empty() {
            el = el.child(field_errors_summary(theme, &self.field_errors));
        }

        // Form fields — single-column flex-col, or wrapping flex-row for multi-col.
        // Container-query collapse (contract §7) is Svelte-only; the wrapping row
        // approximates columns at desktop widths.
        if self.columns <= 1 {
            let mut fields = div().flex().flex_col().gap(row_gap);
            for child in self.children {
                fields = fields.child(child);
            }
            el = el.child(fields);
        } else {
            let mut fields = div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(column_gap)
                .gap_y(row_gap);

            let basis_pct = 100.0 / self.columns as f32;
            for child in self.children {
                let wrapper = div()
                    .flex_grow()
                    .flex_shrink_0()
                    .flex_basis(relative(basis_pct / 100.0 - 0.01))
                    .min_w(column_min_width)
                    .child(child);
                fields = fields.child(wrapper);
            }
            el = el.child(fields);
        }

        // Actions delegate to the FormActions primitive (contract §2 / §8).
        if let Some(actions) = self.actions {
            el = el.child(FormActions::new(theme).with_action(actions));
        }

        el.into_any_element()
    }
}
