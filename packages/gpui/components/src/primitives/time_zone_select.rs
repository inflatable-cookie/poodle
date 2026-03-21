//! TimeZoneSelect — real GPUI component backed by TimeZoneSelectSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::TimeZoneSelectSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI timezone select dropdown component backed by `TimeZoneSelectSpec`.
pub struct TimeZoneSelect {
    spec: TimeZoneSelectSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for TimeZoneSelect {
    type Target = TimeZoneSelectSpec;
    fn deref(&self) -> &TimeZoneSelectSpec { &self.spec }
}

impl TimeZoneSelect {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TimeZoneSelectSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: TimeZoneSelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self { self.spec.placeholder = Some(v.into()); self }
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }

}

impl IntoElement for TimeZoneSelect {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let control_height = resolve_px(theme, "semantic.size.control.height");
        let inline_padding = resolve_px(theme, "semantic.space.inline.md");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");
        let control_radius = resolve_radius(theme, "semantic.radius.control");

        let border = resolve_color(theme, spec.border_token());
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");

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
            .h(control_height)
            .px(inline_padding)
            .rounded(control_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_between()
            .gap(inline_gap)
            .text_sm()
            .child(div().text_color(text_col).child(trigger_text))
            .child(
                div()
                    .text_xs()
                    .text_color(text_secondary)
                    .child(if spec.is_open { "\u{25b4}" } else { "\u{25be}" }),
            );

        if spec.is_disabled {
            trigger = trigger.opacity(disabled_opacity);
        } else {
            trigger = trigger.cursor_pointer();
        }

        let mut wrapper = div().flex().flex_col().gap(px(4.0)).child(trigger);

        if spec.is_open {
            let dropdown = div()
                .rounded(control_radius)
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
