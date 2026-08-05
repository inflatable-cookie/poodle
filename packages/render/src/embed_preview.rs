//! EmbedPreview — embed display with loading/error/empty/placeholder states.
//!
//! Contract: `docs/contracts/components/embed-preview.md`
//! Ported from: `packages/jetstream/components/src/embed_preview.rs`.
//!
//! Composes the real `skeleton` (block) loading primitive and `text_link`
//! fallback. Live iframes are a host gap; the parsed-media state renders a
//! contract-sanctioned placeholder panel honoring the effective aspect ratio.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
};
use poodle_specs::{EmbedPreviewSpec, SkeletonSpec, TextLinkSpec};

use crate::presentation::rem_to_px;
use crate::skeleton::skeleton;
use crate::text_link::text_link;

pub fn embed_preview(spec: &EmbedPreviewSpec, theme: &dyn ThemeProvider) -> Node {
    let panel_bg = theme.resolve_color(spec.fill_token()); // background-panel
    let radius = theme.resolve_radius("radius.surface");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_tertiary = theme.resolve_color("color.text.tertiary");
    let danger_color = theme.resolve_color("color.status.danger");

    // Contract §7 sizing. gap 0.5rem → space.inline.sm; text 0.8125rem →
    // typography.label.size; fallback padding 0.75rem/1rem → space.panel.y / .x.
    // min-h 8rem, padding 1.5rem, icon 2rem have no exact named token — exact rem.
    let state_gap = theme.resolve_space("space.inline.sm");
    let text_size = theme.resolve_space("typography.label.size");
    let state_min_h = rem_to_px(8.0);
    let state_pad = rem_to_px(1.5);
    let icon_2rem = rem_to_px(2.0);
    let fallback_pad_y = theme.resolve_space("space.panel.y");
    let fallback_pad_x = theme.resolve_space("space.panel.x");

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };
    let label = |content: String, color| -> Node {
        let mut t = Node::text(content);
        t.style.descriptor.text_color = Some(color);
        t.style.text_size = Some(text_size);
        t
    };

    // Root: radius-surface + overflow hidden; state children carry the panel bg.
    let root = || {
        let mut r = Node::container();
        // Explicit Row (see switch.rs) — the old tier's bare div.
        r.style.descriptor.layout.direction = LayoutDirection::Row;
        r.style.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        r.style.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        all_radius(&mut r, radius);
        r
    };

    // Centered state column shared by loading / error / empty.
    let state_column = || {
        let mut c = Node::container();
        {
            let s = &mut c.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = state_gap;
            s.min_height = Some(state_min_h);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = state_pad;
            pad.right = state_pad;
            pad.top = state_pad;
            pad.bottom = state_pad;
            s.descriptor.background = Some(panel_bg);
        }
        all_radius(&mut c, radius);
        c
    };

    // ── Render priority: loading > error > empty > iframe > raw > trusted > fallback ──

    // Loading: real Skeleton primitive (block) + loading text.
    if spec.is_loading {
        return root().child(
            state_column()
                .child(skeleton(&SkeletonSpec::new().with_shape("block"), theme))
                .child(label("Loading preview...".to_string(), text_secondary)),
        );
    }

    // Error: alert-circle icon (text-danger) + error text.
    if let Some(ref error) = spec.error {
        let mut glyph = Node::icon("alert-circle", icon_2rem);
        glyph.style.descriptor.text_color = Some(danger_color);
        return root().child(
            state_column()
                .child(glyph)
                .child(label(error.clone(), text_secondary)),
        );
    }

    // Empty: play-rectangle icon (text-tertiary) + empty message.
    if spec.is_empty_state() {
        let mut glyph = Node::icon("monitor-play", icon_2rem);
        glyph.style.descriptor.text_color = Some(text_tertiary);
        return root().child(
            state_column()
                .child(glyph)
                .child(label(spec.empty_message.clone(), text_secondary)),
        );
    }

    // Aspect-ratio container. No live iframe → contract-sanctioned placeholder
    // panel. Min-height derives from the effective aspect ratio against a
    // nominal reference width; "auto" (audio / ratio None) falls back to the
    // static 10rem media height from the contract.
    let container = |child: Node| -> Node {
        let mut frame = Node::container();
        {
            let s = &mut frame.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.background = Some(panel_bg);
            s.descriptor.layout.width = LayoutSizing::Grow;
            match spec.effective_aspect_ratio() {
                Some(ratio) => {
                    let ref_width = rem_to_px(28.0);
                    s.min_height = Some((ref_width / ratio).max(rem_to_px(8.0)));
                }
                None => {
                    s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(10.0));
                }
            }
        }
        all_radius(&mut frame, radius);
        frame.child(child)
    };

    // Iframe / embed-URL state → placeholder panel with the derived embed URL.
    if let Some(url) = spec.embed_url() {
        let provider = spec
            .parsed
            .as_ref()
            .map(|p| p.provider.clone())
            .unwrap_or_default();
        let mut column = Node::container();
        {
            let s = &mut column.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = state_gap;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = state_pad;
            pad.right = state_pad;
            pad.top = state_pad;
            pad.bottom = state_pad;
            s.descriptor.layout.width = LayoutSizing::Grow;
        }
        let mut glyph = Node::icon("monitor-play", icon_2rem);
        glyph.style.descriptor.text_color = Some(text_tertiary);
        return root().child(container(
            column
                .child(glyph)
                .child(label(format!("{provider} embed"), text_secondary))
                .child(label(url, text_tertiary)),
        ));
    }

    let padded_grow = || {
        let mut p = Node::container();
        {
            let s = &mut p.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = state_pad;
            pad.right = state_pad;
            pad.top = state_pad;
            pad.bottom = state_pad;
            s.descriptor.layout.width = LayoutSizing::Grow;
        }
        p
    };

    // Raw embed (parsed.originalEmbed, no embed URL).
    if spec.has_raw_embed() {
        let raw = spec
            .parsed
            .as_ref()
            .and_then(|p| p.original_embed.clone())
            .unwrap_or_default();
        return root().child(container(padded_grow().child(label(raw, text_secondary))));
    }

    // Trusted HTML (caller-sanitized) — rendered in the same container.
    if spec.has_trusted_html() {
        let html = spec.trusted_html.clone().unwrap_or_default();
        return root().child(container(padded_grow().child(label(html, text_secondary))));
    }

    // Fallback: real TextLink to the original URL.
    let href = spec
        .parsed
        .as_ref()
        .and_then(|p| p.original_url.clone().or_else(|| Some(p.id.clone())))
        .unwrap_or_default();
    let mut fallback = Node::container();
    {
        let s = &mut fallback.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.background = Some(panel_bg);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = fallback_pad_x;
        pad.right = fallback_pad_x;
        pad.top = fallback_pad_y;
        pad.bottom = fallback_pad_y;
    }
    all_radius(&mut fallback, radius);
    root().child(fallback.child(text_link(
        &TextLinkSpec::new(href.clone()).with_href(href),
        theme,
        None,
    )))
}
