//! DockRegion — real GPUI component backed by DockRegionSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_specs::{
    DockCollapsedPosture, DockEdge, DockEmphasis, DockRegionSpec, DockSizing, DockTabsPlacement,
    PanelTabItem,
};

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI dock region backed by `DockRegionSpec`.
///
/// Renders a dockable panel region with tabs along the specified edge or top.
/// Supports flexible (tabbed/collapsible) and static (stacked) sizing, collapsed
/// icon-strip / hidden postures, emphasis variants, compact tabs, and a
/// cross-region drop-target affordance.
pub struct DockRegion {
    spec: DockRegionSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    /// Content for the active panel.
    content: Option<AnyElement>,
    on_tab_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_collapse_toggle: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for DockRegion {
    type Target = DockRegionSpec;
    fn deref(&self) -> &DockRegionSpec {
        &self.spec
    }
}

impl DockRegion {
    pub fn new(edge: DockEdge, items: Vec<PanelTabItem>, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: DockRegionSpec::new(edge, items),
            theme: theme.clone(),
            id_prefix: String::new(),
            content: None,
            on_tab_change: None,
            on_collapse_toggle: None,
        }
    }

    pub fn from_spec(spec: DockRegionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-dock".to_string(),
            content: None,
            on_tab_change: None,
            on_collapse_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn edge(mut self, v: DockEdge) -> Self {
        self.spec.edge = v;
        self
    }
    pub fn sizing(mut self, v: DockSizing) -> Self {
        self.spec.sizing = v;
        self
    }
    pub fn collapsed(mut self, v: bool) -> Self {
        self.spec.is_collapsed = v;
        self
    }
    pub fn collapsible(mut self, v: bool) -> Self {
        self.spec.is_collapsible = v;
        self
    }
    pub fn collapsed_posture(mut self, v: DockCollapsedPosture) -> Self {
        self.spec.collapsed_posture = v;
        self
    }
    pub fn emphasis(mut self, v: DockEmphasis) -> Self {
        self.spec.emphasis = v;
        self
    }
    pub fn can_accept_panel(mut self, v: bool) -> Self {
        self.spec.can_accept_panel = v;
        self
    }
    pub fn tabs_placement(mut self, v: DockTabsPlacement) -> Self {
        self.spec.tabs_placement = v;
        self
    }
    pub fn items(mut self, v: Vec<PanelTabItem>) -> Self {
        self.spec.items = v;
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
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

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn on_tab_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tab_change = Some(Box::new(handler));
        self
    }

    pub fn on_collapse_toggle(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_collapse_toggle = Some(Box::new(handler));
        self
    }
}

impl IntoElement for DockRegion {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        // Density → spacing (contract: density controls horizontal padding / gaps).
        let space_x = rem_to_px(control_space_x_rem(spec.density));
        let space_y = rem_to_px(panel_space_y_rem(spec.density));
        let label_size = resolve_px(theme, "typography.label.size");
        let radius_control = resolve_radius(theme, "radius.control");
        let radius_surface = resolve_radius(theme, "radius.surface");

        let strip_fill = resolve_color(theme, spec.strip_fill_token());
        let panel_fill = resolve_color(theme, "color.background.panel");
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");
        let hover_bg = resolve_color(theme, "color.background.elevated");

        let current_value = spec.current_value().map(|s| s.to_string());
        let is_side_edge = matches!(spec.edge, DockEdge::Left | DockEdge::Right);

        // ── Root region treatment (emphasis + edge border) ─────────
        // Standard: panel fill, subtle border on the inner edge.
        // Quiet: transparent border + background.
        // Strong: accent mixed 32% into the subtle border.
        let (root_bg, root_border): (Hsla, Hsla) = match spec.emphasis {
            DockEmphasis::Standard => (panel_fill, border_subtle),
            DockEmphasis::Quiet => (
                gpui::transparent_black(),
                gpui::transparent_black(),
            ),
            DockEmphasis::Strong => (panel_fill, color_mix(accent, border_subtle, 0.32)),
        };

        let apply_edge_border = |el: Div, color: Hsla| -> Div {
            match spec.edge {
                DockEdge::Left => el.border_r_1().border_color(color),
                DockEdge::Right => el.border_l_1().border_color(color),
                DockEdge::Top => el.border_b_1().border_color(color),
                DockEdge::Bottom => el.border_t_1().border_color(color),
            }
        };

