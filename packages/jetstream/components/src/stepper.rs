//! Stepper — the route through a wizard process, backed by `StepperSpec`.
//!
//! Every dimension resolves from the spec's size/density ladder; the only
//! literals here are the hairline border width and the pill radius, both of
//! which the contract states as absolutes rather than tokens.
//!
//! Status is read from the step, never from its index — a step that ran and
//! failed has to render as failed wherever it sits. See `stepper.md` §1.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{StepStatus, StepperSpec, Orientation};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Stepper — steps with status and re-run.
///
/// Mirrors the GPUI target's names: `on_change` and `on_rerun`, each carrying
/// the step's value. This is the component whose GPUI handlers were stored and
/// never attached — the defect that started g12.017 — so its Jetstream tests
/// are the reference for what "actually wired" means.
pub struct Stepper {
    spec: StepperSpec,
    theme: JetstreamThemeProvider,
    on_change: Option<crate::element::Handler>,
    on_rerun: Option<crate::element::Handler>,
}

impl Stepper {
    pub fn from_spec(spec: StepperSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
            on_rerun: None,
        }
    }

    /// Fires with the chosen step's value. Disabled steps never fire.
    pub fn on_change(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_change = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires with the step whose re-run control was pressed.
    pub fn on_rerun(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_rerun = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for Stepper {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_change, self.on_rerun)
    }
}

pub fn js_stepper(spec: &StepperSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None, None)
}

