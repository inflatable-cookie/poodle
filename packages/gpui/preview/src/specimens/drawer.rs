use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Button, Drawer, Eyebrow};
use poodle_specs::{ButtonSpec, ButtonVariant, DrawerEdge, DrawerSpec, EyebrowSpec};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let trigger = |id: &'static str,
                   key: &'static str,
                   label: &'static str,
                   cx: &mut Context<PreviewRoot>| {
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label(label),
            theme,
        )
        .with_id(id)
        .on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
            this.state.specimens.toggles.insert(key.to_string(), true);
            cx.notify();
        }))
        .into_any_element()
    };

    let mut root = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Right edge (default)"),
                    theme,
                ))
                .child(trigger(
                    "drawer-right-trigger",
                    "drawer-right-open",
                    "Open right drawer",
                    cx,
                )),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Left edge"),
                    theme,
                ))
                .child(trigger(
                    "drawer-left-trigger",
                    "drawer-left-open",
                    "Open left drawer",
                    cx,
                )),
        );

    if state.specimens.is_on("drawer-right-open") {
        root = root.child(
            Drawer::from_spec(
                DrawerSpec::new()
                    .with_title("Settings")
                    .with_description("Configure your preferences."),
                theme,
            )
            .with_content(
                div()
                    .text_size(px(14.0))
                    .text_color(color_to_hsla(text_secondary))
                    .child(
                        "Drawer content goes here. You can put forms, navigation, or any other content.",
                    ),
            )
            .with_main_content(
                div()
                    .flex()
                    .gap(px(6.0))
                    .justify_end()
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Secondary)
                                .with_label("Cancel"),
                            theme,
                        )
                        .with_id("drawer-cancel")
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state
                                .specimens
                                .toggles
                                .insert("drawer-right-open".to_string(), false);
                            cx.notify();
                        })),
                    )
                    .child(
                        Button::from_spec(ButtonSpec::new().with_label("Save"), theme)
                            .with_id("drawer-save")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state
                                    .specimens
                                    .toggles
                                    .insert("drawer-right-open".to_string(), false);
                                cx.notify();
                            })),
                    ),
            ),
        );
    }

    if state.specimens.is_on("drawer-left-open") {
        root = root.child(
            Drawer::from_spec(
                DrawerSpec::new()
                    .with_edge(DrawerEdge::Left)
                    .with_title("Navigation"),
                theme,
            )
            .with_content(
                div()
                    .text_size(px(14.0))
                    .text_color(color_to_hsla(text_secondary))
                    .child("Side navigation or filters can live in a left-edge drawer."),
            ),
        );
    }

    root
}
