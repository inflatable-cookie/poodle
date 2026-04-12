use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{EmbedPreview, Eyebrow};
use poodle_specs::EyebrowSpec;
use poodle_specs::{EmbedPreviewSpec, ParsedEmbed};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0)).max_w(px(560.0))
        // --- YouTube embed ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("YouTube embed"), theme))
                .child(
                    EmbedPreview::from_spec(
                        EmbedPreviewSpec::new()
                            .with_parsed(
                                ParsedEmbed::new("youtube", "dQw4w9WgXcQ")
                                    .with_original_url("https://youtube.com/watch?v=dQw4w9WgXcQ"),
                            ),
                        theme,
                    )
                )
        )
        // --- Vimeo embed ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Vimeo embed"), theme))
                .child(
                    EmbedPreview::from_spec(
                        EmbedPreviewSpec::new()
                            .with_parsed(
                                ParsedEmbed::new("vimeo", "76979871")
                                    .with_original_url("https://vimeo.com/76979871"),
                            ),
                        theme,
                    )
                )
        )
        // --- Generic iframe embed ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Generic iframe embed"), theme))
                .child(
                    EmbedPreview::from_spec(
                        EmbedPreviewSpec::new()
                            .with_parsed(
                                ParsedEmbed::new("generic", "https://example.com/embed/1")
                                    .with_original_embed(r#"<iframe src="https://example.com/embed/1"></iframe>"#)
                                    .with_original_url("https://example.com/embed/1"),
                            ),
                        theme,
                    )
                )
        )
        // --- Loading state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Loading state"), theme))
                .child(
                    EmbedPreview::from_spec(
                        EmbedPreviewSpec::new().with_loading(true),
                        theme,
                    )
                )
        )
        // --- Error state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Error state"), theme))
                .child(
                    EmbedPreview::from_spec(
                        EmbedPreviewSpec::new()
                            .with_error("Failed to load embed. The URL may be invalid or the provider is unavailable."),
                        theme,
                    )
                )
        )
        // --- Empty state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Empty state"), theme))
                .child(
                    EmbedPreview::from_spec(
                        EmbedPreviewSpec::new()
                            .with_empty_message("Paste a URL above to see a preview"),
                        theme,
                    )
                )
        )
}
