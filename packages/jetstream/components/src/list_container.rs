//! ListContainer — Jetstream paginated list view with header, pagination, and state handling
//! backed by ListContainerSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::{ListContainerSpec, ListContainerState};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

pub fn js_list_container(
    spec: &ListContainerSpec,
    theme: &JetstreamThemeProvider,
    content: Option<JsEl>,
) -> JsEl {
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let danger_color = resolve_color(theme, "semantic.color.status.danger");
    let stack_gap = resolve_px(theme, "semantic.space.stack.md");

    let heading_size = rem_to_px(1.125);
    let body_size = rem_to_px(0.8125);
    let eyebrow_size = rem_to_px(0.6875);

    let mut container = ui_element::div().flex_col().gap(stack_gap).grow();

    // Header
    let mut header = ui_element::div().flex_col().gap(rem_to_px(0.25));

    if let Some(ref eyebrow) = spec.eyebrow {
        header = header.child(
            ui_element::label(eyebrow)
                .text_color(text_secondary).text_size(eyebrow_size).text_weight(600)
        );
    }

    header = header.child(
        ui_element::label(&spec.title)
            .text_color(text_primary).text_size(heading_size).text_weight(600)
    );

    if let Some(ref subtitle) = spec.subtitle {
        header = header.child(
            ui_element::label(subtitle)
                .text_color(text_secondary).text_size(body_size)
        );
    }

    container = container.child(header);

    // State-dependent body
    match spec.state {
        ListContainerState::Loading => {
            let msg = spec.loading_message.as_deref().unwrap_or("Loading items...");
            container = container.child(
                ui_element::div()
                    .flex_row().items_center().gap(rem_to_px(0.5))
                    .child(ui_element::icon("loader").w(rem_to_px(1.0)).h(rem_to_px(1.0)).text_color(text_secondary))
                    .child(ui_element::label(msg).text_color(text_secondary).text_size(body_size))
            );
        }
        ListContainerState::Error => {
            let title = spec.error_title.as_deref().unwrap_or("Unable to load list");
            let mut error_el = ui_element::div().flex_col().gap(rem_to_px(0.25));
            error_el = error_el.child(
                ui_element::label(title).text_color(danger_color).text_size(body_size).text_weight(600)
            );
            if let Some(ref msg) = spec.error_message {
                error_el = error_el.child(
                    ui_element::label(msg).text_color(text_secondary).text_size(body_size)
                );
            }
            container = container.child(error_el);
        }
        ListContainerState::Empty => {
            let title = spec.empty_title.as_deref().unwrap_or("Nothing here yet");
            let mut empty_el = ui_element::div()
                .flex_col().items_center().justify_center().gap(rem_to_px(0.5))
                .pt(rem_to_px(2.0)).pb(rem_to_px(2.0));
            empty_el = empty_el.child(
                ui_element::label(title).text_color(text_secondary).text_size(body_size)
            );
            if let Some(ref msg) = spec.empty_message {
                empty_el = empty_el.child(
                    ui_element::label(msg).text_color(text_secondary).text_size(rem_to_px(0.75))
                );
            }
            container = container.child(empty_el);
        }
        ListContainerState::Ready => {
            if let Some(c) = content {
                container = container.child(c);
            }
        }
    }

    // Pagination summary
    if spec.show_pagination && spec.total_pages > 1 {
        let page_text = format!("Page {} of {}", spec.current_page, spec.total_pages);
        container = container.child(
            ui_element::div()
                .flex_row().justify_center()
                .pt(rem_to_px(0.5))
                .child(ui_element::label(&page_text).text_color(text_secondary).text_size(rem_to_px(0.75)))
        );
    }

    container
}
