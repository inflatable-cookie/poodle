use crate::node_compat::{Eyebrow, PageLoading};
use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::EyebrowSpec;
use poodle_specs::{ControlSize, PageLoadingPresentation, PageLoadingSpec};

fn group(theme: &GpuiThemeProvider, label: &'static str, body: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(body)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let root = div().flex().flex_col().gap(px(24.0));

    // Inline presentation — centered spinner + message inside an in-flow shell,
    // no overlay chrome (contract §8 inline-card override).
    let inline = group(
        theme,
        "Inline",
        div()
            .min_h(px(288.0))
            .border_1()
            .border_color(color_to_hsla(theme.resolve_color("color.border.default")))
            .rounded(px(8.0))
            // GPUI has no dashed border. Approximate the Svelte inline shell.
            .bg(color_to_hsla(
                theme.resolve_color("color.background.surface"),
            ))
            .child(PageLoading::from_spec(
                PageLoadingSpec::new()
                    .with_message("Loading section content...")
                    .with_presentation(PageLoadingPresentation::Inline),
                theme,
            )),
    );

    // Indeterminate (overlay) — centered ring spinner + message, no progress.
    let indeterminate = group(
        theme,
        "Indeterminate (spinner only)",
        PageLoading::from_spec(
            PageLoadingSpec::new().with_message("Loading data..."),
            theme,
        ),
    );

    // Determinate (overlay) — spinner + progress bar + percentage message.
    let determinate = group(
        theme,
        "Determinate (with progress bar)",
        PageLoading::from_spec(
            PageLoadingSpec::new()
                .with_value(64.0)
                .with_max(100.0)
                .with_message("Uploading files... 64%"),
            theme,
        ),
    );

    // With cancel — spinner + message + bordered cancel control.
    let with_cancel = group(
        theme,
        "With cancel button",
        PageLoading::from_spec(
            PageLoadingSpec::new()
                .with_message("Processing request...")
                .with_can_cancel(true),
            theme,
        ),
    );

    // Size ladder — xs → xl; size drives message font size.
    let sizes: Vec<(ControlSize, &'static str)> = vec![
        (ControlSize::Xs, "xs"),
        (ControlSize::Sm, "sm"),
        (ControlSize::Md, "md"),
        (ControlSize::Lg, "lg"),
        (ControlSize::Xl, "xl"),
    ];
    let mut size_row = div().flex().flex_col().gap(px(16.0));
    for (size, label) in sizes {
        size_row = size_row.child(group(
            theme,
            label,
            PageLoading::from_spec(
                PageLoadingSpec::new()
                    .with_message("Loading data...")
                    .with_size(size),
                theme,
            ),
        ));
    }
    let size_ladder = group(theme, "Sizes", size_row);

    root.child(inline)
        .child(indeterminate)
        .child(determinate)
        .child(with_cancel)
        .child(size_ladder)
}
