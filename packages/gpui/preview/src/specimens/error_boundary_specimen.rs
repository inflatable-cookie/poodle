use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{ErrorBoundary, Text};
use poodle_specs::{ErrorBoundarySpec, TextSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(
            ErrorBoundary::from_spec(ErrorBoundarySpec::new(), theme).with_child(Text::from_spec(
                TextSpec::new("Stable child content."),
                theme,
            )),
        )
        .child(ErrorBoundary::from_spec(
            ErrorBoundarySpec::new()
                .with_title("Preview failed")
                .with_retry_label("Reset boundary")
                .with_error_message("Preview child failed during render."),
            theme,
        ))
}
