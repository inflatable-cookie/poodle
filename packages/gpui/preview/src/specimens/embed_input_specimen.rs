use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_composites::{EmbedInputSpec, ParsedEmbed};
use poodle_gpui_components::{EmbedInput, Eyebrow, Field};
use poodle_primitives::{EyebrowSpec, FieldSpec};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let panel_bg = theme.resolve_color("color.background.panel");

    div().flex().flex_col().gap(px(24.0)).max_w(px(640.0))
        // --- Supported providers (reference table) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Supported providers"), theme))
                .child(
                    div().flex().flex_col()
                        .border_1().border_color(color_to_hsla(border_subtle))
                        .rounded(px(4.0))
                        .child(provider_header_row(theme, text_secondary, border_subtle))
                        .child(provider_row(
                            "youtube",
                            "youtube.com/watch?v=, youtube.com/embed/, youtu.be/",
                            theme,
                            text_secondary,
                            panel_bg,
                            border_subtle,
                            false,
                        ))
                        .child(provider_row(
                            "vimeo",
                            "vimeo.com/{id}",
                            theme,
                            text_secondary,
                            panel_bg,
                            border_subtle,
                            false,
                        ))
                        .child(provider_row(
                            "generic",
                            "Any valid URL, or <iframe> embed code",
                            theme,
                            text_secondary,
                            panel_bg,
                            border_subtle,
                            true, // last row — no bottom border
                        ))
                )
        )
        // --- Detection matrix ---
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
        // --- URL or embed code input (interactive placeholder) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("URL or embed code input"), theme))
                .child(
                    EmbedInput::from_spec(
                        EmbedInputSpec::new()
                            .with_placeholder("Paste a YouTube URL, Vimeo link, or embed code..."),
                        theme,
                    )
                )
        )
        // --- With Field wrapper ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With Field wrapper"), theme))
                .child(
                    Field::from_spec(
                        FieldSpec::new("embed-input-video", "Video embed"),
                        theme,
                    )
                    .with_control(
                        EmbedInput::from_spec(
                            EmbedInputSpec::new()
                                .with_placeholder("https://youtube.com/watch?v=..."),
                            theme,
                        )
                    )
                )
        )
        // --- Restricted providers ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Restricted providers"), theme))
                .child(
                    EmbedInput::from_spec(
                        EmbedInputSpec::new()
                            .with_providers(vec!["youtube".into(), "vimeo".into()])
                            .with_placeholder("Only YouTube and Vimeo allowed..."),
                        theme,
                    )
                )
        )
        // --- Preset examples (previously the "Default" block) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Preset states"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
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
}

fn provider_header_row(
    theme: &GpuiThemeProvider,
    text_secondary: poodle_tokens::typed::ColorValue,
    border_subtle: poodle_tokens::typed::ColorValue,
) -> Div {
    let _ = theme;
    div().flex()
        .border_b_1().border_color(color_to_hsla(border_subtle))
        .child(
            div().flex_1().px(px(10.0)).py(px(6.0))
                .text_size(px(11.0)).font_weight(FontWeight::SEMIBOLD)
                .text_color(color_to_hsla(text_secondary))
                .child("Provider")
        )
        .child(
            div().flex_grow().px(px(10.0)).py(px(6.0))
                .text_size(px(11.0)).font_weight(FontWeight::SEMIBOLD)
                .text_color(color_to_hsla(text_secondary))
                .child("Detected patterns")
        )
}

fn provider_row(
    provider: &'static str,
    patterns: &'static str,
    theme: &GpuiThemeProvider,
    text_secondary: poodle_tokens::typed::ColorValue,
    panel_bg: poodle_tokens::typed::ColorValue,
    border_subtle: poodle_tokens::typed::ColorValue,
    is_last: bool,
) -> Div {
    let _ = theme;
    let code_bg = {
        let mut h = color_to_hsla(panel_bg);
        h.a *= 0.9;
        h
    };

    let code_chip = |text: &str| {
        div()
            .px(px(4.0)).py(px(1.0))
            .rounded(px(3.0))
            .bg(code_bg)
            .text_size(px(11.0))
            .font_family("Menlo")
            .child(text.to_string())
    };

    let mut row = div().flex();
    if !is_last {
        row = row.border_b_1().border_color(color_to_hsla(border_subtle));
    }

    row
        .child(
            div().flex_1().px(px(10.0)).py(px(6.0))
                .child(code_chip(provider))
        )
        .child(
            div().flex_grow().px(px(10.0)).py(px(6.0))
                .text_size(px(11.0))
                .text_color(color_to_hsla(text_secondary))
                .child(patterns.to_string())
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
