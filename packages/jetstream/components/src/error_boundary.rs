//! JsErrorBoundary — error fallback backed by ErrorBoundarySpec.
//!
//! Contract: `docs/contracts/components/error-boundary.md`
//! Reference: GPUI `composites/error_boundary.rs`.
//!
//! When the spec carries an error message, render the EmptyState fallback
//! (title + message + retry action); otherwise render the wrapped child. The
//! actual error *catching* is the host app's job — this renders the fallback UI.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{EmptyStateSpec, ErrorBoundarySpec, RemediationAction};

use crate::empty_state::js_empty_state;

/// Build an error-boundary element. `child` is the normal content shown when
/// there is no error.
pub fn js_error_boundary(
    spec: &ErrorBoundarySpec,
    theme: &JetstreamThemeProvider,
    child: Option<JsEl>,
) -> JsEl {
    if let Some(message) = &spec.error_message {
        return js_empty_state(
            &EmptyStateSpec::new(spec.title.as_str())
                .with_message(message.as_str())
                .with_actions(vec![RemediationAction::new("retry", spec.retry_label.as_str())]),
            theme,
        );
    }
    child.unwrap_or_else(ui_element::div)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn error_renders_fallback_message() {
        let el = js_error_boundary(
            &ErrorBoundarySpec::new().with_error_message("Boom"),
            &theme(),
            None,
        );
        let tree = probe(&el, 320.0, 220.0);
        assert!(
            tree.texts().iter().any(|t| t.contains("Boom")),
            "error message missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn no_error_renders_child() {
        let el = js_error_boundary(
            &ErrorBoundarySpec::new(),
            &theme(),
            Some(ui_element::label("content")),
        );
        let tree = probe(&el, 320.0, 220.0);
        assert!(tree.has_text("content"), "child not rendered: {:?}", tree.texts());
    }
}
