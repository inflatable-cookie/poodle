//! PageHeader — page title block with eyebrow, section, count, back link,
//! subtitle, host slots and a tone banner.
//!
//! Contract: `docs/contracts/components/page-header.md`
//! Ported from: `packages/jetstream/components/src/page_header.rs`.
//!
//! Anatomy: root (stacked) → top-row (title block + actions-row) → secondary
//! content (subtitle / breadcrumbs / meta) → banner. The title block stacks
//! eyebrow, section (distinct from eyebrow), title (+ count Pill), subtitle.
//! Actions, breadcrumbs, and meta are host-owned `Node` slots. Render-only:
//! the back-link click and any action behaviour live in the host.

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node,
};
use poodle_specs::{
    ControlSize, PageHeaderAlign, PageHeaderSpec, PillAppearance, PillSize, PillSpec, PillTone,
};

use crate::color::mix_srgb;
use crate::context::RenderContext;
use crate::pill::pill;
use crate::presentation::{rem_to_px, resolve_supporting_visual_size, size_font_rem};

/// Map the resolved control size to the supporting-visual `PillSize` the count
/// badge renders at (matches Svelte `resolveSupportingVisualSize`).
fn count_pill_size(size: ControlSize) -> PillSize {
    match resolve_supporting_visual_size(size) {
        ControlSize::Xs => PillSize::Xs,
        ControlSize::Sm => PillSize::Sm,
        ControlSize::Md => PillSize::Md,
        ControlSize::Lg => PillSize::Lg,
        ControlSize::Xl => PillSize::Xl,
    }
}

/// Per-level heading scale applied to the base heading-size token. Mirrors the
/// Svelte `--poodle-page-header-title-size` ladder (level 2 = base, 3+ compact).
fn level_scale(level: u8) -> f32 {
    match level {
        1 => 1.143, // 2rem-ish over the 1.75rem level-2 base
        2 => 1.0,
        _ => 0.714, // compact heading for levels 3–6 (≈1.25rem)
    }
}

fn styled_text(content: &str, color: ColorValue, size: f32) -> Node {
    let mut t = Node::text(content);
    t.style.descriptor.text_color = Some(color);
    t.style.text_size = Some(size);
    t
}

