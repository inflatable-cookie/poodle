//! EmbedInput specimen — URL or embed code input.
//!
//! Mirrors the GPUI specimen group set: supported-providers reference table,
//! detection matrix, live URL input, Field wrapper, restricted providers,
//! visual states (empty / resolved / external error / disabled), and preset
//! states. Every group composes the real `js_embed_input` (which itself wraps
//! the real `js_text_input` + `js_pill`) — no hand-rolled inputs or fakes.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::embed_input::js_embed_input;
use poodle_jetstream_components::field::js_field;
use poodle_jetstream_components::presentation::rem_to_px;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{EmbedInputSpec, FieldSpec, ParsedEmbed};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let panel_bg = resolve_color(theme, "color.background.panel");
    let radius = resolve_radius(theme, "radius.control");
    let label_size = resolve_px(theme, "typography.label.size");

    div().flex_col().gap(24.0).max_w(640.0)
        // --- Supported providers (reference table) ---
        .child(group("Supported providers", secondary,
            div().flex_col()
                .border_1().border_color(border_subtle).rounded(radius)
                .child(provider_header_row(secondary, border_subtle, label_size))
                .child(provider_row(
                    "youtube",
                    "youtube.com/watch?v=, youtube.com/embed/, youtu.be/",
                    secondary, panel_bg, border_subtle, radius, label_size, false,
                ))
                .child(provider_row(
                    "vimeo",
                    "vimeo.com/{id}",
                    secondary, panel_bg, border_subtle, radius, label_size, false,
                ))
                .child(provider_row(
                    "generic",
                    "Any valid URL, or <iframe> embed code",
                    secondary, panel_bg, border_subtle, radius, label_size, true,
                ))
        ))
        // --- Detection matrix ---
        .child(group("Detection matrix", secondary,
            div().flex_col().gap(12.0)
                .child(state_row("YouTube short link", secondary, label_size,
                    EmbedInputSpec::new()
                        .with_value("https://youtu.be/dQw4w9WgXcQ")
                        .with_detected_parse(),
                    theme,
                ))
                .child(state_row("Vimeo link", secondary, label_size,
                    EmbedInputSpec::new()
                        .with_value("https://vimeo.com/123456")
                        .with_detected_parse(),
                    theme,
                ))
                .child(state_row("Iframe embed", secondary, label_size,
                    EmbedInputSpec::new()
                        .with_value(r#"<iframe src="https://example.com/embed/1" width="640" height="480"></iframe>"#)
                        .with_detected_parse(),
                    theme,
                ))
                .child(state_row("Restricted generic URL", secondary, label_size,
                    EmbedInputSpec::new()
                        .with_value("https://example.com/file.zip")
                        .with_providers(vec!["youtube".into(), "vimeo".into()])
                        .with_detected_parse(),
                    theme,
                ))
        ))
        // --- URL or embed code input (live placeholder) ---
        .child(group("URL or embed code input", secondary,
            js_embed_input(
                &EmbedInputSpec::new()
                    .with_placeholder("Paste a YouTube URL, Vimeo link, or embed code..."),
                theme,
            )
        ))
        // --- With Field wrapper ---
        .child(group("With Field wrapper", secondary,
            js_field(
                &FieldSpec::new("embed-input-video", "Video embed"),
                theme,
                Some(js_embed_input(
                    &EmbedInputSpec::new()
                        .with_placeholder("https://youtube.com/watch?v=..."),
                    theme,
                )),
            )
        ))
        // --- Restricted providers ---
        .child(group("Restricted providers", secondary,
            js_embed_input(
                &EmbedInputSpec::new()
                    .with_providers(vec!["youtube".into(), "vimeo".into()])
                    .with_placeholder("Only YouTube and Vimeo allowed..."),
                theme,
            )
        ))
        // --- Visual states (empty / resolved / external error / disabled) ---
        .child(group("States", secondary,
            div().flex_col().gap(12.0)
                .child(state_row("Empty", secondary, label_size,
                    EmbedInputSpec::new()
                        .with_placeholder("Paste a URL or embed code..."),
                    theme,
                ))
                .child(state_row("Resolved (success)", secondary, label_size,
                    EmbedInputSpec::new()
                        .with_value("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
                        .with_detected_parse(),
                    theme,
                ))
                .child(state_row("Error (external)", secondary, label_size,
                    EmbedInputSpec::new()
                        .with_value("not a url")
                        .with_error("Could not parse embed source"),
                    theme,
                ))
                .child(state_row("Disabled", secondary, label_size,
                    EmbedInputSpec::new()
                        .with_value("https://vimeo.com/76979871")
                        .with_disabled(true)
                        .with_detected_parse(),
                    theme,
                ))
        ))
        // --- Preset examples ---
        .child(group("Preset states", secondary,
            div().flex_col().gap(8.0)
                .child(js_embed_input(
                    &EmbedInputSpec::new()
                        .with_value("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
                        .with_detected_parse(),
                    theme,
                ))
                .child(js_embed_input(
                    &EmbedInputSpec::new()
                        .with_value("https://example.com/file.zip")
                        .with_providers(vec!["youtube".into(), "vimeo".into()])
                        .with_detected_parse(),
                    theme,
                ))
                .child(js_embed_input(
                    &EmbedInputSpec::new()
                        .with_value("https://custom.example/embed/42")
                        .with_parsed(
                            ParsedEmbed::new("custom", "42")
                                .with_original_url("https://custom.example/embed/42"),
                        ),
                    theme,
                ))
        ))
}

fn provider_header_row(text_secondary: glam::Vec4, border: glam::Vec4, font: f32) -> JsEl {
    div().flex_row().border_b_1().border_color(border)
        .child(
            label("Provider").flex_1()
                .px(rem_to_px(0.625)).py(rem_to_px(0.375))
                .text_size(font).font_weight(600).text_color(text_secondary),
        )
        .child(
            label("Detected patterns").grow()
                .px(rem_to_px(0.625)).py(rem_to_px(0.375))
                .text_size(font).font_weight(600).text_color(text_secondary),
        )
}

#[allow(clippy::too_many_arguments)]
fn provider_row(
    provider: &str,
    patterns: &str,
    text_secondary: glam::Vec4,
    panel_bg: glam::Vec4,
    border: glam::Vec4,
    radius: f32,
    font: f32,
    is_last: bool,
) -> JsEl {
    let mut row = div().flex_row();
    if !is_last {
        row = row.border_b_1().border_color(border);
    }
    row
        .child(
            div().flex_1().px(rem_to_px(0.625)).py(rem_to_px(0.375))
                .child(
                    label(provider.to_string())
                        .px(rem_to_px(0.25)).py(rem_to_px(0.0625))
                        .rounded(radius).bg(tint(panel_bg, 0.9))
                        .text_size(font).text_color(text_secondary),
                ),
        )
        .child(
            label(patterns.to_string()).grow()
                .px(rem_to_px(0.625)).py(rem_to_px(0.375))
                .text_size(font).text_color(text_secondary),
        )
}

fn state_row(
    title: &str,
    text_secondary: glam::Vec4,
    font: f32,
    spec: EmbedInputSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    div().flex_col().gap(4.0)
        .child(label(title.to_string()).text_size(font).font_weight(500).text_color(text_secondary))
        .child(js_embed_input(&spec, theme))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
