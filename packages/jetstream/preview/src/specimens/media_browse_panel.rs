//! MediaBrowsePanel specimen — grid of selectable media items.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::media_browse_panel::js_media_browse_panel;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlDensity, ControlSize, MediaBrowseItem, MediaBrowsePanelSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let items = vec![
        MediaBrowseItem::new("a", "Approval still", "image").with_meta("1920x1080"),
        MediaBrowseItem::new("b", "Stem waveform", "audio").with_meta("WAV 48kHz"),
        MediaBrowseItem::new("c", "Final render", "video").with_meta("4K MP4"),
        MediaBrowseItem::new("d", "Thumbnail set", "image").with_meta("512x512"),
    ];

    div().flex_col().gap(24.0)
        .child(group("With items", secondary,
            js_media_browse_panel(
                &MediaBrowsePanelSpec::new()
                    .with_items(items.clone())
                    .with_has_more(true),
                theme,
            )
        ))
        .child(group("Loading more", secondary,
            js_media_browse_panel(
                &MediaBrowsePanelSpec::new()
                    .with_items(items.clone())
                    .with_has_more(true)
                    .with_loading(true),
                theme,
            )
        ))
        .child(group("Loading", secondary,
            js_media_browse_panel(
                &MediaBrowsePanelSpec::new().with_loading(true),
                theme,
            )
        ))
        .child(group("Error", secondary,
            js_media_browse_panel(
                &MediaBrowsePanelSpec::new().with_error("Upload service is unavailable right now."),
                theme,
            )
        ))
        .child(group("Empty", secondary,
            js_media_browse_panel(
                &MediaBrowsePanelSpec::new().with_empty_message("No media matched this filter."),
                theme,
            )
        ))
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new().with_items(items[..2].to_vec()).with_size(ControlSize::Sm),
                    theme,
                ))
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new().with_items(items[..2].to_vec()).with_size(ControlSize::Md),
                    theme,
                ))
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new().with_items(items[..2].to_vec()).with_size(ControlSize::Lg),
                    theme,
                ))
        ))
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new().with_items(items[..2].to_vec()).with_density(ControlDensity::Compact),
                    theme,
                ))
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new().with_items(items[..2].to_vec()).with_density(ControlDensity::Default),
                    theme,
                ))
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new().with_items(items[..2].to_vec()).with_density(ControlDensity::Comfortable),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
