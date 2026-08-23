//! ErrorBoundary — error fallback.
//!
//! Contract: `docs/contracts/components/error-boundary.md`
//! Ported from: `packages/jetstream/components/src/error_boundary.rs`.
//!
//! When the spec carries an error message, render the EmptyState fallback
//! (title + message + retry action); otherwise render the wrapped child. The
//! actual error *catching* is the host app's job — this renders the fallback.

use poodle_node::Node;
use poodle_specs::{EmptyStateSpec, ErrorBoundarySpec, RemediationAction};

use crate::context::RenderContext;
use crate::empty_state::empty_state;

/// Build an error-boundary element. `child` is the normal content shown when
/// there is no error.
pub fn error_boundary(
    spec: &ErrorBoundarySpec,
    ctx: &RenderContext<'_>,
    child: Option<Node>,
) -> Node {
    if let Some(message) = &spec.error_message {
        return empty_state(
            &EmptyStateSpec::new(spec.title.as_str())
                .with_message(message.as_str())
                .with_actions(vec![RemediationAction::new(
                    "retry",
                    spec.retry_label.as_str(),
                )]),
            ctx,
        );
    }
    child.unwrap_or_else(Node::container)
}
