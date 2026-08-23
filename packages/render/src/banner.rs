//! Banner — status strip with tone icon, title, message, dismiss affordance.
//!
//! Ported from: `packages/jetstream/components/src/banner.rs`.

use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node};
use poodle_specs::{BannerSpec, StatusTone};

use crate::color::mix_srgb;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

fn tone_icon(tone: StatusTone) -> &'static str {
    match tone {
        StatusTone::Neutral | StatusTone::Info => "info",
        StatusTone::Success => "check-circle",
        StatusTone::Warning => "alert-triangle",
        StatusTone::Danger => "x-circle",
        StatusTone::Pending => "loader",
    }
}

pub fn banner(spec: &BannerSpec, ctx: &RenderContext<'_>) -> Node {
    let theme = ctx.theme();
    let tone_color = theme.resolve_color(spec.fill_token());
    let icon_color = theme.resolve_color(spec.icon_color_token());
    let border_color = theme.resolve_color(spec.border_token());
    let text_primary = theme.resolve_color("color.text.primary");
    let panel = theme.resolve_color("color.background.panel");

    let fill = mix_srgb(tone_color, panel, 0.12);

    let pad_x = rem_to_px(0.75);
    let pad_y = rem_to_px(0.5);
    let gap = rem_to_px(0.5);
    let font_size = rem_to_px(0.8125);
    let icon_size = rem_to_px(1.0);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border_color;
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.spacing.padding.top = pad_y;
        s.descriptor.layout.spacing.padding.bottom = pad_y;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
    }

    if spec.has_icon {
        let mut icon = Node::icon(tone_icon(spec.tone), icon_size);
        icon.style.descriptor.text_color = Some(icon_color);
        el = el.child(icon);
    }

    if let Some(ref title) = spec.title {
        let mut t = Node::text(title);
        t.style.descriptor.text_color = Some(text_primary);
        t.style.text_size = Some(font_size);
        t.style.text_weight = Some(600);
        el = el.child(t);
    }

    if let Some(ref message) = spec.message {
        let mut m = Node::text(message);
        m.style.descriptor.text_color = Some(text_primary);
        m.style.text_size = Some(font_size);
        m.style.descriptor.layout.width = LayoutSizing::Grow;
        el = el.child(m);
    }

    if spec.is_dismissible {
        let mut x = Node::icon("x", icon_size);
        x.style.descriptor.text_color = Some(text_primary);
        x.style.descriptor.cursor = CursorHint::Pointer;
        el = el.child(x);
    }

    el
}
