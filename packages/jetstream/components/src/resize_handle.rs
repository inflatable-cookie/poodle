//! ResizeHandle — Jetstream resize handle backed by ResizeHandleSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::{Orientation, ResizeHandleSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity};

pub fn js_resize_handle(spec: &ResizeHandleSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let handle_color = resolve_color(theme, spec.border_color_token());

    // Contract: visual affordance 2px (0.125rem), hit target min 8px (0.5rem)
    let visual_size = rem_to_px(0.125);
    let hit_target = rem_to_px(0.5);

    let mut el = match spec.orientation {
        Orientation::Horizontal => {
            // Horizontal separator = horizontal line, vertical drag
            ui_element::div()
                .h(hit_target).grow()
                .flex_row().items_center().justify_center()
                .child(ui_element::div().h(visual_size).grow().bg(handle_color))
        }
        Orientation::Vertical => {
            // Vertical separator = vertical line, horizontal drag
            ui_element::div()
                .w(hit_target).grow()
                .flex_col().items_center().justify_center()
                .child(ui_element::div().w(visual_size).grow().bg(handle_color))
        }
    };

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity);
    }

    el
}
