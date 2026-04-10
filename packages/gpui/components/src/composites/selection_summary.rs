//! SelectionSummary — real GPUI component backed by SelectionSummarySpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{RemediationAction, SelectionSummaryItem, SelectionSummarySpec};
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

use crate::presentation::{resolve_semantic_size, size_font_rem, control_space_x_rem, rem_to_px};
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
    fn deref(&self) -> &SelectionSummarySpec { &self.spec }
}

impl SelectionSummary {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SelectionSummarySpec::default(), theme: theme.clone(), on_remove: None, on_clear: None }
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
    pub fn items(mut self, v: Vec<SelectionSummaryItem>) -> Self { self.spec.items = v; self }
    pub fn clear_action(mut self, v: RemediationAction) -> Self { self.spec.clear_action = Some(v); self }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }


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
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let _font_size = rem_to_px(size_font_rem(effective_size));
        let _item_gap = rem_to_px(control_space_x_rem(spec.density));

        let gap = resolve_px(theme, spec.gap_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let border = resolve_color(theme, "color.border.subtle");
        let accent = resolve_color(theme, "color.accent.base");
        let bg = resolve_color(theme, "color.background.surface");

        let mut container = div()
            .w_full()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(gap);

        // Selected item pills
        for item in &spec.items {
            let item_id = SharedString::from(format!("selection-pill-{}", item.id));

            let mut pill = div()
                .id(item_id)
                .flex()
                .items_center()
                .gap(px(4.0))
                .px(px(8.0))
                .py(px(3.0))
                .rounded(px(12.0))
                .bg(bg)
                .border_1()
                .border_color(border);

            pill = pill.child(
                div()
                    .text_size(px(12.0))
                    .text_color(text_primary)
                    .child(item.label.clone()),
            );

            if let Some(ref meta) = item.meta {
                pill = pill.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_secondary.opacity(0.7))
                        .child(meta.clone()),
                );
            }

            // Remove button on pill
            pill = pill.child(
                div()
                    .cursor_pointer()
                    .text_size(px(12.0))
                    .text_color(text_secondary)
                    .child("\u{2715}"),
            );

            container = container.child(pill);
        }

        // Clear all action
        if spec.has_clear_action() {
            if let Some(ref clear_action) = spec.clear_action {
                let clear_id = SharedString::from("selection-summary-clear");
                let mut clear_btn = div()
                    .id(clear_id)
                    .cursor_pointer()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(accent)
                    .child(clear_action.label.clone());

                if let Some(handler) = self.on_clear {
                    clear_btn =
                        clear_btn.on_click(move |event, window, cx| handler(event, window, cx));
                }

                container = container.child(clear_btn);
            }
        }

        container.into_any_element()
    }
}
