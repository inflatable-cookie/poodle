//! PageHeader — real GPUI component backed by PageHeaderSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::{PageHeaderAlign, PageHeaderSpec};

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI page header component backed by `PageHeaderSpec`.
///
/// Renders a standardized title and action region with optional breadcrumbs,
/// eyebrow text, subtitle, and actions slot.
pub struct PageHeader {
    spec: PageHeaderSpec,
    theme: GpuiThemeProvider,
    breadcrumbs_slot: Option<AnyElement>,
    actions_slot: Option<AnyElement>,
}

impl std::ops::Deref for PageHeader {
    type Target = PageHeaderSpec;
    fn deref(&self) -> &PageHeaderSpec { &self.spec }
}

impl PageHeader {
    pub fn new(title: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self { spec: PageHeaderSpec::new(title), theme: theme.clone(), breadcrumbs_slot: None, actions_slot: None }
    }

    pub fn from_spec(spec: PageHeaderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            breadcrumbs_slot: None,
            actions_slot: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn subtitle(mut self, v: impl Into<String>) -> Self { self.spec.subtitle = Some(v.into()); self }
    pub fn eyebrow(mut self, v: impl Into<String>) -> Self { self.spec.eyebrow = Some(v.into()); self }
    pub fn align(mut self, v: PageHeaderAlign) -> Self { self.spec.align = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_breadcrumbs(mut self, breadcrumbs: impl IntoElement) -> Self {
        self.breadcrumbs_slot = Some(breadcrumbs.into_any_element());
        self
    }

    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions_slot = Some(actions.into_any_element());
        self
    }
}

impl IntoElement for PageHeader {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let gap = resolve_px(theme, spec.gap_token());
        let header_gap = resolve_px(theme, spec.header_gap_token());
        let padding_y = resolve_px(theme, spec.padding_y_token());

        let title_color = resolve_color(theme, spec.title_color_token());
        let subtitle_color = resolve_color(theme, spec.subtitle_color_token());
        let eyebrow_color = resolve_color(theme, spec.eyebrow_color_token());

        let mut wrapper = div().w_full().flex().flex_col().py(padding_y);

        // Optional breadcrumbs above
        if let Some(breadcrumbs) = self.breadcrumbs_slot {
            wrapper = wrapper.child(
                div().w_full().mb(gap).child(breadcrumbs),
            );
        }

        // Main row: title block + actions
        let justify = match spec.align {
            PageHeaderAlign::Between => true,
            PageHeaderAlign::Start => false,
        };

        let mut main_row = div()
            .w_full()
            .flex()
            .items_center()
            .gap(header_gap);

        if justify {
            main_row = main_row.justify_between();
        }

        // Title block: eyebrow, title, subtitle stacked
        let mut title_block = div().flex().flex_col().gap(gap);

        if let Some(ref eyebrow) = spec.eyebrow {
            title_block = title_block.child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(eyebrow_color)
                    .child(eyebrow.clone()),
            );
        }

        title_block = title_block.child(
            div()
                .text_base()
                .font_weight(FontWeight::BOLD)
                .text_color(title_color)
                .child(spec.title.clone()),
        );

        if let Some(ref subtitle) = spec.subtitle {
            title_block = title_block.child(
                div()
                    .text_size(px(14.0))
                    .text_color(subtitle_color)
                    .child(subtitle.clone()),
            );
        }

        main_row = main_row.child(title_block);

        if let Some(actions) = self.actions_slot {
            main_row = main_row.child(
                div().flex().items_center().flex_shrink_0().child(actions),
            );
        }

        wrapper = wrapper.child(main_row);

        wrapper.into_any_element()
    }
}
