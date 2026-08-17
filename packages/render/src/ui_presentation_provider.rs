//! UiPresentationProvider — a child-passthrough presentation boundary.
//!
//! Contract: `docs/contracts/components/ui-presentation-provider.md`
//!
//! GPUI has no CSS custom properties. Descendants consume size and density
//! from their own specs (contract §10), so a Node wrapper that invented a
//! cascade would be chrome the contract does not describe. This returns the
//! child unchanged and records the declared scope on `roles` so the absence
//! of a cascade is visible, not silent.

use poodle_node::Node;
use poodle_specs::UiPresentationProviderSpec;

pub fn ui_presentation_provider(
    spec: &UiPresentationProviderSpec,
    child: Option<Node>,
) -> Node {
    let mut root = child.unwrap_or_else(Node::container);
    root.roles
        .insert("density".to_owned(), format!("{:?}", spec.density).to_ascii_lowercase());
    root.roles
        .insert("size_scale".to_owned(), format!("{:?}", spec.size_scale).to_ascii_lowercase());
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{ControlDensity, ControlSize};

    #[test]
    fn passthrough_keeps_the_child_and_declares_the_scope() {
        let child = Node::text("inner");
        let out = ui_presentation_provider(
            &UiPresentationProviderSpec::new()
                .with_density(ControlDensity::Compact)
                .with_size_scale(ControlSize::Sm),
            Some(child),
        );
        assert_eq!(out.texts(), vec!["inner"]);
        assert_eq!(out.roles.get("density").map(String::as_str), Some("compact"));
        assert_eq!(out.roles.get("size_scale").map(String::as_str), Some("sm"));
    }
}
