//! BulkActionBar specimen — contextual action bar for bulk selections.
//!
//! Mirrors the GPUI specimen groups (`gpui/preview/src/specimens/bulk_action_bar_specimen.rs`)
//! and contract §13: selection count + total + select-all + clear + a four-tone
//! action set (default / default / danger / warning, each with an icon), a
//! single-item subset, a fully disabled bar, a loading + per-action-disabled
//! bar, plus size and density ladders. The bar — count text, select-all
//! checkbox, clear button, and toned ghost `js_icon_button` actions — is built
//! entirely by `js_bulk_action_bar` from the spec. No hand-rolled chrome.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_bulk_action_bar;

use poodle_specs::{
    BulkAction, BulkActionBarSpec, BulkActionTone, ControlDensity, ControlSize,
};

/// The four-tone demo action set (default, default, danger, warning) shared by
/// several groups and both ladders — mirrors the GPUI `default_actions`.
fn default_actions() -> Vec<BulkAction> {
    vec![
        BulkAction::new("export", "Export").with_icon("download"),
        BulkAction::new("archive", "Archive").with_icon("folder"),
        BulkAction::new("delete", "Delete")
            .with_tone(BulkActionTone::Danger)
            .with_icon("trash-2"),
        BulkAction::new("review", "Review")
            .with_tone(BulkActionTone::Warning)
            .with_icon("circle-alert"),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Selection count + total + select-all + four toned actions.
        .child(group("With selection count and select all", secondary,
            js_bulk_action_bar(
                &BulkActionBarSpec::new()
                    .with_selection_count(5)
                    .with_total_count(42)
                    .with_actions(default_actions())
                    .with_show_select_all(true)
                    .with_all_selected(false),
                theme,
            )
        ))
        // Single item selected — two-action subset, no select-all.
        .child(group("Single item selected", secondary,
            js_bulk_action_bar(
                &BulkActionBarSpec::new()
                    .with_selection_count(1)
                    .with_actions(vec![
                        BulkAction::new("export", "Export").with_icon("download"),
                        BulkAction::new("archive", "Archive").with_icon("folder"),
                    ]),
                theme,
            )
        ))
        // Disabled bar — whole bar dimmed via disabled_opacity.
        .child(group("Disabled bar", secondary,
            js_bulk_action_bar(
                &BulkActionBarSpec::new()
                    .with_selection_count(3)
                    .with_total_count(42)
                    .with_actions(default_actions())
                    .with_show_select_all(true)
                    .with_disabled(true),
                theme,
            )
        ))
        // Loading + per-action-disabled (Delete disabled, all-selected).
        .child(group("Loading and disabled actions", secondary,
            js_bulk_action_bar(
                &BulkActionBarSpec::new()
                    .with_selection_count(12)
                    .with_total_count(12)
                    .with_actions(vec![
                        BulkAction::new("publish", "Publish").with_icon("upload"),
                        BulkAction::new("delete", "Delete")
                            .with_tone(BulkActionTone::Danger)
                            .with_icon("trash-2")
                            .with_disabled(true),
                    ])
                    .with_show_select_all(true)
                    .with_all_selected(true)
                    .with_loading(true),
                theme,
            )
        ))
        // Size ladder — control height + font scale xs..xl.
        .child(group("Size ladder", secondary,
            div().flex_col().gap(16.0)
                .child(size_block(theme, "XS", ControlSize::Xs))
                .child(size_block(theme, "SM", ControlSize::Sm))
                .child(size_block(theme, "MD", ControlSize::Md))
                .child(size_block(theme, "LG", ControlSize::Lg))
                .child(size_block(theme, "XL", ControlSize::Xl))
        ))
        // Density ladder — inter-element gaps tighten/loosen, height unchanged.
        .child(group("Density ladder", secondary,
            div().flex_col().gap(16.0)
                .child(density_block(theme, "COMPACT", ControlDensity::Compact))
                .child(density_block(theme, "DEFAULT", ControlDensity::Default))
                .child(density_block(theme, "COMFORTABLE", ControlDensity::Comfortable))
        ))
}

fn size_block(theme: &JetstreamThemeProvider, lbl: &str, size: ControlSize) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    ladder(lbl, secondary,
        js_bulk_action_bar(
            &BulkActionBarSpec::new()
                .with_selection_count(5)
                .with_actions(default_actions())
                .with_size(size),
            theme,
        ))
}

fn density_block(theme: &JetstreamThemeProvider, lbl: &str, density: ControlDensity) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    ladder(lbl, secondary,
        js_bulk_action_bar(
            &BulkActionBarSpec::new()
                .with_selection_count(5)
                .with_actions(default_actions())
                .with_density(density),
            theme,
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}

fn ladder(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0).text_weight(700))
        .child(content)
}
