use gpui::*;
use pug_gpui_workstation::{WorkspaceShellSpec, AppHeaderSpec};
use pug_gpui_components::{PugWorkspaceShell, PugAppHeader};
use pug_gpui::GpuiThemeProvider;
use pug_adapter::ThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let shell_spec = WorkspaceShellSpec::new()
        .with_app_header(true);

    let header_spec = AppHeaderSpec::new()
        .with_title("My App");

    let app_header = PugAppHeader::new(header_spec, theme);

    div().h(px(160.0)).child(
        PugWorkspaceShell::new(shell_spec, theme)
            .with_app_header(app_header)
            .with_content(
                div().flex_1().flex().items_center().justify_center()
                    .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Workspace content area"))
            )
    )
}
