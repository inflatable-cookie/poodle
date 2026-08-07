//! Collapsible — a single disclosure region with heading, chevron, content.
//!
//! Contract: `docs/contracts/components/collapsible.md`
//! Ported from: `packages/jetstream/components/src/collapsible.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, Node, NodeRole, ShadowLayer};
use poodle_specs::{CollapsibleSpec, ControlDensity, ControlSize};

use crate::color::{mix_srgb, with_alpha};
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};

pub fn collapsible(
    spec: &CollapsibleSpec,
    theme: &dyn ThemeProvider,
    content: Option<Node>,
    on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let is_open = spec.current_open();

    let open_gap = theme.resolve_space("space.stack.md");
    let root_gap = if is_open { open_gap } else { 0.0 };
    let trigger_gap = theme.resolve_space("space.inline.md");

    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let title_font = rem_to_px(match effective_size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.0625,
        ControlSize::Xl => 1.125,
    });
    let desc_font = rem_to_px(size_font_rem(effective_size));
    let icon_size = rem_to_px(0.75);

    let elevated = theme.resolve_color("color.background.elevated");
    let panel = theme.resolve_color("color.background.panel");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let accent_base = theme.resolve_color(spec.highlight_accent_token());
    let radius = theme.resolve_radius("radius.surface");

    let root_bg = mix_srgb(elevated, panel, 0.40);
    let root_border = with_alpha(border_subtle, border_subtle.3 * spec.border_subtle_alpha());
    let highlight_border = with_alpha(accent_base, accent_base.3 * spec.highlight_border_alpha());
    let highlight_halo = with_alpha(accent_base, accent_base.3 * spec.highlight_halo_alpha());

    let pad_y = rem_to_px(0.625);
    let pad_x = rem_to_px(match spec.density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.0,
    });

    let mut outer = Node::container();
    {
        let s = &mut outer.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
        s.min_width = Some(0.0);
        s.self_stretch = true;
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.spacing.padding.top = pad_y;
        s.descriptor.layout.spacing.padding.bottom = pad_y;
        s.descriptor.background = Some(root_bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = if spec.highlighted {
            highlight_border
        } else {
            root_border
        };
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        if spec.highlighted {
            s.shadow_layers = vec![ShadowLayer {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: rem_to_px(0.125),
                color: highlight_halo,
                inset: false,
            }];
        }
    }

    // Trigger: heading block + chevron.
    let mut heading = Node::container();
    {
        let s = &mut heading.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.min_width = Some(0.0);
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
    }
    if let Some(ref title) = spec.title {
        let mut t = Node::text(title);
        t.style.descriptor.text_color = Some(text_primary);
        t.style.text_size = Some(title_font);
        t.style.text_weight = Some(700);
        t.style.line_height = Some(1.2);
        heading = heading.child(t);
    }
    if let Some(ref description) = spec.description {
        let mut d = Node::text(description);
        d.style.descriptor.text_color = Some(text_secondary);
        d.style.text_size = Some(desc_font);
        d.style.line_height = Some(1.45);
        heading = heading.child(d);
    }

    let chevron_icon = if is_open {
        "chevron-down"
    } else {
        "chevron-right"
    };
    let mut indicator = Node::icon(chevron_icon, icon_size);
    indicator.style.flex_shrink_zero = true;
    indicator.style.descriptor.text_color = Some(text_secondary);

    let mut trigger = Node::container();
    {
        let s = &mut trigger.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = trigger_gap;
        s.fill_width = true;
        s.descriptor.cursor = CursorHint::Pointer;
    }
    trigger.interaction.focusable = true;
    trigger = trigger.child(heading).child(indicator);

    if let (false, Some(handler)) = (spec.is_disabled, &on_open_change) {
        let handler = Arc::clone(handler);
        let next = !is_open;
        trigger.interaction.on_activate = Some(Arc::new(move || handler(next)));
    }
    outer = outer.child(trigger);

    // Content region only when open.
    if is_open {
        if let Some(content_el) = content {
            let mut wrapper = Node::container();
            {
                let s = &mut wrapper.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.min_width = Some(0.0);
                s.self_stretch = true;
                s.descriptor.layout.spacing.padding.top = rem_to_px(0.125);
            }
            outer = outer.child(wrapper.child(content_el));
        }
    }

    // Disabled: whole-element opacity (contract §8 Root disabled).
    if spec.is_disabled {
        outer.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
    }

    if let Some(label) = spec.aria_label.as_deref() {
        outer.a11y.label = Some(label.to_string());
    }
    outer.a11y.role = Some(NodeRole::Region);
    outer.a11y.expanded = Some(spec.open.unwrap_or(false));
    outer
}
