use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, OrderBySpec, SortField, ActiveSort, SortDirection, EyebrowSpec};
use poodle_gpui_components::{OrderBy, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Sort controls ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sort controls"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            OrderBy::from_spec(
                                OrderBySpec::new()
                                    .with_fields(vec![
                                        SortField::new("name", "Name"),
                                        SortField::new("date", "Date"),
                                        SortField::new("size", "Size"),
                                        SortField::new("type", "Type").with_disabled(true),
                                    ])
                                    .with_active_sort(ActiveSort::new("name", SortDirection::Asc)),
                                theme,
                            )
                        )
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child("Sorted by: name (ascending)")
                        )
                )
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child({
                    let make_spec = || OrderBySpec::new()
                        .with_fields(vec![
                            SortField::new("name", "Name"),
                            SortField::new("date", "Date"),
                        ])
                        .with_active_sort(ActiveSort::new("name", SortDirection::Asc));
                    div().flex().flex_col().gap(px(8.0))
                        .child(OrderBy::from_spec(make_spec(), theme).size(ControlSize::Xs))
                        .child(OrderBy::from_spec(make_spec(), theme).size(ControlSize::Sm))
                        .child(OrderBy::from_spec(make_spec(), theme).size(ControlSize::Md))
                        .child(OrderBy::from_spec(make_spec(), theme).size(ControlSize::Lg))
                        .child(OrderBy::from_spec(make_spec(), theme).size(ControlSize::Xl))
                })
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child({
                    let make_spec = || OrderBySpec::new()
                        .with_fields(vec![
                            SortField::new("name", "Name"),
                            SortField::new("date", "Date"),
                        ])
                        .with_active_sort(ActiveSort::new("name", SortDirection::Asc));
                    div().flex().flex_col().gap(px(8.0))
                        .child(OrderBy::from_spec(make_spec(), theme).with_density(ControlDensity::Compact))
                        .child(OrderBy::from_spec(make_spec(), theme).with_density(ControlDensity::Default))
                        .child(OrderBy::from_spec(make_spec(), theme).with_density(ControlDensity::Comfortable))
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    OrderBy::from_spec(
                        OrderBySpec::new()
                            .with_fields(vec![
                                SortField::new("name", "Name"),
                                SortField::new("date", "Date"),
                            ])
                            .with_disabled(true),
                        theme,
                    )
                )
        )
}
