//! Slider and RangeSlider machines. Mirror of core `slider.ts`.

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
