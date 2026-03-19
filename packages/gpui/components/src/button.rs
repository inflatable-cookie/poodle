//! PugButton — real GPUI component backed by ButtonSpec.
//!
//! Implements the button contract (`docs/contracts/foundation/button.md`) exactly,
//! resolving all tokens through ButtonSpec + GpuiThemeProvider, and matching the
//! Svelte implementation's visual output.

use std::time::Duration;

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{ButtonSpec, ButtonTone, ButtonVariant, IconSize, IconSpec};

use crate::icon::PugIcon;
use crate::theme_ext::{color_mix, color_mix_black, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI button component backed by `ButtonSpec`.
pub struct PugButton {
    spec: ButtonSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugButton {
    pub fn new(spec: ButtonSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_click: None,
        }
    }

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

impl IntoElement for PugButton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve base tokens ──────────────────────────────────
        let base_fill = resolve_color(theme, spec.resolved_fill_token());
        let base_text = resolve_color(theme, spec.resolved_text_token());
        let base_border = resolve_color(theme, spec.resolved_border_token());
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let radius = resolve_radius(theme, spec.radius_token());

        // ── Size-aware layout values (contract §7, §8) ──────────
        let base_height = resolve_px(theme, spec.control_height_token());
        let height = base_height + px(spec.height_offset_px());
        let min_width = px(spec.min_width_px());
        let base_pad_x = resolve_px(theme, spec.horizontal_padding_token());
        let pad_x = base_pad_x + px(spec.padding_x_offset_px());
        let font_size = px(spec.font_size_px());
        let gap = px(6.0); // contract: 0.375rem = 6px

        // Icon padding adjustment (contract §8): reduce padding on icon side by 2px
        let has_leading = spec.leading_icon.is_some() || spec.is_loading;
        let has_trailing = spec.trailing_icon.is_some() || spec.chevron;
        let icon_inset = px(2.0);
        let pad_left = if has_leading { pad_x - icon_inset } else { pad_x };
        let pad_right = if has_trailing { pad_x - icon_inset } else { pad_x };

        let is_disabled = spec.is_disabled || spec.is_loading;
        let is_ghost = spec.variant == ButtonVariant::Ghost;

        // ── Compute variant-specific colors ──────────────────────

        // Danger×secondary needs color-mix blends (contract §8 Tone: danger)
        let (mut fill, mut border_color, mut text_color) = match (spec.variant, spec.tone) {
            (ButtonVariant::Secondary, ButtonTone::Danger) => {
                // fill: color-mix(status-danger 16%, background-surface)
                let danger = resolve_color(theme, "semantic.color.status.danger");
                let surface = resolve_color(theme, "semantic.color.background.surface");
                let border_default = resolve_color(theme, "semantic.color.border.default");
                let fill = color_mix(danger, surface, 0.16);
                // border: color-mix(status-danger 46%, border-default)
                let border = color_mix(danger, border_default, 0.46);
                (fill, border, base_text)
            }
            (ButtonVariant::Primary, _) => {
                // Primary border: color-mix(accent-base 84%, black)
                let darkened_border = color_mix_black(base_fill, 0.84);
                (base_fill, darkened_border, base_text)
            }
            _ => (base_fill, base_border, base_text),
        };

        // Ghost: transparent fill and border (contract §8 CSS Custom Properties)
        let (mut fill, mut border_color) = if is_ghost {
            (gpui::transparent_black(), gpui::transparent_black())
        } else {
            (fill, border_color)
        };

        // ── Hover/active colors (contract §8 Hover/Active) ──────
        // hover fill: color-mix(fill 84%, elevated)
        // active fill: color-mix(fill 72%, elevated)
        let hover_fill = color_mix(fill, elevated, 0.84);
        let active_fill = color_mix(fill, elevated, 0.72);
        // hover border: color-mix(border 78%, text-primary)
        // For ghost, base border is transparent — CSS color-mix(transparent 78%, text-primary)
        // produces ~22% text-primary. We replicate by mixing the resolved border-default
        // token with text-primary so the ghost gets a visible border on hover.
        let hover_border = if is_ghost {
            let border_default = resolve_color(theme, "semantic.color.border.default");
            color_mix(border_default, text_primary, 0.78)
        } else {
            color_mix(border_color, text_primary, 0.78)
        };

        // ── Build element ID ─────────────────────────────────────
        let label_text = spec.label.clone().unwrap_or_default();
        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-btn-{}", suffix)
        } else {
            format!("pug-btn-{}", label_text)
        };

        // ── Build root element (contract §8 Root) ────────────────
        let mut el = div()
            .id(SharedString::from(id_str))
            .h(height)
            .min_w(min_width)
            .pl(pad_left)
            .pr(pad_right)
            .rounded(radius)
            .bg(fill)
            .border_1()
            .border_color(border_color)
            .text_color(text_color)
            .flex()
            .items_center()
            .justify_center()
            .gap(gap)
            .text_size(font_size)
            .font_weight(FontWeight::MEDIUM); // contract: typography-label-weight = 500

        // ── Interactive states ────────────────────────────────────
        let icon_color = text_color;
        if is_disabled {
            // contract §8 Disabled: opacity: state-opacity-disabled, cursor: not-allowed
            let dis_o = resolve_opacity(theme, spec.disabled_opacity_token());
            el = el.opacity(dis_o);
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| s.bg(hover_fill).border_color(hover_border))
                .active(move |s| s.bg(active_fill));
        }

        // ── Spinner (contract §8 Spinner) ────────────────────────
        // Contract: 0.75rem (12px) border spinner, rotating 360° in 0.8s.
        // We use a custom spinner SVG with GPUI's animation API to rotate it.
        if spec.is_loading {
            let spinner_size = px(12.0); // 0.75rem
            el = el.child(
                svg()
                    .path(SharedString::from("assets/icons/spinner.svg"))
                    .size(spinner_size)
                    .flex_shrink_0()
                    .text_color(icon_color)
                    .with_animation(
                        "button-spinner",
                        Animation::new(Duration::from_millis(800))
                            .repeat(),
                        |svg, delta| {
                            svg.with_transformation(
                                Transformation::rotate(gpui::radians(delta * std::f32::consts::TAU))
                            )
                        },
                    ),
            );
        }

        // ── Leading icon ─────────────────────────────────────────
        if let Some(ref icon_name) = spec.leading_icon {
            el = el.child(
                PugIcon::new(
                    IconSpec::new(icon_name.clone()).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_color),
            );
        }

        // ── Label ────────────────────────────────────────────────
        if !label_text.is_empty() {
            el = el.child(label_text);
        }

        // ── Trailing icon ────────────────────────────────────────
        if let Some(ref icon_name) = spec.trailing_icon {
            el = el.child(
                PugIcon::new(
                    IconSpec::new(icon_name.clone()).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(icon_color),
            );
        }

        // ── Chevron (contract §8 Chevron) ────────────────────────
        // opacity: 0.5, margin-left: -2px to tighten from gap
        if spec.chevron {
            el = el.child(
                div()
                    .flex()
                    .items_center()
                    .opacity(0.5)
                    .ml(px(-2.0))
                    .child(
                        PugIcon::new(
                            IconSpec::new("chevron-down").with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(icon_color),
                    ),
            );
        }

        // ── Click handler ────────────────────────────────────────
        if let Some(handler) = self.on_click {
            if !is_disabled {
                el = el.on_click(move |event, window, cx| handler(event, window, cx));
            }
        }

        el.into_any_element()
    }
}
