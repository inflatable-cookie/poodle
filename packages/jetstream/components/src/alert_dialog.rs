//! AlertDialog — Jetstream alert dialog backed by AlertDialogSpec.
//!
//! Contract: `docs/contracts/components/alert-dialog.md`
//! Uses overlay() with backdrop. Alert dialogs are not dismissible by backdrop click.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::AlertDialogSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_alert_dialog(spec: &AlertDialogSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_font = rem_to_px(size_font_rem(effective_size) + 0.1875);
    let body_font = rem_to_px(size_font_rem(effective_size));

    // Padding from tokens (spec.padding_x/y_token replace the old +0.5 formula)
    let space_x = resolve_px(theme, spec.padding_x_token());
    let space_y = resolve_px(theme, spec.padding_y_token());

    // Gap between content sections
    let content_gap = resolve_px(theme, spec.content_gap_token());
    // Gap between action buttons
    let actions_gap = resolve_px(theme, spec.actions_gap_token());

    let fill: Color = resolve_color(theme, "color.background.elevated").into();
    let backdrop: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border_color: Color = resolve_color(theme, spec.border_token()).into();
    let radius = resolve_radius(theme, "radius.surface");
    let ctrl_radius = resolve_radius(theme, spec.button_radius_token());

    let title_color = resolve_color(theme, spec.title_color_token());
    let desc_color = resolve_color(theme, spec.description_color_token());
    let confirm_fill = resolve_color(theme, spec.confirm_fill_token());
    let confirm_text = resolve_color(theme, spec.confirm_text_color_token());
    let cancel_text = resolve_color(theme, spec.cancel_text_color_token());

    // Compact action button padding (not tied to control height — these are small inline actions)
    let btn_pad_x = rem_to_px(0.75);
    let btn_pad_y = rem_to_px(0.375);

    let mut panel = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border_color)
        .rounded(radius)
        .pl(space_x).pr(space_x).pt(space_y).pb(space_y)
        .flex_col().gap(content_gap)
        .min_w(rem_to_px(20.0))
        .shadow_lg();

    if !spec.title.is_empty() {
        panel = panel.child(
            ui_element::label(&spec.title)
                .text_color(title_color)
                .text_size(title_font)
                .text_weight(600)
        );
    }

    if let Some(ref desc) = spec.description {
        panel = panel.child(
            ui_element::label(desc)
                .text_color(desc_color)
                .text_size(body_font)
        );
    }

    // Separator above actions
    panel = panel.child(
        ui_element::div()
            .h(1.0)
            .bg(border_color)
    );

    // Actions row
    let actions = ui_element::div()
        .flex_row()
        .justify_end()
        .gap(actions_gap)
        .child(
            ui_element::button(&spec.cancel_label)
                .text_color(cancel_text)
                .text_size(body_font)
                .bg(Color::TRANSPARENT)
                .pl(btn_pad_x).pr(btn_pad_x).pt(btn_pad_y).pb(btn_pad_y)
                .rounded(ctrl_radius)
                .cursor_pointer()
                .focusable()
        )
        .child(
            ui_element::button(&spec.confirm_label)
                .text_color(confirm_text)
                .text_size(body_font)
                .bg(confirm_fill)
                .pl(btn_pad_x).pr(btn_pad_x).pt(btn_pad_y).pb(btn_pad_y)
                .rounded(ctrl_radius)
                .cursor_pointer()
                .focusable()
        );

    panel = panel.child(actions);

    // Backdrop + centered panel as overlay
    ui_element::div()
        .bg(backdrop)
        .overlay()
        .items_center().justify_center()
        .child(panel)
}
