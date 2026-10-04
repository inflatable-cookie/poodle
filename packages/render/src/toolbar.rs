//! Toolbar — a grouping chrome for compact action controls.
//!
//! Contract: `docs/contracts/components/toolbar.md`
//! Ported from: `packages/jetstream/components/src/toolbar.rs`.

use std::sync::{Arc, Mutex};

use poodle_node::{
    CrossAxisAlignment, FocusRing, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeKey, NodeRole,
};
use poodle_specs::{Alignment, Orientation, ToolbarSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::{
    rem_to_px, toolbar_density_gap_rem, toolbar_density_pad_inline_rem, toolbar_gap_rem,
    toolbar_pad_block_rem, toolbar_pad_inline_rem,
};

fn collect_focus_targets(
    node: &mut Node,
    scope: &str,
    targets: &mut Vec<String>,
    current: &Arc<Mutex<Option<usize>>>,
) {
    if node.interaction.focusable && !node.interaction.disabled {
        let index = targets.len();
        let id = node
            .runtime_id
            .as_ref()
            .or(node.id.as_ref())
            .cloned()
            .unwrap_or_else(|| format!("{scope}:item:{index}"));
        if node.runtime_id.is_none() && node.id.is_none() {
            node.runtime_id = Some(id.clone());
        }
        let previous = node.interaction.on_focus_change.take();
        let current = Arc::clone(current);
        node.interaction.on_focus_change = Some(Arc::new(move |focused| {
            if focused {
                *current.lock().expect("toolbar focus state") = Some(index);
            }
            if let Some(previous) = &previous {
                previous(focused);
            }
        }));
        targets.push(id);
    }

    for child in &mut node.children {
        collect_focus_targets(child, scope, targets, current);
    }
}

pub fn toolbar(spec: &ToolbarSpec, ctx: &RenderContext<'_>, children: Vec<Node>) -> Node {
    let panel_raw = ctx.theme().resolve_color(spec.bg_token());
    let bg = with_alpha(panel_raw, panel_raw.3 * 0.94);
    let border_raw = ctx.theme().resolve_color(spec.border_token());
    let border = with_alpha(border_raw, border_raw.3 * 0.78);
    let radius = ctx.theme().resolve_radius(spec.radius_token());

    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let pad_v = rem_to_px(toolbar_pad_block_rem(effective_size));
    let pad_h = rem_to_px(
        toolbar_density_pad_inline_rem(density)
            .unwrap_or_else(|| toolbar_pad_inline_rem(effective_size)),
    );
    let gap = rem_to_px(
        toolbar_density_gap_rem(density).unwrap_or_else(|| toolbar_gap_rem(effective_size)),
    );

    let is_vertical = spec.orientation == Orientation::Vertical;
    let label = spec.aria_label.as_deref().unwrap_or("Toolbar");
    let scope = format!("toolbar:{}", label.to_ascii_lowercase().replace(' ', "-"));
    let focus_state = Arc::new(Mutex::new(None));
    let mut children = children;
    let mut focus_targets = Vec::new();
    for child in &mut children {
        collect_focus_targets(child, &scope, &mut focus_targets, &focus_state);
    }

    let mut el = Node::container();
    el.runtime_id = Some(format!("{scope}:root"));
    {
        let s = &mut el.style;
        if is_vertical {
            s.descriptor.layout.direction = LayoutDirection::Column;
            // items_stretch: taffy's default cross alignment — no call needed.
        } else {
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        }
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.spacing.padding.top = pad_v;
        s.descriptor.layout.spacing.padding.bottom = pad_v;
        s.descriptor.layout.spacing.padding.left = pad_h;
        s.descriptor.layout.spacing.padding.right = pad_h;
        s.descriptor.background = Some(bg);
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color("color.accent.focusRing"),
            width: ctx.theme().resolve_border_width("border.width.focus"),
            offset: rem_to_px(0.125),
        });
        match spec.alignment {
            Alignment::Start => {}
            Alignment::Center => s.descriptor.layout.alignment.main = MainAxisAlignment::Center,
            Alignment::End => s.descriptor.layout.alignment.main = MainAxisAlignment::End,
            Alignment::Stretch => s.descriptor.layout.width = LayoutSizing::Grow,
        }
    }

    for child in children {
        el = el.child(child);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::Toolbar);
    el.a11y.orientation = Some(if is_vertical {
        "vertical".to_owned()
    } else {
        "horizontal".to_owned()
    });
    el.a11y.tab_index = Some(0);
    el.interaction.focusable = true;
    let focus_state_on_entry = Arc::clone(&focus_state);
    el.interaction.on_focus_change = Some(Arc::new(move |focused| {
        if focused {
            *focus_state_on_entry
                .lock()
                .expect("toolbar focus state") = None;
        }
    }));
    let targets = focus_targets;
    let current = Arc::clone(&focus_state);
    let orientation = spec.orientation;
    el.interaction.on_key = Some(Arc::new(move |key, _modifiers| {
        let direction = match (orientation, key) {
            (Orientation::Horizontal, NodeKey::ArrowRight)
            | (Orientation::Vertical, NodeKey::ArrowDown) => 1isize,
            (Orientation::Horizontal, NodeKey::ArrowLeft)
            | (Orientation::Vertical, NodeKey::ArrowUp) => -1isize,
            _ => return None,
        };
        if targets.is_empty() {
            return None;
        }
        let next = match *current.lock().expect("toolbar focus state") {
            None => 0,
            Some(index) => {
                (index as isize + direction).rem_euclid(targets.len() as isize) as usize
            }
        };
        Some(targets[next].clone())
    }));
    el
}
