//! DetailItem — label/value row: inline or stacked, simple or surface.
//!
//! Contract: `docs/contracts/components/detail-item.md`
//! Ported from: `packages/jetstream/components/src/detail_item.rs`.

use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, FontFamily, LayoutDirection, LayoutSizing, Node};
use poodle_specs::{
    DetailItemLayout, DetailItemPresentation, DetailItemSpan, DetailItemSpec, IconSpec, PopoverSpec,
    TextSpec, TextWeight,
};

use crate::color::mix_srgb;
use crate::context::RenderContext;
use crate::icon::icon;
use crate::popover::{popover, PopoverHandlers, POPOVER_TRIGGER_ID};
use crate::presentation::rem_to_px;
use crate::text;

/// Accessible name Svelte gives the description trigger and its surface.
const INFO_LABEL: &str = "More information";

fn find_id_mut<'a>(node: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    if node.id.as_deref() == Some(id) {
        return Some(node);
    }
    node.children
        .iter_mut()
        .find_map(|child| find_id_mut(child, id))
}

fn part(node: &mut Node, name: &str) {
    node.roles.insert("part".to_owned(), name.to_owned());
}

/// Compose the production Text primitive with DetailItem-owned typography.
fn detail_text(
    content: &str,
    part_name: &str,
    color: poodle_node::ColorValue,
    size: f32,
    weight: TextWeight,
    line_height: f32,
    ctx: &RenderContext<'_>,
) -> Node {
    let mut node = text(&TextSpec::new(content).with_weight(weight), ctx);
    part(&mut node, part_name);
    node.roles
        .insert("dependency".to_owned(), "text".to_owned());
    node.style.descriptor.text_color = Some(color);
    node.style.text_size = Some(size);
    node.style.text_weight = Some(match weight {
        TextWeight::Normal => 400,
        TextWeight::Medium => 500,
        TextWeight::Semibold => 600,
        TextWeight::Bold => 700,
    });
    node.style.font_family = Some(FontFamily::Sans);
    node.style.line_height = Some(line_height);
    node
}

pub fn detail_item(spec: &DetailItemSpec, ctx: &RenderContext<'_>) -> Node {
    detail_item_with_slots(spec, ctx, None, None)
}

/// Variant exposing the value-content and trailing-action slots.
pub fn detail_item_with_slots(
    spec: &DetailItemSpec,
    ctx: &RenderContext<'_>,
    value_content: Option<Node>,
    action: Option<Node>,
) -> Node {
    detail_item_with_slots_state(spec, ctx, value_content, action, false, None)
}

