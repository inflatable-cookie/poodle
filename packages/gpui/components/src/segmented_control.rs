//! PugSegmentedControl — real GPUI component backed by SegmentedControlSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SegmentedControlSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI segmented control component backed by `SegmentedControlSpec`.
pub struct PugSegmentedControl {
    spec: SegmentedControlSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl PugSegmentedControl {
    pub fn new(spec: SegmentedControlSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-seg".to_string(),
            on_change: None,
        }
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PugSegmentedControl {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let accent = resolve_color(theme, self.spec.selected_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_disabled = self.spec.is_disabled;

        let mut row = div()
            .flex()
            .rounded(px(6.0))
            .border_1()
            .border_color(border)
            .bg(surface_bg);

        if is_disabled {
            row = row.opacity(0.48);
        }

        for (i, option) in self.spec.options.iter().enumerate() {
            let is_selected = current_value.as_deref() == Some(&option.value);
            let is_opt_disabled = option.is_disabled;
            let seg_id = SharedString::from(format!("{}-{}", self.id_prefix, option.value));

            let mut seg = div()
                .id(seg_id)
                .px(px(12.0))
                .py(px(6.0))
                .text_sm();

            if is_selected {
                seg = seg.bg(accent).text_color(text_inverse).rounded(px(4.0));
            } else {
                seg = seg.text_color(text_primary);
            }

            if !is_disabled && !is_opt_disabled {
                seg = seg
                    .cursor_pointer()
                    .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.06)));
            }

            // Add separator between non-selected items
            if i > 0 && !is_selected {
                let prev_selected = current_value.as_deref()
                    == self.spec.options.get(i - 1).map(|o| o.value.as_str());
                if !prev_selected {
                    row = row.child(
                        div()
                            .w(px(1.0))
                            .h(px(16.0))
                            .bg(border.opacity(0.5)),
                    );
                }
            }

            seg = seg.child(option.label.clone());
            row = row.child(seg);
        }

        row.into_any_element()
    }
}
