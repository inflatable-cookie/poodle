//! FilterToolbar — layout container for filter controls backed by FilterToolbarSpec.
//!
//! Renders a bordered panel with:
//!   1. Header row — optional collapse toggle, summary text, actions slot
//!   2. Grid of filter controls (children) when expanded
//!   3. Optional secondary row below the grid
//!
//! Matches the Svelte FilterToolbar composite (packages/svelte/components/src/FilterToolbar.svelte).

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::FilterToolbarSpec;
use poodle_specs::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};

use crate::presentation::rem_to_px;
use crate::primitives::Icon;
use crate::theme_ext::{elevation_surface_shadow, resolve_color, resolve_px, resolve_radius};

pub struct FilterToolbar {
    spec: FilterToolbarSpec,
    theme: GpuiThemeProvider,
    /// Filter control children rendered in the responsive grid.
    children: Vec<AnyElement>,
    /// Actions slot — typically icon buttons next to the header summary.
    actions: Option<AnyElement>,
    /// Secondary slot — rendered below the main grid, e.g. a "Reset all" button.
    secondary: Option<AnyElement>,
    /// Click handler fired when the collapse toggle is clicked.
    on_toggle: Option<std::rc::Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for FilterToolbar {
    type Target = FilterToolbarSpec;
    fn deref(&self) -> &FilterToolbarSpec {
        &self.spec
    }
}

impl FilterToolbar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: FilterToolbarSpec::new(),
            theme: theme.clone(),
            children: Vec::new(),
            actions: None,
            secondary: None,
            on_toggle: None,
        }
    }

    pub fn from_spec(spec: FilterToolbarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
            actions: None,
            secondary: None,
            on_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = v.into();
        self
    }
    pub fn summary_text(mut self, v: impl Into<String>) -> Self {
        self.spec.summary_text = Some(v.into());
        self
    }
    pub fn collapsible(mut self, v: bool) -> Self {
        self.spec.collapsible = v;
        self
    }
    pub fn collapsed(mut self, v: bool) -> Self {
        self.spec.collapsed = v;
        self
    }
    pub fn columns(mut self, v: u32) -> Self {
        self.spec.columns = v;
        self
    }
    pub fn min_item_width_rem(mut self, v: f32) -> Self {
        self.spec.min_item_width_rem = v;
        self
    }
    pub fn sticky(mut self, v: bool) -> Self {
        self.spec.sticky = v;
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

    /// Append a filter control child (TextInput, Select, etc.).
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Append multiple filter control children at once.
    pub fn with_children(mut self, children: impl IntoIterator<Item = AnyElement>) -> Self {
        self.children.extend(children);
        self
    }

    /// Set the actions slot (rendered in the header row).
    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }

    /// Set the secondary slot (rendered below the main grid).
    pub fn with_secondary(mut self, secondary: impl IntoElement) -> Self {
        self.secondary = Some(secondary.into_any_element());
        self
    }

    /// Click handler for the collapse toggle. Signature matches
    /// `Div::on_click` so `cx.listener(...)` passes through.
    pub fn on_toggle(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for FilterToolbar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let _effective_size_role = spec.size_role;

        // Contract §8 density table: root padding-block/inline, root gap,
        // and controls gap all vary by density.
        let panel_px = px(rem_to_px(spec.padding_inline_rem()));
        let panel_py = px(rem_to_px(spec.padding_block_rem()));

        // Root gap: density compact resolves a literal rem, otherwise the
        // density-aware gap token (default → inline.sm, comfortable → inline.md).
        let root_gap = match spec.density_gap_rem() {
            Some(rem) => px(rem_to_px(rem)),
            None => resolve_px(theme, spec.gap_token()),
        };
        // Header row gap: space.inline.sm.
        let header_gap = resolve_px(theme, "space.inline.sm");
        // Controls grid gap: density compact resolves a literal rem, otherwise
        // the density-aware controls gap token.
        let controls_gap = match spec.density_controls_gap_rem() {
            Some(rem) => px(rem_to_px(rem)),
            None => resolve_px(theme, spec.controls_gap_token()),
        };
        let actions_gap = resolve_px(theme, spec.actions_gap_token());
        let summary_size = px(rem_to_px(spec.summary_font_size_rem()));

        let elevated_bg = resolve_color(theme, spec.background_token());
        let bg = Hsla {
            a: elevated_bg.a * spec.background_opacity(),
            ..elevated_bg
        };
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let summary_color = resolve_color(theme, spec.summary_color_token());
        let icon_muted = resolve_color(theme, "color.icon.muted");
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let focus_ring_width = resolve_px(theme, spec.focus_ring_width_token());
        // Collapse-toggle hit-area + corner radius, both token-resolved.
        let toggle_size = resolve_px(theme, spec.toggle_size_token());
        let toggle_radius = resolve_radius(theme, spec.toggle_radius_token());

        let is_expanded = spec.is_grid_visible();

        // ── Root container ───────────────────────────────────────
        let mut toolbar = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(root_gap)
            .px(panel_px)
            .py(panel_py)
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(radius);

        // Sticky elevation — token-resolved `elevation.surface` shadow.
        if spec.sticky {
            toolbar = toolbar.shadow(elevation_surface_shadow());
        }

        // ── Header row ───────────────────────────────────────────
        let needs_header =
            spec.collapsible || spec.summary_text.is_some() || self.actions.is_some();

        if needs_header {
            // Build the header content into a child list, then attach it to a
            // stateful (collapsible, focus-ring-bearing) or plain container.
            let mut header_children: Vec<AnyElement> = Vec::new();

            // Collapse toggle (chevron) — token-resolved hit-area + radius.
            if spec.collapsible {
                let chevron_name = if is_expanded {
                    "chevron-down"
                } else {
                    "chevron-right"
                };
                let icon =
                    Icon::from_spec(IconSpec::new(chevron_name).with_size(IconSize::Sm), theme)
                        .with_color(icon_muted);

                header_children.push(
                    div()
                        .w(toggle_size)
                        .h(toggle_size)
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded(toggle_radius)
                        .child(icon)
                        .into_any_element(),
                );
            }

            // Summary text
            if let Some(ref summary) = spec.summary_text {
                header_children.push(
                    div()
                        .flex_1()
                        .text_size(summary_size)
                        .text_color(summary_color)
                        .child(summary.clone())
                        .into_any_element(),
                );
            } else {
                // Reserve the flex-1 so actions still anchor right.
                header_children.push(div().flex_1().into_any_element());
            }

            // Actions slot
            if let Some(actions) = self.actions {
                header_children.push(
                    div()
                        .flex()
                        .items_center()
                        // Contract §8 actions gap = space.inline.xs (0.25rem).
                        .gap(actions_gap)
                        .flex_shrink_0()
                        .child(actions)
                        .into_any_element(),
                );
            }

            if spec.collapsible {
                // The whole header row is the toggle target and carries the
                // focus ring (contract §6). GPUI has no ARIA channel, but the
                // focus-visible outline is reproduced here.
                let mut header_row = div()
                    .id("filter-toolbar-header")
                    .flex()
                    .items_center()
                    .gap(header_gap)
                    .cursor_pointer()
                    .focusable()
                    .focus(move |s| s.border(focus_ring_width).border_color(focus_ring))
                    .children(header_children);

                if let Some(ref handler) = self.on_toggle {
                    let handler = handler.clone();
                    header_row = header_row.on_click(move |event, window, cx| {
                        handler(event, window, cx);
                    });
                }

                toolbar = toolbar.child(header_row);
            } else {
                toolbar = toolbar.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(header_gap)
                        .children(header_children),
                );
            }
        }

        // Capture "had children" before draining them so the secondary
        // slot can make layout decisions after the grid is consumed.
        let had_children = !self.children.is_empty();

        // ── Filter controls grid ─────────────────────────────────
        if is_expanded && had_children {
            // Flex-wrap grid where each child has a minimum width so the
            // row collapses to fewer columns on narrow containers. GPUI
            // doesn't have CSS grid so flex-wrap with min-width on each
            // child is the closest equivalent to the Svelte implementation.
            let min_item_width = px(rem_to_px(spec.min_item_width_rem));
            let mut grid = div().flex().flex_wrap().gap(controls_gap).w_full();

            for child in self.children {
                grid = grid.child(
                    div()
                        .flex_grow()
                        .flex_shrink()
                        .min_w(min_item_width)
                        .child(child),
                );
            }

            toolbar = toolbar.child(grid);
        }

        // ── Secondary slot ───────────────────────────────────────
        if let Some(secondary) = self.secondary {
            let _ = had_children;
            toolbar = toolbar.child(div().w_full().child(secondary));
        }

        toolbar.into_any_element()
    }
}
