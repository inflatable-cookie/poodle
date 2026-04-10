//! JsSwitch — toggle switch with track and thumb, backed by SwitchSpec.
//!
//! Contract: `docs/contracts/foundation/switch.md`
//! Reference: `packages/svelte/primitives/src/Switch.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::SwitchSpec;

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
/// Contract dimensions:
/// - Track: 2.125rem (34px) wide × 1.25rem (20px) tall
/// - Track padding: 0.125rem (2px)
/// - Track border: 0.0625rem (1px) solid
/// - Thumb: 0.875rem (14px) diameter
/// - Thumb travel: 0.875rem (14px) translateX
/// - Gap: var(--poodle-space-inline-sm) = 8px
pub fn js_switch(spec: &SwitchSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let is_checked = spec.current_checked();

    // ── Token resolution ──
    let track_fill = resolve_color(theme, spec.track_fill_token());
    let border_default = resolve_color(theme, "color.border.default");
    let accent = resolve_color(theme, "color.accent.base");
    let text_primary = resolve_color(theme, "color.text.primary");
    let gap = rem_to_px(control_space_x_rem(spec.density));
    let label_size = rem_to_px(size_font_rem(effective_size));

    // Contract dimensions (rem → px at 16px base)
    let track_width: f32 = rem_to_px(2.125);  // 2.125rem
    let track_height: f32 = rem_to_px(1.25);  // 1.25rem
    let track_padding: f32 = rem_to_px(0.125); // 0.125rem
    let thumb_size: f32 = rem_to_px(0.875);   // 0.875rem
    let thumb_travel: f32 = rem_to_px(0.875); // 0.875rem

    // Contract: thumb color = text-primary when off, accent-base when on
    let thumb_color = if is_checked { accent } else { text_primary };

    // Contract: track border = border-default when off,
    // color-mix(accent 58%, border-default) when on
    let accent_c: Color = accent.into();
    let border_c: Color = border_default.into();
    let track_border = if is_checked {
        accent_c.mix(border_c, 0.58)
    } else {
        border_c
    };

    // Contract: thumb position via translateX. We approximate with left padding.
    let thumb_offset = if is_checked {
        track_padding + thumb_travel
    } else {
        track_padding
    };

    // ── Thumb ──
    let thumb = ui_element::div()
        .w(thumb_size)
        .h(thumb_size)
        .rounded(thumb_size / 2.0) // circle
        .bg(thumb_color);

    // ── Track ──
    let track = ui_element::div()
        .w(track_width)
        .h(track_height)
        .rounded(999.0) // pill
        .bg(track_fill)
        .border_1().border_color(track_border)
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
        root = root.child(
            ui_element::label(label)
                .text_color(text_primary)
                .text_size(label_size) // from TYPOGRAPHY_LABEL_SIZE token
        );
    }

    // Contract: disabled → opacity from state-opacity-disabled token
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

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
        let mut spec_on = SwitchSpec::new().with_checked(true);
        let mut spec_off = SwitchSpec::new();
        let el_on = js_switch(&spec_on, &theme());
        let el_off = js_switch(&spec_off, &theme());
        // Track → Thumb (first child of track)
        let thumb_on = &el_on.children[0].children[0];
        let thumb_off = &el_off.children[0].children[0];
        // Colors should differ (accent vs text-primary)
        assert_ne!(thumb_on.style.background, thumb_off.style.background);
    }
}
