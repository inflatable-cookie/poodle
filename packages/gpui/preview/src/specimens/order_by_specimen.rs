use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, OrderBy};
use poodle_specs::{EyebrowSpec, OrderByField, OrderBySpec, SortDirection, SortField};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Sort controls ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Sort controls"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(OrderBy::from_spec(
                            OrderBySpec::new()
                                .with_fields(vec![
                                    SortField::new("name", "Name"),
                                    SortField::new("date", "Date"),
                                    SortField::new("size", "Size"),
                                    SortField::new("type", "Type").with_disabled(true),
                                ])
                                .with_value(vec![
                                    OrderByField::new("name", SortDirection::Asc),
                                    OrderByField::new("date", SortDirection::Desc),
                                ])
                                .with_open(true),
                            theme,
                        ))
                        .child(
                            div()
                                .text_xs()
                                .text_color(color_to_hsla(text_secondary))
                                .child("Sorted by: name (ascending)"),
                        ),
                ),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(OrderBy::from_spec(
                    OrderBySpec::new()
                        .with_fields(vec![
                            SortField::new("name", "Name"),
                            SortField::new("date", "Date"),
                        ])
                        .with_disabled(true)
                        .with_open(true),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "order-by",
        examples,
        |size, theme: &GpuiThemeProvider| {
            OrderBy::from_spec(
                OrderBySpec::new()
                    .with_fields(vec![
                        SortField::new("name", "Name"),
                        SortField::new("date", "Date"),
                    ])
                    .with_value(vec![OrderByField::new("name", SortDirection::Asc)])
                    .with_open(true),
                theme,
            )
            .size(size)
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            OrderBy::from_spec(
                OrderBySpec::new()
                    .with_fields(vec![
                        SortField::new("name", "Name"),
                        SortField::new("date", "Date"),
                    ])
                    .with_value(vec![OrderByField::new("name", SortDirection::Asc)])
                    .with_open(true),
                theme,
            )
            .with_density(density)
            .into_any_element()
        },
    )
}
