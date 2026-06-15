use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::ListCardCounter;
use poodle_specs::{InlineTypographyMode, ListCardCounterSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(
            div()
                .flex()
                .gap(px(14.0))
                .child(ListCardCounter::from_spec(
                    ListCardCounterSpec::new("file-text", 24).with_tooltip("24 documents"),
                    theme,
                ))
                .child(ListCardCounter::from_spec(
                    ListCardCounterSpec::new("image", 8).with_tooltip("8 images"),
                    theme,
                ))
                .child(ListCardCounter::from_spec(
                    ListCardCounterSpec::new("layers", 3).with_href("#sub-folders"),
                    theme,
                )),
        )
        .child(ListCardCounter::from_spec(
            ListCardCounterSpec::new("file-text", 24)
                .with_tooltip("24 documents")
                .with_typography(InlineTypographyMode::Inherit),
            theme,
        ))
}
