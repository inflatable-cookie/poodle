//! NumberEntry — real GPUI component backed by NumberEntrySpec.
//!
//! Contract: grid layout with input field + vertical steppers.
//! Focus ring via border-color on focus. No hover on root.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{NumberEntrySpec, ValidationState};

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI numeric input component with +/- stepper buttons backed by `NumberEntrySpec`.
pub struct NumberEntry {
    spec: NumberEntrySpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_increment: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_decrement: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for NumberEntry {
    type Target = NumberEntrySpec;
    fn deref(&self) -> &NumberEntrySpec { &self.spec }
}

impl NumberEntry {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: NumberEntrySpec::default(), theme: theme.clone(), id_suffix: None, on_increment: None, on_decrement: None }
    }

    pub fn from_spec(spec: NumberEntrySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_increment: None,
            on_decrement: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: f64) -> Self { self.spec.value = v; self }
    pub fn min(mut self, v: f64) -> Self { self.spec.min = v; self }
    pub fn max(mut self, v: f64) -> Self { self.spec.max = v; self }
    pub fn step(mut self, v: f64) -> Self { self.spec.step = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn validation_state(mut self, v: ValidationState) -> Self { self.spec.validation_state = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_increment(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_increment = Some(Box::new(handler));
        self
    }

    pub fn on_decrement(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_decrement = Some(Box::new(handler));
        self
    }
}

impl IntoElement for NumberEntry {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let control_padding_x = resolve_px(theme, "semantic.space.control.x");
        let control_radius = resolve_radius(theme, spec.radius_token());

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, spec.fill_token());
        let text_primary = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated = resolve_color(theme, spec.stepper_fill_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let stepper_bg = color_mix(elevated, surface_bg, 0.88);
        let display_value = format!("{}", spec.clamped_value());

        // Contract: stepper width 1.25rem, radius = control - 0.125rem
        let stepper_width = px(20.0); // 1.25rem
        let stepper_inner_radius = resolve_radius(theme, spec.radius_token()) - px(2.0);

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-number-entry-{}", suffix)
        } else {
            "pug-number-entry".to_string()
        };

        // Wrap callbacks in Rc for sharing across stepper clicks + keyboard handler
        let on_inc_rc: Option<std::rc::Rc<dyn Fn(&mut Window, &mut App)>> =
            self.on_increment.map(|h| std::rc::Rc::from(h));
        let on_dec_rc: Option<std::rc::Rc<dyn Fn(&mut Window, &mut App)>> =
            self.on_decrement.map(|h| std::rc::Rc::from(h));

        // Increment button (top)
        let mut inc_btn = div()
            .id("pug-number-entry-inc")
            .w(stepper_width)
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .rounded(stepper_inner_radius)
            .bg(stepper_bg)
            .text_size(px(12.0))
            .text_color(text_secondary)
            .cursor_pointer()
            .child("+");

        if !spec.is_disabled {
            if let Some(ref handler) = on_inc_rc {
                let handler = handler.clone();
                inc_btn = inc_btn.on_click(move |_event, window, cx| {
                    handler(window, cx);
                });
            }
        }

        // Decrement button (bottom)
        let mut dec_btn = div()
            .id("pug-number-entry-dec")
            .w(stepper_width)
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .rounded(stepper_inner_radius)
            .bg(stepper_bg)
            .text_size(px(12.0))
            .text_color(text_secondary)
            .cursor_pointer()
            .child("\u{2212}");

        if !spec.is_disabled {
            if let Some(ref handler) = on_dec_rc {
                let handler = handler.clone();
                dec_btn = dec_btn.on_click(move |_event, window, cx| {
                    handler(window, cx);
                });
            }
        }

        // Vertical stepper column
        let steppers = div()
            .flex()
            .flex_col()
            .gap(px(1.0))
            .h_full()
            .py(px(2.0))
            .pr(px(2.0))
            .child(inc_btn)
            .child(dec_btn);

        // Value display — centered in the input area
        let value_display = div()
            .flex_1()
            .px(control_padding_x)
            .flex()
            .items_center()
            .text_size(px(14.0))
            .text_color(text_primary)
            .child(display_value);

        // Root: grid-like layout with input on left, steppers on right
        let mut wrapper = div()
            .id(SharedString::from(id_str))
            .focusable()
            .min_h(control_height)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .overflow_hidden()
            // Contract: focus-within = border switches to focus ring color
            .focus(move |s| s.border_color(focus_ring))
            .child(value_display)
            .child(steppers);

        // Contract: ArrowUp increments, ArrowDown decrements
        if !spec.is_disabled {
            let key_inc = on_inc_rc.clone();
            let key_dec = on_dec_rc.clone();
            wrapper = wrapper.on_key_down(move |event: &KeyDownEvent, window, cx| {
                match event.keystroke.key.as_str() {
                    "up" => {
                        if let Some(ref handler) = key_inc {
                            handler(window, cx);
                        }
                    }
                    "down" => {
                        if let Some(ref handler) = key_dec {
                            handler(window, cx);
                        }
                    }
                    _ => {}
                }
            });
        }

        if spec.is_disabled {
            wrapper = wrapper
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        wrapper.into_any_element()
    }
}
