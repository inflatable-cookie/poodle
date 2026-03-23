use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{ListCardSpec, StatusIndicatorSpec, StatusTone, PillSpec, PillTone};
use pug_gpui_components::{ListCard, StatusIndicator, Pill};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let last_clicked = state.specimens.text.get("list-card-clicked").cloned();

    div().flex().flex_col().gap(px(16.0)).max_w(px(400.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    ListCard::from_spec(
                        ListCardSpec::new()
                            .with_title("Project Alpha")
                            .with_subtitle("Last updated 2 hours ago"),
                        theme,
                    )
                )
                .child(
                    ListCard::from_spec(
                        ListCardSpec::new()
                            .with_title("Project Beta")
                            .with_subtitle("Last updated yesterday"),
                        theme,
                    )
                )
        )
        // --- With leading and trailing ---
        .child(section_label("WITH LEADING AND TRAILING", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child({
                    let mut status = StatusIndicatorSpec::new().with_status(StatusTone::Success);
                    status.aria_label = Some("Active".to_string());
                    ListCard::from_spec(
                        ListCardSpec::new()
                            .with_title("API Server")
                            .with_subtitle("Running on port 8080"),
                        theme,
                    )
                    .with_leading(StatusIndicator::from_spec(status, theme))
                    .with_trailing(Pill::from_spec(
                        PillSpec::new().with_label("Active").with_tone(PillTone::Success),
                        theme,
                    ))
                })
                .child({
                    let mut status = StatusIndicatorSpec::new().with_status(StatusTone::Warning);
                    status.aria_label = Some("Degraded".to_string());
                    ListCard::from_spec(
                        ListCardSpec::new()
                            .with_title("Background Worker")
                            .with_subtitle("High queue depth"),
                        theme,
                    )
                    .with_leading(StatusIndicator::from_spec(status, theme))
                    .with_trailing(Pill::from_spec(
                        PillSpec::new().with_label("Degraded").with_tone(PillTone::Warning),
                        theme,
                    ))
                })
        )
        // --- Interactive ---
        .child(section_label("INTERACTIVE (CLICKABLE)", text_secondary))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    ListCard::from_spec(
                        ListCardSpec::new()
                            .with_title("Dashboard")
                            .with_subtitle("View metrics and charts")
                            .with_interactive(true),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.text.insert(
                            "list-card-clicked".to_string(),
                            "Dashboard".to_string(),
                        );
                        cx.notify();
                    }))
                )
                .child(
                    ListCard::from_spec(
                        ListCardSpec::new()
                            .with_title("Settings")
                            .with_subtitle("Configure preferences")
                            .with_interactive(true),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state.specimens.text.insert(
                            "list-card-clicked".to_string(),
                            "Settings".to_string(),
                        );
                        cx.notify();
                    }))
                )
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            ListCard::from_spec(
                ListCardSpec::new()
                    .with_title("Archived Project")
                    .with_subtitle("No longer accessible")
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
