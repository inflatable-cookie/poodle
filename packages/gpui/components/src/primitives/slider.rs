//! Slider — real GPUI component backed by SliderSpec.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, Orientation, SliderSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

static SLIDER_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// A real GPUI slider component backed by `SliderSpec`.
pub struct Slider {
    spec: SliderSpec,
    theme: GpuiThemeProvider,
    id: Option<SharedString>,
    on_change: Option<Box<dyn Fn(&f64, &mut Window, &mut App) + 'static>>,
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
        }
    }

    pub fn from_spec(spec: SliderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id: None,
            on_change: None,
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
    pub fn value_text(mut self, v: impl Into<String>) -> Self {
        self.spec.value_text = Some(v.into());
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

impl IntoElement for Slider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let accent = resolve_color(theme, spec.range_fill_token());
        let border = resolve_color(theme, "color.border.default");
        let surface_bg = resolve_color(theme, "color.background.surface");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let stack_gap = resolve_px(theme, "space.stack.sm");

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let elevated_bg = resolve_color(theme, "color.background.elevated");
        // Track/thumb scale with effective size
        let track_f: f32 = match effective_size {
            ControlSize::Xs => 4.0,
            ControlSize::Sm => 5.0,
            ControlSize::Md => 6.0,
            ControlSize::Lg => 7.0,
            ControlSize::Xl => 8.0,
        };
        let track_height = px(track_f);
        let track_radius = px(track_f / 2.0);
        let thumb_f = theme.resolve_space("size.icon.md");
        let thumb_size = px(thumb_f);
        let thumb_radius = px(thumb_f / 2.0);
        let label_font_size = px(rem_to_px(size_font_rem(effective_size)));

        let progress = spec.normalized_progress().clamp(0.0, 1.0) as f32;

        // Track with filled portion and thumb
        let track = div()
            .w_full()
            .h(track_height)
            .rounded(track_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border.opacity(0.3))
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

        // Value labels
        let labels = div()
            .flex()
            .items_center()
            .justify_between()
            .text_size(label_font_size)
            .text_color(text_secondary)
            .child(format!("{:.0}", spec.min))
            .child(format!("{:.0}", spec.clamped_value()))
            .child(format!("{:.0}", spec.max));

        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let slider_id: SharedString = self.id.unwrap_or_else(|| {
            SharedString::from(format!(
                "poodle-slider-{}",
                SLIDER_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ))
        });

        let on_change = self.on_change;
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
            .gap(stack_gap)
            .cursor(if is_disabled {
                CursorStyle::OperationNotAllowed
            } else {
                CursorStyle::PointingHand
            })
            .child(track)
            .child(labels);

        wrapper = wrapper.focus(move |s| {
            s.border_color(focus_ring)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
        });

        if is_disabled {
            wrapper = wrapper.opacity(disabled_opacity);
        } else if let Some(on_change) = on_change {
            // Compute value from click position using Pixels arithmetic
            let compute_value =
                move |pos_x: Pixels, origin_x: Pixels, width: Pixels| -> Option<f64> {
                    let local = pos_x - origin_x;
                    // Use px division: local / width gives a ratio
                    // Pixels supports Div<Pixels> -> f32 via: local / width
                    let ratio_f32 = local / width;
                    let ratio = (ratio_f32 as f64).clamp(0.0, 1.0);
                    let raw = min + ratio * (max - min);
                    let stepped = if step > 0.0 {
                        (raw / step).round() * step
                    } else {
                        raw
                    };
                    Some(stepped.clamp(min, max))
                };

            let on_change = std::rc::Rc::new(on_change);
            let on_change_drag = on_change.clone();
            let compute_drag = compute_value;

            wrapper = wrapper
                .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                    let bounds = window.bounds();
                    if let Some(val) =
                        compute_value(event.position.x, bounds.origin.x, bounds.size.width)
                    {
                        on_change(&val, window, cx);
                    }
                })
                .on_mouse_move(move |event, window, cx| {
                    if event.pressed_button == Some(MouseButton::Left) {
                        let bounds = window.bounds();
                        if let Some(val) =
                            compute_drag(event.position.x, bounds.origin.x, bounds.size.width)
                        {
                            on_change_drag(&val, window, cx);
                        }
                    }
                });
        }

        wrapper.into_any_element()
    }
}
