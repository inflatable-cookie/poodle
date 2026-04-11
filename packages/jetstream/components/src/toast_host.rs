//! ToastHost — Jetstream positioned toast container backed by ToastHostSpec.
//!
//! Wraps a ToastStack and positions it at the chosen screen corner.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::{ToastHostPlacement, ToastHostSpec, ToastStackSpec};

use crate::presentation::rem_to_px;
use crate::toast_stack::js_toast_stack;

pub fn js_toast_host(
    spec: &ToastHostSpec,
    theme: &JetstreamThemeProvider,
    stack_spec: &ToastStackSpec,
) -> JsEl {
    // Empty toasts — render nothing
    if stack_spec.toasts.is_empty() {
        return ui_element::div();
    }

    let padding = rem_to_px(1.0);
    let width = rem_to_px(28.0);

    let mut container = ui_element::div()
        .w(width);

    // Position at chosen corner using absolute positioning
    match spec.placement {
        ToastHostPlacement::BottomEnd => {
            container = container
                .absolute()
                .right(padding).bottom(padding);
        }
        ToastHostPlacement::BottomStart => {
            container = container
                .absolute()
                .left(padding).bottom(padding);
        }
        ToastHostPlacement::TopEnd => {
            container = container
                .absolute()
                .right(padding).top(padding);
        }
        ToastHostPlacement::TopStart => {
            container = container
                .absolute()
                .left(padding).top(padding);
        }
    }

    // Render inner toast stack
    let inner = js_toast_stack(stack_spec, theme);
    container = container.child(inner);

    container
}
