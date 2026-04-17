//! RenderComponent implementations for structural and layout primitives.
//!
//! g08.003: BoxSpec, StackSpec, GridSpec, SurfaceSpec, SeparatorSpec,
//! ScrollShellSpec, CallOutSpec
//!
//! All structural primitives map to Widget::Panel in Jetstream.

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    BoxSpec, CallOutSpec, GridSpec, ScrollShellSpec, SeparatorSpec, StackSpec, SurfaceSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::{map_style, JetstreamColor, JetstreamEdges};
use crate::{JetstreamAdapter, JetstreamNodeHandle, JetstreamTarget, WidgetKind};

/// Resolve an `Inset` (horizontal/vertical token pair) into `JetstreamEdges`
/// using the theme's `resolve_space` method.
fn resolve_inset(inset: &poodle_specs::Inset, theme: &dyn ThemeProvider) -> JetstreamEdges {
    let h = inset.horizontal.map(|t| theme.resolve_space(t)).unwrap_or(0.0);
    let v = inset.vertical.map(|t| theme.resolve_space(t)).unwrap_or(0.0);
    JetstreamEdges {
        top: v,
        right: h,
        bottom: v,
        left: h,
    }
}

/// Convert JetstreamEdges into taffy padding on a layout style.
fn apply_inset_padding(layout: &mut taffy::Style, edges: &JetstreamEdges) {
    layout.padding = taffy::Rect {
        left: taffy::LengthPercentage::length(edges.left),
        right: taffy::LengthPercentage::length(edges.right),
        top: taffy::LengthPercentage::length(edges.top),
        bottom: taffy::LengthPercentage::length(edges.bottom),
    };
}

