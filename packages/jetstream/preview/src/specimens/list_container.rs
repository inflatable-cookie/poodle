//! ListContainer specimen — paginated list with state handling.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::list_container::js_list_container;
use poodle_jetstream_components::theme_ext::*;
use poodle_composites::{ListContainerSpec, ListContainerState};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    let list_content = div().flex_col().gap(4.0)
        .child(label("Item 1").text_color(text_primary).text_size(13.0))
        .child(label("Item 2").text_color(text_primary).text_size(13.0))
        .child(label("Item 3").text_color(text_primary).text_size(13.0));

    div().flex_col().gap(24.0)
        .child(group("With items", secondary,
            js_list_container(
                &ListContainerSpec::new("Recent Projects")
                    .with_subtitle("Showing your most recent work.")
                    .with_current_page(1)
                    .with_total_pages(5),
                theme,
                Some(list_content),
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
            )
        ))
        .child(group("Loading", secondary,
            js_list_container(
                &ListContainerSpec::new("Search Results")
                    .with_state(ListContainerState::Loading)
                    .with_loading_message("Searching..."),
                theme,
                None,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
