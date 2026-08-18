use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{ActionDiscoveryPanel, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ActionDiscoveryPanelSpec, ActionDiscoverySection, CommandActionItem, DiscoveryState,
    EyebrowSpec,
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

fn framed(theme: &GpuiThemeProvider, inner: impl IntoElement) -> Div {
    div()
        .max_w(px(512.0))
        .max_h(px(320.0))
        .border_1()
        .border_color(color_to_hsla(theme.resolve_color("color.border.subtle")))
        .rounded(px(8.0))
        .p(px(12.0))
        .overflow_hidden()
        .child(inner)
}

fn compact_panel() -> ActionDiscoveryPanelSpec {
    ActionDiscoveryPanelSpec::new(vec![ActionDiscoverySection::new(
        "file",
        "File",
        vec![
            CommandActionItem::new("save", "Save").with_shortcut("⌘S"),
            CommandActionItem::new("open-file", "Open File").with_shortcut("⌘O"),
        ],
    )])
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let active = state
        .specimens
        .text
        .get("action-discovery-active")
        .cloned()
        .unwrap_or_else(|| "save".to_string());
    let events = state.node_events.clone();

    let grouped = ActionDiscoveryPanelSpec::new(vec![
        ActionDiscoverySection::new(
            "file",
            "File",
            vec![
                CommandActionItem::new("save", "Save").with_shortcut("⌘S"),
                CommandActionItem::new("open-file", "Open File").with_shortcut("⌘O"),
                CommandActionItem::new("close-tab", "Close Tab")
                    .with_shortcut("⌘W")
                    .with_disabled(true),
            ],
        ),
        ActionDiscoverySection::new(
            "edit",
            "Edit",
            vec![
                CommandActionItem::new("find-in-files", "Find in Files").with_shortcut("⇧⌘F"),
                CommandActionItem::new("find-and-replace", "Find and Replace").with_shortcut("⌘H"),
            ],
        ),
    ])
    .with_active_id(&active);

    let described = ActionDiscoveryPanelSpec::new(vec![
        ActionDiscoverySection::new(
            "ci-cd",
            "CI/CD",
            vec![
                CommandActionItem::new("deploy", "Deploy to Production")
                    .with_description("Push current branch to production environment")
                    .with_badge("Dangerous"),
                CommandActionItem::new("preview", "Open Preview")
                    .with_description("Launch preview in a new tab")
                    .with_shortcut("⇧⌘P"),
            ],
        ),
        ActionDiscoverySection::new(
            "tools",
            "Tools",
            vec![CommandActionItem::new("lint", "Run Linter").with_shortcut("⇧⌘L")],
        ),
    ]);

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Grouped actions",
            theme,
            framed(
                theme,
                ActionDiscoveryPanel::from_spec(grouped, theme)
                    .with_instance_id("grouped")
                    .on_select(Arc::new(move |id| {
                        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                            key: "action-discovery-active".to_string(),
                            value: id.to_string(),
                        });
                    })),
            ),
        ))
        .child(group(
            "With descriptions and badges",
            theme,
            framed(theme, ActionDiscoveryPanel::from_spec(described, theme)),
        ))
        .child(group(
            "Empty state",
            theme,
            framed(
                theme,
                ActionDiscoveryPanel::from_spec(
                    ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::Empty),
                    theme,
                ),
            ),
        ))
        .child(group(
            "Loading",
            theme,
            framed(
                theme,
                ActionDiscoveryPanel::from_spec(
                    ActionDiscoveryPanelSpec::new(vec![]).with_state(DiscoveryState::Loading),
                    theme,
                ),
            ),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "action-discovery-panel",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                framed(
                    theme,
                    ActionDiscoveryPanel::from_spec(compact_panel().with_size(size), theme)
                        .with_instance_id(format!("size-{size:?}")),
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                framed(
                    theme,
                    ActionDiscoveryPanel::from_spec(compact_panel().with_density(density), theme)
                        .with_instance_id(format!("density-{density:?}")),
                )
                .into_any_element()
            }),
    )
}
