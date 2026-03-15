use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_composites::{DetailShellSpec, DetailState};
use pug_gpui_components::PugDetailShell;
use pug_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let ready_spec = DetailShellSpec::new()
        .with_title("Detail View")
        .with_state(DetailState::Ready);

    let loading_spec = DetailShellSpec::new()
        .with_title("Loading View")
        .with_state(DetailState::Loading);

    let error_spec = DetailShellSpec::new()
        .with_title("Error View")
        .with_state(DetailState::Error);

    div().flex().flex_col().gap(px(12.0))
        .child(
            div().h(px(120.0)).child(
                PugDetailShell::new(ready_spec, theme)
                    .with_content(
                        div().flex().flex_col().gap(px(4.0))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Key: Value"))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Status: Active"))
                            .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Owner: Team Alpha"))
                    )
            )
        )
        .child(div().h(px(80.0)).child(PugDetailShell::new(loading_spec, theme)))
        .child(div().h(px(80.0)).child(PugDetailShell::new(error_spec, theme)))
}
