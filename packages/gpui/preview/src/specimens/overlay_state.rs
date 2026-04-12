use std::time::Duration;

use gpui::*;

use crate::PreviewRoot;

pub(crate) fn set_toggle(
    root: &mut PreviewRoot,
    key: &'static str,
    value: bool,
    cx: &mut Context<PreviewRoot>,
) {
    root.state.specimens.set_toggle(key, value);
    cx.notify();
}

pub(crate) fn set_toggle_via_entity(
    root: &WeakEntity<PreviewRoot>,
    key: impl Into<String>,
    value: bool,
    cx: &mut App,
) {
    let key = key.into();
    root.update(cx, |this, cx| {
        this.state.specimens.set_toggle(&key, value);
        cx.notify();
    })
    .ok();
}

pub(crate) fn sync_hover_intent(
    root: &WeakEntity<PreviewRoot>,
    open_key: impl Into<String>,
    hovered_key: impl Into<String>,
    hovered: bool,
    cx: &mut App,
) {
    let open_key = open_key.into();
    let hovered_key = hovered_key.into();
    root.update(cx, |this, cx| {
        this.state.specimens.set_toggle(&hovered_key, hovered);
        if !hovered {
            this.state.specimens.set_toggle(&open_key, false);
        }
        cx.notify();
    })
    .ok();
}

pub(crate) fn schedule_toggle_if(
    window: &mut Window,
    cx: &mut App,
    root: WeakEntity<PreviewRoot>,
    check_key: impl Into<String>,
    expected: bool,
    target_key: impl Into<String>,
    target_value: bool,
    delay_ms: u32,
) {
    let check_key = check_key.into();
    let target_key = target_key.into();
    window
        .spawn(cx, async move |cx| {
            cx.background_executor()
                .timer(Duration::from_millis(u64::from(delay_ms)))
                .await;
            cx.update(|_window, cx| {
                root.update(cx, |this, cx| {
                    if this.state.specimens.is_on(&check_key) == expected {
                        this.state.specimens.set_toggle(&target_key, target_value);
                        cx.notify();
                    }
                })
                .ok();
            })
            .ok();
        })
        .detach();
}
