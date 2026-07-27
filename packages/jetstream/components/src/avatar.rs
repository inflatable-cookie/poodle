//! JsAvatar — avatar backed by AvatarSpec.
//!
//! Contract: `docs/contracts/components/avatar.md`
//! Reference: GPUI `primitives/avatar.rs` (initials treatment).
//!
//! Renders the initials fallback in a tone-colored, shaped frame. Image `src`
//! loading is a runtime gap (GPUI is also initials-only). ZERO hardcoded values.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::AvatarSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

/// Build an avatar element from an `AvatarSpec`.
pub fn js_avatar(spec: &AvatarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let size = rem_to_px(spec.size_rem());
    let font_size = rem_to_px(spec.font_size_rem());

    // Tone colors via the spec token targets + mix ratio (contract §3), not
    // inlined token strings. Cross-target color delta (linear vs sRGB mix) is
    // an accepted §12 delta.
    let base = resolve_color(theme, spec.background_base_token());
    let mix = resolve_color(theme, spec.background_mix_token());
    let bg = color_mix(base, mix, spec.background_mix_ratio());
    let fg = resolve_color(theme, spec.color_token());

    // Circle = border-radius:50% → half the box size; rounded = radius token.
    let radius = if spec.is_circle() {
        rem_to_px(spec.circle_radius_rem())
    } else {
        resolve_radius(theme, spec.radius_token())
    };

    let mut root = ui_element::div()
        .w(size)
        .h(size)
        .flex_row()
        .items_center()
        .justify_center()
        .overflow_hidden()
        .flex_none()
        .rounded(radius)
        .bg(bg);

    if spec.has_image() {
        // Image fills the square (object-fit: cover, contract §3). Decode/load
        // of the URL is owned by the Jetstream texture pipeline.
        let src = spec.src.clone().unwrap_or_default();
        root = root.child(ui_element::image(src).w(size).h(size).object_fit_cover());
    } else {
        // Initials (or "?") fallback frame.
        root = root.child(
            ui_element::label(spec.fallback_text())
                .text_color(fg)
                .text_size(font_size)
                .text_weight(600),
        );
    }

    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::{AvatarShape, AvatarSize, AvatarTone};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
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

    #[test]
    fn size_scale_matches_contract() {
        // Contract §3 size table → box dimension (rem * 16).
        let cases = [
            (AvatarSize::Xs, 1.5_f32),
            (AvatarSize::Sm, 2.0),
            (AvatarSize::Md, 2.75),
            (AvatarSize::Lg, 4.5),
            (AvatarSize::Xl, 6.0),
        ];
        for (size, rem) in cases {
            let el = js_avatar(&AvatarSpec::new().with_size(size), &theme());
            assert_eq!(
                el.layout.size.width,
                taffy::Dimension::length(rem_to_px(rem)),
                "{size:?} box width"
            );
        }
    }

    #[test]
    fn circle_radius_is_half_the_box() {
        // Circle = border-radius:50% → half the size, never a 999 sentinel.
        let el = js_avatar(
            &AvatarSpec::new().with_size(AvatarSize::Md).with_shape(AvatarShape::Circle),
            &theme(),
        );
        assert_eq!(el.style.corner_radii[0], rem_to_px(2.75 / 2.0));
    }

    #[test]
    fn rounded_uses_control_radius_token() {
        let th = theme();
        let el = js_avatar(&AvatarSpec::new().with_shape(AvatarShape::Rounded), &th);
        assert_eq!(el.style.corner_radii[0], resolve_radius(&th, "radius.control"));
    }

    #[test]
    fn image_src_renders_image_node_not_initials() {
        let el = js_avatar(
            &AvatarSpec::new().with_src("https://example.com/a.png").with_initials("tw"),
            &theme(),
        );
        let tree = probe(&el, 80.0, 80.0);
        // src present → image node, no initials label.
        assert_eq!(tree.count_kind("Image"), 1, "expected one image node");
        assert!(!tree.has_text("TW"), "initials should not render when src is set");
    }

    #[test]
    fn accent_tone_background_differs_from_neutral() {
        let th = theme();
        let neutral = js_avatar(&AvatarSpec::new().with_tone(AvatarTone::Neutral), &th);
        let accent = js_avatar(&AvatarSpec::new().with_tone(AvatarTone::Accent), &th);
        assert_ne!(
            neutral.style.background, accent.style.background,
            "tone must change the resolved background fill"
        );
    }
}
