use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonVariant, ChoiceOption, ControlDensity, ControlSize, FilterBuilderSpec, FilterCombinator,
    IconButtonSpec, IconSize, IconSpec, SelectSpec, SemanticControlSizeRole,
};

use super::{Icon, IconButton, Select};
use crate::presentation::{rem_to_px, resolve_semantic_size, size_padding_x_offset_rem};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

/// FilterBuilder — anchored filter-clause builder backed by `FilterBuilderSpec`.
///
/// Contract: `docs/contracts/components/filter-builder.md`. Companion to OrderBy:
/// a Filter trigger with active-clause count, editable/removable clause pills, and
/// an anchored surface with the `Match all` / `Match any` combinator plus an
/// add-field `Select`. GPUI has no ARIA channel and no anchored-popover
/// positioning — the surface renders inline below the trigger; interactive draft
/// editing (field → operator → operand → Add) lives in the host event loop.
/// Build-verified only.
pub struct FilterBuilder {
    spec: FilterBuilderSpec,
    theme: GpuiThemeProvider,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for FilterBuilder {
    type Target = FilterBuilderSpec;
    fn deref(&self) -> &FilterBuilderSpec {
        &self.spec
    }
}

impl FilterBuilder {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(FilterBuilderSpec::new(), theme)
    }

    pub fn from_spec(spec: FilterBuilderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_reset: None,
        }
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

    pub fn on_reset(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_reset = Some(Box::new(handler));
        self
    }
}

