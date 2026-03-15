//! PugBanner — real GPUI component backed by BannerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::BannerSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

/// A real GPUI banner component backed by `BannerSpec`.
pub struct PugBanner {
    spec: BannerSpec,
    theme: GpuiThemeProvider,
    on_dismiss: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugBanner {
    pub fn new(spec: BannerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_dismiss: None,
        }
    }

    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugBanner {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.surface");

        let mut el = div()
            .w_full()
            .px(px(12.0))
            .py(px(10.0))
            .rounded(radius)
            .bg(fill.opacity(0.12))
            .border_l(px(3.0))
            .border_color(border)
            .flex()
            .items_start()
            .gap(px(8.0));

        // Icon area
        if spec.has_icon {
            let icon_color = resolve_color(theme, spec.icon_color_token());
            el = el.child(
                div()
                    .text_sm()
                    .text_color(icon_color)
                    .child("!"),
            );
        }

        // Content area
        let mut content = div().flex().flex_col().gap(px(2.0)).flex_grow();

        if let Some(ref title) = spec.title {
            content = content.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(title.clone()),
            );
        }

        if let Some(ref message) = spec.message {
            content = content.child(
                div()
                    .text_sm()
                    .child(message.clone()),
            );
        }

        el = el.child(content);

        // Dismiss button
        if spec.is_dismissible {
            let dismiss_id = SharedString::from("pug-banner-dismiss");
            let mut dismiss_btn = div()
                .id(dismiss_id)
                .cursor_pointer()
                .text_xs()
                .child("x");

            if let Some(handler) = self.on_dismiss {
                dismiss_btn =
                    dismiss_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }

            el = el.child(dismiss_btn);
        }

        el.into_any_element()
    }
}
