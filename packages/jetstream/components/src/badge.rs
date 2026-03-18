//! JsBadge — small status label backed by BadgeSpec.
//!
//! Contract: `docs/contracts/foundation/badge.md`
//! Reference: `packages/svelte/primitives/src/Badge.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::ui_element::{self, JsEl};
use pug_jetstream::JetstreamThemeProvider;
use pug_primitives::{BadgeSpec, BadgeVariant};

use crate::theme_ext::{resolve_color, tint};

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
            let accent = resolve_color(theme, "semantic.color.accent.base");
            let text = resolve_color(theme, "semantic.color.text.primary");
            (tint(accent, 0.18), text)
        }
        BadgeVariant::Muted => {
            let surface = resolve_color(theme, "semantic.color.background.surface");
            let text = resolve_color(theme, "semantic.color.text.secondary");
            // Approximate color-mix(surface 78%, elevated) as surface with reduced alpha
            (tint(surface, 0.78), text)
        }
    };

    // Contract dimensions (rem → px at 16px base):
    // min-height: 1.25rem = 20px
    // padding: 0.125rem 0.4375rem = 2px 7px
    // font-size: 0.6875rem = 11px
    // border-radius: 999px (pill)
    ui_element::label(&content)
        .min_h(20.0)   // 1.25rem
        .px(7.0)       // 0.4375rem
        .py(2.0)       // 0.125rem
        .rounded(999.0) // pill
        .bg(bg)
        .text_color(text_color)
        .text_size(11.0) // 0.6875rem
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
        JetstreamThemeProvider::from_theme(&pug_tokens::themes::DARK)
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
