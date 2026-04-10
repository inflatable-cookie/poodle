//! EmbedPreview — Jetstream iframe/image/audio/video embed display backed by EmbedPreviewSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::EmbedPreviewSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_embed_preview(spec: &EmbedPreviewSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let danger_color = resolve_color(theme, "color.status.danger");
    let success_color = resolve_color(theme, "color.status.success");
    let subtle_bg = resolve_color(theme, "color.background.subtle");

    let pad_x = rem_to_px(1.0);
    let pad_y = rem_to_px(0.75);
    let gap = rem_to_px(0.5);
    let label_size = rem_to_px(0.8125);
    let small_size = rem_to_px(0.75);

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .flex_col().gap(gap)
        .overflow_hidden();

    // Loading state
    if spec.is_loading {
        let skeleton = ui_element::div()
            .bg(subtle_bg).rounded(rem_to_px(0.25))
            .h(rem_to_px(0.75)).grow();
        let skeleton_short = ui_element::div()
            .bg(subtle_bg).rounded(rem_to_px(0.25))
            .h(rem_to_px(0.75)).w(rem_to_px(10.0));
        el = el.child(
            ui_element::div().flex_col().gap(rem_to_px(0.5))
                .child(skeleton)
                .child(skeleton_short)
        );
        return el;
    }

    // Error state
    if let Some(ref error) = spec.error {
        el = el.child(
            ui_element::div()
                .flex_col().items_center().justify_center().gap(rem_to_px(0.375))
                .pt(rem_to_px(1.0)).pb(rem_to_px(1.0))
                .child(ui_element::icon("alert-circle").w(rem_to_px(1.5)).h(rem_to_px(1.5)).text_color(danger_color))
                .child(ui_element::label(error).text_color(text_secondary).text_size(small_size))
        );
        return el;
    }

    // Empty state
    if spec.parsed.is_none() {
        el = el.child(
            ui_element::div()
                .flex_col().items_center().justify_center().gap(rem_to_px(0.375))
                .pt(rem_to_px(1.0)).pb(rem_to_px(1.0))
                .child(ui_element::icon("monitor-play").w(rem_to_px(1.5)).h(rem_to_px(1.5)).text_color(text_secondary))
                .child(ui_element::label(&spec.empty_message).text_color(text_secondary).text_size(small_size))
        );
        return el;
    }

    let parsed = spec.parsed.as_ref().unwrap();
    let embed_url = spec.embed_url();

    // Provider pill
    el = el.child(
        ui_element::div().flex_row().items_center().gap(rem_to_px(0.375))
            .child(ui_element::icon("link").w(rem_to_px(0.875)).h(rem_to_px(0.875)).text_color(success_color))
            .child(
                ui_element::label(&parsed.provider)
                    .text_color(success_color).text_size(rem_to_px(0.6875))
                    .bg(subtle_bg)
                    .pl(rem_to_px(0.375)).pr(rem_to_px(0.375))
                    .pt(rem_to_px(0.125)).pb(rem_to_px(0.125))
                    .rounded(rem_to_px(0.25))
            )
    );

    // Media frame placeholder
    if let Some(url) = embed_url {
        let preview_label = if parsed.provider == "youtube" || parsed.provider == "vimeo" {
            "Platform preview placeholder"
        } else {
            "External embed placeholder"
        };

        let min_frame_h = if spec.effective_aspect_ratio().is_some() {
            rem_to_px(12.5)
        } else {
            rem_to_px(10.0)
        };

        el = el.child(
            ui_element::div()
                .bg(subtle_bg)
                .border(1.0).border_color(border)
                .rounded(radius)
                .min_h(min_frame_h).grow()
                .flex_col().items_center().justify_center().gap(rem_to_px(0.5))
                .pl(rem_to_px(1.0)).pr(rem_to_px(1.0))
                .pt(rem_to_px(1.25)).pb(rem_to_px(1.25))
                .child(ui_element::icon("monitor-play").w(rem_to_px(1.5)).h(rem_to_px(1.5)).text_color(success_color))
                .child(ui_element::label(preview_label).text_color(text_primary).text_size(label_size).text_weight(500))
                .child(ui_element::label(&url).text_color(text_secondary).text_size(small_size))
        );
    } else if spec.has_raw_embed() {
        el = el.child(
            ui_element::div()
                .bg(subtle_bg)
                .border(1.0).border_color(border)
                .rounded(radius)
                .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
                .flex_col().gap(rem_to_px(0.375))
                .child(ui_element::label("Raw embed code").text_color(text_primary).text_size(label_size).text_weight(500))
                .child(ui_element::label(parsed.original_embed.as_deref().unwrap_or("")).text_color(text_secondary).text_size(small_size))
        );
    } else {
        el = el.child(
            ui_element::label(parsed.original_url.as_deref().unwrap_or(&parsed.id))
                .text_color(success_color).text_size(small_size)
        );
    }

    el
}
