//! Toggle — pressable button with persistent pressed state.
//!
//! Contract: `docs/contracts/foundation/toggle.md`

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ButtonVariant, ControlSize, ToggleLayout, ToggleSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem};
use crate::theme_ext::{color_mix, color_mix_black, resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct Toggle {
    spec: ToggleSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Toggle {
    type Target = ToggleSpec;
    fn deref(&self) -> &ToggleSpec { &self.spec }
}

impl Toggle {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ToggleSpec::new(), theme: theme.clone(), on_click: None }
    }

    pub fn from_spec(spec: ToggleSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_click: None }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn pressed(mut self, v: bool) -> Self { self.spec.is_pressed = Some(v); self }
    pub fn default_pressed(mut self, v: bool) -> Self { self.spec.default_pressed = v; self }
    pub fn variant(mut self, v: ButtonVariant) -> Self { self.spec.variant = v; self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn layout(mut self, v: ToggleLayout) -> Self { self.spec.layout = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }
    pub fn icon(mut self, v: impl Into<String>) -> Self { self.spec.icon = Some(v.into()); self }
    pub fn size_role(mut self, v: poodle_primitives::SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn density(mut self, v: poodle_primitives::ControlDensity) -> Self { self.spec.density = v; self }

    // ── GPUI-specific ─────────────────────────────────────────
    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Toggle {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let is_pressed = spec.current_pressed();

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size ──────────────────────────────────────────────────
        let base_height = resolve_px(theme, spec.control_height_token());
        let height = base_height + px(rem_to_px(size_height_offset_rem(effective_size)));
        let pad_x = resolve_px(theme, "semantic.space.control.x");
        let font_size = px(rem_to_px(size_font_rem(effective_size)));
        let radius = resolve_radius(theme, spec.radius_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        // ── Colors ────────────────────────────────────────────────
        let accent = resolve_color(theme, "semantic.color.accent.base");
        let surface = resolve_color(theme, "semantic.color.background.surface");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let border_default = resolve_color(theme, "semantic.color.border.default");

        let (fill, text_color, border_color) = if is_pressed {
            // Pressed: accent bg, accent border (darkened), inverse text
            let pressed_border = color_mix_black(accent, 0.78);
            (accent, text_inverse, pressed_border)
        } else {
            // Unpressed: variant-specific
            match spec.variant {
                ButtonVariant::Ghost => (gpui::transparent_black(), text_primary, gpui::transparent_black()),
                ButtonVariant::Primary => {
                    let darkened = color_mix_black(accent, 0.84);
                    (accent, text_inverse, darkened)
                }
                _ => (surface, text_primary, border_default),
            }
        };

        let hover_fill = color_mix(fill, elevated, 0.84);

        // ── Build ─────────────────────────────────────────────────
        let label_text = spec.label.clone().unwrap_or_default();
        let id = SharedString::from(format!("poodle-toggle-{}", label_text));

        let is_ghost = !is_pressed && matches!(spec.variant, ButtonVariant::Ghost);

        let mut el = div()
            .id(id)
            .focusable()
            .h(height)
            .min_w(px(36.0)) // contract: 2.25rem
            .px(pad_x)
            .rounded(radius);

        // Brand-raised treatment: gradient fills and elevated shadows
        if theme.brand_raised && !is_ghost && !spec.is_disabled {
            use crate::theme_ext::{
                brand_raised_primary_fill, brand_raised_interactive_fill,
                brand_raised_primary_shadow, brand_raised_interactive_shadow,
            };
            if is_pressed || matches!(spec.variant, ButtonVariant::Primary) {
                el = el.bg(brand_raised_primary_fill(fill))
                    .shadow(brand_raised_primary_shadow());
            } else {
                el = el.bg(brand_raised_interactive_fill(fill))
                    .shadow(brand_raised_interactive_shadow());
            }
        } else {
            el = el.bg(fill);
        }

        el = el.border_1().border_color(border_color)
            .text_color(text_color)
            .text_size(font_size)
            .font_weight(FontWeight::SEMIBOLD)
            .line_height(relative(1.0))
            .flex().items_center().justify_center()
            .focus(move |s| s.border_color(focus_ring).shadow(crate::theme_ext::focus_ring_shadow(focus_ring)));

        // Stack layout = full width
        if spec.layout == ToggleLayout::Stack {
            el = el.w_full();
        }

        if spec.is_disabled {
            let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
            el = el.opacity(opacity).cursor(CursorStyle::OperationNotAllowed);
        } else {
            el = el.cursor_pointer().hover(move |s| s.bg(hover_fill));
            if let Some(handler) = self.on_click {
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

        // Icon (rendered before label if both present)
        if let Some(ref icon_name) = spec.icon {
            use super::icon::Icon;
            use poodle_primitives::{IconSize, IconSpec};
            let icon_size = match effective_size {
                ControlSize::Xs | ControlSize::Sm => IconSize::Sm,
                _ => IconSize::Sm,
            };
            el = el.child(
                Icon::from_spec(
                    IconSpec::new(icon_name).with_size(icon_size),
                    theme,
                ).with_color(text_color)
            );
            // Add gap between icon and label
            if !label_text.is_empty() {
                el = el.gap(px(4.0));
            }
        }

        if !label_text.is_empty() {
            el = el.child(div().whitespace_nowrap().child(label_text));
        }

        el.into_any_element()
    }
}
