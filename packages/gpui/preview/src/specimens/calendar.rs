use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{CalendarSpec, ControlDensity, ControlSize, EyebrowSpec};
use poodle_gpui_components::{Calendar, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");

    let selected_date = state.specimens.text.get("calendar-selected").cloned();
    let nav_month = state.specimens.text.get("calendar-nav-month").cloned();

    div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child({
                    let mut spec = CalendarSpec::new();
                    spec.aria_label = Some("Select a date".to_string());
                    if let Some(ref date) = selected_date {
                        spec.value = Some(date.clone());
                    }
                    if let Some(ref month) = nav_month {
                        spec.visible_month = Some(month.clone());
                    }
                    Calendar::from_spec(spec, theme)
                        .with_id("interactive")
                        .on_select(cx.listener(|this, date: &str, _w, cx| {
                            this.state.specimens.text.insert("calendar-selected".to_string(), date.to_string());
                            cx.notify();
                        }))
                        .on_navigate(cx.listener(|this, month: &str, _w, cx| {
                            this.state.specimens.text.insert("calendar-nav-month".to_string(), month.to_string());
                            cx.notify();
                        }))
                })
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!("Selected: {}", selected_date.as_deref().unwrap_or("(none)")))
                )
        )
        // --- With pre-selected date ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With pre-selected date"), theme))
                .child({
                    let mut spec = CalendarSpec::new();
                    spec.default_value = Some("2026-03-14".to_string());
                    spec.aria_label = Some("Calendar with default".to_string());
                    Calendar::from_spec(spec, theme).with_id("preselected")
                })
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child({
                    let sizes: &[(&str, ControlSize)] = &[
                        ("xs", ControlSize::Xs),
                        ("sm", ControlSize::Sm),
                        ("md", ControlSize::Md),
                        ("lg", ControlSize::Lg),
                        ("xl", ControlSize::Xl),
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &(key, size) in sizes {
                        let mut spec = CalendarSpec::new();
                        spec.default_value = Some("2026-03-14".to_string());
                        spec.aria_label = Some(format!("Calendar size {}", key));
                        col = col.child(
                            Calendar::from_spec(spec, theme)
                                .with_id(format!("size-{}", key))
                                .size(size)
                        );
                    }
                    col
                })
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child({
                    let densities: &[(&str, ControlDensity)] = &[
                        ("compact", ControlDensity::Compact),
                        ("default", ControlDensity::Default),
                        ("comfortable", ControlDensity::Comfortable),
                    ];
                    let mut col = div().flex().flex_col().gap(px(8.0));
                    for &(key, density) in densities {
                        let mut spec = CalendarSpec::new();
                        spec.default_value = Some("2026-03-14".to_string());
                        spec.aria_label = Some(format!("Calendar density {}", key));
                        col = col.child(
                            Calendar::from_spec(spec, theme)
                                .with_id(format!("density-{}", key))
                                .with_density(density)
                        );
                    }
                    col
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = CalendarSpec::new();
                    spec.default_value = Some("2026-03-01".to_string());
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled calendar".to_string());
                    Calendar::from_spec(spec, theme).with_id("disabled")
                })
        )
}
