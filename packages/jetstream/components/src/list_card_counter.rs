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
        let link_id = format!("poodle-lcc-{}-{}", spec.icon.replace(['/', '\\', ' '], "-"), spec.count);
        row = row
            .id(link_id)
            .cursor_pointer()
            .hover(move |s| s.text_color(primary));
    }

    row
}
