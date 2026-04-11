//! DetailShell — real GPUI component backed by DetailShellSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_components::{DetailShellSpec, DetailState, ScrollOwner};
use poodle_components::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::primitives::Spinner;
use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI detail shell component backed by `DetailShellSpec`.
///
/// Renders a detail page container with a header area (title) and a content slot.
/// Shows loading/error/empty states when content is not ready.
pub struct DetailShell {
    spec: DetailShellSpec,
    theme: GpuiThemeProvider,
    header_slot: Option<AnyElement>,
    content_slot: Option<AnyElement>,
}

impl std::ops::Deref for DetailShell {
    type Target = DetailShellSpec;
    fn deref(&self) -> &DetailShellSpec { &self.spec }
}

impl DetailShell {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: DetailShellSpec::new(), theme: theme.clone(), header_slot: None, content_slot: None }
    }

    pub fn from_spec(spec: DetailShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            header_slot: None,
            content_slot: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn scroll_owner(mut self, v: ScrollOwner) -> Self { self.spec.scroll_owner = v; self }
    pub fn state(mut self, v: DetailState) -> Self { self.spec.state = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_header(mut self, header: impl IntoElement) -> Self {
        self.header_slot = Some(header.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content_slot = Some(content.into_any_element());
        self
    }
}

impl IntoElement for DetailShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_gap = resolve_px(theme, "space.inline.sm");
        let body_size = resolve_px(theme, "typography.body.size");
        let heading_size = resolve_px(theme, "typography.heading.size");

        let body_bg = resolve_color(theme, spec.body_fill_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let border = resolve_color(theme, "color.border.subtle");

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
            .gap(inline_gap);

        if let Some(ref title) = spec.title {
            header = header.child(
                div()
                    .text_size(heading_size)
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
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(8.0))
                .py(px(32.0))
                .child(
                    Spinner::from_spec(
                        SpinnerSpec::new()
                            .with_variant(SpinnerVariant::Grid)
                            .with_size(SpinnerSize::Md)
                            .with_tone(SpinnerTone::Accent),
                        theme,
                    ),
                )
                .child(
                    div()
                        .text_size(body_size)
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
                        .text_size(body_size)
                        .text_color(resolve_color(theme, "color.status.danger"))
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
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child("No content available."),
                ),
        };

        shell = shell.child(body);

        shell.into_any_element()
    }
}
