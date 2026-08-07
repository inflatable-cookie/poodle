use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, SplitView};
use crate::style_bridge::{color_to_hsla, hsla_to_color_value};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::EyebrowSpec;
use poodle_specs::{SplitOrientation, SplitViewSpec};

/// The specimen frames are full-width inside the preview's content column, so
/// this is the axis extent the divider's pixel deltas are measured against.
/// See `SplitView::with_extent_px`.
const FRAME_WIDTH_PX: f32 = 640.0;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let panel_bg = theme.resolve_color("color.background.panel");

    // Small helper: coloured region block used as a stand-in for
    // Svelte's <Region> primitive (which is a simple labelled swatch).
    let region = move |label: &'static str, hue: f32| -> Node {
        let region_bg = Hsla {
            h: hue / 360.0,
            s: 0.55,
            l: 0.35,
            a: 0.22,
        };
        let region_text = Hsla {
            h: hue / 360.0,
            s: 0.65,
            l: 0.78,
            a: 1.0,
        };
        let mut block = Node::container();
        {
            let s = &mut block.style;
            s.descriptor.background = Some(hsla_to_color_value(region_bg));
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.fill_width = true;
            s.fill_height = true;
            // Typography sits on the block, not the caption: the old tier set
            // it on the wrapper and let the string inherit, and centring a
            // caption that carries its own metrics lands a few px off.
            s.descriptor.text_color = Some(hsla_to_color_value(region_text));
            s.text_size = Some(13.0);
            s.text_weight = Some(500);
        }
        block.child(Node::text(label.to_string()))
    };

    // Frame wrapper: bordered container so the split view has visible
    // bounds in the specimen layout.
    let frame = move |height: f32| {
        div()
            .h(px(height))
            .w_full()
            .border_1()
            .border_color(color_to_hsla(border_subtle))
            .rounded(px(6.0))
            .overflow_hidden()
    };

    // Drag-to-resize: the ratio lives in specimen state (as a percentage,
    // seeded at 50) and streams back through on_ratio_change.
    let ratio_key = "split-view-ratio";
    let ratio_pct = match state.specimens.selected(ratio_key) {
        0 => 50,
        pct => pct,
    };
    let interactive = SplitView::from_spec(
        SplitViewSpec::new(SplitOrientation::Horizontal).with_ratio(ratio_pct as f32 / 100.0),
        theme,
    )
    .with_primary(region("Sidebar", 220.0))
    .with_secondary(region("Main content", 140.0))
    .with_extent_px(FRAME_WIDTH_PX)
    .on_ratio_change({
        // Context-free: the drag streams through the node event queue, which is
        // drained at the top of the next render.
        let queue = std::sync::Arc::clone(&state.node_events);
        std::sync::Arc::new(move |ratio: f32| {
            queue.lock().unwrap().push(NodeSpecimenEvent::Select {
                key: ratio_key.to_string(),
                index: (ratio * 100.0).round().max(1.0) as usize,
            });
        })
    });
    let _ = cx;

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Drag to resize ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content(format!("Drag to resize (ratio: {ratio_pct}%)")),
                    theme,
                ))
                .child(frame(160.0).child(interactive)),
        )
        // --- Basic horizontal layout ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Basic horizontal layout"),
                    theme,
                ))
                .child(
                    frame(160.0).child(
                        SplitView::from_spec(
                            SplitViewSpec::new(SplitOrientation::Horizontal)
                                .with_default_ratio(0.5),
                            theme,
                        )
                        .with_primary(region("Sidebar", 220.0))
                        .with_secondary(region("Main content", 140.0)),
                    ),
                ),
        )
        // --- Basic vertical layout ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Basic vertical layout"),
                    theme,
                ))
                .child(
                    frame(256.0).child(
                        SplitView::from_spec(
                            SplitViewSpec::new(SplitOrientation::Vertical).with_default_ratio(0.5),
                            theme,
                        )
                        .with_primary(region("Editor", 220.0))
                        .with_secondary(region("Terminal", 280.0)),
                    ),
                ),
        )
        // --- Horizontal with collapse toggles ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Horizontal with collapse toggles"),
                    theme,
                ))
                .child(
                    frame(160.0).child(
                        SplitView::from_spec(
                            SplitViewSpec::new(SplitOrientation::Horizontal)
                                .with_default_ratio(0.35)
                                .with_show_collapse_primary(true)
                                .with_show_collapse_secondary(true),
                            theme,
                        )
                        .with_primary(region("Primary", 220.0))
                        .with_secondary(region("Secondary", 140.0)),
                    ),
                ),
        )
        // --- Vertical with collapse toggles ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Vertical with collapse toggles"),
                    theme,
                ))
                .child(
                    frame(256.0).child(
                        SplitView::from_spec(
                            SplitViewSpec::new(SplitOrientation::Vertical)
                                .with_default_ratio(0.6)
                                .with_show_collapse_primary(true)
                                .with_show_collapse_secondary(true),
                            theme,
                        )
                        .with_primary(region("Top", 220.0))
                        .with_secondary(region("Bottom", 280.0)),
                    ),
                ),
        )
        // --- Nested splits (IDE-style layout) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Nested splits (IDE-style layout)"),
                    theme,
                ))
                .child(
                    frame(256.0).child(
                        SplitView::from_spec(
                            SplitViewSpec::new(SplitOrientation::Horizontal)
                                .with_default_ratio(0.25)
                                .with_show_collapse_primary(true),
                            theme,
                        )
                        .with_primary(region("Explorer", 220.0))
                        .with_secondary(
                            SplitView::from_spec(
                                SplitViewSpec::new(SplitOrientation::Vertical)
                                    .with_default_ratio(0.65)
                                    .with_show_collapse_secondary(true),
                                theme,
                            )
                            .with_primary(region("Editor", 140.0))
                            .with_secondary(region("Terminal", 280.0)),
                        ),
                    ),
                ),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(
                    frame(160.0).child(
                        SplitView::from_spec(
                            SplitViewSpec::new(SplitOrientation::Horizontal)
                                .with_default_ratio(0.5)
                                .with_disabled(true)
                                .with_show_collapse_primary(true)
                                .with_show_collapse_secondary(true),
                            theme,
                        )
                        .with_primary(region("Left", 220.0))
                        .with_secondary(region("Right", 140.0)),
                    ),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(color_to_hsla(text_secondary))
                        .child("Drag cursor and hover state disabled."),
                ),
        )
        .child({
            let _ = panel_bg;
            div()
        })
}
