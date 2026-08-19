use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{DockRegion, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_specs::{
    DockCollapsedPosture, DockEdge, DockEmphasis, DockRegionSpec, DockSizing, EyebrowSpec,
    PanelTabItem,
};
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

fn main_content(label: &str, theme: &GpuiThemeProvider) -> Div {
    div()
        .flex_1()
        .min_w(px(0.0))
        .flex()
        .items_center()
        .justify_center()
        .text_xs()
        .text_color(color_to_hsla(theme.resolve_color("color.text.secondary")))
        .child(label.to_string())
}

fn flex_frame(height: f32, theme: &GpuiThemeProvider, dock: impl IntoElement, main: &str) -> Div {
    div()
        .h(px(height))
        .flex()
        .border_1()
        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
        .rounded(px(8.0))
        .overflow_hidden()
        .child(dock)
        .child(main_content(main, theme))
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

fn flex_items() -> Vec<PanelTabItem> {
    vec![
        PanelTabItem::new("explorer", "Explorer")
            .with_icon("folder")
            .with_closable(true),
        PanelTabItem::new("search", "Search")
            .with_icon("search")
            .with_closable(true),
        PanelTabItem::new("git", "Source Control")
            .with_icon("git-branch")
            .with_closable(false),
    ]
}

fn iconless_items() -> Vec<PanelTabItem> {
    vec![
        PanelTabItem::new("inspector", "Inspector"),
        PanelTabItem::new("browser", "Media Browser"),
        PanelTabItem::new("clips", "Clip Editor"),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let flex_active = state
        .specimens
        .text
        .get("dock-flex-active")
        .cloned()
        .unwrap_or_else(|| "explorer".to_string());
    let interactive_collapsed = state.specimens.is_on("dock-interactive-collapsed");
    let interactive_active = state
        .specimens
        .text
        .get("dock-interactive-active")
        .cloned()
        .unwrap_or_else(|| "files".to_string());
    let bottom_collapsed = state.specimens.is_on("dock-bottom-collapsed");
    let bottom_active = state
        .specimens
        .text
        .get("dock-bottom-active")
        .cloned()
        .unwrap_or_else(|| "terminal".to_string());
    let left_active = state
        .specimens
        .text
        .get("dock-left-active")
        .cloned()
        .unwrap_or_else(|| "explorer".to_string());
    let right_active = state
        .specimens
        .text
        .get("dock-right-active")
        .cloned()
        .unwrap_or_else(|| "outline".to_string());
    let tab_variant_active = state
        .specimens
        .text
        .get("dock-tab-variant-active")
        .cloned()
        .unwrap_or_else(|| "explorer".to_string());

    let interactive_collapse_events = state.node_events.clone();
    let bottom_collapse_events = state.node_events.clone();

    let expanded_spec = DockRegionSpec::new(DockEdge::Left, flex_items())
        .with_value(&flex_active);
    let iconless_spec = DockRegionSpec::new(DockEdge::Left, iconless_items())
        .with_value("inspector");
    let collapsed_spec = DockRegionSpec::new(DockEdge::Left, flex_items())
        .with_collapsed(true)
        .with_collapsed_posture(DockCollapsedPosture::IconStrip)
        .with_value(&flex_active);
    let interactive_spec = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("files", "Files")
                .with_icon("folder")
                .with_closable(true),
            PanelTabItem::new("outline", "Outline")
                .with_icon("list")
                .with_closable(true),
            PanelTabItem::new("debug", "Debug")
                .with_icon("terminal")
                .with_closable(false),
        ],
    )
    .with_collapsible(true)
    .with_collapsed(interactive_collapsed)
    .with_collapsed_posture(DockCollapsedPosture::IconStrip)
    .with_value(&interactive_active);
    let bottom_spec = DockRegionSpec::new(
        DockEdge::Bottom,
        vec![
            PanelTabItem::new("terminal", "Terminal")
                .with_icon("terminal")
                .with_closable(true),
            PanelTabItem::new("output", "Output")
                .with_icon("file-text")
                .with_closable(true),
            PanelTabItem::new("problems", "Problems")
                .with_icon("alert-circle")
                .with_closable(false),
        ],
    )
    .with_collapsible(true)
    .with_collapsed(bottom_collapsed)
    .with_collapsed_posture(DockCollapsedPosture::IconStrip)
    .with_value(&bottom_active);
    let left_spec = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("explorer", "Explorer")
                .with_icon("folder")
                .with_closable(true),
            PanelTabItem::new("search", "Search")
                .with_icon("search")
                .with_closable(true),
            PanelTabItem::new("git", "Source Control")
                .with_icon("git-branch")
                .with_closable(true),
        ],
    )
    .with_can_accept_panel(true)
    .with_aria_label("Left dock")
    .with_value(&left_active);
    let right_spec = DockRegionSpec::new(
        DockEdge::Right,
        vec![PanelTabItem::new("outline", "Outline")
            .with_icon("list")
            .with_closable(true)],
    )
    .with_can_accept_panel(true)
    .with_aria_label("Right dock")
    .with_value(&right_active);
    let static_top = DockRegionSpec::new(
        DockEdge::Top,
        vec![
            PanelTabItem::new("meter", "Meter Strip"),
            PanelTabItem::new("transport", "Transport"),
            PanelTabItem::new("mixer", "Mixer"),
        ],
    )
    .with_sizing(DockSizing::Static);
    let static_left = DockRegionSpec::new(
        DockEdge::Left,
        vec![
            PanelTabItem::new("toolbar", "Toolbar"),
            PanelTabItem::new("inspector", "Inspector"),
        ],
    )
    .with_sizing(DockSizing::Static);

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Expanded side dock",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(flex_frame(
                    160.0,
                    theme,
                    div()
                        .w(px(220.0))
                        .h_full()
                        .child(
                            DockRegion::from_spec(expanded_spec, theme)
                                .with_instance_id("expanded")
                                .on_tab_change(set_text(&state.node_events, "dock-flex-active"))
                                .with_content(panel_body(
                                    format!(
                                        "Panel content for {flex_active}. Tabs are closable and reorderable."
                                    ),
                                    text_secondary,
                                )),
                        ),
                    "Main content area",
                ))
                .child(
                    flex_frame(
                        160.0,
                        theme,
                        div().w(px(180.0)).h_full().child(
                            DockRegion::from_spec(iconless_spec, theme)
                                .with_instance_id("iconless")
                                .with_content(panel_body(
                                    "Panels without icons keep their labels when the strip is squeezed.",
                                    text_secondary,
                                )),
                        ),
                        "Main content area",
                    )
                    .max_w(px(360.0)),
                ),
        ))
        .child(group(
            "Collapse and edge placement",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(flex_frame(
                    100.0,
                    theme,
                    div().h_full().child(
                        DockRegion::from_spec(collapsed_spec, theme)
                            .with_instance_id("collapsed")
                            .on_tab_change(set_text(&state.node_events, "dock-flex-active"))
                            .with_content(Node::container()),
                    ),
                    "Main content area",
                ))
                .child(flex_frame(
                    if interactive_collapsed { 60.0 } else { 160.0 },
                    theme,
                    div()
                        .w(px(if interactive_collapsed { 48.0 } else { 220.0 }))
                        .h_full()
                        .child(
                            DockRegion::from_spec(interactive_spec, theme)
                                .with_instance_id("interactive")
                                .on_tab_change(set_text(
                                    &state.node_events,
                                    "dock-interactive-active",
                                ))
                                .on_collapse_toggle(Arc::new(move |collapsed| {
                                    interactive_collapse_events
                                        .lock()
                                        .unwrap()
                                        .push(NodeSpecimenEvent::SetToggle {
                                            key: "dock-interactive-collapsed".to_string(),
                                            value: collapsed,
                                        });
                                }))
                                .with_content(panel_body(
                                    format!(
                                        "{interactive_active}: click the collapse toggle to switch between expanded and icon-strip modes."
                                    ),
                                    text_secondary,
                                )),
                        ),
                    "Main content area",
                ))
                .child(
                    div()
                        .h(px(220.0))
                        .flex()
                        .flex_col()
                        .border_1()
                        .border_color(color_to_hsla(
                            theme.resolve_color("color.border.subtle"),
                        ))
                        .rounded(px(8.0))
                        .overflow_hidden()
                        .child(main_content("Editor area", theme).flex_1())
                        .child(
                            DockRegion::from_spec(bottom_spec, theme)
                                .with_instance_id("bottom")
                                .on_tab_change(set_text(&state.node_events, "dock-bottom-active"))
                                .on_collapse_toggle(Arc::new(move |collapsed| {
                                    bottom_collapse_events
                                        .lock()
                                        .unwrap()
                                        .push(NodeSpecimenEvent::SetToggle {
                                            key: "dock-bottom-collapsed".to_string(),
                                            value: collapsed,
                                        });
                                }))
                                .with_content(panel_body(
                                    format!(
                                        "{bottom_active}: bottom panel content. Collapses downward, keeping horizontal tabs."
                                    ),
                                    text_secondary,
                                )),
                        ),
                ),
        ))
        .child(group(
            "Tab strip presentation",
            theme,
            div()
                .grid()
                .gap(px(16.0))
                .grid_cols(3)
                .child(
                    div()
                        .h(px(180.0))
                        .border_1()
                        .border_color(color_to_hsla(
                            theme.resolve_color("color.border.subtle"),
                        ))
                        .rounded(px(8.0))
                        .overflow_hidden()
                        .child(
                            DockRegion::from_spec(
                                DockRegionSpec::new(DockEdge::Left, flex_items())
                                    .with_value(&tab_variant_active)
                                    .with_emphasis(DockEmphasis::Standard),
                                theme,
                            )
                            .with_instance_id("tab-standard")
                            .on_tab_change(set_text(
                                &state.node_events,
                                "dock-tab-variant-active",
                            ))
                            .with_content(panel_body(
                                format!(
                                    "{tab_variant_active}: standard emphasis — default active-tab tint."
                                ),
                                text_secondary,
                            )),
                        ),
                )
                .child(
                    div()
                        .h(px(180.0))
                        .border_1()
                        .border_color(color_to_hsla(
                            theme.resolve_color("color.border.subtle"),
                        ))
                        .rounded(px(8.0))
                        .overflow_hidden()
                        .child(
                            DockRegion::from_spec(
                                DockRegionSpec::new(DockEdge::Left, flex_items())
                                    .with_value(&tab_variant_active)
                                    .with_emphasis(DockEmphasis::Quiet),
                                theme,
                            )
                            .with_instance_id("tab-quiet")
                            .on_tab_change(set_text(
                                &state.node_events,
                                "dock-tab-variant-active",
                            ))
                            .with_content(panel_body(
                                format!(
                                    "{tab_variant_active}: quiet emphasis — transparent chrome."
                                ),
                                text_secondary,
                            )),
                        ),
                )
                .child(
                    div()
                        .h(px(180.0))
                        .border_1()
                        .border_color(color_to_hsla(
                            theme.resolve_color("color.border.subtle"),
                        ))
                        .rounded(px(8.0))
                        .overflow_hidden()
                        .child(
                            DockRegion::from_spec(
                                DockRegionSpec::new(DockEdge::Left, flex_items())
                                    .with_value(&tab_variant_active)
                                    .with_emphasis(DockEmphasis::Strong)
                                    .with_can_accept_panel(true),
                                theme,
                            )
                            .with_instance_id("tab-strong")
                            .on_tab_change(set_text(
                                &state.node_events,
                                "dock-tab-variant-active",
                            ))
                            .with_content(panel_body(
                                format!(
                                    "{tab_variant_active}: strong emphasis with drop-target affordance."
                                ),
                                text_secondary,
                            )),
                        ),
                ),
        ))
        .child(group(
            "Move panels between docks",
            theme,
            div()
                .grid()
                .gap(px(16.0))
                .grid_cols(2)
                .child(
                    div()
                        .h(px(180.0))
                        .border_1()
                        .border_color(color_to_hsla(
                            theme.resolve_color("color.border.subtle"),
                        ))
                        .rounded(px(8.0))
                        .overflow_hidden()
                        .child(
                            DockRegion::from_spec(left_spec, theme)
                                .with_instance_id("dnd-left")
                                .on_tab_change(set_text(&state.node_events, "dock-left-active"))
                                .with_content(panel_body(
                                    format!("Left dock — active: {left_active}"),
                                    text_secondary,
                                )),
                        ),
                )
                .child(
                    div()
                        .h(px(180.0))
                        .border_1()
                        .border_color(color_to_hsla(
                            theme.resolve_color("color.border.subtle"),
                        ))
                        .rounded(px(8.0))
                        .overflow_hidden()
                        .child(
                            DockRegion::from_spec(right_spec, theme)
                                .with_instance_id("dnd-right")
                                .on_tab_change(set_text(&state.node_events, "dock-right-active"))
                                .with_content(panel_body(
                                    format!("Right dock — active: {right_active}"),
                                    text_secondary,
                                )),
                        ),
                ),
        ))
        .child(group(
            "Static panel stacks",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(
                    div()
                        .h(px(72.0))
                        .border_1()
                        .border_color(color_to_hsla(
                            theme.resolve_color("color.border.subtle"),
                        ))
                        .rounded(px(8.0))
                        .overflow_hidden()
                        .child(DockRegion::from_spec(static_top, theme).with_instance_id(
                            "static-top",
                        )),
                )
                .child(
                    div()
                        .h(px(160.0))
                        .border_1()
                        .border_color(color_to_hsla(
                            theme.resolve_color("color.border.subtle"),
                        ))
                        .rounded(px(8.0))
                        .overflow_hidden()
                        .child(
                            DockRegion::from_spec(static_left, theme)
                                .with_instance_id("static-left"),
                        ),
                ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "dock-region",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let label = format!("{size:?}").to_lowercase();
                flex_frame(
                    160.0,
                    theme,
                    div().w(px(220.0)).h_full().child(
                        DockRegion::from_spec(
                            DockRegionSpec::new(
                                DockEdge::Left,
                                vec![PanelTabItem::new(format!("git-{label}"), "Source Control")
                                    .with_icon("git-branch")],
                            )
                            .with_size(size)
                            .with_value(format!("git-{label}")),
                            theme,
                        )
                        .with_content(panel_body("Panel content.", text_secondary)),
                    ),
                    "Main content area",
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let label = format!("{density:?}").to_lowercase();
                flex_frame(
                    160.0,
                    theme,
                    div().w(px(220.0)).h_full().child(
                        DockRegion::from_spec(
                            DockRegionSpec::new(
                                DockEdge::Left,
                                vec![PanelTabItem::new(format!("git-{label}"), "Source Control")
                                    .with_icon("git-branch")],
                            )
                            .with_density(density)
                            .with_value(format!("git-{label}")),
                            theme,
                        )
                        .with_content(panel_body("Panel content.", text_secondary)),
                    ),
                    "Main content area",
                )
                .into_any_element()
            }),
    )
}
