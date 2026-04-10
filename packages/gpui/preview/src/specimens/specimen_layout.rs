//! SpecimenLayout — three-tab (Examples / Sizes / Densities) specimen container.
//!
//! Mirrors the Svelte `SpecimenLayout` component (packages/svelte/preview/src/components/SpecimenLayout.svelte)
//! so the GPUI preview app structures each primitive specimen the same way:
//! a set of curated Examples + a full Sizes sweep (xs→xl) + a full Densities
//! sweep (compact / default / comfortable).
//!
//! Specimens pass their three panes as separate builder closures:
//!
//! ```ignore
//! specimen_layout(
//!     state, cx, "button",
//!     |theme| examples_body(theme),                        // Examples pane
//!     |size, theme| Button::...with_size(size).child("Enabled"),  // per-Size pane
//!     |density, theme| Button::...with_density(density).child("Toggle"), // per-Density pane
//! )
//! ```
//!
//! The current active tab is stored in `state.specimens.text` under the
//! key `specimen-layout-tab-<name>` so each specimen tracks its own tab
//! independently.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize};

use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

/// The three tabs that every SpecimenLayout exposes.
const TABS: &[(&str, &str)] = &[
    ("examples", "Examples"),
    ("sizes", "Sizes"),
    ("densities", "Densities"),
];

/// Every ControlSize iterated in the Sizes pane, in the display order
/// used throughout the Svelte preview.
pub const ALL_SIZES: &[(ControlSize, &str)] = &[
    (ControlSize::Xs, "xs"),
    (ControlSize::Sm, "sm"),
    (ControlSize::Md, "md"),
    (ControlSize::Lg, "lg"),
    (ControlSize::Xl, "xl"),
];

/// Every ControlDensity iterated in the Densities pane.
pub const ALL_DENSITIES: &[(ControlDensity, &str)] = &[
    (ControlDensity::Compact, "compact"),
    (ControlDensity::Default, "default"),
    (ControlDensity::Comfortable, "comfortable"),
];

/// Render a three-tab SpecimenLayout.
///
/// `name` is a short identifier used to scope the tab-state key so each
/// specimen remembers its own active tab.
///
/// `examples` is a pre-built element containing the Examples pane body
/// (typically a div built by the caller before invoking this helper).
/// Passing a concrete element rather than a closure lets the caller
/// freely capture `cx` when wiring per-example click handlers.
///
/// `sizes_row` and `densities_row` are closures invoked once per variant;
/// the helper takes care of the label column and stacking.
pub fn specimen_layout<SF, DF>(
    state: &AppState,
    cx: &mut Context<PreviewRoot>,
    name: &str,
    examples: AnyElement,
    sizes_row: SF,
    densities_row: DF,
) -> Div
where
    SF: Fn(ControlSize, &GpuiThemeProvider) -> AnyElement,
    DF: Fn(ControlDensity, &GpuiThemeProvider) -> AnyElement,
{
    let theme = &state.theme;

    let key = format!("specimen-layout-tab-{name}");
    let active_tab = state
        .specimens
        .text
        .get(&key)
        .cloned()
        .unwrap_or_else(|| "examples".to_string());

    let border_subtle = theme.resolve_color("color.border.subtle");
    let panel_bg = theme.resolve_color("color.background.panel");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");

    // ── Tab bar ──
    let mut tab_bar = div()
        .flex()
        .items_end()
        .gap(px(4.0))
        .border_b_1()
        .border_color(color_to_hsla(border_subtle));

    for (value, label) in TABS {
        let is_active = active_tab == *value;
        let tab_color = if is_active {
            color_to_hsla(text_primary)
        } else {
            color_to_hsla(text_secondary)
        };

        let mut tab = div()
            .id(SharedString::from(format!("specimen-tab-{name}-{value}")))
            .px(px(12.0))
            .py(px(8.0))
            .text_size(px(13.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(tab_color)
            .cursor_pointer()
            .child(*label);

        if is_active {
            tab = tab.border_b_2().border_color(color_to_hsla(accent));
        } else {
            tab = tab.border_b_2().border_color(hsla(0.0, 0.0, 0.0, 0.0));
        }

        let key_owned = key.clone();
        let value_owned = value.to_string();
        tab = tab.on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
            this.state
                .specimens
                .text
                .insert(key_owned.clone(), value_owned.clone());
            cx.notify();
        }));

        tab_bar = tab_bar.child(tab);
    }

    // ── Pane body ──
    let body: Div = match active_tab.as_str() {
        "sizes" => {
            let mut col = div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .p(px(16.0))
                .border_1()
                .border_color(color_to_hsla(border_subtle))
                .rounded(px(6.0))
                .bg(color_to_hsla(panel_bg).alpha(0.4));
            for (size, label) in ALL_SIZES {
                col = col.child(variant_row(*label, sizes_row(*size, theme), text_secondary));
            }
            col
        }
        "densities" => {
            let mut col = div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .p(px(16.0))
                .border_1()
                .border_color(color_to_hsla(border_subtle))
                .rounded(px(6.0))
                .bg(color_to_hsla(panel_bg).alpha(0.4));
            for (density, label) in ALL_DENSITIES {
                col = col.child(variant_row(*label, densities_row(*density, theme), text_secondary));
            }
            col
        }
        _ => {
            // "examples" tab (default)
            div().flex().flex_col().gap(px(16.0)).child(examples)
        }
    };

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(tab_bar)
        .child(body)
}

/// Single variant row: small label column on the left, the specimen body
/// on the right. Used inside the Sizes and Densities panes.
fn variant_row(label: &'static str, body: AnyElement, label_color: poodle_tokens::typed::ColorValue) -> Div {
    div()
        .flex()
        .items_start()
        .gap(px(16.0))
        .child(
            div()
                .w(px(80.0))
                .pt(px(4.0))
                .text_size(px(11.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(color_to_hsla(label_color))
                .child(label),
        )
        .child(div().flex_1().child(body))
}

/// Hsla extension: alpha helper for panel tinting.
trait HslaAlpha {
    fn alpha(self, a: f32) -> Self;
}

impl HslaAlpha for Hsla {
    fn alpha(mut self, a: f32) -> Self {
        self.a *= a;
        self
    }
}
