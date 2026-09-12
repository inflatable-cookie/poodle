//! Slider and RangeSlider machines. Mirror of core `slider.ts`; the
//! step-quantization tie law is identical there and here (half ties round
//! toward positive infinity, JavaScript `Math.round` semantics).

use crate::audio::{denormalize_value, normalize_value, AudioValueLaw};

pub fn clamp_value(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

pub fn snap_to_step(value: f64, min: f64, step: f64) -> f64 {
    if !step.is_finite() || step <= 0.0 {
        return value;
    }

    // Portable tie law: an index exactly halfway between two steps rounds
    // toward positive infinity, matching JavaScript `Math.round` in core
    // `slider.ts`. `f64::round` would round half away from zero instead, so
    // `snap_to_step(-0.5-index)` returned one step below `min`.
    min + ((value - min) / step + 0.5).floor() * step
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
    pub fill_split_at_center: bool,
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
        fill_split_at_center: negative_fill_span_norm > 0.0 && positive_fill_span_norm > 0.0,
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

/// Logical-pixel effective target for every block thumb.
pub const SLIDER_BLOCK_HIT_PX: f32 = 44.0;
/// Internal inline inset used by the block fit law. Not a public metric.
pub const SLIDER_BLOCK_CONTENT_INSET_PX: f32 = 8.0;

pub fn block_region_available(unoccluded_span: f32, content_inset: f32) -> f32 {
    (unoccluded_span - 2.0 * content_inset).floor()
}

pub fn block_item_fits(available: f32, required_advance: f32) -> bool {
    available >= required_advance.ceil()
}

pub fn block_inline_fits(items: &[(Option<&str>, f32)], measure: impl Fn(&str) -> f32) -> bool {
    items.iter().all(|(text, unoccluded)| match text {
        None | Some("") => true,
        Some(text) => block_item_fits(
            block_region_available(*unoccluded, SLIDER_BLOCK_CONTENT_INSET_PX),
            measure(text),
        ),
    })
}

pub fn omit_empty_visible_text(text: Option<&str>) -> Option<String> {
    match text {
        Some(value) if !value.is_empty() => Some(value.to_owned()),
        _ => None,
    }
}

/// Decimal places implied by a finite number's shortest representation.
fn implied_decimal_places(value: f64) -> usize {
    let text = format!("{value}");
    match text.split_once(['e', 'E']) {
        None => text
            .split_once('.')
            .map_or(0, |(_, fraction)| fraction.len()),
        Some((mantissa, exponent)) => {
            let mantissa_places = mantissa
                .split_once('.')
                .map_or(0, |(_, fraction)| fraction.len());
            let exponent: i32 = exponent.parse().unwrap_or(0);
            (mantissa_places as i32 - exponent).max(0) as usize
        }
    }
}

/// Decimal precision implied by `min` and a finite positive `step` (g18.024).
/// The snapped value `min + n * step` is exact to at most that many decimal
/// places, so binary tails beyond it are arithmetic noise, never data.
pub fn slider_display_precision(min: f64, step: f64) -> usize {
    let step_places = if step.is_finite() && step > 0.0 {
        implied_decimal_places(step)
    } else {
        0
    };
    (implied_decimal_places(min).max(step_places)).min(100)
}

/// g18.024 default visible value: a short step-aware decimal. The value must
/// already be step-snapped; it is rounded to the precision implied by `min`
/// and a finite positive `step`, insignificant zeroes are trimmed, and
/// negative zero normalizes to `"0"`. Binary tails never survive.
pub fn default_visible_value_text(value: f64, min: f64, step: f64) -> String {
    if !value.is_finite() {
        return format!("{value}");
    }
    if !step.is_finite() || step <= 0.0 {
        // No finite positive step: no snapping happened, so the value keeps
        // its shortest exact form; Rust prints `-0.0` as `-0`, so normalize.
        if value == 0.0 {
            return "0".to_owned();
        }
        return format!("{value}");
    }
    let precision = slider_display_precision(min, step);
    let rounded = format!("{value:.precision$}");
    let rounded: f64 = rounded.parse().unwrap_or(value);
    // Re-parsing prints the shortest exact decimal, which trims insignificant
    // zeroes for free; negative zero normalizes to `"0"` explicitly.
    if rounded == 0.0 {
        return "0".to_owned();
    }
    format!("{rounded}")
}

pub fn physical_to_value_norm(physical_norm: f64, rtl: bool) -> f64 {
    let clamped = physical_norm.clamp(0.0, 1.0);
    if rtl {
        1.0 - clamped
    } else {
        clamped
    }
}

pub fn resolved_visible_text(value: f64, min: f64, step: f64, explicit: Option<&str>) -> Option<String> {
    match explicit {
        Some("") => None,
        Some(text) => Some(text.to_owned()),
        None => omit_empty_visible_text(Some(&default_visible_value_text(value, min, step))),
    }
}

/// Approximate inline advance when no shaper is available. Tests may pass a
/// different measure into [`layout_slider_block`].
pub fn measure_block_advance(text: &str, font_size: f32) -> f32 {
    text.chars().count() as f32 * font_size * 0.5
}

/// Fixed inline placement for a single Slider (g18.017): whole-track fit.
/// Label and value pin to the logical inline edges at every value; when they
/// cannot coexist the optional label is suppressed and the exact value stays
/// at the logical end. There is no external fallback in this appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliderBlockLayout {
    pub label_inline: bool,
    pub value_inline: bool,
}

pub fn layout_slider_block(
    capsule_span: f32,
    label: Option<&str>,
    value_text: Option<&str>,
    measure: impl Fn(&str) -> f32,
) -> SliderBlockLayout {
    let available = block_region_available(capsule_span, SLIDER_BLOCK_CONTENT_INSET_PX);
    let label_advance = label.map_or(0.0, |text| measure(text).ceil());
    let value_advance = value_text.map_or(0.0, |text| measure(text).ceil());
    let value_inline = value_text.is_some();
    let label_inline = label.is_some() && available >= label_advance + value_advance;
    SliderBlockLayout {
        label_inline,
        value_inline,
    }
}

/// Fixed whole-capsule anchors for RangeSlider block text (g18.022): lower
/// value at the logical start, optional label centered, upper value at the
/// logical end, at every value pair. Endpoints are required; when the three
/// items cannot coexist the optional label is suppressed. There is no
/// external fallback in this variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeSliderBlockLayout {
    pub label_inline: bool,
    pub lower_inline: bool,
    pub upper_inline: bool,
}

