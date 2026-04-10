//! EmbedInput — Jetstream URL input with parsed embed preview pills backed by EmbedInputSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::EmbedInputSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_embed_input(spec: &EmbedInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");
    let placeholder_color = resolve_color(theme, "color.text.secondary");
    let status_color = resolve_color(theme, spec.status_text_color_token());

    let font_size = rem_to_px(0.8125);
    let status_font = rem_to_px(0.75);
    let pad_x = rem_to_px(0.75);
    let pad_y = rem_to_px(0.5);
    let min_h = rem_to_px(4.5);
    let pill_px = rem_to_px(0.375);
    let pill_py = rem_to_px(0.125);
    let pill_radius = rem_to_px(0.25);
    let gap = rem_to_px(0.25);

    let display = if spec.value.is_empty() {
        spec.placeholder.as_deref().unwrap_or("Paste URL or embed code...")
    } else {
        &spec.value
    };
    let color = if spec.value.is_empty() { placeholder_color } else { text_color };

    let (parsed, error) = spec.resolved_parse_state();

    // Textarea-like input surface
    let mut textarea = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .min_h(min_h)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y)
        .flex_col()
        .child(ui_element::label(display).text_color(color).text_size(font_size));

    if spec.is_disabled {
        textarea = textarea.opacity(0.5);
    }

    let mut wrapper = ui_element::div().flex_col().gap(gap).child(textarea);

    // Status row with parsed pill or error
    if error.is_some() || parsed.is_some() {
        let mut status_row = ui_element::div()
            .flex_row().items_center().gap(rem_to_px(0.25))
            .min_h(rem_to_px(1.25));

        if let Some(ref parsed) = parsed {
            let pill_bg = resolve_color(theme, "color.background.subtle");
            status_row = status_row.child(
                ui_element::label(&parsed.provider)
                    .text_color(status_color).text_size(status_font)
                    .bg(pill_bg)
                    .pl(pill_px).pr(pill_px).pt(pill_py).pb(pill_py)
                    .rounded(pill_radius)
            );
            status_row = status_row.child(
                ui_element::label("Embed detected").text_color(status_color).text_size(status_font)
            );
        }

        if let Some(ref err) = error {
            status_row = status_row.child(
                ui_element::label(err).text_color(status_color).text_size(status_font)
            );
        }

        wrapper = wrapper.child(status_row);
    }

    wrapper
}
