//! Button — real GPUI component backed by ButtonSpec.
//!
//! Implements the button contract (`docs/contracts/components/button.md`) exactly,
//! resolving all tokens through ButtonSpec + GpuiThemeProvider, and matching the
//! Svelte implementation's visual output.
//!
//! `ButtonSpec::aria_expanded` is stored for cross-runtime parity with the
//! contract’s `ariaExpanded` prop. GPUI’s fluent element API does not yet expose
//! ARIA attributes on nodes (see `docs/roadmaps/g10/012-gpui-runtime-truth-and-deferred-work-closure.md`), so
//! the value is not emitted to the platform accessibility tree until that gap
//! closes.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonTone, ButtonVariant, ControlDensity, ControlSize, IconSize, IconSpec,
    SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant,
};

use super::icon::Icon;
use super::spinner::Spinner;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem, size_min_width_rem,
    size_padding_x_offset_rem,
};
use crate::theme_ext::{
    color_mix, color_mix_black, resolve_color, resolve_opacity, resolve_px, resolve_radius,
};

/// A real GPUI button component backed by `ButtonSpec`.
pub struct Button {
    spec: ButtonSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Button {
    type Target = ButtonSpec;
    fn deref(&self) -> &ButtonSpec {
        &self.spec
    }
}

impl Button {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ButtonSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_click: None,
        }
    }

    pub fn from_spec(spec: ButtonSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_click: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.spec.variant = v;
        self
    }
    pub fn tone(mut self, v: ButtonTone) -> Self {
        self.spec.tone = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn label(mut self, v: impl Into<String>) -> Self {
        self.spec.label = Some(v.into());
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn loading(mut self, v: bool) -> Self {
        self.spec.is_loading = v;
        self
    }
    pub fn leading_icon(mut self, v: impl Into<String>) -> Self {
        self.spec.leading_icon = Some(v.into());
        self
    }
    pub fn trailing_icon(mut self, v: impl Into<String>) -> Self {
        self.spec.trailing_icon = Some(v.into());
        self
    }
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
    pub fn chevron(mut self, v: bool) -> Self {
        self.spec.chevron = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn described_by(mut self, v: impl Into<String>) -> Self {
        self.spec.described_by = Some(v.into());
        self
    }
    pub fn aria_expanded(mut self, expanded: Option<bool>) -> Self {
        self.spec.aria_expanded = expanded;
        self
    }

    // ── GPUI-specific builders ────────────────────────────────
    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Button {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve base tokens ──────────────────────────────────
        let base_fill = resolve_color(theme, spec.resolved_fill_token());
        let base_text = resolve_color(theme, spec.resolved_text_token());
        let base_border = resolve_color(theme, spec.resolved_border_token());
        let elevated = resolve_color(theme, "color.background.elevated");
        let text_primary = resolve_color(theme, "color.text.primary");
        let radius = resolve_radius(theme, spec.radius_token());

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size-aware layout values (contract §7, §8) ──────────
        let base_height = resolve_px(theme, spec.control_height_token());
        let height = base_height + px(rem_to_px(size_height_offset_rem(effective_size)));
        let min_width = px(rem_to_px(size_min_width_rem(effective_size)));
        let base_pad_x = resolve_px(theme, spec.horizontal_padding_token());
        let pad_x = base_pad_x + px(rem_to_px(size_padding_x_offset_rem(effective_size)));
        let font_size = px(rem_to_px(size_font_rem(effective_size)));
        let gap = match spec.density {
            ControlDensity::Compact => resolve_px(theme, "space.inline.xs"),
            ControlDensity::Default => resolve_px(theme, ButtonSpec::content_gap_token()),
            ControlDensity::Comfortable => resolve_px(theme, "space.inline.md"),
        };

        // Icon padding adjustment (contract §8): reduce padding on icon side by 0.125rem
        let has_leading = spec.leading_icon.is_some() || spec.is_loading;
        let has_trailing = spec.trailing_icon.is_some() || spec.chevron;
        let icon_inset = resolve_px(theme, ButtonSpec::icon_side_inset_token());
        let pad_left = if has_leading {
            pad_x - icon_inset
        } else {
            pad_x
        };
        let pad_right = if has_trailing {
            pad_x - icon_inset
        } else {
            pad_x
        };

        let is_disabled = spec.is_disabled || spec.is_loading;
        let is_ghost = spec.variant == ButtonVariant::Ghost;

        // ── Compute variant-specific colors ──────────────────────

        // Danger×secondary needs color-mix blends (contract §8 Tone: danger)
        let (fill, border_color, text_color) = match (spec.variant, spec.tone) {
            (ButtonVariant::Secondary, ButtonTone::Danger) => {
                // fill: color-mix(status-danger 16%, background-surface)
                let danger = resolve_color(theme, "color.status.danger");
                let surface = resolve_color(theme, "color.background.surface");
                let border_default = resolve_color(theme, "color.border.default");
                let fill = color_mix(danger, surface, 0.16);
                // idle border: plain border-default (contract §8 Tone: danger)
                (fill, border_default, base_text)
            }
            (ButtonVariant::Primary, _) => {
                // Svelte treatment-interactive-primary-border: accent-base 86% + black
                let darkened_border = color_mix_black(base_fill, 0.86);
                (base_fill, darkened_border, base_text)
            }
            (ButtonVariant::Ghost, ButtonTone::Danger) => {
                // Ghost×danger: text uses status-danger instead of text-primary
                let danger_text = resolve_color(theme, "color.status.danger");
                (base_fill, base_border, danger_text)
            }
            _ => (base_fill, base_border, base_text),
        };

        // Ghost: transparent fill and border (contract §8 CSS Custom Properties)
        let (fill, border_color) = if is_ghost {
            (gpui::transparent_black(), gpui::transparent_black())
        } else {
            (fill, border_color)
        };

        // Pressed/toggle state (contract §8 Pressed/toggle state):
        // Non-primary variants get accent fill, accent border, inverse text
        let is_pressed = spec.is_toggle_mode() && spec.current_pressed();
        let (fill, border_color, text_color) =
            if is_pressed && spec.variant != ButtonVariant::Primary {
                let accent = resolve_color(theme, "color.accent.base");
                let text_inverse = resolve_color(theme, "color.text.inverse");
                let pressed_border = color_mix_black(accent, 0.86);
                (accent, pressed_border, text_inverse)
            } else {
                (fill, border_color, text_color)
            };

        // ── Hover/active colors (contract §8 Hover/Active) ──────
        // Danger tones use within-family color-mix formulas; all others mix toward
        // elevated / text-primary (contract §8 Tone: danger, Hover/Active).
        let (hover_fill, active_fill, hover_border) = match (spec.variant, spec.tone) {
            (ButtonVariant::Ghost, ButtonTone::Danger) => {
                let danger = resolve_color(theme, "color.status.danger");
                (
                    Hsla { a: 0.10, ..danger },
                    Hsla { a: 0.16, ..danger },
                    Hsla { a: 0.28, ..danger },
                )
            }
            (ButtonVariant::Secondary, ButtonTone::Danger) => {
                let danger = resolve_color(theme, "color.status.danger");
                let surface = resolve_color(theme, "color.background.surface");
                let border_default = resolve_color(theme, "color.border.default");
                (
                    color_mix(danger, surface, 0.24),
                    color_mix(danger, surface, 0.32),
                    color_mix(danger, border_default, 0.62),
                )
            }
            (ButtonVariant::Primary, ButtonTone::Danger) => {
                let danger = resolve_color(theme, "color.status.danger");
                let white = Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 };
                let black = Hsla { h: 0.0, s: 0.0, l: 0.0, a: 1.0 };
                let hf = color_mix(danger, white, 0.88);
                (hf, color_mix(danger, black, 0.88), color_mix_black(hf, 0.86))
            }
            _ => {
                let hf = color_mix(fill, elevated, 0.84);
                let af = color_mix(fill, elevated, 0.72);
                let hb = if is_ghost {
                    let border_default = resolve_color(theme, "color.border.default");
                    color_mix(border_default, text_primary, 0.78)
                } else {
                    color_mix(border_color, text_primary, 0.78)
                };
                (hf, af, hb)
            }
        };

        // ── Build element ID ─────────────────────────────────────
        let label_text = spec.label.clone().unwrap_or_default();
        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-btn-{}", suffix)
        } else {
            format!("poodle-btn-{}", label_text)
        };

        // ── Icon-only detection (contract §8 Icon-only) ──────────
        let is_icon_only = label_text.is_empty() && !spec.chevron;

        // ── Build root element (contract §8 Root) ────────────────
        let mut el = div()
            .id(SharedString::from(id_str))
            .focusable()
            .h(height)
            .rounded(radius);

        // Brand-raised treatment: use gradient fills and elevated shadows
        if theme.brand_raised && !is_ghost && !is_disabled {
            use crate::theme_ext::{
                brand_raised_interactive_fill, brand_raised_interactive_shadow,
                brand_raised_primary_fill, brand_raised_primary_shadow,
            };
            match spec.variant {
                ButtonVariant::Primary => {
                    el = el
                        .bg(brand_raised_primary_fill(fill))
                        .shadow(brand_raised_primary_shadow());
                }
                _ => {
                    el = el
                        .bg(brand_raised_interactive_fill(fill))
                        .shadow(brand_raised_interactive_shadow());
                }
            }
        } else {
            el = el.bg(fill);
        }

        el = el
            .border_1()
            .border_color(border_color)
            .text_color(text_color)
            .flex()
            .items_center()
            .justify_center()
            .gap(gap)
            .text_size(font_size)
            .font_weight(FontWeight::MEDIUM) // contract: typography-label-weight = 500
            .line_height(relative(1.0));

        // Icon-only: square button, no min-width, no padding
        if is_icon_only {
            el = el.w(height);
        } else {
            el = el.min_w(min_width).pl(pad_left).pr(pad_right);
        }

        // ── Focus ring (contract §8 Focus) ────────────────────────
        // GPUI has no CSS outline equivalent. We approximate with border-color
        // change on focus. Known delta: no outline-offset separation.
        let focus_ring_color = resolve_color(theme, spec.focus_ring_color_token());
        el = el.focus(move |s| {
            s.border_color(focus_ring_color)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring_color))
        });

        // ── Interactive states ────────────────────────────────────
        let icon_color = text_color;
        if is_disabled {
            // contract §8 Disabled: opacity: state-opacity-disabled, cursor: not-allowed
            let dis_o = resolve_opacity(theme, spec.disabled_opacity_token());
            el = el.opacity(dis_o).cursor(CursorStyle::OperationNotAllowed);
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| {
                    s.bg(hover_fill)
                        .border_color(hover_border)
                        .shadow(vec![gpui::BoxShadow {
                            color: hsla(0.0, 0.0, 1.0, 0.10),
                            offset: point(px(0.0), px(1.0)),
                            blur_radius: px(0.0),
                            spread_radius: px(0.0),
                        }])
                })
                // Contract: active press effect — use relative positioning to avoid layout shift
                .active(move |s| s.bg(active_fill));
        }

        // ── Spinner (contract §8 Spinner) ────────────────────────
        // Contract: 0.75rem (12px) border spinner, rotating 360° in 0.8s.
        // We use a custom spinner SVG with GPUI's animation API to rotate it.
        if spec.is_loading {
            el = el.child(
                Spinner::from_spec(
                    SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_size(SpinnerSize::Sm)
                        .with_tone(SpinnerTone::Current),
                    theme,
                )
                .with_color(icon_color),
            );
        }

        // ── Leading icon ─────────────────────────────────────────
        if let Some(ref icon_name) = spec.leading_icon {
            el = el.child(
                Icon::from_spec(
                    IconSpec::new(icon_name.clone()).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_color),
            );
        }

        // ── Label (contract §8 Label) ─────────────────────────────
        if !label_text.is_empty() {
            el = el.child(div().whitespace_nowrap().min_w(px(0.0)).child(label_text));
        }

        // ── Trailing icon ────────────────────────────────────────
        if let Some(ref icon_name) = spec.trailing_icon {
            el = el.child(
                Icon::from_spec(
                    IconSpec::new(icon_name.clone()).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_color),
            );
        }

        // ── Chevron (contract §8 Chevron) ────────────────────────
        // opacity: 0.5; margin-left: calc(space.inline.sm * -0.25) to tighten from gap
        if spec.chevron {
            let chevron_ml = px(-theme.resolve_space("space.inline.sm") * 0.25);
            el = el.child(
                div().flex().items_center().opacity(0.5).ml(chevron_ml).child(
                    Icon::from_spec(IconSpec::new("chevron-down").with_size(IconSize::Sm), theme)
                        .with_color(icon_color),
                ),
            );
        }

        // ── Click + keyboard handler ────────────────────────────
        if let Some(handler) = self.on_click {
            if !is_disabled {
                let handler = std::rc::Rc::new(handler);
                let key_handler = handler.clone();
                el = el
                    .on_click(move |event, window, cx| handler(event, window, cx))
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                            key_handler(&ClickEvent::default(), window, cx);
                        }
                    });
            }
        }

        el.into_any_element()
    }
}
