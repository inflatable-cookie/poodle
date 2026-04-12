use crate::app_state::AppState;
use crate::specimens::overlay_state;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Eyebrow, HoverCard};
use poodle_specs::{EyebrowSpec, HoverCardSpec, OverlayPlacement};

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
                                overlay_state::sync_hover_intent(
                                    &root,
                                    "hover-card-default-open",
                                    "hover-card-default-hovered",
                                    open,
                                    cx,
                                );
                                overlay_state::schedule_toggle_if(
                                    window,
                                    cx,
                                    root.clone(),
                                    "hover-card-default-hovered",
                                    open,
                                    "hover-card-default-open",
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
                            overlay_state::sync_hover_intent(
                                &root,
                                "hover-card-bottom-open",
                                "hover-card-bottom-hovered",
                                open,
                                cx,
                            );
                            overlay_state::schedule_toggle_if(
                                window,
                                cx,
                                root.clone(),
                                "hover-card-bottom-hovered",
                                open,
                                "hover-card-bottom-open",
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
