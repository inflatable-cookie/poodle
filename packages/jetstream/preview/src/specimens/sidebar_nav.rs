//! SidebarNav specimen — vertical navigation with grouped items.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::sidebar_nav::js_sidebar_nav;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{SidebarNavGroup, SidebarNavItem, SidebarNavSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    let groups = vec![
        SidebarNavGroup::new("main", vec![
            SidebarNavItem::new("dashboard", "Dashboard"),
            SidebarNavItem::new("projects", "Projects"),
            SidebarNavItem::new("assets", "Assets"),
        ]).with_label("Workspace"),
        SidebarNavGroup::new("settings", vec![
            SidebarNavItem::new("account", "Account"),
            SidebarNavItem::new("billing", "Billing"),
            SidebarNavItem::new("team", "Team").with_disabled(true),
        ]).with_label("Settings"),
    ];

    // Single untitled group → continuous plain list (contract specimen).
    let plain = vec![SidebarNavGroup::new("docs", vec![
        SidebarNavItem::new("overview", "Overview"),
        SidebarNavItem::new("components", "Components"),
        SidebarNavItem::new("tokens", "Tokens"),
        SidebarNavItem::new("guides", "Guides"),
    ])];

    div().flex_col().gap(24.0)
        .child(group("Plain list (untitled, active item)", secondary,
            js_sidebar_nav(
                &SidebarNavSpec::new(plain)
                    .with_value("components"),
                theme,
            )
        ))
        .child(group("Grouped, with active item + disabled", secondary,
            js_sidebar_nav(
                &SidebarNavSpec::new(groups.clone())
                    .with_value("projects"),
                theme,
            )
        ))
        .child(group("No selection", secondary,
            js_sidebar_nav(
                &SidebarNavSpec::new(groups),
                theme,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
