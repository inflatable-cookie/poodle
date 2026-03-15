//! PugToggle — a pressed-state button variant.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;

use crate::theme_ext::{resolve_color, resolve_radius};

/// A toggle button that renders with a pressed/unpressed visual state.
pub struct PugToggle {
    theme: GpuiThemeProvider,
    label: String,
    is_pressed: bool,
    is_disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugToggle {
    pub fn new(label: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            label: label.into(),
            is_pressed: false,
            is_disabled: false,
            on_click: None,
        }
    }

    pub fn with_pressed(mut self, is_pressed: bool) -> Self {
        self.is_pressed = is_pressed;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
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

impl IntoElement for PugToggle {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let (fill, text_color) = if self.is_pressed {
            (
                resolve_color(theme, "semantic.color.accent.base"),
                resolve_color(theme, "semantic.color.text.inverse"),
            )
        } else {
            (
                resolve_color(theme, "semantic.color.background.surface"),
                resolve_color(theme, "semantic.color.text.primary"),
            )
        };
        let border = resolve_color(theme, "semantic.color.border.default");
        let radius = resolve_radius(theme, "semantic.radius.control");

        let id = SharedString::from(format!("pug-toggle-{}", self.label));

        let mut el = div()
            .id(id)
            .px(px(10.0))
            .py(px(4.0))
            .rounded(radius)
            .bg(fill)
            .border_1()
            .border_color(border)
            .text_sm()
            .text_color(text_color)
            .flex()
            .items_center()
            .justify_center()
            .child(self.label);

        if self.is_disabled {
            el = el.opacity(0.5);
        } else {
            el = el.cursor_pointer();
            if let Some(handler) = self.on_click {
                el = el.on_click(move |event, window, cx| handler(event, window, cx));
            }
        }

        el.into_any_element()
    }
}
