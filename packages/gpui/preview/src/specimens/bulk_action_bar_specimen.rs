use gpui::*;
use poodle_components::{BulkActionBarSpec, BulkAction, BulkActionTone, EyebrowSpec};
use poodle_gpui_components::{BulkActionBar, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let default_actions = || vec![
        BulkAction::new("export", "Export"),
        BulkAction::new("archive", "Archive"),
        BulkAction::new("delete", "Delete").with_tone(BulkActionTone::Danger),
        BulkAction::new("review", "Review").with_tone(BulkActionTone::Warning),
    ];

    div().flex().flex_col().gap(px(24.0))
        // --- With selection count and select all ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With selection count and select all"), theme))
                .child(
                    BulkActionBar::from_spec(
                        BulkActionBarSpec::new()
                            .with_selection_count(5)
                            .with_total_count(42)
                            .with_actions(default_actions())
                            .with_show_select_all(true)
                            .with_all_selected(false),
                        theme,
                    )
                )
        )
        // --- Single item selected ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Single item selected"), theme))
                .child(
                    BulkActionBar::from_spec(
                        BulkActionBarSpec::new()
                            .with_selection_count(1)
                            .with_actions(vec![
                                BulkAction::new("export", "Export"),
                                BulkAction::new("archive", "Archive"),
                            ]),
                        theme,
                    )
                )
        )
        // --- Loading and disabled actions ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Loading and disabled actions"), theme))
                .child(
                    BulkActionBar::from_spec(
                        BulkActionBarSpec::new()
                            .with_selection_count(12)
                            .with_total_count(12)
                            .with_actions(vec![
                                BulkAction::new("publish", "Publish"),
                                BulkAction::new("delete", "Delete")
                                    .with_tone(BulkActionTone::Danger)
                                    .with_disabled(true),
                            ])
                            .with_show_select_all(true)
                            .with_all_selected(true)
                            .with_loading(true),
                        theme,
                    )
                )
        )
}
