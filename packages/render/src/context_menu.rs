//! ContextMenu — the right-click wrapper around the shared Menu surface.
//!
//! Contract: `docs/contracts/components/context-menu.md`
//! The host owns the trigger and anchor point. The component recipe is the
//! same menu panel with ContextMenu's size and density forwarded.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::ContextMenuSpec;

use crate::menu::menu;

pub fn context_menu(
    spec: &ContextMenuSpec,
    theme: &dyn ThemeProvider,
    on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let mut menu_spec = spec.menu.clone();
    menu_spec.size = spec.size;
    menu_spec.size_role = spec.size_role;
    menu_spec.density = spec.density;
    menu(&menu_spec, theme, on_action)
}
