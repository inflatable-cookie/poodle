use crate::node_compat::{Eyebrow, MediaBrowsePanel};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::EyebrowSpec;
use poodle_specs::{
    ControlDensity, ControlSize, MediaBrowseItem, MediaBrowsePanelSpec, SemanticControlSizeRole,
};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let items = vec![
        MediaBrowseItem::new("1", "Hero banner", "image").with_meta("Image • 1920 x 1080"),
        MediaBrowseItem::new("2", "Launch trailer", "video").with_meta("Video • 3:42"),
        MediaBrowseItem::new("3", "Podcast intro", "audio").with_meta("Audio • 1:18"),
        MediaBrowseItem::new("4", "Quarterly report", "document").with_meta("Document • PDF"),
    ];

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Browse panel"),
                    theme,
                ))
                .child(MediaBrowsePanel::from_spec(
                    MediaBrowsePanelSpec::new()
                        .with_items(items.clone())
                        .with_has_more(true),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Browse panel states"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(MediaBrowsePanel::from_spec(
                            MediaBrowsePanelSpec::new().with_loading(true),
                            theme,
                        ))
                        .child(MediaBrowsePanel::from_spec(
                            MediaBrowsePanelSpec::new().with_error("Failed to load media"),
                            theme,
                        ))
                        .child(MediaBrowsePanel::from_spec(
                            MediaBrowsePanelSpec::new(),
                            theme,
                        )),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading more"),
                    theme,
                ))
                // Items present + loading + has_more → load-more Button shows
                // "Loading..." and is disabled (contract §4 loading-more posture).
                .child(MediaBrowsePanel::from_spec(
                    MediaBrowsePanelSpec::new()
                        .with_items(items.clone())
                        .with_has_more(true)
                        .with_loading(true),
                    theme,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Semantic presentation"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(MediaBrowsePanel::from_spec(
                            MediaBrowsePanelSpec::new()
                                .with_items(items.clone())
                                .with_has_more(true)
                                .with_size(ControlSize::Sm)
                                .with_density(ControlDensity::Compact),
                            theme,
                        ))
                        .child(MediaBrowsePanel::from_spec(
                            MediaBrowsePanelSpec::new()
                                .with_items(items)
                                .with_has_more(true)
                                .with_size_role(SemanticControlSizeRole::Prominent),
                            theme,
                        )),
                ),
        )
}
