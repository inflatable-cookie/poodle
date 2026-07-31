//! Slider — Jetstream slider backed by SliderSpec.
//!
//! Contract: `docs/contracts/components/slider.md`
//! Reference: `packages/svelte/components/src/Slider.svelte`
//!
//! Track layout: fixed-width flex-row with two segments (fill + remainder)
//! so the filled portion reflects the actual fraction. Thumb is absolutely
//! positioned at the junction of fill and remainder.
//!
//! All dimensions resolve from the contract §8 size table — no invented ratios.
//! Drag/keyboard interaction is preview-event-loop bound; this renders the
//! track + fill + thumb at the spec's current value only.

use jetstream_ui::{BoxShadow, Color};
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, SliderSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

/// Fixed track length — 10 rem, matching the GPUI reference basis.
fn track_w() -> f32 {
    rem_to_px(10.0)
}

/// Thumb diameter in rem per the contract §8 size table.
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

/// Slider — a value along a track.
///
/// Mirrors the GPUI target's `on_change` / `on_value_commit`.
///
/// ## Why the value comes from deltas
///
/// A drag handler is built before layout runs, so it cannot know where its
/// track ended up on screen. `DragEvent` carries the pointer's *global*
/// position, which is useless without that, but it also carries a per-frame
/// delta, which is not: a pixel delta over a known track width is a value
/// delta. The handler therefore accumulates from the spec's current value
/// rather than deriving an absolute one, which is also what makes it correct
/// when the track is nested inside something scrolled or offset.
///
/// Snapping and clamping come from `poodle_headless::slider::slider_transition`
/// — the machine the web target drives — so the two agree on what a step is.
pub struct Slider {
    spec: SliderSpec,
    theme: JetstreamThemeProvider,
    on_change: Option<std::sync::Arc<dyn Fn(f64) + Send + Sync>>,
    on_value_commit: Option<std::sync::Arc<dyn Fn(f64) + Send + Sync>>,
}

impl Slider {
    pub fn from_spec(spec: SliderSpec, theme: &JetstreamThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_change: None, on_value_commit: None }
    }

    /// Fires continuously while dragging, with the value under the pointer,
    /// clamped to the bounds and snapped to `step`.
    pub fn on_change(mut self, handler: impl Fn(f64) + Send + Sync + 'static) -> Self {
        self.on_change = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires once when the drag ends, with the settled value. Hosts that write
    /// to a store on every frame want the first; hosts that persist want this.
    pub fn on_value_commit(mut self, handler: impl Fn(f64) + Send + Sync + 'static) -> Self {
        self.on_value_commit = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for Slider {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_change, self.on_value_commit)
    }
}

pub fn js_slider(spec: &SliderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None, None)
}


