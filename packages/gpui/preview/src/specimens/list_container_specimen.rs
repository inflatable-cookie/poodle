use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, ListContainer};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_node::Node;
use poodle_specs::EyebrowSpec;
use poodle_specs::{ListContainerSpec, ListContainerState};

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Ready with content ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Ready with slots and built-in pagination"),
                    theme,
                ))
                .child(
                    ListContainer::from_spec(
                        ListContainerSpec::new("Team members")
                            .with_subtitle("Manage your team's access and roles.")
                            .with_total_pages(3)
                            .with_total_items(24)
                            .with_page_size(10),
                        theme,
                    )
                    .with_content({
                        let mut content = Node::text("[ List items rendered here ]");
                        content.style.text_size = Some(14.0);
                        content.style.descriptor.text_color = Some(text_secondary);
                        content
                    }),
                ),
        )
        // --- State handling ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("State handling"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(ListContainer::from_spec(
                            ListContainerSpec::new("Loading example")
                                .with_state(ListContainerState::Loading),
                            theme,
                        ))
                        .child(ListContainer::from_spec(
                            ListContainerSpec::new("Error example")
                                .with_state(ListContainerState::Error)
                                .with_error_message("Network request failed. Please try again."),
                            theme,
                        ))
                        .child(ListContainer::from_spec(
                            ListContainerSpec::new("Empty example")
                                .with_state(ListContainerState::Empty)
                                .with_empty_message(
                                    "Create your first team member to get started.",
                                ),
                            theme,
                        )),
                ),
        )
}
