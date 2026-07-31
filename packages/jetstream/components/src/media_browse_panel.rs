//! MediaBrowsePanel — Jetstream media browse panel backed by MediaBrowsePanelSpec.
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    AspectRatio, ButtonSpec, ButtonVariant, CallOutSpec, MediaBrowsePanelSpec, MediaKind,
    MediaThumbnailSpec, StatusTone,
};

use crate::button::js_button;
use crate::callout::js_callout;
use crate::media_thumbnail::js_media_thumbnail;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius, tint};

/// MediaBrowsePanel — a grid of media items.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_select(handler)`.
///
/// No `on_load_more`: paging is scroll-driven on the web, and this runtime's
/// scroll events are not yet surfaced through the builder.
pub struct MediaBrowsePanel {
    spec: MediaBrowsePanelSpec,
    theme: JetstreamThemeProvider,
    on_select: Option<crate::element::Handler>,
}

impl MediaBrowsePanel {
    pub fn from_spec(spec: MediaBrowsePanelSpec, theme: &JetstreamThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_select: None }
    }

    /// Fires with the chosen item's id.
    pub fn on_select(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_select = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for MediaBrowsePanel {
    fn into_js_el(self) -> JsEl {
        build(&self.spec, &self.theme, self.on_select)
    }
}

pub fn js_media_browse_panel(spec: &MediaBrowsePanelSpec, theme: &JetstreamThemeProvider) -> JsEl {
    build(spec, theme, None)
}

fn build(
    spec: &MediaBrowsePanelSpec,
    theme: &JetstreamThemeProvider,
    on_select: Option<crate::element::Handler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let body_font = rem_to_px(size_font_rem(effective_size));
    // Contract §8 Meta / State `p` font-size = 0.8125rem (13px) — resolved from
    // the label typography token rather than a literal.
    let label_font = resolve_px(theme, spec.meta_font_token());
    // Contract §8 Size Adjustments: xs 8.5 / sm 10 / md 11 / lg 12 / xl 13.
    let min_column = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 8.5,
        poodle_specs::ControlSize::Sm => 10.0,
        poodle_specs::ControlSize::Md => 11.0,
        poodle_specs::ControlSize::Lg => 12.0,
        poodle_specs::ControlSize::Xl => 13.0,
    });

    // Density-driven spacing from contract
    let (grid_gap, item_gap, item_pad) = match spec.density {
        poodle_specs::ControlDensity::Compact => (0.375, 0.25, 0.5),
        poodle_specs::ControlDensity::Default => (0.5, 0.375, 0.75),
        poodle_specs::ControlDensity::Comfortable => (0.75, 0.5, 0.875),
    };

    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let border_subtle = resolve_color(theme, spec.item_border_token());
    let radius = resolve_radius(theme, spec.item_radius_token());
    let panel_bg = resolve_color(theme, spec.item_bg_token());

    // Root
    let mut el = ui_element::div().flex_col().self_stretch().min_h(rem_to_px(18.0));

    // Loading state
    if spec.loading && spec.items.is_empty() {
        let state = ui_element::div()
            .items_center()
            .justify_center()
            .min_h(rem_to_px(18.0))
            .child(
                ui_element::label("Loading media...")
                    .text_color(text_secondary)
                    .text_size(label_font),
            );
        return el.child(state);
    }

    // Error state
    if let Some(ref error) = spec.error {
        let state = ui_element::div()
            .self_stretch()
            .min_h(rem_to_px(18.0))
            .items_center()
            .justify_center()
            .child(js_callout(
                &CallOutSpec::new()
                    .with_tone(StatusTone::Danger)
                    .with_content(error)
                    .with_size(spec.size)
                    .with_size_role(spec.size_role)
                    .with_density(spec.density),
                theme,
            ));
        return el.child(state);
    }

    // Empty state
    if spec.items.is_empty() {
        let state = ui_element::div()
            .items_center()
            .justify_center()
            .min_h(rem_to_px(18.0))
            .child(
                ui_element::label(&spec.empty_message)
                    .text_color(text_secondary)
                    .text_size(label_font),
            );
        return el.child(state);
    }

    // Ready: render grid. Contract §8 Grid `margin-top` equals the grid gap.
    let mut grid = ui_element::div()
        .flex_row()
        .flex_wrap()
        .gap(rem_to_px(grid_gap))
        .mt(rem_to_px(grid_gap));

    // Contract §8 Item background `color-mix(background-panel 92%, transparent)`.
    let panel_bg_tinted = tint(panel_bg, 0.92);
    for item in &spec.items {
        let mut card = ui_element::button("")
            .flex_col()
            .gap(rem_to_px(item_gap))
            .pl(rem_to_px(item_pad))
            .pr(rem_to_px(item_pad))
            .pt(rem_to_px(item_pad))
            .pb(rem_to_px(item_pad))
            .min_w(min_column)
            .border(1.0)
            .border_color(border_subtle)
            .rounded(radius)
            .bg(panel_bg_tinted)
            .focusable();

        card = card.child(
            js_media_thumbnail(
                &MediaThumbnailSpec::new(match item.kind.as_str() {
                    "image" => MediaKind::Image,
                    "audio" => MediaKind::Audio,
                    "video" => MediaKind::Video,
                    "document" => MediaKind::Document,
                    _ => MediaKind::Embed,
                })
                    .with_aspect_ratio(AspectRatio::Square)
                    .with_show_caption(false),
                theme,
            ),
        );

        // Label
        card = card.child(
            ui_element::label(&item.label)
                .text_color(text_primary)
                .text_size(body_font)
                .text_weight(600),
        );

        // Meta (optional)
        let meta_text = item.meta.as_ref().cloned().unwrap_or_else(|| item.kind.clone());
        card = card.child(
            ui_element::label(&meta_text)
                .text_color(text_secondary)
                .text_size(label_font),
        );

        if let Some(handler) = &on_select {
            let handler = std::sync::Arc::clone(handler);
            let id = item.id.clone();
            card = card.on_click(move |_event| handler(&id));
        }

        grid = grid.child(card);
    }
    el = el.child(grid);

    // Load more
    if spec.has_more {
        let load_label = if spec.loading {
            "Loading..."
        } else {
            spec.load_more_label.as_str()
        };
        // Contract §8 Actions `margin-top` equals the grid gap.
        let actions = ui_element::div()
            .flex_row()
            .justify_center()
            .self_stretch()
            .mt(rem_to_px(grid_gap))
            .child(
                js_button(
                    &ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_size(spec.size)
                        .with_size_role(spec.size_role)
                        .with_density(spec.density)
                        .with_label(load_label)
                        .with_disabled(spec.loading),
                    theme,
                ),
            );
        el = el.child(actions);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{ControlSize, MediaBrowseItem};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn items() -> Vec<MediaBrowseItem> {
        vec![
            MediaBrowseItem::new("a", "hero.png", "image").with_meta("3.1 MB"),
            MediaBrowseItem::new("b", "clip.mp4", "video").with_meta("128 MB"),
        ]
    }

    fn base() -> MediaBrowsePanelSpec {
        MediaBrowsePanelSpec::new().with_items(items())
    }

    #[test]
    fn ready_renders_items_label_and_meta() {
        let th = theme();
        let tree = probe(&js_media_browse_panel(&base(), &th), 480.0, 320.0);
        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(tree.has_text("hero.png"), "label missing: {:?}", tree.texts());
        assert!(tree.has_text("3.1 MB"), "meta missing: {:?}", tree.texts());
        assert!(tree.has_text("clip.mp4"), "second label missing: {:?}", tree.texts());
    }

    #[test]
    fn loading_state_shows_copy() {
        let th = theme();
        let spec = MediaBrowsePanelSpec::new().with_loading(true);
        let tree = probe(&js_media_browse_panel(&spec, &th), 480.0, 320.0);
        assert!(tree.has_text("Loading media..."), "loading copy missing: {:?}", tree.texts());
    }

    #[test]
    fn error_state_renders_message() {
        let th = theme();
        let spec = MediaBrowsePanelSpec::new().with_error("Failed to load media");
        let tree = probe(&js_media_browse_panel(&spec, &th), 480.0, 320.0);
        assert!(
            tree.has_text("Failed to load media"),
            "error callout message missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn empty_state_renders_message() {
        let th = theme();
        let spec = MediaBrowsePanelSpec::new().with_empty_message("Nothing here");
        let tree = probe(&js_media_browse_panel(&spec, &th), 480.0, 320.0);
        assert!(tree.has_text("Nothing here"), "empty copy missing: {:?}", tree.texts());
    }

    #[test]
    fn load_more_label_switches_while_loading() {
        let th = theme();
        let idle = base().with_has_more(true).with_load_more_label("Load more");
        let idle_tree = probe(&js_media_browse_panel(&idle, &th), 480.0, 320.0);
        assert!(idle_tree.has_text("Load more"), "load-more label missing: {:?}", idle_tree.texts());

        let busy = base().with_has_more(true).with_loading(true);
        let busy_tree = probe(&js_media_browse_panel(&busy, &th), 480.0, 320.0);
        assert!(
            busy_tree.has_text("Loading..."),
            "load-more should switch to Loading...: {:?}",
            busy_tree.texts()
        );
    }

    #[test]
    fn meta_font_is_label_size_not_caption() {
        let th = theme();
        // Contract §8 meta/state font = 0.8125rem = 13px = typography.label.size.
        // Regression guard: caption (11px) and the old 0px GPUI-adapter miss are wrong.
        let expected = resolve_px(&th, MediaBrowsePanelSpec::new().meta_font_token());
        assert!((expected - 13.0).abs() < 0.01, "label token should be 13px, got {expected}");
        let tree = probe(&js_media_browse_panel(&base(), &th), 480.0, 320.0);
        let meta = tree
            .nodes
            .iter()
            .find(|n| n.text.as_deref() == Some("3.1 MB"))
            .expect("meta node present");
        assert!(
            meta.text_size.map(|s| (s - expected).abs() < 0.01).unwrap_or(false),
            "meta font should resolve to {expected}px, got {:?}",
            meta.text_size
        );
    }

    #[test]
    fn lg_min_column_matches_contract() {
        let th = theme();
        // Contract §8 Size Adjustments: lg min-column = 12rem (192px), not 12.5rem.
        // Probe one item at a viewport narrower than the min so the min-width floor
        // is exposed as the item's computed width (flex can't shrink below it).
        let one = vec![MediaBrowseItem::new("a", "hero.png", "image").with_meta("3.1 MB")];
        let spec = MediaBrowsePanelSpec::new().with_items(one).with_size(ControlSize::Lg);
        let tree = probe(&js_media_browse_panel(&spec, &th), 120.0, 320.0);
        let target = rem_to_px(12.0); // 192px
        let stale = rem_to_px(12.5); // 200px — the bug we fixed
        let widths: Vec<f32> = tree.nodes.iter().map(|n| n.w).collect();
        assert!(
            widths.iter().any(|w| (w - target).abs() < 0.5),
            "expected an item floored at lg min-column {target}px; widths: {widths:?}"
        );
        assert!(
            !widths.iter().any(|w| (w - stale).abs() < 0.5),
            "no item should sit at the stale 12.5rem ({stale}px) min-column; widths: {widths:?}"
        );
    }

    #[test]
    fn choosing_an_item_reports_its_id() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let ids = Arc::clone(&seen);

        let spec = MediaBrowsePanelSpec::new().with_items(items());
        let first = items().first().cloned().expect("an item");

        let el = MediaBrowsePanel::from_spec(spec, &theme())
            .on_select(move |id| ids.lock().unwrap().push(id.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 640.0, 480.0, &first.label);

        assert_eq!(seen.lock().unwrap().as_slice(), [first.id]);
    }

}
