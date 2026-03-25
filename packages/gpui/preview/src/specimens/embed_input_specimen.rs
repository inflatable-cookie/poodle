use gpui::*;
use poodle_composites::EmbedInputSpec;
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
                                    .with_value("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
                                theme,
                            )
                        )
                )
        )
}
