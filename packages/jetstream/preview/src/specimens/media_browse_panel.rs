//! MediaBrowsePanel specimen — browse grid of selectable media items.
//!
//! Mirrors the GPUI `media_browse_panel` specimen: the browse grid with
//! load-more, the loading / error / empty postures, the loading-more posture,
//! and the semantic (compact / prominent) presentation row. Every panel is the
//! real `js_media_browse_panel`, which composes the real `js_media_thumbnail`
//! (grid cards), `js_callout` (error), and `js_button` (load-more) internally.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::media_browse_panel::js_media_browse_panel;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    ControlDensity, ControlSize, MediaBrowseItem, MediaBrowsePanelSpec, SemanticControlSizeRole,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let items = vec![
        MediaBrowseItem::new("1", "Hero banner", "image").with_meta("Image \u{2022} 1920 x 1080"),
        MediaBrowseItem::new("2", "Launch trailer", "video").with_meta("Video \u{2022} 3:42"),
        MediaBrowseItem::new("3", "Podcast intro", "audio").with_meta("Audio \u{2022} 1:18"),
        MediaBrowseItem::new("4", "Quarterly report", "document").with_meta("Document \u{2022} PDF"),
    ];

    div()
        .flex_col()
        .gap(24.0)
        // Browse grid + load-more action (real js_button footer).
        .child(group(
            "Browse panel",
            secondary,
            js_media_browse_panel(
                &MediaBrowsePanelSpec::new()
                    .with_items(items.clone())
                    .with_has_more(true),
                theme,
            ),
        ))
        // Loading / error / empty postures stacked (loading copy, danger callout, empty copy).
        .child(group(
            "Browse panel states",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new().with_loading(true),
                    theme,
                ))
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new().with_error("Failed to load media"),
                    theme,
                ))
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new().with_empty_message("No media found"),
                    theme,
                )),
        ))
        // Items present + loading + has_more → load-more button shows "Loading..." disabled.
        .child(group(
            "Loading more",
            secondary,
            js_media_browse_panel(
                &MediaBrowsePanelSpec::new()
                    .with_items(items.clone())
                    .with_has_more(true)
                    .with_loading(true),
                theme,
            ),
        ))
        // Compact/sm + prominent semantic presentation.
        .child(group(
            "Semantic presentation",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new()
                        .with_items(items.clone())
                        .with_has_more(true)
                        .with_size(ControlSize::Sm)
                        .with_density(ControlDensity::Compact),
                    theme,
                ))
                .child(js_media_browse_panel(
                    &MediaBrowsePanelSpec::new()
                        .with_items(items)
                        .with_has_more(true)
                        .with_size_role(SemanticControlSizeRole::Prominent),
                    theme,
                )),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
