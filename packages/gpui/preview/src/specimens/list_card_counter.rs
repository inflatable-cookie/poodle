use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, ListCardCounterSpec};

use crate::node_compat::{Eyebrow, ListCardCounter};

/// One captioned example group.
fn group(theme: &GpuiThemeProvider, caption: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(caption),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "Static footer counters",
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
                )),
        ))
        .child(group(
            theme,
            "Linked footer counter",
            ListCardCounter::from_spec(
                ListCardCounterSpec::new("layers", 3)
                    .with_tooltip("3 sub-folders")
                    .with_href("#sub-folders"),
                theme,
            ),
        ))
}
