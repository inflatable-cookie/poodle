use gpui::*;
use gpui::prelude::FluentBuilder;
use poodle_adapter::ThemeProvider;
use poodle_components::{MenuSpec, MenuEntry, MenuItemKind, EyebrowSpec};
use poodle_gpui_components::{Menu, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let last_action = state.specimens.text.get("menu-last-action")
        .cloned()
        .unwrap_or_default();

    // --- With shortcuts ---
    let file_items = vec![
        MenuEntry::new("new", "New file").with_shortcut_label("\u{2318}N"),
        MenuEntry::new("open", "Open\u{2026}").with_shortcut_label("\u{2318}O"),
        MenuEntry::new("save", "Save").with_shortcut_label("\u{2318}S"),
        MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
        MenuEntry::new("export", "Export as PDF"),
        MenuEntry::new("print", "Print\u{2026}").with_shortcut_label("\u{2318}P").with_disabled(true),
    ];

    let file_spec = MenuSpec::new(file_items)
        .with_default_open(true)
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

    let settings_spec = MenuSpec::new(settings_items)
        .with_default_open(true)
        .with_aria_label("Settings menu");

    let examples = div().flex().flex_col().gap(px(24.0))
        // With shortcuts
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With shortcuts"), theme))
                .child(
                    Menu::from_spec(file_spec, theme)
                        .with_id("specimen-menu-shortcuts")
                        .on_select(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert(
                                "menu-last-action".to_string(),
                                format!("Selected: {}", val),
                            );
                            cx.notify();
                        }))
                )
        )
        // With checkboxes
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With checkboxes"), theme))
                .child(
                    Menu::from_spec(settings_spec, theme)
                        .with_id("specimen-menu-checkboxes")
                        .on_select(cx.listener(|this, val: &str, _w, cx| {
                            match val {
                                "theme" => { this.state.specimens.toggle("menu-dark-mode"); },
                                "notifications" => { this.state.specimens.toggle("menu-notifications"); },
                                _ => {}
                            }
                            this.state.specimens.text.insert(
                                "menu-last-action".to_string(),
                                format!("Selected: {}", val),
                            );
                            cx.notify();
                        }))
                )
        )
        // With destructive item
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With destructive item"), theme))
                .child(
                    Menu::from_spec(
                        MenuSpec::new(vec![
                            MenuEntry::new("rename", "Rename"),
                            MenuEntry::new("duplicate", "Duplicate"),
                            MenuEntry::new("export", "Export\u{2026}"),
                            MenuEntry::new("sep", "").with_kind(MenuItemKind::Separator),
                            MenuEntry::new("delete", "Delete")
                                .with_destructive(true)
                                .with_shortcut_label("\u{2318} \u{232B}"),
                        ])
                        .with_default_open(true)
                        .with_aria_label("Item actions"),
                        theme,
                    )
                    .with_id("specimen-menu-destructive")
                )
        )
        // --- Last action feedback ---
        .when(!last_action.is_empty(), |d| {
            d.child(
                div().text_sm().text_color(color_to_hsla(text_secondary))
                    .child(last_action)
            )
        })
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "menu",
        examples,
        |size, theme: &GpuiThemeProvider| {
            let items = vec![
                MenuEntry::new("cut", "Cut"),
                MenuEntry::new("copy", "Copy"),
                MenuEntry::new("paste", "Paste"),
            ];
            let spec = MenuSpec::new(items)
                .with_default_open(true)
                .with_aria_label("Menu");
            Menu::from_spec(spec, theme)
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            let items = vec![
                MenuEntry::new("cut", "Cut"),
                MenuEntry::new("copy", "Copy"),
                MenuEntry::new("paste", "Paste"),
            ];
            let spec = MenuSpec::new(items)
                .with_default_open(true)
                .with_aria_label("Menu");
            Menu::from_spec(spec, theme)
                .with_id(format!("specimen-density-{:?}", density))
                .with_density(density)
                .into_any_element()
        },
    )
}
