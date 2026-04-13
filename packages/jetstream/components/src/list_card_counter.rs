//! ListCardCounter — icon + count for list card footers ([`ListCardCounterSpec`]).
//!
//! Contract: `docs/contracts/components/list-card-counter.md`
//!
//! - **`href`:** Jetstream has no `<a>` widget; linked styling (`cursor_pointer`, hover
//!   color) matches the contract’s linked state. Navigation is a shell concern.
//! - **`tooltip`:** [`crate::tooltip::js_tooltip`] is panel-only; without a standard
//!   trigger+overlay helper the row renders alone (same as GPUI without
//!   `on_tooltip_open_change`).

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{IconSpec, ListCardCounterSpec};

use crate::icon::js_icon;
use crate::theme_ext::{resolve_color, resolve_px};

/// Compact footer counter: decorative icon + numeric label.
pub fn js_list_card_counter(spec: &ListCardCounterSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let gap = resolve_px(theme, ListCardCounterSpec::gap_token());
    let font_size = resolve_px(theme, ListCardCounterSpec::font_size_token());
    let secondary = resolve_color(theme, ListCardCounterSpec::text_secondary_token());
    let primary = resolve_color(theme, ListCardCounterSpec::text_primary_token());

    let icon_el = js_icon(
        &IconSpec::new(spec.icon.clone()).with_size(ListCardCounterSpec::icon_size()),
        theme,
    );

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
