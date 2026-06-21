//! DetailItem — real GPUI component backed by DetailItemSpec.
//!
//! Contract: `docs/contracts/components/detail-item.md`
//! Reference: `packages/svelte/components/src/DetailItem.svelte`
//!
//! A label/value pair with inline (default) or stacked layout, simple/surface
//! presentation, density-driven spacing, an optional trailing action slot, an
//! optional info description, and an em-dash empty placeholder. All geometry
//! and colour resolve from tokens / the density-aware spec helpers.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, DetailItemLayout, DetailItemPresentation, DetailItemSpan, DetailItemSpec,
};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct DetailItem {
    spec: DetailItemSpec,
    label_color: Hsla,
    value_color: Hsla,
    description_color: Hsla,
    tertiary_color: Hsla,
    background: Hsla,
    radius: Pixels,
    label_size: Pixels,
    value_size: Pixels,
    action: Option<AnyElement>,
    value_content: Option<AnyElement>,
}

impl std::ops::Deref for DetailItem {
    type Target = DetailItemSpec;
    fn deref(&self) -> &DetailItemSpec {
        &self.spec
    }
}

impl DetailItem {
    pub fn new(label: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(DetailItemSpec::new(label), theme)
    }

    pub fn from_spec(spec: DetailItemSpec, theme: &GpuiThemeProvider) -> Self {
        let label_color = resolve_color(theme, spec.label_color_token());
        let value_color = resolve_color(theme, spec.value_color_token());
        let description_color = resolve_color(theme, spec.description_color_token());
        let tertiary_color = resolve_color(theme, spec.stacked_label_color_token());
        let background = resolve_color(theme, spec.background_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let label_size = resolve_px(theme, spec.label_size_token());
        let value_size = resolve_px(theme, spec.value_size_token());

        Self {
            spec,
            label_color,
            value_color,
            description_color,
            tertiary_color,
            background,
            radius,
            label_size,
            value_size,
            action: None,
            value_content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = v.into();
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn empty_text(mut self, v: impl Into<String>) -> Self {
        self.spec.empty_text = v.into();
        self
    }
    pub fn truncate_value(mut self, v: bool) -> Self {
        self.spec.truncate_value = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn layout(mut self, v: DetailItemLayout) -> Self {
        self.spec.layout = v;
        self
    }
    pub fn presentation(mut self, v: DetailItemPresentation) -> Self {
        self.spec.presentation = v;
        self
    }
    pub fn span(mut self, v: DetailItemSpan) -> Self {
        self.spec.span = Some(v);
        self
    }
    pub fn density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn with_action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    pub fn with_value_content(mut self, content: impl IntoElement) -> Self {
        self.value_content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for DetailItem {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let is_stacked = self.spec.layout == DetailItemLayout::Stacked;
        let is_surface = self.spec.presentation == DetailItemPresentation::Surface;
        let is_surface_stacked = is_surface && is_stacked;

        // Density-aware spacing resolved from the spec (contract §7/§8).
        let row_gap = px(rem_to_px(self.spec.row_gap_rem()));
        let inline_gap = px(rem_to_px(self.spec.inline_gap_rem()));
        let padding_x = px(rem_to_px(self.spec.surface_padding_x_rem()));
        let padding_y = px(rem_to_px(self.spec.surface_padding_y_rem()));

        // Surface+stacked: label shifts to tertiary, 0.75rem / lh 1.35.
        let label_color = if is_surface_stacked {
            self.tertiary_color
        } else {
            self.label_color
        };
        let label_size = if is_surface_stacked {
            px(rem_to_px(0.75))
        } else {
            self.label_size
        };

        let mut label_el = div()
            .flex()
            .flex_col()
            .gap(row_gap)
            .child(
                div()
                    .text_size(label_size)
                    .text_color(label_color)
                    .child(self.spec.label.clone()),
            )
            .when_some(self.spec.description.as_ref(), |el, desc| {
                el.child(
                    div()
                        .text_size(px(rem_to_px(0.75)))
                        .text_color(self.description_color)
                        .child(desc.clone()),
                )
            });

        // Inline layout: fixed label column (contract: minmax(8rem, 11.25rem)).
        if !is_stacked {
            label_el = label_el.w(px(rem_to_px(11.25))).flex_shrink_0();
        }

        // Surface+stacked: value emphasis — 1rem / weight 600.
        let value_size = if is_surface_stacked {
            px(rem_to_px(1.0))
        } else {
            self.value_size
        };

        let value_block = if let Some(content) = self.value_content {
            div().flex_1().child(content)
        } else if let Some(ref value) = self.spec.value {
            div()
                .flex_1()
                .text_size(value_size)
                .text_color(self.value_color)
                .when(is_surface_stacked, |el| el.font_weight(FontWeight::SEMIBOLD))
                .when(self.spec.truncate_value, |el| {
                    el.overflow_x_hidden().text_ellipsis()
                })
                .child(value.clone())
        } else {
            // Empty state: em-dash placeholder in muted colour.
            div()
                .flex_1()
                .text_size(value_size)
                .text_color(self.description_color)
                .child(self.spec.empty_text.clone())
        };

        let mut row = div().flex().gap(if is_stacked { row_gap } else { inline_gap });

        // Span="full" stretches across the parent grid; on a flex parent we
        // approximate by filling available width. Half is inert without a grid
        // parent (noted in parity doc).
        if matches!(self.spec.span, Some(DetailItemSpan::Full)) {
            row = row.w_full();
        }

        if is_surface {
            row = row
                .bg(self.background)
                .rounded(self.radius)
                .px(padding_x)
                .py(padding_y);
        }

        if is_stacked {
            row = row.flex_col();
            if is_surface_stacked {
                row = row.items_start();
            }
        } else {
            row = row.flex_row().items_center();
        }

        row.child(label_el)
            .child(value_block)
            .when_some(self.action, |el, action| el.child(action))
            .into_any_element()
    }
}
