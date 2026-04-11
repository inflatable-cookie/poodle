//! ConfirmAction — Jetstream confirm action backed by ConfirmActionSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::ConfirmActionSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px,
    resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius, resolve_px};

pub fn js_confirm_action(spec: &ConfirmActionSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let title_size = rem_to_px(size_font_rem(effective_size) + 0.1875);
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let gap = resolve_px(theme, "space.stack.md");
    let action_gap = resolve_px(theme, "space.inline.sm");

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let confirm_fill = resolve_color(theme, spec.confirm_fill_token());
    let text_inverse = resolve_color(theme, "color.text.inverse");
    let border = resolve_color(theme, "color.border.default");
    let surface = resolve_color(theme, "color.background.surface");
    let radius = resolve_radius(theme, "radius.surface");
    let ctrl_radius = resolve_radius(theme, "radius.control");
    let btn_height = rem_to_px(control_height_rem(effective_size));

    if !spec.is_open {
        // Closed state: show only trigger button
        let trigger_tone = if spec.is_destructive() {
            resolve_color(theme, "color.status.danger")
        } else {
            text_primary
        };

        return ui_element::button(&spec.title)
            .text_color(trigger_tone)
            .text_size(font_size)
            .h(btn_height)
            .pl(pad_x).pr(pad_x)
            .border(1.0).border_color(border)
            .rounded(ctrl_radius)
            .bg(surface)
            .focusable();
    }

    // Open state: render the alert dialog
    let backdrop = resolve_color(theme, spec.backdrop_fill_token());

    let mut dialog = ui_element::div()
        .bg(backdrop)
        .flex_col().items_center().justify_center()
        .grow();

    let mut card = ui_element::div()
        .bg(surface)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col().gap(gap)
        .pl(rem_to_px(1.5)).pr(rem_to_px(1.5))
        .pt(rem_to_px(1.25)).pb(rem_to_px(1.25))
        .max_w(rem_to_px(28.0));

    // Title
    card = card.child(
        ui_element::label(&spec.title)
            .text_color(text_primary).text_size(title_size).text_weight(600)
    );

    // Description
    if !spec.description.is_empty() {
        card = card.child(
            ui_element::label(&spec.description)
                .text_color(text_secondary).text_size(font_size)
        );
    }

    // Action buttons
    let actions = ui_element::div()
        .flex_row().gap(action_gap).justify_end()
        .child(
            ui_element::button(&spec.cancel_label)
                .text_color(text_primary).text_size(font_size)
                .h(btn_height).pl(pad_x).pr(pad_x)
                .border(1.0).border_color(border)
                .rounded(ctrl_radius)
                .bg(surface)
                .focusable()
        )
        .child(
            ui_element::button(&spec.confirm_label)
                .text_color(text_inverse).text_size(font_size)
                .h(btn_height).pl(pad_x).pr(pad_x)
                .rounded(ctrl_radius)
                .bg(confirm_fill)
                .focusable()
        );
    card = card.child(actions);

    dialog = dialog.child(card);

    dialog
}
