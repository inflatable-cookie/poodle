//! Spacer — flexible space filler for flex layouts.
//!
//! Contract: `docs/contracts/components/spacer.md`
//! Reference: `packages/gpui/components/src/primitives/spacer.rs`

use jetstream_ui::ui_element::{self, JsEl};
use poodle_specs::SpacerSpec;

pub fn js_spacer(spec: &SpacerSpec) -> JsEl {
    let mut el = ui_element::div();
    if spec.grow > 0.0 {
        el = el.flex_grow();
    }
    if let Some(size) = spec.min_size {
        el = el.min_w(size).min_h(size);
    }
    el
}
