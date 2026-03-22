//! Rating — real GPUI component backed by RatingSpec.

use std::rc::Rc;

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec, RatingSpec};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI rating component backed by `RatingSpec`.
pub struct Rating {
    spec: RatingSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Box<dyn Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static>>,
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

    pub fn on_change(
        mut self,
        handler: impl Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static,
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

        let active_color = resolve_color(theme, spec.active_color_token());
        let inactive_color = resolve_color(theme, spec.inactive_color_token());
        let filled = spec.filled_count();
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let is_interactive = !spec.is_readonly && !spec.is_disabled;

        // Wrap the callback in Rc so it can be shared across per-star click handlers.
        let handler: Option<Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>> =
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
                let star_id = SharedString::from(format!("pug-rating-star-{}", i));
                let hover_color = active_color;

                let mut star_wrapper = div()
                    .id(star_id)
                    .cursor_pointer()
                    .child(icon)
                    .hover(move |s| s.text_color(hover_color));

                if let Some(ref cb) = handler {
                    let cb = cb.clone();
                    let star_value = (i + 1) as usize; // 1-based rating value
                    star_wrapper = star_wrapper
                        .on_click(move |event, window, cx| cb(star_value, event, window, cx));
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
