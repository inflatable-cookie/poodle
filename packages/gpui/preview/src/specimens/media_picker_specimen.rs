use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Eyebrow, MediaPicker, MediaPickerItem};
use poodle_specs::MediaPickerSpec;
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec, SemanticControlSizeRole};

fn sample_items() -> Vec<MediaPickerItem> {
    vec![
        MediaPickerItem {
            id: "img-1".to_string(),
            label: "Banner image".to_string(),
            is_selected: false,
        },
        MediaPickerItem {
            id: "img-2".to_string(),
            label: "Profile photo".to_string(),
            is_selected: true,
        },
        MediaPickerItem {
            id: "img-3".to_string(),
            label: "Icon set".to_string(),
            is_selected: false,
        },
        MediaPickerItem {
            id: "doc-1".to_string(),
            label: "Readme.pdf".to_string(),
            is_selected: false,
        },
        MediaPickerItem {
            id: "vid-1".to_string(),
            label: "Demo video".to_string(),
            is_selected: false,
        },
        MediaPickerItem {
            id: "img-4".to_string(),
            label: "Screenshot".to_string(),
            is_selected: false,
        },
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
                    .with_thumbnails(sample_items()),
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
                        .child(
                            MediaPicker::from_spec(
                                MediaPickerSpec::new("Compact asset picker")
                                    .with_open(true)
                                    .with_size(ControlSize::Sm)
                                    .with_density(ControlDensity::Compact),
                                theme,
                            )
                            .with_thumbnails(sample_items()),
                        )
                        .child(
                            MediaPicker::from_spec(
                                MediaPickerSpec::new("Prominent asset picker")
                                    .with_open(true)
                                    .with_size_role(SemanticControlSizeRole::Prominent)
                                    .with_density(ControlDensity::Compact),
                                theme,
                            )
                            .with_thumbnails(sample_items()),
                        ),
                ),
        )
}
