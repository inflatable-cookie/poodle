//! Pill — real GPUI component backed by PillSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{
    IconSize, IconSpec, PillAppearance, PillFont, PillSize, PillSpec, PillTone,
};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI pill component backed by `PillSpec`.
pub struct Pill {
    spec: PillSpec,
    theme: GpuiThemeProvider,
    on_remove: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Pill {
    type Target = PillSpec;
    fn deref(&self) -> &PillSpec { &self.spec }
}

impl Pill {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: PillSpec::new(), theme: theme.clone(), on_remove: None }
    }

    pub fn from_spec(spec: PillSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_remove: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = v.into(); self }
    pub fn tone(mut self, v: PillTone) -> Self { self.spec.tone = v; self }
    pub fn appearance(mut self, v: PillAppearance) -> Self { self.spec.appearance = v; self }
    pub fn size(mut self, v: PillSize) -> Self { self.spec.size = v; self }
    pub fn font(mut self, v: PillFont) -> Self { self.spec.font = v; self }
    pub fn muted(mut self, v: bool) -> Self { self.spec.is_muted = v; self }
    pub fn removable(mut self, v: bool) -> Self { self.spec.is_removable = v; self }
    pub fn selected(mut self, v: bool) -> Self { self.spec.is_selected = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }


    pub fn on_remove(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_remove = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Pill {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Size-dependent values ────────────────────────────────
        let (min_h, pad_x, pad_y, font_size) = match spec.size {
            PillSize::Xxs => (px(16.0), px(6.0), px(2.0), px(10.0)),
            PillSize::Xs  => (px(20.0), px(8.0), px(3.0), px(11.0)),
            PillSize::Sm  => (px(22.0), px(10.0), px(4.0), px(12.0)),
        };

        // ── Colors ───────────────────────────────────────────────
        let surface_bg = resolve_color(theme, spec.fill_token());
        let text_color = resolve_color(theme, spec.text_color_token());
        let tone_color = resolve_color(theme, spec.tone_color_token());

        // Compute tinted background: for toned pills, mix tone into surface
        let bg = match spec.tone {
            PillTone::Neutral => {
                if spec.appearance == PillAppearance::Badge {
                    // Neutral badge: slightly tinted surface
                    surface_bg.blend(surface_bg.opacity(0.96))
                } else {
                    surface_bg.blend(surface_bg.opacity(0.9))
                }
            }
            _ => {
                if spec.appearance == PillAppearance::Badge {
                    // Badge: stronger tone tint
                    surface_bg.blend(tone_color.opacity(0.18))
                } else {
                    // Solid/Subtle: light tone tint
                    surface_bg.blend(tone_color.opacity(0.14))
                }
            }
        };

        // Border: tone-tinted for non-neutral, subtle for neutral
        let border_subtle = resolve_color(theme, "semantic.color.border.subtle");
        let border = match spec.tone {
            PillTone::Neutral => border_subtle.opacity(0.82),
            _ => {
                if spec.appearance == PillAppearance::Badge {
                    gpui::transparent_black()
                } else {
                    border_subtle.blend(tone_color.opacity(0.38))
                }
            }
        };

        let radius = resolve_radius(theme, "semantic.radius.pill");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let inline_gap = resolve_px(theme, "semantic.space.inline.xs");
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        // ── Font weight ──────────────────────────────────────────
        let weight = if spec.appearance == PillAppearance::Badge {
            FontWeight::BOLD
        } else {
            FontWeight::SEMIBOLD
        };

        let mut el = div()
            .id(SharedString::from("pug-pill"))
            .focusable()
            .min_h(min_h)
            .px(pad_x)
            .py(pad_y)
            .rounded(radius)
            .bg(bg)
            .border_1()
            .border_color(border)
            .text_size(font_size)
            .font_weight(weight)
            .text_color(text_color)
            .flex()
            .items_center()
            .justify_center()
            .gap(inline_gap);

        // Badge: auto-uppercase the label (Svelte uses CSS text-transform)
        let display_label = if spec.appearance == PillAppearance::Badge {
            spec.label.to_uppercase()
        } else {
            spec.label.clone()
        };
        el = el.child(display_label);

        el = el.focus(move |s| s.border_color(focus_ring));

        // Muted: reduced opacity
        if spec.is_muted {
            el = el.opacity(0.72);
        }

        // Subtle appearance: reduced opacity
        if spec.appearance == PillAppearance::Subtle {
            let base_opacity = if spec.is_muted { 0.72 } else { 1.0 };
            el = el.opacity(base_opacity * 0.85);
        }

        if spec.is_disabled {
            el = el
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        if spec.is_removable {
            let icon_muted = resolve_color(theme, "semantic.color.icon.muted");
            let remove_id = SharedString::from("pug-pill-remove");
            let mut remove_btn = div()
                .id(remove_id)
                .cursor_pointer()
                .flex()
                .items_center()
                .child(
                    Icon::from_spec(
                        IconSpec::new("x").with_size(IconSize::Sm),
                        theme,
                    )
                    .with_color(icon_muted),
                );

            if let Some(handler) = self.on_remove {
                remove_btn =
                    remove_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }

            el = el.child(remove_btn);
        }

        el.into_any_element()
    }
}
