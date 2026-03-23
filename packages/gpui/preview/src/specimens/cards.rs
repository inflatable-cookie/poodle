use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{SurfaceSpec, SurfaceTone, SurfaceBorder, ChoiceOption, EyebrowSpec};
use poodle_composites::CardRadioGroupSpec;
use poodle_gpui_components::{Surface, CardRadioGroup, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("semantic.color.text.primary");
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let divider_color = theme.resolve_color("semantic.color.border.subtle");

    div().flex().flex_col().gap(px(24.0))
        // --- Default variant ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default variant"), theme))
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
                                                .child("A design system component library for building consistent interfaces.".to_string()),
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
                                                .child("48 components shipped across 3 packages this month.".to_string()),
                                        ),
                                ),
                            ),
                        ),
                )
        )
        // --- Outlined variant ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Outlined variant"), theme))
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
                                        .child("Outlined card".to_string()),
                                )
                                .child(
                                    div().text_sm().text_color(color_to_hsla(text_secondary))
                                        .child("This card uses a subtle border instead of elevation.".to_string()),
                                ),
                        ),
                    ),
                )
        )
        // --- Elevated variant ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Elevated variant"), theme))
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
                                        .child("Elevated card".to_string()),
                                )
                                .child(
                                    div().text_sm().text_color(color_to_hsla(text_secondary))
                                        .child("This card uses a drop shadow for visual prominence.".to_string()),
                                ),
                        ),
                    ),
                )
        )
        // --- Interactive ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Interactive"), theme))
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
                                            .child("Interactive card".to_string()),
                                    )
                                    .child(
                                        div().text_sm().text_color(color_to_hsla(text_secondary))
                                            .child("Hover to see the interactive state. Cursor changes to pointer.".to_string()),
                                    ),
                            ),
                        ),
                )
        )

        // --- Card radio group: Plan selection ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Card radio group: Plan selection"), theme))
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
        )

        // --- Card radio group: Instance size ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Card radio group: Instance size"), theme))
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
        )

        // --- Card radio group: Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Card radio group: Disabled"), theme))
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
        )
}
