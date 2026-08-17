use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{DockRegion, Eyebrow};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_specs::{DockCollapsedPosture, DockEdge, DockRegionSpec, EyebrowSpec, PanelTabItem};
use poodle_tokens::typed::ColorValue;
use std::sync::Arc;

fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(child)
}

fn panel_body(text: impl Into<String>, color: ColorValue) -> Node {
    let mut body = Node::container();
    {
        let s = &mut body.style;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = 8.0;
        pad.right = 8.0;
        pad.bottom = 8.0;
        pad.left = 8.0;
        s.text_size = Some(12.0);
        s.descriptor.text_color = Some(color);
    }
    body.child(Node::text(text.into()))
}

fn set_text(
    queue: &Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
    key: &'static str,
) -> Arc<dyn Fn(&str) + Send + Sync> {
    let queue = Arc::clone(queue);
    Arc::new(move |value: &str| {
        queue.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: value.to_string(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let active_tab = state
        .specimens
        .text
        .get("dock-active-tab")
        .cloned()
        .unwrap_or_else(|| "explorer".to_string());
    let toggle_collapsed = state.specimens.is_on("dock-toggle-collapsed");
    let toggle_tab = state
        .specimens
        .text
        .get("dock-toggle-tab")
        .cloned()
        .unwrap_or_else(|| "files".to_string());
    let bottom_tab = state
        .specimens
        .text
        .get("dock-bottom-tab")
        .cloned()
        .unwrap_or_else(|| "terminal".to_string());

    let expanded = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
            PanelTabItem::new("search", "Search").with_icon("search"),
            PanelTabItem::new("git", "Source Control").with_icon("git-branch"),
        ],
    )
    .with_value(&active_tab);

    let collapsed = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("inspector", "Inspector").with_icon("list"),
            PanelTabItem::new("browser", "Media Browser").with_icon("folder"),
            PanelTabItem::new("clips", "Clip Editor").with_icon("code"),
        ],
    )
    .with_collapsed(true)
    .with_collapsed_posture(DockCollapsedPosture::IconStrip);

    let toggle = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("files", "Files").with_icon("folder"),
            PanelTabItem::new("outline", "Outline").with_icon("list"),
            PanelTabItem::new("debug", "Debug").with_icon("terminal"),
        ],
    )
    .with_collapsible(true)
    .with_collapsed(toggle_collapsed)
    .with_collapsed_posture(DockCollapsedPosture::IconStrip)
    .with_value(&toggle_tab);

    let bottom = DockRegionSpec::new(
        DockEdge::Bottom,
        vec![
            PanelTabItem::new("terminal", "Terminal").with_icon("terminal"),
            PanelTabItem::new("output", "Output").with_icon("file-text"),
        ],
    )
    .with_value(&bottom_tab);

    let collapse_events = state.node_events.clone();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Flexible dock — expanded (left edge)",
            theme,
            div().h(px(160.0)).flex().child(
                div().w(px(220.0)).h_full().child(
                    DockRegion::from_spec(expanded, theme)
                        .on_tab_change(set_text(&state.node_events, "dock-active-tab"))
                        .with_content(panel_body(
                            format!("Panel content for {active_tab}."),
                            text_secondary,
                        )),
                ),
            ),
        ))
        .child(group(
            "Flexible dock — collapsed icon-strip (left edge)",
            theme,
            div()
                .h(px(100.0))
                .flex()
                .child(DockRegion::from_spec(collapsed, theme).with_content(Node::container())),
        ))
        .child(group(
            "Interactive collapse toggle",
            theme,
            div()
                .h(px(if toggle_collapsed { 60.0 } else { 160.0 }))
                .flex()
                .child(
                    div()
                        .w(px(if toggle_collapsed { 48.0 } else { 220.0 }))
                        .h_full()
                        .child(
                            DockRegion::from_spec(toggle, theme)
                                .on_tab_change(set_text(&state.node_events, "dock-toggle-tab"))
                                .on_collapse_toggle(Arc::new(move |collapsed| {
                                    collapse_events.lock().unwrap().push(
                                        NodeSpecimenEvent::SetToggle {
                                            key: "dock-toggle-collapsed".to_string(),
                                            value: collapsed,
                                        },
                                    );
                                }))
                                .with_content(panel_body(
                                    format!("Active: {toggle_tab}"),
                                    text_secondary,
                                )),
                        ),
                ),
        ))
        .child(group(
            "Bottom edge dock",
            theme,
            div().h(px(140.0)).child(
                DockRegion::from_spec(bottom, theme)
                    .on_tab_change(set_text(&state.node_events, "dock-bottom-tab"))
                    .with_content(panel_body(
                        format!("Bottom panel: {bottom_tab}"),
                        text_secondary,
                    )),
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "dock-region",
        examples,
        |size, theme: &GpuiThemeProvider| {
            let label = format!("{size:?}").to_lowercase();
            DockRegion::from_spec(
                DockRegionSpec::new(
                    DockEdge::Left,
                    vec![PanelTabItem::new(format!("files-{label}"), "Files").with_icon("folder")],
                )
                .with_size(size)
                .with_value(format!("files-{label}")),
                theme,
            )
            .with_content(panel_body("Panel content.", text_secondary))
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            let label = format!("{density:?}").to_lowercase();
            DockRegion::from_spec(
                DockRegionSpec::new(
                    DockEdge::Left,
                    vec![PanelTabItem::new(format!("files-{label}"), "Files").with_icon("folder")],
                )
                .with_density(density)
                .with_value(format!("files-{label}")),
                theme,
            )
            .with_content(panel_body("Panel content.", text_secondary))
            .into_any_element()
        },
    )
}
