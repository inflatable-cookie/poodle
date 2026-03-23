//! ContextMenu — real GPUI component backed by ContextMenuSpec.

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::{ContextMenuSpec, MenuSpec};

use super::menu::Menu;

/// A real GPUI context menu component backed by `ContextMenuSpec`.
///
/// Wraps Menu with additional positioning context for right-click menus.
/// Supports an optional trigger element (the right-click target) and
/// fixed positioning at the anchor point.
pub struct ContextMenu {
    spec: ContextMenuSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    selected_value: Option<String>,
    /// The element that, when right-clicked, opens the context menu.
    trigger: Option<AnyElement>,
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ContextMenu {
    type Target = ContextMenuSpec;
    fn deref(&self) -> &ContextMenuSpec { &self.spec }
}

impl ContextMenu {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ContextMenuSpec::default(), theme: theme.clone(), id_prefix: String::new(), selected_value: None, trigger: None, on_select: None, on_close: None }
    }

    pub fn from_spec(spec: ContextMenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "flint-ctx-menu".to_string(),
            selected_value: None,
            trigger: None,
            on_select: None,
            on_close: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn menu(mut self, v: MenuSpec) -> Self { self.spec.menu = v; self }
    pub fn anchor_point(mut self, v: (i32, i32)) -> Self { self.spec.anchor_point = Some(v); self }

    /// Set the trigger element (the right-click target that opens the menu).
    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn with_selected(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }

    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }
}

impl IntoElement for ContextMenu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        // Delegate to Menu with the inner menu spec
        let mut menu = Menu::from_spec(self.spec.menu.clone(), &self.theme)
            .with_id(self.id_prefix);

        if let Some(selected) = self.selected_value {
            menu = menu.with_selected(selected);
        }
        if let Some(handler) = self.on_select {
            menu = menu.on_select(handler);
        }
        if let Some(handler) = self.on_close {
            menu = menu.on_close(handler);
        }

        // Apply fixed positioning at anchor point when provided
        let menu_el = if let Some((x, y)) = self.spec.anchor_point {
            div()
                .absolute()
                .left(px(x as f32))
                .top(px(y as f32))
                .child(menu)
                .into_any_element()
        } else {
            menu.into_any_element()
        };

        // If a trigger is provided, wrap both trigger and menu overlay in
        // a relatively-positioned container so the menu anchors correctly.
        if let Some(trigger) = self.trigger {
            div()
                .relative()
                .child(trigger)
                .child(menu_el)
                .into_any_element()
        } else {
            menu_el
        }
    }
}
