//! IconProvider — a child-passthrough context boundary.
//!
//! Contract: `docs/contracts/components/icon-provider.md`
//!
//! GPUI resolves icons through one shared registry (contract §10), so there
//! is nothing for a Node recipe to emit. This returns the child unchanged.

use poodle_node::Node;
use poodle_specs::IconProviderSpec;

use crate::context::RenderContext;

/// Every public component renderer receives the construction context
/// (architecture 010); the icon boundary has no presentation-dependent
/// output, so the context is accepted and unused.
pub fn icon_provider(
    _spec: &IconProviderSpec,
    _ctx: &RenderContext<'_>,
    child: Option<Node>,
) -> Node {
    child.unwrap_or_else(Node::container)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passthrough_returns_the_child_unchanged() {
        let theme = poodle_jetstream::JetstreamThemeProvider::from_theme(
            &poodle_tokens::themes::ECLIPSE,
        );
        let ctx = RenderContext::new(&theme);
        let child = Node::text("search");
        let out = icon_provider(&IconProviderSpec::new(), &ctx, Some(child));
        assert_eq!(out.texts(), vec!["search"]);
        assert!(icon_provider(&IconProviderSpec::new(), &ctx, None)
            .children
            .is_empty());
    }
}
