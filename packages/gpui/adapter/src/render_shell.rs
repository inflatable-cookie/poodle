//! RenderComponent implementations for shell and layout composites.
//!
//! Replaces the retired `render_workstation` module. The workstation tier was a
//! parallel spec crate (`poodle-workstation`) that duplicated six specs
//! `poodle-specs` already owned and added seven more with no component, no
//! contract and no Svelte counterpart. The six that are real live here, sourced
//! from `poodle-specs` like every other module in this crate.
//!
//! NOTE: as elsewhere in the adapter, `map_style` is called as proof of token
//! resolution but is not yet wired into the returned handle.

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    ActionDiscoveryPanelSpec, AppHeaderSpec, CommandPaletteSpec, DockRegionSpec,
    ShellStatusBarSpec, SplitViewSpec,
};
use poodle_style::StyleDescriptor;

use crate::style_map::map_style;
use crate::{GpuiAdapter, GpuiElementHandle, GpuiTarget};

impl RenderComponent<ActionDiscoveryPanelSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &ActionDiscoveryPanelSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("action-discovery-panel", "ActionDiscoveryPanelSpec")
    }
}

impl RenderComponent<AppHeaderSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &AppHeaderSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("app-header", "AppHeaderSpec")
    }
}

impl RenderComponent<CommandPaletteSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &CommandPaletteSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("command-palette", "CommandPaletteSpec")
    }
}

impl RenderComponent<DockRegionSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &DockRegionSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("dock-region", "DockRegionSpec")
    }
}

impl RenderComponent<ShellStatusBarSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &ShellStatusBarSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("shell-status-bar", "ShellStatusBarSpec")
    }
}

impl RenderComponent<SplitViewSpec> for GpuiAdapter {
    type Target = GpuiTarget;
    fn render(
        &self,
        _spec: &SplitViewSpec,
        style: &StyleDescriptor,
        _theme: &dyn ThemeProvider,
    ) -> GpuiElementHandle {
        let _s = map_style(style);
        GpuiElementHandle::new("split-view", "SplitViewSpec")
    }
}
