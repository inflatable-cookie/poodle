//! PageLoading — full-page loading state backed by PageLoadingSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::PageLoadingSpec;
use crate::theme_ext::resolve_color;

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
        let backdrop = resolve_color(&self.theme, self.spec.backdrop_fill_token());
        let accent = resolve_color(&self.theme, self.spec.progress_fill_token());
        let text_color = resolve_color(&self.theme, "semantic.color.text.primary");

        let mut el = div()
            .bg(backdrop)
            .size_full()
            .flex().items_center().justify_center()
            .flex_col().gap(px(16.0));
        // Spinner placeholder
        el = el.child(div().text_color(accent).text_size(px(24.0)).child("⟳"));
        if let Some(ref msg) = self.spec.message {
            el = el.child(div().text_size(px(14.0)).text_color(text_color).child(msg.clone()));
        }
        el.into_any_element()
    }
}
