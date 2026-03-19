//! PugSegmentedControl — real GPUI component backed by SegmentedControlSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SegmentedControlSpec;

use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

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

        let control_padding_x = resolve_px(theme, "semantic.space.control.x");
        let control_padding_y = resolve_px(theme, "semantic.space.control.y");
        let control_radius = resolve_radius(theme, "semantic.radius.control");
        let inner_radius = px(theme.resolve_radius("semantic.radius.control") * 0.6);

        let accent = resolve_color(theme, self.spec.selected_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_inverse = resolve_color(theme, "semantic.color.text.inverse");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

        // Contract: hover = color-mix with elevated
        let hover_bg = color_mix(surface_bg, elevated, 0.84);

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let is_disabled = self.spec.is_disabled;

        let mut row = div()
            .flex()
            .rounded(control_radius)
            .border_1()
            .border_color(border)
            .bg(surface_bg);

        if is_disabled {
            row = row.opacity(disabled_opacity);
        }

        for (i, option) in self.spec.options.iter().enumerate() {
            let is_selected = current_value.as_deref() == Some(&option.value);
            let is_opt_disabled = option.is_disabled;
            let seg_id = SharedString::from(format!("{}-{}", self.id_prefix, option.value));

            let mut seg = div()
                .id(seg_id)
                .px(control_padding_x)
                .py(control_padding_y)
                .text_sm();

            if is_selected {
                seg = seg.bg(accent).text_color(text_inverse).rounded(inner_radius);
            } else {
                seg = seg.text_color(text_primary);
            }

            if !is_disabled && !is_opt_disabled {
                seg = seg
                    .cursor_pointer()
                    .hover(move |s| s.bg(hover_bg));
            }

            // Add separator between non-selected items
            if i > 0 && !is_selected {
                let prev_selected = current_value.as_deref()
                    == self.spec.options.get(i - 1).map(|o| o.value.as_str());
                if !prev_selected {
                    row = row.child(
                        div()
                            .w(px(1.0))
                            .h_full()
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