/// Host-owned description popover open state. Svelte keeps that overlay
/// inside DetailItem; the native composition matches it.
pub fn detail_item_with_slots_state(
    spec: &DetailItemSpec,
    ctx: &RenderContext<'_>,
    value_content: Option<Node>,
    action: Option<Node>,
    description_open: bool,
    on_description_toggle: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let density = ctx.resolve_density(spec.density);
    let theme = ctx.theme();
    let label_color = theme.resolve_color(spec.label_color_token());
    let value_color = theme.resolve_color(spec.value_color_token());
    let desc_color = theme.resolve_color(spec.description_color_token());
    let tertiary_color = theme.resolve_color(spec.stacked_label_color_token());
    let surface = theme.resolve_color(spec.background_token());
    let radius = (theme.resolve_radius(spec.radius_token()) - rem_to_px(0.0625)).max(0.0);

    let label_font = theme.resolve_space(spec.label_size_token());
    let value_font = theme.resolve_space(spec.value_size_token());
    let label_line_height = theme.resolve_space(spec.label_line_height_token());
    let value_line_height = theme.resolve_space(spec.value_line_height_token());
    let desc_font = rem_to_px(0.75);

    let row_gap = rem_to_px(spec.row_gap_rem(density));
    let inline_gap = rem_to_px(spec.inline_gap_rem(density));
    let surface_stacked_gap = rem_to_px(spec.surface_stacked_gap_rem(density));
    let pad_x = rem_to_px(spec.surface_padding_x_rem(density));
    let pad_y = rem_to_px(spec.surface_padding_y_rem(density));

    let is_stacked = spec.layout == DetailItemLayout::Stacked;
    let is_surface = spec.presentation == DetailItemPresentation::Surface;
    let is_surface_stacked = is_surface && is_stacked;

    let (eff_label_color, eff_label_font, eff_label_line_height) = if is_surface_stacked {
        (tertiary_color, rem_to_px(0.75), 1.35)
    } else {
        (label_color, label_font, label_line_height / label_font)
    };
    let (eff_value_font, value_weight) = if is_surface_stacked {
        (rem_to_px(1.0), 600u16)
    } else {
        (value_font, 400u16)
    };

    // ── Label block ──
    let mut label_block = Node::container();
    part(&mut label_block, "label-block");
    label_block.style.descriptor.layout.direction = LayoutDirection::Column;
    label_block.style.descriptor.layout.spacing.gap = row_gap;
    let l = detail_text(
        &spec.label,
        "label",
        eff_label_color,
        eff_label_font,
        TextWeight::Normal,
        eff_label_line_height,
        ctx,
    );
    label_block = label_block.child(l);
    if let Some(ref desc) = spec.description {
        // Svelte wraps a role-less icon span in the Popover trigger, so the
        // composition projects exactly one button. The trigger name comes from
        // the icon's own label, which the native trigger cannot read as
        // visible text, so it is set on the wrapper here.
        let mut glyph = icon(&IconSpec::new("info"), ctx);
        glyph.a11y.role = None;
        glyph.a11y.label = None;
        let mut trigger = Node::container();
        part(&mut trigger, "info-trigger");
        trigger = trigger.child(glyph);
        let popover_spec = PopoverSpec::new()
            .with_open(description_open)
            .with_aria_label(INFO_LABEL);
        let handlers = PopoverHandlers {
            instance_id: Some("detail-item-info".to_owned()),
            on_activate: on_description_toggle,
            ..PopoverHandlers::default()
        };
        let mut info = popover(
            &popover_spec,
            ctx,
            &handlers,
            Some(trigger),
            Some(detail_text(
                desc,
                "supporting",
                desc_color,
                desc_font,
                TextWeight::Normal,
                1.5,
                ctx,
            )),
        );
        part(&mut info, "info");
        if let Some(node) = find_id_mut(&mut info, POPOVER_TRIGGER_ID) {
            node.a11y.label = Some(INFO_LABEL.to_string());
        }
        label_block = label_block.child(info);
    }
    if !is_stacked {
        label_block.style.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(11.25));
        label_block.style.flex_shrink_zero = true;
    }

    // ── Value: slot > text > em-dash placeholder ──
    let value_block = if let Some(content) = value_content {
        let mut wrap = Node::container();
        part(&mut wrap, "value");
        wrap.roles
            .insert("value-kind".to_owned(), "custom".to_owned());
        // Explicit Row (see switch.rs), preemptively — this slot path has no
        // old-tier caller in the fixtures but the same silent-Row shape.
        wrap.style.descriptor.layout.direction = LayoutDirection::Row;
        wrap.style.descriptor.layout.width = LayoutSizing::Grow;
        wrap.style.min_width = Some(0.0);
        wrap.child(content)
    } else if let Some(ref value) = spec.value {
        let weight = if value_weight == 600 {
            TextWeight::Semibold
        } else {
            TextWeight::Normal
        };
        let mut v = detail_text(
            value,
            "value",
            value_color,
            eff_value_font,
            weight,
            value_line_height / eff_value_font,
            ctx,
        );
        v.roles.insert("value-kind".to_owned(), "text".to_owned());
        v.style.descriptor.layout.width = LayoutSizing::Grow;
        v.style.min_width = Some(0.0);
        if spec.truncate_value {
            v.style.text_ellipsis = true;
            v.style.text_wrap = false;
            v.style.no_wrap = true;
        }
        v
    } else {
        let weight = if value_weight == 600 {
            TextWeight::Semibold
        } else {
            TextWeight::Normal
        };
        let mut v = detail_text(
            &spec.empty_text,
            "value",
            value_color,
            eff_value_font,
            weight,
            value_line_height / eff_value_font,
            ctx,
        );
        v.roles.insert("value-kind".to_owned(), "empty".to_owned());
        v.style.descriptor.layout.width = LayoutSizing::Grow;
        v.style.min_width = Some(0.0);
        v
    };

    // ── Root ──
    let mut el = Node::container();
    el.roles
        .insert("component".to_owned(), "detail-item".to_owned());
    el.roles.insert(
        "layout".to_owned(),
        if is_stacked { "stacked" } else { "inline" }.to_owned(),
    );
    el.roles.insert(
        "presentation".to_owned(),
        if is_surface { "surface" } else { "simple" }.to_owned(),
    );
    el.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );
    el.roles.insert(
        "span".to_owned(),
        match spec.span {
            Some(DetailItemSpan::Full) => "full",
            Some(DetailItemSpan::Half) => "half",
            None => "none",
        }
        .to_owned(),
    );
    el.roles
        .insert("truncate".to_owned(), spec.truncate_value.to_string());
    {
        let s = &mut el.style;
        if is_stacked {
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = if is_surface_stacked {
                surface_stacked_gap
            } else {
                row_gap
            };
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = if is_surface {
                CrossAxisAlignment::Center
            } else {
                CrossAxisAlignment::Baseline
            };
            s.descriptor.layout.spacing.gap = inline_gap;
        }
        if is_surface_stacked {
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        }
        if is_surface {
            s.descriptor.background = Some(mix_srgb(surface, value_color, 0.93));
            s.descriptor.corner_radii.top_left = radius;
            s.descriptor.corner_radii.top_right = radius;
            s.descriptor.corner_radii.bottom_right = radius;
            s.descriptor.corner_radii.bottom_left = radius;
            s.descriptor.layout.spacing.padding.left = pad_x;
            s.descriptor.layout.spacing.padding.right = pad_x;
            s.descriptor.layout.spacing.padding.top = pad_y;
            s.descriptor.layout.spacing.padding.bottom = pad_y;
        }
        if matches!(spec.span, Some(DetailItemSpan::Full)) {
            s.self_stretch = true;
        }
        if matches!(spec.span, Some(DetailItemSpan::Half)) {
            s.width_pct = Some(0.5);
        } else {
            s.fill_width = true;
        }
    }

    let action = action.map(|action_el| {
        let mut slot = Node::container();
        part(&mut slot, "action");
        {
            let s = &mut slot.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.flex_shrink_zero = true;
        }
        slot.child(action_el)
    });

    el = el.child(label_block);
    if is_surface_stacked {
        let mut content = Node::container();
        part(&mut content, "content");
        content.style.descriptor.layout.direction = LayoutDirection::Row;
        content.style.descriptor.layout.spacing.gap = surface_stacked_gap;
        content.style.descriptor.layout.width = LayoutSizing::Grow;
        content.style.min_width = Some(0.0);
        content = content.child(value_block);
        if let Some(action) = action {
            content = content.child(action);
        }
        el = el.child(content);
    } else {
        el = el.child(value_block);
        if let Some(action) = action {
            el = el.child(action);
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}
