//! PugIconButton — real GPUI component backed by IconButtonSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::IconButtonSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI icon button component backed by `IconButtonSpec`.
pub struct PugIconButton {
    spec: IconButtonSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugIconButton {
    pub fn new(spec: IconButtonSpec, theme: &GpuiThemeProvider) -> Self {
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

impl IntoElement for PugIconButton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.variant.fill_token(pug_gpui_primitives::ButtonTone::Default));
        let text_color = resolve_color(theme, spec.variant.text_token(pug_gpui_primitives::ButtonTone::Default));
        let border_color = resolve_color(theme, spec.variant.border_token(pug_gpui_primitives::ButtonTone::Default));
        let control_height = resolve_px(theme, spec.control_height_token());
        let radius = resolve_radius(theme, "semantic.radius.control");

        let is_disabled = spec.is_disabled;
        let is_pressed = spec.is_pressed.unwrap_or(false);

        let icon_text = spec.icon.clone().unwrap_or_default();
        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-icon-btn-{}", suffix)
        } else {
            format!("pug-icon-btn-{}", icon_text)
        };

        let hover_fill = fill.opacity(0.85);
        let active_fill = fill.opacity(0.7);

        let mut el = div()
            .id(SharedString::from(id_str))
            .w(control_height)
            .h(control_height)
            .rounded(radius)
            .bg(if is_pressed { fill.opacity(0.15) } else { fill })
            .text_color(text_color)
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .text_sm();

        if is_disabled {
            let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
            el = el.opacity(opacity);
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| s.bg(hover_fill))
                .active(move |s| s.bg(active_fill));
        }

        // Icon placeholder (text representation)
        if !icon_text.is_empty() {
            el = el.child(icon_text);
        }

        // Click handler
        if let Some(handler) = self.on_click {
            if !is_disabled {
                el = el.on_click(move |event, window, cx| handler(event, window, cx));
            }
        }

        el.into_any_element()
    }
}
