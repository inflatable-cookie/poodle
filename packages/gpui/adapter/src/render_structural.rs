//! RenderComponent implementations for structural and layout primitives.
//!
//! Implements the adapter trait for:
//! - BoxSpec, StackSpec, GridSpec, SurfaceSpec
//! - SeparatorSpec, ScrollShellSpec
//! - BannerSpec, CallOutSpec
//!
//! NOTE: `gpui_style` is built and mutated as proof of token resolution but is
//! not yet wired into the returned `GpuiElementHandle`. Suppressed until wired up.
#![allow(unused_variables, unused_assignments)]

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    BannerSpec, BoxSpec, CallOutSpec, GridSpec, ScrollShellSpec, SeparatorSpec, StackSpec,
    SurfaceSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

fn resolve_inset_padding(
    theme: &dyn ThemeProvider,
    inset: &poodle_specs::Inset,
) -> crate::GpuiEdges {
    let h = inset
        .horizontal
        .map(|t| theme.resolve_space(t))
        .unwrap_or(0.0);
    let v = inset
        .vertical
        .map(|t| theme.resolve_space(t))
        .unwrap_or(0.0);
    crate::GpuiEdges {
        top: v,
        right: h,
        bottom: v,
        left: h,
    }
}

impl RenderComponent<BoxSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &BoxSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);
        gpui_style.padding = resolve_inset_padding(theme, &spec.resolved_padding());
        GpuiElementHandle::new(
            format!("box-{}", spec.role.as_deref().unwrap_or("anonymous")),
            "BoxSpec",
        )
    }
}

impl RenderComponent<StackSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &StackSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);
        if let Some(gap_token) = spec.resolved_gap() {
            gpui_style.gap = theme.resolve_space(gap_token);
        }
        gpui_style.padding = resolve_inset_padding(theme, &spec.resolved_padding());
        GpuiElementHandle::new(
            format!("stack-{}", spec.role.as_deref().unwrap_or("anonymous")),
            "StackSpec",
        )
    }
}

impl RenderComponent<GridSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &GridSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);
        if let Some(col_gap_token) = spec.resolved_column_gap() {
            gpui_style.gap = theme.resolve_space(col_gap_token);
        }
        // Row gap applied separately so flex-wrap grids get correct vertical spacing
        if let Some(row_gap_token) = spec.resolved_row_gap() {
            gpui_style.row_gap = theme.resolve_space(row_gap_token);
        }
        gpui_style.padding = resolve_inset_padding(theme, &spec.resolved_padding());
        GpuiElementHandle::new(
            format!("grid-{}", spec.role.as_deref().unwrap_or("anonymous")),
            "GridSpec",
        )
    }
}

impl RenderComponent<SurfaceSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &SurfaceSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        // Resolve surface-specific tokens
        let bg = theme.resolve_color(spec.resolved_background_token());
        gpui_style.background = Some(bg.into());

        if let Some(border_color_token) = spec.resolved_border_color() {
            let border_color = theme.resolve_color(border_color_token);
            gpui_style.border_color = Some(border_color.into());
            if let Some(border_width_token) = spec.resolved_border_width() {
                gpui_style.border_width = theme.resolve_border_width(border_width_token);
            }
        }

        GpuiElementHandle::new(
            format!("surface-{}", spec.label.as_deref().unwrap_or("anonymous")),
            "SurfaceSpec",
        )
    }
}

impl RenderComponent<SeparatorSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &SeparatorSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        let color = theme.resolve_color(spec.resolved_color());
        let width = theme.resolve_border_width(spec.resolved_stroke_width());
        gpui_style.border_color = Some(color.into());
        gpui_style.border_width = width;

        GpuiElementHandle::new("separator", "SeparatorSpec")
    }
}

impl RenderComponent<ScrollShellSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &ScrollShellSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        if spec.is_focusable {
            let ring_color = theme.resolve_color(spec.focus_ring_color_token());
            let ring_width = theme.resolve_border_width(spec.focus_ring_width_token());
            gpui_style.focus_ring_color = Some(ring_color.into());
            gpui_style.focus_ring_width = ring_width;
        }

        GpuiElementHandle::new(
            format!(
                "scroll-shell-{}",
                spec.label.as_deref().unwrap_or("anonymous")
            ),
            "ScrollShellSpec",
        )
    }
}

