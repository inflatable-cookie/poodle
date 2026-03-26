use gpui::*;
use poodle_composites::{EmbedInputSpec, ParsedEmbed};
use poodle_gpui_components::{EmbedInput, Eyebrow};
use poodle_primitives::EyebrowSpec;
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        // -- Default --
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            EmbedInput::from_spec(
                                EmbedInputSpec::new()
                                    .with_placeholder("Paste a URL to embed..."),
                                theme,
                            )
                        )
                        .child(
                            EmbedInput::from_spec(
                                EmbedInputSpec::new()
                                    .with_value("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
                                    .with_detected_parse(),
                                theme,
                            )
                        )
                        .child(
                            EmbedInput::from_spec(
                                EmbedInputSpec::new()
                                    .with_value("https://example.com/file.zip")
                                    .with_providers(vec!["youtube".into(), "vimeo".into()])
                                    .with_detected_parse(),
                                theme,
                            )
                        )
                        .child(
                            EmbedInput::from_spec(
                                EmbedInputSpec::new()
                                    .with_value("https://custom.example/embed/42")
                                    .with_parsed(
                                        ParsedEmbed::new("custom", "42")
                                            .with_original_url("https://custom.example/embed/42"),
                                    ),
                                theme,
                            )
                        )
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Detection matrix"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(example_row(
                            "YouTube short link",
                            EmbedInputSpec::new()
                                .with_value("https://youtu.be/dQw4w9WgXcQ")
                                .with_detected_parse(),
                            theme,
                        ))
                        .child(example_row(
                            "Vimeo link",
                            EmbedInputSpec::new()
                                .with_value("https://vimeo.com/123456")
                                .with_detected_parse(),
                            theme,
                        ))
                        .child(example_row(
                            "Iframe embed",
                            EmbedInputSpec::new()
                                .with_value(r#"<iframe src="https://example.com/embed/1" width="640" height="480"></iframe>"#)
                                .with_detected_parse(),
                            theme,
                        ))
                        .child(example_row(
                            "Restricted generic URL",
                            EmbedInputSpec::new()
                                .with_value("https://example.com/file.zip")
                                .with_providers(vec!["youtube".into(), "vimeo".into()])
                                .with_detected_parse(),
                            theme,
                        ))
                )
        )
}

fn example_row(label: impl Into<String>, spec: EmbedInputSpec, theme: &GpuiThemeProvider) -> Div {
    let label = label.into();
    div().flex().flex_col().gap(px(4.0))
        .child(
            div()
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .child(label),
        )
        .child(EmbedInput::from_spec(spec, theme))
}
