//! Slider — real GPUI component backed by SliderSpec.
//!
//! Drag and click use the track element's layout bounds (via `on_children_prepainted`),
//! not the window bounds.
//!
//! # Known GPUI deltas
//! - `aria-valuemin/max/now/text`, `aria-disabled`: not expressible on GPUI native
//!   elements via the fluent Div builder.
//! - `on_value_commit` fires on click-release (`on_click`). GPUI 0.2.2 does not expose
//!   `on_mouse_up` through the fluent builder, so drag-release commits are not captured.

use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::SliderSpec;

use poodle_adapter::ThemeProvider;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity};

static SLIDER_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// A real GPUI slider component backed by `SliderSpec`.
pub struct Slider {
    spec: SliderSpec,
    theme: GpuiThemeProvider,
    id: Option<SharedString>,
    on_change: Option<Box<dyn Fn(&f64, &mut Window, &mut App) + 'static>>,
    /// Fires on click-release. See module-level GPUI delta note.
    on_value_commit: Option<Box<dyn Fn(&f64, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Slider {
    type Target = SliderSpec;
    fn deref(&self) -> &SliderSpec {
        &self.spec
    }
}

impl Slider {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SliderSpec::default(),
            theme: theme.clone(),
            id: None,
            on_change: None,
            on_value_commit: None,
        }
    }

    pub fn from_spec(spec: SliderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id: None,
            on_change: None,
            on_value_commit: None,
        }
    }

    pub fn with_id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&f64, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Register a callback fired when the user completes an interaction.
    /// See module-level GPUI delta note for limitations.
    pub fn on_value_commit(
        mut self,
        handler: impl Fn(&f64, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_commit = Some(Box::new(handler));
        self
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self {
        self.spec.value = v;
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
    pub fn orientation(mut self, v: poodle_specs::Orientation) -> Self {
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
    pub fn value_text(mut self, v: impl Into<String>) -> Self {
        self.spec.value_text = Some(v.into());
        self
    }
    pub fn size(mut self, v: poodle_specs::ControlSize) -> Self {
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

impl IntoElement for Slider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let accent = resolve_color(theme, spec.range_fill_token());
        let border = resolve_color(theme, "color.border.default");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let elevated_bg = resolve_color(theme, "color.background.elevated");

        // Track height: Svelte uses a fixed 0.375rem (6 px) regardless of size.
        // No dedicated slider-track-height token exists in the design system yet;
        // the fixed value is retained to match the Svelte reference exactly.
        let track_f: f32 = rem_to_px(0.375); // 6 px
        let track_height = px(track_f);
        let track_radius = px(999.0); // Svelte: border-radius: 999px (full pill)

        let thumb_f = theme.resolve_space("size.icon.md");
        let thumb_size = px(thumb_f);
        let thumb_radius = px(thumb_f / 2.0);

        let progress = spec.normalized_progress().clamp(0.0, 1.0) as f32;

        // Track layout bounds (window coordinates) — updated each frame for hit math.
        let track_bounds_store: Arc<Mutex<Option<Bounds<Pixels>>>> = Arc::new(Mutex::new(None));
        let track_bounds_for_prepaint = track_bounds_store.clone();

        // Track with filled portion and thumb
        // Svelte: background = color-mix(surface 88%, transparent); no border
        let track_bg = Hsla { a: surface_bg.a * 0.88, ..surface_bg };
        let track = div()
            .w_full()
            .h(track_height)
            .rounded(track_radius)
            .bg(track_bg)
            .relative()
            .child(
                div()
                    .h_full()
                    .rounded(track_radius)
                    .bg(accent)
                    .w(relative(progress)),
            )
            // Thumb
            .child(
                div()
                    .absolute()
                    .top(px(-(thumb_f - track_f) / 2.0))
                    .left(relative(progress))
                    .ml(px(-(thumb_f / 2.0)))
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    // Svelte: thumb bg = elevated, border = border-default
                    .bg(elevated_bg)
                    .border_1()
                    .border_color(border)
                    // Svelte: 0 0.125rem 0.5rem shadow
                    .shadow(vec![gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.18),
                        offset: point(px(0.0), px(2.0)),
                        blur_radius: px(8.0),
                        spread_radius: px(0.0),
                    }]),
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
            .child(track);

        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let slider_id: SharedString = self.id.unwrap_or_else(|| {
            SharedString::from(format!(
                "poodle-slider-{}",
                SLIDER_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ))
        });

        let on_change = self.on_change;
        let on_value_commit = self.on_value_commit;
        let min = spec.min;
        let max = spec.max;
        let step = spec.step;
        let is_disabled = spec.is_disabled;

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
            let compute_value = move |pos_x: Pixels, track: &Bounds<Pixels>| -> Option<f64> {
                let local = pos_x - track.origin.x;
                let ratio_f32 = local / track.size.width;
                let ratio = (ratio_f32 as f64).clamp(0.0, 1.0);
                let raw = min + ratio * (max - min);
                let stepped = if step > 0.0 {
                    (raw / step).round() * step
                } else {
                    raw
                };
                Some(stepped.clamp(min, max))
            };

            if let Some(on_change) = on_change {
                let on_change = std::rc::Rc::new(on_change);
                let on_change_drag = on_change.clone();
                let compute_drag = compute_value;
                let tbs_down = track_bounds_store.clone();
                let tbs_move = track_bounds_store.clone();

                wrapper = wrapper
                    .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                        let tb = tbs_down.lock().ok().and_then(|g| *g);
                        if let Some(track) = tb {
                            if let Some(val) = compute_value(event.position.x, &track) {
                                on_change(&val, window, cx);
                            }
                        }
                    })
                    .on_mouse_move(move |event, window, cx| {
                        if event.pressed_button == Some(MouseButton::Left) {
                            let tb = tbs_move.lock().ok().and_then(|g| *g);
                            if let Some(track) = tb {
                                if let Some(val) = compute_drag(event.position.x, &track) {
                                    on_change_drag(&val, window, cx);
                                }
                            }
                        }
                    });
            }

            // on_value_commit: fires on click-release via on_click.
            // Full drag-release support requires on_mouse_up (GPUI 0.2.2 delta).
            if let Some(commit_handler) = on_value_commit {
                let current_val = spec.clamped_value();
                let tbs_click = track_bounds_store;
                let compute_commit = compute_value;
                wrapper = wrapper.on_click(move |event, window, cx| {
                    let tb = tbs_click.lock().ok().and_then(|g| *g);
                    let val = tb
                        .and_then(|track| compute_commit(event.position().x, &track))
                        .unwrap_or(current_val);
                    commit_handler(&val, window, cx);
                });
            }
        }

        wrapper.into_any_element()
    }
}
