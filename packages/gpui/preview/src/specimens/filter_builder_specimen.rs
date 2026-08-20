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
                FilterOption::new("au", "AU"),
                FilterOption::new("vst2", "VST2"),
            ],
        ),
        FilterFieldDefinition::new("category", "Category", FilterFieldKind::Enum).with_options(
            vec![
                FilterOption::new("effect", "Effect"),
                FilterOption::new("instrument", "Instrument"),
                FilterOption::new("midi", "MIDI"),
            ],
        ),
        FilterFieldDefinition::new("name", "Name", FilterFieldKind::Text),
        FilterFieldDefinition::new("tag-count", "Tag count", FilterFieldKind::Number),
        FilterFieldDefinition::new("rating", "Rating", FilterFieldKind::Range),
        FilterFieldDefinition::new("hidden", "Hidden", FilterFieldKind::Boolean),
        FilterFieldDefinition::new("tag", "Tag", FilterFieldKind::MultiEnum)
            .with_allow_multiple(true)
            .with_options(vec![
                FilterOption::new("compressor", "Compressor"),
                FilterOption::new("mastering", "Mastering"),
                FilterOption::new("reverb", "Reverb"),
            ]),
    ]
}

fn overflow_value() -> FilterExpression {
    FilterExpression {
        combinator: FilterCombinator::And,
        clauses: vec![
            FilterClause::new(
                "format-1",
                "format",
                "any_of",
                FilterOperand::Options(vec![
                    "clap".into(),
                    "vst3".into(),
                    "lv2".into(),
                    "au".into(),
                ]),
            ),
            FilterClause::new(
                "category-1",
                "category",
                "is",
                FilterOperand::Options(vec!["effect".into()]),
            ),
            FilterClause::new("hidden-1", "hidden", "is", FilterOperand::Boolean(true)),
            FilterClause::new(
                "tag-count-1",
                "tag-count",
                "gte",
                FilterOperand::Number(2.0),
            ),
            FilterClause::new(
                "rating-1",
                "rating",
                "between",
                FilterOperand::Range {
                    min: Some(3.0),
                    max: Some(5.0),
                },
            ),
            FilterClause::new(
                "tag-1",
                "tag",
                "all_of",
                FilterOperand::Options(vec!["mastering".into()]),
            ),
            FilterClause::new(
                "tag-2",
                "tag",
                "none_of",
                FilterOperand::Options(vec!["reverb".into()]),
            ),
        ],
    }
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

    let group = |title: &'static str, body: AnyElement| {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Eyebrow::from_spec(
                EyebrowSpec::new().with_content(title),
                theme,
            ))
            .child(body)
    };

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Building filters",
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
                )
                .into_any_element(),
        ))
        .child(group(
            "Match all and match any",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(FilterBuilder::from_spec(
                    FilterBuilderSpec::new()
                        .with_fields(demo_fields())
                        .with_value(demo_value())
                        .with_show_combinator(true)
                        .with_open(true)
                        .with_draft(FilterDraft::editing(&demo_value().clauses[0])),
                    theme,
                ))
                .child(FilterBuilder::from_spec(
                    FilterBuilderSpec::new()
                        .with_fields(demo_fields())
                        .with_value(FilterExpression {
                            combinator: FilterCombinator::Or,
                            clauses: demo_value().clauses,
                        })
                        .with_show_combinator(true)
                        .with_open(true),
                    theme,
                ))
                .into_any_element(),
        ))
        .child(group(
            "Empty and limited builders",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(FilterBuilder::from_spec(
                    FilterBuilderSpec::new()
                        .with_fields(demo_fields())
                        .with_value(FilterExpression {
                            combinator: FilterCombinator::And,
                            clauses: vec![],
                        })
                        .with_open(true),
                    theme,
                ))
                .child(FilterBuilder::from_spec(
                    FilterBuilderSpec::new()
                        .with_fields(demo_fields())
                        .with_value(FilterExpression {
                            combinator: FilterCombinator::And,
                            clauses: vec![
                                FilterClause::new(
                                    "hidden-1",
                                    "hidden",
                                    "is",
                                    FilterOperand::Boolean(true),
                                ),
                                FilterClause::new(
                                    "name-1",
                                    "name",
                                    "contains",
                                    FilterOperand::Text("bus".into()),
                                ),
                            ],
                        })
                        .with_max_clauses(2)
                        .with_open(true),
                    theme,
                ))
                .into_any_element(),
        ))
        .child(group(
            "Field types and overflow",
            FilterBuilder::from_spec(
                FilterBuilderSpec::new()
                    .with_fields(demo_fields())
                    .with_value(overflow_value())
                    .with_open(true),
                theme,
            )
            .into_any_element(),
        ))
        .child(group(
            "Disabled",
            FilterBuilder::from_spec(
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
            )
            .into_any_element(),
        ))
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
