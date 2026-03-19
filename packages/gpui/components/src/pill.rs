//! PugPill — real GPUI component backed by PillSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::PillSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// A real GPUI pill component backed by `PillSpec`.
pub struct PugPill {
    spec: PillSpec,
    theme: GpuiThemeProvider,
    on_remove: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugPill {
    pub fn new(spec: PillSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_remove: None,
        }
    }

    pub fn on_remove(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_remove = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugPill {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let text_color = resolve_color(theme, spec.text_color_token());
        let radius = resolve_radius(theme, "semantic.radius.pill");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        let mut el = div()
            .px(px(8.0))
            .py(px(2.0))
            .rounded(radius)
            .bg(fill)
            .text_xs()
            .text_color(text_color)
            .flex()
            .items_center()
            .gap(px(4.0))
            .child(spec.label.clone());

        if spec.is_disabled {
            el = el.opacity(disabled_opacity);
        }

        if spec.is_removable {
            let remove_id = SharedString::from("pug-pill-remove");
            let mut remove_btn = div()
                .id(remove_id)
                .cursor_pointer()
                .text_xs()
                .child("x");

            if let Some(handler) = self.on_remove {
                remove_btn =
                    remove_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }

            el = el.child(remove_btn);
        }

        el.into_any_element()
    }
}
