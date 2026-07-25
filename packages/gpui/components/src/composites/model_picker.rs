//! ModelPicker — GPUI model + capability-axis picker backed by ModelPickerSpec.
//!
//! Contract: `docs/contracts/components/model-picker.md`
//! Reference: `packages/svelte/components/src/ModelPicker.svelte`
//!
//! One trigger (optional model icon + label + axis summary + chevron) over an
//! anchored dialog holding the model rows and one section per applicable axis.
//! GPUI has no ARIA channel and no anchored-popover positioning — the surface
//! renders inline below the trigger, and interaction (open/close, model choice,
//! axis edits) lives in the host event loop. Build-verified only.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ChoiceOption, ControlDensity, ControlSize, IconSize, IconSpec, ModelAxisControlKind,
    ModelAxisKind, ModelAxisValue, ModelPickerSpec, ModelPickerVariant, SegmentedControlSpec,
    SemanticControlSizeRole, SwitchSpec,
};

use super::super::primitives::{Icon, SegmentedControl, Switch};
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};

pub struct ModelPicker {
    spec: ModelPickerSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for ModelPicker {
    type Target = ModelPickerSpec;
    fn deref(&self) -> &ModelPickerSpec {
        &self.spec
    }
}

impl ModelPicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(ModelPickerSpec::new(), theme)
    }

    pub fn from_spec(spec: ModelPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
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

    pub fn with_open(mut self, v: bool) -> Self {
        self.spec.is_open = v;
        self
    }
}

