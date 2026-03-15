use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_composites::{
    PaginationSummarySpec,
    SelectionSummarySpec, SelectionSummaryItem, RemediationAction,
};
use pug_gpui_components::{PugPaginationSummary, PugSelectionSummary};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // Filter toolbar placeholder
    let filter_row = div().flex().items_center().gap(px(6.0))
        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Filters: 2 active"))
        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("|"))
        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("8 results"));

    // Pagination summary
    let pagination = PaginationSummarySpec::new(1, 10, 42);

    // Selection summary
    let selection = SelectionSummarySpec::new(vec![
        SelectionSummaryItem::new("slug-1", "my-project-slug"),
        SelectionSummaryItem::new("slug-2", "another-slug"),
    ])
    .with_clear_action(RemediationAction::new("clear", "Clear"));

    div().flex().flex_col().gap(px(10.0))
        .child(filter_row)
        .child(PugPaginationSummary::new(pagination, theme))
        .child(PugSelectionSummary::new(selection, theme))
}
