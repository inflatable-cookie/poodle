use gpui::*;
use poodle_primitives::{BulkActionBarSpec, BulkAction, BulkActionTone, EyebrowSpec};
use poodle_gpui_components::{BulkActionBar, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With selection count"), theme))
                .child(
                    BulkActionBar::from_spec(
                        BulkActionBarSpec::new()
                            .with_selection_count(5)
                            .with_total_count(42)
                            .with_actions(vec![
                                BulkAction::new("export", "Export"),
                                BulkAction::new("archive", "Archive"),
                                BulkAction::new("delete", "Delete").with_tone(BulkActionTone::Danger),
                            ]),
                        theme,
                    )
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Single selection"), theme))
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
}
