use gpui::*;
use pug_gpui_primitives::{MenubarSpec, MenubarEntry, MenuEntry};
use pug_gpui_components::PugMenubar;
use pug_gpui::GpuiThemeProvider;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let menubar_items = vec![
        MenubarEntry::new("file", "File", vec![
            MenuEntry::new("new", "New File").with_shortcut_label("⌘N"),
            MenuEntry::new("open", "Open").with_shortcut_label("⌘O"),
        ]),
        MenubarEntry::new("edit", "Edit", vec![
            MenuEntry::new("undo", "Undo").with_shortcut_label("⌘Z"),
            MenuEntry::new("redo", "Redo").with_shortcut_label("⇧⌘Z"),
        ]),
    ];

    let spec = MenubarSpec::new(menubar_items);

    div().child(
        PugMenubar::new(spec, theme).with_id("specimen-menubar")
    )
}
