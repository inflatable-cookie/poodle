//! MediaBrowsePanel — grid of selectable media items.
//!
//! Contract: `docs/contracts/components/media-browse-panel.md`
//! Ported from: `packages/jetstream/components/src/media_browse_panel.rs`.
//!
//! No `on_load_more`: paging is scroll-driven on the web and scroll events
//! are not yet surfaced through the node vocabulary.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::{
    AspectRatio, ButtonSpec, ButtonVariant, CallOutSpec, ControlDensity, ControlSize,
    MediaBrowsePanelSpec, MediaKind, MediaThumbnailSpec, StatusTone,
};

use crate::button::button;
use crate::callout::callout;
use crate::color::with_alpha;
use crate::media_thumbnail::media_thumbnail;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};

pub fn media_browse_panel(
    spec: &MediaBrowsePanelSpec,
    theme: &dyn ThemeProvider,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let body_font = rem_to_px(size_font_rem(effective_size));
    // Contract §8 Meta / State `p` font-size — resolved from the label
    // typography token rather than a literal.
    let label_font = theme.resolve_space(spec.meta_font_token());
    // Contract §8 Size Adjustments: xs 8.5 / sm 10 / md 11 / lg 12 / xl 13.
    let min_column = rem_to_px(match effective_size {
        ControlSize::Xs => 8.5,
        ControlSize::Sm => 10.0,
        ControlSize::Md => 11.0,
        ControlSize::Lg => 12.0,
        ControlSize::Xl => 13.0,
    });

    // Density-driven spacing from contract
    let (grid_gap, item_gap, item_pad) = match spec.density {
        ControlDensity::Compact => (0.375, 0.25, 0.5),
        ControlDensity::Default => (0.5, 0.375, 0.75),
        ControlDensity::Comfortable => (0.75, 0.5, 0.875),
    };

    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_primary = theme.resolve_color("color.text.primary");
    let border_subtle = theme.resolve_color(spec.item_border_token());
    let radius = theme.resolve_radius(spec.item_radius_token());
    let panel_bg = theme.resolve_color(spec.item_bg_token());

    // Root
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.self_stretch = true;
        s.min_height = Some(rem_to_px(18.0));
    }

    let centered_state = |child: Node, stretch: bool| -> Node {
        let mut state = Node::container();
        {
            let s = &mut state.style;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.min_height = Some(rem_to_px(18.0));
            if stretch {
                s.self_stretch = true;
            }
        }
        state.child(child)
    };

    // Loading state
    if spec.loading && spec.items.is_empty() {
        let mut copy = Node::text("Loading media...");
        copy.style.descriptor.text_color = Some(text_secondary);
        copy.style.text_size = Some(label_font);
        return el.child(centered_state(copy, false));
    }

    // Error state
    if let Some(ref error) = spec.error {
        let alert = callout(
            &CallOutSpec::new()
                .with_tone(StatusTone::Danger)
                .with_content(error)
                .with_size(spec.size)
                .with_size_role(spec.size_role)
                .with_density(spec.density),
            theme,
            None,
        );
        return el.child(centered_state(alert, true));
    }

    // Empty state
    if spec.items.is_empty() {
        let mut copy = Node::text(&spec.empty_message);
        copy.style.descriptor.text_color = Some(text_secondary);
        copy.style.text_size = Some(label_font);
        return el.child(centered_state(copy, false));
    }

    // Ready: render grid. Contract §8 Grid `margin-top` equals the grid gap.
    let mut grid = Node::container();
    {
        let s = &mut grid.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.spacing.gap = rem_to_px(grid_gap);
        s.descriptor.layout.spacing.margin.top = rem_to_px(grid_gap);
    }

    // Contract §8 Item background `color-mix(background-panel 92%, transparent)`.
    let panel_bg_tinted = with_alpha(panel_bg, panel_bg.3 * 0.92);
    for item in &spec.items {
        let mut card = Node::button("");
        {
            let s = &mut card.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(item_gap);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(item_pad);
            pad.right = rem_to_px(item_pad);
            pad.top = rem_to_px(item_pad);
            pad.bottom = rem_to_px(item_pad);
            s.min_width = Some(min_column);
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = border_subtle;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            s.descriptor.background = Some(panel_bg_tinted);
        }
        card.interaction.focusable = true;

        card = card.child(media_thumbnail(
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
        ));

        // Label
        let mut label = Node::text(&item.label);
        label.style.descriptor.text_color = Some(text_primary);
        label.style.text_size = Some(body_font);
        label.style.text_weight = Some(600);
        card = card.child(label);

        // Meta (optional; falls back to the kind)
        let meta_text = item
            .meta
            .as_ref()
            .cloned()
            .unwrap_or_else(|| item.kind.clone());
        let mut meta = Node::text(&meta_text);
        meta.style.descriptor.text_color = Some(text_secondary);
        meta.style.text_size = Some(label_font);
        card = card.child(meta);

        if let Some(handler) = &on_select {
            let handler = Arc::clone(handler);
            let id = item.id.clone();
            card.interaction.on_activate = Some(Arc::new(move || handler(&id)));
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
        let mut actions = Node::container();
        {
            let s = &mut actions.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.self_stretch = true;
            s.descriptor.layout.spacing.margin.top = rem_to_px(grid_gap);
        }
        actions = actions.child(button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_size(spec.size)
                .with_size_role(spec.size_role)
                .with_density(spec.density)
                .with_label(load_label)
                .with_disabled(spec.loading),
            theme,
            None,
        ));
        el = el.child(actions);
    }

    el
}
