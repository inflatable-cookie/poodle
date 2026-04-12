//! Stack — real GPUI component backed by StackSpec.
//!
//! Contract: `docs/contracts/components/stack.md`

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{Alignment, LayoutJustify, PaddingScale, StackDirection, StackSpec};

use crate::theme_ext::resolve_px;

/// A flex layout container with direction, gap, alignment, and justification.
pub struct Stack {
    spec: StackSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for Stack {
    type Target = StackSpec;
    fn deref(&self) -> &StackSpec {
        &self.spec
    }
}

impl Stack {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: StackSpec::new(),
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn from_spec(spec: StackSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn direction(mut self, v: StackDirection) -> Self {
        self.spec.direction = v;
        self
    }
    pub fn gap(mut self, v: PaddingScale) -> Self {
        self.spec.gap = v;
        self
    }
    pub fn align(mut self, v: Alignment) -> Self {
        self.spec.align = v;
        self
    }
    pub fn justify(mut self, v: LayoutJustify) -> Self {
        self.spec.justify = Some(v);
        self
    }
    pub fn wrap(mut self, v: bool) -> Self {
        self.spec.wrap = v;
        self
    }
    pub fn padding(mut self, v: PaddingScale) -> Self {
        self.spec.padding = v;
        self
    }
    pub fn role(mut self, v: impl Into<String>) -> Self {
        self.spec.role = Some(v.into());
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }

    // ── GPUI-specific builders ────────────────────────────────
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for Stack {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let padding = spec.resolved_padding();

        // Direction
        let mut el = div().flex();
        match spec.direction {
            StackDirection::Column => {
                el = el.flex_col();
            }
            StackDirection::Row => {
                el = el.flex_row();
            }
        }

        // Wrap
        if spec.wrap {
            el = el.flex_wrap();
        }

        // Gap
        if let Some(gap_token) = spec.resolved_gap() {
            el = el.gap(resolve_px(theme, gap_token));
        }

        // Cross-axis alignment
        match spec.align {
            Alignment::Start => {
                el = el.items_start();
            }
            Alignment::Center => {
                el = el.items_center();
            }
            Alignment::End => {
                el = el.items_end();
            }
            Alignment::Stretch => {} // default
        }

        // Main-axis justification
        if let Some(justify) = &spec.justify {
            match justify {
                LayoutJustify::Start => {
                    el = el.justify_start();
                }
                LayoutJustify::End => {
                    el = el.justify_end();
                }
                LayoutJustify::Center => {
                    el = el.justify_center();
                }
                LayoutJustify::SpaceBetween => {
                    el = el.justify_between();
                }
            }
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
