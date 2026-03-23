use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::CalendarSpec;
use pug_gpui_components::Calendar;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let selected_date = state.specimens.text.get("calendar-selected").cloned();
    let nav_month = state.specimens.text.get("calendar-nav-month").cloned();

    div().flex().flex_col().gap(px(16.0))
        // --- Interactive ---
        .child(section_label("INTERACTIVE", text_secondary))
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
        // --- With pre-selected date ---
        .child(section_label("WITH PRE-SELECTED DATE", text_secondary))
        .child({
            let mut spec = CalendarSpec::new();
            spec.default_value = Some("2026-03-14".to_string());
            spec.aria_label = Some("Calendar with default".to_string());
            Calendar::from_spec(spec, theme).with_id("preselected")
        })
        // --- Disabled ---
        .child(section_label("DISABLED", text_secondary))
        .child({
            let mut spec = CalendarSpec::new();
            spec.default_value = Some("2026-03-01".to_string());
            spec.is_disabled = true;
            spec.aria_label = Some("Disabled calendar".to_string());
            Calendar::from_spec(spec, theme).with_id("disabled")
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
