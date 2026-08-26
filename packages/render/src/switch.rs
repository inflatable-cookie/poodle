//! Switch — on/off toggle with track and thumb.
//!
//! Contract: `docs/contracts/components/switch.md`
//! Ported from: `packages/jetstream/components/src/switch.rs`.

use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodePosition,
    NodeRole, NodeToggled, ShadowLayer, StylePatch,
};
use poodle_specs::{ControlDensity, SwitchSpec};

use crate::color::{hex_color, mix_srgb};
use crate::context::RenderContext;
use crate::presentation::{
    rem_to_px, switch_label_font_rem, switch_thumb_rem, switch_track_h_rem, switch_track_w_rem,
    switch_travel_rem,
};

/// Build a switch node. `on_change` fires with the state moving **to** unless
/// disabled or read-only.
pub fn switch(
    spec: &SwitchSpec,
    ctx: &RenderContext<'_>,
    on_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let is_checked = spec.current_checked();

    let surface = ctx.theme().resolve_color("color.background.surface");
    let border_default = ctx.theme().resolve_color("color.border.default");
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let accent_base = ctx.theme().resolve_color("color.accent.base");

    let gap = match density {
        ControlDensity::Compact => ctx.theme().resolve_space("space.inline.xs"),
        ControlDensity::Default => ctx.theme().resolve_space("space.inline.sm"),
        ControlDensity::Comfortable => ctx.theme().resolve_space("space.inline.md"),
    };
    let label_size = rem_to_px(switch_label_font_rem(effective_size));

    // Contract resolution order per side: hex override → tone token → default.
    let off_track_color: ColorValue = spec
        .left_tone
        .color_token()
        .map(|t| ctx.theme().resolve_color(t))
        .unwrap_or(text_primary);
    let on_track_color: ColorValue = spec
        .right_tone
        .color_token()
        .map(|t| ctx.theme().resolve_color(t))
        .unwrap_or(accent_base);
    let off_tone_color = spec
        .left_tone
        .color_token()
        .map(|token| ctx.theme().resolve_color(token))
        .unwrap_or(text_primary);
    let on_tone_color = spec
        .right_tone
        .color_token()
        .map(|token| ctx.theme().resolve_color(token))
        .unwrap_or(accent_base);

    // Contract §8 mixes.
    // The established GPUI recipe treats explicit hex values as the final
    // track fill; only tone/default colours are mixed with the surface.
    let off_track = spec
        .off_color
        .as_deref()
        .and_then(hex_color)
        .unwrap_or_else(|| mix_srgb(off_track_color, surface, 0.18));
    let on_track = spec
        .on_color
        .as_deref()
        .and_then(hex_color)
        .unwrap_or_else(|| mix_srgb(on_track_color, surface, 0.24));
    let track_fill = if is_checked { on_track } else { off_track };
    let track_border = if is_checked {
        mix_srgb(on_tone_color, border_default, 0.58)
    } else if spec.left_tone != poodle_specs::SwitchTone::Default {
        mix_srgb(off_tone_color, border_default, 0.58)
    } else {
        border_default
    };
    let thumb_color = if is_checked {
        on_tone_color
    } else {
        off_tone_color
    };

    // Contract §8 per-size flat rem literals.
    let track_width = rem_to_px(switch_track_w_rem(effective_size));
    let track_height = rem_to_px(switch_track_h_rem(effective_size));
    let track_padding = rem_to_px(0.125);
    let thumb_size = rem_to_px(switch_thumb_rem(effective_size));
    let thumb_travel = rem_to_px(switch_travel_rem(effective_size));
    let border_width = rem_to_px(0.0625);

    let thumb_offset = if is_checked {
        track_padding + thumb_travel
    } else {
        track_padding
    };

    // ── Thumb — circle with the contract's outset drop ──
    let mut thumb = Node::container();
    thumb.position = NodePosition::Absolute {
        top: Some(track_padding),
        left: Some(thumb_offset),
        right: None,
        bottom: None,
    };
    {
        let s = &mut thumb.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(thumb_size);
        s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
        let r = thumb_size / 2.0;
        s.descriptor.corner_radii.top_left = r;
        s.descriptor.corner_radii.top_right = r;
        s.descriptor.corner_radii.bottom_right = r;
        s.descriptor.corner_radii.bottom_left = r;
        s.descriptor.background = Some(thumb_color);
        s.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: rem_to_px(0.125),
            blur: rem_to_px(0.5),
            spread: 0.0,
            color: ColorValue(0.0, 0.0, 0.0, 0.18),
            inset: false,
        }];
    }

    // ── Track — pill with the contract's inset top highlight ──
    let mut track = Node::container();
    track.position = NodePosition::Relative;
    {
        let s = &mut track.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(track_width);
        s.descriptor.layout.height = LayoutSizing::Fixed(track_height);
        let r = track_height / 2.0;
        s.descriptor.corner_radii.top_left = r;
        s.descriptor.corner_radii.top_right = r;
        s.descriptor.corner_radii.bottom_right = r;
        s.descriptor.corner_radii.bottom_left = r;
        s.descriptor.background = Some(track_fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = track_border;
        s.flex_none = true;
        s.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: rem_to_px(0.0625),
            color: ColorValue(1.0, 1.0, 1.0, 0.08),
            inset: false,
        }];
    }
    let track = track.child(thumb);

    // ── Root: dual labels flank the track, or a single trailing label ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    if !spec.is_disabled {
        root.interaction.focusable = true;
        root.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
            text_color: None,
            opacity: None,
        });
    }

    if spec.is_dual_label() {
        // Both side labels rest at text-muted; the active side re-tints.
        let mut inactive = ctx.theme().resolve_color("color.text.secondary");
        inactive.3 *= 0.85;
        if let Some(ref left) = spec.left_label {
            let c = if is_checked { inactive } else { off_tone_color };
            let mut l = Node::text(left);
            l.style.descriptor.text_color = Some(c);
            l.style.text_size = Some(label_size);
            l.style.text_weight = Some(500);
            root = root.child(l);
        }
        root = root.child(track);
        if let Some(ref right) = spec.right_label {
            let c = if is_checked { on_tone_color } else { inactive };
            let mut l = Node::text(right);
            l.style.descriptor.text_color = Some(c);
            l.style.text_size = Some(label_size);
            l.style.text_weight = Some(500);
            root = root.child(l);
        }
    } else {
        root = root.child(track);
        if let Some(ref label) = spec.label {
            let mut l = Node::text(label);
            l.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.primary"));
            l.style.text_size = Some(label_size);
            l.style.text_weight = Some(500);
            root = root.child(l);
        }
    }

    // Contract §8 cursor/opacity states.
    if spec.is_disabled {
        root.style.descriptor.opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
        root.interaction.disabled = true;
    } else if spec.is_read_only {
        root.style.descriptor.cursor = CursorHint::Default;
    } else {
        root.style.descriptor.cursor = CursorHint::Pointer;
    }

    if !(spec.is_disabled || spec.is_read_only) {
        if let Some(handler) = on_change {
            let next = !spec.current_checked();
            root.interaction.on_activate = Some(Arc::new(move || handler(next)));
        }
    }

    // Accessible name: ariaLabel, else label, else a composition of the two
    // end labels — a switch captioned only "Off"/"On" still has to say what it
    // switches.
    let composed = match (&spec.left_label, &spec.right_label) {
        (Some(left), Some(right)) => Some(format!("{left} / {right}")),
        (Some(only), None) | (None, Some(only)) => Some(only.clone()),
        (None, None) => None,
    };
    let name = spec
        .aria_label
        .as_deref()
        .or(spec.label.as_deref())
        .map(str::to_owned)
        .or(composed);
    if let Some(name) = name {
        root.a11y.label = Some(name);
    }
    root.a11y.role = Some(NodeRole::Switch);
    root.a11y.toggled = Some(if spec.current_checked() {
        NodeToggled::True
    } else {
        NodeToggled::False
    });
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::SwitchSpec;
    use std::sync::{Arc, Mutex};

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn activation_emits_the_next_checked_value() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let seen: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let node = switch(
            &SwitchSpec::new().with_checked(false).with_label("Dark"),
            &ctx,
            Some(Arc::new(move |next| sink.lock().unwrap().push(next))),
        );
        assert_eq!(node.a11y.toggled, Some(NodeToggled::False));
        (node.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), [true]);
    }

    #[test]
    fn readonly_stays_focusable_without_an_activate_handler() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = switch(
            &SwitchSpec::new()
                .with_checked(true)
                .with_read_only(true)
                .with_label("Locked"),
            &ctx,
            Some(Arc::new(|_: bool| panic!("readonly must not emit"))),
        );
        assert!(node.interaction.focusable);
        assert!(node.style.focus.is_some());
        assert!(node.interaction.on_activate.is_none());
        assert_eq!(node.a11y.toggled, Some(NodeToggled::True));
    }

    #[test]
    fn disabled_is_out_of_focus_and_activation() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = switch(
            &SwitchSpec::new()
                .with_checked(false)
                .with_disabled(true)
                .with_label("Off"),
            &ctx,
            Some(Arc::new(|_: bool| panic!("disabled must not emit"))),
        );
        assert!(!node.interaction.focusable);
        assert!(node.interaction.disabled);
        assert!(node.interaction.on_activate.is_none());
        assert!(node.style.focus.is_none());
    }
}
