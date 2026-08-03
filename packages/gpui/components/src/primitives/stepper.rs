//! Stepper — real GPUI component backed by `StepperSpec`.
//!
//! Status is read from the step, never inferred from its index. A step that ran
//! and was rejected has to render as failed wherever it sits in the sequence;
//! `index < current` would draw it as "not yet reached". See `stepper.md` §1.
//!
//! `on_change` and `on_rerun` reach real `on_click` handlers. They were stored
//! and never attached for a while: the builders type-checked, the pointing-hand
//! cursor promised a click, and nothing happened when you made one.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{StepStatus, StepperSpec, StepperStep, Orientation};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// A real GPUI stepper component backed by `StepperSpec`.
pub struct Stepper {
    spec: StepperSpec,
    theme: GpuiThemeProvider,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_rerun: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Stepper {
    type Target = StepperSpec;
    fn deref(&self) -> &StepperSpec {
        &self.spec
    }
}

impl Stepper {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: StepperSpec::default(),
            theme: theme.clone(),
            on_change: None,
            on_rerun: None,
        }
    }

    pub fn from_spec(spec: StepperSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
            on_rerun: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn steps(mut self, v: Vec<StepperStep>) -> Self {
        self.spec.steps = v;
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = Some(v.into());
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn show_rerun(mut self, v: bool) -> Self {
        self.spec.show_rerun = v;
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    /// Re-run is wired separately from selection on purpose.
    ///
    /// Re-running spends whatever the step costs, so it must not be reachable
    /// by the same gesture that opens a finished step to look at it.
    pub fn on_rerun(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_rerun = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for Stepper {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let border = resolve_color(theme, self.spec.border_token());
        let panel = resolve_color(theme, self.spec.surface_token());
        let label_color = resolve_color(theme, self.spec.label_token());
        let active_label = resolve_color(theme, self.spec.active_label_token());
        let accent = resolve_color(theme, self.spec.accent_token());
        let danger = resolve_color(theme, self.spec.danger_token());
        let disabled_opacity = resolve_opacity(theme, self.spec.disabled_opacity_token());
        let radius = resolve_radius(theme, self.spec.radius_token());

        let row_height = px(rem_to_px(self.spec.row_height_rem()));
        let marker_size = px(rem_to_px(self.spec.marker_size_rem()));
        let font_size = px(rem_to_px(self.spec.font_size_rem()));
        let marker_font_size = px(rem_to_px(self.spec.marker_font_size_rem()));
        let pad_y = px(rem_to_px(self.spec.padding_block_rem()));
        let pad_x = px(rem_to_px(self.spec.padding_inline_rem()));
        let gap = px(rem_to_px(self.spec.gap_rem()));

        // Contract §8 root background: panel at 92%.
        let root_bg = Hsla {
            a: panel.a * 0.92,
            ..panel
        };

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_control_disabled = self.spec.is_disabled;
        let last = self.spec.steps.len().saturating_sub(1);
        let is_vertical = self.spec.orientation == Orientation::Vertical;

        let mut root = div()
            .flex()
            .when(is_vertical, |el| el.flex_col())
            .rounded(radius)
            .border_1()
            .border_color(border)
            .bg(root_bg)
            .overflow_hidden();

        for (index, step) in self.spec.steps.iter().enumerate() {
            let is_current = current_value.as_deref() == Some(step.value.as_str());
            let has_rerun = self.spec.show_rerun && step.status == StepStatus::Complete;
            let is_disabled = is_control_disabled || step.is_disabled;

            // Failed wins over current: a step that is both the one you are on
            // and the one that broke reports the breakage, which is the more
            // urgent of the two facts.
            let marker_color = match step.status {
                StepStatus::Failed => danger,
                StepStatus::Complete | StepStatus::Running => accent,
                StepStatus::Pending if is_current => accent,
                StepStatus::Pending => label_color,
            };
            let text_color = match step.status {
                StepStatus::Failed => danger,
                StepStatus::Complete | StepStatus::Running => active_label,
                StepStatus::Pending if is_current => active_label,
                StepStatus::Pending => label_color,
            };

            // Only a pending step numbers itself; the others say what happened.
            // A numbered marker is how a step reports "not reached", so a failed
            // one must never show one.
            let marker_glyph: String = match step.status {
                StepStatus::Complete => "✓".to_string(),
                StepStatus::Failed => "✕".to_string(),
                StepStatus::Running => "◌".to_string(),
                StepStatus::Pending => format!("{}", index + 1),
            };

            let marker = div()
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .w(marker_size)
                .h(marker_size)
                .rounded_full()
                .border_1()
                .border_color(marker_color)
                .text_color(marker_color)
                .text_size(marker_font_size)
                .child(marker_glyph);

            let mut trigger = div()
                // `on_click` is only available on a stateful element, which in
                // GPUI means one carrying an id.
                .id(SharedString::from(format!("poodle-stepper-step-{}", step.value)))
                .flex()
                .flex_1()
                .items_center()
                .gap(gap)
                .min_h(row_height)
                .pl(pad_x)
                .pr(if has_rerun { px(0.0) } else { pad_x })
                .py(pad_y)
                .text_color(text_color)
                .text_size(font_size)
                .child(marker)
                .child(div().min_w_0().child(step.label.clone()));

            if is_disabled {
                trigger = trigger
                    .opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                trigger = trigger.cursor(CursorStyle::PointingHand);
                if let Some(handler) = &self.on_change {
                    let handler = handler.clone();
                    let value = step.value.clone();
                    trigger = trigger.on_click(move |_event, window, cx| {
                        handler(&value, window, cx);
                    });
                }
            }

            // The tint belongs to the whole column — see the Jetstream note.
            let mut cell = div().flex().flex_1().min_w_0().child(trigger);
            // Both the current tint and hover live on the cell — see the
            // Jetstream note.
            if is_current {
                cell = cell.bg(Hsla {
                    a: accent.a * 0.10,
                    ..accent
                });
            }
            if !is_disabled {
                let hover_fill = if is_current {
                    Hsla { a: accent.a * 0.16, ..accent }
                } else {
                    Hsla { a: active_label.a * 0.06, ..active_label }
                };
                cell = cell.hover(move |s| s.bg(hover_fill));
            }

            // A separate control, outside the trigger — see the `on_rerun` note.
            if has_rerun {
                let mut rerun = div()
                    .id(SharedString::from(format!("poodle-stepper-rerun-{}", step.value)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_shrink_0()
                    .w(marker_size)
                    .h(marker_size)
                    // Room on both sides — see the Jetstream note.
                    .ml(gap)
                    .mr(pad_x)
                    .text_color(label_color)
                    // The label size, not the marker size: marker glyphs sit
                    // inside a bordered circle that gives them presence, and
                    // this one does not.
                    .text_size(font_size)
                    .child("⟳");
                if is_disabled {
                    rerun = rerun.opacity(disabled_opacity);
                } else {
                    rerun = rerun.cursor(CursorStyle::PointingHand);
                    if let Some(handler) = &self.on_rerun {
                        let handler = handler.clone();
                        let value = step.value.clone();
                        rerun = rerun.on_click(move |_event, window, cx| {
                            handler(&value, window, cx);
                        });
                    }
                }
                cell = cell.child(div().flex().items_center().child(rerun));
            }

            // Dividers live inside the shared track, so the outer border stays a
            // single rectangle rather than a box per step. Vertical flows the
            // same track as rows, so the divider moves to the bottom edge.
            if index < last {
                cell = if is_vertical {
                    cell.border_b_1().border_color(border)
                } else {
                    cell.border_r_1().border_color(border)
                };
            }

            root = root.child(cell);
        }

        root.into_any_element()
    }
}

// No unit tests here on purpose. `cargo test` cannot build this crate's test
// harness at all — `recursion limit reached while expanding #[test]`, which
// reproduces on a clean tree and is why `check:gpui` is a `cargo check`. The
// status logic these tests would cover lives in `poodle-specs`, where it is
// tested and shared with every target.
