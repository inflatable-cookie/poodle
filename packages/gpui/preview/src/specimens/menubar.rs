use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{MenubarSpec, MenubarEntry, MenuEntry, MenuItemKind, EyebrowSpec};
use poodle_gpui_components::{Menubar, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");

    // Track which menu is open and last selected action
    let active_menu = state.specimens.text.get("menubar-active").cloned();
    let last_action = state.specimens.text.get("menubar-last-action").cloned();

    let menubar_items = vec![
        MenubarEntry::new("file", "File", vec![
            MenuEntry::new("new", "New").with_shortcut_label("\u{2318}N"),
            MenuEntry::new("open", "Open\u{2026}").with_shortcut_label("\u{2318}O"),
            MenuEntry::new("save", "Save").with_shortcut_label("\u{2318}S"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("quit", "Quit").with_shortcut_label("\u{2318}Q"),
        ]),
        MenubarEntry::new("edit", "Edit", vec![
            MenuEntry::new("undo", "Undo").with_shortcut_label("\u{2318}Z"),
            MenuEntry::new("redo", "Redo").with_shortcut_label("\u{21E7}\u{2318}Z"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("cut", "Cut").with_shortcut_label("\u{2318}X"),
            MenuEntry::new("copy", "Copy").with_shortcut_label("\u{2318}C"),
            MenuEntry::new("paste", "Paste").with_shortcut_label("\u{2318}V"),
        ]),
        MenubarEntry::new("view", "View", vec![
            MenuEntry::new("zoom-in", "Zoom in").with_shortcut_label("\u{2318}+"),
            MenuEntry::new("zoom-out", "Zoom out").with_shortcut_label("\u{2318}-"),
            MenuEntry::new("sep1", "").with_kind(MenuItemKind::Separator),
            MenuEntry::new("fullscreen", "Full screen").with_shortcut_label("\u{2303}\u{2318}F"),
        ]),
    ];

    let mut spec = MenubarSpec::new(menubar_items)
        .with_aria_label("Application menu");
    if let Some(ref active) = active_menu {
        spec = spec.with_value(active.as_str());
    }

    let examples = div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Application menu bar"), theme))
                .child(
                    Menubar::from_spec(spec, theme)
                        .with_id("specimen-menubar")
                        .on_trigger(cx.listener(|this, val: &str, _w, cx| {
                            let current = this.state.specimens.text.get("menubar-active").cloned();
                            if current.as_deref() == Some(val) {
                                this.state.specimens.text.remove("menubar-active");
                            } else {
                                this.state.specimens.text.insert("menubar-active".to_string(), val.to_string());
                            }
                            cx.notify();
                        }))
                        .on_select(cx.listener(|this, val: &str, _w, cx| {
                            this.state.specimens.text.insert("menubar-last-action".to_string(), val.to_string());
                            this.state.specimens.text.remove("menubar-active");
                            cx.notify();
                        }))
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!("Last action: {}", last_action.as_deref().unwrap_or("(none)")))
                )
        )
        .into_any_element();

    let make_items = || vec![
        MenubarEntry::new("file", "File", vec![
            MenuEntry::new("new", "New"),
            MenuEntry::new("open", "Open\u{2026}"),
        ]),
        MenubarEntry::new("edit", "Edit", vec![
            MenuEntry::new("undo", "Undo"),
            MenuEntry::new("redo", "Redo"),
        ]),
    ];

    specimen_layout(
        state,
        cx,
        "menubar",
        examples,
        move |size, theme: &GpuiThemeProvider| {
            Menubar::from_spec(
                MenubarSpec::new(make_items()).with_aria_label("Menubar"),
                theme,
            )
            .with_id(format!("specimen-size-{:?}", size))
            .size(size)
            .into_any_element()
        },
        move |density, theme: &GpuiThemeProvider| {
            Menubar::from_spec(
                MenubarSpec::new(make_items()).with_aria_label("Menubar"),
                theme,
            )
            .with_id(format!("specimen-density-{:?}", density))
            .with_density(density)
            .into_any_element()
        },
    )
}
