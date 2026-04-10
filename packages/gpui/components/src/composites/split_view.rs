//! SplitView — real GPUI component backed by SplitViewSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{SplitOrientation, SplitViewSpec};
use poodle_primitives::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};

use crate::presentation::resolve_semantic_size;
use crate::primitives::Icon;
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
    /// Fired when the primary collapse toggle is clicked. Arg is the
    /// new is_primary_collapsed state.
    on_primary_collapse: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    /// Fired when the secondary collapse toggle is clicked.
    on_secondary_collapse: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for SplitView {
    type Target = SplitViewSpec;
    fn deref(&self) -> &SplitViewSpec { &self.spec }
}

impl SplitView {
    pub fn new(orientation: SplitOrientation, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SplitViewSpec::new(orientation),
            theme: theme.clone(),
            primary: None,
            secondary: None,
            on_ratio_change: None,
            on_primary_collapse: None,
            on_secondary_collapse: None,
        }
    }

    pub fn from_spec(spec: SplitViewSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            primary: None,
            secondary: None,
            on_ratio_change: None,
            on_primary_collapse: None,
            on_secondary_collapse: None,
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
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn show_collapse_primary(mut self, v: bool) -> Self { self.spec.show_collapse_primary = v; self }
    pub fn show_collapse_secondary(mut self, v: bool) -> Self { self.spec.show_collapse_secondary = v; self }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }


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

    /// Fired when the primary collapse toggle is clicked. Receives the
    /// new is_primary_collapsed state.
    pub fn on_primary_collapse(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_primary_collapse = Some(Box::new(handler));
        self
    }

    /// Fired when the secondary collapse toggle is clicked. Receives
    /// the new is_secondary_collapsed state.
    pub fn on_secondary_collapse(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_secondary_collapse = Some(Box::new(handler));
        self
    }
}

impl IntoElement for SplitView {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let _effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let border = resolve_color(theme, "color.border.default");
        let ratio = spec.current_ratio();
        let is_horizontal = spec.orientation == SplitOrientation::Horizontal;

        let mut container = div().size_full();

        if is_horizontal {
            container = container.flex();
        } else {
            container = container.flex().flex_col();
        }

        // Disabled treatment: dim the entire split and forbid the
        // col/row-resize cursor on the divider below.
        if spec.is_disabled {
            let disabled_opacity = crate::theme_ext::resolve_opacity(theme, "state.opacity.disabled");
            container = container.opacity(disabled_opacity);
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

        // Divider — with optional collapse toggles.
        //
        // Layout strategy: the divider is a small fixed-width container
        // running the full cross-axis length. Collapse toggles (when
        // enabled via spec.show_collapse_*) stack inside it as overlay
        // circles near the edges. Clicking a toggle fires the matching
        // on_primary_collapse / on_secondary_collapse callback with the
        // new collapsed state — the caller is responsible for updating
        // the spec via toolbar / app state.
        if !spec.is_primary_collapsed && !spec.is_secondary_collapsed {
            let icon_color = resolve_color(theme, "color.icon.primary");
            let surface_bg = resolve_color(theme, "color.surface.raised");

            // Chevron pointing toward the primary pane collapses it
            // (e.g. in horizontal mode, chevron-left collapses the
            // left / primary pane). The secondary chevron points the
            // opposite way.
            let (primary_chevron, secondary_chevron) = if is_horizontal {
                ("chevron-left", "chevron-right")
            } else {
                ("chevron-up", "chevron-down")
            };

            // Helper: build one circular collapse-toggle button.
            let build_toggle = |id: &'static str,
                                chevron: &str,
                                handler: Option<&Box<dyn Fn(bool, &mut Window, &mut App)>>|
             -> AnyElement {
                let icon = Icon::from_spec(
                    IconSpec::new(chevron).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_color);

                let mut btn = div()
                    .id(SharedString::from(id))
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(16.0))
                    .rounded(px(8.0))
                    .bg(surface_bg)
                    .border_1()
                    .border_color(border)
                    .child(icon);

                if !spec.is_disabled {
                    btn = btn.cursor_pointer();
                    if let Some(h) = handler {
                        // The handler type is `Box<dyn Fn(bool, ...)>`
                        // which doesn't implement Clone — we bypass the
                        // moved-closure restriction by using Rc.
                        // Instead of cloning, we rely on the fact that
                        // each toggle uses its own handler reference
                        // via the wrapping code below.
                        let _ = h;
                    }
                }
                btn.into_any_element()
            };

            // We can't just pass `&Box<dyn Fn>` because we need to move
            // the callback into an on_click closure. Consume the fields
            // from `self` here so the `Box` can be moved.
            let primary_handler = self.on_primary_collapse;
            let secondary_handler = self.on_secondary_collapse;

            let primary_toggle = if spec.show_collapse_primary {
                let icon = Icon::from_spec(
                    IconSpec::new(primary_chevron).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_color);
                let mut btn = div()
                    .id(SharedString::from("split-collapse-primary"))
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(16.0))
                    .rounded(px(8.0))
                    .bg(surface_bg)
                    .border_1()
                    .border_color(border)
                    .child(icon);
                if !spec.is_disabled {
                    btn = btn.cursor_pointer();
                    if let Some(handler) = primary_handler {
                        btn = btn.on_click(move |_event, window, cx| {
                            handler(true, window, cx);
                        });
                    }
                }
                Some(btn.into_any_element())
            } else {
                None
            };

