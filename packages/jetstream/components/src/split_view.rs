//! SplitView — Jetstream split view backed by SplitViewSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{SplitOrientation, SplitViewSpec};

use crate::presentation::resolve_semantic_size;

pub fn js_split_view(spec: &SplitViewSpec, _theme: &JetstreamThemeProvider, primary: Option<JsEl>, secondary: Option<JsEl>) -> JsEl {
    let _effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let mut el = match spec.orientation {
        SplitOrientation::Horizontal => ui_element::div().flex_row().grow(),
        SplitOrientation::Vertical => ui_element::div().flex_col().grow(),
    };

    if let Some(p) = primary {
        el = el.child(p);
    }
    if let Some(s) = secondary {
        el = el.child(s);
    }

    el
}
