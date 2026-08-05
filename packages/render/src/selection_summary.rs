//! SelectionSummary — chip row for a selection, with overflow and clear.
//!
//! Contract: `docs/contracts/components/selection-summary.md`
//! Ported from: `packages/jetstream/components/src/selection_summary.rs`.
//!
//! Chip/overflow fills are the old tier's linear-space lerps between
//! elevated and surface → `mix_linear`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, MainAxisAlignment, Node,
};
use poodle_specs::{ControlDensity, SelectionSummarySpec};

use crate::color::mix_linear;
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};

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
    theme: &dyn ThemeProvider,
    handlers: SelectionSummaryHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let chip_font = rem_to_px(SelectionSummarySpec::chip_font_rem(effective_size));
    // Overflow badge carries its own font-size per size, distinct from chips.
    let overflow_font = rem_to_px(SelectionSummarySpec::overflow_font_rem(effective_size));
    let gap = rem_to_px(match spec.density {
        ControlDensity::Compact => 0.375,
        ControlDensity::Default => control_space_x_rem(spec.density),
        ControlDensity::Comfortable => 0.75,
    });
    let chip_radius = theme.resolve_radius(spec.radius_token());
    let chip_border_width = theme.resolve_space(spec.border_width_token());

    let text_color = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_tertiary = theme.resolve_color("color.text.tertiary");
    let surface = theme.resolve_color("color.background.surface");
    let elevated = theme.resolve_color("color.background.elevated");
    // Linear-space lerps toward surface (glam Vec4::lerp in the reference).
    let chip_bg = mix_linear(surface, elevated, 0.40);
    let overflow_bg = mix_linear(surface, elevated, 0.32);
    let chip_border = theme.resolve_color("color.border.subtle");
    let accent = theme.resolve_color("color.accent.base");
    let bottom_pad = rem_to_px(match spec.density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.625,
        ControlDensity::Comfortable => 0.75,
    });
    let chip_px = rem_to_px(match spec.density {
        ControlDensity::Compact => 0.625,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 0.875,
    });
    let overflow_px = rem_to_px(match spec.density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.625,
        ControlDensity::Comfortable => 0.75,
    });
    let chip_min_h = rem_to_px(SelectionSummarySpec::chip_min_height_rem(effective_size));

    let mut el = Node::container();
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
        return el.child(empty);
    }

    for item in spec.items.iter().take(spec.visible_item_count()) {
        let mut chip = Node::button("");
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
        let mut x = Node::text("×");
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

    if spec.overflow_count() > 0 {
        let mut overflow = Node::text(format!("+{} more", spec.overflow_count()));
        {
            let s = &mut overflow.style;
            s.descriptor.text_color = Some(text_secondary);
            s.text_size = Some(overflow_font);
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
    clear.style.descriptor.text_color = Some(accent);
    clear.style.text_size = Some(font_size);
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
        s.descriptor.layout.alignment.main = MainAxisAlignment::End;
    }
    el.child(clear_lane.child(clear))
}
