//! RemediationBanner — announcing recovery banner.
//!
//! Contract: `docs/contracts/components/remediation-banner.md`
//! Ported from: `packages/jetstream/components/src/remediation_banner.rs`.
//!
//! Anatomy (contract §2): icon + content (title/message/actions) + dismiss.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole,
};
use poodle_specs::{ButtonSpec, RemediationAction, RemediationBannerSpec};

use crate::button::button;
use crate::color::mix_srgb;
use crate::presentation::rem_to_px;

pub fn remediation_banner(spec: &RemediationBannerSpec, theme: &dyn ThemeProvider) -> Node {
    // ── Colors (all token-resolved) ──
    let tone_color = theme.resolve_color(spec.border_token());
    let panel = theme.resolve_color(spec.background_token());
    let icon_color = theme.resolve_color(spec.icon_color_token());
    let text_primary = theme.resolve_color(spec.title_color_token());
    let text_secondary = theme.resolve_color(spec.message_color_token());

    // Surface fill = color-mix(tone, panel) at the spec's tone ratio; border = tone.
    let fill = mix_srgb(tone_color, panel, spec.fill_tone_ratio());
    let border = theme.resolve_color(spec.border_token());

    // ── Dimensions ──
    // Radius / border-width resolve from tokens; the rest are contract-exact
    // rem values (no semantic token exists for them).
    let radius = theme.resolve_radius(spec.radius_token());
    let border_width = theme.resolve_border_width(spec.border_width_token());
    // Typography: title at body size, message at label size (contract §2).
    let title_size = theme.resolve_space("typography.body.size");
    let message_size = theme.resolve_space("typography.label.size");
    let icon_size = rem_to_px(1.25); // note: contract icon size, no token
    let pad_x = theme.resolve_space("space.panel.x");
    let pad_y = theme.resolve_space("space.panel.y");
    let gap = theme.resolve_space("space.inline.md"); // root row gap (note: approx)
    let content_gap = theme.resolve_space("space.inline.xs"); // title↔message↔actions
    let action_gap = theme.resolve_space("space.inline.sm"); // between action buttons
    let dismiss_size = rem_to_px(1.0); // note: contract dismiss size, no token

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.spacing.gap = gap;
    }
    el.a11y.role = Some(NodeRole::Alert);

    // ── Icon (contract §2: tone-based leading indicator) ──
    let mut glyph = Node::icon(spec.tone_icon_name(), icon_size);
    glyph.style.descriptor.text_color = Some(icon_color);
    let mut el = el.child(glyph);

    // ── Content column: Title + Message + Actions ──
    let mut content = Node::container();
    {
        let s = &mut content.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = content_gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }

    let mut title = Node::text(&spec.title);
    title.style.descriptor.text_color = Some(text_primary);
    title.style.text_size = Some(title_size);
    title.style.text_weight = Some(600);
    let mut content = content.child(title);

    let mut message = Node::text(&spec.message);
    message.style.descriptor.text_color = Some(text_secondary);
    message.style.text_size = Some(message_size);
    content = content.child(message);

    // ── Actions row: real buttons honoring variant + is_disabled ──
    if spec.action_count() > 0 {
        let mut actions_row = Node::container();
        {
            let s = &mut actions_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = action_gap;
            s.descriptor.layout.spacing.padding.top = content_gap;
        }

        if let Some(ref primary) = spec.primary_action {
            actions_row = actions_row.child(action_button(primary, theme));
        }
        if let Some(ref secondary) = spec.secondary_action {
            actions_row = actions_row.child(action_button(secondary, theme));
        }

        content = content.child(actions_row);
    }

    el = el.child(content);

    // ── Dismiss (contract §5: aria-label="Dismiss") ──
    if spec.is_dismissible {
        let mut dismiss = Node::icon("x", dismiss_size);
        dismiss.id = Some("remediation-banner-dismiss".to_string());
        dismiss.style.descriptor.text_color = Some(text_secondary);
        dismiss.style.descriptor.cursor = CursorHint::Pointer;
        el = el.child(dismiss);
    }

    el
}

/// Build a real `button` for a `RemediationAction`, honoring its variant and
/// disabled flag (contract §2: RemediationAction button; §3 `is_disabled`).
/// `button` already applies the disabled-opacity token.
fn action_button(action: &RemediationAction, theme: &dyn ThemeProvider) -> Node {
    let mut b = button(
        &ButtonSpec::new()
            .with_variant(action.variant)
            .with_label(action.label.clone())
            .with_disabled(action.is_disabled),
        theme,
        None,
    );
    b.id = Some(format!("remediation-action-{}", action.id));
    b
}
