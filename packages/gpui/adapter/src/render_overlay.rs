//! RenderComponent implementations for overlay, disclosure, navigation, and menu primitives.
//!
//! g07.005: Accordion, Collapsible, Dialog, Drawer, Popover, Tooltip, HoverCard,
//! Menu, ContextMenu, Tabs, TabStrip, NavigationMenu, Menubar

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_primitives::{
    AccordionSpec, CollapsibleSpec, ContextMenuSpec, DialogSpec, DrawerSpec, HoverCardSpec,
    MenuSpec, MenubarSpec, NavigationMenuSpec, PopoverSpec, TabStripSpec, TabsSpec, TooltipSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<AccordionSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &AccordionSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("accordion", "AccordionSpec")
    }
}

impl RenderComponent<CollapsibleSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &CollapsibleSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("collapsible", "CollapsibleSpec")
    }
}

impl RenderComponent<DialogSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, spec: &DialogSpec, style: &StyleDescriptor, theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let mut s = map_style(style);
        let backdrop = theme.resolve_color(spec.backdrop_fill_token());
        s.background = Some(backdrop.into());
        GpuiElementHandle::new(
            format!("dialog-{}", spec.title.as_deref().unwrap_or("anonymous")),
            "DialogSpec",
        )
    }
}

impl RenderComponent<DrawerSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &DrawerSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("drawer", "DrawerSpec")
    }
}

impl RenderComponent<PopoverSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &PopoverSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("popover", "PopoverSpec")
    }
}

impl RenderComponent<TooltipSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &TooltipSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("tooltip", "TooltipSpec")
    }
}

impl RenderComponent<HoverCardSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &HoverCardSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("hover-card", "HoverCardSpec")
    }
}

impl RenderComponent<MenuSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &MenuSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("menu", "MenuSpec")
    }
}

impl RenderComponent<ContextMenuSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &ContextMenuSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("context-menu", "ContextMenuSpec")
    }
}

impl RenderComponent<TabsSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &TabsSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("tabs", "TabsSpec")
    }
}

impl RenderComponent<TabStripSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &TabStripSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("tab-strip", "TabStripSpec")
    }
}

impl RenderComponent<NavigationMenuSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &NavigationMenuSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("navigation-menu", "NavigationMenuSpec")
    }
}

impl RenderComponent<MenubarSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(&self, _spec: &MenubarSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("menubar", "MenubarSpec")
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::RenderComponent;
    use poodle_primitives::*;
    use poodle_style::StyleDescriptor;
    use crate::{GpuiAdapter, theme::GpuiThemeProvider};

    fn a() -> GpuiAdapter { GpuiAdapter::new(GpuiThemeProvider::default()) }
    fn s() -> StyleDescriptor { StyleDescriptor::new() }
    fn t() -> GpuiThemeProvider { GpuiThemeProvider::default() }

    #[test] fn accordion() { assert_eq!(a().render(&AccordionSpec::new(vec![]), &s(), &t()).spec_type, "AccordionSpec"); }
    #[test] fn collapsible() { assert_eq!(a().render(&CollapsibleSpec::new(), &s(), &t()).spec_type, "CollapsibleSpec"); }
    #[test] fn dialog() { assert_eq!(a().render(&DialogSpec::new(), &s(), &t()).spec_type, "DialogSpec"); }
    #[test] fn drawer() { assert_eq!(a().render(&DrawerSpec::new(), &s(), &t()).spec_type, "DrawerSpec"); }
    #[test] fn popover() { assert_eq!(a().render(&PopoverSpec::new(), &s(), &t()).spec_type, "PopoverSpec"); }
    #[test] fn tooltip() { assert_eq!(a().render(&TooltipSpec::new(), &s(), &t()).spec_type, "TooltipSpec"); }
    #[test] fn hover_card() { assert_eq!(a().render(&HoverCardSpec::new(), &s(), &t()).spec_type, "HoverCardSpec"); }
    #[test] fn menu() { assert_eq!(a().render(&MenuSpec::new(vec![]), &s(), &t()).spec_type, "MenuSpec"); }
    #[test] fn context_menu() { assert_eq!(a().render(&ContextMenuSpec::new(vec![]), &s(), &t()).spec_type, "ContextMenuSpec"); }
    #[test] fn tabs() { assert_eq!(a().render(&TabsSpec::new(vec![]), &s(), &t()).spec_type, "TabsSpec"); }
    #[test] fn tab_strip() { assert_eq!(a().render(&TabStripSpec::new(vec![]), &s(), &t()).spec_type, "TabStripSpec"); }
    #[test] fn navigation_menu() { assert_eq!(a().render(&NavigationMenuSpec::new(vec![]), &s(), &t()).spec_type, "NavigationMenuSpec"); }
    #[test] fn menubar() { assert_eq!(a().render(&MenubarSpec::new(vec![]), &s(), &t()).spec_type, "MenubarSpec"); }
}
