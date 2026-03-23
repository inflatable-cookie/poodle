//! Skeleton — Jetstream placeholder component backed by SkeletonSpec.
//!
//! Jetstream cannot animate, so skeletons render as static gray boxes.

use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::SkeletonSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_skeleton(spec: &SkeletonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let radius = resolve_radius(theme, spec.radius_token());

    let mut el = ui_element::div()
        .bg(fill)
        .rounded(radius)
        .min_h(16.0);

    // Default width fills container
    el = el.grow();

    el
}
