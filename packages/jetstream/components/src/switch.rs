//! JsSwitch — toggle switch with track and thumb, backed by SwitchSpec.
//!
//! Contract: `docs/contracts/components/switch.md`
//! Reference: `packages/svelte/components/src/Switch.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, BoxShadow, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlDensity, SwitchSpec};

use crate::presentation::{
    rem_to_px, resolve_semantic_size, switch_label_font_rem, switch_thumb_rem, switch_track_h_rem,
    switch_track_w_rem, switch_travel_rem,
};
use crate::theme_ext::{hex_to_rgb255, resolve_color, resolve_opacity, resolve_px, rgb255_to_vec4};

/// Build a switch element from a SwitchSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .switch] — <label>
///   ├── [Control] — <input type="checkbox" role="switch"> (visually hidden)
///   ├── [Track]   — <span>, 2.125rem × 1.25rem, border-radius 999px
///   │   └── [Thumb] — <span>, 0.875rem diameter
///   └── [Label]   — <span> (optional)
/// ```
///
/// Contract dimensions (§8 Size adjustments — per-size flat rem literals):
/// - Track (md): 2.25rem × 1.375rem; thumb 1.125rem; travel 0.875rem
/// - Track padding: 0.125rem (2px) at every size
/// - Track border: 0.0625rem (1px) solid
/// - Gap (density): compact space.inline.xs, default space.inline.sm,
///   comfortable space.inline.md
///
/// Contract token formulas (section 8):
/// - off-color  = text-primary (or leftTone token)
/// - on-color   = accent-base (or rightTone token)
/// - off-track  = color-mix(off-color 18%, background-surface)
/// - on-track   = color-mix(on-color  24%, background-surface)
/// - off-thumb  = off-color
/// - on-thumb   = on-color
/// - off-border = border-default
/// - on-border  = color-mix(on-thumb 58%, border-default)
pub fn js_switch(spec: &SwitchSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let is_checked = spec.current_checked();

    // ── Base color resolution ──
    let surface = resolve_color(theme, "color.background.surface");
    let border_default = resolve_color(theme, "color.border.default");
    let text_primary = resolve_color(theme, "color.text.primary");
    let accent_base = resolve_color(theme, "color.accent.base");
    let text_primary_c: Color = text_primary.into();
    let accent_base_c: Color = accent_base.into();
    let surface_c: Color = surface.into();
    let border_default_c: Color = border_default.into();

    // Gap (density): compact = space.inline.xs, default = space.inline.sm,
    // comfortable = space.inline.md — same tokens GPUI resolves. Replaces the
    // old `control_space_x_rem` heuristic which produced 0.5/0.75/1.0rem.
    let gap = match spec.density {
        ControlDensity::Compact => resolve_px(theme, "space.inline.xs"),
        ControlDensity::Default => resolve_px(theme, "space.inline.sm"),
        ControlDensity::Comfortable => resolve_px(theme, "space.inline.md"),
    };
    let label_size = rem_to_px(switch_label_font_rem(effective_size));

    // Resolve a custom hex override (#rgb / #rrggbb / #rrggbbaa) into a Color.
    let hex_color = |hex: &str| -> Option<Color> {
        hex_to_rgb255(hex).map(|rgb| rgb255_to_vec4(rgb, rgb.a).into())
    };

    // Contract resolution order for each side:
    //   1. explicit hex override (on_color / off_color)
    //   2. tone token (right_tone / left_tone)
    //   3. native default (accent-base on / text-primary off)
    let off_color_c: Color = spec
        .off_color
        .as_deref()
        .and_then(hex_color)
        .or_else(|| spec.left_tone.color_token().map(|t| resolve_color(theme, t).into()))
        .unwrap_or(text_primary_c);
    let on_color_c: Color = spec
        .on_color
        .as_deref()
        .and_then(hex_color)
        .or_else(|| spec.right_tone.color_token().map(|t| resolve_color(theme, t).into()))
        .unwrap_or(accent_base_c);

    // Contract: track fill = color-mix(in srgb, tone N%, surface)
    // off-track: mix(off-color 18%, surface)  → 18% off-color + 82% surface
    // on-track:  mix(on-color  24%, surface)  → 24% on-color  + 76% surface
    // mix(other, f): self*f + other*(1-f), so self.mix_srgb(surface, 0.18) = 18% self + 82% surface
    let off_track_c = off_color_c.mix_srgb(surface_c, 0.18);
    let on_track_c  = on_color_c.mix_srgb(surface_c, 0.24);
    let track_fill: Color = if is_checked { on_track_c } else { off_track_c };

    // Contract: border = border-default (off), color-mix(on-thumb 58%, border-default) (on)
    let on_border_c = on_color_c.mix_srgb(border_default_c, 0.58); // 58% on-thumb, 42% border
    let track_border: Color = if is_checked { on_border_c } else { border_default_c };

    // Contract: thumb = off-color (off) or on-color (on)
    let thumb_color: Color = if is_checked { on_color_c } else { off_color_c };

    // Contract §8 Size adjustments — per-size flat rem literals (matches Svelte).
    let track_width:   f32 = rem_to_px(switch_track_w_rem(effective_size));
    let track_height:  f32 = rem_to_px(switch_track_h_rem(effective_size));
    let track_padding: f32 = rem_to_px(0.125); // 0.125rem at every size
    let thumb_size:    f32 = rem_to_px(switch_thumb_rem(effective_size));
    let thumb_travel:  f32 = rem_to_px(switch_travel_rem(effective_size));
    let border_width:  f32 = rem_to_px(0.0625);

    // Contract §8 track :focus-visible outline = accent.focusRing, offset 0.125rem.
    // JsEl has no :focus-visible hook and js_switch takes no focus-state input, so
    // the ring is painted by the runtime/preview focus layer (same posture as
    // tree/nav_card/number_input). Resolved here so the token is referenced.
    let _focus_ring: Color = resolve_color(theme, "color.accent.focusRing").into();

    // Contract: thumb offset = track_padding (off) or track_padding + thumb_travel (on)
    let thumb_offset = if is_checked {
        track_padding + thumb_travel
    } else {
        track_padding
    };

    // ── Thumb ──
    // Contract §8 Thumb box-shadow: `0 0.125rem 0.5rem black@18%` outset drop.
    let thumb = ui_element::div()
        .w(thumb_size)
        .h(thumb_size)
        .rounded(thumb_size / 2.0) // circle — 999px pill equivalent
        .bg(thumb_color)
        .shadow_layers(vec![BoxShadow {
            offset_x: 0.0,
            offset_y: rem_to_px(0.125),
            blur: rem_to_px(0.5),
            spread: 0.0,
            color: glam::Vec4::new(0.0, 0.0, 0.0, 0.18),
            inset: false,
        }]);

    // ── Track ──
    // Contract §8 Track box-shadow: `inset 0 0.0625rem 0 white@8%` top highlight.
    let track = ui_element::div()
        .w(track_width)
        .h(track_height)
        .rounded(999.0) // pill
        .bg(track_fill)
        .border(border_width).border_color(track_border)
        .items_center()
        .pl(thumb_offset)
        .shadow_layers(vec![BoxShadow {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: glam::Vec4::new(1.0, 1.0, 1.0, 0.08),
            inset: true,
        }])
        .child(thumb);

    // ── Root: dual labels flank the track, or a single trailing label ──
    let mut root = ui_element::div()
        .flex_row()
        .gap(gap) // from SPACE_INLINE_SM token
        .items_center()
        .focusable();

    if spec.is_dual_label() {
        // Contract §8 "Label color rules": both side labels rest at text-muted;
        // the active side (left when off, right when on) re-tints to the off/on
        // color (thumb_color == off/on color).
        let muted: Color = resolve_color(theme, "color.text.muted").into();
        if let Some(ref left) = spec.left_label {
            let c: Color = if is_checked { muted } else { off_color_c };
            root = root.child(ui_element::label(left).text_color(c).text_size(label_size));
        }
        root = root.child(track);
        if let Some(ref right) = spec.right_label {
            let c: Color = if is_checked { on_color_c } else { muted };
            root = root.child(ui_element::label(right).text_color(c).text_size(label_size));
        }
    } else {
        root = root.child(track);
        if let Some(ref label) = spec.label {
            let label_color = resolve_color(theme, "color.text.primary");
            root = root.child(
                ui_element::label(label)
                    .text_color(label_color)
                    .text_size(label_size), // from TYPOGRAPHY_LABEL_SIZE token
            );
        }
    }

    // Contract §8 cursor/opacity states.
    // - disabled: opacity = state.opacity.disabled, cursor not-allowed (JsEl has
    //   no not-allowed cursor; `.disabled(true)` flags it for the runtime).
    // - readOnly: default cursor, full opacity, reverts on toggle (preview-loop).
    // - interactive: pointer cursor.
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    } else if spec.is_read_only {
        root = root.cursor_default();
    } else {
        root = root.cursor_pointer();
    }

    // The visible label is the accessible name unless something
    // overrides it — the caption sits beside the control, not inside,
    // so name-from-content cannot reach it.
    // Contract: the accessible name comes from `ariaLabel`, else `label`, else
    // a composition of the two end labels — a switch captioned only
    // "Off"/"On" still has to say what it switches.
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
    crate::aria::with_aria_label(root, name.as_deref())
        .aria_role(jetstream_ui::accesskit::Role::Switch).aria_checked(crate::aria::toggled(spec.checked))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::ControlSize;

    /// The track is root's first child (non-dual) / second node (dual) — it's the
    /// only Panel with a non-transparent fill that also has a child Panel (thumb).
    /// For these tests the single-label / no-label switch puts the track at node[1].
    fn md_track_width() -> f32 {
        rem_to_px(switch_track_w_rem(ControlSize::Md))
    }

    #[test]
    fn md_track_geometry_matches_svelte() {
        // Contract §8: md track 2.25rem × 1.375rem = 36 × 22px (not the old 34×20).
        let el = js_switch(&SwitchSpec::new(), &theme());
        let track = &el.children[0];
        assert_eq!(track.layout.size.width, taffy::Dimension::length(36.0));
        assert_eq!(track.layout.size.height, taffy::Dimension::length(22.0));
        assert_eq!(md_track_width(), 36.0);
    }

    #[test]
    fn per_size_track_scales() {
        // xs track 1.75rem = 28px, xl 3rem = 48px — sizes must differ now.
        let xs = js_switch(&SwitchSpec::new().with_size(ControlSize::Xs), &theme());
        let xl = js_switch(&SwitchSpec::new().with_size(ControlSize::Xl), &theme());
        assert_eq!(xs.children[0].layout.size.width, taffy::Dimension::length(28.0));
        assert_eq!(xl.children[0].layout.size.width, taffy::Dimension::length(48.0));
    }

    #[test]
    fn thumb_shifts_right_when_checked() {
        // Thumb left edge = padding (off) vs padding + travel (on). Probe the
        // laid-out x of the thumb (deepest Panel under the track).
        let off = probe(&js_switch(&SwitchSpec::new(), &theme()), 220.0, 60.0);
        let on = probe(&js_switch(&SwitchSpec::new().with_checked(true), &theme()), 220.0, 60.0);
        // Deepest node = thumb in both trees.
        let thumb_off_x = off.nodes.iter().max_by_key(|n| n.depth).unwrap().x;
        let thumb_on_x = on.nodes.iter().max_by_key(|n| n.depth).unwrap().x;
        assert!(
            thumb_on_x > thumb_off_x,
            "thumb did not slide right: off={thumb_off_x}, on={thumb_on_x}",
        );
    }

    #[test]
    fn thumb_changes_color_when_checked() {
        let el_on = js_switch(&SwitchSpec::new().with_checked(true), &theme());
        let el_off = js_switch(&SwitchSpec::new(), &theme());
        let thumb_on = &el_on.children[0].children[0];
        let thumb_off = &el_off.children[0].children[0];
        assert_ne!(thumb_on.style.background, thumb_off.style.background);
    }

    #[test]
    fn custom_on_color_tints_the_track_and_thumb() {
        // on_color hex override (#ff0000) must drive the checked thumb fill —
        // pure red, distinct from the default accent thumb.
        let custom = js_switch(
            &SwitchSpec::new().with_checked(true).with_on_color("#ff0000"),
            &theme(),
        );
        let tree = probe(&custom, 220.0, 60.0);
        // Thumb is the deepest node; its fill should be ~pure red.
        let thumb = tree.nodes.iter().max_by_key(|n| n.depth).unwrap();
        let bg = thumb.bg.expect("thumb has no fill");
        assert!(
            bg.approx(ProbeColor { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, 0.05),
            "custom on_color not applied to thumb: {bg:?}",
        );
    }

    #[test]
    fn read_only_switch_is_not_disabled() {
        let el = js_switch(&SwitchSpec::new().with_read_only(true), &theme());
        assert!(!el.style.disabled);
    }

    #[test]
    fn dual_labels_flank_the_track() {
        let el = js_switch(
            &SwitchSpec::new().with_left_label("Off").with_right_label("On"),
            &theme(),
        );
        let tree = probe(&el, 220.0, 40.0);
        assert!(
            tree.has_text("Off") && tree.has_text("On"),
            "dual labels missing: {:?}",
            tree.texts(),
        );
    }
}