impl IntoElement for ModelPicker {
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
        let trigger_font = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.8125,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 0.9375,
            ControlSize::Xl => 1.0,
        }));
        let trigger_gap = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        }));

        // ── Colors ────────────────────────────────────────────────────────────
        let text_primary = resolve_color(theme, spec.label_color_token());
        let text_secondary = resolve_color(theme, spec.secondary_color_token());
        let muted = resolve_color(theme, spec.muted_color_token());
        let border = resolve_color(theme, spec.trigger_border_token());
        let item_border = resolve_color(theme, spec.item_border_token());
        let surface = resolve_color(theme, spec.trigger_fill_token());
        let elevated = resolve_color(theme, spec.surface_fill_token());
        let accent = resolve_color(theme, spec.selected_color_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let surface_radius = resolve_radius(theme, spec.surface_radius_token());
        let row_selected_bg = color_mix(elevated, accent, 0.86);
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        // ── Trigger ───────────────────────────────────────────────────────────
        let mut trigger = div()
            .flex()
            .items_center()
            .gap(trigger_gap)
            .min_h(trigger_height)
            .px(px(rem_to_px(0.375)))
            .rounded(radius);

        if spec.variant == ModelPickerVariant::Outlined {
            trigger = trigger.border_1().border_color(border).bg(surface);
        }

        // An arbitrary image (a provider logo) wins over a registry icon name.
        if let Some(image) = spec.selected_model().and_then(|model| model.image.clone()) {
            trigger = trigger.child(
                img(image.src.clone())
                    .w(trigger_font)
                    .h(trigger_font)
                    .flex_none(),
            );
        } else if let Some(icon) = spec.selected_model().and_then(|model| model.icon.clone()) {
            trigger = trigger.child(
                Icon::from_spec(IconSpec::new(icon).with_size(IconSize::Sm), theme)
                    .with_color(text_secondary),
            );
        }

        // Subdued emphasis dims the resting trigger so the picker recedes beside
        // a more important control; hover/focus restoration is web-only
        // (contract §12).
        let subdued_opacity = if spec.emphasis.is_subdued() {
            resolve_opacity(theme, spec.trigger_subdued_opacity_token())
        } else {
            1.0
        };
        let label_color = if spec.has_selection() {
            resolve_color(theme, spec.trigger_label_color_token())
        } else {
            muted
        };
        let dim = |color: Hsla| Hsla {
            a: color.a * subdued_opacity,
            ..color
        };
        trigger = trigger.child(
            div()
                .text_size(trigger_font)
                .text_color(label_color)
                .child(spec.trigger_label()),
        );

        let summary = spec.summary_text();
        if !summary.is_empty() {
            trigger = trigger
                .child(
                    div()
                        .w(px(rem_to_px(0.0625)))
                        .h(trigger_font)
                        .flex_none()
                        .bg(item_border),
                )
                .child(
                    div()
                        .text_size(trigger_font)
                        .text_color(dim(text_secondary))
                        .child(summary),
                );
        }

        trigger = trigger.child(
            div()
                .text_size(trigger_font)
                .text_color(dim(text_secondary))
                .child("▾"),
        );

        let mut root = div()
            .flex()
            .flex_col()
            .gap(px(rem_to_px(0.5)))
            .child(trigger);

        // ── Dialog surface (rendered inline when open) ────────────────────────
        if spec.is_open {
            // Two columns (models | axes) whenever the selected model has
            // applicable axes; a plain list otherwise (contract §7).
            let applicable = spec.applicable_axes();
            let is_split = !applicable.is_empty();

            let mut models = div()
                .flex()
                .flex_col()
                .flex_grow()
                .gap(px(rem_to_px(0.125)));
            for (index, model) in spec.models.iter().enumerate() {
                if let Some(heading) = spec.group_heading_for(index) {
                    models = models.child(
                        div()
                            .flex_none()
                            .pl(px(rem_to_px(0.5)))
                            // Space above every heading but the first, so group
                            // runs read as sections.
                            .pt(px(rem_to_px(if index == 0 { 0.5 } else { 0.875 })))
                            .pb(px(rem_to_px(0.25)))
                            .text_size(px(rem_to_px(0.6875)))
                            .text_color(text_secondary)
                            .child(heading.to_string()),
                    );
                }

                let is_selected = model.value == spec.value.model;
                // Never shrink: the list is height-capped and scrolls, so a
                // shrinkable row would squash below its own content.
                let mut row = div()
                    .flex()
                    .flex_none()
                    .items_start()
                    .gap(px(rem_to_px(0.5)))
                    .px(px(rem_to_px(0.5)))
                    .py(px(rem_to_px(0.375)))
                    .rounded(radius);

                if is_selected {
                    row = row.bg(row_selected_bg);
                }
                if model.is_disabled {
                    row = row.opacity(disabled_opacity);
                }

                if let Some(image) = &model.image {
                    row = row.child(
                        img(image.src.clone())
                            .w(px(rem_to_px(1.0)))
                            .h(px(rem_to_px(1.0)))
                            .flex_none(),
                    );
                } else if let Some(icon) = &model.icon {
                    row = row.child(
                        Icon::from_spec(
                            IconSpec::new(icon.clone()).with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(text_secondary),
                    );
                }

                let mut text = div().flex().flex_col().flex_grow().child(
                    div()
                        .text_size(px(rem_to_px(0.875)))
                        .text_color(text_primary)
                        .child(model.label.clone()),
                );

                if spec.show_model_descriptions {
                    if let Some(description) = &model.description {
                        text = text.child(
                            div()
                                .text_size(px(rem_to_px(0.75)))
                                .text_color(text_secondary)
                                .child(description.clone()),
                        );
                    }
                }
                row = row.child(text);

                if let Some(badge) = &model.badge {
                    row = row.child(
                        div()
                            .flex_none()
                            .px(px(rem_to_px(0.375)))
                            .rounded(px(rem_to_px(0.5)))
                            .border_1()
                            .border_color(item_border)
                            .text_size(px(rem_to_px(0.6875)))
                            .text_color(text_secondary)
                            .child(badge.clone()),
                    );
                }

                if is_selected {
                    row = row.child(
                        Icon::from_spec(IconSpec::new("check").with_size(IconSize::Sm), theme)
                            .with_color(accent),
                    );
                }

                models = models.child(row);
            }

            // One section per applicable axis, in declaration order, stacked in
            // the right-hand column.
            let mut axes_column = div()
                .flex()
                .flex_col()
                .gap(px(rem_to_px(0.75)))
                .w(px(rem_to_px(13.0)))
                .flex_none()
                .pl(px(rem_to_px(0.75)))
                .border_l_1()
                .border_color(item_border);

            for (index, axis) in applicable.iter().enumerate() {
                let current = spec.axis_value(axis);
                let mut section = div().flex().flex_col().gap(px(rem_to_px(0.375)));
                // The column rule already separates the axes from the list, so
                // only sections after the first carry a top rule.
                if index > 0 {
                    section = section.pt(px(rem_to_px(0.5))).border_t_1().border_color(item_border);
                }
                section = section.child(
                        div()
                            .text_size(px(rem_to_px(0.6875)))
                            .text_color(text_secondary)
                            .child(axis.label.clone()),
                    );

                if let Some(description) = &axis.description {
                    section = section.child(
                        div()
                            .text_size(px(rem_to_px(0.75)))
                            .text_color(text_secondary)
                            .child(description.clone()),
                    );
                }

                section = match axis.kind {
                    // Many-level scales render as a vertical list; short ones
                    // stay segmented (contract §4).
                    ModelAxisKind::Select if axis.control_kind() == ModelAxisControlKind::List => {
                        let selected = current.as_text().unwrap_or_default().to_string();
                        let mut list = div().flex().flex_col().gap(px(rem_to_px(0.0625)));
                        for option in axis.options.iter() {
                            let is_selected = option.value == selected;
                            let mut row = div()
                                .flex()
                                .items_center()
                                .gap(px(rem_to_px(0.5)))
                                .px(px(rem_to_px(0.375)))
                                .py(px(rem_to_px(0.25)))
                                .rounded(radius)
                                .child(
                                    div()
                                        .flex_grow()
                                        .text_size(px(rem_to_px(0.8125)))
                                        .text_color(if is_selected {
                                            text_primary
                                        } else {
                                            text_secondary
                                        })
                                        .child(option.label.clone()),
                                );
                            if is_selected {
                                row = row.child(
                                    Icon::from_spec(
                                        IconSpec::new("check").with_size(IconSize::Sm),
                                        theme,
                                    )
                                    .with_color(accent),
                                );
                            }
                            if option.is_disabled {
                                row = row.opacity(disabled_opacity);
                            }
                            list = list.child(row);
                        }
                        section.child(list)
                    }
                    ModelAxisKind::Select => {
                        let options: Vec<ChoiceOption> = axis
                            .options
                            .iter()
                            .map(|option| {
                                ChoiceOption::new(option.value.clone(), option.label.clone())
                            })
                            .collect();
                        section.child(
                            SegmentedControl::from_spec(
                                SegmentedControlSpec::new(options),
                                theme,
                            )
                            .value(current.as_text().unwrap_or_default())
                            .size(effective_size)
                            .density(spec.density)
                            .disabled(spec.is_disabled || axis.is_disabled),
                        )
                    }
                    ModelAxisKind::Toggle => section.child(
                        Switch::from_spec(SwitchSpec::new(), theme)
                            .checked(matches!(current, ModelAxisValue::Flag(true)))
                            .size(effective_size)
                            .density(spec.density)
                            .disabled(spec.is_disabled || axis.is_disabled),
                    ),
                };

                axes_column = axes_column.child(section);
            }

            // Stretch (the flex default), not items_start: the rail's left rule
            // must run the panel's full height.
            let mut panel = div().flex().gap(px(rem_to_px(0.75))).child(models);
            if is_split {
                panel = panel.child(axes_column);
            }

            root = root.child(
                div()
                    // The split layout needs room for both columns (contract §7).
                    .min_w(px(rem_to_px(if is_split { 32.0 } else { 18.0 })))
                    .max_w(px(rem_to_px(if is_split { 40.0 } else { 26.0 })))
                    .p(px(rem_to_px(0.5)))
                    .rounded(surface_radius)
                    .border_1()
                    .border_color(item_border)
                    .bg(elevated)
                    .child(panel),
            );
        }

        if spec.is_disabled {
            root = root.opacity(disabled_opacity);
        }

        root.into_any_element()
    }
}
