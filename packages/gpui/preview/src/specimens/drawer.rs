use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{DrawerSpec, DrawerEdge, ButtonSpec, ButtonVariant};
use pug_gpui_components::{PugDrawer, PugButton};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Right edge (default) ---
        .child(section_label("RIGHT EDGE (DEFAULT)", text_secondary))
        .child({
            let spec = DrawerSpec::new()
                .with_title("Settings")
                .with_description("Configure your preferences.");

            PugDrawer::new(spec, theme)
                .with_content(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            div().text_xs().text_color(color_to_hsla(text_secondary))
                                .child("Drawer body content")
                        )
                )
                .with_main_content(
                    div().flex().gap(px(6.0)).justify_end()
                        .child(
                            PugButton::new(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_label("Cancel"),
                                theme,
                            )
                            .with_id("drawer-cancel")
                        )
                        .child(
                            PugButton::new(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Primary)
                                    .with_label("Save"),
                                theme,
                            )
                            .with_id("drawer-save")
                        )
                )
        })
        // --- Left edge ---
        .child(section_label("LEFT EDGE", text_secondary))
        .child({
            let spec = DrawerSpec::new()
                .with_edge(DrawerEdge::Left)
                .with_title("Navigation");

            PugDrawer::new(spec, theme)
                .with_content(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("Navigation body content")
                )
        })
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
