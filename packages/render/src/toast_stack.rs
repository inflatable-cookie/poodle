//! ToastStack — toast notification stack.
//!
//! Contract: `docs/contracts/components/toast-stack.md`
//! Ported from: `packages/jetstream/components/src/toast_stack.rs`.
//!
//! Each toast: leading tone accent bar, title + optional message + optional
//! action chip, dismiss ×, tone-tinted gradient fill over the tint,
//! elevation-overlay shadow, and a one-shot enter animation keyed by the
//! toast's stable id (fade + rise; completion persists across rebuilds).

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    AnimEasing, AnimKeyframe, AnimLoop, AnimProperty, CrossAxisAlignment, CursorHint,
    LayoutDirection, LayoutSizing, Node, NodeAnimation, NodePosition, NodeRole, TextAlign,
};
use poodle_specs::{ControlDensity, ControlSize, ToastPosition, ToastStackSpec};

use crate::color::{mix_srgb, WHITE};
use crate::presentation::{rem_to_px, resolve_semantic_size};

/// Host callbacks: dismiss and action, each carrying the toast's id.
#[derive(Default)]
pub struct ToastStackHandlers {
    pub on_dismiss: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

/// Per-size title font-size in rem (contract §8 size table).
fn title_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.71875,
        ControlSize::Sm | ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Per-size message font-size in rem (contract §8 size table).
fn message_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// Per-size dismiss square dimension in rem (contract §8 size table).
fn dismiss_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.0,
        ControlSize::Sm => 1.125,
        ControlSize::Md => 1.25,
        ControlSize::Lg => 1.5,
        ControlSize::Xl => 1.75,
    }
}

/// Density toast-padding multiplier (contract §8 density table).
fn density_pad_scale(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.75,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.25,
    }
}

fn all_corners(node: &mut Node, r: f32) {
    let c = &mut node.style.descriptor.corner_radii;
    c.top_left = r;
    c.top_right = r;
    c.bottom_right = r;
    c.bottom_left = r;
}

/// One-shot enter animation: fade in + rise 0.5rem, ease-out.
fn toast_enter(key: String) -> NodeAnimation {
    NodeAnimation {
        key,
        keyframes: vec![
            AnimKeyframe {
                at: 0.0,
                values: vec![
                    (AnimProperty::Opacity, 0.0),
                    (AnimProperty::TranslateY, rem_to_px(0.5)),
                ],
            },
            AnimKeyframe {
                at: 1.0,
                values: vec![
                    (AnimProperty::Opacity, 1.0),
                    (AnimProperty::TranslateY, 0.0),
                ],
            },
        ],
        duration_secs: 0.18,
        easing: AnimEasing::EaseOut,
        loop_mode: AnimLoop::Once,
    }
}

