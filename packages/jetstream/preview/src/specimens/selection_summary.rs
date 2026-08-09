//! SelectionSummary specimen — summary of selected items.

use crate::compat::js_selection_summary;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    ControlDensity, ControlSize, RemediationAction, SelectionSummaryItem, SelectionSummarySpec,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    // Plain {id, label} items — matches Svelte authority (no `meta`; anatomy is
    // ChipLabel + RemoveIcon only per contract §2).
    let items = vec![
        SelectionSummaryItem::new("1", "Button"),
        SelectionSummaryItem::new("2", "Card"),
        SelectionSummaryItem::new("3", "Dialog"),
        SelectionSummaryItem::new("4", "Table"),
        SelectionSummaryItem::new("5", "Tabs"),
    ];

    div()
        .flex_col()
        .gap(24.0)
        // Multiple items selected — chip row IS the count summary; clear control present
        .child(group(
            "Multiple items selected",
            secondary,
            js_selection_summary(
                &SelectionSummarySpec::new(items.clone())
                    .with_clear_action(RemediationAction::new("clear", "Clear")),
                theme,
            ),
        ))
        // Empty state — reserved-height container, "No selection" placeholder, no clear
        .child(group(
            "Empty state",
            secondary,
            js_selection_summary(&SelectionSummarySpec::new(vec![]), theme),
        ))
        // Single item
        .child(group(
            "Single item",
            secondary,
            js_selection_summary(
                &SelectionSummarySpec::new(vec![SelectionSummaryItem::new("1", "Primary button")])
                    .with_clear_action(RemediationAction::new("clear", "Clear")),
                theme,
            ),
        ))
        // Truncated (max 3 visible) — "+3 more" overflow + clear link
        .child(group(
            "Truncated (max 3 visible)",
            secondary,
            js_selection_summary(
                &SelectionSummarySpec::new(vec![
                    SelectionSummaryItem::new("a", "Alpha"),
                    SelectionSummaryItem::new("b", "Beta"),
                    SelectionSummaryItem::new("c", "Gamma"),
                    SelectionSummaryItem::new("d", "Delta"),
                    SelectionSummaryItem::new("e", "Epsilon"),
                    SelectionSummaryItem::new("f", "Zeta"),
                ])
                .with_max_visible_items(3)
                .with_clear_action(RemediationAction::new("clear", "Clear")),
                theme,
            ),
        ))
        // Sizes — full 5-step ladder (contract §8)
        .child(group(
            "Sizes",
            secondary,
            div().flex_col().gap(12.0).children(
                [
                    ControlSize::Xs,
                    ControlSize::Sm,
                    ControlSize::Md,
                    ControlSize::Lg,
                    ControlSize::Xl,
                ]
                .into_iter()
                .map(|size| {
                    js_selection_summary(
                        &SelectionSummarySpec::new(items[..3].to_vec())
                            .with_size(size)
                            .with_clear_action(RemediationAction::new("clear", "Clear")),
                        theme,
                    )
                }),
            ),
        ))
        // Densities — compact / default / comfortable (contract §8)
        .child(group(
            "Densities",
            secondary,
            div().flex_col().gap(12.0).children(
                [
                    ControlDensity::Compact,
                    ControlDensity::Default,
                    ControlDensity::Comfortable,
                ]
                .into_iter()
                .map(|density| {
                    js_selection_summary(
                        &SelectionSummarySpec::new(items[..3].to_vec())
                            .with_density(density)
                            .with_clear_action(RemediationAction::new("clear", "Clear")),
                        theme,
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
