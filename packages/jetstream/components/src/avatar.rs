//! JsAvatar — avatar backed by AvatarSpec.
//!
//! Contract: `docs/contracts/components/avatar.md`
//! Reference: GPUI `primitives/avatar.rs` (initials treatment).
//!
//! Renders the initials fallback in a tone-colored, shaped frame. Image `src`
//! loading is a runtime gap (GPUI is also initials-only). ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{AvatarShape, AvatarSpec, AvatarTone};

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

/// Build an avatar element from an `AvatarSpec`.
pub fn js_avatar(spec: &AvatarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let size = rem_to_px(spec.size_rem());
    let font_size = rem_to_px(spec.font_size_rem());

    let surface = resolve_color(theme, "color.background.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");

    let (bg, fg) = match spec.tone {
        AvatarTone::Neutral => (color_mix(surface, text_primary, 0.82), text_secondary),
        AvatarTone::Accent => (color_mix(accent, surface, 0.76), text_primary),
    };
    let radius = match spec.shape {
        AvatarShape::Circle => rem_to_px(999.0), // pill → circle for a square box
        AvatarShape::Rounded => resolve_radius(theme, "radius.control"),
    };

    ui_element::div()
        .w(size)
        .h(size)
        .flex_row()
        .items_center()
        .justify_center()
        .overflow_hidden()
        .flex_none()
        .rounded(radius)
        .bg(bg)
        .child(
            ui_element::label(spec.fallback_text())
                .text_color(fg)
                .text_size(font_size)
                .text_weight(600),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::AvatarSize;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_initials_fallback() {
        let el = js_avatar(&AvatarSpec::new().with_initials("tw"), &theme());
        let tree = probe(&el, 80.0, 80.0);
        // fallback_text upper-cases initials.
        assert!(tree.has_text("TW"), "avatar initials missing: {:?}", tree.texts());
    }

    #[test]
    fn empty_initials_show_question_mark() {
        let el = js_avatar(&AvatarSpec::new(), &theme());
        let tree = probe(&el, 80.0, 80.0);
        assert!(tree.has_text("?"), "empty avatar should fall back to '?'");
    }

    #[test]
    fn size_drives_box_dimension() {
        let el = js_avatar(&AvatarSpec::new().with_size(AvatarSize::Xl), &theme());
        // xl = 6rem = 96px square.
        assert_eq!(el.layout.size.width, taffy::Dimension::length(rem_to_px(6.0)));
    }
}
