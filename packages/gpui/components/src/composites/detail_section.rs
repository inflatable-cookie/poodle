//! DetailSection — real GPUI component backed by DetailSectionSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::DetailSectionSpec;

use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI detail section component backed by `DetailSectionSpec`.
///
/// Renders a titled grouping of related detail rows with an optional separator,
/// header (title + description + actions), and body slot for content.
pub struct DetailSection {
    spec: DetailSectionSpec,
    theme: GpuiThemeProvider,
    actions_slot: Option<AnyElement>,
    body_slot: Option<AnyElement>,
}

impl std::ops::Deref for DetailSection {
    type Target = DetailSectionSpec;
    fn deref(&self) -> &DetailSectionSpec { &self.spec }
}

impl DetailSection {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DetailSectionSpec::new(), theme: theme.clone(), actions_slot: None, body_slot: None }
    }

    pub fn from_spec(spec: DetailSectionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            actions_slot: None,
            body_slot: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn separated(mut self, v: bool) -> Self { self.spec.is_separated = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions_slot = Some(actions.into_any_element());
        self
    }

    pub fn with_body(mut self, body: impl IntoElement) -> Self {
        self.body_slot = Some(body.into_any_element());
        self
    }
}

impl IntoElement for DetailSection {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let section_gap = resolve_px(theme, spec.section_gap_token());
        let body_gap = resolve_px(theme, spec.body_gap_token());
        let header_gap = resolve_px(theme, spec.header_gap_token());
        let title_body_gap = resolve_px(theme, spec.title_body_gap_token());

        let title_color = resolve_color(theme, spec.title_color_token());
        let description_color = resolve_color(theme, spec.description_color_token());
        let separator_color = resolve_color(theme, spec.separator_color_token());

        let mut section = div().w_full().flex().flex_col();

        // Top separator when is_separated
        if spec.is_separated {
            section = section.child(
                div()
                    .w_full()
                    .h(px(1.0))
                    .bg(separator_color)
                    .mb(section_gap),
            );
        }

        // Header row: title block on left, actions on right
        let has_header = spec.title.is_some() || spec.description.is_some() || self.actions_slot.is_some();
        if has_header {
            let mut header = div()
                .w_full()
                .flex()
                .items_center()
                .justify_between()
                .gap(header_gap)
                .mb(title_body_gap);

            // Title block: title + description stacked
            let mut title_block = div().flex().flex_col();

            if let Some(ref title) = spec.title {
                title_block = title_block.child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(title_color)
                        .child(title.clone()),
                );
            }

            if let Some(ref description) = spec.description {
                title_block = title_block.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(description_color)
                        .child(description.clone()),
                );
            }

            header = header.child(title_block);

            if let Some(actions) = self.actions_slot {
                header = header.child(
                    div().flex().items_center().flex_shrink_0().child(actions),
                );
            }

            section = section.child(header);
        }

        // Body: children slot for detail rows
        if let Some(body) = self.body_slot {
            section = section.child(
                div().w_full().flex().flex_col().gap(body_gap).child(body),
            );
        }

        section.into_any_element()
    }
}
