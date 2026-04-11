use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_components::ShellStatusBarSpec;
use poodle_gpui_components::{StatusBar, Eyebrow};
use poodle_components::{StatusIndicatorSpec, StatusTone, EyebrowSpec};
use poodle_gpui_components::StatusIndicator;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    // --- Default ---
    let mut branch_indicator = StatusIndicatorSpec::new().with_status(StatusTone::Info);
    branch_indicator.label = Some("main".to_string());

    let mut error_indicator = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    error_indicator.label = Some("0 errors".to_string());

    let status_spec = ShellStatusBarSpec::new()
        .with_summary("Ready");

    let meta_item = |text: &str| {
        div().text_xs().text_color(color_to_hsla(text_secondary)).child(text.to_string())
    };

    div().flex().flex_col().gap(px(24.0))
        // --- Default with leading/trailing ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    StatusBar::from_spec(status_spec, theme)
                        .with_leading_items(
                            div().flex().items_center().gap(px(8.0))
                                .child(StatusIndicator::from_spec(branch_indicator, theme))
                                .child(StatusIndicator::from_spec(error_indicator, theme))
                        )
                        .with_trailing_items(
                            div().flex().items_center().gap(px(8.0))
                                .child(meta_item("Ln 42, Col 18"))
                                .child(meta_item("UTF-8"))
                                .child(meta_item("TypeScript"))
                        )
                )
        )
        // --- Summary only ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Summary only"), theme))
                .child(
                    StatusBar::from_spec(
                        ShellStatusBarSpec::new().with_summary("3 items selected"),
                        theme,
                    )
                )
        )
}
