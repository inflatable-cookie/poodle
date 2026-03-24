//! DockRegion — real GPUI component backed by DockRegionSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{DockEdge, DockRegionSpec, DockTabsPlacement, PanelTabItem};

use crate::theme_ext::resolve_color;

/// A real GPUI dock region backed by `DockRegionSpec`.
///
/// Renders a dockable panel region with tabs along the specified edge or top.
/// Supports collapsed state with a narrow gutter.
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
    fn deref(&self) -> &DockRegionSpec { &self.spec }
}

impl DockRegion {
    pub fn new(edge: DockEdge, items: Vec<PanelTabItem>, theme: &GpuiThemeProvider) -> Self {
        Self { spec: DockRegionSpec::new(edge, items), theme: theme.clone(), id_prefix: String::new(), content: None, on_tab_change: None, on_collapse_toggle: None }
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
    pub fn edge(mut self, v: DockEdge) -> Self { self.spec.edge = v; self }
    pub fn collapsed(mut self, v: bool) -> Self { self.spec.is_collapsed = v; self }
    pub fn tabs_placement(mut self, v: DockTabsPlacement) -> Self { self.spec.tabs_placement = v; self }
    pub fn items(mut self, v: Vec<PanelTabItem>) -> Self { self.spec.items = v; self }
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


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

        let strip_fill = resolve_color(theme, spec.strip_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let _text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let hover_bg = resolve_color(theme, "semantic.color.background.elevated");

        let current_value = spec.current_value().map(|s| s.to_string());
        let is_horizontal_edge = matches!(spec.edge, DockEdge::Left | DockEdge::Right);

        // Collapsed gutter
        if spec.is_collapsed {
            let mut gutter = div().bg(strip_fill);

            if is_horizontal_edge {
                gutter = gutter.w(px(4.0)).h_full();
            } else {
                gutter = gutter.h(px(4.0)).w_full();
            }

            // Border on the inner edge
            match spec.edge {
                DockEdge::Left => gutter = gutter.border_r_1().border_color(border),
                DockEdge::Right => gutter = gutter.border_l_1().border_color(border),
                DockEdge::Top => gutter = gutter.border_b_1().border_color(border),
                DockEdge::Bottom => gutter = gutter.border_t_1().border_color(border),
            }

            return gutter.into_any_element();
        }

        // Tabs strip
        let is_tabs_on_edge = spec.tabs_placement == DockTabsPlacement::Edge && is_horizontal_edge;

        let mut tabs_strip = div().bg(strip_fill);

        if is_tabs_on_edge {
            // Vertical tab strip along the edge
            tabs_strip = tabs_strip.flex().flex_col().gap(px(2.0)).py(px(4.0)).w(px(36.0));
        } else {
            // Horizontal tab strip on top
            tabs_strip = tabs_strip
                .flex()
                .items_center()
                .gap(px(2.0))
                .px(px(4.0))
                .h(px(32.0))
                .border_b_1()
                .border_color(border);
        }

        for item in &spec.items {
            let is_active = current_value.as_deref() == Some(item.value.as_str());
            let tab_id = SharedString::from(format!("{}-tab-{}", self.id_prefix, item.value));

            let mut tab = div()
                .id(tab_id)
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .rounded(px(3.0));

            if is_tabs_on_edge {
                tab = tab.w_full().py(px(6.0)).text_size(px(12.0));
            } else {
                tab = tab.px(px(8.0)).py(px(4.0)).text_size(px(12.0));
            }

            if is_active {
                tab = tab
                    .bg(accent.opacity(0.10))
                    .text_color(accent);
            } else {
                tab = tab
                    .text_color(text_secondary)
                    .hover(|s| s.bg(hover_bg));
            }

            tab = tab.child(item.label.clone());
            tabs_strip = tabs_strip.child(tab);
        }

        // Assemble dock region
        let mut region = div();

        if is_horizontal_edge {
            region = region.flex().h_full();
        } else {
            region = region.flex().flex_col().w_full();
        }

        // Border on outer edge
        match spec.edge {
            DockEdge::Left => region = region.border_r_1().border_color(border),
            DockEdge::Right => region = region.border_l_1().border_color(border),
            DockEdge::Top => region = region.border_b_1().border_color(border),
            DockEdge::Bottom => region = region.border_t_1().border_color(border),
        }

        // Place tabs strip and content
        if is_tabs_on_edge && spec.edge == DockEdge::Right {
            // Content first, then tabs on the right
            if let Some(content) = self.content {
                region = region.child(
                    div().flex_1().overflow_hidden().child(content),
                );
            }
            region = region.child(tabs_strip);
        } else {
            // Tabs first, then content
            region = region.child(tabs_strip);
            if let Some(content) = self.content {
                region = region.child(
                    div().flex_1().overflow_hidden().child(content),
                );
            }
        }

        region.into_any_element()
    }
}
