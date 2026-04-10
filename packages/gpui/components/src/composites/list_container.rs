//! ListContainer — paginated list view with header and state handling.

use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{ListContainerSpec, ListContainerState};

use crate::theme_ext::{resolve_color, resolve_px};

pub struct ListContainer {
    spec: ListContainerSpec,
    theme: GpuiThemeProvider,
    content: Option<AnyElement>,
}

impl ListContainer {
    pub fn from_spec(spec: ListContainerSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), content: None }
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for ListContainer {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let body_size = resolve_px(theme, "typography.body.size");
        let heading_size = resolve_px(theme, "typography.heading.size");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let stack_gap = resolve_px(theme, "space.stack.md");

        let mut container = div().flex().flex_col().gap(stack_gap).w_full();

        // Header
        container = container.child(
            div().flex().flex_col().gap(px(4.0))
                .child(
                    div().text_size(heading_size).font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child(spec.title.clone())
                )
                .when(spec.subtitle.is_some(), |el| {
                    el.child(
                        div().text_size(body_size).text_color(text_secondary)
                            .child(spec.subtitle.clone().unwrap_or_default())
                    )
                })
        );

        // State-dependent content
        match spec.state {
            ListContainerState::Loading => {
                let msg = spec.loading_message.as_deref().unwrap_or("Loading items...");
                container = container.child(
                    div().text_size(body_size).text_color(text_secondary)
                        .child(msg.to_string())
                );
            }
            ListContainerState::Error => {
                let title = spec.error_title.as_deref().unwrap_or("Unable to load list");
                container = container.child(
                    div().text_size(body_size).text_color(resolve_color(theme, "color.status.danger"))
                        .child(title.to_string())
                );
            }
            ListContainerState::Empty => {
                let title = spec.empty_title.as_deref().unwrap_or("Nothing here yet");
                container = container.child(
                    div().text_size(body_size).text_color(text_secondary)
                        .child(title.to_string())
                );
            }
            ListContainerState::Ready => {
                if let Some(content) = self.content {
                    container = container.child(content);
                }
            }
        }

        container.into_any_element()
    }
}
