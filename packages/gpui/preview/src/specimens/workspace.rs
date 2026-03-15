use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{MenubarSpec, MenubarEntry, MenuEntry, SurfaceSpec, SurfaceTone, SurfaceBorder};
use pug_gpui_components::{PugMenubar, PugSurface};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let header_surface = SurfaceSpec::new()
        .with_tone(SurfaceTone::Panel)
        .with_border(SurfaceBorder::Subtle);

    let menubar_items = vec![
        MenubarEntry::new("file", "File", vec![
            MenuEntry::new("new", "New File").with_shortcut_label("⌘N"),
            MenuEntry::new("open", "Open").with_shortcut_label("⌘O"),
        ]),
        MenubarEntry::new("edit", "Edit", vec![
            MenuEntry::new("undo", "Undo").with_shortcut_label("⌘Z"),
            MenuEntry::new("redo", "Redo").with_shortcut_label("⇧⌘Z"),
        ]),
        MenubarEntry::new("view", "View", vec![
            MenuEntry::new("sidebar", "Toggle Sidebar"),
        ]),
    ];

    let menubar_spec = MenubarSpec::new(menubar_items);

    div().flex().flex_col().h(px(120.0))
        .child(
            PugSurface::new(header_surface, theme)
                .with_content(
                    div().flex().items_center().gap(px(10.0))
                        .child(div().text_sm().child("My App"))
                        .child(PugMenubar::new(menubar_spec, theme).with_id("ws-menubar"))
                )
        )
        .child(
            div().flex_1().flex().items_center().justify_center()
                .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Workspace content area"))
        )
}
