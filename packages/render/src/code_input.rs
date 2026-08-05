//! CodeInput — segmented code entry with visual digit slots.
//!
//! Contract: `docs/contracts/components/code-input.md`
//! Ported from: `packages/jetstream/components/src/code_input.rs`.
//!
//! Runtime gaps (noted): the real/hidden input, paste, autofill, one-time-code
//! autocomplete, slot-click caret placement and the text caret are host/engine
//! concerns. This builder renders the slot grid, the distributed value, the
//! active-slot highlight, and the error label.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, FontFamily, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    TextAlign,
};
use poodle_specs::{CodeInputSpec, ControlDensity, ValidationState};

use crate::presentation::{
    code_input_slot_font_rem, code_input_slot_size_rem, rem_to_px, resolve_semantic_size,
};

pub fn code_input(spec: &CodeInputSpec, theme: &dyn ThemeProvider) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let validation = spec.effective_validation_state();
    let is_invalid = validation == ValidationState::Invalid;

    // ── Token resolution ──
    let surface = theme.resolve_color("color.background.surface");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_default = theme.resolve_color("color.border.default");
    let accent_border = theme.resolve_color("color.accent.border");
    let danger = theme.resolve_color("color.status.danger");
    let radius = theme.resolve_radius("radius.control");

    // Contract §7: only the invalid case (or an error) changes slot colors.
    let slot_border = if is_invalid { danger } else { border_default };
    let active_border = if is_invalid { danger } else { accent_border };

    // ── Sizing (contract §7) ──
    // Slots are square at every size; font follows the slot font ladder.
    let slot_size = rem_to_px(code_input_slot_size_rem(effective_size));
    let font_size = rem_to_px(code_input_slot_font_rem(effective_size));
    let border_width = rem_to_px(0.0625); // 1px = 0.0625rem, contract border width
    // Inter-slot gap: compact space.inline.xs, default space.inline.sm,
    // comfortable space.inline.md.
    let gap = match spec.density {
        ControlDensity::Compact => theme.resolve_space("space.inline.xs"),
        ControlDensity::Default => theme.resolve_space("space.inline.sm"),
        ControlDensity::Comfortable => theme.resolve_space("space.inline.md"),
    };
    // Contract §7 split-after: fixed margin-right = space.inline.md at index 2
    // when length == 6 (3+3 grouping).
    let split_margin = theme.resolve_space("space.inline.md");

    // ── Distribute the sanitized value across slots (per numbers_only) ──
    let chars = spec.sanitized_chars();

    // ── Root container ──
    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Row;
    root.style.descriptor.layout.spacing.gap = gap;
    root.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;

    // ── Visual slots ──
    for i in 0..spec.length {
        let ch = chars.get(i).copied();
        // Active slot = the next empty slot to fill (caret position).
        let is_active = i == chars.len() && !spec.is_disabled;
        let display_text = match ch {
            Some(_) if spec.mask => "\u{2022}".to_string(),
            Some(c) => c.to_string(),
            None => String::new(),
        };

        let slot_text_color = if ch.is_some() {
            text_primary
        } else {
            text_secondary
        };

        let mut slot = Node::container();
        {
            let s = &mut slot.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(slot_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(slot_size);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            s.descriptor.background = Some(surface);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = if is_active { active_border } else { slot_border };
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            // 3+3 split for 6-digit codes.
            if spec.length == 6 && i == 2 {
                s.descriptor.layout.spacing.margin.right = split_margin;
            }
        }

        // Slot value uses the code font family (contract §7).
        let mut value = Node::text(&display_text);
        {
            let s = &mut value.style;
            s.text_size = Some(font_size);
            s.descriptor.text_color = Some(slot_text_color);
            s.text_weight = Some(600);
            s.font_family = Some(FontFamily::Mono);
            s.text_align = Some(TextAlign::Center);
        }

        root = root.child(slot.child(value));
    }

    // ── Error message (below slots) ──
    if let Some(ref error) = spec.error {
        let mut error_label = Node::text(error.as_str());
        error_label.style.text_size = Some(theme.resolve_space("typography.label.size"));
        error_label.style.descriptor.text_color = Some(danger);

        let mut outer = Node::container();
        {
            let s = &mut outer.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        }
        let mut outer = outer.child(root).child(error_label);

        if spec.is_disabled {
            outer.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
            outer.interaction.disabled = true;
        }
        if let Some(label) = spec.aria_label.as_deref() {
            if !label.is_empty() {
                outer.a11y.label = Some(label.to_string());
            }
        }
        return outer;
    }

    // ── Disabled state ──
    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        root.interaction.disabled = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}
