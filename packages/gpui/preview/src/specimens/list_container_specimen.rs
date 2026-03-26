use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_composites::{ListContainerSpec, ListContainerState};
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{ListContainer, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(24.0))
        // --- Ready with content ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Ready with slots and built-in pagination"), theme))
                .child(
                    ListContainer::from_spec(
                        ListContainerSpec::new("Team members")
                            .with_subtitle("Manage your team's access and roles.")
                            .with_total_pages(3)
                            .with_total_items(24)
                            .with_page_size(10),
                        theme,
                    )
                    .with_content(
                        div().text_sm().text_color(color_to_hsla(text_secondary))
                            .child("[ List items rendered here ]")
                    )
                )
        )
        // --- State handling ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("State handling"), theme))
                .child(
                    div().flex().flex_col().gap(px(12.0))
                        .child(
                            ListContainer::from_spec(
                                ListContainerSpec::new("Loading example")
                                    .with_state(ListContainerState::Loading),
                                theme,
                            )
                        )
                        .child(
                            ListContainer::from_spec(
                                ListContainerSpec::new("Error example")
                                    .with_state(ListContainerState::Error)
                                    .with_error_message("Network request failed. Please try again."),
                                theme,
                            )
                        )
                        .child(
                            ListContainer::from_spec(
                                ListContainerSpec::new("Empty example")
                                    .with_state(ListContainerState::Empty)
                                    .with_empty_message("Create your first team member to get started."),
                                theme,
                            )
                        )
                )
        )
}
