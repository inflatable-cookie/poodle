//! ListContainer specimen — paginated list with state handling.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::list_container::js_list_container;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ControlSize, ListContainerSpec, ListContainerState};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    let rows = || {
        div().flex_col().gap(4.0)
            .child(label("Item 1").text_color(text_primary).text_size(body_font))
            .child(label("Item 2").text_color(text_primary).text_size(body_font))
            .child(label("Item 3").text_color(text_primary).text_size(body_font))
    };

    div().flex_col().gap(24.0)
        // Ready with pagination summary + controls.
        .child(group("Ready with pagination", secondary,
            js_list_container(
                &ListContainerSpec::new("Recent Projects")
                    .with_subtitle("Showing your most recent work.")
                    .with_current_page(2)
                    .with_total_pages(5)
                    .with_total_items(48)
                    .with_page_size(10),
                theme,
                Some(rows()),
                None,
                None,
            )
        ))
        // Ready with filters + batch slots.
        .child(group("Filters and batch", secondary,
            js_list_container(
                &ListContainerSpec::new("Team members")
                    .with_eyebrow("Settings"),
                theme,
                Some(rows()),
                Some(label("Filter toolbar").text_color(secondary).text_size(body_font)),
                Some(label("3 selected").text_color(secondary).text_size(body_font)),
            )
        ))
        .child(group("Empty", secondary,
            js_list_container(
                &ListContainerSpec::new("Assets")
                    .with_state(ListContainerState::Empty)
                    .with_empty_title("No assets yet")
                    .with_empty_message("Upload your first asset to get started."),
                theme,
                None,
                None,
                None,
            )
        ))
        .child(group("Loading", secondary,
            js_list_container(
                &ListContainerSpec::new("Search Results")
                    .with_state(ListContainerState::Loading)
                    .with_loading_message("Searching..."),
                theme,
                None,
                None,
                None,
            )
        ))
        .child(group("Error", secondary,
            js_list_container(
                &ListContainerSpec::new("Reports")
                    .with_state(ListContainerState::Error)
                    .with_error_message("A network error occurred. Please try again."),
                theme,
                None,
                None,
                None,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
