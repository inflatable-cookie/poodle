//! SlugField — URL slug input backed by SlugFieldSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::SlugFieldSpec;
use pug_primitives::{IconSize, IconSpec};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct SlugField {
    spec: SlugFieldSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for SlugField {
    type Target = SlugFieldSpec;
    fn deref(&self) -> &SlugFieldSpec { &self.spec }
}

impl SlugField {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SlugFieldSpec::new(""), theme: theme.clone() }
    }
    pub fn from_spec(spec: SlugFieldSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for SlugField {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let text_color = resolve_color(theme, spec.text_color_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.control");
        let fill = resolve_color(theme, "semantic.color.background.surface");
        let muted = resolve_color(theme, "semantic.color.text.secondary");

        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let mut el = div().id("pug-slug-field").flex().flex_row().items_center()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .h(px(36.0))
            .focus(move |s| s.border_color(focus_ring));

        if let Some(ref prefix) = spec.prefix {
            el = el.child(div().text_size(px(14.0)).text_color(muted).pl(px(12.0)).child(prefix.clone()));
        }
        el = el.child(
            div()
                .flex_1()
                .text_size(px(14.0))
                .text_color(text_color)
                .px(px(8.0))
                .child(spec.value.clone()),
        );

        // Reset button (x icon) to clear the slug value
        if !spec.value.is_empty() && !spec.is_disabled {
            let icon_color = resolve_color(theme, "semantic.color.text.secondary");
            el = el.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .pr(px(8.0))
                    .cursor_pointer()
                    .child(
                        Icon::from_spec(
                            IconSpec::new("x").with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(icon_color),
                    ),
            );
        }

        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
            el = el.opacity(opacity);
        }
        el.into_any_element()
    }
}