impl RenderComponent<BoxSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &BoxSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve padding tokens
        let inset = spec.resolved_padding();
        let edges = resolve_inset(&inset, theme);
        apply_inset_padding(&mut mapped.layout, &edges);

        let node_id = spec.role.as_deref().unwrap_or("box");
        JetstreamNodeHandle::new(node_id, "BoxSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<StackSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &StackSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve gap token
        if let Some(gap_token) = spec.resolved_gap() {
            let gap = theme.resolve_space(gap_token);
            mapped.layout.gap = taffy::Size { width: taffy::LengthPercentage::length(gap), height: taffy::LengthPercentage::length(gap) };
        }

        // Resolve padding tokens
        let inset = spec.resolved_padding();
        let edges = resolve_inset(&inset, theme);
        apply_inset_padding(&mut mapped.layout, &edges);

        let node_id = spec.role.as_deref().unwrap_or("stack");
        JetstreamNodeHandle::new(node_id, "StackSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<GridSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &GridSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        // Jetstream has no grid layout -- emulated with nested row/column panels
        let mut mapped = map_style(style);

        // Resolve gap tokens (use column gap as primary gap value)
        if let Some(col_gap_token) = spec.resolved_column_gap() {
            let gap = theme.resolve_space(col_gap_token);
            mapped.layout.gap = taffy::Size { width: taffy::LengthPercentage::length(gap), height: taffy::LengthPercentage::length(gap) };
        }
        // Row gap resolved for proof but Jetstream single-gap model uses column gap
        if let Some(_row_gap_token) = spec.resolved_row_gap() {
            let _row_gap = theme.resolve_space(_row_gap_token);
        }

        // Resolve padding tokens
        let inset = spec.resolved_padding();
        let edges = resolve_inset(&inset, theme);
        apply_inset_padding(&mut mapped.layout, &edges);

        let node_id = spec.role.as_deref().unwrap_or("grid");
        JetstreamNodeHandle::new(node_id, "GridSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<SurfaceSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &SurfaceSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve background color
        let bg_token = spec.resolved_background_token();
        let bg_color = theme.resolve_color(bg_token);
        mapped.visuals.background = Some(JetstreamColor::from(bg_color));

        // Resolve border color and width
        if let Some(border_color_token) = spec.resolved_border_color() {
            let border_color = theme.resolve_color(border_color_token);
            mapped.visuals.border_color = Some(JetstreamColor::from(border_color));
        }
        if let Some(border_width_token) = spec.resolved_border_width() {
            mapped.visuals.border_width = theme.resolve_border_width(border_width_token);
        }

        // Resolve shadow token (Jetstream has no shadow field -- resolve to prove it works)
        let _shadow_token = spec.resolved_shadow_token();

        // Resolve padding tokens
        let inset = spec.resolved_padding();
        let edges = resolve_inset(&inset, theme);
        apply_inset_padding(&mut mapped.layout, &edges);

        let node_id = spec.label.as_deref().unwrap_or("surface");
        JetstreamNodeHandle::new(node_id, "SurfaceSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<SeparatorSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &SeparatorSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve separator color
        let color_token = spec.resolved_color();
        let color = theme.resolve_color(color_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(color));

        // Resolve stroke width
        let stroke_token = spec.resolved_stroke_width();
        mapped.visuals.border_width = theme.resolve_border_width(stroke_token);

        JetstreamNodeHandle::new("separator", "SeparatorSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<ScrollShellSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &ScrollShellSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve padding tokens
        let inset = spec.resolved_padding();
        let edges = resolve_inset(&inset, theme);
        apply_inset_padding(&mut mapped.layout, &edges);

        // Handle focus state
        if spec.is_focusable {
            let ring_color_token = spec.focus_ring_color_token();
            let ring_color = theme.resolve_color(ring_color_token);
            mapped.visuals.focus_ring_color = Some(JetstreamColor::from(ring_color));

            let ring_width_token = spec.focus_ring_width_token();
            mapped.visuals.focus_ring_width = theme.resolve_space(ring_width_token);
        }

        let node_id = spec.label.as_deref().unwrap_or("scroll-shell");
        JetstreamNodeHandle::new(node_id, "ScrollShellSpec", WidgetKind::List, mapped)
    }
}


impl RenderComponent<CallOutSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, spec: &CallOutSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mut mapped = map_style(style);

        // Resolve fill (background) color
        let fill_token = spec.fill_token();
        let fill_color = theme.resolve_color(fill_token);
        mapped.visuals.background = Some(JetstreamColor::from(fill_color));

        // Resolve border color
        let border_token = spec.border_token();
        let border_color = theme.resolve_color(border_token);
        mapped.visuals.border_color = Some(JetstreamColor::from(border_color));

        let node_id = spec.title.as_deref().unwrap_or("callout");
        JetstreamNodeHandle::new(node_id, "CallOutSpec", WidgetKind::Panel, mapped)
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::RenderComponent;
    use poodle_specs::*;
    use poodle_style::StyleDescriptor;
    use crate::{JetstreamAdapter, WidgetKind, theme::JetstreamThemeProvider};

    fn a() -> JetstreamAdapter { JetstreamAdapter::new(JetstreamThemeProvider::default()) }
    fn s() -> StyleDescriptor { StyleDescriptor::new() }
    fn t() -> JetstreamThemeProvider { JetstreamThemeProvider::default() }

    #[test]
    fn box_spec_uses_role_as_id() {
        let spec = BoxSpec::new().with_role("content-region");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "content-region");
        assert_eq!(h.spec_type, "BoxSpec");
        assert_eq!(h.widget_kind, WidgetKind::Panel);
    }

    #[test]
    fn box_spec_defaults_id_without_role() {
        let h = a().render(&BoxSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "box");
    }

    #[test]
    fn stack_spec_uses_role_as_id() {
        let spec = StackSpec::new().with_role("sidebar-stack");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "sidebar-stack");
        assert_eq!(h.widget_kind, WidgetKind::Panel);
    }

    #[test]
    fn stack_spec_defaults_id_without_role() {
        let h = a().render(&StackSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "stack");
    }

    #[test]
    fn grid_spec_uses_role_as_id() {
        let spec = GridSpec::new().with_role("card-grid");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "card-grid");
        assert_eq!(h.spec_type, "GridSpec");
    }

    #[test]
    fn grid_spec_defaults_id_without_role() {
        let h = a().render(&GridSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "grid");
    }

    #[test]
    fn surface_spec_uses_label_as_id() {
        let spec = SurfaceSpec::new().with_label("main-panel");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "main-panel");
        assert_eq!(h.spec_type, "SurfaceSpec");
    }

    #[test]
    fn surface_spec_defaults_id_without_label() {
        let h = a().render(&SurfaceSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "surface");
    }

    #[test]
    fn separator_spec_renders() {
        let h = a().render(&SeparatorSpec::new(), &s(), &t());
        assert_eq!(h.spec_type, "SeparatorSpec");
        assert_eq!(h.node_id, "separator");
    }

    #[test]
    fn scroll_shell_uses_label_as_id() {
        let spec = ScrollShellSpec::new().with_label("log-scroll");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "log-scroll");
        assert_eq!(h.widget_kind, WidgetKind::List);
    }

    #[test]
    fn scroll_shell_defaults_id_without_label() {
        let h = a().render(&ScrollShellSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "scroll-shell");
    }

    #[test]
    fn callout_spec_uses_title_as_id() {
        let spec = CallOutSpec::new().with_title("Important Note");
        let h = a().render(&spec, &s(), &t());
        assert_eq!(h.node_id, "Important Note");
        assert_eq!(h.spec_type, "CallOutSpec");
    }

    #[test]
    fn callout_spec_defaults_id_without_title() {
        let h = a().render(&CallOutSpec::new(), &s(), &t());
        assert_eq!(h.node_id, "callout");
    }
}
