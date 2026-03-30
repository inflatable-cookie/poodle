//! ToastStack — Jetstream toast notification stack backed by ToastStackSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::ToastStackSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_toast_stack(spec: &ToastStackSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density) - 0.25);
    let stack_gap = resolve_px(theme, spec.gap_token());
    let item_gap = rem_to_px(0.5);

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_color = resolve_color(theme, spec.title_color_token());

    let mut el = ui_element::div().flex_col().gap(stack_gap);

    for toast in &spec.toasts {
        let toast_el = ui_element::div()
            .bg(fill)
            .border(1.0).border_color(border)
            .rounded(radius)
            .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
            .flex_row().items_center().gap(item_gap)
            .child(ui_element::label(toast.message.as_deref().unwrap_or("")).text_color(text_color).text_size(font_size));
        el = el.child(toast_el);
    }

    el
}
