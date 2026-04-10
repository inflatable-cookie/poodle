//! Rating — real GPUI component backed by RatingSpec.

use std::rc::Rc;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, IconSize, IconSpec, RatingSpec, SemanticControlSizeRole};

use super::icon::Icon;
use crate::presentation::{rem_to_px, resolve_semantic_size, control_height_rem};
use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI rating component backed by `RatingSpec`.
pub struct Rating {
    spec: RatingSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Box<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Rating {
    type Target = RatingSpec;
    fn deref(&self) -> &RatingSpec { &self.spec }
}

impl Rating {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: RatingSpec::new(), theme: theme.clone(), on_change: None }
    }

    pub fn from_spec(spec: RatingSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self { self.spec.value = v; self }
    pub fn readonly(mut self, v: bool) -> Self { self.spec.is_readonly = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn precision(mut self, v: f64) -> Self { self.spec.precision = v; self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }

    pub fn on_change(
        mut self,
        handler: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Rating {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let star_touch_size = px(rem_to_px(control_height_rem(effective_size)));

        let active_color = resolve_color(theme, spec.active_color_token());
        let inactive_color = resolve_color(theme, spec.inactive_color_token());
        let filled = spec.filled_count();
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");

        let is_interactive = !spec.is_readonly && !spec.is_disabled;

        // Wrap the callback in Rc so it can be shared across per-star click handlers.
        let handler: Option<Rc<dyn Fn(&usize, &mut Window, &mut App)>> =
            self.on_change.map(|h| Rc::from(h));

        // Contract: gap 0.125rem (2px)
        let mut el = div().flex().items_center().gap(px(2.0));

        for i in 0..spec.max {
            let color = if i < filled {
                active_color
            } else {
                inactive_color
            };

            let icon = Icon::from_spec(IconSpec::new("star").with_size(IconSize::Sm), theme)
                .with_color(color);

            if is_interactive {
                let star_id = SharedString::from(format!("poodle-rating-star-{}", i));
                let hover_color = active_color;

                // Contract: touch target per effective size
                let mut star_wrapper = div()
                    .id(star_id)
                    .focusable()
                    .cursor_pointer()
                    .min_w(star_touch_size)
                    .min_h(star_touch_size)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon)
                    .hover(move |s| s.text_color(hover_color));

                if let Some(ref cb) = handler {
                    let cb_click = cb.clone();
                    let star_value = (i + 1) as usize; // 1-based rating value
                    star_wrapper = star_wrapper
                        .on_click(move |_event, window, cx| cb_click(&star_value, window, cx));

                    // Arrow key navigation: Left decreases, Right increases
                    let cb_key = cb.clone();
                    let current_i = i;
                    let max = spec.max;
                    star_wrapper = star_wrapper
                        .on_key_down(move |event: &KeyDownEvent, window, cx| {
                            let new_val = if event.keystroke.key == "right" || event.keystroke.key == "up" {
                                Some(((current_i + 1) as usize + 1).min(max as usize))
                            } else if event.keystroke.key == "left" || event.keystroke.key == "down" {
                                Some((current_i as usize).max(1))
                            } else {
                                None
                            };
                            if let Some(v) = new_val {
                                cb_key(&v, window, cx);
                            }
                        });
                }

                el = el.child(star_wrapper);
            } else {
                el = el.child(icon);
            }
        }

        if spec.is_disabled {
            el = el
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        el.into_any_element()
    }
}
