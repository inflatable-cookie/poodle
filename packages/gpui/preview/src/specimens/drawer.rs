use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Button, CompatRow, Drawer, Eyebrow};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_specs::{ButtonSpec, ButtonVariant, DrawerEdge, DrawerSpec, EyebrowSpec};
use std::sync::Arc;

fn set_toggle_click(
    state: &AppState,
    key: impl Into<String>,
    value: bool,
) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    let key = key.into();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.clone(),
            value,
        });
    })
}

fn set_toggle_open_change(
    state: &AppState,
    key: impl Into<String>,
) -> Arc<dyn Fn(bool) + Send + Sync> {
    let events = state.node_events.clone();
    let key = key.into();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.clone(),
            value,
        });
    })
}

fn body_copy(theme: &impl ThemeProvider, text: impl Into<String>) -> Node {
    let mut node = Node::text(text);
    node.style.text_size = Some(14.0);
    node.style.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
    node
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let trigger = |id: &'static str, key: &'static str, label: &'static str| {
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label(label),
            theme,
        )
        .with_id(id)
        .on_click(set_toggle_click(state, key, true))
        .into_any_element()
    };

    let mut root = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Right edge (default)"),
                    theme,
                ))
                .child(trigger(
                    "drawer-right-trigger",
                    "drawer-right-open",
                    "Open right drawer",
                )),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Left edge"),
                    theme,
                ))
                .child(trigger(
                    "drawer-left-trigger",
                    "drawer-left-open",
                    "Open left drawer",
                )),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Top edge"),
                    theme,
                ))
                .child(trigger(
                    "drawer-top-trigger",
                    "drawer-top-open",
                    "Open top drawer",
                )),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Bottom edge"),
                    theme,
                ))
                .child(trigger(
                    "drawer-bottom-trigger",
                    "drawer-bottom-open",
                    "Open bottom drawer",
                )),
        );

    if state.specimens.is_on("drawer-right-open") {
        root = root.child(
            Drawer::from_spec(
                DrawerSpec::new()
                    .with_title("Settings")
                    .with_description("Configure your preferences."),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "drawer-right-open"))
            .with_content(body_copy(
                theme,
                "Drawer content goes here. You can put forms, navigation, or any other content.",
            ))
            .with_actions(
                CompatRow::new()
                    .gap(6.0)
                    .justify_end()
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Cancel"),
                            theme,
                        )
                        .with_id("drawer-cancel")
                        .on_click(set_toggle_click(
                            state,
                            "drawer-right-open",
                            false,
                        )),
                    )
                    .child(
                        Button::from_spec(ButtonSpec::new().with_label("Save"), theme)
                            .with_id("drawer-save")
                            .on_click(set_toggle_click(state, "drawer-right-open", false)),
                    ),
            ),
        );
    }

    if state.specimens.is_on("drawer-left-open") {
        root = root.child(
            Drawer::from_spec(
                DrawerSpec::new()
                    .with_edge(DrawerEdge::Left)
                    .with_title("Navigation"),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "drawer-left-open"))
            .with_content(body_copy(
                theme,
                "Side navigation or filters can live in a left-edge drawer.",
            )),
        );
    }

    if state.specimens.is_on("drawer-top-open") {
        root = root.child(
            Drawer::from_spec(
                DrawerSpec::new()
                    .with_edge(DrawerEdge::Top)
                    .with_title("Notifications")
                    .with_description("Recent activity slides down from the top edge."),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "drawer-top-open"))
            .with_content(body_copy(
                theme,
                "Top-anchored drawers span the full width and are useful for banners or alerts.",
            ))
            .with_actions(
                CompatRow::new().justify_end().child(
                    Button::from_spec(ButtonSpec::new().with_label("Dismiss"), theme)
                        .with_id("drawer-top-dismiss")
                        .on_click(set_toggle_click(state, "drawer-top-open", false)),
                ),
            ),
        );
    }

    if state.specimens.is_on("drawer-bottom-open") {
        root = root.child(
            Drawer::from_spec(
                DrawerSpec::new()
                    .with_edge(DrawerEdge::Bottom)
                    .with_title("Quick actions")
                    .with_description("A bottom sheet anchored to the lower edge."),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "drawer-bottom-open"))
            .with_content(body_copy(
                theme,
                "Bottom-anchored drawers span the full width and rise from the lower edge.",
            ))
            .with_actions(
                CompatRow::new()
                    .gap(6.0)
                    .justify_end()
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Cancel"),
                            theme,
                        )
                        .with_id("drawer-bottom-cancel")
                        .on_click(set_toggle_click(
                            state,
                            "drawer-bottom-open",
                            false,
                        )),
                    )
                    .child(
                        Button::from_spec(ButtonSpec::new().with_label("Apply"), theme)
                            .with_id("drawer-bottom-apply")
                            .on_click(set_toggle_click(state, "drawer-bottom-open", false)),
                    ),
            ),
        );
    }
    let examples = root.into_any_element();

    specimen_layout(
        state,
        cx,
        "drawer",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let open_key = format!("drawer-axis-size-{}", size_key(size));
                let mut row = div().flex().flex_col().gap(px(8.0)).child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label(format!("Open {} drawer", size_key(size))),
                        theme,
                    )
                    .with_id(format!("drawer-axis-size-{}-trigger", size_key(size)))
                    .on_click(set_toggle_click(state, open_key.clone(), true)),
                );
                if state.specimens.is_on(&open_key) {
                    row = row.child(
                        Drawer::from_spec(
                            DrawerSpec::new()
                                .with_title("Settings")
                                .with_description("Configure your preferences.")
                                .with_size(size),
                            theme,
                        )
                        .on_open_change(set_toggle_open_change(state, open_key.clone()))
                        .with_content(body_copy(
                            theme,
                            "Drawer content goes here. You can put forms, navigation, or any other content.",
                        )),
                    );
                }
                row.into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let open_key = format!("drawer-axis-density-{}", density_key(density));
                let mut row = div().flex().flex_col().gap(px(8.0)).child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label(format!("Open {} drawer", density_key(density))),
                        theme,
                    )
                    .with_id(format!("drawer-axis-density-{}-trigger", density_key(density)))
                    .on_click(set_toggle_click(state, open_key.clone(), true)),
                );
                if state.specimens.is_on(&open_key) {
                    row = row.child(
                        Drawer::from_spec(
                            DrawerSpec::new()
                                .with_title("Settings")
                                .with_description("Configure your preferences.")
                                .with_density(density),
                            theme,
                        )
                        .on_open_change(set_toggle_open_change(state, open_key.clone()))
                        .with_content(body_copy(
                            theme,
                            "Drawer content goes here. You can put forms, navigation, or any other content.",
                        )),
                    );
                }
                row.into_any_element()
            }),
    )
}
