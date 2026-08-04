//! Switch — on/off toggle with track and thumb.
//!
//! Contract: `docs/contracts/components/switch.md`
//! Ported from: `packages/jetstream/components/src/switch.rs`.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole,
    NodeToggled, ShadowLayer,
};
use poodle_specs::{ControlDensity, SwitchSpec};

use crate::color::{hex_color, mix_srgb};
use crate::presentation::{
    rem_to_px, resolve_semantic_size, switch_label_font_rem, switch_thumb_rem, switch_track_h_rem,
    switch_track_w_rem, switch_travel_rem,
};

/// Build a switch node. `on_change` fires with the state moving **to** unless
/// disabled or read-only.
pub fn switch(
    spec: &SwitchSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let is_checked = spec.current_checked();

    let surface = theme.resolve_color("color.background.surface");
    let border_default = theme.resolve_color("color.border.default");
    let text_primary = theme.resolve_color("color.text.primary");
    let accent_base = theme.resolve_color("color.accent.base");

    let gap = match spec.density {
        ControlDensity::Compact => theme.resolve_space("space.inline.xs"),
        ControlDensity::Default => theme.resolve_space("space.inline.sm"),
        ControlDensity::Comfortable => theme.resolve_space("space.inline.md"),
    };
    let label_size = rem_to_px(switch_label_font_rem(effective_size));

    // Contract resolution order per side: hex override → tone token → default.
    let off_color: ColorValue = spec
        .off_color
        .as_deref()
        .and_then(hex_color)
        .or_else(|| spec.left_tone.color_token().map(|t| theme.resolve_color(t)))
        .unwrap_or(text_primary);
    let on_color: ColorValue = spec
        .on_color
        .as_deref()
        .and_then(hex_color)
        .or_else(|| spec.right_tone.color_token().map(|t| theme.resolve_color(t)))
        .unwrap_or(accent_base);

    // Contract §8 mixes.
    let off_track = mix_srgb(off_color, surface, 0.18);
    let on_track = mix_srgb(on_color, surface, 0.24);
    let track_fill = if is_checked { on_track } else { off_track };
    let on_border = mix_srgb(on_color, border_default, 0.58);
    let track_border = if is_checked { on_border } else { border_default };
    let thumb_color = if is_checked { on_color } else { off_color };

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
    {
        let s = &mut track.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(track_width);
        s.descriptor.layout.height = LayoutSizing::Fixed(track_height);
        s.descriptor.corner_radii.top_left = 999.0;
        s.descriptor.corner_radii.top_right = 999.0;
        s.descriptor.corner_radii.bottom_right = 999.0;
        s.descriptor.corner_radii.bottom_left = 999.0;
        s.descriptor.background = Some(track_fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = track_border;
        // Explicit Row: the old tier never set a direction here and got
        // taffy's Row default. The vocabulary's default is Column, so a port
        // that stays silent flips the axis items_center works on.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.padding.left = thumb_offset;
        s.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: ColorValue(1.0, 1.0, 1.0, 0.08),
            inset: true,
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
    root.interaction.focusable = true;

    if spec.is_dual_label() {
        // Both side labels rest at text-muted; the active side re-tints.
        let muted = theme.resolve_color("color.text.muted");
        if let Some(ref left) = spec.left_label {
            let c = if is_checked { muted } else { off_color };
            let mut l = Node::text(left);
            l.style.descriptor.text_color = Some(c);
            l.style.text_size = Some(label_size);
            root = root.child(l);
        }
        root = root.child(track);
        if let Some(ref right) = spec.right_label {
            let c = if is_checked { on_color } else { muted };
            let mut l = Node::text(right);
            l.style.descriptor.text_color = Some(c);
            l.style.text_size = Some(label_size);
            root = root.child(l);
        }
    } else {
        root = root.child(track);
        if let Some(ref label) = spec.label {
            let mut l = Node::text(label);
            l.style.descriptor.text_color = Some(theme.resolve_color("color.text.primary"));
            l.style.text_size = Some(label_size);
            root = root.child(l);
        }
    }

    // Contract §8 cursor/opacity states.
    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
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
    root.a11y.toggled = Some(match spec.checked {
        Some(true) => NodeToggled::True,
        Some(false) => NodeToggled::False,
        None => NodeToggled::Mixed,
    });
    root
}
