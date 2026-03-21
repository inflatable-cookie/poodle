use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::CheckboxSpec;
use pug_gpui_components::Checkbox;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    div().flex().flex_col().gap(px(16.0))
        // --- Default ---
        .child(section_label("DEFAULT", text_secondary))
        .child({
            let items: &[(&str, &str)] = &[
                ("checkbox-email", "Enable email notifications"),
                ("checkbox-marketing", "Subscribe to marketing emails"),
                ("checkbox-terms", "I agree to the terms and conditions"),
            ];
            let mut col = div().flex().flex_col().gap(px(12.0));
            for &(key, label) in items {
                let checked = state.specimens.is_on(key);
                let key_owned = key.to_string();
                col = col.child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(checked)
                            .with_label(label),
                        theme,
                    )
                    .with_id(key)
                    .on_change(cx.listener(move |this, _checked: &bool, _w, cx| {
                        this.state.specimens.toggle(&key_owned);
                        cx.notify();
                    }))
                );
            }
            col
        })
        // --- States ---
        .child(section_label("STATES", text_secondary))
        .child(
            div().flex().flex_col().gap(px(12.0))
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(false)
                            .with_disabled(true)
                            .with_label("Disabled unchecked"),
                        theme,
                    )
                    .with_id("cb-disabled-unchecked")
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_disabled(true)
                            .with_label("Disabled checked"),
                        theme,
                    )
                    .with_id("cb-disabled-checked")
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_mixed(true)
                            .with_label("Mixed / indeterminate"),
                        theme,
                    )
                    .with_id("cb-mixed")
                )
                .child(
                    Checkbox::from_spec(
                        CheckboxSpec::new()
                            .with_checked(true)
                            .with_disabled(true)
                            .with_label("Read-only checked"),
                        theme,
                    )
                    .with_id("cb-readonly-checked")
                )
        )
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
