//! PageLoading — full-page loading state backed by PageLoadingSpec.

use crate::presentation::{
    control_space_x_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::primitives::{Progress, Spinner};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, ProgressSpec, SemanticControlSizeRole, SpinnerSize, SpinnerSpec,
    SpinnerTone, SpinnerVariant,
};
use poodle_specs::{PageLoadingPresentation, PageLoadingSpec};

pub struct PageLoading {
    spec: PageLoadingSpec,
    theme: GpuiThemeProvider,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for PageLoading {
    type Target = PageLoadingSpec;
    fn deref(&self) -> &PageLoadingSpec {
        &self.spec
    }
}

impl PageLoading {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: PageLoadingSpec::new(),
            theme: theme.clone(),
            on_cancel: None,
        }
    }
    pub fn from_spec(spec: PageLoadingSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_cancel: None,
        }
    }
    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
    pub fn presentation(mut self, v: PageLoadingPresentation) -> Self {
        self.spec.presentation = v;
        self
    }
    pub fn inline(mut self) -> Self {
        self.spec.presentation = PageLoadingPresentation::Inline;
        self
    }
}

impl IntoElement for PageLoading {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        if !self.spec.is_visible {
            return div().into_any_element();
        }
        let theme = &self.theme;
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let pad_x = rem_to_px(panel_space_x_rem(self.spec.density));
        let pad_y = rem_to_px(panel_space_y_rem(self.spec.density));
        let item_gap = rem_to_px(control_space_x_rem(self.spec.density));

        let backdrop = resolve_color(theme, self.spec.backdrop_fill_token());
        let accent = resolve_color(theme, self.spec.progress_fill_token());
        let text_color = resolve_color(theme, "color.text.primary");
        let body_size = px(font_size);
        let label_size = resolve_px(theme, "typography.label.size");
        let surface_bg = resolve_color(theme, "color.background.elevated");
        let border = resolve_color(theme, "color.border.default");
        let radius = resolve_radius(theme, "radius.surface");
        let is_inline = self.spec.presentation.is_inline();

        // Card container. Overlay mode is an elevated card with chrome;
        // inline mode (contract §8 `.page-loading[data-presentation="inline"]
        // .page-loading__card`) drops border/bg/shadow and uses no padding.
        let mut card = div().flex().flex_col().items_center().gap(px(item_gap * 2.0));
        if is_inline {
            // Inline card: transparent container, no chrome, no padding.
            card = card.max_w(px(rem_to_px(24.0)));
        } else {
            // Overlay card: full elevated chrome.
            card = card
                .bg(surface_bg)
                .border_1()
                .border_color(border)
                .rounded(radius)
                // Contract §8 card: box-shadow = var(--poodle-elevation-overlay).
                .shadow(crate::theme_ext::elevation_overlay_shadow())
                .px(px(pad_x))
                .py(px(pad_y));
        }

        // Spinner — always rendered per the contract doc
        // (page-loading.md L72: determinate state shows "Spinner shown
        // plus progress bar"). The ring spinner provides continuous
        // motion feedback even when a determinate value is known.
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

        // Progress bar — rendered only when a determinate value is
        // provided. Composes the shared Progress primitive (contract §2
        // anatomy: "Progress primitive") so the determinate track resolves
        // its chrome/fill from the same tokens as a standalone Progress.
        // Wrapped in a full-width box (contract §8 `.page-loading__progress`
        // width: 100%).
        if let Some(value) = self.spec.value {
            let mut progress_spec = ProgressSpec::new().with_value(value);
            progress_spec.max = self.spec.max;
            let progress = Progress::from_spec(progress_spec, theme)
            .aria_label(self.spec.message.clone().unwrap_or_else(|| "Loading progress".into()));
            card = card.child(div().w_full().child(progress));
        }

        // Message
        if let Some(ref msg) = self.spec.message {
            card = card.child(
                div()
                    .text_size(body_size)
                    .text_color(text_color)
                    .child(msg.clone()),
            );
        }

        // Cancel button — gated on spec.can_cancel alone (Svelte renders
        // the button on `canCancel`, handler optional). The click handler
        // is wired only when one was provided.
        if self.spec.can_cancel {
            let muted = resolve_color(theme, "color.text.secondary");
            let hover_bg = resolve_color(theme, "color.background.surface");
            let cancel_border = resolve_color(theme, "color.border.default");
            let control_radius = resolve_radius(theme, "radius.control");
            let mut cancel = div()
                .id("poodle-page-loading-cancel")
                .text_size(label_size)
                .text_color(muted)
                .cursor_pointer()
                // Contract §8 `.page-loading__cancel`: padding 0.375rem 0.875rem,
                // 1px border-default, radius-control.
                .px(px(rem_to_px(0.875)))
                .py(px(rem_to_px(0.375)))
                .border_1()
                .border_color(cancel_border)
                .rounded(control_radius)
                .hover(move |s| s.bg(hover_bg))
                .child("Cancel");
            if let Some(handler) = self.on_cancel {
                cancel = cancel.on_click(move |_event, window, cx| {
                    handler(window, cx);
                });
            }
            card = card.child(cancel);
        }

        // Branch on presentation: Inline renders the card on its
        // own, sized to fit the parent container; Overlay wraps it
        // in a full-viewport backdrop scrim.
        match self.spec.presentation {
            PageLoadingPresentation::Inline => div()
                .w_full()
                .flex()
                .items_center()
                .justify_center()
                // Svelte: padding = 3rem 1rem
                .py(px(rem_to_px(3.0)))
                .px(px(rem_to_px(1.0)))
                .child(card)
                .into_any_element(),
            PageLoadingPresentation::Overlay => div()
                .bg(backdrop)
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(card)
                .into_any_element(),
        }
    }
}
