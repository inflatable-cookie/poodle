use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_composites::{SplitViewSpec, SplitOrientation};
use poodle_primitives::EyebrowSpec;
use poodle_gpui_components::{SplitView, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let h_split_spec = SplitViewSpec::new(SplitOrientation::Horizontal)
        .with_default_ratio(0.5);
    let v_split_spec = SplitViewSpec::new(SplitOrientation::Vertical)
        .with_default_ratio(0.5);

    div().flex().flex_col().gap(px(24.0))
        // --- Horizontal split ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Horizontal split"), theme))
                .child(
                    div().h(px(100.0)).child(
                        SplitView::from_spec(h_split_spec, theme)
                            .with_primary(
                                div().p(px(8.0))
                                    .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Primary pane"))
                            )
                            .with_secondary(
                                div().p(px(8.0))
                                    .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Secondary pane"))
                            )
                    )
                )
        )
        // --- Vertical split ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Vertical split"), theme))
                .child(
                    div().h(px(120.0)).child(
                        SplitView::from_spec(v_split_spec, theme)
                            .with_primary(
                                div().p(px(8.0))
                                    .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Primary pane"))
                            )
                            .with_secondary(
                                div().p(px(8.0))
                                    .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Secondary pane"))
                            )
                    )
                )
        )
}
