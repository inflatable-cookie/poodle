use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{RadioGroupSpec, ChoiceOption, Orientation};
use pug_gpui_components::RadioGroup;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    let plan_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("free", "Free"),
        ChoiceOption::new("pro", "Pro"),
        ChoiceOption::new("enterprise", "Enterprise"),
    ];

    let size_options: Vec<ChoiceOption> = vec![
        ChoiceOption::new("sm", "Small"),
        ChoiceOption::new("md", "Medium"),
        ChoiceOption::new("lg", "Large"),
        ChoiceOption::new("xl", "Extra large"),
    ];

    // Read current selections from specimen state, defaulting to contract initial values
    let plan_value = state.specimens.text.get("radio-plan")
        .cloned()
        .unwrap_or_else(|| "pro".to_string());
    let size_value = state.specimens.text.get("radio-size")
        .cloned()
        .unwrap_or_else(|| "md".to_string());

    div().flex().flex_col().gap(px(16.0))
        // --- Vertical (default) ---
        .child(section_label("VERTICAL (DEFAULT)", text_secondary))
        .child({
            let spec = RadioGroupSpec::new(plan_options.clone())
                .with_value(plan_value.clone());

            div().flex().flex_col().gap(px(4.0))
                .child(
                    RadioGroup::from_spec(spec, theme)
                        .with_id("radio-plan")
                        .on_change(cx.listener(|this, value: &str, _w, cx| {
                            this.state.specimens.text.insert("radio-plan".to_string(), value.to_string());
                            cx.notify();
                        }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Selected: {}", plan_value))
                )
        })
        // --- Horizontal ---
        .child(section_label("HORIZONTAL", text_secondary))
        .child({
            let spec = RadioGroupSpec::new(size_options)
                .with_value(size_value.clone())
                .with_orientation(Orientation::Horizontal);

            div().flex().flex_col().gap(px(4.0))
                .child(
                    RadioGroup::from_spec(spec, theme)
                        .with_id("radio-size")
                        .on_change(cx.listener(|this, value: &str, _w, cx| {
                            this.state.specimens.text.insert("radio-size".to_string(), value.to_string());
                            cx.notify();
                        }))
                )
                .child(
                    div().text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("Selected: {}", size_value))
                )
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = RadioGroupSpec::new(plan_options)
                .with_value("free");
            spec.is_disabled = true;

            RadioGroup::from_spec(spec, theme)
                .with_id("radio-disabled")
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
