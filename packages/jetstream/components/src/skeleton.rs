//! Skeleton — Jetstream placeholder component backed by SkeletonSpec.
//!
//! Jetstream cannot animate, so skeletons render as static gray boxes.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::SkeletonSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_skeleton(spec: &SkeletonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let radius = resolve_radius(theme, spec.radius_token());
    // Contract: default height from body typography size token (0.875rem = 14px)
    let default_height = resolve_px(theme, spec.default_height_token());

    let mut el = ui_element::div()
        .bg(fill)
        .rounded(radius)
        .min_h(default_height);

    // Default width fills container
    el = el.grow();

    el
}