pub fn layout_range_slider_block(
    capsule_span: f32,
    label: Option<&str>,
    lower_text: Option<&str>,
    upper_text: Option<&str>,
    measure: impl Fn(&str) -> f32,
) -> RangeSliderBlockLayout {
    let lower_advance = lower_text.map_or(0.0, |text| measure(text).ceil());
    let upper_advance = upper_text.map_or(0.0, |text| measure(text).ceil());
    let label_advance = label.map_or(0.0, |text| measure(text).ceil());
    // One content inset between each of the three fixed anchors.
    let coexist = block_region_available(capsule_span, SLIDER_BLOCK_CONTENT_INSET_PX)
        >= lower_advance + upper_advance + label_advance + 2.0 * SLIDER_BLOCK_CONTENT_INSET_PX;
    RangeSliderBlockLayout {
        label_inline: label.is_some() && coexist,
        lower_inline: lower_text.is_some(),
        upper_inline: upper_text.is_some(),
    }
}

#[cfg(test)]
mod control_tests {
    use super::*;

    #[test]
    fn negative_half_ties_round_toward_positive_infinity() {
        // Portable law mirroring JavaScript Math.round in core `slider.ts`;
        // f64::round would return one step lower at these exact ties.
        assert_eq!(snap_to_step(-0.5, 0.0, 1.0), 0.0);
        assert_eq!(snap_to_step(-1.0, 0.0, 2.0), 0.0);
        assert_eq!(snap_to_step(-1.5, 0.0, 1.0), -1.0);
        assert_eq!(snap_to_step(5.0, 10.0, 10.0), 10.0);
        assert_eq!(snap_to_step(15.0, 10.0, 10.0), 20.0);
        assert_eq!(snap_to_step(0.5, 0.0, 1.0), 1.0);
        assert_eq!(snap_to_step(-1.5, -1.0, 1.0), -1.0);
    }

