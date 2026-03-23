use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{SliderSpec, EyebrowSpec};
use poodle_gpui_components::{Slider, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let volume = state.specimens.text.get("slider-volume")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(65.0);
    let opacity = state.specimens.text.get("slider-opacity")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(100.0);

    div().flex().flex_col().gap(px(24.0)).max_w(px(320.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child(
                    Slider::from_spec(
                        {
                            let mut spec = SliderSpec::new(volume).with_bounds(0.0, 100.0);
                            spec.step = 1.0;
                            spec.aria_label = Some("Volume".to_string());
                            spec
                        },
                        theme,
                    )
                    .with_id("slider-volume")
                    .on_change(cx.listener(|this, val: &f64, _w, cx| {
                        this.state.specimens.text.insert(
                            "slider-volume".to_string(),
                            format!("{:.0}", val),
                        );
                        cx.notify();
                    }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Volume: {:.0}%", volume))
                )
        )
        // --- With step ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With step"), theme))
                .child(
                    Slider::from_spec(
                        {
                            let mut spec = SliderSpec::new(opacity).with_bounds(0.0, 100.0);
                            spec.step = 10.0;
                            spec.aria_label = Some("Opacity".to_string());
                            spec
                        },
                        theme,
                    )
                    .with_id("slider-opacity")
                    .on_change(cx.listener(|this, val: &f64, _w, cx| {
                        this.state.specimens.text.insert(
                            "slider-opacity".to_string(),
                            format!("{:.0}", val),
                        );
                        cx.notify();
                    }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Opacity: {:.0}%", opacity))
                )
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(10.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(
                    Slider::from_spec(
                        {
                            let mut spec = SliderSpec::new(40.0).with_bounds(0.0, 100.0);
                            spec.is_disabled = true;
                            spec.aria_label = Some("Disabled slider".to_string());
                            spec
                        },
                        theme,
                    )
                    .with_id("slider-disabled")
                )
        )
}
