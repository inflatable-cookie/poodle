use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::StatusIndicator;
use poodle_gpui_components::{Eyebrow, StatusBar};
use poodle_specs::ShellStatusBarSpec;
use poodle_specs::{EyebrowSpec, StatusIndicatorSpec, StatusTone};

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let mut branch_indicator = StatusIndicatorSpec::new().with_status(StatusTone::Info);
    branch_indicator.label = Some("main".to_string());

    let mut diagnostics_indicator = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    diagnostics_indicator.label = Some("0 errors".to_string());

    let mut sync_indicator = StatusIndicatorSpec::new().with_status(StatusTone::Warning);
    sync_indicator.label = Some("Sync delayed".to_string());

    let meta_item = |text: &str| {
        div()
            .text_xs()
            .text_color(color_to_hsla(text_secondary))
            .child(text.to_string())
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Workspace status"),
                    theme,
                ))
                .child(
                    StatusBar::from_spec(ShellStatusBarSpec::new().with_summary("Ready"), theme)
                        .with_leading_items(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(StatusIndicator::from_spec(branch_indicator, theme))
                                .child(StatusIndicator::from_spec(diagnostics_indicator, theme)),
                        )
                        .with_trailing_items(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(meta_item("Ln 42, Col 18"))
                                .child(meta_item("UTF-8"))
                                .child(meta_item("TypeScript")),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Active operation"),
                    theme,
                ))
                .child(
                    StatusBar::from_spec(
                        ShellStatusBarSpec::new().with_summary("Publishing 3 selected assets"),
                        theme,
                    )
                    .with_leading_items(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(StatusIndicator::from_spec(sync_indicator, theme)),
                    )
                    .with_trailing_items(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(meta_item("3 selected"))
                            .child(meta_item("2 warnings")),
                    ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Summary only"),
                    theme,
                ))
                .child(StatusBar::from_spec(
                    ShellStatusBarSpec::new().with_summary("3 items selected"),
                    theme,
                )),
        )
}
