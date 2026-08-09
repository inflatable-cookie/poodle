//! MediaPicker specimen — media asset selection dialog (single-select).

use crate::compat::js_media_picker;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{MediaKind, MediaPickerItem, MediaPickerSpec, MediaPickerTab};

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

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Browse tab",
            secondary,
            js_media_picker(
                &MediaPickerSpec::new("Select an asset")
                    .with_open(true)
                    .with_items(sample_items()),
                theme,
            ),
        ))
        .child(group(
            "Upload tab",
            secondary,
            js_media_picker(
                &MediaPickerSpec::new("Select an asset")
                    .with_open(true)
                    .with_active_tab(MediaPickerTab::Upload)
                    .with_accept("image/*")
                    .with_items(sample_items()),
                theme,
            ),
        ))
        .child(group(
            "Empty state",
            secondary,
            js_media_picker(
                &MediaPickerSpec::new("Select an asset")
                    .with_open(true)
                    .with_empty_message("No media items yet."),
                theme,
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
