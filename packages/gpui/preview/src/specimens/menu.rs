use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Button, Eyebrow, Menu};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, ButtonVariant, EyebrowSpec, MenuEntry, MenuItemKind, MenuSpec};
use std::sync::Arc;

fn menu_click(
    state: &AppState,
    key: &'static str,
    close_keys: [&'static str; 2],
) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move || {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::Toggle(key.to_string()));
        for close_key in close_keys {
            events.push(NodeSpecimenEvent::SetToggle {
                key: close_key.to_string(),
                value: false,
            });
        }
    })
}

fn menu_select(state: &AppState, open_key: &'static str) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::Change {
            open_key: open_key.to_string(),
            value_key: "menu-last-action".to_string(),
            value: value.to_string(),
        });
    })
}

fn settings_select(state: &AppState) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |value| {
        let mut events = events.lock().unwrap();
        match value {
            "theme" => events.push(NodeSpecimenEvent::Toggle("menu-dark-mode".to_string())),
            "notifications" => {
                events.push(NodeSpecimenEvent::Toggle("menu-notifications".to_string()))
            }
            _ => {}
        }
        events.push(NodeSpecimenEvent::SetText {
            key: "menu-last-action".to_string(),
            value: value.to_string(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_action = state
        .specimens
        .text
        .get("menu-last-action")
        .cloned()
        .unwrap_or_default();

    // --- With shortcuts ---
    let file_items = vec![
        MenuEntry::new("new", "New file").with_shortcut_label("\u{2318}N"),
        MenuEntry::new("open", "Open\u{2026}").with_shortcut_label("\u{2318}O"),
        MenuEntry::new("save", "Save").with_shortcut_label("\u{2318}S"),
        MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("export", "Export as PDF"),
        MenuEntry::new("print", "Print\u{2026}")
            .with_shortcut_label("\u{2318}P")
            .with_disabled(true),
    ];

    let file_open = state.specimens.is_on("menu-file-open");
    let file_spec = MenuSpec::new(file_items)
        .with_open(file_open)
        .with_aria_label("File menu");

    // --- With checkboxes ---
    let dark_mode = state.specimens.is_on("menu-dark-mode");
    let notifications = state.specimens.is_on("menu-notifications");

    let settings_items = vec![
        MenuEntry::new("theme", "Dark mode")
            .with_kind(MenuItemKind::Checkbox)
            .with_checked(dark_mode),
        MenuEntry::new("notifications", "Notifications")
            .with_kind(MenuItemKind::Checkbox)
            .with_checked(notifications),
        MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("settings", "Settings\u{2026}"),
    ];

    let settings_open = state.specimens.is_on("menu-settings-open");
    let settings_spec = MenuSpec::new(settings_items)
        .with_open(settings_open)
        .with_aria_label("Settings menu");

    let destructive_open = state.specimens.is_on("menu-destructive-open");

    let examples = div()
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
                    EyebrowSpec::new().with_content("With shortcuts"),
                    theme,
                ))
                .child(
                    Menu::from_spec(file_spec, theme)
                        .with_id("specimen-menu-shortcuts")
                        .with_trigger(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("File"),
                                theme,
                            )
                            .with_id("menu-file-trigger")
                            .on_click(menu_click(
                                state,
                                "menu-file-open",
                                ["menu-settings-open", "menu-destructive-open"],
                            )),
                        )
                        .on_select(menu_select(state, "menu-file-open")),
                ),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With checkboxes"),
                    theme,
                ))
                .child(
                    Menu::from_spec(settings_spec, theme)
                        .with_id("specimen-menu-checkboxes")
                        .with_trigger(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Settings"),
                                theme,
                            )
                            .with_id("menu-settings-trigger")
                            .on_click(menu_click(
                                state,
                                "menu-settings-open",
                                ["menu-file-open", "menu-destructive-open"],
                            )),
                        )
                        .on_select(settings_select(state)),
                ),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Destructive action"),
                    theme,
                ))
                .child(
                    Menu::from_spec(
                        MenuSpec::new(vec![
                            MenuEntry::new("rename", "Rename"),
                            MenuEntry::new("archive", "Archive"),
                            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
                            MenuEntry::new("delete", "Delete").with_destructive(true),
                        ])
                        .with_open(destructive_open)
                        .with_aria_label("Item actions"),
                        theme,
                    )
                    .with_id("specimen-menu-destructive")
                    .with_trigger(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Actions"),
                            theme,
                        )
                        .with_id("menu-actions-trigger")
                        .on_click(menu_click(
                            state,
                            "menu-destructive-open",
                            ["menu-file-open", "menu-settings-open"],
                        )),
                    )
                    .on_select(menu_select(state, "menu-destructive-open")),
                ),
        )
        // --- Last action feedback ---
        .when(!last_action.is_empty(), |d| {
            d.child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child(format!("Last: {}", last_action)),
            )
        })
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "menu",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let items = vec![
                    MenuEntry::new("cut", "Cut"),
                    MenuEntry::new("copy", "Copy"),
                    MenuEntry::new("paste", "Paste"),
                ];
                let spec = MenuSpec::new(items).with_open(true).with_aria_label("Menu");
                Menu::from_spec(spec, theme)
                    .with_id(format!("specimen-size-{:?}", size))
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let items = vec![
                    MenuEntry::new("cut", "Cut"),
                    MenuEntry::new("copy", "Copy"),
                    MenuEntry::new("paste", "Paste"),
                ];
                let spec = MenuSpec::new(items).with_open(true).with_aria_label("Menu");
                Menu::from_spec(spec, theme)
                    .with_id(format!("specimen-density-{:?}", density))
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
