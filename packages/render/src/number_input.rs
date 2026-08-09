//! NumberInput — numeric field with optional steppers and boxed affixes.
//!
//! Contract: `docs/contracts/components/number-input.md`
//! Ported from: `packages/jetstream/components/src/number_input.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeRole,
};
use poodle_specs::{NumberInputSpec, ValidationState};

use crate::color::with_alpha;
use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size, resolve_supporting_visual_size,
    size_font_rem, size_padding_x_offset_rem,
};

/// Host callbacks: increment / decrement presses. Bounds- or state-disabled
/// steppers never fire.
#[derive(Default)]
pub struct NumberInputHandlers {
    pub on_increment: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_decrement: Option<Arc<dyn Fn() + Send + Sync>>,
}

/// A boxed prefix/suffix affix: bordered box with surface bg + muted text,
/// full control height.
#[expect(clippy::too_many_arguments, reason = "affix rendering keeps resolved token metrics explicit")]
fn affix_box(
    text: &str,
    text_color: ColorValue,
    bg: ColorValue,
    border_color: ColorValue,
    border_width: f32,
    font_size: f32,
    pad_x: f32,
    height: f32,
) -> Node {
    let mut el = Node::container();
    {
        let s = &mut el.style;
        // GPUI's `.h_full()` fills the value row's cross axis; using a fixed
        // height here leaves the border-box one pixel outside that row in the
        // node backend and produces a doubled vertical rule.
        let _ = height;
        s.fill_height = true;
        s.descriptor.background = Some(bg);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border_color;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    let mut label = Node::text(text);
    label.style.descriptor.text_color = Some(text_color);
    label.style.text_size = Some(font_size);
    el.child(label)
}

pub fn number_input(
    spec: &NumberInputSpec,
    theme: &dyn ThemeProvider,
    handlers: NumberInputHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    // The old GPUI component resolves this token through the active theme
    // density (the visual axis), then applies the semantic size offset. The
    // spec density is for callers' standalone contracts and is not the
    // preview theme's density override.
    let pad_x = theme.resolve_space(spec.horizontal_padding_token())
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        effective_size,
    )));
    let border_width = theme.resolve_border_width(spec.border_width_token());

    let border = theme.resolve_color(spec.border_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let fill = theme.resolve_color(spec.fill_token());
    let text_color = theme.resolve_color(spec.text_color_token());
    let stepper_icon_color = theme.resolve_color(spec.stepper_icon_color_token());
    // Boxed-affix chrome (border-default box + surface bg + muted text).
    let affix_text = theme.resolve_color(spec.affix_text_token());
    let affix_bg = theme.resolve_color(spec.affix_fill_token());
    let affix_border = theme.resolve_color(spec.affix_border_token());
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());

    // The field border recolors per validation state.
    let effective_border = match spec.validation_state {
        ValidationState::Invalid => theme.resolve_color("color.status.danger"),
        ValidationState::Valid => theme.resolve_color("color.status.success"),
        ValidationState::Pending => theme.resolve_color("color.accent.base"),
        ValidationState::None => border,
    };

    let value = spec.clamped_value();
    let value_text = match spec.precision {
        Some(precision) => format!("{:.*}", precision as usize, value),
        None => format!("{value}"),
    };

    // Bounds checks for stepper button disabled state.
    let at_min = !spec.min.is_infinite() && spec.value <= spec.min;
    let at_max = !spec.max.is_infinite() && spec.value >= spec.max;

    // ── Root container ─────────────────────────────────────────────────────
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = effective_border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        // Old GPUI wrapper declares `.w_full()`; channel fields rely on that
        // declaration when their parent distributes the three inputs.
        s.fill_width = true;
    }

    let stepper_bg = theme.resolve_color(spec.stepper_fill_token());
    let stepper_bg = with_alpha(stepper_bg, stepper_bg.3 * 0.88);
    let stepper = |icon: &str,
                   label: &str,
                   id: &str,
                   blocked: bool,
                   handler: Option<Arc<dyn Fn() + Send + Sync>>|
     -> Node {
        let mut btn = Node::button("");
        btn.a11y.label = Some(label.to_string());
        btn.id = Some(id.to_string());
        {
            let s = &mut btn.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.flex_grow = Some(1.0);
            s.descriptor.background = Some(stepper_bg);
            let inner_radius = (radius - rem_to_px(0.125)).max(0.0);
            s.descriptor.corner_radii.top_left = inner_radius;
            s.descriptor.corner_radii.top_right = inner_radius;
            s.descriptor.corner_radii.bottom_right = inner_radius;
            s.descriptor.corner_radii.bottom_left = inner_radius;
            let pad = &mut s.descriptor.layout.spacing.padding;
            s.descriptor.cursor = CursorHint::Pointer;
            pad.top = 0.0;
            pad.bottom = 0.0;
        }
        btn.interaction.focusable = true;
        let mut glyph = Node::icon(icon, icon_size);
        glyph.style.descriptor.text_color = Some(stepper_icon_color);
        let mut btn = btn.child(glyph);
        if blocked {
            btn.style.descriptor.opacity = disabled_opacity;
            btn.interaction.disabled = true;
        } else if let Some(handler) = handler {
            btn.interaction.on_activate = Some(Arc::new(move || handler()));
        }
        btn
    };

    // ── Value row: the old GPUI tier's left-hand field area ────────────────
    let mut value_row = Node::container();
    {
        let s = &mut value_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        // Old GPUI/Svelte contract: affix↔value gap is a fixed 0.5rem,
        // independent of the active density's inline token ladder.
        s.descriptor.layout.spacing.gap = rem_to_px(0.5);
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
        s.min_width = Some(0.0);
    }

    // Prefix affix (boxed: border + surface bg, inside left edge).
    if let Some(prefix) = &spec.prefix {
        value_row = value_row.child(affix_box(
            prefix,
            affix_text,
            affix_bg,
            affix_border,
            border_width,
            font_size,
            pad_x,
            height,
        ));
    }

    // Value display.
    let mut value = Node::text(&value_text);
    {
        let s = &mut value.style;
        s.descriptor.text_color = Some(text_color);
        s.text_size = Some(font_size);
        s.line_height = Some(1.4);
        // The old GPUI tier uses `.flex_1()` for the value slot: grow and
        // shrink from a zero basis, rather than intrinsic text width. That
        // distinction only shows up when the field is one of several
        // fractional channel inputs (ColorPicker's RGB/HSL rows).
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
        s.min_width = Some(0.0);
    }
    value_row = value_row.child(value);

    // Suffix affix (boxed, inside right edge).
    if let Some(suffix) = &spec.suffix {
        value_row = value_row.child(affix_box(
            suffix,
            affix_text,
            affix_bg,
            affix_border,
            border_width,
            font_size,
            pad_x,
            height,
        ));
    }

    el = el.child(value_row);

    // ── Vertical steppers (only when enabled) ─────────────────────────────
    if spec.show_steppers {
        let stepper_width = rem_to_px(1.25);
        let mut steppers = Node::container();
        {
            let s = &mut steppers.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.width = LayoutSizing::Fixed(stepper_width);
            s.fill_height = true;
            s.descriptor.layout.spacing.padding.top = 1.0;
            s.descriptor.layout.spacing.padding.right = 1.0;
            s.descriptor.layout.spacing.padding.bottom = 1.0;
            s.descriptor.layout.spacing.padding.left = 1.0;
        }
        steppers = steppers
            .child(stepper(
                "plus",
                "Increment",
                "poodle-number-input-inc",
                at_max || spec.is_disabled || spec.is_read_only,
                handlers.on_increment.clone(),
            ))
            .child(stepper(
                "minus",
                "Decrement",
                "poodle-number-input-dec",
                at_min || spec.is_disabled || spec.is_read_only,
                handlers.on_decrement.clone(),
            ));
        el = el.child(steppers);
    }

    if spec.is_disabled {
        el.style.descriptor.opacity = disabled_opacity;
        el.interaction.disabled = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el.a11y.role = Some(NodeRole::SpinButton);
    el
}
