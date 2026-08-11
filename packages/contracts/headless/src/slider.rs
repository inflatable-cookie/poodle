//! Slider and RangeSlider machines. Mirror of core `slider.ts`.

use crate::audio::{denormalize_value, normalize_value, AudioValueLaw};

pub fn clamp_value(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

pub fn snap_to_step(value: f64, min: f64, step: f64) -> f64 {
    if !step.is_finite() || step <= 0.0 {
        return value;
    }

    min + ((value - min) / step).round() * step
}

/// Degenerate ranges (max <= min) widen to one step so percentage math stays finite.
pub fn safe_slider_max(min: f64, max: f64) -> f64 {
    if max <= min {
        min + 1.0
    } else {
        max
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderContext {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderEvent {
    Input { raw: f64 },
    Commit { raw: f64 },
    SetValue { value: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderEffect {
    EmitValueChange { value: f64 },
    EmitValueCommit { value: f64 },
}

pub fn normalize_slider_value(context: SliderContext, raw: f64) -> f64 {
    clamp_value(
        snap_to_step(raw, context.min, context.step),
        context.min,
        safe_slider_max(context.min, context.max),
    )
}

pub fn slider_transition(
    context: SliderContext,
    event: SliderEvent,
) -> (SliderContext, Vec<SliderEffect>) {
    match event {
        SliderEvent::Input { raw } => {
            let value = normalize_slider_value(context, raw);

            (
                SliderContext { value, ..context },
                vec![SliderEffect::EmitValueChange { value }],
            )
        }
        SliderEvent::Commit { raw } => {
            let value = normalize_slider_value(context, raw);

            (
                SliderContext { value, ..context },
                vec![SliderEffect::EmitValueCommit { value }],
            )
        }
        SliderEvent::SetValue { value } => (SliderContext { value, ..context }, vec![]),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeSliderContext {
    pub value: (f64, f64),
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeThumb {
    Lower,
    Upper,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RangeSliderEvent {
    Input { thumb: RangeThumb, raw: f64 },
    Commit { thumb: RangeThumb, raw: f64 },
    SetValue { value: (f64, f64) },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RangeSliderEffect {
    EmitValueChange { value: (f64, f64) },
    EmitValueCommit { value: (f64, f64) },
}

/// Display pair with thumbs ordered and clamped into range.
pub fn normalize_range_value(context: RangeSliderContext) -> (f64, f64) {
    let max = safe_slider_max(context.min, context.max);
    let lower = clamp_value(context.value.0.min(context.value.1), context.min, max);
    let upper = clamp_value(context.value.0.max(context.value.1), context.min, max);

    (lower, upper)
}

pub fn range_slider_transition(
    context: RangeSliderContext,
    event: RangeSliderEvent,
) -> (RangeSliderContext, Vec<RangeSliderEffect>) {
    match event {
        RangeSliderEvent::Input { thumb, raw } | RangeSliderEvent::Commit { thumb, raw } => {
            let max = safe_slider_max(context.min, context.max);
            let (lower, upper) = normalize_range_value(context);
            let snapped = snap_to_step(raw, context.min, context.step);
            // A thumb cannot cross its sibling.
            let value = match thumb {
                RangeThumb::Lower => (clamp_value(snapped, context.min, upper), upper),
                RangeThumb::Upper => (lower, clamp_value(snapped, lower, max)),
            };

            let effect = match event {
                RangeSliderEvent::Input { .. } => RangeSliderEffect::EmitValueChange { value },
                _ => RangeSliderEffect::EmitValueCommit { value },
            };

            (RangeSliderContext { value, ..context }, vec![effect])
        }
        RangeSliderEvent::SetValue { value } => (RangeSliderContext { value, ..context }, vec![]),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderPolarity {
    #[default]
    Unipolar,
    Bipolar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderControlContext {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    pub law: AudioValueLaw,
    pub polarity: SliderPolarity,
    pub center_value: Option<f64>,
    pub pointer_active: bool,
}

impl Default for SliderControlContext {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            law: AudioValueLaw::Linear,
            polarity: SliderPolarity::Unipolar,
            center_value: None,
            pointer_active: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliderControlEvent {
    PointerBegin { value_norm: f64 },
    PointerMove { value_norm: f64 },
    PointerEnd,
    SetValue { value: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderVisualState {
    pub value: f64,
    pub value_norm: f64,
    pub center_norm: f64,
    pub fill_start_norm: f64,
    pub fill_span_norm: f64,
    pub fill_tone: SliderFillTone,
    pub polarity: SliderPolarity,
    pub pointer_active: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderFillTone {
    Positive,
    Negative,
}

fn slider_center_value(context: SliderControlContext) -> f64 {
    let max = safe_slider_max(context.min, context.max);
    match context.polarity {
        SliderPolarity::Unipolar => clamp_value(0.0, context.min, max),
        SliderPolarity::Bipolar => context.center_value.map_or_else(
            || {
                if context.min < 0.0 && max > 0.0 {
                    0.0
                } else {
                    context.min + (max - context.min) / 2.0
                }
            },
            |center| clamp_value(center, context.min, max),
        ),
    }
}

fn slider_control_value_at(context: SliderControlContext, value_norm: f64) -> f64 {
    let value = denormalize_value(
        value_norm,
        context.min,
        safe_slider_max(context.min, context.max),
        context.law,
    );
    normalize_slider_value(
        SliderContext {
            value: context.value,
            min: context.min,
            max: context.max,
            step: context.step,
            disabled: context.disabled,
        },
        value,
    )
}

pub fn slider_visual_state(context: SliderControlContext) -> SliderVisualState {
    let max = safe_slider_max(context.min, context.max);
    let value = normalize_slider_value(
        SliderContext {
            value: context.value,
            min: context.min,
            max: context.max,
            step: context.step,
            disabled: context.disabled,
        },
        context.value,
    );
    let value_norm = normalize_value(value, context.min, max, context.law);
    let center_norm = normalize_value(slider_center_value(context), context.min, max, context.law);
    SliderVisualState {
        value,
        value_norm,
        center_norm,
        fill_start_norm: value_norm.min(center_norm),
        fill_span_norm: (value_norm - center_norm).abs(),
        fill_tone: if context.polarity == SliderPolarity::Bipolar && value_norm < center_norm {
            SliderFillTone::Negative
        } else {
            SliderFillTone::Positive
        },
        polarity: context.polarity,
        pointer_active: context.pointer_active,
        enabled: !context.disabled,
    }
}

pub fn slider_control_transition(
    context: SliderControlContext,
    event: SliderControlEvent,
) -> (SliderControlContext, Vec<SliderEffect>) {
    match event {
        SliderControlEvent::PointerBegin { value_norm } if !context.disabled => {
            let value = slider_control_value_at(context, value_norm);
            (
                SliderControlContext {
                    value,
                    pointer_active: true,
                    ..context
                },
                vec![SliderEffect::EmitValueChange { value }],
            )
        }
        SliderControlEvent::PointerMove { value_norm }
            if !context.disabled && context.pointer_active =>
        {
            let value = slider_control_value_at(context, value_norm);
            (
                SliderControlContext { value, ..context },
                vec![SliderEffect::EmitValueChange { value }],
            )
        }
        SliderControlEvent::PointerEnd if context.pointer_active => (
            SliderControlContext {
                pointer_active: false,
                ..context
            },
            vec![SliderEffect::EmitValueCommit {
                value: context.value,
            }],
        ),
        SliderControlEvent::SetValue { value } => {
            let value = normalize_slider_value(
                SliderContext {
                    value: context.value,
                    min: context.min,
                    max: context.max,
                    step: context.step,
                    disabled: context.disabled,
                },
                value,
            );
            (SliderControlContext { value, ..context }, vec![])
        }
        _ => (context, vec![]),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeSliderControlContext {
    pub value: (f64, f64),
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    pub law: AudioValueLaw,
    pub polarity: SliderPolarity,
    pub center_value: Option<f64>,
    pub pointer_active: bool,
    pub active_thumb: Option<RangeThumb>,
}

impl Default for RangeSliderControlContext {
    fn default() -> Self {
        Self {
            value: (0.0, 100.0),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            law: AudioValueLaw::Linear,
            polarity: SliderPolarity::Unipolar,
            center_value: None,
            pointer_active: false,
            active_thumb: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RangeSliderControlEvent {
    PointerBegin { value_norm: f64 },
    PointerMove { value_norm: f64 },
    PointerEnd,
    SetValue { value: (f64, f64) },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeSliderVisualState {
    pub value: (f64, f64),
    pub lower_norm: f64,
    pub upper_norm: f64,
    pub center_norm: f64,
    pub fill_start_norm: f64,
    pub fill_span_norm: f64,
    pub negative_fill_start_norm: f64,
    pub negative_fill_span_norm: f64,
    pub positive_fill_start_norm: f64,
    pub positive_fill_span_norm: f64,
    pub polarity: SliderPolarity,
    pub pointer_active: bool,
    pub active_thumb: Option<RangeThumb>,
    pub enabled: bool,
}

pub fn range_slider_visual_state(context: RangeSliderControlContext) -> RangeSliderVisualState {
    let max = safe_slider_max(context.min, context.max);
    let value = normalize_range_value(RangeSliderContext {
        value: context.value,
        min: context.min,
        max: context.max,
        step: context.step,
        disabled: context.disabled,
    });
    let lower_norm = normalize_value(value.0, context.min, max, context.law);
    let upper_norm = normalize_value(value.1, context.min, max, context.law);
    let center_norm = normalize_value(
        slider_center_value(SliderControlContext {
            value: value.0,
            min: context.min,
            max: context.max,
            step: context.step,
            disabled: context.disabled,
            law: context.law,
            polarity: context.polarity,
            center_value: context.center_value,
            pointer_active: context.pointer_active,
        }),
        context.min,
        max,
        context.law,
    );
    let negative_fill_span_norm = if context.polarity == SliderPolarity::Bipolar {
        (upper_norm.min(center_norm) - lower_norm).max(0.0)
    } else {
        0.0
    };
    let positive_fill_start_norm = if context.polarity == SliderPolarity::Bipolar {
        lower_norm.max(center_norm)
    } else {
        lower_norm
    };
    let positive_fill_span_norm = if context.polarity == SliderPolarity::Bipolar {
        (upper_norm - positive_fill_start_norm).max(0.0)
    } else {
        upper_norm - lower_norm
    };
    RangeSliderVisualState {
        value,
        lower_norm,
        upper_norm,
        center_norm,
        fill_start_norm: lower_norm,
        fill_span_norm: upper_norm - lower_norm,
        negative_fill_start_norm: lower_norm,
        negative_fill_span_norm,
        positive_fill_start_norm,
        positive_fill_span_norm,
        polarity: context.polarity,
        pointer_active: context.pointer_active,
        active_thumb: context.active_thumb,
        enabled: !context.disabled,
    }
}

fn range_control_value_at(context: RangeSliderControlContext, value_norm: f64) -> f64 {
    let max = safe_slider_max(context.min, context.max);
    let value = denormalize_value(value_norm, context.min, max, context.law);
    clamp_value(
        snap_to_step(value, context.min, context.step),
        context.min,
        max,
    )
}

pub fn range_slider_control_transition(
    context: RangeSliderControlContext,
    event: RangeSliderControlEvent,
) -> (RangeSliderControlContext, Vec<RangeSliderEffect>) {
    match event {
        RangeSliderControlEvent::PointerBegin { value_norm } if !context.disabled => {
            let visual = range_slider_visual_state(context);
            let thumb = if (value_norm - visual.lower_norm).abs()
                <= (visual.upper_norm - value_norm).abs()
            {
                RangeThumb::Lower
            } else {
                RangeThumb::Upper
            };
            let raw = range_control_value_at(context, value_norm);
            let (next, effects) = range_slider_transition(
                RangeSliderContext {
                    value: context.value,
                    min: context.min,
                    max: context.max,
                    step: context.step,
                    disabled: context.disabled,
                },
                RangeSliderEvent::Input { thumb, raw },
            );
            (
                RangeSliderControlContext {
                    value: next.value,
                    pointer_active: true,
                    active_thumb: Some(thumb),
                    ..context
                },
                effects,
            )
        }
        RangeSliderControlEvent::PointerMove { value_norm }
            if !context.disabled && context.pointer_active && context.active_thumb.is_some() =>
        {
            let thumb = context.active_thumb.expect("guarded active thumb");
            let raw = range_control_value_at(context, value_norm);
            let (next, effects) = range_slider_transition(
                RangeSliderContext {
                    value: context.value,
                    min: context.min,
                    max: context.max,
                    step: context.step,
                    disabled: context.disabled,
                },
                RangeSliderEvent::Input { thumb, raw },
            );
            (
                RangeSliderControlContext {
                    value: next.value,
                    ..context
                },
                effects,
            )
        }
        RangeSliderControlEvent::PointerEnd if context.pointer_active => (
            RangeSliderControlContext {
                pointer_active: false,
                active_thumb: None,
                ..context
            },
            vec![RangeSliderEffect::EmitValueCommit {
                value: context.value,
            }],
        ),
        RangeSliderControlEvent::SetValue { value } => {
            let value = normalize_range_value(RangeSliderContext {
                value,
                min: context.min,
                max: context.max,
                step: context.step,
                disabled: context.disabled,
            });
            (RangeSliderControlContext { value, ..context }, vec![])
        }
        _ => (context, vec![]),
    }
}

#[cfg(test)]
mod control_tests {
    use super::*;

    #[test]
    fn bipolar_fill_grows_from_center() {
        let state = slider_visual_state(SliderControlContext {
            value: -0.5,
            min: -1.0,
            max: 1.0,
            step: 0.01,
            polarity: SliderPolarity::Bipolar,
            ..SliderControlContext::default()
        });
        assert!((state.center_norm - 0.5).abs() < 1e-9);
        assert!((state.fill_start_norm - 0.25).abs() < 1e-9);
        assert!((state.fill_span_norm - 0.25).abs() < 1e-9);
        assert_eq!(state.fill_tone, SliderFillTone::Negative);
    }

    #[test]
    fn bipolar_range_splits_negative_and_positive_fill() {
        let state = range_slider_visual_state(RangeSliderControlContext {
            value: (-0.5, 0.5),
            min: -1.0,
            max: 1.0,
            step: 0.01,
            polarity: SliderPolarity::Bipolar,
            ..RangeSliderControlContext::default()
        });
        assert!((state.negative_fill_start_norm - 0.25).abs() < 1e-9);
        assert!((state.negative_fill_span_norm - 0.25).abs() < 1e-9);
        assert!((state.positive_fill_start_norm - 0.5).abs() < 1e-9);
        assert!((state.positive_fill_span_norm - 0.25).abs() < 1e-9);
    }

    #[test]
    fn range_gesture_keeps_the_chosen_thumb() {
        let context = RangeSliderControlContext {
            value: (0.2, 0.8),
            min: 0.0,
            max: 1.0,
            step: 0.01,
            ..RangeSliderControlContext::default()
        };
        let (context, _) = range_slider_control_transition(
            context,
            RangeSliderControlEvent::PointerBegin { value_norm: 0.75 },
        );
        assert_eq!(context.active_thumb, Some(RangeThumb::Upper));
        let (context, _) = range_slider_control_transition(
            context,
            RangeSliderControlEvent::PointerMove { value_norm: 0.1 },
        );
        assert_eq!(context.active_thumb, Some(RangeThumb::Upper));
        assert!((context.value.1 - context.value.0).abs() < 1e-9);
    }
}
