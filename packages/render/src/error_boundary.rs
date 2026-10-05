//! ErrorBoundary — error fallback.
//!
//! Contract: `docs/contracts/components/error-boundary.md`
//! Ported from: `packages/jetstream/components/src/error_boundary.rs`.
//!
//! When the spec carries an error message, render the EmptyState fallback
//! (title + message + retry action); otherwise render the wrapped child. The
//! actual error *catching* is the host app's job — this renders the fallback.
//! The retry action is host-reset (Svelte's `reset()` clears the error and
//! re-renders children), so it needs the host callback or it paints inert.

use std::sync::Arc;

use poodle_node::Node;
use poodle_specs::{EmptyStateSpec, ErrorBoundarySpec, RemediationAction};

use crate::context::RenderContext;
use crate::empty_state::{empty_state, EmptyStateHandlers};

/// Host callbacks for the error boundary. The retry press reaches the host,
/// which clears the error and rebuilds — the native counterpart of Svelte's
/// `reset()` inside the error-state snippet.
#[derive(Default, Clone)]
pub struct ErrorBoundaryHandlers {
    pub on_retry: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Stable native instance scope, forwarded to the fallback empty state.
    pub instance_id: Option<String>,
}

/// Build an error-boundary element. `child` is the normal content shown when
/// there is no error.
pub fn error_boundary(
    spec: &ErrorBoundarySpec,
    ctx: &RenderContext<'_>,
    child: Option<Node>,
    handlers: ErrorBoundaryHandlers,
) -> Node {
    if let Some(message) = &spec.error_message {
        let on_action = handlers.on_retry.as_ref().map(|retry| {
            let retry = Arc::clone(retry);
            Arc::new(move |id: &str| {
                if id == "retry" {
                    retry();
                }
            }) as Arc<dyn Fn(&str) + Send + Sync>
        });
        return empty_state(
            &EmptyStateSpec::new(spec.title.as_str())
                .with_message(message.as_str())
                .with_actions(vec![RemediationAction::new(
                    "retry",
                    spec.retry_label.as_str(),
                )]),
            ctx,
            EmptyStateHandlers {
                on_action,
                instance_id: handlers.instance_id.clone(),
            },
        );
    }
    child.unwrap_or_else(Node::container)
}
