//! RenderComponent implementations for overlay primitives.
//!
//! g08.008: DialogSpec, DrawerSpec, PopoverSpec, MenuSpec, TooltipSpec,
//! TabsSpec, AccordionSpec, CollapsibleSpec, HoverCardSpec, ContextMenuSpec,
//! TabStripSpec, NavigationMenuSpec, MenubarSpec
//!
//! In Jetstream, overlays map to ScreenStack entries (Dialog, Drawer, Popover)
//! or to Panel containers with visibility toggling (Accordion, Tabs).

use flint_adapter::{RenderComponent, ThemeProvider};
use flint_primitives::{
    AccordionSpec, CollapsibleSpec, ContextMenuSpec, DialogSpec, DrawerSpec, HoverCardSpec,
    MenuSpec, MenubarSpec, NavigationMenuSpec, PopoverSpec, TabStripSpec, TabsSpec, TooltipSpec,
};
use flint_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{JetstreamAdapter, JetstreamNodeHandle, JetstreamTarget, WidgetKind};

impl RenderComponent<DialogSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &DialogSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("dialog", "DialogSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<DrawerSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &DrawerSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("drawer", "DrawerSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<PopoverSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &PopoverSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("popover", "PopoverSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<MenuSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &MenuSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("menu", "MenuSpec", WidgetKind::List, mapped)
    }
}

impl RenderComponent<TooltipSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &TooltipSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("tooltip", "TooltipSpec", WidgetKind::Label, mapped)
    }
}

impl RenderComponent<TabsSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &TabsSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("tabs", "TabsSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<AccordionSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &AccordionSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("accordion", "AccordionSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<CollapsibleSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &CollapsibleSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("collapsible", "CollapsibleSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<HoverCardSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &HoverCardSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("hover-card", "HoverCardSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<ContextMenuSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &ContextMenuSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("context-menu", "ContextMenuSpec", WidgetKind::List, mapped)
    }
}

impl RenderComponent<TabStripSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &TabStripSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("tab-strip", "TabStripSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<NavigationMenuSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &NavigationMenuSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("navigation-menu", "NavigationMenuSpec", WidgetKind::Panel, mapped)
    }
}

impl RenderComponent<MenubarSpec> for JetstreamAdapter {
    type Target = JetstreamTarget;
    fn render(&self, _spec: &MenubarSpec, style: &StyleDescriptor, _theme: &dyn ThemeProvider) -> JetstreamNodeHandle {
        let mapped = map_style(style);
        JetstreamNodeHandle::new("menubar", "MenubarSpec", WidgetKind::Panel, mapped)
    }
}

#[cfg(test)]
mod tests {
    use flint_adapter::RenderComponent;
    use flint_primitives::*;
    use flint_style::StyleDescriptor;
    use crate::{JetstreamAdapter, WidgetKind, theme::JetstreamThemeProvider};

    fn a() -> JetstreamAdapter { JetstreamAdapter::new(JetstreamThemeProvider::default()) }
    fn s() -> StyleDescriptor { StyleDescriptor::new() }
    fn t() -> JetstreamThemeProvider { JetstreamThemeProvider::default() }

    #[test] fn dialog() { assert_eq!(a().render(&DialogSpec::new(), &s(), &t()).spec_type, "DialogSpec"); }
    #[test] fn drawer() { assert_eq!(a().render(&DrawerSpec::new(), &s(), &t()).spec_type, "DrawerSpec"); }
    #[test] fn popover() { assert_eq!(a().render(&PopoverSpec::new(), &s(), &t()).spec_type, "PopoverSpec"); }
    #[test] fn menu() { assert_eq!(a().render(&MenuSpec::new(vec![]), &s(), &t()).widget_kind, WidgetKind::List); }
    #[test] fn tooltip() { assert_eq!(a().render(&TooltipSpec::new(), &s(), &t()).widget_kind, WidgetKind::Label); }
    #[test] fn tabs() { assert_eq!(a().render(&TabsSpec::new(vec![]), &s(), &t()).spec_type, "TabsSpec"); }
    #[test] fn accordion() { assert_eq!(a().render(&AccordionSpec::new(vec![]), &s(), &t()).spec_type, "AccordionSpec"); }
    #[test] fn collapsible() { assert_eq!(a().render(&CollapsibleSpec::new(), &s(), &t()).spec_type, "CollapsibleSpec"); }
    #[test] fn hover_card() { assert_eq!(a().render(&HoverCardSpec::new(), &s(), &t()).spec_type, "HoverCardSpec"); }
    #[test] fn context_menu() { assert_eq!(a().render(&ContextMenuSpec::new(vec![]), &s(), &t()).widget_kind, WidgetKind::List); }
    #[test] fn tab_strip() { assert_eq!(a().render(&TabStripSpec::new(vec![]), &s(), &t()).spec_type, "TabStripSpec"); }
    #[test] fn navigation_menu() { assert_eq!(a().render(&NavigationMenuSpec::new(vec![]), &s(), &t()).spec_type, "NavigationMenuSpec"); }
    #[test] fn menubar() { assert_eq!(a().render(&MenubarSpec::new(vec![]), &s(), &t()).spec_type, "MenubarSpec"); }
}
