//! SelectionSummary — chip row for a selection, with overflow and clear.
//!
//! Contract: `docs/contracts/components/selection-summary.md`
//! Ported from: `packages/jetstream/components/src/selection_summary.rs`.
//!
//! Chip/overflow fills and stacked borders follow the old GPUI tier's
//! elevated fill and reduced-alpha border recipe.

use std::sync::Arc;

use poodle_node::{CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, Node};
use poodle_specs::{ControlDensity, SelectionSummarySpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::{control_space_x_rem, rem_to_px, size_font_rem};

/// Host callbacks: per-chip remove (item id) + clear-all.
#[derive(Default)]
pub struct SelectionSummaryHandlers {
    pub on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_clear: Option<Arc<dyn Fn() + Send + Sync>>,
}

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

pub fn selection_summary(
    spec: &SelectionSummarySpec,
    ctx: &RenderContext<'_>,
    handlers: SelectionSummaryHandlers,
) -> Node {
    selection_summary_inner(spec, ctx, handlers, None, None)
}

/// Render a SelectionSummary with its optional split-chip activation callback
/// and a stable instance identity for the mounted controls.
pub fn selection_summary_with_actions(
    spec: &SelectionSummarySpec,
    ctx: &RenderContext<'_>,
    handlers: SelectionSummaryHandlers,
    on_activate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    instance_id: &str,
) -> Node {
    selection_summary_inner(spec, ctx, handlers, on_activate, Some(instance_id))
}

fn selection_summary_inner(
    spec: &SelectionSummarySpec,
    ctx: &RenderContext<'_>,
    handlers: SelectionSummaryHandlers,
    on_activate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    instance_id: Option<&str>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let chip_font = rem_to_px(SelectionSummarySpec::chip_font_rem(effective_size));
    // Overflow badge carries its own font-size per size, distinct from chips.
    let overflow_font = rem_to_px(SelectionSummarySpec::overflow_font_rem(effective_size));
    let overflow_line_height = SelectionSummarySpec::overflow_line_height_rem(effective_size)
        / SelectionSummarySpec::overflow_font_rem(effective_size);
    let gap = rem_to_px(match density {
        ControlDensity::Compact => 0.375,
        ControlDensity::Default => control_space_x_rem(density),
        ControlDensity::Comfortable => 0.75,
    });
    let chip_radius = ctx.theme().resolve_radius(spec.radius_token());
    let chip_border_width = ctx.theme().resolve_space(spec.border_width_token());

    let text_color = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let text_tertiary = ctx.theme().resolve_color("color.text.tertiary");
    let elevated = ctx.theme().resolve_color("color.background.elevated");
    // The old GPUI tier keeps the elevated hue and only uses the surface
    // alpha in its Hsla construction. Both source fills are opaque, so the
    // effective fill is elevated. Borders are stacked at 70% alpha.
    let chip_bg = elevated;
    let overflow_bg = elevated;
    let chip_border_base = ctx.theme().resolve_color("color.border.subtle");
    let chip_border = with_alpha(chip_border_base, chip_border_base.3 * 0.7);
    let accent = ctx.theme().resolve_color("color.accent.base");
    let focus_ring = FocusRing {
        color: ctx.theme().resolve_color("color.accent.focusRing"),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset: rem_to_px(0.0625),
    };
    let bottom_pad = rem_to_px(match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.625,
        ControlDensity::Comfortable => 0.75,
    });
    let chip_px = rem_to_px(match density {
        ControlDensity::Compact => 0.625,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 0.875,
    });
    let overflow_px = rem_to_px(match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.625,
        ControlDensity::Comfortable => 0.75,
    });
    let chip_min_h = rem_to_px(SelectionSummarySpec::chip_min_height_rem(effective_size));

    let mut el = Node::container();
    el.a11y.role = Some(poodle_node::NodeRole::Region);
    el.a11y.label = Some("Current selection".to_owned());
    if let Some(instance_id) = instance_id {
        el.runtime_id = Some(format!("{instance_id}:root"));
    }
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.flex_wrap = true;
        s.self_stretch = true;
        s.descriptor.layout.spacing.padding.bottom = bottom_pad;
        s.min_height = Some(chip_min_h);
    }

    if spec.items.is_empty() {
        let mut empty = Node::text("No selection");
        empty.style.descriptor.text_color = Some(text_tertiary);
        empty.style.text_size = Some(chip_font);
        empty.style.text_italic = true;
        return el.child(empty);
    }

    for item in spec.items.iter().take(spec.visible_item_count()) {
        if let Some(on_activate) = &on_activate {
            let mut activate = Node::button(&item.label);
            if let Some(instance_id) = instance_id {
                activate.runtime_id = Some(format!("{instance_id}:activate:{}", item.id));
            }
            activate.a11y.role = Some(poodle_node::NodeRole::Button);
            activate.a11y.label = Some(format!("Edit {}", item.label));
            activate.a11y.tab_index = Some(0);
            activate.interaction.focusable = true;
            activate.style.descriptor.layout.direction = LayoutDirection::Row;
            activate.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            activate.style.descriptor.layout.spacing.padding.left = chip_px;
            activate.style.descriptor.layout.spacing.padding.right = chip_px;
            activate.style.descriptor.text_color = Some(text_color);
            activate.style.text_size = Some(chip_font);
            activate.style.descriptor.background = Some(chip_bg);
            activate.style.descriptor.border.width = chip_border_width;
            activate.style.descriptor.border.color = chip_border;
            activate.style.min_height = Some(chip_min_h);
            activate.style.descriptor.cursor = CursorHint::Pointer;
            activate.style.focus_ring = Some(focus_ring.clone());
            all_corners(&mut activate, chip_radius);
            let activate_handler = Arc::clone(on_activate);
            let activate_id = item.id.clone();
            activate.interaction.on_activate =
                Some(Arc::new(move || activate_handler(&activate_id)));

            let mut remove = Node::button("\u{2715}");
            if let Some(instance_id) = instance_id {
                remove.runtime_id = Some(format!("{instance_id}:remove:{}", item.id));
            }
            remove.a11y.role = Some(poodle_node::NodeRole::Button);
            remove.a11y.label = Some(format!("Remove {}", item.label));
            remove.a11y.tab_index = Some(0);
            remove.interaction.focusable = true;
            remove.style.descriptor.layout.direction = LayoutDirection::Row;
            remove.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            remove.style.descriptor.layout.spacing.padding.left = chip_px * 0.5;
            remove.style.descriptor.layout.spacing.padding.right = chip_px * 0.5;
            remove.style.descriptor.text_color = Some(text_secondary);
            remove.style.text_size = Some(chip_font);
            remove.style.descriptor.background = Some(chip_bg);
            remove.style.descriptor.border.width = chip_border_width;
            remove.style.descriptor.border.color = chip_border;
            remove.style.min_height = Some(chip_min_h);
            remove.style.descriptor.cursor = CursorHint::Pointer;
            remove.style.focus_ring = Some(focus_ring.clone());
            all_corners(&mut remove, chip_radius);
            if let Some(handler) = &handlers.on_remove {
                let handler = Arc::clone(handler);
                let id = item.id.clone();
                remove.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }

            let mut split_chip = Node::container().child(activate).child(remove);
            split_chip.style.descriptor.layout.direction = LayoutDirection::Row;
            split_chip.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            split_chip.style.descriptor.layout.spacing.gap = gap;
            el = el.child(split_chip);
        } else {
            let mut chip = Node::button("");
            if let Some(instance_id) = instance_id {
                chip.runtime_id = Some(format!("{instance_id}:remove:{}", item.id));
            }
            chip.a11y.role = Some(poodle_node::NodeRole::Button);
            chip.a11y.label = Some(format!("Remove {}", item.label));
            chip.a11y.tab_index = Some(0);
            chip.interaction.focusable = true;
            chip.style.focus_ring = Some(focus_ring.clone());
            {
                let s = &mut chip.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = gap;
                s.descriptor.text_color = Some(text_color);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = chip_px;
                pad.right = chip_px;
                s.min_height = Some(chip_min_h);
                s.descriptor.background = Some(chip_bg);
                s.descriptor.border.width = chip_border_width;
                s.descriptor.border.color = chip_border;
            }
            all_corners(&mut chip, chip_radius);
            let mut label = Node::text(&item.label);
            label.style.descriptor.text_color = Some(text_color);
            label.style.text_size = Some(chip_font);
            // Anatomy is ChipLabel + RemoveIcon only (contract §2).
            let mut x = Node::text("\u{2715}");
            x.style.descriptor.text_color = Some(text_secondary);
            x.style.text_size = Some(chip_font);
            let mut chip = chip.child(label).child(x);

            if let Some(handler) = &handlers.on_remove {
                let handler = Arc::clone(handler);
                let id = item.id.clone();
                chip.style.descriptor.cursor = CursorHint::Pointer;
                chip.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }

            el = el.child(chip);
        }
    }

    if spec.overflow_count() > 0 {
        let mut overflow = Node::text(format!("+{} more", spec.overflow_count()));
        {
            let s = &mut overflow.style;
            s.descriptor.text_color = Some(text_secondary);
            s.text_size = Some(overflow_font);
            s.line_height = Some(overflow_line_height);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = overflow_px;
            pad.right = overflow_px;
            s.min_height = Some(chip_min_h);
            s.descriptor.background = Some(overflow_bg);
            s.descriptor.border.width = chip_border_width;
            s.descriptor.border.color = chip_border;
        }
        all_corners(&mut overflow, chip_radius);
        el = el.child(overflow);
    }

    // Clear link — rendered whenever the selection is populated (contract
    // §4). Label defaults to "Clear", overridable via clear_action.
    let clear_label = spec
        .clear_action
        .as_ref()
        .map(|c| c.label.clone())
        .unwrap_or_else(|| "Clear".to_string());
    let mut clear = Node::button(&clear_label);
    if let Some(instance_id) = instance_id {
        clear.runtime_id = Some(format!("{instance_id}:clear"));
    }
    clear.a11y.role = Some(poodle_node::NodeRole::Link);
    clear.a11y.label = Some(clear_label.clone());
    clear.a11y.tab_index = Some(0);
    clear.style.descriptor.text_color = Some(accent);
    clear.style.text_size = Some(font_size);
    clear.style.focus_ring = Some(FocusRing {
        offset: rem_to_px(0.125),
        ..focus_ring
    });
    clear.interaction.focusable = true;
    if let Some(handler) = handlers.on_clear {
        clear.style.descriptor.cursor = CursorHint::Pointer;
        clear.interaction.on_activate = Some(Arc::new(move || handler()));
    }

    let mut clear_lane = Node::container();
    {
        let s = &mut clear_lane.style;
        s.flex_fill = true;
        s.descriptor.layout.direction = LayoutDirection::Row;
    }
    el.child(clear_lane.child(clear))
}
