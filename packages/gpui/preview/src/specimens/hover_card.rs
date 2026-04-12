use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Eyebrow, HoverCard};
use poodle_specs::{EyebrowSpec, HoverCardSpec, OverlayPlacement};
use std::time::Duration;

fn update_hover_intent(
    root: &WeakEntity<PreviewRoot>,
    intent_key: &'static str,
    hovered: bool,
    cx: &mut App,
) {
    root.update(cx, |this, cx| {
        this.state
            .specimens
            .toggles
            .insert(intent_key.to_string(), hovered);
        cx.notify();
    })
    .ok();
}

fn schedule_hover_card_state(
    window: &mut Window,
    cx: &mut App,
    root: WeakEntity<PreviewRoot>,
    open_key: &'static str,
    intent_key: &'static str,
    hovered: bool,
    delay_ms: u32,
) {
    window
        .spawn(cx, async move |cx| {
            cx.background_executor()
                .timer(Duration::from_millis(u64::from(delay_ms)))
                .await;
            cx.update(|_window, cx| {
                root.update(cx, |this, cx| {
                    if this.state.specimens.is_on(intent_key) == hovered {
                        this.state
                            .specimens
                            .toggles
                            .insert(open_key.to_string(), hovered);
                        cx.notify();
                    }
                })
                .ok();
            })
            .ok();
        })
        .detach();
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");
    let root = cx.weak_entity();
    let default_open = state.specimens.is_on("hover-card-default-open");
    let bottom_open = state.specimens.is_on("hover-card-bottom-open");
    let default_spec = HoverCardSpec::new();
    let bottom_spec = HoverCardSpec::new().with_placement(OverlayPlacement::Bottom);

    div().flex().flex_col().gap(px(24.0))
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default (top placement)"), theme))
                .child(
                    HoverCard::from_spec(default_spec.clone().with_open(default_open), theme)
                        .on_open_change({
                            let root = root.clone();
                            move |open, window, cx| {
                                update_hover_intent(&root, "hover-card-default-hovered", open, cx);
                                schedule_hover_card_state(
                                    window,
                                    cx,
                                    root.clone(),
                                    "hover-card-default-open",
                                    "hover-card-default-hovered",
                                    open,
                                    if open {
                                        default_spec.open_delay_ms
                                    } else {
                                        default_spec.close_delay_ms
                                    },
                                );
                            }
                        })
                        .with_trigger(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(accent))
                                .underline()
                                .cursor_pointer()
                                .child("@clay".to_string())
                        )
                        .with_content(
                            div().flex().flex_col().gap(px(4.0))
                                .max_w(px(256.0))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Clay".to_string())
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("Design systems engineer working on Poodle. Loves component architecture and accessibility.".to_string())
                                )
                        )
                )
        )
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Bottom placement"), theme))
                .child(
                    HoverCard::from_spec(bottom_spec.clone().with_open(bottom_open), theme)
                    .on_open_change({
                        let root = root.clone();
                        move |open, window, cx| {
                            update_hover_intent(&root, "hover-card-bottom-hovered", open, cx);
                            schedule_hover_card_state(
                                window,
                                cx,
                                root.clone(),
                                "hover-card-bottom-open",
                                "hover-card-bottom-hovered",
                                open,
                                if open {
                                    bottom_spec.open_delay_ms
                                } else {
                                    bottom_spec.close_delay_ms
                                },
                            );
                        }
                    })
                    .with_trigger(
                        div()
                            .text_sm()
                            .text_color(color_to_hsla(accent))
                            .underline()
                            .cursor_pointer()
                            .child("poodle/svelte-primitives".to_string())
                    )
                    .with_content(
                        div().flex().flex_col().gap(px(4.0))
                            .max_w(px(256.0))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("svelte-primitives".to_string())
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child("Core primitive components for the Poodle design system. 64 components, 94% coverage.".to_string())
                            )
                    )
                )
        )
}