impl RenderComponent<BannerSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &BannerSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        let fill = theme.resolve_color(spec.fill_token());
        let border = theme.resolve_color(spec.border_token());
        gpui_style.background = Some(fill.into());
        gpui_style.border_color = Some(border.into());

        GpuiElementHandle::new(
            format!("banner-{}", spec.title.as_deref().unwrap_or("anonymous")),
            "BannerSpec",
        )
    }
}

impl RenderComponent<CallOutSpec> for GpuiAdapter {
    type Target = GpuiTarget;

    fn render(
        &self,
        spec: &CallOutSpec,
        style: &StyleDescriptor,
        theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let mut gpui_style = map_style(style);

        let fill = theme.resolve_color(spec.fill_token());
        let border = theme.resolve_color(spec.border_token());
        gpui_style.background = Some(fill.into());
        gpui_style.border_color = Some(border.into());

        GpuiElementHandle::new(
            format!("callout-{}", spec.title.as_deref().unwrap_or("anonymous")),
            "CallOutSpec",
        )
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::RenderComponent;
    use poodle_specs::{
        BannerSpec, BoxSpec, CallOutSpec, GridSpec, ScrollShellSpec, SeparatorSpec, StackSpec,
        SurfaceSpec,
    };
    use poodle_style::StyleDescriptor;

    use crate::theme::GpuiThemeProvider;
    use crate::GpuiAdapter;

    fn adapter() -> GpuiAdapter {
        GpuiAdapter::new(GpuiThemeProvider::default())
    }

    fn style() -> StyleDescriptor {
        StyleDescriptor::new()
    }

    fn theme() -> GpuiThemeProvider {
        GpuiThemeProvider::default()
    }

    #[test]
    fn render_box_produces_handle() {
        let a = adapter();
        let handle = a.render(&BoxSpec::new(), &style(), &theme());
        assert_eq!(handle.element_id, "box-anonymous");
        assert_eq!(handle.spec_type, "BoxSpec");
    }

    #[test]
    fn render_box_with_role() {
        let a = adapter();
        let spec = BoxSpec::new().with_role("sidebar");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "box-sidebar");
    }

    #[test]
    fn render_stack_produces_handle() {
        let a = adapter();
        let spec = StackSpec::new().with_role("content");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "stack-content");
    }

    #[test]
    fn render_stack_anonymous_fallback() {
        let a = adapter();
        let handle = a.render(&StackSpec::new(), &style(), &theme());
        assert_eq!(handle.element_id, "stack-anonymous");
    }

    #[test]
    fn render_grid_produces_handle() {
        let a = adapter();
        let handle = a.render(&GridSpec::new(), &style(), &theme());
        assert_eq!(handle.element_id, "grid-anonymous");
        assert_eq!(handle.spec_type, "GridSpec");
    }

    #[test]
    fn render_grid_with_role() {
        let a = adapter();
        let spec = GridSpec::new().with_role("gallery");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "grid-gallery");
    }

    #[test]
    fn render_surface_resolves_background() {
        let a = adapter();
        let spec = SurfaceSpec::new().with_label("main");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "surface-main");
    }

    #[test]
    fn render_separator_produces_handle() {
        let a = adapter();
        let handle = a.render(&SeparatorSpec::new(), &style(), &theme());
        assert_eq!(handle.spec_type, "SeparatorSpec");
    }

    #[test]
    fn render_scroll_shell_with_focus_ring() {
        let a = adapter();
        let spec = ScrollShellSpec::new()
            .with_focusable(true)
            .with_label("sidebar");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "scroll-shell-sidebar");
    }

    #[test]
    fn render_banner_resolves_tone_tokens() {
        let a = adapter();
        let spec = BannerSpec::new().with_title("Warning");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "banner-Warning");
    }

    #[test]
    fn render_callout_resolves_tone_tokens() {
        let a = adapter();
        let spec = CallOutSpec::new().with_title("Note");
        let handle = a.render(&spec, &style(), &theme());
        assert_eq!(handle.element_id, "callout-Note");
    }
}
