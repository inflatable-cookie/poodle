use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{ButtonVariant, EyebrowSpec};
use poodle_gpui_components::toggle::Toggle;
use poodle_gpui_components::Eyebrow;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let bold_pressed = state.specimens.is_on("toggle-bold");
    let italic_pressed = state.specimens.is_on("toggle-italic");
    let pinned_pressed = !state.specimens.is_on("toggle-pinned-off");
    let favorite_pressed = state.specimens.is_on("toggle-favorite");

    div().flex().flex_col().gap(px(24.0))
        // --- Ghost variant (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Ghost variant (default)"), theme))
                .child(
                    div().flex().gap(px(4.0))
                        .child(
                            Toggle::new(theme).icon("bold")
                                .pressed(bold_pressed)
                                .aria_label("Bold")
                                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                    this.state.specimens.toggle("toggle-bold");
                                    cx.notify();
                                }))
                        )
                        .child(
                            Toggle::new(theme).icon("italic")
                                .pressed(italic_pressed)
                                .aria_label("Italic")
                                .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                    this.state.specimens.toggle("toggle-italic");
                                    cx.notify();
                                }))
                        )
                )
                .child(
                    div().text_sm().text_color(color_to_hsla(text_secondary))
                        .child(format!("Bold: {}, Italic: {}", bold_pressed, italic_pressed))
                )
        )
        // --- Primary variant ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Primary variant"), theme))
                .child(
                    Toggle::new(theme).icon("pin").label("Pinned")
                        .variant(ButtonVariant::Primary)
                        .pressed(pinned_pressed)
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggle("toggle-pinned-off");
                            cx.notify();
                        }))
                )
        )
        // --- Secondary variant ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Secondary variant"), theme))
                .child(
                    Toggle::new(theme).icon("star").label("Favorite")
                        .variant(ButtonVariant::Secondary)
                        .pressed(favorite_pressed)
                        .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggle("toggle-favorite");
                            cx.notify();
                        }))
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    Toggle::new(theme).label("Locked")
                        .pressed(true)
                        .disabled(true)
                )
        )
}
