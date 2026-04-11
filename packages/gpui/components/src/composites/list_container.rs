//! ListContainer — paginated list view with header and state handling.

use gpui::*;
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

        let label_size = resolve_px(theme, "typography.label.size");
        let caption_size = resolve_px(theme, "typography.caption.size");
        let header_gap = resolve_px(theme, "space.stack.sm");

        let mut container = div().flex().flex_col().gap(stack_gap).w_full();

        // Header — eyebrow → title → subtitle
        let mut header = div().flex().flex_col().gap(header_gap);

        if let Some(ref eyebrow) = spec.eyebrow {
            header = header.child(
                div()
                    .text_size(label_size)
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(text_secondary)
                    .child(eyebrow.clone()),
            );
        }

        header = header.child(
            div()
                .text_size(heading_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(spec.title.clone()),
        );

        if let Some(ref subtitle) = spec.subtitle {
            header = header.child(
                div()
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .child(subtitle.clone()),
            );
        }

        container = container.child(header);

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
                let danger = resolve_color(theme, "color.status.danger");
                let mut block = div().flex().flex_col().gap(header_gap)
                    .child(
                        div()
                            .text_size(body_size)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(danger)
                            .child(title.to_string()),
                    );
                if let Some(ref msg) = spec.error_message {
                    block = block.child(
                        div()
                            .text_size(body_size)
                            .text_color(text_secondary)
                            .child(msg.clone()),
                    );
                }
                container = container.child(block);
            }
            ListContainerState::Empty => {
                let title = spec.empty_title.as_deref().unwrap_or("Nothing here yet");
                let mut block = div().flex().flex_col().gap(header_gap)
                    .child(
                        div()
                            .text_size(body_size)
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text_primary)
                            .child(title.to_string()),
                    );
                if let Some(ref msg) = spec.empty_message {
                    block = block.child(
                        div()
                            .text_size(body_size)
                            .text_color(text_secondary)
                            .child(msg.clone()),
                    );
                }
                container = container.child(block);
            }
            ListContainerState::Ready => {
                if let Some(content) = self.content {
                    container = container.child(content);
                }
            }
        }

        // Pagination summary row — rendered when the spec requests
        // pagination visibility and the container is in its Ready state.
        // The contract currently surfaces no pager controls at the GPUI
        // level, so this renders the summary text only: "Page X of Y"
        // and "Showing N items" / "Showing N of M". A full pager primitive
        // would go here once one exists.
        if spec.show_pagination && spec.state == ListContainerState::Ready && spec.total_pages > 0 {
            let page_text = format!("Page {} of {}", spec.current_page.max(1), spec.total_pages);
            let items_text = match (spec.page_size, spec.total_items) {
                (_, Some(total)) => {
                    if let Some(size) = spec.page_size {
                        let start = (spec.current_page.saturating_sub(1)) * size + 1;
                        let end = (start + size - 1).min(total);
                        format!("Showing {}–{} of {}", start.min(total.max(1)), end, total)
                    } else {
                        format!("{} items total", total)
                    }
                }
                _ => String::new(),
            };
            let mut row = div()
                .flex()
                .flex_row()
                .justify_between()
                .items_center()
                .w_full()
                .child(
                    div()
                        .text_size(caption_size)
                        .text_color(text_secondary)
                        .child(page_text),
                );
            if !items_text.is_empty() {
                row = row.child(
                    div()
                        .text_size(caption_size)
                        .text_color(text_secondary)
                        .child(items_text),
                );
            }
            container = container.child(row);
        }

        container.into_any_element()
    }
}
