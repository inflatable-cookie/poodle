use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, DrawerSpec, DrawerEdge, ButtonSpec, ButtonVariant, EyebrowSpec};
use poodle_gpui_components::{Drawer, Button, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let right_open = state.specimens.is_on("drawer-right-open");
    let left_open = state.specimens.is_on("drawer-left-open");

    div().flex().flex_col().gap(px(24.0))
        // --- Right edge (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Right edge (default)"), theme))
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
                                                .child("Drawer content goes here. You can put forms, navigation, or any other content.")
                                        )
                                )
                                .with_main_content(
                                    div().flex().gap(px(6.0)).justify_end()
                                        .child(
                                            Button::from_spec(
                                                ButtonSpec::new()
                                                    .with_variant(ButtonVariant::Secondary)
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
        )
        // --- Left edge ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Left edge"), theme))
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
                                        .child("Side navigation or filters can live in a left-edge drawer.")
                                )
                        );
                    }

                    col
                })
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(Drawer::from_spec(
                            DrawerSpec::new().with_title("Xs size").with_description("Extra small drawer."),
                            theme,
                        ).size(ControlSize::Xs).with_content(
                            div().text_size(px(12.0)).text_color(color_to_hsla(text_secondary))
                                .child("Xs drawer body content.")
                        ))
                        .child(Drawer::from_spec(
                            DrawerSpec::new().with_title("Sm size").with_description("Small drawer."),
                            theme,
                        ).size(ControlSize::Sm).with_content(
                            div().text_size(px(12.0)).text_color(color_to_hsla(text_secondary))
                                .child("Sm drawer body content.")
                        ))
                        .child(Drawer::from_spec(
                            DrawerSpec::new().with_title("Md size").with_description("Medium drawer."),
                            theme,
                        ).size(ControlSize::Md).with_content(
                            div().text_size(px(12.0)).text_color(color_to_hsla(text_secondary))
                                .child("Md drawer body content.")
                        ))
                        .child(Drawer::from_spec(
                            DrawerSpec::new().with_title("Lg size").with_description("Large drawer."),
                            theme,
                        ).size(ControlSize::Lg).with_content(
                            div().text_size(px(12.0)).text_color(color_to_hsla(text_secondary))
                                .child("Lg drawer body content.")
                        ))
                        .child(Drawer::from_spec(
                            DrawerSpec::new().with_title("Xl size").with_description("Extra large drawer."),
                            theme,
                        ).size(ControlSize::Xl).with_content(
                            div().text_size(px(12.0)).text_color(color_to_hsla(text_secondary))
                                .child("Xl drawer body content.")
                        ))
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(Drawer::from_spec(
                            DrawerSpec::new().with_title("Compact").with_description("Tighter layout."),
                            theme,
                        ).with_density(ControlDensity::Compact).with_content(
                            div().text_size(px(12.0)).text_color(color_to_hsla(text_secondary))
                                .child("Compact drawer body.")
                        ))
                        .child(Drawer::from_spec(
                            DrawerSpec::new().with_title("Default").with_description("Default layout."),
                            theme,
                        ).with_density(ControlDensity::Default).with_content(
                            div().text_size(px(12.0)).text_color(color_to_hsla(text_secondary))
                                .child("Default drawer body.")
                        ))
                        .child(Drawer::from_spec(
                            DrawerSpec::new().with_title("Comfortable").with_description("Looser layout."),
                            theme,
                        ).with_density(ControlDensity::Comfortable).with_content(
                            div().text_size(px(12.0)).text_color(color_to_hsla(text_secondary))
                                .child("Comfortable drawer body.")
                        ))
                )
        )
}
