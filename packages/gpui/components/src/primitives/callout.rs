//! Callout — real GPUI component backed by CallOutSpec (contract: callout).

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CallOutSpec, IconSize, IconSpec, StatusTone};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI call-out component backed by `CallOutSpec`.
pub struct Callout {
    spec: CallOutSpec,
    theme: GpuiThemeProvider,
    is_dismissible: bool,
    on_dismiss: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Callout {
    type Target = CallOutSpec;
    fn deref(&self) -> &CallOutSpec { &self.spec }
}

impl Callout {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CallOutSpec::new(),
            theme: theme.clone(),
            is_dismissible: false,
            on_dismiss: None,
        }
    }

    pub fn from_spec(spec: CallOutSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            is_dismissible: false,
            on_dismiss: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tone(mut self, v: StatusTone) -> Self { self.spec.tone = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn content(mut self, v: impl Into<String>) -> Self { self.spec.content = Some(v.into()); self }

    /// Alias for `content` — some contracts call it `message`.
    pub fn message(mut self, v: impl Into<String>) -> Self { self.spec.content = Some(v.into()); self }

    pub fn dismissible(mut self, v: bool) -> Self { self.is_dismissible = v; self }

    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Callout {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let panel_x = resolve_px(theme, "semantic.space.panel.x");
        let panel_y = resolve_px(theme, "semantic.space.panel.y");

        let fill = resolve_color(theme, spec.fill_token());
        let border = resolve_color(theme, spec.border_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let radius = resolve_radius(theme, "semantic.radius.surface");

        // Contract: tone icon name
        let icon_name = match spec.tone {
            StatusTone::Info => "info",
            StatusTone::Success => "check-circle",
            StatusTone::Warning => "alert-triangle",
            StatusTone::Danger => "alert-circle",
            _ => "info",
        };

        // Contract: full border, tone-colored bg at 10% opacity, grid layout with icon
        let mut el = div()
            .w_full()
            .px(panel_x)
            .py(panel_y)
            .rounded(radius)
            .bg(fill.opacity(0.1))
            .border_1()
            .border_color(border)
            .flex()
            .gap(px(12.0)); // 0.75rem gap between icon and content

        // Icon column — Contract: tone-colored status icon
        el = el.child(
            div()
                .flex_shrink_0()
                .pt(px(2.0))
                .child(
                    Icon::from_spec(
                        IconSpec::new(icon_name).with_size(IconSize::Sm),
                        theme,
                    )
                    .with_color(border),
                ),
        );

        // Content column
        let mut content_col = div().flex().flex_col().gap(px(4.0)).flex_1().min_w(px(0.0));

        // Contract: title font 0.875rem (14px), weight 600
        if let Some(ref title) = spec.title {
            content_col = content_col.child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Contract: content font 0.875rem (14px)
        if let Some(ref content) = spec.content {
            content_col = content_col.child(
                div()
                    .text_size(px(14.0))
                    .text_color(text_secondary)
                    .child(content.clone()),
            );
        }

        el = el.child(content_col);

        // Dismiss button column — shown when dismissible
        if self.is_dismissible {
            let dismiss_id = SharedString::from("pug-callout-dismiss");
            let mut dismiss_btn = div()
                .id(dismiss_id)
                .flex_shrink_0()
                .cursor_pointer()
                .pt(px(2.0))
                .child(
                    Icon::from_spec(
                        IconSpec::new("x").with_size(IconSize::Sm),
                        theme,
                    )
                    .with_color(text_secondary),
                );

            if let Some(handler) = self.on_dismiss {
                dismiss_btn =
                    dismiss_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }

            el = el.child(dismiss_btn);
        }

        el.into_any_element()
    }
}
