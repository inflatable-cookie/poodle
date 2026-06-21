use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonVariant, ChoiceOption, ControlDensity, ControlSize, IconButtonSpec, IconSize, IconSpec,
    OrderBySpec, SelectSpec, SemanticControlSizeRole, SortDirection,
};

use super::{Icon, IconButton, Select};
use crate::presentation::{rem_to_px, resolve_semantic_size, size_padding_x_offset_rem};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

/// OrderBy — anchored multi-field sort builder backed by `OrderBySpec`.
///
/// Contract: `docs/contracts/components/order-by.md`. Mirrors the corrected
/// contract (reconciled against `OrderBy.svelte`): single-row items (drag
/// handle + label + direction-toggle IconButton + remove IconButton), a ghost
/// `×` reset IconButton, "No sort fields" empty text, and an add-field `Select`.
/// The move-up/move-down buttons and the footer "Clear all" button are gone.
/// GPUI has no ARIA channel and no anchored-popover positioning — the surface
/// renders inline below the trigger; interaction (open/select/drag/Alt-arrow)
/// lives in the host event loop. Build-verified only.
pub struct OrderBy {
    spec: OrderBySpec,
    theme: GpuiThemeProvider,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for OrderBy {
    type Target = OrderBySpec;
    fn deref(&self) -> &OrderBySpec {
        &self.spec
    }
}

impl OrderBy {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(OrderBySpec::new(), theme)
    }

    pub fn from_spec(spec: OrderBySpec, theme: &GpuiThemeProvider) -> Self {
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

impl IntoElement for OrderBy {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size table (corrected contract §8) ────────────────────────────────
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
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let surface_radius = resolve_radius(theme, spec.surface_radius_token());
        // Item bg = color-mix(surface 90%, elevated).
        let item_bg = color_mix(surface, elevated, 0.90);

        // ── Trigger ───────────────────────────────────────────────────────────
        let mut trigger = div()
            .min_w(px(rem_to_px(0.0)))
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

        if !spec.compact {
            trigger = trigger.child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child("SORT BY"),
            );
        }

        let summary_is_placeholder = !spec.has_value();
        trigger = trigger
            .child(
                div()
                    .flex_grow()
                    .min_w(px(0.0))
                    .text_size(summary_size)
                    .text_color(if summary_is_placeholder {
                        muted
                    } else {
                        text_primary
                    })
                    .child(spec.summary_text()),
            )
            .child(
                Icon::from_spec(IconSpec::new("chevron-down").with_size(IconSize::Sm), theme)
                    .with_color(text_secondary),
            );

        let mut root = div()
            .id("order-by")
            .flex()
            .flex_col()
            .gap(px(rem_to_px(0.375)));

        let mut trigger_row = div()
            .flex()
            .items_center()
            .gap(px(rem_to_px(0.375)))
            .child(trigger);

        // Reset = ghost IconButton (no bespoke square chrome).
        if spec.has_value() {
            let mut reset = IconButton::from_spec(
                IconButtonSpec::new()
                    .with_icon("x")
                    .with_aria_label("Clear sort")
                    .with_variant(ButtonVariant::Ghost)
                    .with_size(effective_size)
                    .with_disabled(spec.is_disabled),
                theme,
            )
            .with_id("order-by-reset");

            if let Some(handler) = self.on_reset {
                reset = reset.on_click(move |event, window, cx| handler(event, window, cx));
            }
            trigger_row = trigger_row.child(reset);
        }

        root = root.child(trigger_row);

        // ── Dialog surface (rendered inline when open) ────────────────────────
        if spec.is_open {
            let current_value = spec.current_value();
            let mut panel = div().flex().flex_col().gap(px(rem_to_px(0.375)));

            if current_value.is_empty() {
                panel = panel.child(
                    div()
                        .text_size(px(rem_to_px(0.75)))
                        .text_color(text_secondary)
                        .child("No sort fields"),
                );
            } else {
                let mut list = div().flex().flex_col().gap(px(rem_to_px(0.25)));
                for item in current_value.iter() {
                    let field_label = spec.active_label(&item.key);
                    let (dir_icon, dir_tooltip, dir_word) = match item.direction {
                        SortDirection::Asc => ("arrow-up", "Asc", "ascending"),
                        SortDirection::Desc => ("arrow-down", "Desc", "descending"),
                    };

                    let row = div()
                        .flex()
                        .items_center()
                        .gap(px(rem_to_px(0.375)))
                        .px(px(rem_to_px(0.5)))
                        .py(px(rem_to_px(0.3125)))
                        .rounded(radius)
                        .border_1()
                        .border_color(item_border)
                        .bg(item_bg)
                        // Drag handle (braille glyph; carries the reorder affordance).
                        .child(
                            div()
                                .min_w(px(rem_to_px(1.5)))
                                .min_h(px(rem_to_px(1.5)))
                                .flex_shrink_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(rem_to_px(0.75)))
                                .text_color(muted)
                                .child("⠿"),
                        )
                        // Field label (single-line).
                        .child(
                            div()
                                .flex_grow()
                                .min_w(px(0.0))
                                .text_size(px(rem_to_px(0.8125)))
                                .text_color(text_primary)
                                .child(field_label.clone()),
                        )
                        // Direction toggle (xs ghost IconButton).
                        .child(IconButton::from_spec(
                            IconButtonSpec::new()
                                .with_icon(dir_icon)
                                .with_aria_label(format!(
                                    "{field_label}: {dir_word}. Click to toggle."
                                ))
                                .with_tooltip(dir_tooltip)
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(ControlSize::Xs)
                                .with_disabled(spec.is_disabled),
                            theme,
                        ))
                        // Remove (xs ghost IconButton, no danger tone).
                        .child(IconButton::from_spec(
                            IconButtonSpec::new()
                                .with_icon("x")
                                .with_aria_label(format!("Remove {field_label}"))
                                .with_tooltip("Remove")
                                .with_variant(ButtonVariant::Ghost)
                                .with_size(ControlSize::Xs)
                                .with_disabled(spec.is_disabled),
                            theme,
                        ));

                    list = list.child(row);
                }
                panel = panel.child(list);
            }

            // Add-field Select.
            let can_add_more = spec
                .max_fields
                .map(|max| spec.active_count() < max)
                .unwrap_or(true);
            if can_add_more && !spec.available_fields().is_empty() {
                let options: Vec<ChoiceOption> = spec
                    .available_fields()
                    .into_iter()
                    .map(|field| {
                        ChoiceOption::new(field.resolved_key().to_string(), field.label.clone())
                    })
                    .collect();
                let select = Select::from_spec(SelectSpec::new(options), theme)
                    .placeholder("+ Add field")
                    .aria_label("Add sort field")
                    .size(effective_size)
                    .density(spec.density)
                    .disabled(spec.is_disabled);
                panel = panel.child(div().flex().items_center().child(select));
            }

            root = root.child(
                div()
                    .min_w(px(rem_to_px(14.0)))
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
