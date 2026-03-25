use gpui::*;
use poodle_composites::MediaPickerSpec;
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{MediaPicker, MediaPickerItem, Eyebrow};
use poodle_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div().flex().flex_col().gap(px(24.0))
        .child(div().flex().flex_col().gap(px(8.0))
            .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Browse tab"), theme))
            .child(
                MediaPicker::from_spec(
                    MediaPickerSpec::new("Select media").with_open(true),
                    theme,
                )
                .with_thumbnails(vec![
                    MediaPickerItem { id: "img-1".to_string(), label: "Banner image".to_string(), is_selected: false },
                    MediaPickerItem { id: "img-2".to_string(), label: "Profile photo".to_string(), is_selected: true },
                    MediaPickerItem { id: "img-3".to_string(), label: "Icon set".to_string(), is_selected: false },
                    MediaPickerItem { id: "doc-1".to_string(), label: "Readme.pdf".to_string(), is_selected: false },
                    MediaPickerItem { id: "vid-1".to_string(), label: "Demo video".to_string(), is_selected: false },
                    MediaPickerItem { id: "img-4".to_string(), label: "Screenshot".to_string(), is_selected: false },
                ])))
}
