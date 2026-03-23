use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::SliderSpec;
use pug_gpui_components::Slider;
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

    div().flex().flex_col().gap(px(16.0)).max_w(px(320.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let mut spec = SliderSpec::new(volume).with_bounds(0.0, 100.0);
            spec.step = 1.0;
            spec.aria_label = Some("Volume".to_string());

            div().flex().flex_col().gap(px(4.0))
                .child(
                    Slider::from_spec(spec, theme)
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
        })
        // --- With step ---
        .child(section_label("WITH STEP", text_secondary))
        .child({
            let mut spec = SliderSpec::new(opacity).with_bounds(0.0, 100.0);
            spec.step = 10.0;
            spec.aria_label = Some("Opacity".to_string());

            div().flex().flex_col().gap(px(4.0))
                .child(
                    Slider::from_spec(spec, theme)
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
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = SliderSpec::new(40.0).with_bounds(0.0, 100.0);
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled".to_string());

            div().flex().flex_col().gap(px(4.0))
                .child(Slider::from_spec(spec, theme).with_id("slider-disabled"))
        })
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
