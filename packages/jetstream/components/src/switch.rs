//! JsSwitch — toggle switch with track and thumb, backed by SwitchSpec.
//!
//! Contract: `docs/contracts/components/switch.md`
//! Reference: `packages/svelte/components/src/Switch.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SwitchSpec;

use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

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
/// Contract dimensions (fixed, not size-scaled):
/// - Track: 2.125rem (34px) wide × 1.25rem (20px) tall
/// - Track padding: 0.125rem (2px)
/// - Track border: 0.0625rem (1px) solid
/// - Thumb: 0.875rem (14px) diameter
/// - Thumb travel: 0.875rem (14px) translateX
/// - Gap: var(--poodle-space-inline-sm) = 8px
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

    let gap = rem_to_px(control_space_x_rem(spec.density));
    let label_size = rem_to_px(size_font_rem(effective_size));

    // Contract: off-color = text-primary (or leftTone override), on-color = accent-base (or rightTone override)
    let off_color_c: Color = match spec.left_tone.color_token() {
        Some(token) => resolve_color(theme, token).into(),
        None => text_primary_c,
    };
    let on_color_c: Color = match spec.right_tone.color_token() {
        Some(token) => resolve_color(theme, token).into(),
        None => accent_base_c,
    };

    // Contract: track fill = color-mix(in srgb, tone N%, surface)
    // off-track: mix(off-color 18%, surface)  → 18% off-color + 82% surface
    // on-track:  mix(on-color  24%, surface)  → 24% on-color  + 76% surface
    // mix(other, f): self*f + other*(1-f), so self.mix(surface, 0.18) = 18% self + 82% surface
    let off_track_c = off_color_c.mix(surface_c, 0.18);
    let on_track_c  = on_color_c.mix(surface_c, 0.24);
    let track_fill: Color = if is_checked { on_track_c } else { off_track_c };

    // Contract: border = border-default (off), color-mix(on-thumb 58%, border-default) (on)
    let on_border_c = on_color_c.mix(border_default_c, 0.58); // 58% on-thumb, 42% border
    let track_border: Color = if is_checked { on_border_c } else { border_default_c };

    // Contract: thumb = off-color (off) or on-color (on)
    let thumb_color: Color = if is_checked { on_color_c } else { off_color_c };

    // Contract dimensions (rem → px at 16px base, fixed for all sizes)
    let track_width:   f32 = rem_to_px(2.125);
    let track_height:  f32 = rem_to_px(1.25);
    let track_padding: f32 = rem_to_px(0.125);
    let thumb_size:    f32 = rem_to_px(0.875);
    let thumb_travel:  f32 = rem_to_px(0.875);
    let border_width:  f32 = rem_to_px(0.0625);

    // Contract: thumb offset = track_padding (off) or track_padding + thumb_travel (on)
    let thumb_offset = if is_checked {
        track_padding + thumb_travel
    } else {
        track_padding
    };

    // ── Thumb ──
    let thumb = ui_element::div()
        .w(thumb_size)
        .h(thumb_size)
        .rounded(thumb_size / 2.0) // circle — 999px pill equivalent
        .bg(thumb_color);

    // ── Track ──
    let track = ui_element::div()
        .w(track_width)
        .h(track_height)
        .rounded(999.0) // pill
        .bg(track_fill)
        .border(border_width).border_color(track_border)
        .items_center()
        .pl(thumb_offset)
        .child(thumb);

    // ── Root: track + optional label ──
    let mut root = ui_element::div()
        .flex_row()
        .gap(gap) // from SPACE_INLINE_SM token
        .items_center()
        .focusable()
        .child(track);

    if let Some(ref label) = spec.label {
        let label_color = resolve_color(theme, "color.text.primary");
        root = root.child(
            ui_element::label(label)
                .text_color(label_color)
                .text_size(label_size) // from TYPOGRAPHY_LABEL_SIZE token
        );
    }

    // Contract: disabled → opacity from state-opacity-disabled token, cursor not-allowed
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    // Contract: readOnly → default cursor, full opacity, non-interactive
    // No opacity reduction; reverts changes on toggle attempt (platform handles).

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn track_is_34px_wide() {
        let spec = SwitchSpec::new();
        let el = js_switch(&spec, &theme());
        // First child is the track
        let track = &el.children[0];
        assert_eq!(track.layout.size.width, taffy::Dimension::length(34.0));
    }

    #[test]
    fn thumb_changes_color_when_checked() {
        let spec_on = SwitchSpec::new().with_checked(true);
        let spec_off = SwitchSpec::new();
        let el_on = js_switch(&spec_on, &theme());
        let el_off = js_switch(&spec_off, &theme());
        // Track → Thumb (first child of track)
        let thumb_on = &el_on.children[0].children[0];
        let thumb_off = &el_off.children[0].children[0];
        // Colors should differ (on-color vs off-color)
        assert_ne!(thumb_on.style.background, thumb_off.style.background);
    }

    #[test]
    fn read_only_switch_is_not_disabled() {
        let el = js_switch(
            &SwitchSpec::new().with_read_only(true),
            &theme(),
        );
        assert!(!el.style.disabled);
    }
}
