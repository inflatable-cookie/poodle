//! HoverCard — real GPUI component backed by HoverCardSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{HoverCardSpec, OverlayPlacement};

use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// A real GPUI hover card component backed by `HoverCardSpec`.
pub struct HoverCard {
    spec: HoverCardSpec,
    theme: GpuiThemeProvider,
    trigger: Option<AnyElement>,
    content: Option<AnyElement>,
}

impl std::ops::Deref for HoverCard {
    type Target = HoverCardSpec;
    fn deref(&self) -> &HoverCardSpec { &self.spec }
}

impl HoverCard {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: HoverCardSpec::new(), theme: theme.clone(), trigger: None, content: None }
    }

    pub fn from_spec(spec: HoverCardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            trigger: None,
            content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }
    pub fn placement(mut self, v: OverlayPlacement) -> Self { self.spec.placement = v; self }


    pub fn with_trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for HoverCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let fill = resolve_color(theme, spec.fill_token());
        let border_raw = resolve_color(theme, "semantic.color.border.default");
        let panel = resolve_color(theme, "semantic.color.background.panel");
        let radius = resolve_radius(theme, "semantic.radius.surface");
        // Contract: border 72% border-default
        let border = color_mix(border_raw, panel, 0.72);

        let mut wrapper = div().flex().flex_col().gap(px(4.0));

        // Trigger (always rendered)
        if let Some(trigger) = self.trigger {
            wrapper = wrapper.child(trigger);
        }

        // Surface content (shown when open)
        if spec.is_open {
            let mut surface = div()
                .px(inline_padding)
                .py(px(10.0))
                .rounded(radius)
                .bg(fill)
                .border_1()
                .border_color(border)
                // Contract: elevation-popover shadow
                .shadow(vec![
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.10),
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(16.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.06),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(4.0),
                        spread_radius: px(0.0),
                    },
                ])
                .min_w(px(192.0))  // 12rem
                .max_w(px(320.0)); // 20rem

            if let Some(content) = self.content {
                surface = surface.child(content);
            }

            wrapper = wrapper.child(surface);
        }

        wrapper.into_any_element()
    }
}
