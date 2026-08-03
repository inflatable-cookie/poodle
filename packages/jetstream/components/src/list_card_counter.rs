//! ListCardCounter — icon + count for list card footers ([`ListCardCounterSpec`]).
//!
//! Contract: `docs/contracts/components/list-card-counter.md`
//!
//! - **`href`:** Jetstream has no `<a>` widget; linked styling (`cursor_pointer`, hover
//!   color) matches the contract’s linked state. Navigation is a shell concern.
//! - **`tooltip`:** [`crate::tooltip::js_tooltip`] is panel-only; without a standard
//!   trigger+overlay helper the row renders alone (same as GPUI without
//!   `on_tooltip_open_change`).

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{IconSpec, ListCardCounterSpec};

use crate::icon::js_icon;
use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

/// Compact footer counter: decorative icon + numeric label.
/// ListCardCounter — an icon-count pair, optionally linked.
///
/// Mirrors the GPUI target's `on_link_click`. Only a linked counter fires —
/// `is_linked` is the condition that already draws the pointer cursor.
pub struct ListCardCounter {
    spec: ListCardCounterSpec,
    theme: JetstreamThemeProvider,
    on_link_click: Option<crate::element::ActionHandler>,
}

impl ListCardCounter {
    pub fn from_spec(spec: ListCardCounterSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_link_click: None,
        }
    }

    pub fn on_link_click(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_link_click = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for ListCardCounter {
    fn into_js_el(self) -> JsEl {
        let el = js_list_card_counter(&self.spec, &self.theme);

        match (self.spec.is_linked(), self.on_link_click) {
            (true, Some(handler)) => el.on_click(move |_event| handler()),
            _ => el,
        }
    }
}

pub fn js_list_card_counter(spec: &ListCardCounterSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let gap = rem_to_px(spec.gap_rem());
    let font_size = rem_to_px(spec.font_size_rem());
    let secondary = resolve_color(theme, ListCardCounterSpec::text_secondary_token());
    let primary = resolve_color(theme, ListCardCounterSpec::text_primary_token());

    let icon_el = js_icon(
        &IconSpec::new(spec.icon.clone()).with_size(ListCardCounterSpec::icon_size()),
        theme,
    )
    .w(rem_to_px(spec.icon_size_rem()))
    .h(rem_to_px(spec.icon_size_rem()));

    // Jetstream does not yet expose tabular numeral or anchor semantics in this
    // layer, so numeric-feature parity and literal link behavior remain
    // documented runtime deltas for now.
    let count = ui_element::label(&format!("{}", spec.count));

    let mut row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .text_size(font_size)
        .text_color(secondary)
        .child(icon_el)
        .child(count);

    if spec.is_linked() {
        let link_id = format!(
            "poodle-lcc-{}-{}",
            spec.icon.replace(['/', '\\', ' '], "-"),
            spec.count
        );
        row = row
            .id(link_id)
            .cursor_pointer()
            .hover(move |s| s.text_color(primary));
    }

    row
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::ListCardCounterSpec;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn a_linked_counter_reports_a_click() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let spec = ListCardCounterSpec::new("message-square", 12).with_href("/comments");
        let el = ListCardCounter::from_spec(spec, &theme())
            .on_link_click(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 200.0, 60.0, "12");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_link_click fired exactly once"
        );
    }

    /// An unlinked counter is a statistic, not a control.
    #[test]
    fn an_unlinked_counter_ignores_clicks() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el =
            ListCardCounter::from_spec(ListCardCounterSpec::new("message-square", 12), &theme())
                .on_link_click(move || {
                    counter.fetch_add(1, Ordering::SeqCst);
                })
                .into_js_el();

        crate::element::click_probe::click_text(&el, 200.0, 60.0, "12");

        assert_eq!(hits.load(Ordering::SeqCst), 0, "an unlinked counter fired");
    }
}
