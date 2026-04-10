use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{RadioGroupSpec, ChoiceOption, Orientation, EyebrowSpec};
use poodle_gpui_components::{RadioGroup, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

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

    div().flex().flex_col().gap(px(24.0))
        // --- Vertical (default) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Vertical (default)"), theme))
                .child(
                    RadioGroup::from_spec(
                        RadioGroupSpec::new(plan_options.clone())
                            .with_value(plan_value.clone()),
                        theme,
                    )
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
        )
        // --- Horizontal ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Horizontal"), theme))
                .child(
                    RadioGroup::from_spec(
                        RadioGroupSpec::new(size_options)
                            .with_value(size_value.clone())
                            .with_orientation(Orientation::Horizontal),
                        theme,
                    )
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
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = RadioGroupSpec::new(plan_options)
                        .with_value("free");
                    spec.is_disabled = true;

                    RadioGroup::from_spec(spec, theme)
                        .with_id("radio-disabled")
                })
        )
        // --- Custom selected color ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Custom selected color"), theme))
                .child(
                    RadioGroup::from_spec(
                        RadioGroupSpec::new(vec![
                            ChoiceOption::new("free", "Free"),
                            ChoiceOption::new("pro", "Pro"),
                            ChoiceOption::new("enterprise", "Enterprise"),
                        ])
                            .with_value(plan_value.clone())
                            .with_selected_color("#22c55e"),
                        theme,
                    )
                    .with_id("radio-custom-color")
                    .on_change(cx.listener(|this, value: &str, _w, cx| {
                        this.state.specimens.text.insert("radio-plan".to_string(), value.to_string());
                        cx.notify();
                    }))
                )
        )
}
