//! NumberInput — real GPUI component backed by NumberInputSpec.
//!
//! Contract: grid layout with input field + vertical steppers.
//! Focus ring via border-color on focus. No hover on root.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlSize, IconSize, IconSpec, NumberInputSpec, ValidationState};

use super::icon::Icon;
use crate::presentation::{rem_to_px, resolve_semantic_size, control_height_rem, size_font_rem, size_padding_x_offset_rem};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI numeric input component with +/- stepper buttons backed by `NumberInputSpec`.
pub struct NumberInput {
    spec: NumberInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_increment: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_decrement: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_change: Option<Box<dyn Fn(&f64, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for NumberInput {
    type Target = NumberInputSpec;
    fn deref(&self) -> &NumberInputSpec { &self.spec }
}

impl NumberInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: NumberInputSpec::default(), theme: theme.clone(), id_suffix: None, on_increment: None, on_decrement: None, on_change: None }
    }

    pub fn from_spec(spec: NumberInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_increment: None,
            on_decrement: None,
            on_change: None,
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
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn size_role(mut self, v: poodle_primitives::SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn density(mut self, v: poodle_primitives::ControlDensity) -> Self { self.spec.density = v; self }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_increment(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_increment = Some(Box::new(handler));
        self
    }

    pub fn on_decrement(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_decrement = Some(Box::new(handler));
        self
    }

    /// Called when the value changes (from stepper or direct editing).
    pub fn on_change(
        mut self,
        handler: impl Fn(&f64, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for NumberInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let control_height = px(rem_to_px(control_height_rem(effective_size)));
        let base_padding_x = resolve_px(theme, spec.horizontal_padding_token());
        let control_padding_x = base_padding_x + px(rem_to_px(size_padding_x_offset_rem(effective_size)));
        let control_radius = resolve_radius(theme, spec.radius_token());
        let body_size_f = rem_to_px(size_font_rem(effective_size));
        let body_size = px(body_size_f);
        let body_line_height = px(body_size_f * 1.4);

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, spec.fill_token());
        let text_primary = resolve_color(theme, spec.text_color_token());
        let elevated = resolve_color(theme, spec.stepper_fill_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let stepper_bg = color_mix(elevated, surface_bg, 0.88);
        let display_value = format!("{}", spec.clamped_value());

        // Contract: stepper width 1.25rem (20px), radius = control - 0.125rem (2px)
        let stepper_width = px(20.0);
        let stepper_inner_radius = control_radius - px(2.0);

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-number-input-{}", suffix)
        } else {
            "poodle-number-input".to_string()
        };

        // Wrap callbacks in Rc for sharing across stepper clicks + keyboard handler
        let on_inc_rc: Option<std::rc::Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>> =
            self.on_increment.map(|h| std::rc::Rc::from(h));
        let on_dec_rc: Option<std::rc::Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>> =
            self.on_decrement.map(|h| std::rc::Rc::from(h));

        // Increment button (top) — Svelte uses Icon component
        let mut inc_btn = div()
            .id("poodle-number-input-inc")
            .w(stepper_width)
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .rounded(stepper_inner_radius)
            .bg(stepper_bg)
            .cursor_pointer()
            .hover(move |s| s.bg(elevated))
            .child(
                Icon::from_spec(
                    IconSpec::new("chevron-up").with_size(IconSize::Sm),
                    theme,
                )
                .with_color(text_primary),
            );

        if !spec.is_disabled {
            if let Some(ref handler) = on_inc_rc {
                let handler = handler.clone();
                inc_btn = inc_btn.on_click(move |event, window, cx| {
                    handler(event, window, cx);
                });
            }
        }

        // Decrement button (bottom) — Svelte uses Icon component
        let mut dec_btn = div()
            .id("poodle-number-input-dec")
            .w(stepper_width)
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .rounded(stepper_inner_radius)
            .bg(stepper_bg)
            .cursor_pointer()
            .hover(move |s| s.bg(elevated))
            .child(
                Icon::from_spec(
                    IconSpec::new("chevron-down").with_size(IconSize::Sm),
                    theme,
                )
                .with_color(text_primary),
            );

        if !spec.is_disabled {
            if let Some(ref handler) = on_dec_rc {
                let handler = handler.clone();
                dec_btn = dec_btn.on_click(move |event, window, cx| {
                    handler(event, window, cx);
                });
            }
        }

        // Vertical stepper column — contract: gap 0, padding 0.0625rem (1px)
        let steppers = div()
            .flex()
            .flex_col()
            .h_full()
            .p(px(1.0))
            .child(inc_btn)
            .child(dec_btn);

        // Value display — centered in the input area
        let value_display = div()
            .flex_1()
            .px(control_padding_x)
            .flex()
            .items_center()
            .text_size(body_size)
            .line_height(body_line_height)
            .text_color(text_primary)
            .child(display_value);

        // Svelte: validation state border colors
        let effective_border = match spec.validation_state {
            ValidationState::Invalid => resolve_color(theme, "semantic.color.status.danger"),
            ValidationState::Valid => resolve_color(theme, "semantic.color.status.success"),
            ValidationState::Pending => resolve_color(theme, "semantic.color.accent.base"),
            _ => border,
        };

        // Root: grid-like layout with input on left, steppers on right
        let mut wrapper = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w_full()
            .min_h(control_height)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(effective_border)
            .flex()
            .items_center()
            .overflow_hidden()
            // Svelte: focus-within = border + shadow ring
            .focus(move |s| s
                .border_color(focus_ring)
                .shadow(vec![gpui::BoxShadow {
                    color: Hsla { a: focus_ring.a * 0.28, ..focus_ring },
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(2.0),
                }])
            )
            .child(value_display)
            .child(steppers);

        // Contract: ArrowUp increments, ArrowDown decrements
        if !spec.is_disabled {
            let key_inc = on_inc_rc.clone();
            let key_dec = on_dec_rc.clone();
            wrapper = wrapper.on_key_down(move |event: &KeyDownEvent, window, cx| {
                let click = ClickEvent::default();
                match event.keystroke.key.as_str() {
                    "up" => {
                        if let Some(ref handler) = key_inc {
                            handler(&click, window, cx);
                        }
                    }
                    "down" => {
                        if let Some(ref handler) = key_dec {
                            handler(&click, window, cx);
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