        // Helper to render a single horizontal/edge tab.
        // Rc so the closures below can each hold a clone. Mirrors the wired
        // Jetstream DockRegion.
        let on_tab_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_tab_change.map(|h| std::rc::Rc::from(h));
        let on_collapse_toggle: Option<std::rc::Rc<dyn Fn(bool, &mut Window, &mut App)>> =
            self.on_collapse_toggle.map(|h| std::rc::Rc::from(h));
        let next_collapsed = !spec.is_collapsed;

        let render_tab = |item: &PanelTabItem, is_active: bool, on_edge: bool, compact: bool| -> Stateful<Div> {
            let tab_id = SharedString::from(format!("{}-tab-{}", self.id_prefix, item.value));
            let mut tab = div()
                .id(tab_id)
                .flex()
                .items_center()
                .justify_center()
                .gap(px(space_x * 0.5))
                .cursor_pointer()
                .rounded(radius_control)
                .text_size(label_size);

            if on_edge {
                tab = tab.w_full().py(px(space_y * 0.5)).px(px(space_x));
            } else if compact {
                // Compact: icon-only, square-ish, no horizontal label padding.
                tab = tab.px(px(space_x * 0.5)).py(px(space_y * 0.5));
            } else {
                tab = tab.px(px(space_x)).py(px(space_y * 0.5));
            }

            if is_active {
                tab = tab.bg(accent.opacity(0.10)).text_color(accent);
            } else {
                tab = tab.text_color(text_secondary).hover(move |s| s.bg(hover_bg));
            }

            // Icon (when present). Compact / icon-strip render icon-only.
            if let Some(icon) = &item.icon {
                tab = tab.child(div().text_color(if is_active { accent } else { text_secondary }).child(icon.clone()));
            }
            if !compact {
                tab = tab.child(item.label.clone());
            }

            if let Some(handler) = &on_tab_change {
                let handler = handler.clone();
                let value = item.value.clone();
                tab = tab.on_click(move |_event, window, cx| {
                    handler(&value, window, cx);
                });
            }

            tab
        };

        // Collapse-toggle affordance (only when collapsible). Token-resolved
        // chevron glyph sized to the strip control height.
        let collapse_toggle = |vertical: bool| -> Stateful<Div> {
            let glyph = match spec.edge {
                DockEdge::Left => "‹",
                DockEdge::Right => "›",
                DockEdge::Top => "‹",
                DockEdge::Bottom => "›",
            };
            let mut t = div()
                .id(SharedString::from(format!("{}-collapse", self.id_prefix)))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .text_color(text_secondary)
                .text_size(px(font_size))
                .rounded(radius_control)
                .hover(move |s| s.bg(hover_bg))
                .child(glyph);
            if vertical {
                t = t.w_full().py(px(space_y * 0.5));
            } else {
                t = t.px(px(space_x * 0.5)).py(px(space_y * 0.5));
            }

            if let Some(handler) = &on_collapse_toggle {
                let handler = handler.clone();
                t = t.on_click(move |_event, window, cx| {
                    handler(next_collapsed, window, cx);
                });
            }

            t
        };

        // ── Static mode: stacked panels, no tabs / collapse ────────
        if spec.sizing == DockSizing::Static {
            // Left/right edge → row direction; top/bottom → column.
            let mut stack = div().flex().w_full().h_full();
            stack = if is_side_edge { stack.flex_row() } else { stack.flex_col() };

            for item in &spec.items {
                let mut cell = div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .id(SharedString::from(format!("{}-stack-{}", self.id_prefix, item.value)));
                // Drop-target affordance: accent inset ring when region accepts panels.
                if spec.can_accept_panel {
                    cell = cell.border_1().border_color(accent.opacity(0.0)).rounded(radius_control);
                }
                cell = cell.child(item.label.clone());
                stack = stack.child(cell);
            }

            let mut region = apply_edge_border(div().relative().w_full().h_full().bg(root_bg), root_border);
            region = region.child(stack);
            if let Some(content) = self.content {
                region = region.child(div().absolute().child(content));
            }
            // Drop-zone overlay affordance.
            if spec.can_accept_panel {
                region = region.child(
                    div()
                        .absolute()
                        .inset_0()
                        .border_2()
                        .border_color(accent.opacity(0.6))
                        .rounded(radius_surface)
                        .bg(accent.opacity(0.08)),
                );
            }
            return region.into_any_element();
        }

