//! PugDetailShell — real GPUI component backed by DetailShellSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::{DetailShellSpec, DetailState};

use crate::theme_ext::resolve_color;

/// A real GPUI detail shell component backed by `DetailShellSpec`.
///
/// Renders a detail page container with a header area (title) and a content slot.
/// Shows loading/error/empty states when content is not ready.
pub struct PugDetailShell {
    spec: DetailShellSpec,
    theme: GpuiThemeProvider,
    header_slot: Option<AnyElement>,
    content_slot: Option<AnyElement>,
}

impl PugDetailShell {
    pub fn new(spec: DetailShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            header_slot: None,
            content_slot: None,
        }
    }

    pub fn with_header(mut self, header: impl IntoElement) -> Self {
        self.header_slot = Some(header.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content_slot = Some(content.into_any_element());
        self
    }
}

impl IntoElement for PugDetailShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let body_bg = resolve_color(theme, spec.body_fill_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let border = resolve_color(theme, "semantic.color.border.subtle");

        let mut shell = div()
            .w_full()
            .flex()
            .flex_col()
            .flex_grow()
            .bg(body_bg)
            .overflow_hidden();

        // Header section
        let mut header = div()
            .w_full()
            .px(px(16.0))
            .py(px(12.0))
            .border_b_1()
            .border_color(border)
            .flex()
            .items_center()
            .gap(px(8.0));

        if let Some(ref title) = spec.title {
            header = header.child(
                div()
                    .text_base()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(title.clone()),
            );
        }

        if let Some(header_content) = self.header_slot {
            header = header.child(header_content);
        }

        shell = shell.child(header);

        // Body section — state-dependent
        let body = match spec.state {
            DetailState::Ready => {
                let mut content_area = div()
                    .w_full()
                    .flex_grow()
                    .px(px(16.0))
                    .py(px(12.0));
                if let Some(content) = self.content_slot {
                    content_area = content_area.child(content);
                }
                content_area
            }
            DetailState::Loading => div()
                .w_full()
                .flex_grow()
                .flex()
                .items_center()
                .justify_center()
                .py(px(32.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child("Loading\u{2026}"),
                ),
            DetailState::Error => div()
                .w_full()
                .flex_grow()
                .flex()
                .items_center()
                .justify_center()
                .py(px(32.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(resolve_color(theme, "semantic.color.status.danger"))
                        .child("An error occurred loading this content."),
                ),
            DetailState::Empty => div()
                .w_full()
                .flex_grow()
                .flex()
                .items_center()
                .justify_center()
                .py(px(32.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(text_secondary)
                        .child("No content available."),
                ),
        };

        shell = shell.child(body);

        shell.into_any_element()
    }
}
