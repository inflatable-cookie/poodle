use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, FilterBuilder};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    EyebrowSpec, FilterBuilderSpec, FilterClause, FilterCombinator, FilterDraft, FilterExpression,
    FilterFieldDefinition, FilterFieldKind, FilterOperand, FilterOption,
};

fn demo_fields() -> Vec<FilterFieldDefinition> {
    vec![
        FilterFieldDefinition::new("format", "Format", FilterFieldKind::MultiEnum).with_options(
            vec![
                FilterOption::new("clap", "CLAP"),
                FilterOption::new("vst3", "VST3"),
                FilterOption::new("lv2", "LV2"),
            ],
        ),
        FilterFieldDefinition::new("hidden", "Hidden", FilterFieldKind::Boolean),
        FilterFieldDefinition::new("tag-count", "Tag count", FilterFieldKind::Number),
        FilterFieldDefinition::new("name", "Name", FilterFieldKind::Text),
    ]
}

fn demo_value() -> FilterExpression {
    FilterExpression {
        combinator: FilterCombinator::And,
        clauses: vec![
            FilterClause::new(
                "format-1",
                "format",
                "any_of",
                FilterOperand::Options(vec!["clap".into(), "vst3".into()]),
            ),
            FilterClause::new("hidden-1", "hidden", "is", FilterOperand::Boolean(false)),
            FilterClause::new(
                "tag-count-1",
                "tag-count",
                "gte",
                FilterOperand::Number(3.0),
            ),
        ],
    }
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Filter builder"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(FilterBuilder::from_spec(
                            FilterBuilderSpec::new()
                                .with_fields(demo_fields())
                                .with_value(demo_value())
                                .with_show_combinator(true)
                                .with_open(true),
                            theme,
                        ))
                        .child(
                            div()
                                .text_xs()
                                .text_color(color_to_hsla(text_secondary))
                                .child("3 active clauses, Match all"),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Adding a filter (draft editor)"),
                    theme,
                ))
                .child(FilterBuilder::from_spec(
                    FilterBuilderSpec::new()
                        .with_fields(demo_fields())
                        .with_value(demo_value())
                        .with_show_combinator(true)
                        .with_open(true)
                        .with_draft(FilterDraft::adding(&demo_fields()[3])),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Editing a clause (combinator hidden)"),
                    theme,
                ))
                .child(FilterBuilder::from_spec(
                    FilterBuilderSpec::new()
                        .with_fields(demo_fields())
                        .with_value(demo_value())
                        .with_show_combinator(true)
                        .with_open(true)
                        .with_draft(FilterDraft::editing(&demo_value().clauses[0])),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(FilterBuilder::from_spec(
                    FilterBuilderSpec::new()
                        .with_fields(demo_fields())
                        .with_value(FilterExpression {
                            combinator: FilterCombinator::And,
                            clauses: vec![FilterClause::new(
                                "hidden-1",
                                "hidden",
                                "is",
                                FilterOperand::Boolean(false),
                            )],
                        })
                        .with_disabled(true)
                        .with_open(true),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "filter-builder",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                FilterBuilder::from_spec(
                    FilterBuilderSpec::new()
                        .with_fields(demo_fields())
                        .with_value(demo_value())
                        .with_open(true),
                    theme,
                )
                .size(size)
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                FilterBuilder::from_spec(
                    FilterBuilderSpec::new()
                        .with_fields(demo_fields())
                        .with_value(demo_value())
                        .with_open(true),
                    theme,
                )
                .with_density(density)
                .into_any_element()
            }),
    )
}
