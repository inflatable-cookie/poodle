use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_workstation::ShellStatusBarSpec;
use pug_gpui_components::PugShellStatusBar;
use pug_gpui_primitives::{StatusIndicatorSpec, StatusTone};
use pug_gpui_components::PugStatusIndicator;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // --- Default ---
    // summary="Ready", leading: "main" branch indicator + "0 errors" status
    // trailing: "Ln 42, Col 18", "UTF-8", "TypeScript"

    let mut branch_indicator = StatusIndicatorSpec::new().with_status(StatusTone::Info);
    branch_indicator.label = Some("main".to_string());

    let mut error_indicator = StatusIndicatorSpec::new().with_status(StatusTone::Success);
    error_indicator.label = Some("0 errors".to_string());

    let status_spec = ShellStatusBarSpec::new()
        .with_summary("Ready");

    let meta_item = |text: &str| {
        div().text_xs().text_color(color_to_hsla(text_secondary)).child(text.to_string())
    };

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("DEFAULT", text_secondary))
        .child(
            PugShellStatusBar::new(status_spec, theme)
                .with_leading_items(
                    div().flex().items_center().gap(px(8.0))
                        .child(PugStatusIndicator::new(branch_indicator, theme))
                        .child(PugStatusIndicator::new(error_indicator, theme))
                )
                .with_trailing_items(
                    div().flex().items_center().gap(px(8.0))
                        .child(meta_item("Ln 42, Col 18"))
                        .child(meta_item("UTF-8"))
                        .child(meta_item("TypeScript"))
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
