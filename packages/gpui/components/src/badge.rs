//! PugBadge — real GPUI component backed by BadgeSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::BadgeSpec;

use crate::theme_ext::resolve_color;

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

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text = match spec.variant {
            pug_gpui_primitives::BadgeVariant::Accent => {
                resolve_color(theme, "semantic.color.text.inverse")
            }
            pug_gpui_primitives::BadgeVariant::Muted => {
                resolve_color(theme, "semantic.color.text.primary")
            }
        };

        let label = spec.content.clone().unwrap_or_default();

        div()
            .px(px(8.0))
            .py(px(2.0))
            .rounded(px(10.0))
            .bg(fill)
            .border_1()
            .border_color(border.opacity(0.3))
            .text_xs()
            .text_color(text)
            .flex()
            .items_center()
            .child(label)
            .into_any_element()
    }
}
