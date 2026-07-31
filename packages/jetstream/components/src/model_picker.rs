//! ModelPicker — Jetstream model + capability-axis picker backed by
//! ModelPickerSpec.
//!
//! Contract: `docs/contracts/components/model-picker.md`
//! Reference: `packages/svelte/components/src/ModelPicker.svelte`
//!
//! Anatomy: root → trigger (optional model icon + label + axis summary +
//! chevron) → dialog surface (model rows with group headings, badges and a
//! selection mark, then one section per applicable axis).
//!
//! Interaction (open/close, model choice, axis edits) lives in the preview
//! event loop, not the component — render-only, build/probe-verified.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    ChoiceOption, ControlSize, ModelAxisControlKind, ModelAxisKind, ModelAxisValue,
    ModelPickerSpec, ModelPickerVariant, SegmentedControlSpec, SwitchSpec,
};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::segmented_control::js_segmented_control;
use crate::switch::js_switch;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius, tint};

/// ModelPicker — a model list with axis options.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_change(handler)`.
pub struct ModelPicker {
    spec: ModelPickerSpec,
    theme: JetstreamThemeProvider,
    on_change: Option<crate::element::Handler>,
}

impl ModelPicker {
    pub fn from_spec(spec: ModelPickerSpec, theme: &JetstreamThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_change: None }
    }

    /// Fires with the chosen option's value.
    pub fn on_change(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_change = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for ModelPicker {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_change)
    }
}

pub fn js_model_picker(spec: &ModelPickerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None)
}

