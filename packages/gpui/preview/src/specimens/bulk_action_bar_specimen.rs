use crate::app_state::AppState;
use crate::node_compat::{BulkActionBar, Eyebrow};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
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

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
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
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "bulk-action-bar",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                size_block(theme, size).into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                density_block(theme, density).into_any_element()
            }),
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

fn size_block(theme: &GpuiThemeProvider, size: ControlSize) -> Div {
    let label = size_key(size).to_uppercase();
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(ladder_label(theme, &label))
        .child(BulkActionBar::from_spec(
            BulkActionBarSpec::new()
                .with_selection_count(5)
                .with_actions(default_actions())
                .with_size(size),
            theme,
        ))
}

fn density_block(theme: &GpuiThemeProvider, density: ControlDensity) -> Div {
    let label = density_key(density).to_uppercase();
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(ladder_label(theme, &label))
        .child(BulkActionBar::from_spec(
            BulkActionBarSpec::new()
                .with_selection_count(5)
                .with_actions(default_actions())
                .with_density(density),
            theme,
        ))
}
