use crate::node_compat::{BulkActionBar, Eyebrow};
use crate::style_bridge::color_to_hsla;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    BulkAction, BulkActionBarSpec, BulkActionTone, ControlDensity, ControlSize, EyebrowSpec,
};

/// The four-tone demo action set (default, default, danger, warning) shared by
/// several specimen groups and both ladders.
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

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- With selection count and select all ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With selection count and select all"),
                    theme,
                ))
                .child(BulkActionBar::from_spec(
                    BulkActionBarSpec::new()
                        .with_selection_count(5)
                        .with_total_count(42)
                        .with_actions(default_actions())
                        .with_show_select_all(true)
                        .with_all_selected(false),
                    theme,
                )),
        )
        // --- Single item selected ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Single item selected"),
                    theme,
                ))
                .child(BulkActionBar::from_spec(
                    BulkActionBarSpec::new()
                        .with_selection_count(1)
                        .with_actions(vec![
                            BulkAction::new("export", "Export").with_icon("download"),
                            BulkAction::new("archive", "Archive").with_icon("folder"),
                        ]),
                    theme,
                )),
        )
        // --- Disabled bar ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled bar"),
                    theme,
                ))
                .child(BulkActionBar::from_spec(
                    BulkActionBarSpec::new()
                        .with_selection_count(3)
                        .with_total_count(42)
                        .with_actions(default_actions())
                        .with_show_select_all(true)
                        .with_disabled(true),
                    theme,
                )),
        )
        // --- Loading and disabled actions ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading and disabled actions"),
                    theme,
                ))
                .child(BulkActionBar::from_spec(
                    BulkActionBarSpec::new()
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
                )),
        )
        // --- Size ladder ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Size ladder"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .child(size_block(theme, "XS", ControlSize::Xs))
                        .child(size_block(theme, "SM", ControlSize::Sm))
                        .child(size_block(theme, "MD", ControlSize::Md))
                        .child(size_block(theme, "LG", ControlSize::Lg))
                        .child(size_block(theme, "XL", ControlSize::Xl)),
                ),
        )
        // --- Density ladder ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Density ladder"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .child(density_block(theme, "COMPACT", ControlDensity::Compact))
                        .child(density_block(theme, "DEFAULT", ControlDensity::Default))
                        .child(density_block(
                            theme,
                            "COMFORTABLE",
                            ControlDensity::Comfortable,
                        )),
                ),
        )
}

/// Label above a ladder entry (mirrors the Svelte specimen variant labels).
fn ladder_label(theme: &GpuiThemeProvider, label: &str) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .text_color(color_to_hsla(theme.resolve_color("color.text.muted")))
        .child(label.to_string())
}

fn size_block(theme: &GpuiThemeProvider, label: &str, size: ControlSize) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(ladder_label(theme, label))
        .child(BulkActionBar::from_spec(
            BulkActionBarSpec::new()
                .with_selection_count(5)
                .with_actions(default_actions())
                .with_size(size),
            theme,
        ))
}

fn density_block(theme: &GpuiThemeProvider, label: &str, density: ControlDensity) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(ladder_label(theme, label))
        .child(BulkActionBar::from_spec(
            BulkActionBarSpec::new()
                .with_selection_count(5)
                .with_actions(default_actions())
                .with_density(density),
            theme,
        ))
}
