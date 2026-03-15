//! PugGridShell — real GPUI component backed by GridShellSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::{GridShellSpec, BrowseState, MinColumnWidth};

use crate::theme_ext::resolve_color;

/// A real GPUI grid shell component backed by `GridShellSpec`.
///
/// Renders a responsive grid container that arranges children in columns.
/// Shows state-dependent content for loading, error, empty, and no-results states.
pub struct PugGridShell {
    spec: GridShellSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
    empty_state: Option<AnyElement>,
}

impl PugGridShell {
    pub fn new(spec: GridShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
            empty_state: None,
        }
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn with_children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        for child in children {
            self.children.push(child.into_any_element());
        }
        self
    }

    pub fn with_empty_state(mut self, empty: impl IntoElement) -> Self {
        self.empty_state = Some(empty.into_any_element());
        self
    }
}

impl IntoElement for PugGridShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let bg = resolve_color(theme, "semantic.color.background.panel");

        // State-dependent rendering
        match spec.state {
            BrowseState::Loading => {
                return div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(32.0))
                    .bg(bg)
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary)
                            .child("Loading\u{2026}"),
                    )
                    .into_any_element();
            }
            BrowseState::Error => {
                return div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(32.0))
                    .bg(bg)
                    .child(
                        div()
                            .text_sm()
                            .text_color(resolve_color(theme, "semantic.color.status.danger"))
                            .child("An error occurred."),
                    )
                    .into_any_element();
            }
            BrowseState::Empty | BrowseState::NoResults => {
                if let Some(empty) = self.empty_state {
                    return div()
                        .w_full()
                        .bg(bg)
                        .child(empty)
                        .into_any_element();
                }
                let msg = if spec.state == BrowseState::NoResults {
                    "No results found."
                } else {
                    "No items yet."
                };
                return div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(32.0))
                    .bg(bg)
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_secondary)
                            .child(msg),
                    )
                    .into_any_element();
            }
            BrowseState::Ready => {}
        }

        // Grid layout with min column width
        let min_col = match spec.min_column_width {
            MinColumnWidth::Sm => px(160.0),
            MinColumnWidth::Md => px(220.0),
            MinColumnWidth::Lg => px(300.0),
        };

        let gap = px(12.0);

        let mut grid = div()
            .w_full()
            .flex()
            .flex_wrap()
            .gap(gap)
            .bg(bg);

        for child in self.children {
            grid = grid.child(
                div()
                    .min_w(min_col)
                    .flex_grow()
                    .child(child),
            );
        }

        grid.into_any_element()
    }
}
