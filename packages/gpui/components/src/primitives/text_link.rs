use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::TextLinkSpec;

use crate::theme_ext::{focus_ring_shadow, resolve_color, resolve_opacity};

pub struct TextLink {
    spec: TextLinkSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TextLink {
    pub fn new(label: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(TextLinkSpec::new(label), theme)
    }

    pub fn from_spec(spec: TextLinkSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TextLink {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let color = resolve_color(&self.theme, self.spec.color_token());
        let disabled_opacity = resolve_opacity(&self.theme, "state.opacity.disabled");
        let focus_ring = resolve_color(&self.theme, "color.accent.focusRing");

        let mut el = div()
            .id(SharedString::from(format!(
                "poodle-text-link-{}",
                self.spec.label
            )))
            .cursor_pointer()
            .text_color(color)
            .underline()
            .focus(move |style| style.shadow(focus_ring_shadow(focus_ring)))
            .when(self.spec.disabled, |el| {
                el.opacity(disabled_opacity).cursor_default()
            })
            .child(self.spec.label.clone());

        if !self.spec.disabled {
            if let Some(handler) = self.on_click {
                el = el.on_click(move |event, window, app| handler(event, window, app));
            }
        }

        el.into_any_element()
    }
}
