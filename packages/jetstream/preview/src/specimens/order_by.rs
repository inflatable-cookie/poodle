//! OrderBy specimen — sort control with active/inactive fields.

use crate::compat::js_order_by;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ControlDensity, ControlSize, OrderByField, OrderBySpec, OrderByTriggerVariant, SortDirection,
    SortField,
};

fn fields() -> Vec<SortField> {
    vec![
        SortField::new("title", "Title"),
        SortField::new("kind", "Kind"),
        SortField::new("updated", "Updated").with_default_direction(SortDirection::Desc),
        SortField::new("created", "Created").with_default_direction(SortDirection::Desc),
        SortField::new("visibility", "Visibility").with_disabled(true),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Single-value sort used for the size / density rows.
    let single = vec![OrderByField::new("title", SortDirection::Asc)];

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Multi-field sort builder (compact, open)",
            secondary,
            js_order_by(
                &OrderBySpec::new()
                    .with_fields(fields())
                    .with_value(vec![
                        OrderByField::new("updated", SortDirection::Desc),
                        OrderByField::new("title", SortDirection::Asc),
                    ])
                    .with_compact(true)
                    .with_open(true),
                theme,
                "order-by-1",
            ),
        ))
        .child(group(
            "Icon trigger (open)",
            secondary,
            js_order_by(
                &OrderBySpec::new()
                    .with_fields(fields())
                    .with_value(vec![
                        OrderByField::new("updated", SortDirection::Desc),
                        OrderByField::new("title", SortDirection::Asc),
                    ])
                    .with_trigger_variant(OrderByTriggerVariant::Icon)
                    .with_open(true),
                theme,
                "order-by-2",
            ),
        ))
        .child(group(
            "Empty (open)",
            secondary,
            js_order_by(
                &OrderBySpec::new().with_fields(fields()).with_open(true),
                theme,
                "order-by-3",
            ),
        ))
        .child(group(
            "Disabled",
            secondary,
            js_order_by(
                &OrderBySpec::new()
                    .with_fields(fields())
                    .with_value(single.clone())
                    .with_disabled(true),
                theme,
                "order-by-4",
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
                    js_order_by(
                        &OrderBySpec::new()
                            .with_fields(fields())
                            .with_value(single.clone())
                            .with_size(size),
                        theme,
                        "order-by-5",
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
                    js_order_by(
                        &OrderBySpec::new()
                            .with_fields(fields())
                            .with_value(single.clone())
                            .with_density(density),
                        theme,
                        "order-by-6",
                    )
                }),
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
