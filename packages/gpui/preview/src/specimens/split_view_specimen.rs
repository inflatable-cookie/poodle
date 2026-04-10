use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_composites::{SplitViewSpec, SplitOrientation};
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{SplitView, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let panel_bg = theme.resolve_color("color.background.panel");

    // Small helper: coloured region block used as a stand-in for
    // Svelte's <Region> primitive (which is a simple labelled swatch).
    let region = move |label: &'static str, hue: f32| {
        let region_bg = Hsla { h: hue / 360.0, s: 0.55, l: 0.35, a: 0.22 };
        let region_text = Hsla { h: hue / 360.0, s: 0.65, l: 0.78, a: 1.0 };
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(region_bg)
            .text_size(px(13.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(region_text)
            .child(label)
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

    div().flex().flex_col().gap(px(24.0))
        // --- Basic horizontal layout ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic horizontal layout"), theme))
                .child(
                    frame(160.0).child(
                        SplitView::from_spec(
                            SplitViewSpec::new(SplitOrientation::Horizontal).with_default_ratio(0.5),
                            theme,
                        )
                        .with_primary(region("Sidebar", 220.0))
                        .with_secondary(region("Main content", 140.0))
                    )
                )
        )
        // --- Basic vertical layout ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Basic vertical layout"), theme))
                .child(
                    frame(256.0).child(
                        SplitView::from_spec(
                            SplitViewSpec::new(SplitOrientation::Vertical).with_default_ratio(0.5),
                            theme,
                        )
                        .with_primary(region("Editor", 220.0))
                        .with_secondary(region("Terminal", 280.0))
                    )
                )
        )
        // --- Horizontal with collapse toggles ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Horizontal with collapse toggles"), theme))
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
                        .with_secondary(region("Secondary", 140.0))
                    )
                )
        )
        // --- Vertical with collapse toggles ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Vertical with collapse toggles"), theme))
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
                        .with_secondary(region("Bottom", 280.0))
                    )
                )
        )
        // --- Nested splits (IDE-style layout) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Nested splits (IDE-style layout)"), theme))
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
                            .with_secondary(region("Terminal", 280.0))
                        )
                    )
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
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
                        .with_secondary(region("Right", 140.0))
                    )
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(color_to_hsla(text_secondary))
                        .child("Drag cursor and hover state disabled.")
                )
        )
        .child({
            let _ = panel_bg;
            div()
        })
}