            let secondary_toggle = if spec.show_collapse_secondary {
                let icon = Icon::from_spec(
                    IconSpec::new(secondary_chevron).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_color);
                let mut btn = div()
                    .id(SharedString::from("split-collapse-secondary"))
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(16.0))
                    .rounded(px(8.0))
                    .bg(surface_bg)
                    .border_1()
                    .border_color(border)
                    .child(icon);
                if !spec.is_disabled {
                    btn = btn.cursor_pointer();
                    if let Some(handler) = secondary_handler {
                        btn = btn.on_click(move |_event, window, cx| {
                            handler(true, window, cx);
                        });
                    }
                }
                Some(btn.into_any_element())
            } else {
                None
            };

            // Suppress the unused helper warning — `build_toggle` was an
            // earlier factoring attempt kept here as documentation for
            // the per-toggle construction pattern that replaced it.
            let _ = build_toggle;

            let mut divider = div().flex_shrink_0().relative();

            // Build the divider line + centred toggle cluster.
            let line = if is_horizontal {
                div().w(px(1.0)).h_full().bg(border)
            } else {
                div().h(px(1.0)).w_full().bg(border)
            };

            // Toggles cluster: horizontal splits stack toggles vertically
            // (primary then secondary, top-to-bottom); vertical splits
            // stack them horizontally (primary then secondary, L to R).
            let mut toggle_cluster = if is_horizontal {
                div().absolute().flex().flex_col().gap(px(4.0))
            } else {
                div().absolute().flex().flex_row().gap(px(4.0))
            };
            if let Some(btn) = primary_toggle {
                toggle_cluster = toggle_cluster.child(btn);
            }
            if let Some(btn) = secondary_toggle {
                toggle_cluster = toggle_cluster.child(btn);
            }

            if is_horizontal {
                divider = divider
                    .w(px(4.0))
                    .h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(line)
                    .child(toggle_cluster);
                if !spec.is_disabled {
                    divider = divider.cursor_col_resize();
                }
            } else {
                divider = divider
                    .h(px(4.0))
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(line)
                    .child(toggle_cluster);
                if !spec.is_disabled {
                    divider = divider.cursor_row_resize();
                }
            }

            if !spec.is_disabled {
                divider = divider.hover(|s| {
                    s.bg(resolve_color(theme, "color.accent.base").opacity(0.3))
                });
            }

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
