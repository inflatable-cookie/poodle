use gpui::*;
use poodle_gpui_components::{Button, Eyebrow, Tooltip};
use poodle_specs::OverlayPlacement;
use poodle_specs::{ButtonSpec, ButtonVariant, EyebrowSpec, TooltipSpec};
use std::time::Duration;

use crate::app_state::AppState;
use crate::PreviewRoot;

fn update_tooltip_hover(
    root: &WeakEntity<PreviewRoot>,
    key: &'static str,
    hovered_key: &'static str,
    open: bool,
    cx: &mut App,
) {
    root.update(cx, |this, cx| {
        this.state
            .specimens
            .toggles
            .insert(hovered_key.to_string(), open);
        if !open {
            this.state.specimens.toggles.insert(key.to_string(), false);
        }
        cx.notify();
    })
    .ok();
}

fn schedule_tooltip_open(
    window: &mut Window,
    cx: &mut App,
    root: WeakEntity<PreviewRoot>,
    key: &'static str,
    hovered_key: &'static str,
    delay_ms: u16,
) {
    window
        .spawn(cx, async move |cx| {
            cx.background_executor()
                .timer(Duration::from_millis(u64::from(delay_ms)))
                .await;
            cx.update(|_window, cx| {
                root.update(cx, |this, cx| {
                    if this.state.specimens.is_on(hovered_key) {
                        this.state.specimens.toggles.insert(key.to_string(), true);
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
    let root = cx.weak_entity();
    let default_open = state.specimens.is_on("tooltip-default-open");
    let top_open = state.specimens.is_on("tooltip-top-open");
    let bottom_open = state.specimens.is_on("tooltip-bottom-open");
    let left_open = state.specimens.is_on("tooltip-left-open");
    let right_open = state.specimens.is_on("tooltip-right-open");
    let default_delay_ms = TooltipSpec::new().delay_ms;
    let placement_delay_ms = TooltipSpec::new().delay_ms;

    // ── Default ──────────────────────────────────────────────────────
    let default_spec = TooltipSpec::new()
        .with_content("Save your changes")
        .with_placement(OverlayPlacement::Top)
        .with_open(default_open);

    let default_trigger = Button::from_spec(
        ButtonSpec::new()
            .with_variant(ButtonVariant::Secondary)
            .with_label("Hover me"),
        theme,
    )
    .with_id("tooltip-default-trigger");

    let default_tooltip = Tooltip::from_spec(default_spec, theme)
        .on_open_change({
            let root = root.clone();
            move |open, window, cx| {
                update_tooltip_hover(
                    &root,
                    "tooltip-default-open",
                    "tooltip-default-hovered",
                    open,
                    cx,
                );
                if open {
                    schedule_tooltip_open(
                        window,
                        cx,
                        root.clone(),
                        "tooltip-default-open",
                        "tooltip-default-hovered",
                        default_delay_ms,
                    );
                }
            }
        })
        .with_trigger(default_trigger);

    // ── Placements ───────────────────────────────────────────────────
    let top_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Top tooltip")
            .with_placement(OverlayPlacement::Top)
            .with_open(top_open),
        theme,
    )
    .on_open_change({
        let root = root.clone();
        move |open, window, cx| {
            update_tooltip_hover(&root, "tooltip-top-open", "tooltip-top-hovered", open, cx);
            if open {
                schedule_tooltip_open(
                    window,
                    cx,
                    root.clone(),
                    "tooltip-top-open",
                    "tooltip-top-hovered",
                    placement_delay_ms,
                );
            }
        }
    })
    .with_trigger(
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Top"),
            theme,
        )
        .with_id("tooltip-top-trigger"),
    );

    let bottom_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Bottom tooltip")
            .with_placement(OverlayPlacement::Bottom)
            .with_open(bottom_open),
        theme,
    )
    .on_open_change({
        let root = root.clone();
        move |open, window, cx| {
            update_tooltip_hover(
                &root,
                "tooltip-bottom-open",
                "tooltip-bottom-hovered",
                open,
                cx,
            );
            if open {
                schedule_tooltip_open(
                    window,
                    cx,
                    root.clone(),
                    "tooltip-bottom-open",
                    "tooltip-bottom-hovered",
                    placement_delay_ms,
                );
            }
        }
    })
    .with_trigger(
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Bottom"),
            theme,
        )
        .with_id("tooltip-bottom-trigger"),
    );

    let left_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Left tooltip")
            .with_placement(OverlayPlacement::Left)
            .with_open(left_open),
        theme,
    )
    .on_open_change({
        let root = root.clone();
        move |open, window, cx| {
            update_tooltip_hover(&root, "tooltip-left-open", "tooltip-left-hovered", open, cx);
            if open {
                schedule_tooltip_open(
                    window,
                    cx,
                    root.clone(),
                    "tooltip-left-open",
                    "tooltip-left-hovered",
                    placement_delay_ms,
                );
            }
        }
    })
    .with_trigger(
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Left"),
            theme,
        )
        .with_id("tooltip-left-trigger"),
    );

    let right_tooltip = Tooltip::from_spec(
        TooltipSpec::new()
            .with_content("Right tooltip")
            .with_placement(OverlayPlacement::Right)
            .with_open(right_open),
        theme,
    )
    .on_open_change({
        let root = root.clone();
        move |open, window, cx| {
            update_tooltip_hover(
                &root,
                "tooltip-right-open",
                "tooltip-right-hovered",
                open,
                cx,
            );
            if open {
                schedule_tooltip_open(
                    window,
                    cx,
                    root.clone(),
                    "tooltip-right-open",
                    "tooltip-right-hovered",
                    placement_delay_ms,
                );
            }
        }
    })
    .with_trigger(
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("Right"),
            theme,
        )
        .with_id("tooltip-right-trigger"),
    );

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // Default
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(12.0))
                        .items_center()
                        .child(default_tooltip),
                ),
        )
        // Placements
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Placements"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(12.0))
                        .items_center()
                        .child(top_tooltip)
                        .child(bottom_tooltip)
                        .child(left_tooltip)
                        .child(right_tooltip),
                ),
        )
}
