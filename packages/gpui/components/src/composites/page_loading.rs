//! PageLoading — full-page loading state backed by PageLoadingSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::PageLoadingSpec;
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};
use crate::presentation::{resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem, control_space_x_rem, rem_to_px};
use crate::primitives::Spinner;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct PageLoading {
    spec: PageLoadingSpec,
    theme: GpuiThemeProvider,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for PageLoading {
    type Target = PageLoadingSpec;
    fn deref(&self) -> &PageLoadingSpec { &self.spec }
}

impl PageLoading {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: PageLoadingSpec::new(), theme: theme.clone(), on_cancel: None }
    }
    pub fn from_spec(spec: PageLoadingSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_cancel: None }
    }
    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }
}

impl IntoElement for PageLoading {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        if !self.spec.is_visible { return div().into_any_element(); }
        let theme = &self.theme;
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let pad_x = rem_to_px(panel_space_x_rem(self.spec.density));
        let pad_y = rem_to_px(panel_space_y_rem(self.spec.density));
        let item_gap = rem_to_px(control_space_x_rem(self.spec.density));

        let backdrop = resolve_color(theme, self.spec.backdrop_fill_token());
        let accent = resolve_color(theme, self.spec.progress_fill_token());
        let text_color = resolve_color(theme, "semantic.color.text.primary");
        let body_size = px(font_size);
        let label_size = px(font_size * 0.85);
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let border = resolve_color(theme, "semantic.color.border.default");
        let radius = resolve_radius(theme, "semantic.radius.surface");

        // Card container
        let mut card = div()
            .bg(surface_bg)
            .border_1().border_color(border)
            .rounded(radius)
            // Contract: elevation-dialog shadow
            .shadow(vec![
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.12),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(24.0),
                    spread_radius: px(0.0),
                },
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.08),
                    offset: point(px(0.0), px(2.0)),
                    blur_radius: px(8.0),
                    spread_radius: px(0.0),
                },
            ])
            .px(px(pad_x)).py(px(pad_y))
            .flex().flex_col().items_center().gap(px(item_gap * 2.0));

        // Spinner
        card = card.child(
            Spinner::from_spec(
                SpinnerSpec::new()
                    .with_variant(SpinnerVariant::Ring)
                    .with_size(SpinnerSize::Lg)
                    .with_tone(SpinnerTone::Accent),
                theme,
            )
            .with_color(accent),
        );

        // Message
        if let Some(ref msg) = self.spec.message {
            card = card.child(div().text_size(body_size).text_color(text_color).child(msg.clone()));
        }

        // Cancel button
        if let Some(handler) = self.on_cancel {
            let muted = resolve_color(theme, "semantic.color.text.secondary");
            let hover_bg = resolve_color(theme, "semantic.color.background.elevated");
            let control_radius = resolve_radius(theme, "semantic.radius.control");
            card = card.child(
                div()
                    .id("poodle-page-loading-cancel")
                    .text_size(label_size)
                    .text_color(muted)
                    .cursor_pointer()
                    .px(px(12.0)).py(px(6.0))
                    .rounded(control_radius)
                    .hover(move |s| s.bg(hover_bg))
                    .child("Cancel")
                    .on_click(move |_event, window, cx| {
                        handler(window, cx);
                    })
            );
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
