use gpui::*;
use pug_gpui_primitives::CheckboxSpec;
use pug_gpui_components::PugCheckbox;
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let items = ["Option A", "Option B", "Option C"];
    let mut col = div().flex().flex_col().gap(px(6.0));

    for (i, label) in items.iter().enumerate() {
        let key = format!("checkbox-{}", i);
        let checked = state.specimens.is_on(&key);

        col = col.child(
            PugCheckbox::new(
                CheckboxSpec::new()
                    .with_checked(checked)
                    .with_label(*label),
                theme,
            )
            .with_id(format!("cb-{}", i))
            .on_change(cx.listener(move |this, _checked: &bool, _w, cx| {
                this.state.specimens.toggle(&format!("checkbox-{}", i));
                cx.notify();
            }))
        );
    }

    col = col.child(
        PugCheckbox::new(
            CheckboxSpec::new()
                .with_checked(true)
                .with_label("Disabled")
                .with_disabled(true),
            theme,
        )
        .with_id("cb-disabled")
    );

    col = col.child(
        PugCheckbox::new(
            CheckboxSpec::new()
                .with_mixed(true)
                .with_label("Mixed"),
            theme,
        )
        .with_id("cb-mixed")
    );

    col
}
