//! Box — real GPUI component backed by BoxSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{BoxSpec, Dimension, Overflow, PaddingScale};

use crate::theme_ext::resolve_px;

/// A layout box with padding and overflow control.
pub struct Box {
    spec: BoxSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for Box {
    type Target = BoxSpec;
    fn deref(&self) -> &BoxSpec { &self.spec }
}

impl Box {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: BoxSpec::new(), theme: theme.clone(), children: Vec::new() }
    }

    pub fn from_spec(spec: BoxSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn padding(mut self, v: PaddingScale) -> Self { self.spec.padding = v; self }
    pub fn width(mut self, v: Dimension) -> Self { self.spec.width = Some(v); self }
    pub fn height(mut self, v: Dimension) -> Self { self.spec.height = Some(v); self }
    pub fn min_width(mut self, v: Dimension) -> Self { self.spec.min_width = Some(v); self }
    pub fn min_height(mut self, v: Dimension) -> Self { self.spec.min_height = Some(v); self }
    pub fn overflow(mut self, v: Overflow) -> Self { self.spec.overflow = v; self }
    pub fn role(mut self, v: impl Into<String>) -> Self { self.spec.role = Some(v.into()); self }


    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

/// Try to parse a Dimension string as px (e.g. "100px" -> Some(100.0), "50%" -> None).
fn parse_dimension_px(dim: &Dimension) -> Option<f32> {
    let s = dim.as_str().trim();
    if let Some(stripped) = s.strip_suffix("px") {
        stripped.trim().parse::<f32>().ok()
    } else {
        // Try plain number as px
        s.parse::<f32>().ok()
    }
}

impl IntoElement for Box {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let padding = spec.resolved_padding();

        let mut el = div();

        // Width / Height / MinWidth / MinHeight
        if let Some(ref w) = spec.width {
            if let Some(v) = parse_dimension_px(w) {
                el = el.w(px(v));
            } else if w.as_str() == "100%" {
                el = el.w_full();
            }
        }
        if let Some(ref h) = spec.height {
            if let Some(v) = parse_dimension_px(h) {
                el = el.h(px(v));
            } else if h.as_str() == "100%" {
                el = el.h_full();
            }
        }
        if let Some(ref mw) = spec.min_width {
            if let Some(v) = parse_dimension_px(mw) {
                el = el.min_w(px(v));
            }
        }
        if let Some(ref mh) = spec.min_height {
            if let Some(v) = parse_dimension_px(mh) {
                el = el.min_h(px(v));
            }
        }

        // Padding
        if let Some(h) = padding.horizontal {
            el = el.px(resolve_px(theme, h));
        }
        if let Some(v) = padding.vertical {
            el = el.py(resolve_px(theme, v));
        }

        // Overflow — Auto/Scroll require id() for scroll support
        match spec.overflow {
            Overflow::Hidden | Overflow::Clip => {
                el = el.overflow_hidden();
            }
            Overflow::Auto | Overflow::Scroll => {
                // Children
                for child in self.children {
                    el = el.child(child);
                }
                return el.id("poodle-box").overflow_y_scroll().into_any_element();
            }
            Overflow::Visible => {}
        }

        // Children
        for child in self.children {
            el = el.child(child);
        }

        el.into_any_element()
    }
}