fn build(
    spec: &ModelPickerSpec,
    theme: &JetstreamThemeProvider,
    on_change: Option<crate::element::Handler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Size table (contract §8) ──────────────────────────────────────────────
    let trigger_h = rem_to_px(match effective_size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.25,
    });
    let trigger_font = rem_to_px(match effective_size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    });
    let trigger_gap = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.25,
        poodle_specs::ControlDensity::Default => 0.375,
        poodle_specs::ControlDensity::Comfortable => 0.5,
    });

    // ── Colors ────────────────────────────────────────────────────────────────
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

    // ── Trigger ───────────────────────────────────────────────────────────────
    let mut trigger = ui_element::div()
        .flex_row()
        .items_center()
        .gap(trigger_gap)
        .min_w(0.0)
        .min_h(trigger_h)
        .pl(rem_to_px(0.375))
        .pr(rem_to_px(0.375))
        .rounded(radius);

    if spec.variant == ModelPickerVariant::Outlined {
        trigger = trigger.border_1().border_color(border).bg(surface);
    }

    // An arbitrary image (a provider logo) wins over a registry icon name.
    if let Some(image) = spec.selected_model().and_then(|model| model.image.clone()) {
        trigger = trigger.child(
            ui_element::image(&image.src)
                .w(trigger_font)
                .h(trigger_font)
                .flex_none()
                .object_fit_contain(),
        );
    } else if let Some(icon) = spec.selected_model().and_then(|model| model.icon.clone()) {
        trigger = trigger.child(
            ui_element::icon(&icon)
                .w(trigger_font)
                .h(trigger_font)
                .text_color(text_secondary),
        );
    }

    // Subdued emphasis dims the resting trigger so the picker recedes beside a
    // more important control; hover/focus restoration is web-only (contract §12).
    let label_color = if spec.has_selection() {
        resolve_color(theme, spec.trigger_label_color_token())
    } else {
        muted
    };
    let subdued_opacity = if spec.emphasis.is_subdued() {
        resolve_opacity(theme, spec.trigger_subdued_opacity_token())
    } else {
        1.0
    };
    trigger = trigger.child(
        ui_element::label(&spec.trigger_label())
            .text_color(label_color)
            .text_size(trigger_font)
            .text_weight(if spec.has_selection() { 500 } else { 400 })
            .text_ellipsis()
            .whitespace_nowrap(),
    );

    let summary = spec.summary_text();
    if !summary.is_empty() {
        // The hairline before the summary is the web's ::before rule.
        trigger = trigger
            .child(
                ui_element::div()
                    .w(rem_to_px(0.0625))
                    .h(trigger_font)
                    .flex_none()
                    .bg(item_border),
            )
            .child(
                ui_element::label(&summary)
                    .text_color(tint(text_secondary, subdued_opacity))
                    .text_size(trigger_font)
                    .text_ellipsis()
                    .whitespace_nowrap(),
            );
    }

    trigger = trigger.child(
        ui_element::icon("chevron-down")
            .w(trigger_font)
            .h(trigger_font)
            .text_color(tint(text_secondary, subdued_opacity)),
    );

    let mut root = ui_element::div()
        .flex_col()
        .gap(rem_to_px(0.5))
        .min_w(0.0)
        .child(trigger);

    // ── Dialog surface (rendered inline when open) ────────────────────────────
    if spec.is_open {
        // Two columns (models | axes) whenever the selected model has applicable
        // axes; a plain list otherwise (contract §7).
        let applicable = spec.applicable_axes();
        let is_split = !applicable.is_empty();

        let mut models = ui_element::div().flex_col().grow().min_w(0.0).gap(rem_to_px(0.125));
        for (index, model) in spec.models.iter().enumerate() {
            if let Some(heading) = spec.group_heading_for(index) {
                models = models.child(
                    ui_element::label(heading)
                        .flex_none()
                        .text_color(text_secondary)
                        .text_size(rem_to_px(0.6875))
                        .text_weight(500)
                        .letter_spacing_em(0.05)
                        .pl(rem_to_px(0.5))
                        // Space above every heading but the first, so group runs
                        // read as sections.
                        .pt(rem_to_px(if index == 0 { 0.5 } else { 0.875 }))
                        .pb(rem_to_px(0.25)),
                );
            }

            let is_selected = model.value == spec.value.model;
            // Never shrink: the list is height-capped and scrolls, so a
            // shrinkable row would squash below its own content.
            let mut row = ui_element::div()
                .flex_none()
                .flex_row()
                .items_start()
                .gap(rem_to_px(0.5))
                .pl(rem_to_px(0.5))
                .pr(rem_to_px(0.5))
                .pt(rem_to_px(0.375))
                .pb(rem_to_px(0.375))
                .rounded(radius);

            if is_selected {
                row = row.bg(row_selected_bg);
            }

            if let Some(image) = &model.image {
                row = row.child(
                    ui_element::image(&image.src)
                        .w(rem_to_px(1.0))
                        .h(rem_to_px(1.0))
                        .flex_none()
                        .object_fit_contain(),
                );
            } else if let Some(icon) = &model.icon {
                row = row.child(
                    ui_element::icon(icon)
                        .w(rem_to_px(0.875))
                        .h(rem_to_px(0.875))
                        .text_color(text_secondary),
                );
            }

            let mut text = ui_element::div().flex_col().grow().min_w(0.0).child(
                ui_element::label(&model.label)
                    .text_color(text_primary)
                    .text_size(rem_to_px(0.875))
                    .text_weight(if is_selected { 600 } else { 400 })
                    .text_ellipsis()
                    .whitespace_nowrap(),
            );

            if spec.show_model_descriptions {
                if let Some(description) = &model.description {
                    text = text.child(
                        ui_element::label(description)
                            .text_color(text_secondary)
                            .text_size(rem_to_px(0.75)),
                    );
                }
            }
            row = row.child(text);

            if let Some(badge) = &model.badge {
                row = row.child(
                    ui_element::label(badge)
                        .flex_none()
                        .pl(rem_to_px(0.375))
                        .pr(rem_to_px(0.375))
                        .rounded(rem_to_px(0.5))
                        .border_1()
                        .border_color(item_border)
                        .text_color(text_secondary)
                        .text_size(rem_to_px(0.6875)),
                );
            }

            if is_selected {
                row = row.child(
                    ui_element::icon("check")
                        .w(rem_to_px(0.75))
                        .h(rem_to_px(0.75))
                        .text_color(accent),
                );
            }

            if model.is_disabled {
                row = row.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
            } else if let Some(handler) = &on_change {
                let handler = std::sync::Arc::clone(handler);
                let id = model.value.clone();
                row = row.cursor_pointer().on_click(move |_event| handler(&id));
            }

            models = models.child(row);
        }

        // One section per applicable axis, in declaration order, stacked in the
        // right-hand column.
        let mut axes_column = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.75))
            .w(rem_to_px(13.0))
            .flex_none()
            .pl(rem_to_px(0.75))
            .border_l(rem_to_px(0.0625))
            .border_color(item_border);

        for (index, axis) in applicable.iter().enumerate() {
            let current = spec.axis_value(axis);
            let mut section = ui_element::div().flex_col().gap(rem_to_px(0.375));
            // The column rule already separates the axes from the list, so only
            // sections after the first carry a top rule.
            if index > 0 {
                section = section
                    .pt(rem_to_px(0.5))
                    .border_t_1()
                    .border_color(item_border);
            }
            section = section.child(
                ui_element::label(&axis.label)
                    .text_color(text_secondary)
                    .text_size(rem_to_px(0.6875))
                    .text_weight(500)
                    .letter_spacing_em(0.05),
            );

            if let Some(description) = &axis.description {
                section = section.child(
                    ui_element::label(description)
                        .text_color(text_secondary)
                        .text_size(rem_to_px(0.75)),
                );
            }

            section = match axis.kind {
                // Many-level scales render as a vertical list; short ones stay
                // segmented (contract §4).
                ModelAxisKind::Select if axis.control_kind() == ModelAxisControlKind::List => {
                    let selected = current.as_text().unwrap_or_default().to_string();
                    let mut list = ui_element::div().flex_col().gap(rem_to_px(0.0625)).min_w(0.0);
                    for option in axis.options.iter() {
                        let is_selected = option.value == selected;
                        let mut row = ui_element::div()
                            // Contract: axis options are mutually exclusive, so
                            // each is a `radio` with its own checked state.
                            .aria_role(jetstream_ui::accesskit::Role::RadioButton)
                            .aria_label(option.label.clone())
                            .aria_checked(crate::aria::toggled(Some(is_selected)))
                            .flex_row()
                            .items_center()
                            .gap(rem_to_px(0.5))
                            .pl(rem_to_px(0.375))
                            .pr(rem_to_px(0.375))
                            .pt(rem_to_px(0.25))
                            .pb(rem_to_px(0.25))
                            .rounded(radius)
                            .child(
                                ui_element::label(&option.label)
                                    .grow()
                                    .min_w(0.0)
                                    .text_color(if is_selected { text_primary } else { text_secondary })
                                    .text_size(rem_to_px(0.8125))
                                    .text_weight(if is_selected { 600 } else { 400 })
                                    .text_ellipsis()
                                    .whitespace_nowrap(),
                            );
                        if is_selected {
                            row = row.child(
                                ui_element::icon("check")
                                    .w(rem_to_px(0.75))
                                    .h(rem_to_px(0.75))
                                    .text_color(accent),
                            );
                        }
                        if option.is_disabled {
                            row = row.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
                        }
                        list = list.child(row);
                    }
                    section.child(list)
                }
                ModelAxisKind::Select => {
                    let options: Vec<ChoiceOption> = axis
                        .options
                        .iter()
                        .map(|option| ChoiceOption::new(option.value.clone(), option.label.clone()))
                        .collect();
                    let mut control = SegmentedControlSpec::new(options)
                        .with_size(effective_size)
                        .with_density(spec.density);
                    control.value = current.as_text().map(|value| value.to_string());
                    control.is_disabled = spec.is_disabled || axis.is_disabled;
                    section.child(js_segmented_control(&control, theme))
                }
                ModelAxisKind::Toggle => {
                    let mut control = SwitchSpec::new()
                        // The axis names itself; the switch renders bare, so
                        // without this it is one of several unnamed toggles.
                        .with_aria_label(axis.label.clone())
                        .with_checked(matches!(current, ModelAxisValue::Flag(true)))
                        .with_size(effective_size)
                        .with_density(spec.density);
                    control.is_disabled = spec.is_disabled || axis.is_disabled;
                    section.child(js_switch(&control, theme))
                }
            };

            axes_column = axes_column.child(section);
        }

        // Stretch (the flex default), not items_start: the rail's left rule must
        // run the panel's full height.
        // Contract: the open picker panel is a `dialog`.
        let mut panel = ui_element::div()
            .flex_row()
            .gap(rem_to_px(0.75))
            .aria_role(jetstream_ui::accesskit::Role::Dialog)
            .child(models);
        if is_split {
            panel = panel.child(axes_column);
        }

        root = root.child(
            ui_element::div()
                // The split layout needs room for both columns (contract §7).
                .min_w(rem_to_px(if is_split { 32.0 } else { 18.0 }))
                .max_w(rem_to_px(if is_split { 40.0 } else { 26.0 }))
                .rounded(surface_radius)
                .border_1()
                .border_color(item_border)
                .bg(elevated)
                .pl(rem_to_px(0.5))
                .pr(rem_to_px(0.5))
                .pt(rem_to_px(0.5))
                .pb(rem_to_px(0.5))
                .child(panel),
        );
    }

    if spec.is_disabled {
        root = root.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    crate::aria::with_aria_label(root, Some(spec.aria_label.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{ModelAxisOption, ModelCapabilityAxis, ModelOption, ModelSelection};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn sample() -> ModelPickerSpec {
        ModelPickerSpec::new()
            .with_models(vec![
                // `big` exposes both axes; `small` only the effort scale.
                ModelOption::new("big", "Big Model")
                    .with_group("Frontier")
                    .with_description("Deep reasoning")
                    .with_badge("1M")
                    .with_axes(vec!["effort".into(), "speed".into()]),
                ModelOption::new("small", "Small Model")
                    .with_group("Frontier")
                    .with_axes(vec!["effort".into()]),
            ])
            .with_axes(vec![
                ModelCapabilityAxis::select(
                    "effort",
                    "Effort",
                    vec![
                        ModelAxisOption::new("low", "Low"),
                        ModelAxisOption::new("high", "High"),
                    ],
                )
                .with_default_value(ModelAxisValue::Text("high".into())),
                ModelCapabilityAxis::toggle("speed", "Speed").with_labels("Fast", "Normal"),
            ])
            .with_value(ModelSelection::new("big"))
    }

    #[test]
    fn trigger_shows_model_and_axis_summary() {
        let tree = crate::render_probe::probe(&js_model_picker(&sample(), &theme()), 360.0, 80.0);
        assert!(tree.has_text("Big Model"), "model label missing: {:?}", tree.texts());
        // Both axes apply to `big`: the select label plus the toggle's off label.
        assert!(tree.has_text("High · Normal"), "summary wrong: {:?}", tree.texts());
    }

    #[test]
    fn open_panel_renders_rows_groups_and_axes() {
        let tree = crate::render_probe::probe(
            &js_model_picker(&sample().with_open(true), &theme()),
            360.0,
            520.0,
        );
        assert!(tree.has_text("Frontier"), "group heading missing: {:?}", tree.texts());
        assert!(tree.has_text("Deep reasoning"), "description missing: {:?}", tree.texts());
        assert!(tree.has_text("1M"), "badge missing: {:?}", tree.texts());
        assert!(tree.has_text("Effort"), "axis label missing: {:?}", tree.texts());
        assert!(tree.has_text("Speed"), "toggle axis missing: {:?}", tree.texts());
    }

    #[test]
    fn axes_the_model_does_not_expose_are_not_rendered() {
        // `small` only references `effort`, so the toggle section disappears.
        let spec = sample()
            .with_open(true)
            .with_value(ModelSelection::new("small"));
        let tree = crate::render_probe::probe(&js_model_picker(&spec, &theme()), 360.0, 520.0);
        assert!(tree.has_text("Effort"), "select axis should stay: {:?}", tree.texts());
        assert!(
            !tree.has_text("Speed"),
            "scoped-out axis must not render: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn many_level_axis_renders_every_level_as_a_row() {
        let spec = ModelPickerSpec::new()
            .with_models(vec![ModelOption::new("m", "M")])
            .with_axes(vec![ModelCapabilityAxis::select(
                "effort",
                "Effort",
                vec![
                    ModelAxisOption::new("minimal", "Minimal"),
                    ModelAxisOption::new("low", "Low"),
                    ModelAxisOption::new("medium", "Medium"),
                    ModelAxisOption::new("high", "High"),
                    ModelAxisOption::new("max", "Maximum"),
                ],
            )])
            .with_value(
                ModelSelection::new("m").with_axis("effort", ModelAxisValue::Text("high".into())),
            )
            .with_open(true);
        let tree = crate::render_probe::probe(&js_model_picker(&spec, &theme()), 640.0, 520.0);
        // Five options > the segmented threshold, so every level is its own row.
        for label in ["Minimal", "Low", "Medium", "High", "Maximum"] {
            assert!(tree.has_text(label), "missing level {label}: {:?}", tree.texts());
        }
    }

    #[test]
    fn subdued_emphasis_keeps_the_label_but_dims_it() {
        use poodle_specs::ModelPickerEmphasis;
        let th = theme();
        let subdued = sample().with_emphasis(ModelPickerEmphasis::Subdued);
        let tree = crate::render_probe::probe(&js_model_picker(&subdued, &th), 360.0, 80.0);
        // Emphasis is a colour change only: the same text still renders.
        assert!(tree.has_text("Big Model"), "label missing when subdued: {:?}", tree.texts());
        assert_ne!(
            resolve_color(&th, sample().trigger_label_color_token()),
            resolve_color(&th, subdued.trigger_label_color_token()),
        );
    }

    #[test]
    fn placeholder_shows_without_a_selection() {
        let spec = sample().with_value(ModelSelection::default());
        let tree = crate::render_probe::probe(&js_model_picker(&spec, &theme()), 360.0, 80.0);
        assert!(tree.has_text("Select model"), "placeholder missing: {:?}", tree.texts());
    }



    #[test]
    fn choosing_a_model_reports_its_value() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let values = Arc::clone(&seen);

        let spec = sample().with_open(true);
        let first = spec.models.first().cloned().expect("a model");

        let el = ModelPicker::from_spec(spec, &theme())
            .on_change(move |value| values.lock().unwrap().push(value.to_string()))
            .into_js_el();

        // The trigger shows the selected model's label too, so index 1 is the
        // panel row rather than the trigger.
        crate::element::click_probe::click_text_nth(&el, 480.0, 600.0, &first.label, 1);

        assert_eq!(seen.lock().unwrap().as_slice(), [first.value]);
    }


}
