//! IconButton — real GPUI component backed by IconButtonSpec.
//!
//! Contract: `docs/contracts/components/icon-button.md`

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonTone, ButtonVariant, ControlSize, IconButtonSpec, IconSize, IconSpec, SpinnerSize,
    SpinnerSpec, SpinnerTone, SpinnerVariant,
};

use super::icon::Icon;
use super::spinner::Spinner;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, resolve_supporting_visual_size, size_height_offset_rem,
};
use crate::theme_ext::{
    color_mix, color_mix_black, resolve_color, resolve_opacity, resolve_px, resolve_radius,
};

/// A real GPUI icon button component backed by `IconButtonSpec`.
pub struct IconButton {
    spec: IconButtonSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for IconButton {
    type Target = IconButtonSpec;
    fn deref(&self) -> &IconButtonSpec {
        &self.spec
    }
}

impl IconButton {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: IconButtonSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_click: None,
        }
    }

    pub fn from_spec(spec: IconButtonSpec, theme: &GpuiThemeProvider) -> Self {
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
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn icon(mut self, v: impl Into<String>) -> Self {
        self.spec.icon = Some(v.into());
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
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
    pub fn pressed(mut self, v: bool) -> Self {
        self.spec.is_pressed = Some(v);
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

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn tone(mut self, tone: ButtonTone) -> Self {
        self.spec.tone = tone;
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.spec.tooltip = Some(tooltip.into());
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

impl IntoElement for IconButton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let tone = self.spec.tone;

        // ── Resolve variant colors ────────────────────────────────
        let base_fill = resolve_color(theme, spec.variant.fill_token(tone));
        let text_color = resolve_color(theme, spec.variant.text_token(tone));
        let base_border = resolve_color(theme, spec.variant.border_token(tone));
        let text_primary = resolve_color(theme, "color.text.primary");
        let elevated = resolve_color(theme, "color.background.elevated");
        let accent = resolve_color(theme, "color.accent.base");
        let radius = resolve_radius(theme, "radius.control");
        let focus_ring_color = resolve_color(theme, "color.accent.focusRing");

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size adjustments (contract: sm -0.375rem, lg +0.375rem) ──
        let base_height = resolve_px(theme, spec.control_height_token());
        let control_size = base_height + px(rem_to_px(size_height_offset_rem(effective_size)));

        // ── Variant-specific colors ───────────────────────────────
        let (fill, border_color) = match spec.variant {
            ButtonVariant::Primary => {
                let darkened = color_mix_black(base_fill, 0.84);
                (base_fill, darkened)
            }
            ButtonVariant::Ghost => (gpui::transparent_black(), gpui::transparent_black()),
            // Secondary danger/success: color-mix(status 16%, surface) fill and
            // color-mix(status 46%, border-default) border (icon-button.md §8
            // Tone: danger / Tone: success). Default secondary uses token values.
            ButtonVariant::Secondary => match tone {
                ButtonTone::Danger | ButtonTone::Success => {
                    let status = if tone == ButtonTone::Success {
                        resolve_color(theme, "color.status.success")
                    } else {
                        resolve_color(theme, "color.status.danger")
                    };
                    let surface = resolve_color(theme, "color.background.surface");
                    let border_default = resolve_color(theme, "color.border.default");
                    (
                        color_mix(status, surface, 0.16),
                        color_mix(status, border_default, 0.46),
                    )
                }
                ButtonTone::Default => (base_fill, base_border),
            },
            _ => (base_fill, base_border),
        };

        let is_unavailable = spec.is_disabled || spec.is_loading;
        let is_pressed = spec.is_pressed.unwrap_or(false);
        let is_primary = matches!(spec.variant, ButtonVariant::Primary);

        // ── Pressed treatment (contract §8 "Root — Pressed") ──────
        // Non-primary variants get a solid-accent treatment: fill accent-base,
        // border accent-base 85% black, inverse text, shadow none. Primary keeps
        // its own variant styling when pressed.
        let pressed_active = is_pressed && !is_primary;
        let text_inverse = resolve_color(theme, "color.text.inverse");
        let (current_fill, current_border, text_color) = if pressed_active {
            (accent, color_mix_black(accent, 0.85), text_inverse)
        } else {
            (fill, border_color, text_color)
        };

        // ── Hover/active fills ────────────────────────────────────
        // Svelte: hover = color-mix(fill 76%, elevated), active = color-mix(fill 64%, elevated).
        // Pressed hover is `color-mix(white 12%, accent-base)`.
        let hover_fill = if pressed_active {
            color_mix(gpui::white(), accent, 0.12)
        } else {
            color_mix(current_fill, elevated, 0.76)
        };
        // Contract: hover border = 74% border mixed toward text-primary.
        let hover_border = color_mix(current_border, text_primary, 0.74);
        let active_fill = color_mix(current_fill, elevated, 0.64);

        let icon_name = spec.icon.clone().unwrap_or_default();
        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-icon-btn-{}", suffix)
        } else {
            format!("poodle-icon-btn-{}", icon_name)
        };

        let is_ghost = matches!(spec.variant, ButtonVariant::Ghost);

        let mut el = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w(control_size)
            .h(control_size)
            .rounded(radius);

        // Brand-raised treatment: gradient fills and elevated shadows
        if theme.brand_raised && !is_ghost && !is_unavailable && !pressed_active {
            use crate::theme_ext::{
                brand_raised_interactive_fill, brand_raised_interactive_shadow,
                brand_raised_primary_fill, brand_raised_primary_shadow,
            };
            match spec.variant {
                ButtonVariant::Primary => {
                    el = el
                        .bg(brand_raised_primary_fill(current_fill))
                        .shadow(brand_raised_primary_shadow());
                }
                _ => {
                    el = el
                        .bg(brand_raised_interactive_fill(current_fill))
                        .shadow(brand_raised_interactive_shadow());
                }
            }
        } else {
            el = el.bg(current_fill);
        }

        el = el
            .text_color(text_color)
            .border_1()
            .border_color(current_border)
            .flex()
            .items_center()
            .justify_center();

        // Contract §8 pressed: shadow is `none` (no inset stack).

        // ── Focus ring ────────────────────────────────────────────
        el = el.focus(move |s| {
            s.border_color(focus_ring_color)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring_color))
        });

        // ── Interactive states ────────────────────────────────────
        if is_unavailable {
            let opacity = resolve_opacity(theme, "state.opacity.disabled");
            el = el.opacity(opacity).cursor(CursorStyle::OperationNotAllowed);
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| s.bg(hover_fill).border_color(hover_border))
                .active(move |s| s.bg(active_fill));
        }

        // ── Icon / spinner ────────────────────────────────────────
        if spec.is_loading {
            el = el.child(
                Spinner::from_spec(
                    SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_size(SpinnerSize::Sm)
                        .with_tone(SpinnerTone::Current),
                    theme,
                )
                .with_color(text_color),
            );
        } else if !icon_name.is_empty() {
            // Contract §13: glyph size resolves through the supporting-size
            // mapping (one stop smaller than the control), not a fixed size.
            let glyph_size = IconSize::from(resolve_supporting_visual_size(effective_size));
            el = el.child(
                Icon::from_spec(IconSpec::new(&icon_name).with_size(glyph_size), theme)
                    .with_color(text_color),
            );
        }

        // ── Click + keyboard handler ─────────────────────────────
        if let Some(handler) = self.on_click {
            if !is_unavailable {
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
