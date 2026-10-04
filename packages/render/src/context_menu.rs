//! ContextMenu — the right-click wrapper around the shared Menu surface.
//!
//! Contract: `docs/contracts/components/context-menu.md`
//! The host owns the trigger and anchor point. The component recipe is the
//! same menu panel with ContextMenu's size and density forwarded.

use std::sync::Arc;

use poodle_node::{DismissReason, Node};
use poodle_specs::ContextMenuSpec;

use crate::context::RenderContext;
use crate::menu::menu;

/// The layer id the open panel registers on the backend dismiss stack.
/// Containment is the surface only: Svelte tests outside dismissal against
/// the overlay, so a press on the host-owned trigger zone still dismisses.
/// Single-flight like the shared `menu-item:*` ids; scope when a host mounts
/// two live menus at once.
pub const CONTEXT_MENU_LAYER_ID: &str = "context-menu-layer";

/// Host-owned interaction intent. The backend turns dismissal into real
/// Escape and outside-pointer listeners; the host closes on the reason.
#[derive(Clone, Default)]
pub struct ContextMenuHandlers {
    /// Item activation with the committed value.
    pub on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Document-level dismissal (escape / outside). Present registers the
    /// surface on the dismiss stack; absent keeps the previous behavior
    /// (the host owns dismissal entirely).
    pub on_dismiss: Option<Arc<dyn Fn(DismissReason) + Send + Sync>>,
}

pub fn context_menu(
    spec: &ContextMenuSpec,
    ctx: &RenderContext<'_>,
    handlers: ContextMenuHandlers,
) -> Node {
    let mut menu_spec = spec.menu.clone();
    menu_spec.size = spec.size;
    menu_spec.size_role = spec.size_role;
    menu_spec.density = spec.density;
    // ContextMenu's own `dismissOnOutsideInteract` wins over the composed
    // menu's (the alert_dialog pattern: the renderer resolves the composed
    // spec's dismissal from its own spec state).
    menu_spec.dismiss_on_outside_interact = spec.dismiss_on_outside_interact;
    let mut panel = menu(&menu_spec, ctx, handlers.on_action);
    if let Some(on_dismiss) = handlers.on_dismiss {
        panel.interaction.dismiss_layer = Some(CONTEXT_MENU_LAYER_ID.to_string());
        panel.interaction.on_dismiss = Some(on_dismiss);
    }
    panel
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn refusal_forwarded_into_composed_menu_surface() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        // Default `true`: the composed menu surface stays marker-free.
        let node = context_menu(&ContextMenuSpec::default(), &ctx, ContextMenuHandlers::default());
        assert!(node.interaction.on_activate.is_none());

        // ContextMenu's own refusal wins over the composed MenuSpec default
        // and reaches the rendered surface.
        let refusing = ContextMenuSpec::default().with_dismiss_on_outside_interact(false);
        let node = context_menu(&refusing, &ctx, ContextMenuHandlers::default());
        assert!(node.interaction.on_activate.is_some());
    }

    #[test]
    fn dismiss_handler_registers_the_surface_layer() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        // No handler: no layer, previous behavior.
        let bare = context_menu(
            &ContextMenuSpec::default(),
            &ctx,
            ContextMenuHandlers::default(),
        );
        assert!(bare.interaction.dismiss_layer.is_none());
        assert!(bare.interaction.on_dismiss.is_none());

        // Handler: the surface joins the dismiss stack under its layer id.
        let layered = context_menu(
            &ContextMenuSpec::default(),
            &ctx,
            ContextMenuHandlers {
                on_dismiss: Some(Arc::new(|_| {})),
                ..ContextMenuHandlers::default()
            },
        );
        assert_eq!(
            layered.interaction.dismiss_layer.as_deref(),
            Some(CONTEXT_MENU_LAYER_ID)
        );
        assert!(layered.interaction.on_dismiss.is_some());
    }
}
