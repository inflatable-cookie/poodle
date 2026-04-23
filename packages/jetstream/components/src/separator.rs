//! JsSeparator — horizontal or vertical divider backed by SeparatorSpec.
//!
//! Contract: `docs/contracts/components/separator.md`
//! Reference: `packages/svelte/components/src/Separator.svelte`
//!
//! ALL dimensions from contract. ZERO hardcoded values.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{SeparatorOrientation, SeparatorSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, tint};

/// Build a separator element from a SeparatorSpec.
///
/// Contract anatomy:
/// ```text
/// [Root .separator] — <div>
/// ```
///
/// Contract dimensions:
/// - Horizontal: width 100%, min-height 0.0625rem (1px)
/// - Vertical: width 0.0625rem (1px), align-self stretch, min-height 100%
/// - flex: 0 0 auto (no grow/shrink)
/// - Subtle tone: color-mix(border-subtle 72%, transparent)
/// - Default tone: border-default full color
pub fn js_separator(spec: &SeparatorSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Contract: color depends on tone
    // - subtle: color-mix(border-subtle 72%, transparent)
    // - default: border-default full color
    let color = match spec.tone {
        poodle_specs::RuleTone::Subtle => {
            let border_subtle = resolve_color(theme, "color.border.subtle");
            tint(border_subtle, 0.72) // color-mix with 72% opacity
        }
        poodle_specs::RuleTone::Default => {
            resolve_color(theme, "color.border.default")
        }
    };

    // Contract: flex 0 0 auto — separator doesn't grow or shrink
    let stroke = rem_to_px(0.0625); // 0.0625rem = 1px at 16px base
    match spec.orientation {
        SeparatorOrientation::Horizontal => {
            ui_element::div()
                .min_h(stroke)
                .self_stretch() // width 100%
                .bg(color)
                .flex_none() // flex: 0 0 auto
        }
        SeparatorOrientation::Vertical => {
            ui_element::div()
                .w(stroke)
                .h_full()   // min-height 100%
                .bg(color)
                .flex_none() // flex: 0 0 auto
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn horizontal_separator_has_bg() {
        let el = js_separator(&SeparatorSpec::new(), &theme());
        assert!(el.style.background.is_some());
    }

    #[test]
    fn horizontal_separator_min_height_1px() {
        let el = js_separator(&SeparatorSpec::new(), &theme());
        assert_eq!(el.layout.min_size.height, taffy::Dimension::length(1.0));
    }

    #[test]
    fn vertical_separator_width_1px() {
        let el = js_separator(
            &SeparatorSpec::new().with_orientation(SeparatorOrientation::Vertical),
            &theme(),
        );
        assert_eq!(el.layout.size.width, taffy::Dimension::length(1.0));
    }
}
