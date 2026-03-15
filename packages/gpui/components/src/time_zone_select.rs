//! PugTimeZoneSelect — real GPUI component backed by TimeZoneSelectSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::TimeZoneSelectSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI timezone select dropdown component backed by `TimeZoneSelectSpec`.
pub struct PugTimeZoneSelect {
    spec: TimeZoneSelectSpec,
    theme: GpuiThemeProvider,
}

impl PugTimeZoneSelect {
    pub fn new(spec: TimeZoneSelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugTimeZoneSelect {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());

        let trigger_text = spec
            .trigger_text()
            .unwrap_or("Select timezone...")
            .to_string();
        let is_placeholder = spec.value.is_none();
        let text_col = if is_placeholder {
            text_secondary
        } else {
            text_primary
        };

        let mut trigger = div()
            .h(px(36.0))
            .px(px(12.0))
            .rounded(px(6.0))
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .text_sm()
            .child(div().text_color(text_col).child(trigger_text))
            .child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .child(if spec.is_open { "\u{25b4}" } else { "\u{25be}" }),
            );

        if spec.is_disabled {
            trigger = trigger.opacity(0.48);
        } else {
            trigger = trigger.cursor_pointer();
        }

        let mut wrapper = div().flex().flex_col().gap(px(4.0)).child(trigger);

        if spec.is_open {
            let dropdown = div()
                .rounded(px(6.0))
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow_md()
                .py(px(4.0))
                .text_sm()
                .text_color(text_primary)
                .child(
                    div()
                        .px(px(10.0))
                        .py(px(6.0))
                        .child("UTC"),
                )
                .child(
                    div()
                        .px(px(10.0))
                        .py(px(6.0))
                        .child("America/New_York"),
                )
                .child(
                    div()
                        .px(px(10.0))
                        .py(px(6.0))
                        .child("Europe/London"),
                );

            wrapper = wrapper.child(dropdown);
        }

        wrapper.into_any_element()
    }
}
