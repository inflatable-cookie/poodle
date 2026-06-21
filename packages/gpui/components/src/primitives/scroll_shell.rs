//! ScrollShell — real GPUI component backed by ScrollShellSpec.
//!
//! Contract: `docs/contracts/components/scroll-shell.md`
//!
//! Three-layer anatomy per contract §2:
//!   Root (.scroll-shell)       → clip boundary: overflow hidden, radius-surface
//!   Viewport (.scroll-shell__viewport) → scroll owner: per-axis overflow, padding, focus ring
//!   Content (.scroll-shell__content)   → sizing wrapper: horizontal `min-width: max-content`
//!
//! gpui handles scrolling natively (mouse wheel / trackpad) once an axis has
//! `overflow_*_scroll`. Keyboard scrolling (contract §6/§10) requires a
//! persisted `ScrollHandle` owned by a stateful host; the stateless
//! `IntoElement` shell can't retain scroll offset across frames, so that
//! behavior is deferred to a stateful host (noted in the parity doc).

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{Direction, PaddingScale, ScrollShellSpec, SurfaceRole};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A scrollable container with directional overflow.
pub struct ScrollShell {
    spec: ScrollShellSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for ScrollShell {
    type Target = ScrollShellSpec;
    fn deref(&self) -> &ScrollShellSpec {
        &self.spec
    }
}

impl ScrollShell {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ScrollShellSpec::new(),
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn from_spec(spec: ScrollShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn direction(mut self, v: Direction) -> Self {
        self.spec.direction = v;
        self
    }
    pub fn padding(mut self, v: PaddingScale) -> Self {
        self.spec.padding = v;
        self
    }
    pub fn role(mut self, v: SurfaceRole) -> Self {
        self.spec.role = Some(v);
        self
    }
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = Some(v.into());
        self
    }
    pub fn focusable(mut self, v: bool) -> Self {
        self.spec.is_focusable = v;
        self
    }

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
        let needs_horizontal =
            matches!(spec.direction, Direction::Horizontal | Direction::Both);

        let surface_radius = resolve_radius(theme, "radius.surface");
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        // ── Content (.scroll-shell__content) — sizing wrapper ──
        // For horizontal/both the content must not collapse: a non-shrinking
        // row sized to its children is gpui's `min-width: max-content` analogue
        // (contract §8 Content). Vertical content stacks and fills the width.
        let mut content = div().flex();
        content = if needs_horizontal {
            content.flex_row().flex_shrink_0()
        } else {
            content.flex_col().w_full()
        };
        for child in self.children {
            content = content.child(child);
        }

        // ── Viewport (.scroll-shell__viewport) — scroll owner ──
        let id_str = spec.label.as_deref().unwrap_or("scroll-shell");
        let mut viewport = div()
            .id(SharedString::from(format!("poodle-{}", id_str)))
            .size_full()
            .rounded(surface_radius);

        // Per-axis overflow + the flex axis the scroll runs along (contract §8).
        match spec.direction {
            Direction::Vertical => {
                viewport = viewport.flex().flex_col().min_h_0().overflow_y_scroll();
            }
            Direction::Horizontal => {
                viewport = viewport.flex().flex_row().min_w_0().overflow_x_scroll();
            }
            Direction::Both => {
                viewport = viewport
                    .flex()
                    .flex_col()
                    .min_h_0()
                    .min_w_0()
                    .overflow_scroll();
            }
        }

        // Focus ring for focusable viewports (contract §8 focus styles).
        if spec.is_focusable {
            viewport = viewport.focusable().focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });
        }

        // Interior padding on the viewport (contract §8 padding scale).
        if let Some(h) = padding.horizontal {
            viewport = viewport.px(resolve_px(theme, h));
        }
        if let Some(v) = padding.vertical {
            viewport = viewport.py(resolve_px(theme, v));
        }

        viewport = viewport.child(content);

        // ── Root (.scroll-shell) — clip boundary ──
        div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .overflow_hidden()
            .rounded(surface_radius)
            .child(viewport)
            .into_any_element()
    }
}
