//! SelectionSummary — real GPUI component backed by SelectionSummarySpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_specs::{RemediationAction, SelectionSummaryItem, SelectionSummarySpec};

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI selection summary component backed by `SelectionSummarySpec`.
///
/// Renders a horizontal display of selected items as pills/chips with an
/// optional clear-all action button.
pub struct SelectionSummary {
    spec: SelectionSummarySpec,
    theme: GpuiThemeProvider,
    on_remove: Option<Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
    on_clear: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for SelectionSummary {
    type Target = SelectionSummarySpec;
    fn deref(&self) -> &SelectionSummarySpec {
        &self.spec
    }
}

impl SelectionSummary {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SelectionSummarySpec::default(),
            theme: theme.clone(),
            on_remove: None,
            on_clear: None,
        }
    }

    pub fn from_spec(spec: SelectionSummarySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_remove: None,
            on_clear: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<SelectionSummaryItem>) -> Self {
        self.spec.items = v;
        self
    }
    pub fn clear_action(mut self, v: RemediationAction) -> Self {
        self.spec.clear_action = Some(v);
        self
    }
    pub fn max_visible_items(mut self, v: usize) -> Self {
        self.spec.max_visible_items = Some(v);
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn on_remove(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_remove = Some(Box::new(handler));
        self
    }

    pub fn on_clear(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_clear = Some(Box::new(handler));
        self
    }
}

impl IntoElement for SelectionSummary {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let on_remove = self.on_remove;
        let on_clear = self.on_clear;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));

        let gap = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => control_space_x_rem(spec.density),
            ControlDensity::Comfortable => 0.75,
        }));
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let border = resolve_color(theme, "color.border.subtle");
        let accent = resolve_color(theme, "color.accent.base");
        let surface = resolve_color(theme, "color.background.surface");
        let elevated = resolve_color(theme, "color.background.elevated");

        let chip_font = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.71875,
            ControlSize::Md => 0.75,
            ControlSize::Lg => 0.8125,
            ControlSize::Xl => 0.875,
        }));
        let chip_pad_x = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 0.875,
        }));
        let overflow_pad_x = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.625,
            ControlDensity::Comfortable => 0.75,
        }));
        let clear_font = px(font_size);
        let chip_min_h = px(rem_to_px(match effective_size {
            ControlSize::Xs => 1.0,
            ControlSize::Sm => 1.125,
            ControlSize::Md => 1.5,
            ControlSize::Lg => 1.75,
            ControlSize::Xl => 2.0,
        }));
        let bottom_pad = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.625,
            ControlDensity::Comfortable => 0.75,
        }));
        let chip_internal_gap = gap;
        let chip_bg = Hsla {
            a: surface.a * 0.6 + elevated.a * 0.4,
            ..elevated
        };
        let overflow_bg = Hsla {
            a: surface.a * 0.68 + elevated.a * 0.32,
            ..elevated
        };
        let stacked_border = Hsla {
            a: border.a * 0.7,
            ..border
        };
        let empty_text = resolve_color(theme, "color.text.tertiary");

        let mut container = div()
            .w_full()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(gap)
            .pb(bottom_pad)
            .min_h(chip_min_h);

        if spec.items.is_empty() {
            return container
                .child(
                    div()
                        .text_size(chip_font)
                        .text_color(empty_text)
                        .child("No selection"),
                )
                .into_any_element();
        }

        // Selected item pills — clamped by max_visible_items if set.
        let visible_count = spec.visible_item_count();
        for item in spec.items.iter().take(visible_count) {
            let item_id = SharedString::from(format!("selection-pill-{}", item.id));

            let mut pill = div()
                .id(item_id)
                .flex()
                .items_center()
                .gap(chip_internal_gap)
                .px(chip_pad_x)
                .min_h(chip_min_h)
                .rounded(px(12.0))
                .bg(chip_bg)
                .border_1()
                .border_color(stacked_border)
                .cursor_pointer();

            pill = pill.child(
                div()
                    .text_size(chip_font)
                    .text_color(text_primary)
                    .child(item.label.clone()),
            );

            if let Some(ref meta) = item.meta {
                pill = pill.child(
                    div()
                        .text_size(chip_font)
                        .text_color(text_secondary)
                        .child(meta.clone()),
                );
            }

            // Remove button on pill
            pill = pill.child(
                div()
                    .text_size(chip_font)
                    .text_color(text_secondary)
                    .child("\u{2715}"),
            );

            if let Some(ref handler) = on_remove {
                let handler_ptr =
                    handler as *const Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App)>;
                let item_id = item.id.clone();
                pill = pill.on_click(move |event, window, cx| {
                    let handler = unsafe { &*handler_ptr };
                    handler(&item_id, event, window, cx);
                });
            }

            container = container.child(pill);
        }

        // Overflow chip — "+N more" when the list was truncated.
        let overflow = spec.overflow_count();
        if overflow > 0 {
            container = container.child(
                div()
                    .flex()
                    .items_center()
                    .px(overflow_pad_x)
                    .min_h(chip_min_h)
                    .rounded(px(12.0))
                    .bg(overflow_bg)
                    .border_1()
                    .border_color(stacked_border)
                    .text_size(chip_font)
                    .text_color(text_secondary)
                    .child(format!("+{overflow} more")),
            );
        }

        // Clear all action
        if spec.has_clear_action() {
            if let Some(ref clear_action) = spec.clear_action {
                let clear_id = SharedString::from("selection-summary-clear");
                let mut clear_btn = div().id(clear_id).flex().flex_grow().child(
                    div()
                        .cursor_pointer()
                        .text_size(clear_font)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(accent)
                        .child(clear_action.label.clone()),
                );

                if let Some(handler) = on_clear {
                    clear_btn =
                        clear_btn.on_click(move |event, window, cx| handler(event, window, cx));
                }

                container = container.child(clear_btn);
            }
        }

        container.into_any_element()
    }
}
