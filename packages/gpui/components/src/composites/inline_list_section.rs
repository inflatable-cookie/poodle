//! InlineListSection — compact card-contained related-list shell.
//!
//! Contract: `docs/contracts/components/inline-list-section.md`
//! Reference: Svelte `InlineListSection.svelte`, Jetstream `inline_list_section.rs`.
//!
//! Owns the outer Card (when `framed`), the compact uppercase header with an
//! optional count pill and header actions, the stacked list with compact muted
//! row chrome, and the empty-state posture. Every dimension/color resolves from
//! a token or a contract-exact rem — no hardcoded px/hsla.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CardSpec, InlineListSectionSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};
use crate::Card;

pub struct InlineListSection {
    spec: InlineListSectionSpec,
    theme: GpuiThemeProvider,
    items: Vec<AnyElement>,
    actions: Option<AnyElement>,
}

impl InlineListSection {
    pub fn from_spec(spec: InlineListSectionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            items: Vec::new(),
            actions: None,
        }
    }

    pub fn with_action(mut self, action: impl IntoElement) -> Self {
        self.actions = Some(action.into_any_element());
        self
    }

    pub fn item(mut self, item: impl IntoElement) -> Self {
        self.items.push(item.into_any_element());
        self
    }
}

impl IntoElement for InlineListSection {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        // Colors (contract Token Usage tables).
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_primary = resolve_color(theme, "color.text.primary");
        let border = resolve_color(theme, "color.border.default");
        let elevated = resolve_color(theme, "color.background.elevated");
        let surface = resolve_color(theme, "color.background.surface");
        // Item chrome: color-mix(in srgb, surface 93%, text-primary).
        let row_bg = color_mix(surface, text_primary, 0.93);

        // Typography (contract Token Usage tables).
        let label_size = resolve_px(theme, "typography.label.size");
        let body_size = resolve_px(theme, "typography.body.size");

        // Spacing (token + contract-exact rem).
        let root_gap = resolve_px(theme, "space.stack.md");
        let items_gap = resolve_px(theme, "space.stack.sm");
        let header_gap = px(rem_to_px(0.75));
        let heading_gap = px(rem_to_px(0.5));

        // Count pill geometry (contract Count table).
        let count_min_w = px(rem_to_px(1.875));
        let count_h = px(rem_to_px(1.375));
        let count_pad_x = px(rem_to_px(0.5));
        let count_radius = px(rem_to_px(999.0));
        let count_border_w = px(rem_to_px(0.0625)); // 1px-equivalent contract border

        // Item chrome geometry (contract Item table).
        let item_gap = px(rem_to_px(0.75));
        let item_pad_x = px(rem_to_px(0.625));
        let item_pad_y = px(rem_to_px(0.5));
        let surface_radius = resolve_radius(theme, "radius.surface");
        let item_radius = px(f32::from(surface_radius) - rem_to_px(0.1875));

        // Header: heading cluster (title + optional count) + optional actions.
        let heading = div()
            .flex()
            .items_center()
            .gap(heading_gap)
            .min_w_0()
            .child(
                div()
                    .text_size(label_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_secondary)
                    .child(self.spec.title.clone().to_uppercase()),
            )
            .when_some(self.spec.count.as_ref(), |el, count| {
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .min_w(count_min_w)
                        .h(count_h)
                        .px(count_pad_x)
                        .rounded(count_radius)
                        .border(count_border_w)
                        .border_color(border)
                        .bg(elevated)
                        .text_size(label_size)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_secondary)
                        .child(count.clone()),
                )
            });

        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .gap(header_gap)
            .child(heading)
            .when_some(self.actions, |el, action| el.child(action));

        let mut body = div().flex().flex_col().gap(root_gap).child(header);

        if self.items.is_empty() {
            if let Some(message) = self.spec.empty_message.clone() {
                body = body.child(
                    div()
                        .text_size(body_size)
                        .italic()
                        .text_color(text_secondary)
                        .child(message),
                );
            }
        } else {
            let mut list = div().flex().flex_col().gap(items_gap);
            for item in self.items {
                list = list.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(item_gap)
                        .min_w_0()
                        .px(item_pad_x)
                        .py(item_pad_y)
                        .rounded(item_radius)
                        .bg(row_bg)
                        .child(item),
                );
            }
            body = body.child(list);
        }

        if self.spec.framed {
            Card::from_spec(CardSpec::new(), theme)
                .with_body(body)
                .into_any_element()
        } else {
            body.into_any_element()
        }
    }
}
