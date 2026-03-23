use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{DrawerSpec, DrawerEdge, ButtonSpec, ButtonVariant};
use pug_gpui_components::{Drawer, Button};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let right_open = state.specimens.is_on("drawer-right-open");
    let left_open = state.specimens.is_on("drawer-left-open");

    div().flex().flex_col().gap(px(16.0))
        // --- Right edge (default) ---
        .child(section_label("RIGHT EDGE (DEFAULT)", text_secondary))
        .child({
            let mut col = div().flex().flex_col().gap(px(8.0));

            col = col.child(
                Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Open right drawer"),
                    theme,
                )
                .with_id("drawer-right-trigger")
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggles.insert("drawer-right-open".to_string(), true);
                    cx.notify();
                }))
            );

            if right_open {
                let spec = DrawerSpec::new()
                    .with_title("Settings")
                    .with_description("Configure your preferences.");

                col = col.child(
                    Drawer::from_spec(spec, theme)
                        .with_content(
                            div().flex().flex_col().gap(px(8.0))
                                .child(
                                    div().text_size(px(14.0)).text_color(color_to_hsla(text_secondary))
                                        .child("Drawer body content goes here. This is the settings panel.")
                                )
                        )
                        .with_main_content(
                            div().flex().gap(px(6.0)).justify_end()
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Ghost)
                                            .with_label("Cancel"),
                                        theme,
                                    )
                                    .with_id("drawer-cancel")
                                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                        this.state.specimens.toggles.insert("drawer-right-open".to_string(), false);
                                        cx.notify();
                                    }))
                                )
                                .child(
                                    Button::from_spec(
                                        ButtonSpec::new()
                                            .with_variant(ButtonVariant::Primary)
                                            .with_label("Save"),
                                        theme,
                                    )
                                    .with_id("drawer-save")
                                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                        this.state.specimens.toggles.insert("drawer-right-open".to_string(), false);
                                        cx.notify();
                                    }))
                                )
                        )
                );
            }

            col
        })
        // --- Left edge ---
        .child(section_label("LEFT EDGE", text_secondary))
        .child({
            let mut col = div().flex().flex_col().gap(px(8.0));

            col = col.child(
                Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Open left drawer"),
                    theme,
                )
                .with_id("drawer-left-trigger")
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggles.insert("drawer-left-open".to_string(), true);
                    cx.notify();
                }))
            );

            if left_open {
                let spec = DrawerSpec::new()
                    .with_edge(DrawerEdge::Left)
                    .with_title("Navigation");

                col = col.child(
                    Drawer::from_spec(spec, theme)
                        .with_content(
                            div().text_size(px(14.0)).text_color(color_to_hsla(text_secondary))
                                .child("Navigation sidebar content goes here.")
                        )
                );
            }

            col
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
