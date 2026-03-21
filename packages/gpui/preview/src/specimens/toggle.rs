use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_components::toggle::Toggle;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let bold_pressed = state.specimens.is_on("toggle-bold");
    let italic_pressed = state.specimens.is_on("toggle-italic");
    let pinned_pressed = !state.specimens.is_on("toggle-pinned-off");
    let favorite_pressed = state.specimens.is_on("toggle-favorite");

    div().flex().flex_col().gap(px(16.0))
        // --- Ghost variant (default) ---
        .child(section_label("GHOST VARIANT (DEFAULT)", text_secondary))
        .child(
            div().flex().gap(px(4.0))
                .child(
                    Toggle::new("Bold", theme)
                        .with_pressed(bold_pressed)
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggle("toggle-bold");
                            cx.notify();
                        }))
                )
                .child(
                    Toggle::new("Italic", theme)
                        .with_pressed(italic_pressed)
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggle("toggle-italic");
                            cx.notify();
                        }))
                )
        )
        // --- Primary variant ---
        .child(section_label("PRIMARY VARIANT", text_secondary))
        .child(
            Toggle::new("Pinned", theme)
                .with_pressed(pinned_pressed)
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggle("toggle-pinned-off");
                    cx.notify();
                }))
        )
        // --- Secondary variant ---
        .child(section_label("SECONDARY VARIANT", text_secondary))
        .child(
            Toggle::new("Favorite", theme)
                .with_pressed(favorite_pressed)
                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                    this.state.specimens.toggle("toggle-favorite");
                    cx.notify();
                }))
        )
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child(
            Toggle::new("Locked", theme)
                .with_pressed(true)
                .with_disabled(true)
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(crate::style_bridge::color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
