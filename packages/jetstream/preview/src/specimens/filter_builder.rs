//! FilterBuilder specimen — filter-clause builder with editable pills.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::filter_builder::js_filter_builder;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ControlDensity, ControlSize, FilterBuilderSpec, FilterClause, FilterCombinator, FilterDraft,
    FilterExpression, FilterFieldDefinition, FilterFieldKind, FilterOperand, FilterOption,
};

fn fields() -> Vec<FilterFieldDefinition> {
    vec![
        FilterFieldDefinition::new("format", "Format", FilterFieldKind::MultiEnum).with_options(vec![
            FilterOption::new("clap", "CLAP"),
            FilterOption::new("vst3", "VST3"),
            FilterOption::new("lv2", "LV2"),
        ]),
        FilterFieldDefinition::new("hidden", "Hidden", FilterFieldKind::Boolean),
        FilterFieldDefinition::new("tag-count", "Tag count", FilterFieldKind::Number),
        FilterFieldDefinition::new("name", "Name", FilterFieldKind::Text),
    ]
}

fn value() -> FilterExpression {
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
            FilterClause::new("tag-count-1", "tag-count", "gte", FilterOperand::Number(3.0)),
        ],
    }
}

fn single() -> FilterExpression {
    FilterExpression {
        combinator: FilterCombinator::And,
        clauses: vec![FilterClause::new("hidden-1", "hidden", "is", FilterOperand::Boolean(false))],
    }
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Filter builder (open, Match all)",
            secondary,
            js_filter_builder(
                &FilterBuilderSpec::new()
                    .with_fields(fields())
                    .with_value(value())
                    .with_show_combinator(true)
                    .with_open(true),
                theme,
            ),
        ))
        .child(group(
            "Adding a filter (draft editor)",
            secondary,
            js_filter_builder(
                &FilterBuilderSpec::new()
                    .with_fields(fields())
                    .with_value(value())
                    .with_show_combinator(true)
                    .with_open(true)
                    .with_draft(FilterDraft::adding(&fields()[3])),
                theme,
            ),
        ))
        .child(group(
            "Editing a clause (combinator hidden)",
            secondary,
            js_filter_builder(
                &FilterBuilderSpec::new()
                    .with_fields(fields())
                    .with_value(value())
                    .with_show_combinator(true)
                    .with_open(true)
                    .with_draft(FilterDraft::editing(&value().clauses[0])),
                theme,
            ),
        ))
        .child(group(
            "Empty (open)",
            secondary,
            js_filter_builder(
                &FilterBuilderSpec::new().with_fields(fields()).with_open(true),
                theme,
            ),
        ))
        .child(group(
            "Disabled",
            secondary,
            js_filter_builder(
                &FilterBuilderSpec::new()
                    .with_fields(fields())
                    .with_value(single())
                    .with_disabled(true),
                theme,
            ),
        ))
        .child(group(
            "Sizes (xs–xl)",
            secondary,
            div().flex_col().gap(8.0).children(
                [
                    ControlSize::Xs,
                    ControlSize::Sm,
                    ControlSize::Md,
                    ControlSize::Lg,
                    ControlSize::Xl,
                ]
                .into_iter()
                .map(|size| {
                    js_filter_builder(
                        &FilterBuilderSpec::new()
                            .with_fields(fields())
                            .with_value(single())
                            .with_size(size),
                        theme,
                    )
                }),
            ),
        ))
        .child(group(
            "Densities (compact/default/comfortable)",
            secondary,
            div().flex_col().gap(8.0).children(
                [
                    ControlDensity::Compact,
                    ControlDensity::Default,
                    ControlDensity::Comfortable,
                ]
                .into_iter()
                .map(|density| {
                    js_filter_builder(
                        &FilterBuilderSpec::new()
                            .with_fields(fields())
                            .with_value(single())
                            .with_density(density),
                        theme,
                    )
                }),
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
