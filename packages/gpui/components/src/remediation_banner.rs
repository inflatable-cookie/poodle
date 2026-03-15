//! PugRemediationBanner — real GPUI component backed by RemediationBannerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::RemediationBannerSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI remediation banner component backed by `RemediationBannerSpec`.
///
/// Renders a banner with tone-colored left border, title, message, optional
/// primary/secondary action buttons, and an optional dismiss button.
pub struct PugRemediationBanner {
    spec: RemediationBannerSpec,
    theme: GpuiThemeProvider,
    on_primary: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_secondary: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_dismiss: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugRemediationBanner {
    pub fn new(spec: RemediationBannerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_primary: None,
            on_secondary: None,
            on_dismiss: None,
        }
    }

    pub fn on_primary(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_primary = Some(Box::new(handler));
        self
    }

    pub fn on_secondary(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_secondary = Some(Box::new(handler));
        self
    }

    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugRemediationBanner {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, spec.border_token());
        let bg = resolve_color(theme, spec.background_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let mut banner = div()
            .w_full()
            .flex()
            .items_start()
            .gap(px(12.0))
            .px(px(14.0))
            .py(px(12.0))
            .bg(bg)
            .border_l(px(3.0))
            .border_color(border)
            .rounded_r(px(6.0));

        // Content
        let mut content = div().flex().flex_col().gap(px(4.0)).flex_grow();

        // Title
        content = content.child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(spec.title.clone()),
        );

        // Message
        content = content.child(
            div()
                .text_sm()
                .text_color(text_secondary)
                .child(spec.message.clone()),
        );

        // Actions row
        if spec.action_count() > 0 {
            let mut actions = div().flex().gap(px(8.0)).pt(px(6.0));

            if let Some(ref primary_action) = spec.primary_action {
                let is_primary =
                    primary_action.variant == pug_gpui_primitives::ButtonVariant::Primary;
                let action_id =
                    SharedString::from(format!("remediation-primary-{}", primary_action.id));

                let mut btn = div()
                    .id(action_id)
                    .cursor_pointer()
                    .px(px(12.0))
                    .py(px(6.0))
                    .rounded(px(4.0))
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .when(is_primary, |el| {
                        el.bg(accent).text_color(gpui::white())
                    })
                    .when(!is_primary, |el| {
                        el.border_1()
                            .border_color(accent)
                            .text_color(accent)
                    })
                    .when(primary_action.is_disabled, |el| el.opacity(0.5))
                    .child(primary_action.label.clone());

                if let Some(handler) = self.on_primary {
                    btn = btn.on_click(move |event, window, cx| handler(event, window, cx));
                }

                actions = actions.child(btn);
            }

            if let Some(ref secondary_action) = spec.secondary_action {
                let action_id = SharedString::from(format!(
                    "remediation-secondary-{}",
                    secondary_action.id
                ));

                let mut btn = div()
                    .id(action_id)
                    .cursor_pointer()
                    .px(px(12.0))
                    .py(px(6.0))
                    .rounded(px(4.0))
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(accent)
                    .when(secondary_action.is_disabled, |el| el.opacity(0.5))
                    .child(secondary_action.label.clone());

                if let Some(handler) = self.on_secondary {
                    btn = btn.on_click(move |event, window, cx| handler(event, window, cx));
                }

                actions = actions.child(btn);
            }

            content = content.child(actions);
        }

        banner = banner.child(content);

        // Dismiss button
        if spec.is_dismissible {
            let dismiss_id = SharedString::from("remediation-banner-dismiss");
            let mut dismiss_btn = div()
                .id(dismiss_id)
                .cursor_pointer()
                .text_xs()
                .text_color(text_secondary)
                .child("\u{2715}");

            if let Some(handler) = self.on_dismiss {
                dismiss_btn =
                    dismiss_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }

            banner = banner.child(dismiss_btn);
        }

        banner.into_any_element()
    }
}
