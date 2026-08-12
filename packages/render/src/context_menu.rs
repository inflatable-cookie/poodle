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
    // ContextMenu's own `dismissOnOutsideInteract` wins over the composed
    // menu's (the alert_dialog pattern: the renderer resolves the composed
    // spec's dismissal from its own spec state).
    menu_spec.dismiss_on_outside_interact = spec.dismiss_on_outside_interact;
    menu(&menu_spec, theme, on_action)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn refusal_forwarded_into_composed_menu_surface() {
        // Default `true`: the composed menu surface stays marker-free.
        let node = context_menu(&ContextMenuSpec::default(), &theme(), None);
        assert!(node.interaction.on_activate.is_none());

        // ContextMenu's own refusal wins over the composed MenuSpec default
        // and reaches the rendered surface.
        let refusing = ContextMenuSpec::default().with_dismiss_on_outside_interact(false);
        let node = context_menu(&refusing, &theme(), None);
        assert!(node.interaction.on_activate.is_some());
    }
}
