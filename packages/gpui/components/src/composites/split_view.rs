//! SplitView — real GPUI component backed by SplitViewSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::{SplitOrientation, SplitViewSpec};

use crate::theme_ext::resolve_color;

/// A real GPUI split view backed by `SplitViewSpec`.
///
/// Renders two panes separated by a draggable divider, either
/// horizontally or vertically, with configurable ratio and
/// collapsible panels.
pub struct SplitView {
    spec: SplitViewSpec,
    theme: GpuiThemeProvider,
    /// Primary (first) pane content.
    primary: Option<AnyElement>,
    /// Secondary (second) pane content.
    secondary: Option<AnyElement>,
    on_ratio_change: Option<Box<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for SplitView {
    type Target = SplitViewSpec;
    fn deref(&self) -> &SplitViewSpec { &self.spec }
}

impl SplitView {
    pub fn new(orientation: SplitOrientation, theme: &GpuiThemeProvider) -> Self {
        Self { spec: SplitViewSpec::new(orientation), theme: theme.clone(), primary: None, secondary: None, on_ratio_change: None }
    }

    pub fn from_spec(spec: SplitViewSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            primary: None,
            secondary: None,
            on_ratio_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn orientation(mut self, v: SplitOrientation) -> Self { self.spec.orientation = v; self }
    pub fn ratio(mut self, v: f32) -> Self { self.spec.ratio = Some(v); self }
    pub fn default_ratio(mut self, v: f32) -> Self { self.spec.default_ratio = v; self }
    pub fn min_primary_size(mut self, v: f32) -> Self { self.spec.min_primary_size = Some(v); self }
    pub fn min_secondary_size(mut self, v: f32) -> Self { self.spec.min_secondary_size = Some(v); self }
    pub fn primary_collapsed(mut self, v: bool) -> Self { self.spec.is_primary_collapsed = v; self }
    pub fn secondary_collapsed(mut self, v: bool) -> Self { self.spec.is_secondary_collapsed = v; self }


    pub fn with_primary(mut self, content: impl IntoElement) -> Self {
        self.primary = Some(content.into_any_element());
        self
    }

    pub fn with_secondary(mut self, content: impl IntoElement) -> Self {
        self.secondary = Some(content.into_any_element());
        self
    }

    pub fn on_ratio_change(
        mut self,
        handler: impl Fn(f32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_ratio_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for SplitView {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, "semantic.color.border.default");
        let ratio = spec.current_ratio();
        let is_horizontal = spec.orientation == SplitOrientation::Horizontal;

        let mut container = div().size_full();

        if is_horizontal {
            container = container.flex();
        } else {
            container = container.flex().flex_col();
        }

        // Primary pane
        if !spec.is_primary_collapsed {
            let mut primary_pane = div()
                .overflow_hidden()
                .flex_basis(relative(ratio))
                .flex_shrink()
                .flex_grow();

            if is_horizontal {
                primary_pane = primary_pane.h_full();
            } else {
                primary_pane = primary_pane.w_full();
            }

            if let Some(primary) = self.primary {
                primary_pane = primary_pane.child(primary);
            }

            container = container.child(primary_pane);
        }

        // Divider
        if !spec.is_primary_collapsed && !spec.is_secondary_collapsed {
            let mut divider = div().flex_shrink_0();

            if is_horizontal {
                divider = divider
                    .w(px(4.0))
                    .h_full()
                    .cursor_col_resize()
                    .bg(border);
            } else {
                divider = divider
                    .h(px(4.0))
                    .w_full()
                    .cursor_row_resize()
                    .bg(border);
            }

            divider = divider.hover(|s| {
                s.bg(resolve_color(theme, "semantic.color.accent.base").opacity(0.3))
            });

            container = container.child(divider);
        }

        // Secondary pane
        if !spec.is_secondary_collapsed {
            let secondary_ratio = 1.0 - ratio;
            let mut secondary_pane = div()
                .overflow_hidden()
                .flex_basis(relative(secondary_ratio))
                .flex_shrink()
                .flex_grow();

            if is_horizontal {
                secondary_pane = secondary_pane.h_full();
            } else {
                secondary_pane = secondary_pane.w_full();
            }

            if let Some(secondary) = self.secondary {
                secondary_pane = secondary_pane.child(secondary);
            }

            container = container.child(secondary_pane);
        }

        container.into_any_element()
    }
}
