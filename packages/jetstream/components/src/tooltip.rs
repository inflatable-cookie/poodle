//! Tooltip — Jetstream tooltip backed by TooltipSpec.
//!
//! Jetstream has no hover. Tooltip content is not rendered visually.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::TooltipSpec;

pub fn js_tooltip(_spec: &TooltipSpec, _theme: &JetstreamThemeProvider) -> JsEl {
    // Tooltip is invisible in Jetstream — no hover trigger available
    ui_element::div()
}
