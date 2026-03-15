//! PugValidationSummary — real GPUI component backed by ValidationSummarySpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::ValidationSummarySpec;
use pug_gpui_primitives::ValidationState;

use crate::theme_ext::resolve_color;

/// A real GPUI validation summary component backed by `ValidationSummarySpec`.
///
/// Renders a list of validation errors/warnings with a tone-colored border,
/// optional title, and individual entry rows showing field label and message.
pub struct PugValidationSummary {
    spec: ValidationSummarySpec,
    theme: GpuiThemeProvider,
    on_entry_click: Option<Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PugValidationSummary {
    pub fn new(spec: ValidationSummarySpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_entry_click: None,
        }
    }

    pub fn on_entry_click(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_entry_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugValidationSummary {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, spec.border_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let danger = resolve_color(theme, "semantic.color.status.danger");
        let warning = resolve_color(theme, "semantic.color.status.warning");

        let active_entries = spec.active_entries();

        let mut container = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .px(px(12.0))
            .py(px(10.0))
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

        // Entry list
        for entry in &active_entries {
            let entry_id = SharedString::from(format!("validation-entry-{}", entry.field_id));

            let indicator_color = match entry.validation_state {
                ValidationState::Invalid => danger,
                ValidationState::Pending => warning,
                _ => text_secondary,
            };

            let mut entry_el = div()
                .id(entry_id)
                .w_full()
                .flex()
                .items_start()
                .gap(px(8.0))
                .py(px(4.0))
                .cursor_pointer();

            // Status indicator dot
            entry_el = entry_el.child(
                div()
                    .size(px(6.0))
                    .rounded(px(3.0))
                    .bg(indicator_color)
                    .mt(px(5.0)),
            );

            // Entry content
            let mut entry_content = div().flex().flex_col().gap(px(1.0)).flex_grow();

            entry_content = entry_content.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(text_primary)
                    .child(entry.label.clone()),
            );

            entry_content = entry_content.child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .child(entry.message.clone()),
            );

            entry_el = entry_el.child(entry_content);

            container = container.child(entry_el);
        }

        // Summary count
        let blocking = spec.blocking_entry_count();
        if blocking > 0 {
            container = container.child(
                div()
                    .pt(px(4.0))
                    .text_xs()
                    .text_color(danger)
                    .child(format!(
                        "{} blocking issue{}",
                        blocking,
                        if blocking == 1 { "" } else { "s" }
                    )),
            );
        }

        container.into_any_element()
    }
}
