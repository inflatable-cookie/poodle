//! Pill — real GPUI component backed by PillSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{
    IconSize, IconSpec, PillAppearance, PillFont, PillSize, PillSpec, PillTone,
};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

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
            PillSize::Xs => (px(14.0), px(5.0), px(1.0), px(9.0)),
            PillSize::Sm => (px(16.0), px(6.0), px(2.0), px(10.0)),
            PillSize::Md => (px(20.0), px(8.0), px(3.0), px(11.0)),
            PillSize::Lg => (px(22.0), px(10.0), px(4.0), px(12.0)),
            PillSize::Xl => (px(24.0), px(12.0), px(5.0), px(13.0)),
        };

        // ── Colors ───────────────────────────────────────────────
        // Matches Svelte Pill.svelte exactly:
        //   - Neutral: 90% surface fill, 82% border-subtle border
        //   - Success/Danger: 14% tone-tinted fill, 38% tone-tinted border
        //   - Info/Warning: neutral fill and border (no Svelte overrides)
        //   - Badge: accent-base at 18% fill (except neutral badge: desaturated)
        //   - Badge border: always transparent
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_color = resolve_color(theme, spec.text_color_token());
        let tone_color = resolve_color(theme, spec.tone_color_token());
        let accent_base = resolve_color(theme, "semantic.color.accent.base");
        let border_subtle = resolve_color(theme, "semantic.color.border.subtle");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let bg = match spec.appearance {
            PillAppearance::Badge => {
                match spec.tone {
                    // Neutral badge: desaturated surface (96% surface + 4% text-primary)
                    PillTone::Neutral => {
                        let text_primary = resolve_color(theme, "semantic.color.text.primary");
                        surface_bg.blend(text_primary.opacity(0.04))
                    }
                    // All other badges: accent-base at 18%
                    _ => accent_base.opacity(0.18),
                }
            }
            _ => {
                match spec.tone {
                    // Success/Danger: 14% tone tint into surface
                    PillTone::Success | PillTone::Danger => {
                        surface_bg.blend(tone_color.opacity(0.14))
                    }
                    // Neutral/Info/Warning: 90% surface (Svelte has no overrides for info/warning)
                    _ => surface_bg.opacity(0.9),
                }
            }
        };

        // Subtle appearance: reduce fill to 50% opacity (Svelte: color-mix 50%, transparent)
        let bg = if spec.appearance == PillAppearance::Subtle {
            Hsla { a: bg.a * 0.5, ..bg }
        } else {
            bg
        };

        // Border: only success and danger get tone-colored borders in Svelte
        let border = match spec.appearance {
            PillAppearance::Badge => gpui::transparent_black(),
            _ => {
                match spec.tone {
                    PillTone::Success | PillTone::Danger => {
                        border_subtle.blend(tone_color.opacity(0.38))
                    }
                    // Neutral/Info/Warning: plain border-subtle at 82%
                    _ => border_subtle.opacity(0.82),
                }
            }
        };

        // Text color: badges use primary for toned, secondary for neutral
        let text_color = match spec.appearance {
            PillAppearance::Badge => {
                match spec.tone {
                    PillTone::Neutral => text_secondary,
                    _ => resolve_color(theme, "semantic.color.text.primary"),
                }
            }
            _ => text_color,
        };

        let radius = resolve_radius(theme, "semantic.radius.pill");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let inline_gap = px(4.0);
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        // ── Font weight ──────────────────────────────────────────
        let weight = if spec.appearance == PillAppearance::Badge {
            FontWeight::BOLD
        } else {
            FontWeight::SEMIBOLD
        };

        let mut el = div()
            .id(SharedString::from("poodle-pill"))
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

        el = el.focus(move |s| s.border_color(focus_ring).shadow(crate::theme_ext::focus_ring_shadow(focus_ring)));

        // Muted: reduced opacity (Svelte: opacity 0.72)
        if spec.is_muted {
            el = el.opacity(0.72);
        }

        if spec.is_disabled {
            el = el
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        if spec.is_removable {
            let icon_muted = resolve_color(theme, "semantic.color.icon.muted");
            let remove_id = SharedString::from("poodle-pill-remove");
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
