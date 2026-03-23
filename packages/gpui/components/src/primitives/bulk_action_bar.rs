//! BulkActionBar — real GPUI component backed by BulkActionBarSpec.

use std::rc::Rc;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{BulkAction, BulkActionBarSpec, BulkActionTone};

use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI bulk-action bar component backed by `BulkActionBarSpec`.
pub struct BulkActionBar {
    spec: BulkActionBarSpec,
    theme: GpuiThemeProvider,
    on_action: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for BulkActionBar {
    type Target = BulkActionBarSpec;
    fn deref(&self) -> &BulkActionBarSpec { &self.spec }
}

impl BulkActionBar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: BulkActionBarSpec::new(), theme: theme.clone(), on_action: None }
    }

    pub fn from_spec(spec: BulkActionBarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_action: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn selection_count(mut self, v: usize) -> Self { self.spec.selection_count = v; self }
    pub fn total_count(mut self, v: usize) -> Self { self.spec.total_count = Some(v); self }
    pub fn actions(mut self, v: Vec<BulkAction>) -> Self { self.spec.actions = v; self }

    pub fn on_action(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_action = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for BulkActionBar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve tokens ──────────────────────────────────────────
        let accent = resolve_color(theme, spec.fill_token());
        let panel_bg = resolve_color(theme, "semantic.color.background.panel");
        let fill = color_mix(accent, panel_bg, 0.10);
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let text_color = resolve_color(theme, spec.text_token());
        let total_text_color = resolve_color(theme, spec.total_text_token());
        let button_fill = resolve_color(theme, spec.button_fill_token());
        let button_border = resolve_color(theme, spec.button_border_token());
        let button_radius = resolve_radius(theme, spec.button_radius_token());
        let danger_border_raw = resolve_color(theme, spec.danger_border_token());
        let danger_text = resolve_color(theme, spec.danger_text_token());
        let gap = resolve_px(theme, spec.gap_token());
        let pad_x = resolve_px(theme, spec.padding_x_token());
        let pad_y = resolve_px(theme, spec.padding_y_token());
        let control_height = resolve_px(theme, "semantic.size.control-height");
        let control_pad_x = resolve_px(theme, "semantic.space.control.x");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");

        // Danger button border: 65% danger mixed with default border
        let danger_border = color_mix(danger_border_raw, button_border, 0.65);

        // ── Summary section (left) ──────────────────────────────────
        let summary = {
            let mut row = div().flex().flex_row().items_center().gap(px(4.0));
            let count_text = format!("{}", spec.selection_count);
            row = row.child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_color)
                    .child(count_text),
            );
            match spec.total_count {
                Some(total) => {
                    row = row.child(
                        div()
                            .text_size(px(14.0))
                            .text_color(total_text_color)
                            .child(format!("of {} selected", total)),
                    );
                }
                None => {
                    row = row.child(
                        div()
                            .text_size(px(14.0))
                            .text_color(text_color)
                            .child("selected".to_string()),
                    );
                }
            }
            row
        };

        // ── Actions section (right) ─────────────────────────────────
        let actions = {
            let mut row = div().flex().flex_row().items_center().gap(gap);
            for action in &spec.actions {
                let is_danger = action.tone == BulkActionTone::Danger;
                let btn_border = if is_danger { danger_border } else { button_border };
                let btn_text = if is_danger { danger_text } else { text_color };
                let btn_id = SharedString::from(format!("bulk-action-{}", action.id));

                let hover_fill = color_mix(button_fill, elevated, 0.84);

                let mut btn = div()
                    .id(btn_id)
                    .h(control_height)
                    .px(control_pad_x)
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(button_radius)
                    .bg(button_fill)
                    .border_1()
                    .border_color(btn_border)
                    .cursor_pointer()
                    .hover(move |s| s.bg(hover_fill))
                    .text_size(px(13.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(btn_text)
                    .child(action.label.clone());

                if let Some(ref handler) = self.on_action {
                    let handler = handler.clone();
                    let action_id = action.id.clone();
                    btn = btn.on_click(move |_event, window, cx| {
                        handler(&action_id, window, cx);
                    });
                }

                row = row.child(btn);
            }
            row
        };

        // ── Root container ──────────────────────────────────────────
        div()
            .w_full()
            .px(pad_x)
            .py(pad_y)
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .rounded(radius)
            .bg(fill)
            .border_1()
            .border_color(border)
            .child(summary)
            .child(actions)
            .into_any_element()
    }
}
