//! RangeSlider — real GPUI component backed by `RangeSliderSpec`.
//!
//! Pointer interaction uses the track element's layout bounds (via `on_children_prepainted`),
//! not the window bounds. Dual-thumb drag state is keyed by the `interaction_key` passed to
//! [`RangeSlider::on_change`] so it survives parent re-renders while dragging.
//!
//! # Known GPUI deltas
//! - Vertical orientation: not implemented. The build returns a horizontal layout
//!   regardless of `spec.orientation`. Tracked for a future pass.
//! - Per-thumb keyboard focus (Tab cycling between thumbs): GPUI 0.2.2 does not
//!   provide per-element focus within a single stateless render tree. Keyboard nav
//!   is implemented as a single handler on the wrapper: Left/Down adjusts the low
//!   thumb, Right/Up adjusts the high thumb.
//! - `on_value_commit` fires on click-release (`on_click`). GPUI 0.2.2 does not
//!   expose `on_mouse_up` through the fluent builder.

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, OnceLock};

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, Orientation, RangeSliderSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

static RANGE_SLIDER_ID_COUNTER: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

#[derive(Clone, Copy, PartialEq, Eq)]
enum DragThumb {
    Low,
    High,
}

fn range_drag_map() -> &'static Mutex<HashMap<String, DragThumb>> {
    static MAP: OnceLock<Mutex<HashMap<String, DragThumb>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Thumb diameter in rem per the contract §8 size table (matches Svelte +
/// the single Slider). lg/xl scale up from the md 1rem base.
fn thumb_diameter_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Snap to the step grid **anchored at `min`** (`min + n*step`), matching
/// Svelte `snapToStep(raw, min, step)`. Anchoring at 0 would land off-grid
/// when `min` is not a multiple of `step` (e.g. min=18, step=5).
fn step_clamp(v: f64, min: f64, max: f64, step: f64) -> f64 {
    let stepped = if step > 0.0 {
        min + ((v - min) / step).round() * step
    } else {
        v
    };
    stepped.clamp(min, max)
}

fn value_from_x(pos_x: Pixels, track: &Bounds<Pixels>, min: f64, max: f64, step: f64) -> f64 {
    if max <= min {
        return min;
    }
    let local = pos_x - track.origin.x;
    let ratio_f = local / track.size.width;
    let ratio = (ratio_f as f64).clamp(0.0, 1.0);
    let raw = min + ratio * (max - min);
    step_clamp(raw, min, max, step)
}

/// A real GPUI dual-thumb range slider component backed by `RangeSliderSpec`.
pub struct RangeSlider {
    spec: RangeSliderSpec,
    theme: GpuiThemeProvider,
    on_change: Option<(
        String,
        Box<dyn Fn(&(f64, f64), &mut Window, &mut App) + 'static>,
    )>,
    /// Fires on click-release. See module-level GPUI delta note.
    on_value_commit: Option<Box<dyn Fn(&(f64, f64), &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for RangeSlider {
    type Target = RangeSliderSpec;
    fn deref(&self) -> &RangeSliderSpec {
        &self.spec
    }
}

impl RangeSlider {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: RangeSliderSpec::default(),
            theme: theme.clone(),
            on_change: None,
            on_value_commit: None,
        }
    }

    pub fn from_spec(spec: RangeSliderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
            on_value_commit: None,
        }
    }

    /// Report `(low, high)` when the user drags a thumb or clicks the track.
    ///
    /// `interaction_key` must be **unique** among range sliders that can be active at once.
    /// It keys internal drag state so the active thumb is remembered across `cx.notify()` rebuilds.
    pub fn on_change(
        mut self,
        interaction_key: impl Into<String>,
        f: impl Fn(&(f64, f64), &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some((interaction_key.into(), Box::new(f)));
        self
    }

    /// Register a callback fired when the user completes an interaction.
    /// See module-level GPUI delta note for limitations.
    pub fn on_value_commit(
        mut self,
        handler: impl Fn(&(f64, f64), &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_commit = Some(Box::new(handler));
        self
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn low(mut self, v: f64) -> Self {
        self.spec.low = v;
        self
    }
    pub fn high(mut self, v: f64) -> Self {
        self.spec.high = v;
        self
    }
    pub fn min(mut self, v: f64) -> Self {
        self.spec.min = v;
        self
    }
    pub fn max(mut self, v: f64) -> Self {
        self.spec.max = v;
        self
    }
    pub fn step(mut self, v: f64) -> Self {
        self.spec.step = v;
        self
    }
    pub fn orientation(mut self, v: Orientation) -> Self {
        self.spec.orientation = v;
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
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
}

impl IntoElement for RangeSlider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let accent = resolve_color(theme, spec.range_fill_token());
        let border = resolve_color(theme, "color.border.default");
        let surface_bg = resolve_color(theme, spec.track_fill_token());
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        // Track height: fixed 0.375rem (6 px) matching the Svelte reference.
        // No per-size token exists for slider track height yet.
        let track_height_f: f32 = rem_to_px(0.375); // 6 px
        let track_height = px(track_height_f);
        // Full pill radius from radius.pill (contract §8 border-radius: 999px).
        let track_radius = resolve_radius(theme, "radius.pill");

        // Thumb diameter from the contract §8 size table (resolves per spec.size),
        // not a fixed size.icon.md — xs/sm/lg/xl now render their own diameter.
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let thumb_f: f32 = rem_to_px(thumb_diameter_rem(effective_size));
        let thumb_size = px(thumb_f);
        let thumb_radius = px(thumb_f / 2.0);

        let norm_low = spec.normalized_low().clamp(0.0, 1.0) as f32;
        let norm_high = spec.normalized_high().clamp(0.0, 1.0) as f32;

        let track_bounds_store: Arc<Mutex<Option<Bounds<Pixels>>>> = Arc::new(Mutex::new(None));
        let track_bounds_for_prepaint = track_bounds_store.clone();

        // Thumb style: elevated background + border-default (matches single Slider).
        // Contract §8 thumb box-shadow: 0 0.125rem 0.5rem color-mix(black 18%,
        // transparent). Offset/blur resolve from rem; black@0.18 has no dedicated
        // shadow token (elevation.* differ), so it is the one noted literal.
        let thumb_shadow = vec![gpui::BoxShadow {
            color: hsla(0.0, 0.0, 0.0, 0.18),
            offset: point(px(0.0), px(rem_to_px(0.125))),
            blur_radius: px(rem_to_px(0.5)),
            spread_radius: px(0.0),
        }];

        // Track with filled range between low and high thumbs
        // Svelte: track bg = color-mix(surface 88%, transparent); no border
        let track_bg = Hsla { a: surface_bg.a * 0.88, ..surface_bg };
        let track_inner = div()
            .w_full()
            .h(track_height)
            .rounded(track_radius)
            .bg(track_bg)
            .relative()
            // Filled range segment
            .child(
                div()
                    .absolute()
                    .top_0()
                    .h_full()
                    .rounded(track_radius)
                    .bg(accent)
                    .left(relative(norm_low))
                    .w(relative(norm_high - norm_low)),
            )
            // Low thumb — elevated bg + border-default (matches single Slider contract)
            .child(
                div()
                    .absolute()
                    .top(px(-(thumb_f - track_height_f) / 2.0))
                    .left(relative(norm_low))
                    .ml(px(-(thumb_f / 2.0)))
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(elevated_bg)
                    .border_1()
                    .border_color(border)
                    .shadow(thumb_shadow.clone()),
            )
            // High thumb — same styling
            .child(
                div()
                    .absolute()
                    .top(px(-(thumb_f - track_height_f) / 2.0))
                    .left(relative(norm_high))
                    .ml(px(-(thumb_f / 2.0)))
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(elevated_bg)
                    .border_1()
                    .border_color(border)
                    .shadow(thumb_shadow),
            );

        let track = div()
            .w_full()
            .on_children_prepainted(move |children_bounds, _window, _cx| {
                if let Some(b) = children_bounds.first() {
                    if let Ok(mut g) = track_bounds_for_prepaint.lock() {
                        *g = Some(*b);
                    }
                }
            })
            .child(track_inner);

        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let slider_id = SharedString::from(format!(
            "poodle-range-slider-{}",
            RANGE_SLIDER_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));

        let min = spec.min;
        let max = spec.max;
        let step = spec.step;
        let low = spec.clamped_low();
        let high = spec.clamped_high();
        let is_disabled = spec.is_disabled;
        let orientation = spec.orientation;

        let mut wrapper = div()
            .id(slider_id)
            .focusable()
            .w_full()
            .flex()
            .flex_col()
            .cursor(if is_disabled {
                CursorStyle::OperationNotAllowed
            } else {
                CursorStyle::PointingHand
            })
            .child(track);

        wrapper = wrapper.focus(move |s| {
            s.border_color(focus_ring)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
        });

        if is_disabled {
            wrapper = wrapper.opacity(disabled_opacity);
        } else {
            if let Some((interaction_key, on_change)) = self.on_change {
                let on_change = Rc::new(on_change);

                let emit: Rc<dyn Fn(DragThumb, f64, f64, f64, &mut Window, &mut App)> =
                    Rc::new({
                        let on_change = on_change.clone();
                        move |side, v, low, high, window, cx| {
                            let (nl, nh) = match side {
                                DragThumb::Low => (v.min(high), high),
                                DragThumb::High => (low, v.max(low)),
                            };
                            on_change(&(nl, nh), window, cx);
                        }
                    });
                let emit_down = emit.clone();
                let emit_move = emit.clone();

                let thumb_half = px(thumb_f * 0.5 + 2.0);

                let key_down = interaction_key.clone();
                let key_move = interaction_key.clone();
                let key_clear = interaction_key.clone();

                let tbs_down = track_bounds_store.clone();
                let tbs_move = track_bounds_store.clone();

                wrapper = wrapper
                    .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                        if orientation != Orientation::Horizontal {
                            return;
                        }
                        let tb = tbs_down.lock().ok().and_then(|g| *g);
                        let Some(track) = tb else { return };

                        let pos = event.position.x;
                        let v = value_from_x(pos, &track, min, max, step);

                        let cx_low = track.origin.x + norm_low * track.size.width;
                        let cx_high = track.origin.x + norm_high * track.size.width;

                        let d_low = (pos - cx_low).abs();
                        let d_high = (pos - cx_high).abs();

                        let side = if d_low <= thumb_half && d_high <= thumb_half {
                            if d_low <= d_high { DragThumb::Low } else { DragThumb::High }
                        } else if d_low <= thumb_half {
                            DragThumb::Low
                        } else if d_high <= thumb_half {
                            DragThumb::High
                        } else if d_low <= d_high {
                            DragThumb::Low
                        } else {
                            DragThumb::High
                        };

                        if let Ok(mut map) = range_drag_map().lock() {
                            map.insert(key_down.clone(), side);
                        }

                        emit_down(side, v, low, high, window, cx);
                    })
                    .on_mouse_move(move |event, window, cx| {
                        if orientation != Orientation::Horizontal {
                            return;
                        }
                        if event.pressed_button != Some(MouseButton::Left) {
                            if let Ok(mut map) = range_drag_map().lock() {
                                map.remove(&key_clear);
                            }
                            return;
                        }

                        let side = range_drag_map()
                            .lock()
                            .ok()
                            .and_then(|g| g.get(&key_move).copied());
                        let Some(side) = side else { return };

                        let tb = tbs_move.lock().ok().and_then(|g| *g);
                        let Some(track) = tb else { return };

                        let v = value_from_x(event.position.x, &track, min, max, step);
                        emit_move(side, v, low, high, window, cx);
                    });

                // Keyboard navigation:
                // Left/Down → decrement low thumb; Right/Up → increment high thumb.
                // Contract requires per-thumb focus and Tab cycling; this is a
                // single-focus simplification (GPUI 0.2.2 delta — see module doc).
                let emit_key = emit;
                wrapper = wrapper.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let delta = if step > 0.0 { step } else { (max - min) / 100.0 };
                    match event.keystroke.key.as_str() {
                        "left" | "down" => {
                            let new_low = step_clamp(low - delta, min, max, step);
                            emit_key(DragThumb::Low, new_low, low, high, window, cx);
                        }
                        "right" | "up" => {
                            let new_high = step_clamp(high + delta, min, max, step);
                            emit_key(DragThumb::High, new_high, low, high, window, cx);
                        }
                        _ => {}
                    }
                });
            }

            // on_value_commit: fires on click-release via on_click.
            if let Some(commit_handler) = self.on_value_commit {
                let snapshot = (low, high);
                let tbs_click = track_bounds_store;
                wrapper = wrapper.on_click(move |event, window, cx| {
                    let tb = tbs_click.lock().ok().and_then(|g| *g);
                    let val = if let Some(track) = tb {
                        let v = value_from_x(event.position().x, &track, min, max, step);
                        // Commit with whichever thumb is closer to the click position.
                        let cx_low = track.origin.x + norm_low * track.size.width;
                        let cx_high = track.origin.x + norm_high * track.size.width;
                        let d_low = (event.position().x - cx_low).abs();
                        let d_high = (event.position().x - cx_high).abs();
                        if d_low <= d_high {
                            (v.min(snapshot.1), snapshot.1)
                        } else {
                            (snapshot.0, v.max(snapshot.0))
                        }
                    } else {
                        snapshot
                    };
                    commit_handler(&val, window, cx);
                });
            }
        }

        wrapper.into_any_element()
    }
}
