//! PugInlineRemediation — real GPUI component backed by InlineRemediationSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::InlineRemediationSpec;

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI inline remediation component backed by `InlineRemediationSpec`.
///
/// Renders an inline validation message with a tone-colored left border,
/// optional title, message text, and an optional action button.
pub struct PugInlineRemediation {
    spec: InlineRemediationSpec,
    theme: GpuiThemeProvider,
    on_action: Option<Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugInlineRemediation {
    pub fn new(spec: InlineRemediationSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_action: None,
        }
    }

    pub fn on_action(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_action = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugInlineRemediation {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, spec.border_token());
        let gap = resolve_px(theme, spec.gap_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        let mut container = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(gap)
            .px(px(12.0))
            .py(px(8.0))
            .border_l(px(3.0))
            .border_color(border)
            .bg(border.opacity(0.06))
            .rounded_r(px(4.0));

        // Title
        if let Some(ref title) = spec.title {
            container = container.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        // Message
        container = container.child(
            div()
                .text_sm()
                .text_color(text_secondary)
                .child(spec.message.clone()),
        );

        // Action button
        if let Some(ref action) = spec.action {
            let action_id = SharedString::from(format!("inline-remediation-{}", action.id));
            let btn = div()
                .id(action_id)
                .cursor_pointer()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(accent)
                .when(action.is_disabled, |el| el.opacity(0.5))
                .child(action.label.clone());

            container = container.child(btn);
        }

        container.into_any_element()
    }
}
