use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::Surface;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let divider_color = theme.resolve_color("semantic.color.border.subtle");

    div().flex().flex_col().gap(px(16.0))
        // --- Default variant ---
        .child(section_label("DEFAULT VARIANT", text_secondary))
        .child(
            div().flex().gap(px(16.0)).flex_wrap()
                // Card 1: Project Alpha with footer
                .child(
                    div().w(px(280.0)).child(
                        Surface::from_spec(
                            SurfaceSpec::new()
                                .with_tone(SurfaceTone::Panel)
                                .with_border(SurfaceBorder::Subtle),
                            theme,
                        )
                        .with_content(
                            div().flex().flex_col().gap(px(12.0))
                                .child(
                                    div().text_base().font_weight(FontWeight::SEMIBOLD)
                                        .text_color(color_to_hsla(text_primary))
                                        .child("Project Alpha".to_string()),
                                )
                                .child(
                                    div().text_sm().text_color(color_to_hsla(text_secondary))
                                        .child("A collaborative workspace for your team to plan, build, and ship products.".to_string()),
                                )
                                .child(
                                    div()
                                        .pt(px(8.0))
                                        .border_color(color_to_hsla(divider_color).opacity(0.52))
                                        .border_t_1()
                                        .child(
                                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                                .child("Updated 2 days ago".to_string()),
                                        ),
                                ),
                        ),
                    ),
                )
                // Card 2: Monthly report without footer
                .child(
                    div().w(px(280.0)).child(
                        Surface::from_spec(
                            SurfaceSpec::new()
                                .with_tone(SurfaceTone::Panel)
                                .with_border(SurfaceBorder::Subtle),
                            theme,
                        )
                        .with_content(
                            div().flex().flex_col().gap(px(12.0))
                                .child(
                                    div().text_base().font_weight(FontWeight::SEMIBOLD)
                                        .text_color(color_to_hsla(text_primary))
                                        .child("Monthly report".to_string()),
                                )
                                .child(
                                    div().text_sm().text_color(color_to_hsla(text_secondary))
                                        .child("Revenue grew 12% month-over-month with improved conversion rates.".to_string()),
                                ),
                        ),
                    ),
                ),
        )
        // --- Outlined variant ---
        .child(section_label("OUTLINED VARIANT", text_secondary))
        .child(
            div().w(px(280.0)).child(
                Surface::from_spec(
                    SurfaceSpec::new()
                        .with_tone(SurfaceTone::Panel)
                        .with_border(SurfaceBorder::Default),
                    theme,
                )
                .with_content(
                    div().flex().flex_col().gap(px(12.0))
                        .child(
                            div().text_base().font_weight(FontWeight::SEMIBOLD)
                                .text_color(color_to_hsla(text_primary))
                                .child("Settings".to_string()),
                        )
                        .child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("Configure your workspace preferences and notification settings.".to_string()),
                        ),
                ),
            ),
        )
        // --- Elevated variant ---
        .child(section_label("ELEVATED VARIANT", text_secondary))
        .child(
            div().w(px(280.0)).child(
                Surface::from_spec(
                    SurfaceSpec::new()
                        .with_tone(SurfaceTone::Elevated)
                        .with_border(SurfaceBorder::Subtle)
                        .with_elevation(true),
                    theme,
                )
                .with_content(
                    div().flex().flex_col().gap(px(12.0))
                        .child(
                            div().text_base().font_weight(FontWeight::SEMIBOLD)
                                .text_color(color_to_hsla(text_primary))
                                .child("Dashboard".to_string()),
                        )
                        .child(
                            div().text_sm().text_color(color_to_hsla(text_secondary))
                                .child("View real-time metrics and performance indicators.".to_string()),
                        ),
                ),
            ),
        )
        // --- Interactive ---
        .child(section_label("INTERACTIVE", text_secondary))
        .child(
            div().w(px(280.0))
                .id("interactive-card")
                .cursor_pointer()
                .hover(|s| s.opacity(0.9))
                .child(
                    Surface::from_spec(
                        SurfaceSpec::new()
                            .with_tone(SurfaceTone::Panel)
                            .with_border(SurfaceBorder::Subtle),
                        theme,
                    )
                    .with_content(
                        div().flex().flex_col().gap(px(12.0))
                            .child(
                                div().text_base().font_weight(FontWeight::SEMIBOLD)
                                    .text_color(color_to_hsla(text_primary))
                                    .child("Learn more".to_string()),
                            )
                            .child(
                                div().text_sm().text_color(color_to_hsla(text_secondary))
                                    .child("Click to explore documentation and guides.".to_string()),
                            ),
                    ),
                ),
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
