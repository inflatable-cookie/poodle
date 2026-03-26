use gpui::*;
use poodle_composites::{MediaBrowsePanelSpec, MediaBrowseItem};
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{MediaBrowsePanel, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let items = vec![
        MediaBrowseItem::new("1", "Hero banner", "image").with_meta("Image"),
        MediaBrowseItem::new("2", "Launch trailer", "video").with_meta("Video"),
        MediaBrowseItem::new("3", "Podcast intro", "audio").with_meta("Audio"),
        MediaBrowseItem::new("4", "Quarterly report", "document").with_meta("Document"),
    ];

    div().flex().flex_col().gap(px(24.0))
        // --- Browse panel ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Browse panel"), theme))
                .child(
                    MediaBrowsePanel::from_spec(
                        MediaBrowsePanelSpec::new()
                            .with_items(items.clone())
                            .with_has_more(true),
                        theme,
                    )
                )
        )
        // --- Semantic presentation ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Semantic presentation"), theme))
                .child(
                    div().flex().flex_col().gap(px(12.0))
                        .child(
                            MediaBrowsePanel::from_spec(
                                MediaBrowsePanelSpec::new()
                                    .with_items(items.clone())
                                    .with_has_more(true),
                                theme,
                            )
                        )
                        .child(
                            MediaBrowsePanel::from_spec(
                                MediaBrowsePanelSpec::new()
                                    .with_items(items)
                                    .with_has_more(true),
                                theme,
                            )
                        )
                )
        )
}
