use gpui::*;
use pug_gpui_primitives::SwitchSpec;
use pug_gpui_components::PugSwitch;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let switches = ["Notifications", "Dark mode", "Auto-save"];
    let mut col = div().flex().flex_col().gap(px(8.0));

    for (i, label) in switches.iter().enumerate() {
        let key = format!("switch-{}", i);
        let is_on = state.specimens.is_on(&key);

        let mut spec = SwitchSpec::new().with_checked(is_on);
        spec.label = Some(label.to_string());

        col = col.child(
            PugSwitch::new(spec, theme)
                .with_id(format!("sw-{}", i))
                .on_change(cx.listener(move |this, _checked: &bool, _w, cx| {
                    this.state.specimens.toggle(&format!("switch-{}", i));
                    cx.notify();
                }))
        );
    }

    col
}
