use crate::node_compat::{ErrorBoundary, Eyebrow, Surface, Text};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ErrorBoundarySpec, EyebrowSpec, PaddingScale, SurfaceBorder, SurfaceSpec, TextSpec,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(560.0))
        // --- Normal children: boundary passes child through with no chrome ---
        .child(group(
            "Normal children",
            theme,
            ErrorBoundary::from_spec(ErrorBoundarySpec::new(), theme).with_child(
                Surface::from_spec(
                    SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Md),
                    theme,
                )
                .with_content(Text::node_from_spec(
                    TextSpec::new("Stable child content renders without boundary chrome."),
                    theme,
                )),
            ),
        ))
        // --- Caught render error: EmptyState fallback with custom title + retry ---
        .child(group(
            "Caught render error",
            theme,
            ErrorBoundary::from_spec(
                ErrorBoundarySpec::new()
                    .with_title("Preview failed")
                    .with_retry_label("Reset boundary")
                    .with_error_message("Preview child failed during render."),
                theme,
            ),
        ))
        // --- Default fallback: default title + retry label from the spec ---
        .child(group(
            "Default fallback",
            theme,
            ErrorBoundary::from_spec(
                ErrorBoundarySpec::new()
                    .with_error_message("An unexpected error occurred while rendering this view."),
                theme,
            ),
        ))
}

fn group(label: &'static str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}
