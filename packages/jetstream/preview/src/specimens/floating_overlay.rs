//! FloatingOverlay specimen — anchor + positioned surface, all four placements.

use crate::compat::js_floating_overlay;
use crate::compat::{control_height_rem, rem_to_px, size_font_rem};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ControlSize, OverlayPlacement};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let surface_bg = resolve_color(theme, "color.background.surface");
    let elevated_bg = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");

    let font_size = rem_to_px(size_font_rem(ControlSize::Md));
    let anchor_h = rem_to_px(control_height_rem(ControlSize::Md));
    let anchor_w = rem_to_px(7.0); // estimated trigger width

    // Reusable anchor button builder
    let anchor = |label_text: &str| -> El {
        button(label_text)
            .text_color(text_primary)
            .text_size(font_size)
            .bg(surface_bg)
            .border_1()
            .border_color(border)
            .rounded(rem_to_px(0.25))
            .px(rem_to_px(0.75))
            .h(anchor_h)
            .focusable()
            .cursor_pointer()
    };

    // Reusable floating panel builder
    let panel = |content: &str| -> El {
        div()
            .bg(elevated_bg)
            .border_1()
            .border_color(border)
            .rounded(rem_to_px(0.375))
            .px(rem_to_px(0.75))
            .py(rem_to_px(0.5))
            .min_w(rem_to_px(8.0))
            .shadow_sm()
            .child(label(content).text_color(text_primary).text_size(font_size))
    };

    div()
        .flex_col()
        .gap(rem_to_px(2.0))
        // ── Bottom placement ──────────────────────────────────────────
        .child(group(
            "Bottom (surface below anchor)",
            secondary,
            div()
                .flex_row()
                .items_start()
                .gap(rem_to_px(3.0))
                // BottomStart: panel aligns to anchor left edge
                .child(
                    div()
                        .flex_col()
                        .gap(rem_to_px(0.5))
                        .items_start()
                        .child(
                            label("BottomStart")
                                .text_color(secondary)
                                .text_size(rem_to_px(0.6875)),
                        )
                        .child(js_floating_overlay(
                            anchor("Trigger"),
                            Some(panel("Bottom-start surface")),
                            OverlayPlacement::BottomStart,
                            anchor_h,
                            anchor_w,
                        )),
                )
                // Bottom: panel aligns to anchor left edge
                .child(
                    div()
                        .flex_col()
                        .gap(rem_to_px(0.5))
                        .items_start()
                        .child(
                            label("Bottom")
                                .text_color(secondary)
                                .text_size(rem_to_px(0.6875)),
                        )
                        .child(js_floating_overlay(
                            anchor("Trigger"),
                            Some(panel("Bottom surface")),
                            OverlayPlacement::Bottom,
                            anchor_h,
                            anchor_w,
                        )),
                )
                // BottomEnd: panel aligns to anchor right edge
                .child(
                    div()
                        .flex_col()
                        .gap(rem_to_px(0.5))
                        .items_start()
                        .child(
                            label("BottomEnd")
                                .text_color(secondary)
                                .text_size(rem_to_px(0.6875)),
                        )
                        .child(js_floating_overlay(
                            anchor("Trigger"),
                            Some(panel("Bottom-end surface")),
                            OverlayPlacement::BottomEnd,
                            anchor_h,
                            anchor_w,
                        )),
                ),
        ))
        // ── Right placement ───────────────────────────────────────────
        .child(group(
            "Right (surface right of anchor)",
            secondary,
            div()
                .flex_row()
                .items_start()
                .gap(rem_to_px(3.0))
                .child(
                    div()
                        .flex_col()
                        .gap(rem_to_px(0.5))
                        .items_start()
                        .child(
                            label("RightStart")
                                .text_color(secondary)
                                .text_size(rem_to_px(0.6875)),
                        )
                        .child(js_floating_overlay(
                            anchor("Trigger"),
                            Some(panel("Right-start surface")),
                            OverlayPlacement::RightStart,
                            anchor_h,
                            anchor_w,
                        )),
                )
                .child(
                    div()
                        .flex_col()
                        .gap(rem_to_px(0.5))
                        .items_start()
                        .child(
                            label("RightEnd")
                                .text_color(secondary)
                                .text_size(rem_to_px(0.6875)),
                        )
                        .child(js_floating_overlay(
                            anchor("Trigger"),
                            Some(panel("Right-end surface")),
                            OverlayPlacement::RightEnd,
                            anchor_h,
                            anchor_w,
                        )),
                ),
        ))
        // ── No surface (anchor only) ──────────────────────────────────
        .child(group(
            "No surface (anchor only — overlay hidden)",
            secondary,
            js_floating_overlay(
                anchor("Just an anchor"),
                None,
                OverlayPlacement::Bottom,
                anchor_h,
                anchor_w,
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