        // ── Collapsed: hidden posture (only the collapse toggle) ───
        if spec.is_collapsed && spec.collapsed_posture == DockCollapsedPosture::Hidden {
            let mut region = div()
                .relative()
                .flex()
                .items_center()
                .justify_center()
                .bg(gpui::transparent_black());
            region = if is_side_edge { region.h_full() } else { region.w_full() };
            if spec.is_collapsible {
                region = region.child(collapse_toggle(false).px(px(space_x)).py(px(space_y)));
            }
            return region.into_any_element();
        }

        // ── Collapsed: icon-strip posture ──────────────────────────
        if spec.is_collapsed && spec.collapsed_posture == DockCollapsedPosture::IconStrip {
            if is_side_edge {
                // Vertical icon strip: toggle on top, icon-only tabs below.
                let mut strip = div()
                    .flex()
                    .flex_col()
                    .items_start()
                    .gap(px(space_y * 0.5))
                    .py(px(space_y))
                    .bg(strip_fill);
                if spec.is_collapsible {
                    strip = strip.child(collapse_toggle(true));
                }
                for item in &spec.items {
                    let is_active = current_value.as_deref() == Some(item.value.as_str());
                    strip = strip.child(render_tab(item, is_active, true, true));
                }
                let region = apply_edge_border(
                    div().relative().h_full().bg(root_bg),
                    root_border,
                )
                .child(strip);
                return region.into_any_element();
            } else {
                // Horizontal compact icon strip: icon-only tabs + toggle.
                let mut strip = div()
                    .flex()
                    .items_center()
                    .gap(px(space_x * 0.5))
                    .px(px(space_x))
                    .h(px(rem_to_px(size_font_rem(effective_size)) + space_y * 2.0))
                    .bg(strip_fill);
                for item in &spec.items {
                    let is_active = current_value.as_deref() == Some(item.value.as_str());
                    strip = strip.child(render_tab(item, is_active, false, true));
                }
                if spec.is_collapsible {
                    strip = strip.child(collapse_toggle(false));
                }
                let region = apply_edge_border(
                    div().relative().w_full().bg(root_bg),
                    root_border,
                )
                .child(strip);
                return region.into_any_element();
            }
        }

        // ── Expanded flexible mode: strip (tabs + toggle) + body ───
        let is_tabs_on_edge = spec.tabs_placement == DockTabsPlacement::Edge && is_side_edge;

        let mut strip = div().bg(strip_fill);
        if is_tabs_on_edge {
            strip = strip
                .flex()
                .flex_col()
                .items_start()
                .gap(px(space_y * 0.5))
                .py(px(space_y));
        } else {
            strip = strip
                .flex()
                .items_center()
                .gap(px(space_x * 0.5))
                .px(px(space_x))
                .py(px(space_y * 0.5))
                .border_b_1()
                .border_color(border_subtle);
        }

        // Tab list grows; toggle is fixed at the end.
        let mut tab_list = div().flex_1().min_w_0().flex();
        tab_list = if is_tabs_on_edge {
            tab_list.flex_col().items_start().gap(px(space_y * 0.5))
        } else {
            tab_list.items_center().gap(px(space_x * 0.5))
        };
        for item in &spec.items {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            tab_list = tab_list.child(render_tab(item, is_active, is_tabs_on_edge, false));
        }
        strip = strip.child(tab_list);
        if spec.is_collapsible {
            strip = strip.child(collapse_toggle(is_tabs_on_edge));
        }

        // Assemble dock region root.
        let mut region = div().relative().bg(root_bg);
        if is_side_edge {
            region = region.flex().h_full();
        } else {
            region = region.flex().flex_col().w_full();
        }
        region = apply_edge_border(region, root_border);

        // Place strip and body in edge-aware order.
        if is_tabs_on_edge && spec.edge == DockEdge::Right {
            if let Some(content) = self.content {
                region = region.child(div().flex_1().overflow_hidden().child(content));
            }
            region = region.child(strip);
        } else {
            region = region.child(strip);
            if let Some(content) = self.content {
                region = region.child(
                    div()
                        .flex_1()
                        .min_h(px(0.0))
                        .overflow_hidden()
                        .child(content),
                );
            }
        }

        // Cross-region drop-zone overlay affordance.
        if spec.can_accept_panel {
            region = region.child(
                div()
                    .absolute()
                    .inset_0()
                    .border_2()
                    .border_color(accent.opacity(0.6))
                    .rounded(radius_surface)
                    .bg(accent.opacity(0.08)),
            );
        }

        region.into_any_element()
    }
}
