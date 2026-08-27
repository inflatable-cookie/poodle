use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Breadcrumbs, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{BreadcrumbItem, BreadcrumbsSpec, EyebrowSpec};
use std::sync::Arc;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_nav = state
        .specimens
        .text
        .get("breadcrumbs-nav")
        .cloned()
        .unwrap_or_default();
    let events = Arc::clone(&state.node_events);

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Basic ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Basic"),
                    theme,
                ))
                .child(
                    Breadcrumbs::from_spec(
                        BreadcrumbsSpec::new(vec![
                            BreadcrumbItem::new("home", "Home"),
                            BreadcrumbItem::new("projects", "Projects"),
                            BreadcrumbItem::new("poodle", "Poodle").with_is_current(true),
                        ]),
                        theme,
                    )
                    .on_navigate(Arc::new(move |value: &str| {
                        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                            key: "breadcrumbs-nav".to_string(),
                            value: value.to_string(),
                        });
                    })),
                )
                .when(!last_nav.is_empty(), |el| {
                    el.child(
                        div()
                            .text_xs()
                            .text_color(color_to_hsla(text_secondary))
                            .child(format!("Navigated to: {last_nav}")),
                    )
                }),
        )
        // --- Icons ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Icons"),
                    theme,
                ))
                .child(Breadcrumbs::from_spec(
                    BreadcrumbsSpec::new(vec![
                        // Icon-only root: no visible text, still named "Home".
                        BreadcrumbItem::new("home", "Home").with_icon_only("home"),
                        BreadcrumbItem::new("projects", "Projects").with_icon("folder"),
                        BreadcrumbItem::new("poodle", "Poodle")
                            .with_icon("package")
                            .with_is_current(true),
                    ]),
                    theme,
                )),
        )
        // --- Deep path ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Deep path"),
                    theme,
                ))
                .child(Breadcrumbs::from_spec(
                    BreadcrumbsSpec::new(vec![
                        BreadcrumbItem::new("home", "Home"),
                        BreadcrumbItem::new("workspace", "Workspace"),
                        BreadcrumbItem::new("projects", "Projects"),
                        BreadcrumbItem::new("poodle", "Poodle Design System"),
                        BreadcrumbItem::new("primitives", "Primitives"),
                        BreadcrumbItem::new("button", "Button").with_is_current(true),
                    ]),
                    theme,
                )),
        )
        // --- Collapsed (max 3 visible) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Collapsed (max 3 visible)"),
                    theme,
                ))
                .child(Breadcrumbs::from_spec(
                    BreadcrumbsSpec::new(vec![
                        BreadcrumbItem::new("home", "Home"),
                        BreadcrumbItem::new("workspace", "Workspace"),
                        BreadcrumbItem::new("projects", "Projects"),
                        BreadcrumbItem::new("poodle", "Poodle Design System"),
                        BreadcrumbItem::new("primitives", "Primitives"),
                        BreadcrumbItem::new("button", "Button").with_is_current(true),
                    ])
                    .with_max_visible_items(3),
                    theme,
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "breadcrumbs",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                Breadcrumbs::from_spec(
                    BreadcrumbsSpec::new(vec![
                        BreadcrumbItem::new("home", "Home"),
                        BreadcrumbItem::new("docs", "Docs"),
                        BreadcrumbItem::new("page", "Page").with_is_current(true),
                    ]),
                    theme,
                )
                .size(size)
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                Breadcrumbs::from_spec(
                    BreadcrumbsSpec::new(vec![
                        BreadcrumbItem::new("home", "Home"),
                        BreadcrumbItem::new("docs", "Docs"),
                        BreadcrumbItem::new("page", "Page").with_is_current(true),
                    ]),
                    theme,
                )
                .with_density(density)
                .into_any_element()
            }),
    )
}
