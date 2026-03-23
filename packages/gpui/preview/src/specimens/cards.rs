use gpui::*;
use gpui::prelude::FluentBuilder;
use pug_adapter::ThemeProvider;
use pug_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder, ChoiceOption};
use pug_composites::CardRadioGroupSpec;
use pug_gpui_components::{Surface, CardRadioGroup};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
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

        // --- Card radio group: Plan selection ---
        .child(section_label("CARD RADIO GROUP: PLAN SELECTION", text_secondary))
        .child({
            let selected_plan = state.specimens.text.get("card-radio-plan").cloned();
            let plan_items = vec![
                ChoiceOption::new("free", "Free").with_description("Basic features for personal use. Up to 3 projects."),
                ChoiceOption::new("pro", "Pro").with_description("Advanced features for professionals. Unlimited projects."),
                ChoiceOption::new("team", "Team").with_description("Collaboration tools for teams. Shared workspace included."),
                ChoiceOption::new("enterprise", "Enterprise").with_description("Custom solutions for large organizations.").with_disabled(true),
            ];
            let mut spec = CardRadioGroupSpec::new(plan_items)
                .with_value("pro");
            if let Some(ref val) = selected_plan {
                spec = spec.with_value(val);
            }
            div().flex().flex_col().gap(px(4.0))
                .child(
                    CardRadioGroup::from_spec(spec, theme)
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("card-radio-plan".to_string(), val.to_string());
                            cx.notify();
                        }))
                )
                .when(selected_plan.is_some(), |d| {
                    d.child(
                        div().text_xs().text_color(color_to_hsla(text_secondary))
                            .child(format!("Selected: {}", selected_plan.as_deref().unwrap_or("")))
                    )
                })
        })

        // --- Card radio group: Instance size ---
        .child(section_label("CARD RADIO GROUP: INSTANCE SIZE", text_secondary))
        .child({
            let selected_size = state.specimens.text.get("card-radio-size").cloned();
            let size_items = vec![
                ChoiceOption::new("sm", "Small").with_description("1 CPU, 512 MB RAM"),
                ChoiceOption::new("md", "Medium").with_description("2 CPU, 2 GB RAM"),
                ChoiceOption::new("lg", "Large").with_description("4 CPU, 8 GB RAM"),
            ];
            let mut spec = CardRadioGroupSpec::new(size_items);
            if let Some(ref val) = selected_size {
                spec = spec.with_value(val);
            }
            div().flex().flex_col().gap(px(4.0))
                .child(
                    CardRadioGroup::from_spec(spec, theme)
                        .on_change(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("card-radio-size".to_string(), val.to_string());
                            cx.notify();
                        }))
                )
                .when(selected_size.is_some(), |d| {
                    d.child(
                        div().text_xs().text_color(color_to_hsla(text_secondary))
                            .child(format!("Selected: {}", selected_size.as_deref().unwrap_or("")))
                    )
                })
        })

        // --- Card radio group: Disabled ---
        .child(section_label("CARD RADIO GROUP: DISABLED", text_secondary))
        .child(
            CardRadioGroup::from_spec(
                CardRadioGroupSpec::new(vec![
                    ChoiceOption::new("sm", "Small").with_description("1 CPU, 512 MB RAM"),
                    ChoiceOption::new("md", "Medium").with_description("2 CPU, 2 GB RAM"),
                    ChoiceOption::new("lg", "Large").with_description("4 CPU, 8 GB RAM"),
                ])
                .with_value("md")
                .with_disabled(true),
                theme,
            )
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
