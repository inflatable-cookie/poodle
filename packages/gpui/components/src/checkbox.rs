//! PugCheckbox — real GPUI component backed by CheckboxSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::{CheckState, CheckboxSpec, IconSize, IconSpec};

use crate::icon::PugIcon;
use crate::theme_ext::resolve_color;

/// A real GPUI checkbox component backed by `CheckboxSpec`.
pub struct PugCheckbox {
    spec: CheckboxSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl PugCheckbox {
    pub fn new(spec: CheckboxSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_change: None,
        }
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugCheckbox {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let accent = resolve_color(theme, spec.indicator_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");

        let state = spec.current_state();
        let is_checked = matches!(state, CheckState::Checked | CheckState::Mixed);
        let is_interactive = spec.is_interactive();

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-checkbox-{}", suffix)
        } else {
            format!(
                "pug-checkbox-{}",
                spec.label.as_deref().unwrap_or("anon")
            )
        };

        // Indicator box
        let indicator = {
            let mut ind = div()
                .w(px(18.0))
                .h(px(18.0))
                .rounded(px(5.0))
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0();

            if is_checked {
                ind = ind.bg(accent).border_1().border_color(accent);
                // Use proper icons for check/minus marks
                match state {
                    CheckState::Checked => {
                        ind = ind.child(
                            PugIcon::new(
                                IconSpec::new("check").with_size(IconSize::Sm),
                                theme,
                            )
                            .with_color(text_inverse),
                        );
                    }
                    CheckState::Mixed => {
                        ind = ind.child(
                            PugIcon::new(
                                IconSpec::new("minus").with_size(IconSize::Sm),
                                theme,
                            )
                            .with_color(text_inverse),
                        );
                    }
                    _ => {}
                }
            } else {
                ind = ind
                    .bg(surface_bg)
                    .border_1()
                    .border_color(border);
            }

            ind
        };

        // Row: indicator + label
        let mut row = div()
            .id(SharedString::from(id_str))
            .flex()
            .items_center()
            .gap(px(8.0));

        if is_interactive {
            row = row
                .cursor_pointer()
                .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.04)));
        }

        if spec.is_disabled {
            row = row.opacity(0.48);
        }

        row = row.child(indicator);

        if let Some(ref label) = spec.label {
            row = row.child(
                div()
                    .text_sm()
                    .text_color(text_primary)
                    .child(label.clone()),
            );
        }

        // Click handler
        if let Some(handler) = self.on_change {
            if is_interactive {
                let next_checked = !is_checked;
                row = row.on_click(move |_event, window, cx| {
                    handler(&next_checked, window, cx);
                });
            }
        }

        row.into_any_element()
    }
}
