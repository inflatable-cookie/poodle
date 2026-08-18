//! EmptyState — dashed frame, variant icon, copy block, remediation actions.
//!
//! Contract: `docs/contracts/components/empty-state.md`
//! Ported from: `packages/jetstream/components/src/empty_state.rs`. Actions
//! compose [`crate::button::button`].

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node};
use poodle_specs::{
    ButtonSpec, ControlDensity, ControlSize, EmptyStateSize, EmptyStateSpec, EmptyStateVariant,
    SemanticControlSizeRole,
};

use crate::button::button;
use crate::color::with_alpha;
use crate::presentation::rem_to_px;

pub fn empty_state(spec: &EmptyStateSpec, theme: &dyn ThemeProvider) -> Node {
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let gap = theme.resolve_space(spec.layout_gap_token());

    let compact = spec.size == EmptyStateSize::Compact;
    let effective_size = if compact {
        ControlSize::Sm
    } else {
        ControlSize::Md
    };
    let title_font = if compact {
        rem_to_px(0.9375)
    } else {
        rem_to_px(1.125)
    };
    let message_font = if compact {
        rem_to_px(0.75)
    } else {
        rem_to_px(0.8125)
    };

    let icon_name = match spec.variant {
        EmptyStateVariant::Search => "search",
        EmptyStateVariant::FirstRun => "plus",
        EmptyStateVariant::Neutral => "inbox",
    };
    let icon_container = if compact {
        rem_to_px(1.75)
    } else {
        rem_to_px(2.25)
    };
    let icon_font = if compact {
        rem_to_px(0.9375)
    } else {
        rem_to_px(1.125)
    };

    let panel = theme.resolve_color("color.background.panel");
    let icon_bg = with_alpha(panel, panel.3 * 0.90);

    let border_default = theme.resolve_color("color.border.default");
    let root_radius = (theme.resolve_radius("radius.surface") - rem_to_px(0.125)).max(0.0);

    let root_bg = match spec.variant {
        EmptyStateVariant::Neutral => {
            let c = theme.resolve_color("color.background.surface");
            with_alpha(c, c.3 * 0.76)
        }
        EmptyStateVariant::Search => {
            let c = theme.resolve_color("color.accent.base");
            with_alpha(c, c.3 * 0.07)
        }
        EmptyStateVariant::FirstRun => {
            let c = theme.resolve_color("color.status.success");
            with_alpha(c, c.3 * 0.07)
        }
    };

    let vertical_padding = match spec.density {
        ControlDensity::Compact => theme.resolve_space("space.stack.lg"),
        ControlDensity::Default => theme.resolve_space("space.panel.y") * 1.5,
        ControlDensity::Comfortable => theme.resolve_space("space.panel.y") * 2.0,
    };
    let horiz_padding = theme.resolve_space("space.panel.x");

    // Circular visual affordance.
    let mut visual = Node::container();
    {
        let s = &mut visual.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(icon_container);
        s.descriptor.layout.height = LayoutSizing::Fixed(icon_container);
        s.descriptor.corner_radii.top_left = 999.0;
        s.descriptor.corner_radii.top_right = 999.0;
        s.descriptor.corner_radii.bottom_right = 999.0;
        s.descriptor.corner_radii.bottom_left = 999.0;
        s.descriptor.background = Some(icon_bg);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    let mut glyph = Node::icon(icon_name, icon_font);
    glyph.style.descriptor.text_color = Some(text_secondary);
    let visual = visual.child(glyph);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.spacing.padding.top = vertical_padding;
        s.descriptor.layout.spacing.padding.bottom = vertical_padding;
        s.descriptor.layout.spacing.padding.left = horiz_padding;
        s.descriptor.layout.spacing.padding.right = horiz_padding;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border_default;
        s.border_dashed = true;
        s.descriptor.corner_radii.top_left = root_radius;
        s.descriptor.corner_radii.top_right = root_radius;
        s.descriptor.corner_radii.bottom_right = root_radius;
        s.descriptor.corner_radii.bottom_left = root_radius;
        s.descriptor.background = Some(root_bg);
        s.fill_width = true;
    }
    el = el.child(visual);

    // Copy block.
    let mut copy = Node::container();
    {
        let s = &mut copy.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
    }
    let mut title = Node::text(&spec.title);
    title.style.descriptor.text_color = Some(text_primary);
    title.style.text_size = Some(title_font);
    title.style.text_weight = Some(600);
    title.style.text_align = Some(poodle_node::TextAlign::Center);
    copy = copy.child(title);

    if let Some(ref desc) = spec.message {
        let mut m = Node::text(desc);
        m.style.descriptor.text_color = Some(text_secondary);
        m.style.text_size = Some(message_font);
        m.style.text_align = Some(poodle_node::TextAlign::Center);
        m.style.max_width = Some(rem_to_px(24.0));
        copy = copy.child(m);
    }
    el = el.child(copy);

    // Actions compose the ported button.
    if spec.action_count() > 0 {
        let mut actions = Node::container();
        {
            let s = &mut actions.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        }
        for action in &spec.actions {
            let btn_spec = ButtonSpec::new()
                .with_label(&action.label)
                .with_variant(action.variant)
                .with_disabled(action.is_disabled)
                .with_size(effective_size)
                .with_size_role(SemanticControlSizeRole::Control);
            actions = actions.child(button(&btn_spec, theme, None));
        }
        el = el.child(actions);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::{LayoutSizing, NodeKind};

    fn walk<'a>(node: &'a Node, visit: &mut impl FnMut(&'a Node)) {
        visit(node);
        for child in &node.children {
            walk(child, visit);
        }
    }

    fn icon_container_side(node: &Node) -> Option<f32> {
        let mut found = None;
        walk(node, &mut |candidate| {
            if !matches!(candidate.kind, NodeKind::Container) {
                return;
            }
            let LayoutSizing::Fixed(width) = candidate.style.descriptor.layout.width else {
                return;
            };
            if candidate
                .children
                .iter()
                .any(|child| matches!(child.kind, NodeKind::Icon { .. }))
            {
                found = Some(width);
            }
        });
        found
    }

    fn title_text_size(node: &Node) -> Option<f32> {
        let mut found = None;
        walk(node, &mut |candidate| {
            if matches!(candidate.kind, NodeKind::Text { .. })
                && candidate.style.text_size.is_some()
                && candidate.style.text_weight == Some(600)
            {
                found = candidate.style.text_size;
            }
        });
        found
    }

    fn icon_glyph_size(node: &Node) -> Option<f32> {
        let mut found = None;
        walk(node, &mut |candidate| {
            if let NodeKind::Icon { size, .. } = &candidate.kind {
                found = Some(*size);
            }
        });
        found
    }

    #[test]
    fn compact_and_default_sizes_use_distinct_title_and_icon_geometry() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let default = empty_state(&EmptyStateSpec::new("No projects yet"), &theme);
        let compact = empty_state(
            &EmptyStateSpec::new("No projects yet").with_size(EmptyStateSize::Compact),
            &theme,
        );

        let default_title = title_text_size(&default).expect("default title size");
        let compact_title = title_text_size(&compact).expect("compact title size");
        assert!(compact_title < default_title);

        let default_icon_box = icon_container_side(&default).expect("default icon box");
        let compact_icon_box = icon_container_side(&compact).expect("compact icon box");
        assert!(compact_icon_box < default_icon_box);

        let default_glyph = icon_glyph_size(&default).expect("default glyph");
        let compact_glyph = icon_glyph_size(&compact).expect("compact glyph");
        assert!(compact_glyph < default_glyph);
    }
}
