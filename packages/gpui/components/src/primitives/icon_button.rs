//! IconButton — real GPUI component backed by IconButtonSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{ButtonTone, ButtonVariant, ControlSize, IconButtonSpec, IconSize, IconSpec};

use super::icon::Icon;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

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
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            tone: ButtonTone::Default,
            on_click: None,
        }
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

    pub fn with_tone(mut self, tone: ButtonTone) -> Self {
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

        let fill = resolve_color(theme, spec.variant.fill_token(tone));
        let text_color = resolve_color(theme, spec.variant.text_token(tone));
        let border_color = resolve_color(theme, spec.variant.border_token(tone));
        let control_height = resolve_px(theme, spec.control_height_token());
        let radius = resolve_radius(theme, "semantic.radius.control");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");

        let is_disabled = spec.is_disabled;
        let is_loading = spec.is_loading;
        let is_pressed = spec.is_pressed.unwrap_or(false);

        let icon_name = spec.icon.clone().unwrap_or_default();
        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-icon-btn-{}", suffix)
        } else {
            format!("pug-icon-btn-{}", icon_name)
        };

        // Contract: hover = color-mix(fill 84%, elevated), active = 72%
        let hover_fill = color_mix(fill, elevated, 0.84);
        let active_fill = color_mix(fill, elevated, 0.72);

        let mut el = div()
            .id(SharedString::from(id_str))
            .w(control_height)
            .h(control_height)
            .rounded(radius)
            .bg(fill)
            .text_color(text_color)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center();

        if is_pressed {
            el = el.bg(active_fill);
        }

        if is_disabled || is_loading {
            let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
            el = el.opacity(opacity);
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| s.bg(hover_fill))
                .active(move |s| s.bg(active_fill));
        }

        // Render icon via Icon (SVG) or spinner when loading
        if is_loading {
            if !icon_name.is_empty() {
                el = el.child(
                    Icon::from_spec(
                        IconSpec::new("loader").with_size(IconSize::Sm),
                        theme,
                    )
                    .with_color(text_color),
                );
            }
        } else if !icon_name.is_empty() {
            el = el.child(
                Icon::from_spec(
                    IconSpec::new(&icon_name).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(text_color),
            );
        }

        // Click handler
        if let Some(handler) = self.on_click {
            if spec.activation_allowed() {
                el = el.on_click(move |event, window, cx| handler(event, window, cx));
            }
        }

        el.into_any_element()
    }
}
