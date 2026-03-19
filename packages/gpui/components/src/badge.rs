//! PugBadge — real GPUI component backed by BadgeSpec.
//!
//! Implements the badge contract exactly: uppercase text, pill radius,
//! correct color-mix formulas per variant.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{BadgeSpec, BadgeVariant};

use crate::theme_ext::{color_mix, resolve_color};

/// A real GPUI badge component backed by `BadgeSpec`.
pub struct PugBadge {
    spec: BadgeSpec,
    theme: GpuiThemeProvider,
}

impl PugBadge {
    pub fn new(spec: BadgeSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugBadge {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Contract §8: variant-specific colors
        let (fill, text) = match spec.variant {
            BadgeVariant::Accent => {
                // background: color-mix(accent-base 18%, transparent)
                let accent = resolve_color(theme, "semantic.color.accent.base");
                let fill = accent.opacity(0.18);
                // color: text-primary
                let text = resolve_color(theme, "semantic.color.text.primary");
                (fill, text)
            }
            BadgeVariant::Muted => {
                // background: color-mix(surface 78%, elevated)
                let surface = resolve_color(theme, "semantic.color.background.surface");
                let elevated = resolve_color(theme, "semantic.color.background.elevated");
                let fill = color_mix(surface, elevated, 0.78);
                // color: text-secondary
                let text = resolve_color(theme, "semantic.color.text.secondary");
                (fill, text)
            }
        };

        // Contract §8: uppercase the label programmatically
        let label = spec
            .content
            .clone()
            .unwrap_or_default()
            .to_uppercase();

        div()
            // Contract §8: padding 0.125rem 0.4375rem = 2px 7px
            .px(px(7.0))
            .py(px(2.0))
            // Contract §8: min-height 1.25rem = 20px
            .min_h(px(20.0))
            // Contract §8: border-radius 999px (pill)
            .rounded(px(999.0))
            .bg(fill)
            // Contract §8: font-size 0.6875rem = 11px
            .text_size(px(11.0))
            // Contract §8: font-weight 700
            .font_weight(FontWeight::BOLD)
            .text_color(text)
            .flex()
            .items_center()
            .justify_center()
            .child(label)
            .into_any_element()
    }
}
