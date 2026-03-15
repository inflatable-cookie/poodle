//! PugFormShell — real GPUI component backed by FormShellSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::FormShellSpec;
use pug_gpui_primitives::{FormActionAlign, ValidationState};

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI form shell component backed by `FormShellSpec`.
///
/// Renders a form container with title, description, sections (with field labels),
/// validation state indicators, status summary, and an action row.
pub struct PugFormShell {
    spec: FormShellSpec,
    theme: GpuiThemeProvider,
    fields_slot: Option<AnyElement>,
    actions_slot: Option<AnyElement>,
}

impl PugFormShell {
    pub fn new(spec: FormShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            fields_slot: None,
            actions_slot: None,
        }
    }

    pub fn with_fields(mut self, fields: impl IntoElement) -> Self {
        self.fields_slot = Some(fields.into_any_element());
        self
    }

    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions_slot = Some(actions.into_any_element());
        self
    }
}

impl IntoElement for PugFormShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let stack_gap = resolve_px(theme, spec.stack_gap_token());
        let section_gap = resolve_px(theme, spec.section_gap_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let danger = resolve_color(theme, "semantic.color.status.danger");
        let warning = resolve_color(theme, "semantic.color.status.warning");
        let border = resolve_color(theme, "semantic.color.border.subtle");

        let mut form = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(stack_gap)
            .when(spec.is_disabled, |el| el.opacity(0.5))
            .when(spec.is_busy, |el| el.opacity(0.7));

        // Title and description
        if spec.title.is_some() || spec.description.is_some() {
            let mut header = div().flex().flex_col().gap(px(4.0));

            if let Some(ref title) = spec.title {
                header = header.child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child(title.clone()),
                );
            }

            if let Some(ref description) = spec.description {
                header = header.child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child(description.clone()),
                );
            }

            form = form.child(header);
        }

        // Status summary
        if let Some(ref summary) = spec.status_summary {
            let tone_color = match summary.tone {
                pug_gpui_primitives::StatusTone::Danger => danger,
                pug_gpui_primitives::StatusTone::Warning => warning,
                _ => resolve_color(theme, "semantic.color.accent.base"),
            };

            form = form.child(
                div()
                    .px(px(12.0))
                    .py(px(8.0))
                    .rounded(px(4.0))
                    .border_l(px(3.0))
                    .border_color(tone_color)
                    .bg(tone_color.opacity(0.08))
                    .text_sm()
                    .text_color(text_primary)
                    .child(summary.message.clone()),
            );
        }

        // Sections with field state indicators
        if !spec.sections.is_empty() {
            for section in &spec.sections {
                let mut section_el = div().flex().flex_col().gap(section_gap);

                // Section title
                section_el = section_el.child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .pb(px(4.0))
                        .border_b_1()
                        .border_color(border)
                        .child(section.title.clone()),
                );

                if let Some(ref desc) = section.description {
                    section_el = section_el.child(
                        div()
                            .text_xs()
                            .text_color(text_secondary)
                            .child(desc.clone()),
                    );
                }

                // Field state indicators
                for field_id in &section.field_ids {
                    if let Some(field) = spec.fields.iter().find(|f| &f.id == field_id) {
                        let mut field_el = div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .py(px(4.0));

                        // Validation indicator
                        let indicator_color = match field.validation_state {
                            ValidationState::Invalid => danger,
                            ValidationState::Pending => warning,
                            _ => border,
                        };

                        field_el = field_el.child(
                            div()
                                .size(px(6.0))
                                .rounded(px(3.0))
                                .bg(indicator_color),
                        );

                        field_el = field_el.child(
                            div()
                                .text_sm()
                                .text_color(text_primary)
                                .child(field.label.clone()),
                        );

                        if let Some(ref msg) = field.message {
                            field_el = field_el.child(
                                div()
                                    .text_xs()
                                    .text_color(text_secondary)
                                    .child(msg.clone()),
                            );
                        }

                        section_el = section_el.child(field_el);
                    }
                }

                form = form.child(section_el);
            }
        }

        // Custom fields slot
        if let Some(fields) = self.fields_slot {
            form = form.child(fields);
        }

        // Actions row
        if let Some(actions) = self.actions_slot {
            let mut actions_row = div()
                .w_full()
                .flex()
                .gap(px(8.0))
                .pt(px(8.0))
                .border_t_1()
                .border_color(border);

            actions_row = match spec.actions.align {
                FormActionAlign::End => actions_row.justify_end(),
                FormActionAlign::Between => actions_row.justify_between(),
                FormActionAlign::Start => actions_row,
            };

            actions_row = actions_row.child(actions);

            form = form.child(actions_row);
        }

        form.into_any_element()
    }
}
