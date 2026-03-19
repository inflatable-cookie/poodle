use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::FileUploadSpec;
use pug_gpui_components::PugFileUpload;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0)).max_w(px(400.0))
        // --- Image Upload With Preview ---
        .child(section_label("IMAGE UPLOAD WITH PREVIEW", text_secondary))
        .child(
            PugFileUpload::new(
                FileUploadSpec::new()
                    .with_accept("image/*")
                    .with_multiple(true)
                    .with_max_size(5 * 1024 * 1024),
                theme,
            )
        )
        // --- Document Upload (Single File) ---
        .child(section_label("DOCUMENT UPLOAD (SINGLE FILE)", text_secondary))
        .child(
            PugFileUpload::new(
                FileUploadSpec::new()
                    .with_accept(".pdf,.doc,.docx,.txt")
                    .with_max_size(10 * 1024 * 1024),
                theme,
            )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            PugFileUpload::new(
                FileUploadSpec::new()
                    .with_disabled(true),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
