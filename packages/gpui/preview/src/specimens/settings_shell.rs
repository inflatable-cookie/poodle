use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::Eyebrow;
use crate::PreviewRoot;
use gpui::*;
use poodle_node::Node;
use poodle_render::{settings_shell, SettingsShellHandlers};
use poodle_specs::{EyebrowSpec, SettingsShellSpec, SidebarNavGroup, SidebarNavItem};

fn groups() -> Vec<SidebarNavGroup> {
    vec![
        SidebarNavGroup::new(
            "general",
            vec![
                SidebarNavItem::new("general", "General"),
                SidebarNavItem::new("appearance", "Appearance"),
            ],
        )
        .with_label("Workspace"),
        SidebarNavGroup::new(
            "input",
            vec![SidebarNavItem::new(
                "shortcuts",
                "Keyboard Shortcuts & Input & More",
            )],
        )
        .with_label("Keyboard Shortcuts & Input & More"),
    ]
}

fn page_for(id: &str) -> Node {
    Node::text(match id {
        "appearance" => "Appearance page",
        "shortcuts" => "Keyboard shortcuts page",
        _ => "General page",
    })
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let query = state
        .specimens
        .text
        .get("settings-shell-query")
        .cloned()
        .unwrap_or_default();
    let page_id = state
        .specimens
        .text
        .get("settings-shell-page")
        .cloned()
        .unwrap_or_else(|| "general".to_string());
    let refused = state
        .specimens
        .toggles
        .get("settings-shell-refused")
        .copied()
        .unwrap_or(false);

    let mut filtered = groups();
    if !query.trim().is_empty() {
        let needle = query.to_ascii_lowercase();
        for group in &mut filtered {
            group
                .items
                .retain(|item| item.label.to_ascii_lowercase().contains(&needle));
        }
        filtered.retain(|group| !group.items.is_empty());
    }

    let events = state.node_events.clone();
    let events_nav = Arc::clone(&events);
    let events_search = Arc::clone(&events);
    let mut spec = SettingsShellSpec::new()
        .with_open(true)
        .with_groups(filtered)
        .with_active_page_id(&page_id)
        .with_page_title(&page_id)
        .with_search_query(&query)
        .with_aria_label("Nucleus settings");
    if refused {
        spec = spec.with_close_refused_reason("Unsaved changes on this page.");
    }

    let node = settings_shell(
        &spec,
        theme,
        SettingsShellHandlers {
            on_navigate: Some(Arc::new(move |id| {
                events_nav.lock().unwrap().push(NodeSpecimenEvent::SetText {
                    key: "settings-shell-page".to_string(),
                    value: id.to_string(),
                });
            })),
            on_search_query_change: Some(Arc::new(move |value| {
                events_search
                    .lock()
                    .unwrap()
                    .push(NodeSpecimenEvent::SetText {
                        key: "settings-shell-query".to_string(),
                        value: value.to_string(),
                    });
            })),
            on_request_close: Some(Arc::new(move || {
                events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                    key: "settings-shell-refused".to_string(),
                    value: true,
                });
            })),
            ..SettingsShellHandlers::default()
        },
        Some(page_for(&page_id)),
    );

    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .min_h(px(520.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new()
                .with_content("Settings dialog: grouped rail, search narrowing, and a page body"),
            theme,
        ))
        .child(poodle_gpui_node_backend::to_gpui(&node))
}