pub fn page_header(
    spec: &PageHeaderSpec,
    ctx: &RenderContext<'_>,
    breadcrumbs: Option<Node>,
    actions: Option<Node>,
    meta: Option<Node>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        effective_size,
    )));

    // ── Typography ────────────────────────────────────────────────────────────
    let heading_base = ctx.theme().resolve_space(spec.heading_size_token());
    let title_size = heading_base * level_scale(spec.level);
    let subtitle_size = rem_to_px(size_font_rem(effective_size));
    let eyebrow_size = rem_to_px(0.6875);
    let section_size = rem_to_px(0.75);
    let back_size = rem_to_px(0.8125);
    let body_font = rem_to_px(size_font_rem(effective_size));

    // ── Spacing ───────────────────────────────────────────────────────────────
    let gap = ctx.theme().resolve_space(spec.gap_token());
    let header_gap = ctx.theme().resolve_space(spec.header_gap_token());
    let title_block_gap = ctx.theme().resolve_space(spec.title_block_gap_token());
    let title_gap = ctx.theme().resolve_space(spec.title_gap_token());
    let actions_gap = ctx.theme().resolve_space(spec.actions_gap_token());
    let pad_y = ctx.theme().resolve_space(spec.padding_y_token());

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = ctx.theme().resolve_color(spec.title_color_token());
    let text_secondary = ctx.theme().resolve_color(spec.subtitle_color_token());
    let eyebrow_color = ctx.theme().resolve_color(spec.eyebrow_color_token());
    let section_color = ctx.theme().resolve_color(spec.section_color_token());
    let back_color = ctx.theme().resolve_color(spec.back_color_token());
    let context_dot = ctx.theme().resolve_color(spec.context_dot_color_token());
    let banner_color = ctx.theme().resolve_color(spec.banner_color_token());
    let surface = ctx.theme().resolve_color("color.background.surface");
    let banner_radius = ctx.theme().resolve_radius(spec.banner_radius_token());

    let primary_title = spec.primary_title();
    let resolved_subtitle = spec.resolved_subtitle();

    let mut outer = Node::container();
    {
        let s = &mut outer.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.spacing.padding.top = pad_y;
        s.descriptor.layout.spacing.padding.bottom = pad_y;
        s.fill_width = true;
    }

    // ── Top row: title block (left) + actions row (right) ─────────────────────
    let mut left_col = Node::container();
    {
        let s = &mut left_col.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = title_block_gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }

    // Eyebrow (category tag) — always rendered first when present.
    if let Some(ref eyebrow) = spec.eyebrow {
        let mut e = styled_text(&eyebrow.to_uppercase(), eyebrow_color, eyebrow_size);
        e.style.text_weight = Some(600);
        e.style.letter_spacing_em = Some(0.12); // contract __eyebrow: letter-spacing 0.12em
        left_col = left_col.child(e);
    }

    // Section label — distinct stacked row (only in default posture with split).
    if spec.has_section_title_split() && !spec.is_entity_detail_posture() {
        if let Some(ref section) = spec.section {
            let mut s = styled_text(&section.to_uppercase(), section_color, section_size);
            s.style.text_weight = Some(700);
            s.style.letter_spacing_em = Some(0.08); // contract __section: letter-spacing 0.08em
            left_col = left_col.child(s);
        }
    }

    // Title row: heading + optional count Pill.
    let mut title_row = Node::container();
    {
        let s = &mut title_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = title_gap;
    }
    let mut title = styled_text(&primary_title, text_primary, title_size);
    title.style.text_weight = Some(700);
    let mut title_row = title_row.child(title);

    if let Some(count) = spec.count {
        title_row = title_row.child(pill(
            &PillSpec::new()
                .with_label(format!("{count}"))
                .with_tone(PillTone::Neutral)
                .with_appearance(PillAppearance::Subtle)
                .with_size(count_pill_size(effective_size)),
            ctx,
        ));
    }

    left_col = left_col.child(title_row);

    // Subtitle (in default posture; in entity-detail it is the swapped title).
    if let Some(ref subtitle) = resolved_subtitle {
        left_col = left_col.child(styled_text(subtitle, text_secondary, subtitle_size));
    }

    // Actions row: back link (left) + actions cluster (right).
    let actions_row = if spec.has_back_link() || actions.is_some() {
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = header_gap;
            s.flex_none = true;
        }

        if spec.has_back_link() {
            let mut back = Node::container();
            {
                let s = &mut back.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = rem_to_px(0.35);
                s.descriptor.cursor = CursorHint::Pointer;
            }
            let mut arrow = Node::icon("arrow-left", icon_size);
            arrow.style.descriptor.text_color = Some(back_color);
            let mut back = back.child(arrow).child(styled_text(
                &spec.back_display_label(),
                back_color,
                back_size,
            ));
            if spec.back_is_contextual {
                let mut dot = Node::container();
                {
                    let s = &mut dot.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(0.375));
                    s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.375));
                    let r = rem_to_px(0.1875);
                    s.descriptor.corner_radii.top_left = r;
                    s.descriptor.corner_radii.top_right = r;
                    s.descriptor.corner_radii.bottom_right = r;
                    s.descriptor.corner_radii.bottom_left = r;
                    s.flex_none = true;
                    s.descriptor.background = Some(context_dot);
                }
                back = back.child(dot);
            }
            row = row.child(back);
        }

        if let Some(actions) = actions {
            let mut cluster = Node::container();
            {
                let s = &mut cluster.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.flex_wrap = true;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = actions_gap;
            }
            row = row.child(cluster.child(actions));
        }
        Some(row)
    } else {
        None
    };

    let mut top_row = Node::container();
    {
        let s = &mut top_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.alignment.main = match spec.align {
            PageHeaderAlign::Start => MainAxisAlignment::Start,
            PageHeaderAlign::Between => MainAxisAlignment::SpaceBetween,
        };
        s.descriptor.layout.spacing.gap = header_gap;
        s.fill_width = true;
    }
    let mut top_row = top_row.child(left_col);
    if let Some(actions_row) = actions_row {
        top_row = top_row.child(actions_row);
    }
    let mut outer = outer.child(top_row);

    // ── Secondary content: breadcrumbs / meta ─────────────────────────────────
    if let Some(breadcrumbs) = breadcrumbs {
        let mut crumb_row = Node::container();
        {
            let s = &mut crumb_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_wrap = true;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.min_width = Some(0.0);
        }
        outer = outer.child(crumb_row.child(breadcrumbs));
    }
    if let Some(meta) = meta {
        let mut meta_wrap = Node::container();
        {
            let s = &mut meta_wrap.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.spacing.margin.top = rem_to_px(0.125);
        }
        outer = outer.child(meta_wrap.child(meta));
    }

    // ── Banner ────────────────────────────────────────────────────────────────
    if spec.has_banner() {
        if let Some(ref message) = spec.banner_message {
            // Tinted banner fill: mix banner tone into the panel surface.
            let tinted_fill = mix_srgb(banner_color, surface, 0.12);
            let banner_border = mix_srgb(banner_color, surface, 0.38);
            let mut banner = Node::container();
            {
                let s = &mut banner.style;
                s.descriptor.background = Some(tinted_fill);
                s.descriptor.corner_radii.top_left = banner_radius;
                s.descriptor.corner_radii.top_right = banner_radius;
                s.descriptor.corner_radii.bottom_right = banner_radius;
                s.descriptor.corner_radii.bottom_left = banner_radius;
                s.border_left_width = Some(2.0);
                s.border_color_left = Some(banner_border);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.75);
                pad.right = rem_to_px(0.75);
                pad.top = rem_to_px(0.5);
                pad.bottom = rem_to_px(0.5);
                s.descriptor.layout.direction = LayoutDirection::Column;
            }
            let msg = styled_text(message, banner_color, body_font);
            outer = outer.child(banner.child(msg));
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            outer.a11y.label = Some(label.to_string());
        }
    }
    outer
}
