use crate::node_compat::{Eyebrow, MediaPicker};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, EyebrowSpec, MediaKind, MediaPickerItem, MediaPickerSpec,
    MediaPickerTab, SemanticControlSizeRole,
};

fn sample_items() -> Vec<MediaPickerItem> {
    vec![
        MediaPickerItem::new("img-1", "Banner image", MediaKind::Image).with_thumbnail(true),
        MediaPickerItem::new("img-2", "Profile photo", MediaKind::Image).with_thumbnail(true),
        MediaPickerItem::new("icon-1", "Icon set", MediaKind::Image),
        MediaPickerItem::new("doc-1", "Readme.pdf", MediaKind::Document),
        MediaPickerItem::new("vid-1", "Demo video", MediaKind::Video),
        MediaPickerItem::new("img-3", "Screenshot", MediaKind::Image).with_thumbnail(true),
    ]
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
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
                    EyebrowSpec::new().with_content("Media picker dialog"),
                    theme,
                ))
                .child(
                    MediaPicker::from_spec(
                        MediaPickerSpec::new("Select an asset").with_open(true),
                        theme,
                    )
                    .with_items(sample_items()),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Upload tab"),
                    theme,
                ))
                .child(
                    MediaPicker::from_spec(
                        MediaPickerSpec::new("Select an asset")
                            .with_open(true)
                            .with_active_tab(MediaPickerTab::Upload)
                            .with_accept("image/*"),
                        theme,
                    )
                    .with_items(sample_items()),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Empty state"),
                    theme,
                ))
                .child(MediaPicker::from_spec(
                    MediaPickerSpec::new("Select an asset")
                        .with_open(true)
                        .with_empty_message("No media items yet."),
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
                        .child(
                            MediaPicker::from_spec(
                                MediaPickerSpec::new("Compact asset picker")
                                    .with_open(true)
                                    .with_size(ControlSize::Sm)
                                    .with_density(ControlDensity::Compact),
                                theme,
                            )
                            .with_items(sample_items()),
                        )
                        .child(
                            MediaPicker::from_spec(
                                MediaPickerSpec::new("Prominent asset picker")
                                    .with_open(true)
                                    .with_size_role(SemanticControlSizeRole::Prominent)
                                    .with_density(ControlDensity::Compact),
                                theme,
                            )
                            .with_items(sample_items()),
                        ),
                ),
        )
}
