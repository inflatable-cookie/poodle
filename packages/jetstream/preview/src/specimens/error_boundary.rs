//! ErrorBoundary specimen — error fallback (icon + title + message + retry).
//!
//! Mirrors the GPUI specimen group set: normal children (boundary passes the
//! child through with no chrome), caught render error (EmptyState fallback with
//! custom title + retry), and default fallback (default title/retry from spec).
//! Composes the real `js_error_boundary` (which wraps the real `js_empty_state`
//! on error); the normal-children child is a real `js_surface` + `js_text` — no
//! fakes. Error is props-driven (`with_error_message`): Jetstream cannot catch
//! render panics, so the boundary renders the fallback UI rather than catching.

use crate::compat::js_error_boundary;
use crate::compat::js_surface;
use crate::compat::js_text;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ErrorBoundarySpec, PaddingScale, SurfaceBorder, SurfaceSpec, TextSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .max_w(560.0)
        // --- Normal children: boundary passes child through with no chrome ---
        .child(group(
            "Normal children",
            secondary,
            js_error_boundary(
                &ErrorBoundarySpec::new(),
                theme,
                Some(js_surface(
                    &SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Md),
                    theme,
                    vec![js_text(
                        &TextSpec::new("Stable child content renders without boundary chrome."),
                        theme,
                    )],
                )),
            ),
        ))
        // --- Caught render error: EmptyState fallback with custom title + retry ---
        .child(group(
            "Caught render error",
            secondary,
            js_error_boundary(
                &ErrorBoundarySpec::new()
                    .with_title("Preview failed")
                    .with_retry_label("Reset boundary")
                    .with_error_message("Preview child failed during render."),
                theme,
                None,
            ),
        ))
        // --- Default fallback: default title + retry label from the spec ---
        .child(group(
            "Default fallback",
            secondary,
            js_error_boundary(
                &ErrorBoundarySpec::new()
                    .with_error_message("An unexpected error occurred while rendering this view."),
                theme,
                None,
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
