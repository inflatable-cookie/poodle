//! ListGrid — responsive card/tile layout backed by [`ListGridSpec`].
//!
//! Contract: `docs/contracts/components/list-grid.md`
//!
//! Mirrors `packages/gpui/components/src/primitives/list_grid.rs`: `flex_wrap` with
//! per-cell `min_w` + `flex_1` approximates CSS `auto-fill` / `minmax`.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ListGridSpec, ListGridVariant};

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_px;

/// List grid layout: optional header row, then a responsive grid or stacked column.
pub fn js_list_grid(
    spec: &ListGridSpec,
    theme: &JetstreamThemeProvider,
    header: Option<JsEl>,
    children: Vec<JsEl>,
) -> JsEl {
    let gap = resolve_px(theme, spec.gap_token());
    let min_w = if let Some(em) = spec.min_item_width_em {
        rem_to_px(em)
    } else {
        resolve_px(theme, spec.min_item_width_token())
    };
    let header_gap = resolve_px(theme, ListGridSpec::header_actions_gap_token());
    let header_after = resolve_px(theme, spec.header_margin_bottom_token());

    let mut root = ui_element::div().flex_col().self_stretch();

    if let Some(h) = header {
        root = root.child(
            ui_element::div()
                .flex_row()
                .justify_end()
                .items_center()
                .gap(header_gap)
                .pb(header_after)
                .child(h),
        );
    }

    let grid = match spec.variant {
        ListGridVariant::Compact => {
            let mut g = ui_element::div().flex_col().gap(gap).self_stretch();
            for c in children {
                g = g.child(c);
            }
            g
        }
        ListGridVariant::Default => {
            let mut g = ui_element::div()
                .flex_row()
                .flex_wrap()
                .gap(gap)
                .self_stretch();
            for c in children {
                let cell = ui_element::div()
                    .min_w(min_w)
                    .flex_1()
                    .child(c);
                g = g.child(cell);
            }
            g
        }
    };

    root.child(grid)
}