fn build(
    spec: &SliderSpec,
    theme: &JetstreamThemeProvider,
    on_change: Option<std::sync::Arc<dyn Fn(f64) + Send + Sync>>,
    on_value_commit: Option<std::sync::Arc<dyn Fn(f64) + Send + Sync>>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Contract §7/§8: thumb diameter + min-height from the size table,
    // track thickness fixed at 0.375rem.
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

    let range = (spec.max - spec.min).max(0.001);
    let fraction = ((spec.value - spec.min) / range).clamp(0.0, 1.0) as f32;

    let tw = track_w();
    let fill_w = fraction * tw;
    // Remaining track width — never negative.
    let rem_w = (tw - fill_w).max(0.0);

    let thumb_r = thumb_size * 0.5;

    // Drag start requires the *exact* hit node to carry the handler — unlike
    // clicks, drags do not bubble to the nearest handler above. The pointer
    // lands on whichever segment is under it, so all three parts get the same
    // handler rather than the track that contains them.
    let drag_handler: Option<std::sync::Arc<dyn Fn(&jetstream_ui::DragEvent) + Send + Sync>> =
        if spec.is_disabled || (on_change.is_none() && on_value_commit.is_none()) {
            None
        } else {
            use std::sync::atomic::{AtomicU64, Ordering};

            // The running value for this drag. `f64` has no atomic, so it
            // travels as its bit pattern — the handler must be `Fn`, not
            // `FnMut`, and `Send + Sync`, which rules out a captured local.
            let live = std::sync::Arc::new(AtomicU64::new(spec.value.to_bits()));
            let context = poodle_headless::slider::SliderContext {
                value: spec.value,
                min: spec.min,
                max: spec.max,
                step: spec.step,
                disabled: false,
            };
            let units_per_px = (spec.max - spec.min) / track_w().max(1.0) as f64;
            let on_change = on_change.clone();
            let on_value_commit = on_value_commit.clone();

            Some(std::sync::Arc::new(move |event: &jetstream_ui::DragEvent| {
                match event.phase {
                    jetstream_ui::DragPhase::Start => {}
                    jetstream_ui::DragPhase::Move => {
                        let current = f64::from_bits(live.load(Ordering::SeqCst));
                        let (next, effects) = poodle_headless::slider::slider_transition(
                            poodle_headless::slider::SliderContext { value: current, ..context },
                            poodle_headless::slider::SliderEvent::Input {
                                raw: current + event.delta_x as f64 * units_per_px,
                            },
                        );
                        live.store(next.value.to_bits(), Ordering::SeqCst);
                        for effect in effects {
                            if let poodle_headless::slider::SliderEffect::EmitValueChange { value } = effect {
                                if let Some(handler) = &on_change {
                                    handler(value);
                                }
                            }
                        }
                    }
                    jetstream_ui::DragPhase::End => {
                        let current = f64::from_bits(live.load(Ordering::SeqCst));
                        let (_, effects) = poodle_headless::slider::slider_transition(
                            poodle_headless::slider::SliderContext { value: current, ..context },
                            poodle_headless::slider::SliderEvent::Commit { raw: current },
                        );
                        for effect in effects {
                            if let poodle_headless::slider::SliderEffect::EmitValueCommit { value } = effect {
                                if let Some(handler) = &on_value_commit {
                                    handler(value);
                                }
                            }
                        }
                    }
                }
            }))
        };

    let draggable = |el: JsEl| match &drag_handler {
        Some(handler) => {
            let handler = std::sync::Arc::clone(handler);
            el.cursor_pointer().on_drag(move |event| handler(event))
        }
        None => el,
    };

    // Fill segment: left portion in accent color.
    let fill = ui_element::div()
        .w(fill_w)
        .h(track_h)
        .bg(accent)
        .rounded_l(pill);
    let fill = draggable(fill);

    // Remainder segment: takes the rest of the track.
    let remainder = ui_element::div()
        .w(rem_w)
        .h(track_h)
        .bg(track_bg)
        .rounded_r(pill);
    let remainder = draggable(remainder);

    // Contract §8 thumb drop shadow: `0 0.125rem 0.5rem color-mix(black 18%,
    // transparent)`. Offset/blur resolve from contract-exact rem; black@0.18 has
    // no dedicated shadow token (elevation.shadow.* use a different color +
    // offsets), so the color is the one noted literal — matches the GPUI target.
    let thumb_shadow = BoxShadow {
        offset_x: 0.0,
        offset_y: rem_to_px(0.125),
        blur: rem_to_px(0.5),
        spread: 0.0,
        color: glam::Vec4::new(0.0, 0.0, 0.0, 0.18),
        inset: false,
    };

    // Thumb: absolutely positioned at the fill/remainder junction.
    // top offsets the thumb vertically to center on the track.
    let thumb_top = -(thumb_r - track_h * 0.5);
    let thumb_left = fill_w - thumb_r;
    let mut thumb = ui_element::div()
        .absolute()
        .top(thumb_top)
        .left(thumb_left)
        .w(thumb_size)
        .h(thumb_size)
        .rounded(pill)
        .bg(elevated)
        .border(border_w)
        .border_color(border_default)
        .cursor_pointer();
    thumb.style.shadow = Some(thumb_shadow);
    let thumb = draggable(thumb);

    // Track row: relative container holding fill, remainder, and thumb.
    let track = ui_element::div()
        .w(tw)
        .h(thumb_size)
        .relative()
        .flex_row()
        .items_center()
        .child(fill)
        .child(remainder)
        .child(thumb);


    let mut el = ui_element::div()
        .h(container_h)
        .grow()
        .flex_row()
        .items_center()
        .child(track);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    } else {
        // Keyboard-operable per contract — mark focusable so it can hold focus and
        // render the focus ring (the runtime draws theme.focus_color when focused).
        el = el.focusable();
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
        // Mid value so both fill and remainder segments are present.
        let el = js_slider(&SliderSpec::new(50.0), &th);
        let tree = probe(&el, 200.0, 40.0);

        let accent = resolve_color(&th, "color.accent.base");
        let surface = resolve_color(&th, "color.background.surface");

        // Contract: unfilled track = color-mix(surface 88%, transparent).
        let expected_track = vec4_to_probe(tint(surface, 0.88));
        let accent_probe = vec4_to_probe(accent);

        // The unfilled track color must be present...
        assert!(
            tree.has_background(expected_track, 0.01),
            "unfilled track should be surface mixed toward transparent (alpha 0.88): {}",
            tree.to_json()
        );
        // ...and it must NOT equal the accent (the original bug mixed toward accent).
        assert!(
            !expected_track.approx(accent_probe, 0.02),
            "unfilled track must not be the accent color"
        );
        // The track's alpha is the transparency mix, not opaque.
        assert!(
            (expected_track.a - 0.88).abs() < 0.001,
            "unfilled track alpha should be 0.88 (surface mixed toward transparent)"
        );

        // The filled portion must be the opaque accent.
        assert!(
            tree.has_background(accent_probe, 0.01),
            "filled track segment should be the accent color: {}",
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
    fn thumb_has_contract_drop_shadow() {
        let th = theme();
        let el = js_slider(&SliderSpec::new(50.0), &th);
        let mut found = Vec::new();
        shadows(&el, &mut found);

        // Contract §8 thumb shadow: 0 0.125rem 0.5rem black@0.18.
        let shadow = found
            .iter()
            .find(|s| {
                (s.offset_y - rem_to_px(0.125)).abs() < 0.5
                    && (s.blur - rem_to_px(0.5)).abs() < 0.5
            })
            .expect("thumb should carry the contract drop shadow");
        assert!((shadow.offset_x).abs() < 0.001, "shadow has no horizontal offset");
        assert!((shadow.spread).abs() < 0.001, "shadow has no spread");
        assert!(
            (shadow.color.w - 0.18).abs() < 0.001,
            "shadow color is black mixed to 18% alpha"
        );
        assert!(
            shadow.color.x < 0.001 && shadow.color.y < 0.001 && shadow.color.z < 0.001,
            "shadow color is black"
        );
    }

    #[test]
    fn thumb_and_track_sizes_match_contract_table() {
        let th = theme();
        // md thumb = 1rem = 16px, track thickness = 0.375rem = 6px,
        // min-height = 1.5rem = 24px.
        let el = js_slider(&SliderSpec::new(50.0).with_size(ControlSize::Md), &th);
        // Root container height = min-height (size role Control = identity for md).
        assert_eq!(
            el.layout.size.height,
            taffy::Dimension::length(rem_to_px(1.5))
        );

        // xs thumb = 0.75rem = 12px from the size table (not an invented ratio).
        let el_xs = js_slider(
            &SliderSpec::new(50.0)
                .with_size(ControlSize::Xs)
                .with_size_role(poodle_specs::SemanticControlSizeRole::Control),
            &th,
        );
        let tree = probe(&el_xs, 200.0, 40.0);
        // The thumb is the only square node of width == height == 12px.
        let has_xs_thumb = tree
            .nodes
            .iter()
            .any(|n| (n.w - 12.0).abs() < 0.5 && (n.h - 12.0).abs() < 0.5);
        assert!(
            has_xs_thumb,
            "xs thumb should be 0.75rem (12px) per the size table: {}",
            tree.to_json()
        );
    }

    /// Dragging right raises the value, and the reported value is snapped and
    /// clamped rather than raw pixels.
    #[test]
    fn dragging_the_track_reports_values() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
        let changes = Arc::clone(&seen);

        let spec = SliderSpec::new(50.0).with_bounds(0.0, 100.0);
        let el = Slider::from_spec(spec, &theme())
            .on_change(move |value| changes.lock().unwrap().push(value))
            .into_js_el();

        // The track is 10rem wide; drag a quarter of it to the right.
        crate::element::click_probe::drag(&el, 400.0, 80.0, (40.0, 40.0), (80.0, 40.0));

        let values = seen.lock().unwrap();
        assert!(!values.is_empty(), "a drag reported nothing");
        let last = *values.last().unwrap();
        assert!(last > 50.0, "dragging right lowered the value: {values:?}");
        assert!(last <= 100.0, "the value escaped its maximum: {last}");
        assert!(values.iter().all(|v| (v - v.round()).abs() < f64::EPSILON), "unsnapped: {values:?}");
    }

    /// Commit fires once, at the end, with the settled value — a host that
    /// persists on every frame would otherwise write a hundred times per drag.
    #[test]
    fn commit_fires_once_with_the_settled_value() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let changes: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
        let commits: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
        let c1 = Arc::clone(&changes);
        let c2 = Arc::clone(&commits);

        let spec = SliderSpec::new(50.0).with_bounds(0.0, 100.0);
        let el = Slider::from_spec(spec, &theme())
            .on_change(move |value| c1.lock().unwrap().push(value))
            .on_value_commit(move |value| c2.lock().unwrap().push(value))
            .into_js_el();

        crate::element::click_probe::drag(&el, 400.0, 80.0, (40.0, 40.0), (80.0, 40.0));

        let commits = commits.lock().unwrap();
        assert_eq!(commits.len(), 1, "commit fired {} times", commits.len());
        assert_eq!(commits[0], *changes.lock().unwrap().last().unwrap());
    }

    /// The value cannot leave its bounds however far the pointer travels.
    #[test]
    fn a_long_drag_stops_at_the_bound() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
        let changes = Arc::clone(&seen);

        let spec = SliderSpec::new(50.0).with_bounds(0.0, 100.0);
        let el = Slider::from_spec(spec, &theme())
            .on_change(move |value| changes.lock().unwrap().push(value))
            .into_js_el();

        crate::element::click_probe::drag(&el, 400.0, 80.0, (40.0, 40.0), (4000.0, 40.0));

        assert_eq!(*seen.lock().unwrap().last().unwrap(), 100.0);
    }

    #[test]
    fn a_disabled_slider_ignores_drags() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let spec = SliderSpec { is_disabled: true, ..SliderSpec::new(50.0) };
        let el = Slider::from_spec(spec, &theme())
            .on_change(move |_| { counter.fetch_add(1, Ordering::SeqCst); })
            .into_js_el();

        crate::element::click_probe::drag(&el, 400.0, 80.0, (40.0, 40.0), (80.0, 40.0));

        assert_eq!(hits.load(Ordering::SeqCst), 0, "a disabled slider moved");
    }


}