impl IntoElement for FilterBuilder {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size table (contract §8) ──────────────────────────────────────────
        let trigger_height = px(rem_to_px(match effective_size {
            ControlSize::Xs => 1.5,
            ControlSize::Sm => 1.75,
            ControlSize::Md => 2.25,
            ControlSize::Lg => 2.75,
            ControlSize::Xl => 3.25,
        }));
        let label_size = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.5625,
            ControlSize::Sm => 0.625,
            ControlSize::Md => 0.75,
            ControlSize::Lg => 0.8125,
            ControlSize::Xl => 0.875,
        }));
        let summary_size = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.8125,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 0.9375,
            ControlSize::Xl => 1.0,
        }));
        let trigger_pad_x = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.5,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.125,
            _ => 0.75 + size_padding_x_offset_rem(effective_size),
        }));
        let trigger_gap = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => 0.5,
            ControlDensity::Comfortable => 0.625,
        }));

        // ── Colors ────────────────────────────────────────────────────────────
        let text_primary = resolve_color(theme, spec.field_text_token());
        let text_secondary = resolve_color(theme, spec.label_color_token());
        let muted = resolve_color(theme, spec.muted_color_token());
        let border = resolve_color(theme, spec.field_border_token());
        let item_border = resolve_color(theme, spec.item_border_token());
        let surface = resolve_color(theme, spec.field_fill_token());
        let elevated = resolve_color(theme, spec.field_hover_fill_token());
        let accent = resolve_color(theme, spec.count_fill_token());
        let accent_text = resolve_color(theme, spec.count_text_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let surface_radius = resolve_radius(theme, spec.surface_radius_token());
        let item_bg = color_mix(surface, elevated, 0.90);

        // ── Trigger ───────────────────────────────────────────────────────────
        let mut trigger = div()
            .min_w(px(0.0))
            .max_w(px(rem_to_px(28.0)))
            .h(trigger_height)
            .px(trigger_pad_x)
            .rounded(radius)
            .border_1()
            .border_color(border)
            .bg(surface)
            .flex()
            .items_center()
            .gap(trigger_gap)
            .text_color(text_primary);

        if !spec.is_compact {
            trigger = trigger.child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child("FILTER"),
            );
        }

        let is_placeholder = !spec.has_value();
        trigger = trigger.child(
            div()
                .flex_grow()
                .min_w(px(0.0))
                .text_size(summary_size)
                .text_color(if is_placeholder { muted } else { text_primary })
                .child(spec.summary_text()),
        );

        if spec.active_count() > 0 {
            trigger = trigger.child(
                div()
                    .flex_shrink_0()
                    .min_w(px(rem_to_px(1.125)))
                    .h(px(rem_to_px(1.125)))
                    .px(px(rem_to_px(0.3125)))
                    .rounded(px(999.0))
                    .bg(accent)
                    .text_color(accent_text)
                    .text_size(px(rem_to_px(0.6875)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(format!("{}", spec.active_count())),
            );
        }

        trigger = trigger.child(
            Icon::from_spec(IconSpec::new("chevron-down").with_size(IconSize::Sm), theme)
                .with_color(text_secondary),
        );

        let mut root = div()
            .id("filter-builder")
            .flex()
            .flex_col()
            .gap(px(rem_to_px(0.375)));

        let mut trigger_row = div()
            .flex()
            .items_center()
            .gap(px(rem_to_px(0.375)))
            .child(trigger);

        if spec.show_clear_button && spec.has_value() {
            let mut reset = IconButton::from_spec(
                IconButtonSpec::new()
                    .with_icon("x")
                    .with_aria_label("Clear filters")
                    .with_variant(ButtonVariant::Ghost)
                    .with_size(effective_size)
                    .with_disabled(spec.is_disabled),
                theme,
            )
            .with_id("filter-builder-reset");

            if let Some(handler) = self.on_reset {
                reset = reset.on_click(move |event, window, cx| handler(event, window, cx));
            }
            trigger_row = trigger_row.child(reset);
        }

        root = root.child(trigger_row);

        // ── Clause pills (editable/removable; interaction host-owned) ──────────
        if spec.show_pills && spec.has_value() {
            let mut pills = div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(rem_to_px(0.375)));
            for clause in spec.value.clauses.iter() {
                pills = pills.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(rem_to_px(0.25)))
                        .px(px(rem_to_px(0.5)))
                        .h(px(rem_to_px(1.5)))
                        .rounded(radius)
                        .border_1()
                        .border_color(item_border)
                        .bg(item_bg)
                        .text_size(px(rem_to_px(0.75)))
                        .text_color(text_primary)
                        .child(spec.clause_label(clause))
                        .child(IconButton::from_spec(
                            IconButtonSpec::new()
                                .with_icon("x")
                                .with_aria_label(format!("Remove {}", spec.clause_label(clause)))
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(ControlSize::Xs)
                                .with_disabled(spec.is_disabled),
                            theme,
                        )),
                );
            }
            root = root.child(pills);
        }

        // ── Anchored surface (rendered inline when open) ──────────────────────
        if spec.is_open {
            let mut panel = div().flex().flex_col().gap(px(rem_to_px(0.5)));

            // Combinator (shown with 2+ clauses) — static two-option indicator.
            if spec.combinator_visible() {
                let mk = |label: &str, selected: bool| {
                    div()
                        .flex_grow()
                        .px(px(rem_to_px(0.5)))
                        .py(px(rem_to_px(0.25)))
                        .rounded(radius)
                        .bg(if selected { accent } else { surface })
                        .text_color(if selected { accent_text } else { text_secondary })
                        .text_size(px(rem_to_px(0.75)))
                        .flex()
                        .justify_center()
                        .child(label.to_string())
                };
                let is_and = spec.value.combinator == FilterCombinator::And;
                panel = panel.child(
                    div()
                        .flex()
                        .gap(px(rem_to_px(0.25)))
                        .pb(px(rem_to_px(0.5)))
                        .border_b_1()
                        .border_color(item_border)
                        .child(mk("Match all", is_and))
                        .child(mk("Match any", !is_and)),
                );
            }

            // Add-field Select (fields still available).
            let available = spec.available_fields();
            if spec.can_add_more() && !available.is_empty() {
                let options: Vec<ChoiceOption> = available
                    .into_iter()
                    .map(|field| ChoiceOption::new(field.key.clone(), field.label.clone()))
                    .collect();
                let select = Select::from_spec(SelectSpec::new(options), theme)
                    .placeholder("+ Add filter")
                    .aria_label("Add filter field")
                    .size(effective_size)
                    .density(spec.density)
                    .disabled(spec.is_disabled);
                panel = panel.child(div().flex().items_center().child(select));
            }

            if !spec.has_value() {
                panel = panel.child(
                    div()
                        .text_size(px(rem_to_px(0.75)))
                        .text_color(text_secondary)
                        .child("No filters"),
                );
            }

            root = root.child(
                div()
                    .min_w(px(rem_to_px(16.0)))
                    .max_w(px(rem_to_px(24.0)))
                    .rounded(surface_radius)
                    .border_1()
                    .border_color(item_border)
                    .bg(elevated)
                    .p(px(rem_to_px(0.375)))
                    .child(panel),
            );
        }

        if spec.is_disabled {
            root = root.opacity(disabled_opacity);
        }

        let _ = focus_ring;
        root.into_any_element()
    }
}
