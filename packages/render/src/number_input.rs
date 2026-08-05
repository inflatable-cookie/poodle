//! NumberInput — numeric field with optional steppers and boxed affixes.
//!
//! Contract: `docs/contracts/components/number-input.md`
//! Ported from: `packages/jetstream/components/src/number_input.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeRole, TextAlign,
};
use poodle_specs::{NumberInputSpec, ValidationState};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
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
#[allow(clippy::too_many_arguments)]
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
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
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
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        effective_size,
    )));
    let inline_sz = rem_to_px(size_font_rem(effective_size));
    // Inner stepper gap from the smallest inline-space token.
    let btn_gap = theme.resolve_space(spec.stepper_gap_token());
    let border_width = theme.resolve_space(spec.border_width_token());

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

    let value_text = spec.formatted_value();

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
    }

    let stepper = |icon: &str, label: &str, id: &str, pad_l: f32, pad_r: f32, blocked: bool,
                   handler: Option<Arc<dyn Fn() + Send + Sync>>|
     -> Node {
        let mut btn = Node::button("");
        btn.a11y.label = Some(label.to_string());
        btn.id = Some(id.to_string());
        {
            let s = &mut btn.style;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_l;
            pad.right = pad_r;
            s.descriptor.cursor = CursorHint::Pointer;
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

    // ── Decrement button (only when steppers enabled) ──────────────────────
    if spec.show_steppers {
        el = el.child(stepper(
            "minus",
            "Decrement",
            "poodle-number-input-dec",
            pad_x,
            btn_gap,
            at_min || spec.is_disabled || spec.is_read_only,
            handlers.on_decrement.clone(),
        ));
    }

    // Prefix affix (boxed: border + surface bg, inside left edge).
    if let Some(prefix) = &spec.prefix {
        el = el.child(affix_box(
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
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.text_align = Some(TextAlign::Center);
    }
    el = el.child(value);

    // Validation state icon (trailing, before suffix/increment).
    if let ValidationState::Invalid = spec.validation_state {
        let mut alert = Node::icon("alert-circle", inline_sz);
        alert.style.descriptor.text_color = Some(theme.resolve_color("color.status.danger"));
        el = el.child(alert);
    }

    // Suffix affix (boxed, inside right edge).
    if let Some(suffix) = &spec.suffix {
        el = el.child(affix_box(
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

    // ── Increment button (only when steppers enabled) ──────────────────────
    if spec.show_steppers {
        el = el.child(stepper(
            "plus",
            "Increment",
            "poodle-number-input-inc",
            btn_gap,
            pad_x,
            at_max || spec.is_disabled || spec.is_read_only,
            handlers.on_increment.clone(),
        ));
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
