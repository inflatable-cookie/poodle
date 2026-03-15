//! PugSplitButton — real GPUI component backed by SplitButtonSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SplitButtonSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI split button component backed by `SplitButtonSpec`.
pub struct PugSplitButton {
    spec: SplitButtonSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_dropdown: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugSplitButton {
    pub fn new(spec: SplitButtonSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_click: None,
            on_dropdown: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn on_dropdown(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dropdown = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugSplitButton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let separator = resolve_color(theme, spec.separator_token());
        let radius = resolve_radius(theme, "semantic.radius.control");
        let height = resolve_px(theme, "semantic.size.control-height");

        let label = spec.label.clone().unwrap_or_default();
        let main_id = SharedString::from("pug-split-btn-main");
        let dropdown_id = SharedString::from("pug-split-btn-dropdown");

        // Main action button
        let mut main_btn = div()
            .id(main_id)
            .h(height)
            .px(px(12.0))
            .bg(fill)
            .rounded_l(radius)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_center()
            .text_sm()
            .child(label);

        if !spec.is_disabled {
            main_btn = main_btn.cursor_pointer();
            if let Some(handler) = self.on_click {
                main_btn = main_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }
        }

        // Separator
        let sep = div()
            .w(px(1.0))
            .h(height)
            .bg(separator);

        // Dropdown trigger
        let mut dropdown_btn = div()
            .id(dropdown_id)
            .h(height)
            .px(px(6.0))
            .bg(fill)
            .rounded_r(radius)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_center()
            .text_xs()
            .child("v");

        if !spec.is_disabled {
            dropdown_btn = dropdown_btn.cursor_pointer();
            if let Some(handler) = self.on_dropdown {
                dropdown_btn =
                    dropdown_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }
        }

        let mut el = div()
            .flex()
            .items_center()
            .child(main_btn)
            .child(sep)
            .child(dropdown_btn);

        if spec.is_disabled {
            el = el.opacity(0.5);
        }

        el.into_any_element()
    }
}
