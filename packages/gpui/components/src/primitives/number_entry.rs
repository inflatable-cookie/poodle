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
}

impl std::ops::Deref for NumberEntry {
    type Target = NumberEntrySpec;
    fn deref(&self) -> &NumberEntrySpec { &self.spec }
}

impl NumberEntry {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: NumberEntrySpec::default(), theme: theme.clone(), id_suffix: None }
    }

    pub fn from_spec(spec: NumberEntrySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
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

        // Increment button (top)
        let inc_btn = div()
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

        // Decrement button (bottom)
        let dec_btn = div()
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

        if spec.is_disabled {
            wrapper = wrapper
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        wrapper.into_any_element()
    }
}
