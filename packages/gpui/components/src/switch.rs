//! PugSwitch — real GPUI component backed by SwitchSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::SwitchSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI switch/toggle component backed by `SwitchSpec`.
pub struct PugSwitch {
    spec: SwitchSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl PugSwitch {
    pub fn new(spec: SwitchSpec, theme: &GpuiThemeProvider) -> Self {
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

impl IntoElement for PugSwitch {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let track_fill = resolve_color(theme, spec.track_fill_token());
        let border = resolve_color(theme, "semantic.color.border.default");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");

        let is_checked = spec.current_checked();
        let is_interactive = !spec.is_disabled;

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-switch-{}", suffix)
        } else {
            format!(
                "pug-switch-{}",
                spec.label.as_deref().unwrap_or("anon")
            )
        };

        // Track (36x20 rounded pill)
        let track_bg = if is_checked {
            track_fill
        } else {
            resolve_color(theme, "semantic.color.background.surface")
        };

        let track_border = if is_checked { track_fill } else { border };

        // Knob offset: checked = right (16px), unchecked = left (2px)
        let knob_offset = if is_checked { px(18.0) } else { px(2.0) };
        // Contract: knob color is text-primary (unchecked) or text-inverse (checked)
        let knob_color = if is_checked {
            resolve_color(theme, "semantic.color.text.inverse")
        } else {
            text_primary
        };

        let track = div()
            .w(px(36.0))
            .h(px(20.0))
            .rounded(px(10.0))
            .bg(track_bg)
            .border_1()
            .border_color(track_border)
            .relative()
            .flex_shrink_0()
            .child(
                div()
                    .w(px(14.0))
                    .h(px(14.0))
                    .rounded(px(7.0))
                    .bg(knob_color)
                    .shadow_sm()
                    .absolute()
                    .top(px(2.0))
                    .left(knob_offset),
            );

        // Row: track + label
        let mut row = div()
            .id(SharedString::from(id_str))
            .flex()
            .items_center()
            .gap(px(8.0));

        if is_interactive {
            row = row.cursor_pointer();
        }

        if spec.is_disabled {
            row = row.opacity(0.48);
        }

        row = row.child(track);

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
                let next = !is_checked;
                row = row.on_click(move |_event, window, cx| {
                    handler(&next, window, cx);
                });
            }
        }

        row.into_any_element()
    }
}
