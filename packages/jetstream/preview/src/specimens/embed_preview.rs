//! EmbedPreview specimen — rich preview of detected embeds.
//!
//! Mirrors the GPUI specimen group set: YouTube, Vimeo, generic iframe raw
//! embed, trusted raw embed (auto aspect), loading skeleton, error, and empty.
//! Every group composes the real `js_embed_preview` (which wraps the real
//! `js_skeleton` block + `js_text_link` fallback) — no fakes.

use crate::compat::js_embed_preview;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{EmbedPreviewSpec, ParsedEmbed};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .max_w(560.0)
        // --- YouTube embed ---
        .child(group(
            "YouTube embed",
            secondary,
            js_embed_preview(
                &EmbedPreviewSpec::new().with_parsed(
                    ParsedEmbed::new("youtube", "dQw4w9WgXcQ")
                        .with_original_url("https://youtube.com/watch?v=dQw4w9WgXcQ"),
                ),
                theme,
            ),
        ))
        // --- Vimeo embed ---
        .child(group(
            "Vimeo embed",
            secondary,
            js_embed_preview(
                &EmbedPreviewSpec::new().with_parsed(
                    ParsedEmbed::new("vimeo", "76979871")
                        .with_original_url("https://vimeo.com/76979871"),
                ),
                theme,
            ),
        ))
        // --- Generic iframe embed (raw originalEmbed) ---
        .child(group(
            "Generic iframe embed",
            secondary,
            js_embed_preview(
                &EmbedPreviewSpec::new().with_parsed(
                    ParsedEmbed::new("generic", "https://example.com/embed/1")
                        .with_original_embed(
                            r#"<iframe src="https://example.com/embed/1"></iframe>"#,
                        )
                        .with_original_url("https://example.com/embed/1"),
                ),
                theme,
            ),
        ))
        // --- Trusted raw embed (caller-sanitized HTML, auto aspect) ---
        .child(group(
            "Trusted raw embed",
            secondary,
            js_embed_preview(
                &EmbedPreviewSpec::new()
                    .with_trusted_html(r#"<iframe title="Audio embed" src="about:blank"></iframe>"#)
                    .with_auto_aspect_ratio(),
                theme,
            ),
        ))
        // --- Loading state ---
        .child(group(
            "Loading state",
            secondary,
            js_embed_preview(&EmbedPreviewSpec::new().with_loading(true), theme),
        ))
        // --- Error state ---
        .child(group(
            "Error state",
            secondary,
            js_embed_preview(
                &EmbedPreviewSpec::new().with_error(
                    "Failed to load embed. The URL may be invalid or the provider is unavailable.",
                ),
                theme,
            ),
        ))
        // --- Empty state (custom message) ---
        .child(group(
            "Empty state",
            secondary,
            js_embed_preview(
                &EmbedPreviewSpec::new().with_empty_message("Paste a URL above to see a preview"),
                theme,
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
