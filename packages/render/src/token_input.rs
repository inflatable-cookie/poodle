//! TokenInput — committed token pills + a live draft input in a wrap row.
//!
//! Contract: `docs/contracts/components/token-input.md`
//! Ported from: `packages/jetstream/components/src/token_input.rs`.
//!
//! `on_remove` fires with the removed token's value. Entry/commit wiring is
//! host-owned; the draft is a real composed text_input.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::{PillAppearance, PillSpec, PillTone, TextInputSpec, TokenInputSpec};

use crate::color::{mix_srgb, TRANSPARENT};
use crate::pill::pill_with_remove;
use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, token_input_gap_rem,
    token_input_pad_x_offset_rem, token_input_pad_y_offset_rem,
};
use crate::text_input::text_input;

/// Token pills track the field size so they remain visually secondary.
fn pill_size(size: poodle_specs::ControlSize) -> poodle_specs::PillSize {
    use poodle_specs::{ControlSize, PillSize};
    match size {
        ControlSize::Xs => PillSize::Xs,
        ControlSize::Sm => PillSize::Sm,
        ControlSize::Md => PillSize::Md,
        ControlSize::Lg => PillSize::Lg,
        ControlSize::Xl => PillSize::Xl,
    }
}

pub fn token_input(
    spec: &TokenInputSpec,
    theme: &dyn ThemeProvider,
    on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let surface = theme.resolve_color("color.background.surface");
    let border = theme.resolve_color("color.border.subtle");
    // Field family matches TextInput's interactive-subtle treatment: a
    // surface-mix fill behind a subtle border.
    let fill = mix_srgb(surface, TRANSPARENT, 0.96);
    let radius = theme.resolve_radius("radius.control");

    // Size-driven font + density-driven wrap gap (contract §8).
    let gap = rem_to_px(token_input_gap_rem(spec.density));

    // Padding-block from control.y + per-size offset; padding-inline from
    // control.x (density) + per-size offset.
    let pad_y = (theme.resolve_space("space.control.y")
        + rem_to_px(token_input_pad_y_offset_rem(effective_size)))
    .max(0.0);
    let pad_x = (rem_to_px(control_space_x_rem(spec.density))
        + rem_to_px(token_input_pad_x_offset_rem(effective_size)))
    .max(0.0);

    let can_edit = spec.can_edit();

    // Wrapping token row: committed pills (+ remove ×) then the draft.
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
        s.descriptor.layout.spacing.gap = gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
    }

    for token in &spec.values {
        let remove = if can_edit {
            on_remove.as_ref().map(|handler| {
                let handler = Arc::clone(handler);
                let value = token.clone();
                Arc::new(move || handler(&value)) as Arc<dyn Fn() + Send + Sync>
            })
        } else {
            None
        };
        let token_pill = PillSpec::new()
            .with_label(token.clone())
            .with_tone(PillTone::Neutral)
            .with_appearance(PillAppearance::Subtle)
            .with_size(pill_size(effective_size))
            .with_removable(can_edit);
        row = row.child(pill_with_remove(&token_pill, theme, remove));
    }

    // Live draft control — a real composed text input. Inherits size,
    // density, placeholder, maxLength, disabled/read-only from the spec.
    let mut draft = TextInputSpec::new()
        .with_size(effective_size)
        .with_size_role(spec.size_role)
        .with_density(spec.density)
        .with_disabled(spec.disabled)
        .with_read_only(spec.read_only);
    if !spec.id.is_empty() {
        draft = draft.with_id(spec.id.clone());
    }
    // Placeholder only shows when there are no committed tokens.
    if spec.values.is_empty() {
        if let Some(placeholder) = &spec.placeholder {
            draft = draft.with_placeholder(placeholder.clone());
        }
    }
    if let Some(max) = spec.max_length {
        draft = draft.with_max_length(max);
    }
    if let Some(aria) = &spec.aria_label {
        draft = draft.with_aria_label(aria.clone());
    }
    // The draft sits inline in the wrap row and grows to fill trailing space.
    let mut draft_slot = Node::container();
    {
        let s = &mut draft_slot.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.flex_basis = Some(rem_to_px(8.0));
        s.min_width = Some(rem_to_px(6.0));
    }
    row = row.child(draft_slot.child(text_input(&draft, theme, None)));

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.fill_width = true;
        s.min_width = Some(0.0);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.background = Some(fill);
    }
    let mut el = el.child(row);

    if spec.disabled {
        el.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
    }
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
