//! PugContextMenu — real GPUI component backed by ContextMenuSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::ContextMenuSpec;

use crate::menu::PugMenu;

/// A real GPUI context menu component backed by `ContextMenuSpec`.
///
/// Wraps PugMenu with additional positioning context for right-click menus.
pub struct PugContextMenu {
    spec: ContextMenuSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    selected_value: Option<String>,
}

impl PugContextMenu {
    pub fn new(spec: ContextMenuSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-ctx-menu".to_string(),
            selected_value: None,
        }
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn with_selected(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }
}

impl IntoElement for PugContextMenu {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        // Delegate to PugMenu with the inner menu spec
        let mut menu = PugMenu::new(self.spec.menu, &self.theme)
            .with_id(self.id_prefix);

        if let Some(selected) = self.selected_value {
            menu = menu.with_selected(selected);
        }

        menu.into_any_element()
    }
}
