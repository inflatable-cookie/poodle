use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_workstation::{WorkspaceShellSpec, WorkspaceShellState, AppHeaderSpec};
use pug_gpui_components::{PugWorkspaceShell, PugAppHeader};
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let border_default = theme.resolve_color("semantic.color.border.default");

    let placeholder = |label: &str| {
        div()
            .px(px(8.0))
            .py(px(6.0))
            .border_1()
            .border_color(color_to_hsla(border_default))
            .rounded(px(2.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_xs()
                    .text_color(color_to_hsla(text_secondary))
                    .child(label.to_string()),
            )
    };

    // --- Default shell ---
    let default_spec = WorkspaceShellSpec::new()
        .with_app_header(true)
        .with_project_header(true)
        .with_surface_tabs(true);

    let default_shell = PugWorkspaceShell::new(default_spec, theme)
        .with_app_header(placeholder("App Header"))
        .with_project_header(placeholder("Project Header"))
        .with_surface_tabs(placeholder("Surface Tabs"))
        .with_content(
            div().flex_1().flex().items_center().justify_center()
                .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Center content"))
        )
        .with_status_bar(placeholder("Status Bar"));

    // --- Loading state ---
    let loading_spec = WorkspaceShellSpec::new()
        .with_state(WorkspaceShellState::Loading);

    let loading_shell = PugWorkspaceShell::new(loading_spec, theme)
        .with_content(
            div().flex_1().flex().flex_col().items_center().justify_center().gap(px(4.0))
                .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(color_to_hsla(text_secondary)).child("Loading workspace"))
                .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Please wait..."))
        );

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("DEFAULT SHELL", text_secondary))
        .child(div().h(px(200.0)).child(default_shell))
        .child(section_label("LOADING", text_secondary))
        .child(div().h(px(120.0)).child(loading_shell))
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
