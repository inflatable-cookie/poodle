//! ScrollShell — real GPUI component backed by ScrollShellSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{Direction, PaddingScale, ScrollShellSpec, SurfaceRole};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A scrollable container with directional overflow.
///
/// Note: gpui handles scrolling differently from web — this component
/// applies overflow_hidden and padding from the spec, with direction-
/// appropriate flex layout. For true scrolling, use gpui's built-in
/// scroll view primitives.
pub struct ScrollShell {
    spec: ScrollShellSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for ScrollShell {
    type Target = ScrollShellSpec;
    fn deref(&self) -> &ScrollShellSpec { &self.spec }
}

impl ScrollShell {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ScrollShellSpec::new(), theme: theme.clone(), children: Vec::new() }
    }

    pub fn from_spec(spec: ScrollShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn direction(mut self, v: Direction) -> Self { self.spec.direction = v; self }
    pub fn padding(mut self, v: PaddingScale) -> Self { self.spec.padding = v; self }
    pub fn role(mut self, v: SurfaceRole) -> Self { self.spec.role = Some(v); self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }
    pub fn focusable(mut self, v: bool) -> Self { self.spec.is_focusable = v; self }


    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for ScrollShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let padding = spec.resolved_padding();

        let surface_radius = resolve_radius(theme, "semantic.radius.surface");
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let mut el = div()
            .overflow_hidden()
            .rounded(surface_radius);

        // Direction-based flex layout
        match spec.direction {
            Direction::Vertical => {
                el = el.flex().flex_col();
            }
            Direction::Horizontal => {
                el = el.flex().flex_row();
            }
            Direction::Both => {
                // Both directions: use flex-wrap column
                el = el.flex().flex_col();
            }
        }

        // Focus ring for focusable shells
        if spec.is_focusable {
            el = el.focus(move |s| s.border_color(focus_ring));
        }

        // Padding
        if let Some(h) = padding.horizontal {
            el = el.px(resolve_px(theme, h));
        }
        if let Some(v) = padding.vertical {
            el = el.py(resolve_px(theme, v));
        }

        for child in self.children {
            el = el.child(child);
        }

        el.into_any_element()
    }
}
