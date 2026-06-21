//! Rating — real GPUI component backed by RatingSpec.

use std::rc::Rc;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, IconSize, IconSpec, RatingSpec, SemanticControlSizeRole,
};

use super::icon::Icon;
use crate::presentation::{control_height_rem, rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI rating component backed by `RatingSpec`.
pub struct Rating {
    spec: RatingSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Box<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Rating {
    type Target = RatingSpec;
    fn deref(&self) -> &RatingSpec {
        &self.spec
    }
}

impl Rating {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: RatingSpec::new(),
            theme: theme.clone(),
            on_change: None,
        }
    }

    pub fn from_spec(spec: RatingSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self {
        self.spec.value = v;
        self
    }
    pub fn readonly(mut self, v: bool) -> Self {
        self.spec.is_readonly = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn precision(mut self, v: f64) -> Self {
        self.spec.precision = v;
        self
    }
    pub fn step(mut self, v: f64) -> Self {
        self.spec.step = v;
        self
    }
    pub fn allow_clear(mut self, v: bool) -> Self {
        self.spec.allow_clear = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

/// Per-size glyph font-size in rem (contract §8 size table).
fn glyph_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Per-density inter-item gap in rem (contract §8: base 0.125, compact 0.0625,
/// comfortable 0.25).
fn item_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.0625,
        ControlDensity::Default => 0.125,
        ControlDensity::Comfortable => 0.25,
    }
}

impl IntoElement for Rating {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        // Contract §7/§8: each item is a 2rem (size-scaled) square touch target.
        let star_touch_size = px(rem_to_px(control_height_rem(effective_size)));
        // Contract §8 glyph font-size table; Svelte sizes the SVG at 1.125em.
        let glyph_px_f = rem_to_px(glyph_font_rem(effective_size)) * 1.125;
        let glyph_px = px(glyph_px_f);

        let active_color = resolve_color(theme, spec.active_color_token());
        // Contract §8: unfilled color = color-mix(text-secondary 48%, transparent).
        let inactive_raw = resolve_color(theme, spec.inactive_color_token());
        let inactive_color = Hsla {
            a: inactive_raw.a * spec.inactive_color_alpha(),
            ..inactive_raw
        };
        let glow_color = resolve_color(theme, spec.hover_glow_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_token());
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

        let is_fractional = spec.is_fractional();
        let is_interactive = !spec.is_readonly && !spec.is_disabled;

        // Wrap the callback in Rc so it can be shared across per-star click handlers.
        let handler: Option<Rc<dyn Fn(&usize, &mut Window, &mut App)>> =
            self.on_change.map(|h| Rc::from(h));

        // Contract §8: gap is density-driven (base 0.125rem).
        let mut el = div()
            .flex()
            .items_center()
            .gap(px(rem_to_px(item_gap_rem(spec.density))));

        for i in 0..spec.max {
            // Contract §2/§4: clipped accent overlay sized by per-star fill ratio.
            let ratio = spec.fill_ratio(i) as f32;

            let base_icon =
                Icon::from_spec(IconSpec::new("star").with_size(IconSize::Sm), theme)
                    .with_px_size(glyph_px_f)
                    .with_color(inactive_color);
            let fill_icon =
                Icon::from_spec(IconSpec::new("star").with_size(IconSize::Sm), theme)
                    .with_px_size(glyph_px_f)
                    .with_color(active_color);

            // Glyph: relative base layer + absolute clipped fill overlay.
            let glyph = div()
                .relative()
                .flex()
                .items_center()
                .justify_center()
                .w(glyph_px)
                .h(glyph_px)
                .child(base_icon)
                .child(
                    // Fill layer: clipped to the per-star ratio (contract §8
                    // .rating__glyph-fill: inset 0 auto 0 0, overflow hidden).
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .h_full()
                        .w(relative(ratio.clamp(0.0, 1.0)))
                        .overflow_hidden()
                        .flex()
                        .items_center()
                        .child(fill_icon),
                );

            if is_interactive {
                let star_id = SharedString::from(format!("poodle-rating-star-{}", i));

                // Contract §7/§8: fixed square touch target per effective size.
                let mut star_wrapper = div()
                    .id(star_id)
                    .focusable()
                    .cursor_pointer()
                    .w(star_touch_size)
                    .h(star_touch_size)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(glyph)
                    // Contract §8 hover: accent drop-shadow glow on hovered item.
                    .hover(move |s| {
                        s.shadow(vec![gpui::BoxShadow {
                            color: Hsla {
                                a: glow_color.a * 0.52,
                                ..glow_color
                            },
                            offset: gpui::point(px(0.0), px(0.0)),
                            blur_radius: px(rem_to_px(0.375)),
                            spread_radius: px(0.0),
                        }])
                    })
                    // Contract §8: focus-visible accent focus ring.
                    .focus(move |s| {
                        s.border_2().border_color(focus_ring)
                    });

                if let Some(ref cb) = handler {
                    let cb_click = cb.clone();
                    // Fractional mode: clicking selects whole-star value; the
                    // sub-star pointer position (preview-loop concern) is not
                    // resolvable in a stateless render, so click selects i+1.
                    let star_value = (i + 1) as usize; // 1-based rating value
                    star_wrapper = star_wrapper
                        .on_click(move |_event, window, cx| cb_click(&star_value, window, cx));

                    // Whole-step roving + fractional step keyboard.
                    let cb_key = cb.clone();
                    let current_i = i;
                    let max = spec.max;
                    let allow_clear = spec.allow_clear;
                    star_wrapper =
                        star_wrapper.on_key_down(move |event: &KeyDownEvent, window, cx| {
                            let key = event.keystroke.key.as_str();
                            let new_val: Option<usize> = match key {
                                "right" | "up" => {
                                    Some(((current_i as usize) + 2).min(max as usize))
                                }
                                "left" | "down" => {
                                    let floor = if allow_clear { 0 } else { 1 };
                                    Some((current_i as usize).max(floor))
                                }
                                "home" => Some(if allow_clear { 0 } else { 1 }),
                                "end" => Some(max as usize),
                                "enter" | "space" => Some((current_i as usize) + 1),
                                _ => None,
                            };
                            if let Some(v) = new_val {
                                cb_key(&v, window, cx);
                            }
                        });
                }

                el = el.child(star_wrapper);
            } else {
                // Readonly/disabled: bare square holding the glyph.
                el = el.child(
                    div()
                        .w(star_touch_size)
                        .h(star_touch_size)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(glyph),
                );
            }
        }

        if spec.is_disabled {
            el = el
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        // Fractional mode maps to a stepped slider-like control (contract §10);
        // GPUI has no ARIA channel so semantics are structural only.
        let _ = is_fractional;

        el.into_any_element()
    }
}
