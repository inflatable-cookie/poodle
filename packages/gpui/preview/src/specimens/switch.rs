use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{SwitchSpec, CheckState, TriStateSwitchSpec};
use pug_gpui_components::{Switch, TriStateSwitch};
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
                ("switch-dark-mode", "Dark mode"),
                ("switch-auto-save", "Auto-save drafts"),
                ("switch-compact", "Compact view"),
            ];
            let mut col = div().flex().flex_col().gap(px(12.0));
            for &(key, label) in items {
                let is_on = state.specimens.is_on(key);
                let key_owned = key.to_string();
                let mut spec = SwitchSpec::new().with_checked(is_on);
                spec.label = Some(label.to_string());
                col = col.child(
                    Switch::from_spec(spec, theme)
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
        .child({
            let mut col = div().flex().flex_col().gap(px(12.0));

            // Disabled off
            let mut spec = SwitchSpec::new().with_checked(false);
            spec.label = Some("Disabled off".to_string());
            spec.is_disabled = true;
            col = col.child(
                Switch::from_spec(spec, theme).with_id("sw-disabled-off")
            );

            // Disabled on
            let mut spec = SwitchSpec::new().with_checked(true);
            spec.label = Some("Disabled on".to_string());
            spec.is_disabled = true;
            col = col.child(
                Switch::from_spec(spec, theme).with_id("sw-disabled-on")
            );

            // Read-only on (using disabled since GPUI doesn't support readonly)
            let mut spec = SwitchSpec::new().with_checked(true);
            spec.label = Some("Read-only on".to_string());
            spec.is_disabled = true;
            col = col.child(
                Switch::from_spec(spec, theme).with_id("sw-readonly-on")
            );

            col
        })

        // --- Tri-state switch ---
        .child(section_label("TRI-STATE SWITCH", text_secondary))
        .child({
            let filter_val = state.specimens.text.get("tri-state-filter")
                .cloned()
                .unwrap_or_else(|| "default".to_string());
            let check_state = match filter_val.as_str() {
                "excluded" => CheckState::Unchecked,
                "included" => CheckState::Checked,
                _ => CheckState::Mixed,
            };
            div().flex().flex_col().gap(px(8.0))
                .child(
                    TriStateSwitch::from_spec(
                        TriStateSwitchSpec::new()
                            .with_state(check_state)
                            .with_label("Filter mode"),
                        theme,
                    )
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        let current = this.state.specimens.text.get("tri-state-filter")
                            .cloned()
                            .unwrap_or_else(|| "default".to_string());
                        let next = match current.as_str() {
                            "excluded" => "default",
                            "default" => "included",
                            "included" => "excluded",
                            _ => "default",
                        };
                        this.state.specimens.text.insert("tri-state-filter".to_string(), next.to_string());
                        cx.notify();
                    }))
                )
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child(format!("Value: {}", filter_val))
                )
        })

        // --- Tri-state: Custom labels ---
        .child(section_label("TRI-STATE: CUSTOM LABELS", text_secondary))
        .child(
            TriStateSwitch::from_spec(
                TriStateSwitchSpec::new()
                    .with_state(CheckState::Mixed)
                    .with_label("Visibility filter"),
                theme,
            )
        )

        // --- Tri-state: Disabled ---
        .child(section_label("TRI-STATE: DISABLED", text_secondary))
        .child(
            TriStateSwitch::from_spec(
                TriStateSwitchSpec::new()
                    .with_state(CheckState::Checked)
                    .with_disabled(true)
                    .with_label("Disabled switch"),
                theme,
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