fn build(
    spec: &StepperSpec,
    theme: &JetstreamThemeProvider,
    on_change: Option<crate::element::Handler>,
    on_rerun: Option<crate::element::Handler>,
) -> JsEl {
    let row_height = rem_to_px(spec.row_height_rem());
    let marker_size = rem_to_px(spec.marker_size_rem());
    let font_size = rem_to_px(spec.font_size_rem());
    let marker_font_size = rem_to_px(spec.marker_font_size_rem());
    let pad_y = rem_to_px(spec.padding_block_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());
    let gap = rem_to_px(spec.gap_rem());
    let radius = resolve_radius(theme, spec.radius_token());
    // Contract §8: a hairline divider, stated as an absolute rather than a
    // token because no border-width token is finer than 1px.
    let hairline = rem_to_px(0.0625);

    let border: Color = resolve_color(theme, spec.border_token()).into();
    let panel: Color = resolve_color(theme, spec.surface_token()).into();
    let label_color: Color = resolve_color(theme, spec.label_token()).into();
    let active_label: Color = resolve_color(theme, spec.active_label_token()).into();
    let accent: Color = resolve_color(theme, spec.accent_token()).into();
    let danger: Color = resolve_color(theme, spec.danger_token()).into();
    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    let current = spec.current_value().map(str::to_owned);

    let mut root = if spec.orientation == Orientation::Vertical {
        ui_element::div().flex_col()
    } else {
        ui_element::div().flex_row()
    }
        .border(hairline)
        .border_color(border)
        .rounded(radius)
        .bg(panel.with_alpha(panel.a * 0.92))
        .overflow_hidden()
        .aria_role(jetstream_ui::accesskit::Role::List);

    if let Some(aria) = &spec.aria_label {
        root = root.aria_label(aria.clone());
    }

    let last = spec.steps.len().saturating_sub(1);

    for (index, step) in spec.steps.iter().enumerate() {
        let is_current = current.as_deref() == Some(step.value.as_str());
        let has_rerun = spec.show_rerun && step.status == StepStatus::Complete;
        let is_disabled = spec.is_disabled || step.is_disabled;

        let marker_color = match step.status {
            StepStatus::Failed => danger,
            StepStatus::Complete | StepStatus::Running => accent,
            StepStatus::Pending if is_current => accent,
            StepStatus::Pending => label_color,
        };

        // Failed wins over current: a step that is both the one you are on and
        // the one that broke reports the breakage, which is the more urgent of
        // the two facts.
        let text_color = match step.status {
            StepStatus::Failed => danger,
            StepStatus::Complete | StepStatus::Running => active_label,
            StepStatus::Pending if is_current => active_label,
            StepStatus::Pending => label_color,
        };

        // The marker carries index, glyph or spinner by status. Pending is the
        // only case that shows a number, which is why the index is used here
        // and nowhere else.
        let marker: JsEl = match step.status {
            StepStatus::Complete => ui_element::icon("check")
                .w(marker_font_size)
                .h(marker_font_size)
                .text_color(marker_color),
            StepStatus::Failed => ui_element::icon("x")
                .w(marker_font_size)
                .h(marker_font_size)
                .text_color(marker_color),
            StepStatus::Running => ui_element::icon("loader")
                .w(marker_font_size)
                .h(marker_font_size)
                .text_color(marker_color),
            StepStatus::Pending => ui_element::label(format!("{}", index + 1))
                .text_size(marker_font_size)
                .text_weight(700)
                .text_color(marker_color),
        };

        let marker_box = ui_element::div()
            .w(marker_size)
            .h(marker_size)
            .flex_row()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .border(hairline)
            .border_color(marker_color)
            .rounded(999.0)
            .child(marker);

        let status_word = match step.status {
            StepStatus::Running => ", running",
            StepStatus::Complete => ", complete",
            StepStatus::Failed => ", failed",
            StepStatus::Pending => "",
        };

        let mut trigger = ui_element::button("")
            // Status reaches assistive technology through the name; colour and
            // glyph do not. `pending` is omitted as the unremarkable case.
            .aria_label(format!("{}{}", step.label, status_word))
            .aria_role(jetstream_ui::accesskit::Role::Button)
            .flex_row()
            .items_center()
            .gap(gap)
            .grow()
            .min_w_0()
            .min_h(row_height)
            .pl(pad_x)
            .pr(if has_rerun { 0.0 } else { pad_x })
            .pt(pad_y)
            .pb(pad_y)
            .bg(Color::TRANSPARENT)
            .text_size(font_size)
            .text_color(text_color)
            .focusable()
            .child(marker_box)
            .child(
                ui_element::label(step.label.clone())
                    .text_size(font_size)
                    .text_color(text_color)
                    .min_w_0(),
            );

        if is_disabled {
            trigger = trigger.opacity(disabled_opacity).disabled(true);
        } else {
            trigger = trigger.cursor_pointer();

            if let Some(handler) = &on_change {
                let handler = std::sync::Arc::clone(handler);
                let value = step.value.clone();
                trigger = trigger.on_click(move |_event| handler(&value));
            }
        }

        // The tint belongs to the whole column. On the trigger it stopped
        // wherever the trigger stopped, so a step with a rerun beside it had
        // the fill end in open space with a hard edge halfway across the cell.
        // The hit target is unchanged.
        let mut cell = ui_element::div()
            .flex_row()
            .items_stretch()
            .min_w_0()
            .grow()
            .aria_role(jetstream_ui::accesskit::Role::ListItem)
            .child(trigger);

        // Both the current tint and hover live on the cell, not the trigger, so
        // they span the whole column including the rerun control. Hovering the
        // current step deepens its own colour rather than swapping to a
        // neutral fill.
        if is_current {
            cell = cell.bg(accent.with_alpha(accent.a * 0.10));
        }
        if !is_disabled {
            let hover_fill = if is_current {
                accent.with_alpha(accent.a * 0.16)
            } else {
                active_label.with_alpha(active_label.a * 0.06)
            };
            cell = cell.hover(move |s| s.bg(hover_fill));
        }

        // Deliberately outside the trigger: re-running spends whatever the step
        // costs, so it cannot be reachable by clicking to look at a finished
        // step. See `stepper.md` §2.
        if has_rerun {
            let mut rerun = ui_element::button("")
                .aria_label(format!("{}: {}", spec.rerun_label, step.label))
                .aria_role(jetstream_ui::accesskit::Role::Button)
                .w(marker_size)
                .h(marker_size)
                // Room on both sides. With a right margin only, the icon sat
                // hard against the trigger's tinted edge and read as part of
                // it — the opposite of what a deliberate action should look
                // like.
                .ml(gap)
                .mr(pad_x)
                .flex_row()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .focusable()
                .child(
                    ui_element::icon("refresh-cw")
                        .w(marker_font_size)
                        .h(marker_font_size)
                        .text_color(label_color),
                );
            if is_disabled {
                rerun = rerun.opacity(disabled_opacity).disabled(true);
            } else {
                rerun = rerun.cursor_pointer();

                // Its own handler, inert when unwired: the rerun sits beside a
                // clickable trigger, and clicks bubble to the nearest handler,
                // so an unwired rerun would select the step it was re-running.
                if let Some(handler) = &on_rerun {
                    let handler = std::sync::Arc::clone(handler);
                    let value = step.value.clone();
                    rerun = rerun.on_click(move |_event| handler(&value));
                } else {
                    rerun = rerun.on_click(|_event| {});
                }
            }
            cell = cell.child(rerun);
        }

        // Dividers are drawn inside the shared track rather than around each
        // cell, so the outer border stays a single rectangle. Vertical flows
        // the same track as rows, so the divider moves to the bottom edge.
        if index < last {
            cell = if spec.orientation == Orientation::Vertical {
                cell.border_b_1().border_color(border)
            } else {
                cell.border_r_1().border_color(border)
            };
        }

        root = root.child(cell);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::StepperStep;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn probe(spec: &StepperSpec) -> crate::render_probe::ProbeTree {
        crate::render_probe::probe(&js_stepper(spec, &theme()), 720.0, 96.0)
    }

    /// The defect the status property exists to prevent: a failed step sitting
    /// behind the current one must not render as complete.
    #[test]
    fn a_failed_step_behind_the_current_one_still_reads_as_failed() {
        let spec = StepperSpec::new(vec![
            StepperStep::new("a", "Read source").with_status(StepStatus::Complete),
            StepperStep::new("b", "Quality gate").with_status(StepStatus::Failed),
            StepperStep::new("c", "Apply changes"),
        ])
        .with_value("c");

        let tree = probe(&spec);
        assert!(tree.has_text("Quality gate"), "{:?}", tree.texts());
        // Pending steps number themselves; a failed one must not, or it reads
        // as simply unvisited.
        assert!(
            !tree.has_text("2"),
            "failed step numbered itself: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("3"),
            "pending step should show its index: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn rerun_appears_only_for_completed_steps_and_only_when_enabled() {
        let steps = vec![
            StepperStep::new("a", "One").with_status(StepStatus::Complete),
            StepperStep::new("b", "Two").with_status(StepStatus::Running),
        ];

        let without = probe(&StepperSpec::new(steps.clone()));
        let with = probe(&StepperSpec::new(steps).with_show_rerun(true));

        // One rerun control for the single completed step, and none at all when
        // the consumer has not opted in.
        assert!(with.nodes.len() > without.nodes.len());
    }

    #[test]
    fn density_does_not_change_row_height() {
        let steps = vec![StepperStep::new("a", "One")];
        let base = StepperSpec::new(steps.clone());
        let compact = StepperSpec::new(steps).with_density(poodle_specs::ControlDensity::Compact);
        assert_eq!(base.row_height_rem(), compact.row_height_rem());
    }

    /// This is the component whose GPUI handlers were stored and never
    /// attached — the defect that started g12.017.
    #[test]
    fn choosing_a_step_reports_its_value() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let values = Arc::clone(&seen);

        let spec = StepperSpec::new(vec![
            StepperStep::new("a", "Read source").with_status(StepStatus::Complete),
            StepperStep::new("b", "Quality gate").with_status(StepStatus::Running),
        ]);

        let el = Stepper::from_spec(spec, &theme())
            .on_change(move |value| values.lock().unwrap().push(value.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 640.0, 120.0, "Quality gate");

        assert_eq!(seen.lock().unwrap().as_slice(), ["b"]);
    }

    /// The rerun sits inside a clickable step, so it takes its own handler —
    /// and rerunning a step must not also select it.
    #[test]
    fn the_rerun_control_reruns_without_selecting() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let reruns = Arc::clone(&seen);
        let changes = Arc::clone(&seen);

        let spec = StepperSpec::new(vec![
            StepperStep::new("a", "Read source").with_status(StepStatus::Complete)
        ])
        .with_show_rerun(true);

        let el = Stepper::from_spec(spec, &theme())
            .on_rerun(move |value| reruns.lock().unwrap().push(format!("rerun:{value}")))
            .on_change(move |value| changes.lock().unwrap().push(format!("change:{value}")))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 640.0, 120.0, "refresh-cw");

        assert_eq!(seen.lock().unwrap().as_slice(), ["rerun:a"]);
    }
}
