//! PageLoading — Jetstream loading overlay with spinner and message backed by PageLoadingSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::PageLoadingSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px,
    resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_page_loading(spec: &PageLoadingSpec, theme: &JetstreamThemeProvider) -> JsEl {
    if !spec.is_visible {
        return ui_element::div();
    }

    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let item_gap = rem_to_px(control_space_x_rem(spec.density));

    let backdrop = resolve_color(theme, spec.backdrop_fill_token());
    let accent = resolve_color(theme, spec.progress_fill_token());
    let text_color = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let surface_bg = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");

    let spinner_size = rem_to_px(2.0);

    // Card container with spinner + message
    let mut card = ui_element::div()
        .bg(surface_bg)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .flex_col().items_center().gap(item_gap * 2.0);

    // Spinner placeholder (ring)
    card = card.child(
        ui_element::icon("loader")
            .w(spinner_size).h(spinner_size)
            .text_color(accent)
    );

    // Message
    if let Some(ref msg) = spec.message {
        card = card.child(
            ui_element::label(msg).text_color(text_color).text_size(font_size)
        );
    }

    // Progress bar (if determinate)
    if let Some(value) = spec.value {
        let fraction = (value / spec.max).clamp(0.0, 1.0) as f32;
        let bar_h = rem_to_px(0.25);
        let bar_w = rem_to_px(12.0);
        let bar_radius = rem_to_px(0.125);

        let track = ui_element::div()
            .bg(border)
            .rounded(bar_radius)
            .h(bar_h).w(bar_w)
            .child(
                ui_element::div()
                    .bg(accent)
                    .rounded(bar_radius)
                    .h(bar_h)
                    .w(bar_w * fraction)
            );

        card = card.child(track);
    }

    // Cancel action
    if spec.can_cancel {
        card = card.child(
            ui_element::label("Cancel")
                .text_color(text_secondary)
                .text_size(rem_to_px(size_font_rem(effective_size) * 0.85))
                .cursor_pointer()
        );
    }

    // Full-page backdrop
    ui_element::div()
        .bg(backdrop)
        .grow()
        .flex_col().items_center().justify_center()
        .child(card)
}
