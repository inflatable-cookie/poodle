//! PugListShell — real GPUI component backed by ListShellSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_composites::{ListShellSpec, BrowseState};

use crate::theme_ext::resolve_color;

/// A real GPUI list shell component backed by `ListShellSpec`.
///
/// Renders a stateful list container with support for loading, error, empty,
/// and no-results states. In the ready state, renders child list items.
pub struct PugListShell {
    spec: ListShellSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
    empty_state: Option<AnyElement>,
}

impl PugListShell {
    pub fn new(spec: ListShellSpec, theme: &GpuiThemeProvider) -> Self {
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

impl IntoElement for PugListShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let viewport_bg = resolve_color(theme, spec.viewport_fill_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let border = resolve_color(theme, "semantic.color.border.subtle");

        // State-dependent rendering
        match spec.state {
            BrowseState::Loading => {
                return div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .py(px(32.0))
                    .bg(viewport_bg)
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
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .py(px(32.0))
                    .bg(viewport_bg)
                    .child(
                        div()
                            .text_sm()
                            .text_color(resolve_color(theme, "semantic.color.status.danger"))
                            .child("An error occurred loading items."),
                    )
                    .into_any_element();
            }
            BrowseState::Empty | BrowseState::NoResults => {
                if let Some(empty) = self.empty_state {
                    return div()
                        .w_full()
                        .bg(viewport_bg)
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
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .py(px(32.0))
                    .bg(viewport_bg)
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

        // Ready state: render list items
        let mut list = div()
            .w_full()
            .flex()
            .flex_col()
            .bg(viewport_bg);

        for (i, child) in self.children.into_iter().enumerate() {
            let mut item = div()
                .w_full()
                .child(child);

            // Add separator between items
            if i > 0 {
                item = item.border_t_1().border_color(border.opacity(0.5));
            }

            list = list.child(item);
        }

        list.into_any_element()
    }
}
