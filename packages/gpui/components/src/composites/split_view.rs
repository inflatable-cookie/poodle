//! SplitView — real GPUI component backed by SplitViewSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CollapseDirection, CollapseToggleSpec, ControlDensity, ControlSize, Orientation,
    ResizeHandleSpec, SemanticControlSizeRole,
};
use poodle_specs::{SplitOrientation, SplitViewSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::primitives::{CollapseToggle, ResizeHandle};

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
    fn deref(&self) -> &SplitViewSpec {
        &self.spec
    }
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
    pub fn orientation(mut self, v: SplitOrientation) -> Self {
        self.spec.orientation = v;
        self
    }
    pub fn ratio(mut self, v: f32) -> Self {
        self.spec.ratio = Some(v);
        self
    }
    pub fn default_ratio(mut self, v: f32) -> Self {
        self.spec.default_ratio = v;
        self
    }
    pub fn min_primary_size(mut self, v: f32) -> Self {
        self.spec.min_primary_size = Some(v);
        self
    }
    pub fn min_secondary_size(mut self, v: f32) -> Self {
        self.spec.min_secondary_size = Some(v);
        self
    }
    pub fn primary_collapsed(mut self, v: bool) -> Self {
        self.spec.is_primary_collapsed = v;
        self
    }
    pub fn secondary_collapsed(mut self, v: bool) -> Self {
        self.spec.is_secondary_collapsed = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn show_collapse_primary(mut self, v: bool) -> Self {
        self.spec.show_collapse_primary = v;
        self
    }
    pub fn show_collapse_secondary(mut self, v: bool) -> Self {
        self.spec.show_collapse_secondary = v;
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

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
            let disabled_opacity =
                crate::theme_ext::resolve_opacity(theme, "state.opacity.disabled");
            container = container.opacity(disabled_opacity);
        }

        // Primary pane
        //
        // Fixed-size takes precedence over ratio: if spec.primary_size
        // is set, the primary pane gets that absolute dimension and
        // secondary fills the remaining space (flex_grow). If
        // spec.secondary_size is set, primary fills remaining space
        // and secondary takes the fixed dimension. Falling back to
        // ratio-based allocation when neither is set.
        let primary_fixed = spec.primary_size;
        let secondary_fixed = spec.secondary_size;
        if !spec.is_primary_collapsed {
            let mut primary_pane = div().overflow_hidden();

            // Flex allocation: fixed → no grow/shrink; filling → grow;
            // ratio-based → grow+shrink with flex_basis.
            if primary_fixed.is_some() {
                primary_pane = primary_pane.flex_shrink_0();
            } else if secondary_fixed.is_some() {
                primary_pane = primary_pane.flex_grow();
            } else {
                primary_pane = primary_pane
                    .flex_basis(relative(ratio))
                    .flex_shrink()
                    .flex_grow();
            }

            if is_horizontal {
                if let Some(size) = primary_fixed {
                    primary_pane = primary_pane.w(px(size));
                }
                primary_pane = primary_pane.h_full();
                if let Some(min) = spec.min_primary_size {
                    primary_pane = primary_pane.min_w(px(min));
                }
            } else {
                if let Some(size) = primary_fixed {
                    primary_pane = primary_pane.h(px(size));
                }
                primary_pane = primary_pane.w_full();
                if let Some(min) = spec.min_primary_size {
                    primary_pane = primary_pane.min_h(px(min));
                }
            }

            if let Some(primary) = self.primary {
                primary_pane = primary_pane.child(primary);
            }

            container = container.child(primary_pane);
        }

        // Divider — composes the real ResizeHandle primitive plus optional
        // CollapseToggle primitives.
        //
        // The divider container is the handle's line thickness on the split axis
        // (contract §7 — the grab area is an overlay), running the full
        // cross-axis length. The ResizeHandle renders the
        // draggable / keyboard-resizable separator (separator semantics,
        // aria-label="Resize"). When collapse toggles are enabled they overlay
        // the handle as a centred cluster. Clicking a toggle fires the matching
        // on_primary_collapse / on_secondary_collapse callback with the *flipped*
        // collapse state (Collapse when expanded, Expand when collapsed).
        if !spec.is_primary_collapsed && !spec.is_secondary_collapsed {
            // ResizeHandle line axis is the inverse of the split axis:
            // horizontal split (side-by-side) → vertical handle line.
            let handle_orientation = if is_horizontal {
                Orientation::Horizontal
            } else {
                Orientation::Vertical
            };
            let handle = ResizeHandle::from_spec(
                ResizeHandleSpec::new()
                    .with_orientation(handle_orientation)
                    .with_disabled(spec.is_disabled)
                    .with_aria_value_now(ratio),
                theme,
            )
            .with_id("split-resize");

            // Contract toggle visibility (both panes are expanded in this
            // branch, so only the show_collapse_* flags gate them).
            let (primary_dir, secondary_dir) = if is_horizontal {
                (CollapseDirection::Left, CollapseDirection::Right)
            } else {
                (CollapseDirection::Up, CollapseDirection::Down)
            };

            let primary_collapsed = spec.is_primary_collapsed;
            let secondary_collapsed = spec.is_secondary_collapsed;
            let disabled = spec.is_disabled;

            let primary_toggle = if spec.show_collapse_primary {
                let mut toggle = CollapseToggle::from_spec(
                    CollapseToggleSpec::new()
                        .with_direction(primary_dir)
                        .with_collapsed(primary_collapsed)
                        .with_disabled(disabled)
                        .with_aria_label(if primary_collapsed {
                            "Expand primary"
                        } else {
                            "Collapse primary"
                        }),
                    theme,
                )
                .with_id("split-primary");
                if let Some(handler) = self.on_primary_collapse {
                    // Toggle flips state: Expand when already collapsed.
                    toggle = toggle.on_toggle(move |_event, window, cx| {
                        handler(!primary_collapsed, window, cx);
                    });
                }
                Some(toggle.into_any_element())
            } else {
                None
            };

            let secondary_toggle = if spec.show_collapse_secondary {
                let mut toggle = CollapseToggle::from_spec(
                    CollapseToggleSpec::new()
                        .with_direction(secondary_dir)
                        .with_collapsed(secondary_collapsed)
                        .with_disabled(disabled)
                        .with_aria_label(if secondary_collapsed {
                            "Expand secondary"
                        } else {
                            "Collapse secondary"
                        }),
                    theme,
                )
                .with_id("split-secondary");
                if let Some(handler) = self.on_secondary_collapse {
                    toggle = toggle.on_toggle(move |_event, window, cx| {
                        handler(!secondary_collapsed, window, cx);
                    });
                }
                Some(toggle.into_any_element())
            } else {
                None
            };

            let has_toggles = primary_toggle.is_some() || secondary_toggle.is_some();

            // Toggle cluster: horizontal splits stack toggles in a column
            // (primary above secondary); vertical splits stack them in a row.
            let toggles_gap = px(rem_to_px(0.125));
            let toggle_cluster = if has_toggles {
                let mut cluster = if is_horizontal {
                    div().absolute().flex().flex_col().gap(toggles_gap)
                } else {
                    div().absolute().flex().flex_row().gap(toggles_gap)
                }
                .items_center()
                .justify_center()
                .p(px(rem_to_px(0.125)));
                if let Some(btn) = primary_toggle {
                    cluster = cluster.child(btn);
                }
                if let Some(btn) = secondary_toggle {
                    cluster = cluster.child(btn);
                }
                Some(cluster)
            } else {
                None
            };

            // Divider container: as thick as the handle's line on the split
            // axis, full cross-axis. The handle overlays its own grab area, so
            // reserving the grab width here would push the panes apart again
            // (contract §7).
            let divider_thickness = px(rem_to_px(ResizeHandleSpec::new().thickness_rem()));
            let mut divider = div()
                .flex_shrink_0()
                .relative()
                .flex()
                .items_center()
                .justify_center();
            if is_horizontal {
                divider = divider.w(divider_thickness).h_full();
                if !spec.is_disabled {
                    divider = divider.cursor_col_resize();
                }
            } else {
                divider = divider.h(divider_thickness).w_full();
                if !spec.is_disabled {
                    divider = divider.cursor_row_resize();
                }
            }

            divider = divider.child(handle);
            if let Some(cluster) = toggle_cluster {
                divider = divider.child(cluster);
            }

            container = container.child(divider);
        }

        // Secondary pane
        if !spec.is_secondary_collapsed {
            let secondary_ratio = 1.0 - ratio;
            let mut secondary_pane = div().overflow_hidden();

            // Symmetric to the primary allocation above.
            if secondary_fixed.is_some() {
                secondary_pane = secondary_pane.flex_shrink_0();
            } else if primary_fixed.is_some() {
                secondary_pane = secondary_pane.flex_grow();
            } else {
                secondary_pane = secondary_pane
                    .flex_basis(relative(secondary_ratio))
                    .flex_shrink()
                    .flex_grow();
            }

            if is_horizontal {
                if let Some(size) = secondary_fixed {
                    secondary_pane = secondary_pane.w(px(size));
                }
                secondary_pane = secondary_pane.h_full();
                if let Some(min) = spec.min_secondary_size {
                    secondary_pane = secondary_pane.min_w(px(min));
                }
            } else {
                if let Some(size) = secondary_fixed {
                    secondary_pane = secondary_pane.h(px(size));
                }
                secondary_pane = secondary_pane.w_full();
                if let Some(min) = spec.min_secondary_size {
                    secondary_pane = secondary_pane.min_h(px(min));
                }
            }

            if let Some(secondary) = self.secondary {
                secondary_pane = secondary_pane.child(secondary);
            }

            container = container.child(secondary_pane);
        }

        container.into_any_element()
    }
}
