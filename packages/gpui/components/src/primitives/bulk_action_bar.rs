//! BulkActionBar — real GPUI component backed by BulkActionBarSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    BulkAction, BulkActionBarSpec, BulkActionTone, ButtonTone, ButtonVariant, ControlDensity,
    ControlSize, SemanticControlSizeRole,
};
use std::rc::Rc;

use super::icon_button::IconButton;
use crate::presentation::{panel_space_x_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI bulk-action bar component backed by `BulkActionBarSpec`.
pub struct BulkActionBar {
    spec: BulkActionBarSpec,
    theme: GpuiThemeProvider,
    on_action: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_select_all: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_clear: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for BulkActionBar {
    type Target = BulkActionBarSpec;
    fn deref(&self) -> &BulkActionBarSpec {
        &self.spec
    }
}

impl BulkActionBar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: BulkActionBarSpec::new(),
            theme: theme.clone(),
            on_action: None,
            on_select_all: None,
            on_clear: None,
        }
    }

    pub fn from_spec(spec: BulkActionBarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_action: None,
            on_select_all: None,
            on_clear: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn selection_count(mut self, v: usize) -> Self {
        self.spec.selection_count = v;
        self
    }
    pub fn total_count(mut self, v: usize) -> Self {
        self.spec.total_count = Some(v);
        self
    }
    pub fn actions(mut self, v: Vec<BulkAction>) -> Self {
        self.spec.actions = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
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
    pub fn show_select_all(mut self, v: bool) -> Self {
        self.spec.show_select_all = v;
        self
    }
    pub fn all_selected(mut self, v: bool) -> Self {
        self.spec.all_selected = v;
        self
    }
    pub fn loading(mut self, v: bool) -> Self {
        self.spec.loading = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.disabled = v;
        self
    }

    pub fn on_action(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_action = Some(Rc::new(handler));
        self
    }

    /// Click handler for the "Select all" / "Deselect all" affordance.
    /// Matches the Div::on_click signature so cx.listener passes through.
    pub fn on_select_all(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select_all = Some(Rc::new(handler));
        self
    }

    /// Click handler for the clear-selection (`x`) IconButton.
    pub fn on_clear(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_clear = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for BulkActionBar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size / density ──────────────────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        // Svelte: body-scale font (one step up from size_font_rem)
        let body_font = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.75,
            ControlSize::Sm => 0.8125,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 0.9375,
            ControlSize::Xl => 1.0,
        }));
        let density_pad_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        // Svelte: Y padding flat 0.5rem (not density-based)
        let density_pad_y = px(rem_to_px(0.5));
        // Svelte: summary/actions rows use space.inline.sm (8px) for default/compact
        // Actions density: compact=0.125rem, default=space.inline.sm, comfortable=0.5rem
        let summary_gap = resolve_px(theme, "space.inline.sm");
        let actions_gap = match spec.density {
            ControlDensity::Compact => px(rem_to_px(0.125)),
            ControlDensity::Default => resolve_px(theme, "space.inline.sm"),
            ControlDensity::Comfortable => px(rem_to_px(0.5)),
        };

        // ── Resolve tokens ──────────────────────────────────────────
        let panel_bg = resolve_color(theme, "color.background.panel");
        let text_primary = resolve_color(theme, "color.text.primary");
        // Svelte: background = color-mix(panel 93%, text-primary)
        let fill = color_mix(panel_bg, text_primary, 0.93);
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let text_color = resolve_color(theme, spec.text_token());
        let total_text_color = resolve_color(theme, spec.total_text_token());
        let pad_x = density_pad_x;
        let pad_y = density_pad_y;
        let body_size = body_font;

        // Shared availability gates (Svelte isUnavailable / actionsDisabled).
        let is_unavailable = spec.is_unavailable();
        let actions_disabled = spec.actions_disabled();

        // ── Summary section (left) ──────────────────────────────────
        let summary = {
            let mut row = div().flex().flex_row().items_center().gap(summary_gap);

            // Count + label block — Svelte: gap = space.inline.sm (8px)
            let mut count_block = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(resolve_px(theme, "space.inline.sm"));
            count_block = count_block.child(
                div()
                    .text_size(body_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_color)
                    .child(format!("{} selected", spec.selection_count)),
            );
            // Svelte: total reads "of {totalCount}" (no trailing word).
            if let Some(total) = spec.total_count {
                count_block = count_block.child(
                    div()
                        .text_size(body_size)
                        .text_color(total_text_color)
                        .child(format!("of {}", total)),
                );
            }
            row = row.child(count_block);

            // Select-all: ghost `check` IconButton (Svelte uses `check-check`;
            // that asset is absent in GPUI, so `check` is the faithful
            // substitute). Shown only while not all-selected, per Svelte.
            if spec.show_select_all && !spec.all_selected {
                let label = match spec.total_count {
                    Some(total) => format!("{} ({})", spec.select_all_label(), total),
                    None => spec.select_all_label().to_string(),
                };
                let mut select_all_btn = IconButton::new(theme)
                    .variant(ButtonVariant::Ghost)
                    .size_role(SemanticControlSizeRole::Chrome)
                    .icon("check")
                    .aria_label(label.clone())
                    .tooltip(label)
                    .disabled(is_unavailable)
                    .with_id("bulk-select-all");

                if let Some(ref handler) = self.on_select_all {
                    let handler = handler.clone();
                    select_all_btn =
                        select_all_btn.on_click(move |event, window, cx| handler(event, window, cx));
                }

                row = row.child(select_all_btn);
            }

            row
        };

        // ── Actions section (right) — ghost IconButtons + clear ──────
        let actions = {
            let mut row = div().flex().flex_row().items_center().gap(actions_gap);

            for action in &spec.actions {
                // GPUI ButtonTone has no Warning; danger → Danger, warning &
                // default → Default (warning has no icon-tint hook here).
                let tone = match action.tone {
                    BulkActionTone::Danger => ButtonTone::Danger,
                    _ => ButtonTone::Default,
                };
                // Svelte: icon shows; label is the accessible name / tooltip.
                let mut btn = IconButton::new(theme)
                    .variant(ButtonVariant::Ghost)
                    .tone(tone)
                    .size(effective_size)
                    .icon(action.resolved_icon().to_string())
                    .aria_label(action.label.clone())
                    .tooltip(action.label.clone())
                    .disabled(actions_disabled || action.is_disabled)
                    .with_id(format!("bulk-action-{}", action.id));

                if let Some(ref handler) = self.on_action {
                    let handler = handler.clone();
                    let action_id = action.id.clone();
                    btn = btn.on_click(move |_event, window, cx| handler(&action_id, window, cx));
                }

                row = row.child(btn);
            }

            // Clear-selection (`x`) ghost IconButton — contract §2.
            let mut clear_btn = IconButton::new(theme)
                .variant(ButtonVariant::Ghost)
                .size(effective_size)
                .icon("x")
                .aria_label("Clear selection")
                .tooltip("Clear selection")
                .disabled(is_unavailable)
                .with_id("bulk-clear");

            if let Some(ref handler) = self.on_clear {
                let handler = handler.clone();
                clear_btn = clear_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }

            row = row.child(clear_btn);
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
