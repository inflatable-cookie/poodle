use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Breadcrumbs, Eyebrow, IntoCompatNode, PageHeader, Pill};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node};
use poodle_render::context::RenderContext;
use poodle_render::icon_button;
use poodle_specs::PageHeaderSpec;
use poodle_specs::{
    BreadcrumbItem, BreadcrumbsSpec, ButtonVariant, EyebrowSpec, IconButtonSpec, PillAppearance,
    PillSpec, PillTone, StatusTone,
};
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

fn icon_action(
    theme: &GpuiThemeProvider,
    icon: &str,
    aria_label: &str,
    action_key: &'static str,
    action_value: &'static str,
    events: &Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
) -> Node {
    let events = Arc::clone(events);
    let icon = icon.to_string();
    let aria_label = aria_label.to_string();
    icon_button(
        &IconButtonSpec::new()
            .with_icon(icon)
            .with_aria_label(aria_label)
            .with_variant(ButtonVariant::Secondary),
        &RenderContext::new(theme),
        Some(Arc::new(move || {
            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: action_key.to_string(),
                value: action_value.to_string(),
            });
        })),
    )
}

fn icon_actions(
    theme: &GpuiThemeProvider,
    action_key: &'static str,
    items: &[(&'static str, &'static str, &'static str)],
    events: &Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.spacing.gap = 6.0;
    for (icon, aria, value) in items {
        row = row.child(icon_action(theme, icon, aria, action_key, value, events));
    }
    row
}

fn last_action_hint(theme: &GpuiThemeProvider, action: &str) -> Div {
    div().when(!action.is_empty(), |d| {
        d.child(
            div()
                .text_sm()
                .text_color(color_to_hsla(theme.resolve_color("color.text.secondary")))
                .child(format!("Last action: {action}")),
        )
    })
}

fn meta(theme: &GpuiThemeProvider) -> Node {
    let secondary = theme.resolve_color("color.text.secondary");
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.style.descriptor.layout.spacing.gap = 12.0;
    row = row.child(
        Pill::from_spec(
            PillSpec::new()
                .with_label("Active")
                .with_tone(PillTone::Success)
                .with_appearance(PillAppearance::Badge),
            theme,
        )
        .into_compat_node(),
    );
    let mut every = Node::text("Every 6 hours");
    every.style.text_size = Some(13.0);
    every.style.descriptor.text_color = Some(secondary);
    row = row.child(every);
    let mut last = Node::container();
    last.style.descriptor.layout.direction = LayoutDirection::Row;
    last.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    last.style.descriptor.layout.spacing.gap = 4.0;
    let mut last_label = Node::text("Last run");
    last_label.style.text_size = Some(13.0);
    last_label.style.descriptor.text_color = Some(secondary);
    row.child(
        last.child(last_label).child({
            let mut last_value = Node::text("4mo ago");
            last_value.style.text_size = Some(13.0);
            last_value.style.descriptor.text_color = Some(secondary);
            last_value
        }),
    )
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let events = state.node_events.clone();
    let nav_action = state
        .specimens
        .text
        .get("page-header-nav-action")
        .cloned()
        .unwrap_or_default();
    let hierarchy_action = state
        .specimens
        .text
        .get("page-header-hierarchy-action")
        .cloned()
        .unwrap_or_default();
    let status_action = state
        .specimens
        .text
        .get("page-header-status-action")
        .cloned()
        .unwrap_or_default();
    let meta_action = state
        .specimens
        .text
        .get("page-header-meta-action")
        .cloned()
        .unwrap_or_default();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Page title and summary",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(PageHeader::from_spec(
                    PageHeaderSpec::new("Components")
                        .with_subtitle("Browse and manage your component library."),
                    theme,
                ))
                .child(PageHeader::from_spec(
                    PageHeaderSpec::new("Settings"),
                    theme,
                )),
        ))
        .child(
            group(
                "Navigation and actions",
                theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        PageHeader::from_spec(
                            PageHeaderSpec::new("Media Library")
                                .with_subtitle("Browse, review, and manage uploaded files.")
                                .with_back("/dashboard", "Dashboard"),
                            theme,
                        )
                        .with_actions(icon_actions(
                            theme,
                            "page-header-nav-action",
                            &[
                                ("upload", "Upload", "Upload"),
                                ("settings", "Settings", "Settings"),
                            ],
                            &events,
                        )),
                    )
                    .child(
                        PageHeader::from_spec(
                            PageHeaderSpec::new("Cash flow forecasts")
                                .with_section("Module")
                                .with_subtitle("Manage content and ordering for this module.")
                                .with_back("/learning/pathways", "Pathways"),
                            theme,
                        )
                        .with_breadcrumbs(Breadcrumbs::from_spec(
                            BreadcrumbsSpec::new(vec![
                                BreadcrumbItem::new("pathways", "Pathways"),
                                BreadcrumbItem::new("foundation", "Foundation"),
                                BreadcrumbItem::new("module", "Module"),
                            ]),
                            theme,
                        ))
                        .with_actions(icon_actions(
                            theme,
                            "page-header-nav-action",
                            &[
                                ("upload", "Upload", "Upload module"),
                                ("settings", "Settings", "Settings module"),
                            ],
                            &events,
                        )),
                    )
                    .child(last_action_hint(theme, &nav_action)),
            ),
        )
        .child(
            group(
                "Hierarchy and count",
                theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .child(
                        PageHeader::from_spec(
                            PageHeaderSpec::new("Button")
                                .with_eyebrow("Primitive")
                                .with_subtitle(
                                    "Primary interactive control for triggering actions.",
                                ),
                            theme,
                        )
                        .with_actions(icon_actions(
                            theme,
                            "page-header-hierarchy-action",
                            &[
                                ("code", "View source", "View source"),
                                ("pencil", "Edit", "Edit"),
                            ],
                            &events,
                        )),
                    )
                    .child(PageHeader::from_spec(
                        PageHeaderSpec::new("Users")
                            .with_count(128)
                            .with_back("/dashboard", "Dashboard"),
                        theme,
                    ))
                    .child(last_action_hint(theme, &hierarchy_action)),
            ),
        )
        .child(
            group(
                "Contextual status",
                theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        PageHeader::from_spec(
                            PageHeaderSpec::new("Nightly Sync")
                                .with_section("Scheduled Task")
                                .with_back("/system/tasks", "Tasks")
                                .with_back_is_contextual(true)
                                .with_banner("This task is currently paused.", StatusTone::Warning),
                            theme,
                        )
                        .with_actions(icon_actions(
                            theme,
                            "page-header-status-action",
                            &[
                                ("play", "Run now", "Run now"),
                                ("pencil", "Edit", "Edit task"),
                            ],
                            &events,
                        )),
                    )
                    .child(last_action_hint(theme, &status_action)),
            ),
        )
        .child(
            group(
                "Operational metadata",
                theme,
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        PageHeader::from_spec(
                            PageHeaderSpec::new("Nightly Sync")
                                .with_section("Scheduled Task")
                                .with_back("/system/tasks", "Tasks"),
                            theme,
                        )
                        .with_meta(meta(theme))
                        .with_actions(icon_actions(
                            theme,
                            "page-header-meta-action",
                            &[
                                ("play", "Run now", "Run now"),
                                ("calendar", "Edit schedule", "Edit schedule"),
                            ],
                            &events,
                        )),
                    )
                    .child(last_action_hint(theme, &meta_action)),
            ),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "page-header",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                PageHeader::from_spec(
                    PageHeaderSpec::new("Media Library")
                        .with_subtitle("Browse, review, and manage uploaded files.")
                        .with_back("/dashboard", "Dashboard")
                        .with_size(size),
                    theme,
                )
                .with_actions(icon_actions(
                    theme,
                    "page-header-nav-action",
                    &[
                        ("upload", "Upload", "Upload"),
                        ("settings", "Settings", "Settings"),
                    ],
                    &Arc::new(std::sync::Mutex::new(Vec::new())),
                ))
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                PageHeader::from_spec(
                    PageHeaderSpec::new("Media Library")
                        .with_subtitle("Browse, review, and manage uploaded files.")
                        .with_back("/dashboard", "Dashboard")
                        .with_density(density),
                    theme,
                )
                .with_actions(icon_actions(
                    theme,
                    "page-header-nav-action",
                    &[
                        ("upload", "Upload", "Upload"),
                        ("settings", "Settings", "Settings"),
                    ],
                    &Arc::new(std::sync::Mutex::new(Vec::new())),
                ))
                .into_any_element()
            }),
    )
}
