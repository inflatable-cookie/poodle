use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, MediaBrowsePanel};
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
                    EyebrowSpec::new().with_content("State variants"),
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
                            MediaBrowsePanelSpec::new()
                                .with_error("Unable to load media results. Check your connection."),
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
