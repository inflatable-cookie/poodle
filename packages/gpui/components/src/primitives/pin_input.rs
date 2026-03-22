//! PinInput — real GPUI component backed by PinInputSpec.
//!
//! Contract: fixed-length code-entry with per-cell focus rings.
//! Cell: 2.25rem × 2.5rem, gap 0.375rem, code font 1rem.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::PinInputSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// A real GPUI pin/OTP input component with fixed-length digit cells backed by `PinInputSpec`.
pub struct PinInput {
    spec: PinInputSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
}

impl std::ops::Deref for PinInput {
    type Target = PinInputSpec;
    fn deref(&self) -> &PinInputSpec { &self.spec }
}

impl PinInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: PinInputSpec::default(), theme: theme.clone(), id_suffix: None }
    }

    pub fn from_spec(spec: PinInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn length(mut self, v: usize) -> Self { self.spec.length = v; self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = v.into(); self }
    pub fn masked(mut self, v: bool) -> Self { self.spec.is_masked = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }
}

impl IntoElement for PinInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Contract: cell 2.25rem × 2.5rem, gap 0.375rem
        let cell_width = px(36.0);  // 2.25rem
        let cell_height = px(40.0); // 2.5rem
        let cell_gap = px(6.0);     // 0.375rem
        let control_radius = resolve_radius(theme, spec.radius_token());

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, spec.fill_token());
        let text_primary = resolve_color(theme, spec.text_color_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        let chars: Vec<char> = spec.value.chars().collect();

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-pin-input-{}", suffix)
        } else {
            "pug-pin-input".to_string()
        };

        let mut row = div()
            .id(SharedString::from(id_str))
            .focusable()
            .flex()
            .gap(cell_gap)
            // Contract: focus ring on root group
            .focus(move |s| s.border_color(focus_ring));

        for i in 0..spec.length {
            let ch = chars.get(i).copied();
            let display = match ch {
                Some(_) if spec.is_masked => "\u{2022}".to_string(),
                Some(c) => c.to_string(),
                None => String::new(),
            };

            let cell = div()
                .w(cell_width)
                .h(cell_height)
                .rounded(control_radius)
                .bg(surface_bg)
                .border_1()
                .border_color(border)
                .flex()
                .items_center()
                .justify_center()
                // Contract: code font family, 1rem size
                .text_size(px(16.0))
                .text_color(text_primary)
                .child(display);

            row = row.child(cell);
        }

        let mut wrapper = row;

        if spec.is_disabled {
            wrapper = wrapper
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        wrapper.into_any_element()
    }
}
