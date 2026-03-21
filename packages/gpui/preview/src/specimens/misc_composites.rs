use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{
    PaginationSummarySpec,
    SelectionSummarySpec, SelectionSummaryItem, RemediationAction,
};
use pug_gpui_components::{PaginationSummary, SelectionSummary};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // ── PaginationSummary ──

        // --- Default ---
        .child(section_label("PAGINATION: DEFAULT", text_secondary))
        .child(
            PaginationSummary::from_spec(
                PaginationSummarySpec::new(1, 20, 156),
                theme,
            )
        )
        // --- Single page ---
        .child(section_label("PAGINATION: SINGLE PAGE", text_secondary))
        .child(
            PaginationSummary::from_spec(
                PaginationSummarySpec::new(1, 20, 12),
                theme,
            )
        )
        // --- Large dataset ---
        .child(section_label("PAGINATION: LARGE DATASET", text_secondary))
        .child(
            PaginationSummary::from_spec(
                PaginationSummarySpec::new(5, 20, 1000),
                theme,
            )
        )

        // ── SelectionSummary ──

        // --- Multiple items selected ---
        .child(section_label("SELECTION: MULTIPLE ITEMS SELECTED", text_secondary))
        .child(
            SelectionSummary::from_spec(
                SelectionSummarySpec::new(vec![
                    SelectionSummaryItem::new("btn", "Button"),
                    SelectionSummaryItem::new("card", "Card"),
                    SelectionSummaryItem::new("dialog", "Dialog"),
                    SelectionSummaryItem::new("table", "Table"),
                    SelectionSummaryItem::new("tabs", "Tabs"),
                ])
                .with_clear_action(RemediationAction::new("clear", "Clear")),
                theme,
            )
        )
        // --- Single item ---
        .child(section_label("SELECTION: SINGLE ITEM", text_secondary))
        .child(
            SelectionSummary::from_spec(
                SelectionSummarySpec::new(vec![
                    SelectionSummaryItem::new("primary-btn", "Primary button"),
                ]),
                theme,
            )
        )
        // --- Truncated (max 3 visible) ---
        .child(section_label("SELECTION: TRUNCATED (MAX 3 VISIBLE)", text_secondary))
        .child(
            SelectionSummary::from_spec(
                SelectionSummarySpec::new(vec![
                    SelectionSummaryItem::new("alpha", "Alpha"),
                    SelectionSummaryItem::new("beta", "Beta"),
                    SelectionSummaryItem::new("gamma", "Gamma"),
                    SelectionSummaryItem::new("delta", "Delta"),
                    SelectionSummaryItem::new("epsilon", "Epsilon"),
                    SelectionSummaryItem::new("zeta", "Zeta"),
                ])
                .with_clear_action(RemediationAction::new("clear", "Clear")),
                theme,
            )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
