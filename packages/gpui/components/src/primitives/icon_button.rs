//! IconButton — real GPUI component backed by IconButtonSpec.
//!
//! Contract: `docs/contracts/foundation/icon-button.md`

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ButtonTone, ButtonVariant, ControlSize, IconButtonSpec, IconSize, IconSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use super::icon::Icon;
use super::spinner::Spinner;
use crate::theme_ext::{color_mix, color_mix_black, resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI icon button component backed by `IconButtonSpec`.
pub struct IconButton {
    spec: IconButtonSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    tone: ButtonTone,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for IconButton {
    type Target = IconButtonSpec;
    fn deref(&self) -> &IconButtonSpec { &self.spec }
}

impl IconButton {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: IconButtonSpec::new(), theme: theme.clone(), id_suffix: None, tone: ButtonTone::Default, on_click: None }
    }

    pub fn from_spec(spec: IconButtonSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), id_suffix: None, tone: ButtonTone::Default, on_click: None }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn variant(mut self, v: ButtonVariant) -> Self { self.spec.variant = v; self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn icon(mut self, v: impl Into<String>) -> Self { self.spec.icon = Some(v.into()); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn loading(mut self, v: bool) -> Self { self.spec.is_loading = v; self }
    pub fn pressed(mut self, v: bool) -> Self { self.spec.is_pressed = Some(v); self }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn tone(mut self, tone: ButtonTone) -> Self {
        self.tone = tone;
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
        let tone = self.tone;

        // ── Resolve variant colors ────────────────────────────────
        let base_fill = resolve_color(theme, spec.variant.fill_token(tone));
        let text_color = resolve_color(theme, spec.variant.text_token(tone));
        let base_border = resolve_color(theme, spec.variant.border_token(tone));
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let radius = resolve_radius(theme, "semantic.radius.control");
        let focus_ring_color = resolve_color(theme, "semantic.color.accent.focusRing");

        // ── Size adjustments (contract: sm -0.375rem, lg +0.375rem) ──
        let base_height = resolve_px(theme, spec.control_height_token());
        let size_offset: f32 = match spec.size {
            ControlSize::Sm => -6.0,
            ControlSize::Md => 0.0,
            ControlSize::Lg => 6.0,
        };
        let control_size = base_height + px(size_offset);

        // ── Variant-specific colors ───────────────────────────────
        let (fill, border_color) = match spec.variant {
            ButtonVariant::Primary => {
                let darkened = color_mix_black(base_fill, 0.84);
                (base_fill, darkened)
            }
            ButtonVariant::Ghost => (gpui::transparent_black(), gpui::transparent_black()),
            _ => (base_fill, base_border),
        };

        let is_unavailable = spec.is_disabled || spec.is_loading;
        let is_pressed = spec.is_pressed.unwrap_or(false);

        // ── Hover/active/pressed fills ────────────────────────────
        let hover_fill = color_mix(fill, elevated, 0.84);
        // Contract: hover border = 74% text-primary mix (darkening)
        let hover_border = color_mix(border_color, text_primary, 0.74);
        let active_fill = color_mix(fill, elevated, 0.72);
        // Pressed: accent-tinted background (contract: 20% accent mix)
        let pressed_fill = color_mix(accent, fill, 0.20);

        let icon_name = spec.icon.clone().unwrap_or_default();
        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-icon-btn-{}", suffix)
        } else {
            format!("poodle-icon-btn-{}", icon_name)
        };

        // ── Build root ────────────────────────────────────────────
        // Contract: pressed = double inset shadow
        let pressed_border = color_mix(border_color, text_primary, 0.74);

        let mut el = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w(control_size)
            .h(control_size)
            .rounded(radius)
            .bg(if is_pressed { pressed_fill } else { fill })
            .text_color(text_color)
            .border_1()
            .border_color(if is_pressed { pressed_border } else { border_color })
            .flex()
            .items_center()
            .justify_center();

        // Contract: pressed = double inset shadow (accent 8%, accent 12%)
        if is_pressed {
            el = el.shadow(vec![
                gpui::BoxShadow {
                    color: accent.opacity(0.08),
                    offset: point(px(0.0), px(1.0)),
                    blur_radius: px(2.0),
                    spread_radius: px(0.0),
                },
                gpui::BoxShadow {
                    color: accent.opacity(0.12),
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(1.0),
                },
            ]);
        }

        // ── Focus ring ────────────────────────────────────────────
        el = el.focus(move |s| s.border_color(focus_ring_color).shadow(crate::theme_ext::focus_ring_shadow(focus_ring_color)));

        // ── Interactive states ────────────────────────────────────
        if is_unavailable {
            let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
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
            el = el.child(
                Icon::from_spec(
                    IconSpec::new(&icon_name).with_size(IconSize::Sm),
                    theme,
                )
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
