//! RangeSlider — Jetstream range slider backed by RangeSliderSpec.
//!
//! Contract: `docs/contracts/components/range-slider.md`
//! Reference: `packages/svelte/components/src/RangeSlider.svelte`
//!
//! Track layout: three fixed-width segments (lo-unfilled | filled | hi-unfilled)
//! side by side in a flex-row. The between-fill needs an offset start; JsEl has
//! no percent sizing and `ui_element::progress` only fills from the left edge,
//! so the offset start is produced structurally by the leading `lo-unfilled`
//! segment (a real fixed-px width = `lo * track_w`). The filled segment is then
//! `(hi - lo) * track_w` px wide and sits at the correct offset — no percent or
//! absolute-left math needed. Two thumbs are absolutely positioned at the
//! segment junctions so each sits at its value position.
//!
//! All dimensions resolve from the contract §8 size table — no invented ratios.
//! Drag/keyboard interaction is preview-event-loop bound; this renders the
//! track + filled window + two thumbs at the spec's current values only.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::{BoxShadow, Color};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, RangeSliderSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

/// Fixed track length — 10 rem, matching the single-thumb Slider basis.
fn track_w() -> f32 {
    rem_to_px(10.0)
}

/// Thumb diameter in rem per the contract §8 size table (same as Slider).
fn thumb_diameter_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Root min-height in rem per the contract §8 size table.
/// (lg/xl inherit the md base of 1.5rem.)
fn min_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.25,
        ControlSize::Sm => 1.375,
        ControlSize::Md | ControlSize::Lg | ControlSize::Xl => 1.5,
    }
}

/// Snap a normalized fraction's underlying value to the step grid anchored at
/// `min` (`min + n*step`), matching Svelte `snapToStep(raw, min, step)`, then
/// re-normalize to 0..1. A zero/absent step returns the fraction unchanged.
fn snap_fraction(frac: f32, min: f64, max: f64, step: f64) -> f32 {
    if step <= 0.0 || max <= min {
        return frac.clamp(0.0, 1.0);
    }
    let raw = min + (frac as f64) * (max - min);
    let snapped = (min + ((raw - min) / step).round() * step).clamp(min, max);
    (((snapped - min) / (max - min)) as f32).clamp(0.0, 1.0)
}

/// RangeSlider — a low and a high value along one track.
///
/// Mirrors the GPUI target's `on_change` / `on_value_commit`, reporting both
/// values together: the pair is the value, and a host told about one alone
/// would have to remember the other.
///
/// Each thumb is dragged separately. Only the thumbs are draggable here, not
/// the segments between them: dragging the middle of a range on the web moves
/// the whole window, which needs both values to travel together and is a
/// different gesture from moving one end.
///
/// Snapping, clamping and the no-crossing rule come from
/// `poodle_headless::slider::range_slider_transition` — the same machine the
/// web target drives — rather than being re-derived here. A first attempt did
/// re-derive them, chose to *swap* a thumb dragged past its partner, and
/// collapsed both onto one value; the shared machine clamps instead.
pub struct RangeSlider {
    spec: RangeSliderSpec,
    theme: JetstreamThemeProvider,
    on_change: Option<std::sync::Arc<dyn Fn(f64, f64) + Send + Sync>>,
    on_value_commit: Option<std::sync::Arc<dyn Fn(f64, f64) + Send + Sync>>,
}