pub fn toast_stack(
    spec: &ToastStackSpec,
    theme: &dyn ThemeProvider,
    handlers: ToastStackHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_px = rem_to_px(title_font_rem(effective_size));
    let message_px = rem_to_px(message_font_rem(effective_size));
    let dismiss_px = rem_to_px(dismiss_size_rem(effective_size));

    // Contract §8 toast padding = space-panel-x scaled by density.
    let base_pad = theme.resolve_space(spec.padding_token());
    let pad = base_pad * density_pad_scale(spec.density);
    // Contract §7 stack gap + toast internal gap = space-stack-sm token.
    let stack_gap = theme.resolve_space(spec.gap_token());
    let item_gap = theme.resolve_space(spec.gap_token());

    let elevated = theme.resolve_color(spec.fill_token());
    let border_default = theme.resolve_color(spec.border_token());
    let radius_base = theme.resolve_radius(spec.radius_token());
    // Contract §8: border-radius = calc(radius-surface - 0.125rem).
    let radius = (radius_base - rem_to_px(0.125)).max(0.0);
    let title_color = theme.resolve_color(spec.title_color_token());
    let message_color = theme.resolve_color(spec.message_color_token());
    let dismiss_color = theme.resolve_color(spec.dismiss_color_token());

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = stack_gap;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(22.5));
    }
    // Corner-mounted overlay: keep the stack out of flow and anchor it to
    // the nearest relative host, matching the old GPUI wrapper.
    el.position = match spec.position {
        ToastPosition::TopRight => NodePosition::Absolute {
            top: Some(pad),
            right: Some(pad),
            left: None,
            bottom: None,
        },
        ToastPosition::TopLeft => NodePosition::Absolute {
            top: Some(pad),
            left: Some(pad),
            right: None,
            bottom: None,
        },
        ToastPosition::BottomRight => NodePosition::Absolute {
            bottom: Some(pad),
            right: Some(pad),
            top: None,
            left: None,
        },
        ToastPosition::BottomLeft => NodePosition::Absolute {
            bottom: Some(pad),
            left: Some(pad),
            top: None,
            right: None,
        },
    };

    for toast in &spec.toasts {
        let tone_color = theme.resolve_color(spec.tone_color(&toast.tone));

        // Contract §8 tone treatments:
        //   accent bar = color-mix(tone 94%, white)
        //   border     = color-mix(tone 34%, border-default)
        //   background = color-mix(tone 12%, elevated) tint
        let accent_bar_color = mix_srgb(tone_color, WHITE, 0.94);
        let toast_border = mix_srgb(tone_color, border_default, 0.34);
        let bg_tinted = mix_srgb(tone_color, elevated, 0.12);

        // Leading tone accent bar — contract §8: 0.1875rem (3px), full height.
        let mut accent_bar = Node::container();
        {
            let s = &mut accent_bar.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(0.1875));
            s.self_stretch = true;
            s.descriptor.background = Some(accent_bar_color);
        }

        // Title + optional message column.
        let mut content = Node::container();
        {
            let s = &mut content.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = rem_to_px(0.25);
            s.descriptor.layout.width = LayoutSizing::Grow;
        }
        let mut title = Node::text(toast.title.as_str());
        title.style.descriptor.text_color = Some(title_color);
        title.style.text_size = Some(title_px);
        title.style.text_weight = Some(600);
        let mut content = content.child(title);

        if let Some(message) = &toast.message {
            let mut msg = Node::text(message.as_str());
            msg.style.descriptor.text_color = Some(message_color);
            msg.style.text_size = Some(message_px);
            content = content.child(msg);
        }
        // Optional action affordance (secondary-button-styled chip).
        if let Some(action) = &toast.action_label {
            let mut chip = Node::text(action.as_str());
            {
                let s = &mut chip.style;
                s.descriptor.text_color = Some(title_color);
                s.text_size = Some(message_px);
                s.text_weight = Some(600);
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = toast_border;
                let padc = &mut s.descriptor.layout.spacing.padding;
                padc.left = rem_to_px(0.5);
                padc.right = rem_to_px(0.5);
                padc.top = rem_to_px(0.25);
                padc.bottom = rem_to_px(0.25);
            }
            all_corners(&mut chip, radius);

            if let Some(handler) = &handlers.on_action {
                let handler = Arc::clone(handler);
                let id = toast.id.clone();
                chip.style.descriptor.cursor = CursorHint::Pointer;
                chip.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }

            content = content.child(chip);
        }

        // Dismiss affordance — × glyph in a sized square.
        let mut dismiss = Node::text("\u{00d7}");
        {
            let s = &mut dismiss.style;
            s.descriptor.text_color = Some(dismiss_color);
            s.text_size = Some(dismiss_px);
            s.descriptor.layout.width = LayoutSizing::Fixed(dismiss_px);
            s.descriptor.layout.height = LayoutSizing::Fixed(dismiss_px);
            s.text_align = Some(TextAlign::Center);
        }
        if let Some(handler) = &handlers.on_dismiss {
            let handler = Arc::clone(handler);
            let id = toast.id.clone();
            dismiss.style.descriptor.cursor = CursorHint::Pointer;
            dismiss.interaction.on_activate = Some(Arc::new(move || handler(&id)));
        }

        // Toast box: tinted fill + fade gradient, tone border,
        // elevation-overlay shadow, clipped, listitem role, enter animation.
        let mut toast_el = Node::container();
        toast_el.a11y.role = Some(NodeRole::ListItem);
        toast_el.position = NodePosition::Relative;
        toast_el.id = Some(format!("poodle-toast-{}", toast.id));
        {
            let s = &mut toast_el.style;
            s.descriptor.background = Some(bg_tinted);
            // Contract §8: linear gradient (90deg) of tone tint fading into
            // elevated.
            s.gradient = Some((
                90.0,
                vec![
                    (mix_srgb(tone_color, elevated, 0.12), 0.0),
                    (elevated, 0.18),
                ],
            ));
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = toast_border;
            s.descriptor.layout.overflow_x = poodle_node::LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = poodle_node::LayoutOverflow::Hidden;
            // Token-accurate elevation.overlay (single layer, spread 0).
            s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
            let padc = &mut s.descriptor.layout.spacing.padding;
            padc.left = pad;
            padc.right = pad + rem_to_px(1.5);
            padc.top = pad;
            padc.bottom = pad;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = item_gap;
            // Enter animation: fade + rise, one-shot, keyed by the stable id.
            s.animation = Some(toast_enter(format!("poodle-toast-{}", toast.id)));
        }
        all_corners(&mut toast_el, radius);

        el = el.child(toast_el.child(accent_bar).child(content).child(dismiss));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    // Contract: the stack is a list of toasts.
    el.a11y.role = Some(NodeRole::List);
    el
}
