//! MediaBrowsePanel — Jetstream media browse panel backed by MediaBrowsePanelSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    AspectRatio, ButtonSpec, ButtonVariant, CallOutSpec, MediaBrowsePanelSpec, MediaKind,
    MediaThumbnailSpec, StatusTone,
};

use crate::button::js_button;
use crate::callout::js_callout;
use crate::media_thumbnail::js_media_thumbnail;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_radius, tint};

pub fn js_media_browse_panel(spec: &MediaBrowsePanelSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let body_font = rem_to_px(size_font_rem(effective_size));
    let label_font = rem_to_px(0.8125);
    let min_column = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 8.5,
        poodle_specs::ControlSize::Sm => 10.0,
        poodle_specs::ControlSize::Md => 11.0,
        poodle_specs::ControlSize::Lg => 12.5,
        poodle_specs::ControlSize::Xl => 14.0,
    });

    // Density-driven spacing from contract
    let (grid_gap, item_gap, item_pad) = match spec.density {
        poodle_specs::ControlDensity::Compact => (0.375, 0.25, 0.5),
        poodle_specs::ControlDensity::Default => (0.5, 0.375, 0.75),
        poodle_specs::ControlDensity::Comfortable => (0.75, 0.5, 0.875),
    };

    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let radius = resolve_radius(theme, "radius.surface");
    let panel_bg = resolve_color(theme, "color.background.panel");

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

    // Ready: render grid
    let mut grid = ui_element::div()
        .flex_row()
        .flex_wrap()
        .gap(rem_to_px(grid_gap));

    for item in &spec.items {
        let panel_bg_tinted = tint(panel_bg, 0.92);
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
        let actions = ui_element::div()
            .flex_row()
            .justify_center()
            .self_stretch()
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