impl RangeSlider {
    pub fn from_spec(spec: RangeSliderSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
            on_value_commit: None,
        }
    }

    /// Fires continuously while dragging, with `(low, high)`. A thumb cannot
    /// cross its sibling; it stops against it.
    pub fn on_change(mut self, handler: impl Fn(f64, f64) + Send + Sync + 'static) -> Self {
        self.on_change = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires once when the drag ends, with the settled pair.
    pub fn on_value_commit(mut self, handler: impl Fn(f64, f64) + Send + Sync + 'static) -> Self {
        self.on_value_commit = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for RangeSlider {
    fn into_js_el(self) -> JsEl {
        build(
            &self.spec,
            &self.theme,
            self.on_change,
            self.on_value_commit,
        )
    }
}

pub fn js_range_slider(spec: &RangeSliderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None, None)
}

fn build(
    spec: &RangeSliderSpec,
    theme: &JetstreamThemeProvider,
    on_change: Option<std::sync::Arc<dyn Fn(f64, f64) + Send + Sync>>,
    on_value_commit: Option<std::sync::Arc<dyn Fn(f64, f64) + Send + Sync>>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Contract §7/§8: thumb diameter + min-height from the size table,
    // track thickness fixed at 0.375rem (size-invariant, same as Slider).
    let thumb_size = rem_to_px(thumb_diameter_rem(effective_size));
    let track_h = rem_to_px(0.375);
    let container_h = rem_to_px(min_height_rem(effective_size));

    // Pill radius (contract: 999px full-pill) and thumb border (0.0625rem).
    let pill = resolve_radius(theme, "radius.pill");
    let border_w = rem_to_px(0.0625);

    let accent: Color = resolve_color(theme, spec.range_fill_token()).into();
    let surface = resolve_color(theme, "color.background.surface");
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();

    // Contract §8 track bg = color-mix(surface 88%, transparent):
    // mix surface toward transparency (alpha 0.88), NOT toward accent.
    let track_bg: Color = tint(surface, 0.88).into();

    // Normalized positions, step-snapped to the contract grid (anchored at min).
    let lo = snap_fraction(spec.normalized_low() as f32, spec.min, spec.max, spec.step);
    let hi = snap_fraction(spec.normalized_high() as f32, spec.min, spec.max, spec.step).max(lo);

    let tw = track_w();
    let thumb_r = thumb_size * 0.5;

    // Three proportional segments (fixed px widths, never negative).
    let lo_w = (lo * tw).max(0.0);
    let fill_w = ((hi - lo) * tw).max(0.0);
    let hi_w = ((1.0 - hi) * tw).max(0.0);

    // Leading unfilled segment: pill-rounded left so the track reads as a pill.
    let seg_lo = ui_element::div()
        .w(lo_w)
        .h(track_h)
        .bg(track_bg)
        .rounded_l(pill);

    // Filled window between the thumbs: accent.
    let seg_fill = ui_element::div().w(fill_w).h(track_h).bg(accent);

    // Trailing unfilled segment: pill-rounded right.
    let seg_hi = ui_element::div()
        .w(hi_w)
        .h(track_h)
        .bg(track_bg)
        .rounded_r(pill);

    // Contract §8 thumb drop shadow: `0 0.125rem 0.5rem color-mix(black 18%,
    // transparent)` (same as the single Slider). Offset/blur resolve from
    // contract-exact rem; black@0.18 has no dedicated shadow token, so the color
    // is the one noted literal — matches the GPUI target.
    let thumb_shadow = BoxShadow {
        offset_x: 0.0,
        offset_y: rem_to_px(0.125),
        blur: rem_to_px(0.5),
        spread: 0.0,
        color: glam::Vec4::new(0.0, 0.0, 0.0, 0.18),
        inset: false,
    };

    // Thumbs absolutely positioned at the segment junctions.
    // top offsets each thumb to center vertically on the track.
    let thumb_top = -(thumb_r - track_h * 0.5);

    let mut thumb_lo = ui_element::div()
        .absolute()
        .top(thumb_top)
        .left(lo_w - thumb_r)
        .w(thumb_size)
        .h(thumb_size)
        .rounded(pill)
        .bg(elevated)
        .border(border_w)
        .border_color(border_default)
        .cursor_pointer();
    thumb_lo.style.shadow = Some(thumb_shadow);

    let mut thumb_hi = ui_element::div()
        .absolute()
        .top(thumb_top)
        .left((lo_w + fill_w) - thumb_r)
        .w(thumb_size)
        .h(thumb_size)
        .rounded(pill)
        .bg(elevated)
        .border(border_w)
        .border_color(border_default)
        .cursor_pointer();
    thumb_hi.style.shadow = Some(thumb_shadow);

    // One shared pair of values, so dragging either thumb reports both.
    let (thumb_lo, thumb_hi) = {
        use poodle_headless::slider::{
            range_slider_transition, RangeSliderContext, RangeSliderEffect, RangeSliderEvent,
            RangeThumb,
        };
        use std::sync::atomic::{AtomicU64, Ordering};

        if spec.is_disabled || (on_change.is_none() && on_value_commit.is_none()) {
            (thumb_lo, thumb_hi)
        } else {
            let low = std::sync::Arc::new(AtomicU64::new(spec.low.to_bits()));
            let high = std::sync::Arc::new(AtomicU64::new(spec.high.to_bits()));
            let context = RangeSliderContext {
                value: (spec.low, spec.high),
                min: spec.min,
                max: spec.max,
                step: spec.step,
                disabled: false,
            };
            let units_per_px = (spec.max - spec.min) / tw.max(1.0) as f64;

            let arm = |thumb: RangeThumb,
                       el: JsEl,
                       low: std::sync::Arc<AtomicU64>,
                       high: std::sync::Arc<AtomicU64>,
                       on_change: Option<std::sync::Arc<dyn Fn(f64, f64) + Send + Sync>>,
                       on_value_commit: Option<std::sync::Arc<dyn Fn(f64, f64) + Send + Sync>>| {
                el.on_drag(move |event| {
                    let live = (
                        f64::from_bits(low.load(Ordering::SeqCst)),
                        f64::from_bits(high.load(Ordering::SeqCst)),
                    );
                    let context = RangeSliderContext { value: live, ..context };

                    let (raw, machine_event): (f64, fn(RangeThumb, f64) -> RangeSliderEvent) =
                        match event.phase {
                            jetstream_ui::DragPhase::Start => return,
                            jetstream_ui::DragPhase::Move => (
                                match thumb {
                                    RangeThumb::Lower => live.0,
                                    RangeThumb::Upper => live.1,
                                } + event.delta_x as f64 * units_per_px,
                                |thumb, raw| RangeSliderEvent::Input { thumb, raw },
                            ),
                            jetstream_ui::DragPhase::End => (
                                match thumb {
                                    RangeThumb::Lower => live.0,
                                    RangeThumb::Upper => live.1,
                                },
                                |thumb, raw| RangeSliderEvent::Commit { thumb, raw },
                            ),
                        };

                    let (next, effects) = range_slider_transition(context, machine_event(thumb, raw));
                    low.store(next.value.0.to_bits(), Ordering::SeqCst);
                    high.store(next.value.1.to_bits(), Ordering::SeqCst);

                    for effect in effects {
                        match effect {
                            RangeSliderEffect::EmitValueChange { value } => {
                                if let Some(handler) = &on_change {
                                    handler(value.0, value.1);
                                }
                            }
                            RangeSliderEffect::EmitValueCommit { value } => {
                                if let Some(handler) = &on_value_commit {
                                    handler(value.0, value.1);
                                }
                            }
                        }
                    }
                })
            };

            (
                arm(
                    RangeThumb::Lower,
                    thumb_lo,
                    std::sync::Arc::clone(&low),
                    std::sync::Arc::clone(&high),
                    on_change.clone(),
                    on_value_commit.clone(),
                ),
                arm(
                    RangeThumb::Upper,
                    thumb_hi,
                    low,
                    high,
                    on_change,
                    on_value_commit,
                ),
            )
        }
    };

    // Track row: relative container holding the three segments and both thumbs.
    let track = ui_element::div()
        .w(tw)
        .h(thumb_size)
        .relative()
        .flex_row()
        .items_center()
        .child(seg_lo)
        .child(seg_fill)
        .child(seg_hi)
        .child(thumb_lo)
        .child(thumb_hi);

    let mut el = ui_element::div()
        .h(container_h)
        .grow()
        .flex_row()
        .items_center()
        .child(track);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn vec4_to_probe(c: glam::Vec4) -> ProbeColor {
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    #[test]
    fn unfilled_track_mixes_toward_transparent_not_accent() {
        let th = theme();
        // 20..80 so all three segments (lo / fill / hi) are present.
        let el = js_range_slider(&RangeSliderSpec::new(20.0, 80.0), &th);
        let tree = probe(&el, 240.0, 40.0);

        let accent = resolve_color(&th, "color.accent.base");
        let surface = resolve_color(&th, "color.background.surface");

        let expected_track = vec4_to_probe(tint(surface, 0.88));
        let accent_probe = vec4_to_probe(accent);

        // Unfilled track = color-mix(surface 88%, transparent), present...
        assert!(
            tree.has_background(expected_track, 0.01),
            "unfilled track should be surface mixed toward transparent (alpha 0.88): {}",
            tree.to_json()
        );
        // ...and NOT the accent (the original bug mixed toward accent).
        assert!(
            !expected_track.approx(accent_probe, 0.02),
            "unfilled track must not be the accent color"
        );
        assert!(
            (expected_track.a - 0.88).abs() < 0.001,
            "unfilled track alpha should be 0.88 (surface mixed toward transparent)"
        );
        // The between-fill segment must be the opaque accent.
        assert!(
            tree.has_background(accent_probe, 0.01),
            "filled window should be the accent color: {}",
            tree.to_json()
        );
    }

    /// Walk the raw JsEl tree (the probe flattens away `style.shadow`) and
    /// collect every node carrying a box shadow.
    fn shadows(el: &JsEl, out: &mut Vec<jetstream_ui::BoxShadow>) {
        if let Some(s) = el.style.shadow {
            out.push(s);
        }
        for c in &el.children {
            shadows(c, out);
        }
    }

    #[test]
    fn both_thumbs_have_contract_drop_shadow() {
        let th = theme();
        let el = js_range_slider(&RangeSliderSpec::new(20.0, 80.0), &th);
        let mut found = Vec::new();
        shadows(&el, &mut found);

        // Contract §8 thumb shadow: 0 0.125rem 0.5rem black@0.18, on BOTH thumbs.
        let matching: Vec<_> = found
            .iter()
            .filter(|s| {
                (s.offset_y - rem_to_px(0.125)).abs() < 0.5
                    && (s.blur - rem_to_px(0.5)).abs() < 0.5
                    && (s.color.w - 0.18).abs() < 0.001
                    && s.color.x < 0.001
                    && s.color.y < 0.001
                    && s.color.z < 0.001
            })
            .collect();
        assert_eq!(
            matching.len(),
            2,
            "both thumbs should carry the contract drop shadow; found {}",
            matching.len()
        );
    }

    #[test]
    fn two_thumbs_render_at_md_size() {
        let th = theme();
        // md thumb = 1rem = 16px from the contract size table (not a ratio).
        let el = js_range_slider(
            &RangeSliderSpec::new(20.0, 80.0).with_size(ControlSize::Md),
            &th,
        );
        // Root container height = min-height md = 1.5rem = 24px.
        assert_eq!(
            el.layout.size.height,
            taffy::Dimension::length(rem_to_px(1.5))
        );

        let tree = probe(&el, 240.0, 40.0);
        // Exactly two 16x16 square thumbs.
        let thumb_count = tree
            .nodes
            .iter()
            .filter(|n| (n.w - 16.0).abs() < 0.5 && (n.h - 16.0).abs() < 0.5)
            .count();
        assert_eq!(
            thumb_count,
            2,
            "two md thumbs (1rem = 16px) expected per the size table: {}",
            tree.to_json()
        );
    }

    #[test]
    fn between_fill_is_offset_and_proportional() {
        let th = theme();
        // 25..75 of 0..100 → fill spans the middle 50% of a 160px track = 80px,
        // offset 40px from the track's left edge (the lo-unfilled segment).
        let el = js_range_slider(&RangeSliderSpec::new(25.0, 75.0), &th);
        let tree = probe(&el, 240.0, 40.0);

        let accent = vec4_to_probe(resolve_color(&th, "color.accent.base"));
        // The accent-filled node is the between-fill window.
        let fill = tree
            .nodes
            .iter()
            .find(|n| n.bg.is_some_and(|b| b.approx(accent, 0.01)))
            .expect("between-fill (accent) segment should exist");

        // Width ≈ 50% of 160px = 80px.
        assert!(
            (fill.w - 80.0).abs() < 1.0,
            "between-fill width should be ~80px (50% of 160px track): got {} in {}",
            fill.w,
            tree.to_json()
        );
        // The fill starts offset from the track's left edge (lo segment ahead of it),
        // so its x is greater than the track origin — not flush-left like a left-only fill.
        let track = &tree.nodes[0];
        assert!(
            fill.x > track.x + 1.0,
            "between-fill should be offset (not flush-left): fill.x={} track.x={}",
            fill.x,
            track.x
        );
    }

    #[test]
    fn disabled_applies_state_opacity() {
        let th = theme();
        let el = js_range_slider(&RangeSliderSpec::new(30.0, 70.0).with_disabled(true), &th);
        let expected = resolve_opacity(&th, "state.opacity.disabled");
        assert!(
            (el.style.opacity - expected).abs() < 0.001,
            "disabled range slider should apply state.opacity.disabled"
        );
    }

    /// Both values travel together: the pair *is* the value, and a host told
    /// about one alone would have to remember the other.
    #[test]
    fn dragging_a_thumb_reports_both_values() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<(f64, f64)>>> = Arc::new(Mutex::new(Vec::new()));
        let changes = Arc::clone(&seen);

        let spec = RangeSliderSpec::new(20.0, 80.0)
            .with_bounds(0.0, 100.0)
            .with_step(1.0);
        let el = RangeSlider::from_spec(spec, &theme())
            .on_change(move |low, high| changes.lock().unwrap().push((low, high)))
            .into_js_el();

        // The low thumb sits at 20% of a 10rem track.
        let tree = crate::render_probe::probe(&el, 400.0, 80.0);
        let thumb = tree
            .nodes
            .iter()
            .filter(|n| n.w == n.h && n.w > 8.0)
            .min_by(|a, b| a.x.total_cmp(&b.x))
            .expect("a thumb");
        let y = thumb.y + thumb.h / 2.0;
        crate::element::click_probe::drag(
            &el,
            400.0,
            80.0,
            (thumb.x + thumb.w / 2.0, y),
            (thumb.x + 40.0, y),
        );

        let values = seen.lock().unwrap();
        let (low, high) = *values.last().expect("a drag reported nothing");
        assert!(low > 20.0, "the low thumb did not move: {values:?}");
        assert_eq!(high, 80.0, "the untouched thumb moved");
    }

    /// A thumb stops against its partner rather than crossing it, which is
    /// what `range_slider_transition` does for the web target too. An earlier
    /// hand-rolled version swapped instead, and collapsed both onto one value.
    #[test]
    fn a_thumb_stops_against_its_partner() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<(f64, f64)>>> = Arc::new(Mutex::new(Vec::new()));
        let changes = Arc::clone(&seen);

        let spec = RangeSliderSpec::new(20.0, 80.0)
            .with_bounds(0.0, 100.0)
            .with_step(1.0);
        let el = RangeSlider::from_spec(spec, &theme())
            .on_change(move |low, high| changes.lock().unwrap().push((low, high)))
            .into_js_el();

        let tree = crate::render_probe::probe(&el, 400.0, 80.0);
        let thumb = tree
            .nodes
            .iter()
            .filter(|n| n.w == n.h && n.w > 8.0)
            .min_by(|a, b| a.x.total_cmp(&b.x))
            .expect("a thumb");
        let y = thumb.y + thumb.h / 2.0;
        crate::element::click_probe::drag(
            &el,
            400.0,
            80.0,
            (thumb.x + thumb.w / 2.0, y),
            (thumb.x + 200.0, y),
        );

        let values = seen.lock().unwrap();
        // Assert the drag happened before asserting anything about it: `all`
        // over an empty list is true, so without this the test passes when the
        // wiring is removed entirely.
        assert!(!values.is_empty(), "the drag reported nothing");
        assert!(
            values.iter().all(|(low, high)| low <= high),
            "the pair inverted: {values:?}"
        );
        let (low, high) = *values.last().unwrap();
        assert_eq!(high, 80.0, "the partner moved");
        assert_eq!(
            low, 80.0,
            "the dragged thumb did not stop against its partner: {low}"
        );
    }

    #[test]
    fn a_disabled_range_slider_ignores_drags() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let spec = RangeSliderSpec::new(20.0, 80.0).with_disabled(true);
        let el = RangeSlider::from_spec(spec, &theme())
            .on_change(move |_, _| {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::drag(&el, 400.0, 80.0, (40.0, 40.0), (120.0, 40.0));

        assert_eq!(
            hits.load(Ordering::SeqCst),
            0,
            "a disabled range slider moved"
        );
    }

    /// Commit fires once per drag, with the settled pair.
    #[test]
    fn commit_fires_once_with_the_settled_pair() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let changes: Arc<Mutex<Vec<(f64, f64)>>> = Arc::new(Mutex::new(Vec::new()));
        let commits: Arc<Mutex<Vec<(f64, f64)>>> = Arc::new(Mutex::new(Vec::new()));
        let c1 = Arc::clone(&changes);
        let c2 = Arc::clone(&commits);

        let spec = RangeSliderSpec::new(20.0, 80.0)
            .with_bounds(0.0, 100.0)
            .with_step(1.0);
        let el = RangeSlider::from_spec(spec, &theme())
            .on_change(move |low, high| c1.lock().unwrap().push((low, high)))
            .on_value_commit(move |low, high| c2.lock().unwrap().push((low, high)))
            .into_js_el();

        let tree = crate::render_probe::probe(&el, 400.0, 80.0);
        let thumb = tree
            .nodes
            .iter()
            .filter(|n| n.w == n.h && n.w > 8.0)
            .min_by(|a, b| a.x.total_cmp(&b.x))
            .expect("a thumb");
        let y = thumb.y + thumb.h / 2.0;
        crate::element::click_probe::drag(
            &el,
            400.0,
            80.0,
            (thumb.x + thumb.w / 2.0, y),
            (thumb.x + 40.0, y),
        );

        let commits = commits.lock().unwrap();
        assert_eq!(commits.len(), 1, "commit fired {} times", commits.len());
        assert_eq!(commits[0], *changes.lock().unwrap().last().unwrap());
    }
}
