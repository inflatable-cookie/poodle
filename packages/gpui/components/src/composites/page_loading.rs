//! PageLoading — full-page loading state backed by PageLoadingSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::PageLoadingSpec;
use pug_primitives::{IconSize, IconSpec};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_radius};

pub struct PageLoading {
    spec: PageLoadingSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for PageLoading {
    type Target = PageLoadingSpec;
    fn deref(&self) -> &PageLoadingSpec { &self.spec }
}

impl PageLoading {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: PageLoadingSpec::new(), theme: theme.clone() }
    }
    pub fn from_spec(spec: PageLoadingSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for PageLoading {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        if !self.spec.is_visible { return div().into_any_element(); }
        let theme = &self.theme;
        let backdrop = resolve_color(theme, self.spec.backdrop_fill_token());
        let accent = resolve_color(theme, self.spec.progress_fill_token());
        let text_color = resolve_color(theme, "semantic.color.text.primary");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let border = resolve_color(theme, "semantic.color.border.default");
        let radius = resolve_radius(theme, "semantic.radius.surface");

        // Card container
        let mut card = div()
            .bg(surface_bg)
            .border_1().border_color(border)
            .rounded(radius)
            .shadow_lg()
            .px(px(24.0)).py(px(20.0))
            .flex().flex_col().items_center().gap(px(16.0));

        // Spinner
        card = card.child(
            Icon::from_spec(IconSpec::new("loader").with_size(IconSize::Md), theme)
                .with_color(accent),
        );

        // Message
        if let Some(ref msg) = self.spec.message {
            card = card.child(div().text_size(px(14.0)).text_color(text_color).child(msg.clone()));
        }

        // Full-page backdrop
        div()
            .bg(backdrop)
            .size_full()
            .flex().items_center().justify_center()
            .child(card)
            .into_any_element()
    }
}
