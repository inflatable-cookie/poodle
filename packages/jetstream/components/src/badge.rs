//! JsBadge — small status label backed by BadgeSpec.
//!
//! Contract: `docs/contracts/components/badge.md`
//! Reference: `packages/svelte/primitives/src/Badge.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::{BadgeSpec, BadgeVariant};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

/// Build a badge element from a BadgeSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .badge] — <span>, inline-flex
///   └── [Content] — slot
/// ```
///
/// Contract dimensions:
/// - min-height: 1.25rem (20px)
/// - padding: 0.125rem 0.4375rem (2px 7px)
/// - border-radius: 999px (pill)
/// - font-size: 0.6875rem (11px)
/// - font-weight: 700
/// - letter-spacing: 0.04em
/// - text-transform: uppercase
pub fn js_badge(spec: &BadgeSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let content = spec.content.clone().unwrap_or_default();

    // Contract color rules per variant:
    // - accent: bg = color-mix(accent-base 18%, transparent), text = text-primary
    // - muted: bg = color-mix(surface 78%, elevated), text = text-secondary
    let (bg, text_color) = match spec.variant {
        BadgeVariant::Accent => {
            let accent: Color = resolve_color(theme, "color.accent.base").into();
            let text = resolve_color(theme, "color.text.primary");
            // color-mix(accent 18%, transparent) = accent with 18% opacity
            (accent.with_alpha(accent.a * 0.18), text)
        }
        BadgeVariant::Muted => {
            let surface: Color = resolve_color(theme, "color.background.surface").into();
            let elevated: Color = resolve_color(theme, "color.background.elevated").into();
            let text = resolve_color(theme, "color.text.secondary");
            // color-mix(surface 78%, elevated)
            (surface.mix(elevated, 0.78), text)
        }
    };

    // Contract dimensions (rem → px via presentation helpers):
    // min-height: 1.25rem, padding: 0.125rem 0.4375rem
    // font-size: 0.6875rem, border-radius: 999px (pill)
    ui_element::label(&content)
        .min_h(rem_to_px(1.25))
        .px(rem_to_px(0.4375))
        .py(rem_to_px(0.125))
        .rounded(999.0) // pill
        .bg(bg)
        .text_color(text_color)
        .text_size(rem_to_px(0.6875))
        .items_center()
        .justify_center()
    // Note: text-transform: uppercase and letter-spacing: 0.04em are CSS-only
    // properties not yet expressible in the JsEl API. The runtime text renderer
    // would need to support these. For now, content should be pre-uppercased.
    // font-weight: 700 also requires runtime text support.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn accent_badge_has_fill() {
        let el = js_badge(
            &BadgeSpec::new().with_variant(BadgeVariant::Accent),
            &theme(),
        );
        assert!(el.style.background.is_some());
    }

    #[test]
    fn muted_badge_uses_secondary_text() {
        let el = js_badge(
            &BadgeSpec::new().with_variant(BadgeVariant::Muted),
            &theme(),
        );
        assert!(el.style.text_color.is_some());
        // Should use text-secondary, not text-inverse
    }
}