    // g18.024: one shared default display serializer. The snapped value
    // rounds to the precision implied by min and a finite positive step,
    // trailing zeroes trim, negative zero normalizes, and binary tails never
    // survive. Consumer-provided explicit text still wins in
    // `resolved_visible_text`.
    #[test]
    fn the_default_display_serializer_emits_short_step_aware_decimals() {
        assert_eq!(default_visible_value_text(0.8500000000000001, 0.0, 0.05), "0.85");
        assert_eq!(default_visible_value_text(0.1 + 0.2, 0.0, 0.1), "0.3");
        assert_eq!(default_visible_value_text(0.35000000000000003, 0.05, 0.1), "0.35");
        assert_eq!(default_visible_value_text(44.99999999999999, 0.0, 1.0), "45");
        assert_eq!(default_visible_value_text(80.0, 0.0, 5.0), "80");
        assert_eq!(default_visible_value_text(1.2, 0.2, 0.2), "1.2");
        assert_eq!(default_visible_value_text(-1.1102230246251565e-16, -1.0, 0.1), "0");
        assert_eq!(default_visible_value_text(-0.45, -1.0, 0.01), "-0.45");
        assert_eq!(
            default_visible_value_text(0.30000000000000004, 0.0, 0.0),
            "0.30000000000000004"
        );
        assert_eq!(resolved_visible_text(0.85, 0.0, 0.05, None), Some("0.85".to_owned()));
        assert_eq!(resolved_visible_text(0.85, 0.0, 0.05, Some("")), None);
        assert_eq!(
            resolved_visible_text(0.85, 0.0, 0.05, Some("85%")),
            Some("85%".to_owned())
        );
    }

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
        assert!(state.fill_split_at_center);
    }

    #[test]
    fn bipolar_range_only_squares_a_join_when_both_segments_meet() {
        let state = range_slider_visual_state(RangeSliderControlContext {
            value: (-0.5, 0.0),
            min: -1.0,
            max: 1.0,
            step: 0.01,
            polarity: SliderPolarity::Bipolar,
            ..RangeSliderControlContext::default()
        });
        assert!(!state.fill_split_at_center);
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

    #[test]
    fn press_move_and_end_emit_live_change_then_one_commit() {
        let mut context = SliderControlContext {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 10.0,
            ..SliderControlContext::default()
        };
        let (next, effects) = slider_control_transition(
            context,
            SliderControlEvent::PointerBegin { value_norm: 0.44 },
        );
        assert_eq!(effects, vec![SliderEffect::EmitValueChange { value: 40.0 }]);
        context = next;
        let (next, effects) = slider_control_transition(
            context,
            SliderControlEvent::PointerMove { value_norm: 0.76 },
        );
        assert_eq!(effects, vec![SliderEffect::EmitValueChange { value: 80.0 }]);
        context = next;
        let (next, effects) = slider_control_transition(context, SliderControlEvent::PointerEnd);
        assert_eq!(effects, vec![SliderEffect::EmitValueCommit { value: 80.0 }]);
        assert!(!next.pointer_active);
    }

    #[test]
    fn set_value_rebuilds_without_emitting() {
        let (next, effects) = slider_transition(
            SliderContext {
                value: 50.0,
                min: 0.0,
                max: 100.0,
                step: 10.0,
                disabled: false,
            },
            SliderEvent::SetValue { value: 70.0 },
        );
        assert_eq!(next.value, 70.0);
        assert!(effects.is_empty());
        let (next, effects) = slider_control_transition(
            SliderControlContext {
                value: 0.0,
                step: 10.0,
                ..SliderControlContext::default()
            },
            SliderControlEvent::SetValue { value: 74.0 },
        );
        assert_eq!(next.value, 70.0);
        assert!(effects.is_empty());
    }

    #[test]
    fn disabled_pointer_is_inert() {
        let context = SliderControlContext {
            disabled: true,
            value: 50.0,
            ..SliderControlContext::default()
        };
        let (_, effects) = slider_control_transition(
            context,
            SliderControlEvent::PointerBegin { value_norm: 0.9 },
        );
        assert!(effects.is_empty());
        let (_, effects) =
            slider_control_transition(context, SliderControlEvent::PointerMove { value_norm: 0.9 });
        assert!(effects.is_empty());
        let (_, effects) = slider_control_transition(context, SliderControlEvent::PointerEnd);
        assert!(effects.is_empty());
    }

    #[test]
    fn equality_fits_and_required_minus_one_falls_back() {
        assert!(block_item_fits(40.0, 40.0));
        assert!(block_item_fits(40.0, 39.2));
        assert!(!block_item_fits(40.0, 41.0));
        assert_eq!(block_region_available(56.0, 8.0), 40.0);
        assert!(block_inline_fits(&[(Some("Blur"), 56.0), (Some("67"), 56.0)], |text| {
            text.len() as f32 * 10.0
        }));
        assert!(!block_inline_fits(
            &[(Some("Blur"), 56.0), (Some("too-long-value"), 56.0)],
            |text| text.len() as f32 * 10.0
        ));
        // Whole-track law: 80px capsule → 64px available. 40+24 fits exactly;
        // one pixel more on the value suppresses the label.
        let miss = layout_slider_block(80.0, Some("Blur"), Some("67"), |text| {
            if text == "Blur" {
                40.0
            } else {
                25.0
            }
        });
        assert!(!miss.label_inline);
        assert!(miss.value_inline);
        let equal = layout_slider_block(80.0, Some("Blur"), Some("67"), |text| {
            if text == "Blur" {
                40.0
            } else {
                24.0
            }
        });
        assert!(equal.label_inline);
        assert!(equal.value_inline);
    }

    #[test]
    fn block_fit_is_value_independent() {
        // The whole-track law takes no selected span at all: low, mid and high
        // values decide identically at the same capsule width. The signature is
        // the proof — there is no value input to depend on.
        let fits = layout_slider_block(160.0, Some("Blur"), Some("67"), |text| {
            text.len() as f32 * 10.0
        });
        assert!(fits.label_inline);
        assert!(fits.value_inline);
    }

    #[test]
    fn collision_suppresses_label_and_keeps_exact_value() {
        // 100px capsule: 84px available. "Compressor" (100) + "12" (20) miss;
        // the value keeps painting, the label never renders a fallback line.
        let tight = layout_slider_block(100.0, Some("Compressor makeup gain"), Some("12"), |text| {
            text.len() as f32 * 10.0
        });
        assert!(!tight.label_inline);
        assert!(tight.value_inline);
        // No value at all: the label stands alone at the logical start.
        let solo = layout_slider_block(100.0, Some("Blur"), None, |text| {
            text.len() as f32 * 10.0
        });
        assert!(solo.label_inline);
        assert!(!solo.value_inline);
        let gone = layout_slider_block(100.0, Some("Compressor makeup gain"), None, |text| {
            text.len() as f32 * 10.0
        });
        assert!(!gone.label_inline);
        assert!(!gone.value_inline);
    }

    #[test]
    fn a_second_pointer_end_is_inert() {
        let first = slider_control_transition(
            SliderControlContext {
                value: 10.0,
                pointer_active: true,
                ..SliderControlContext::default()
            },
            SliderControlEvent::PointerEnd,
        );
        assert_eq!(first.1, vec![SliderEffect::EmitValueCommit { value: 10.0 }]);
        let second = slider_control_transition(first.0, SliderControlEvent::PointerEnd);
        assert!(second.1.is_empty());
    }
}
