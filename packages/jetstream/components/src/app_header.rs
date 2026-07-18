//! AppHeader — Jetstream app header backed by AppHeaderSpec.
//!
//! Contract: `docs/contracts/components/app-header.md`
//! Reference: `packages/svelte/components/src/AppHeader.svelte`
//!
//! Renders the contract §2 three-region shell:
//!   - Identity region (title group: title + optional subtitle, or a custom
//!     identity slot)
//!   - Actions region (optional global-actions slot — compose `js_button` etc.)
//!   - Utility region (optional trailing utility slot — compose `js_icon_button`)
//!
//! Jetstream has no CSS grid; the contract `grid minmax(0,1fr) auto auto` is
//! emulated with flex — identity grows (`grow` + `min_w_0` for truncation),
//! actions/utility hold intrinsic width (`flex_shrink_0`), utility justifies
//! to the end. All dimensions/colors resolve from tokens or contract-exact rem
//! ladders carried on `AppHeaderSpec`; zero hardcoded literals.
//!
//! `aria_label` is stored on the spec for parity but this runtime has no
//! accessibility channel. Child-action click handling lives in the host
//! preview event loop, not the component.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::AppHeaderSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::resolve_color;

/// Build a Jetstream app-header element from an `AppHeaderSpec`, rendering only
/// the identity region (default title group). Use [`js_app_header_with_slots`]
/// to compose the actions / utility regions.
pub fn js_app_header(spec: &AppHeaderSpec, theme: &JetstreamThemeProvider) -> JsEl {
    js_app_header_with_slots(spec, theme, None, None, None)
}

/// Build the full three-region app header.
///
/// - `identity` — optional custom identity slot; replaces the default title
///   group built from `spec.title` / `spec.subtitle`.
/// - `actions` — optional global-actions cluster (e.g. `js_button` row).
/// - `utility` — optional trailing utility cluster (e.g. `js_icon_button` row),
///   right-aligned.
pub fn js_app_header_with_slots(
    spec: &AppHeaderSpec,
    theme: &JetstreamThemeProvider,
    identity: Option<JsEl>,
    actions: Option<JsEl>,
    utility: Option<JsEl>,
) -> JsEl {
    // ── Token / contract-rem resolution ──────────────────────────
    let panel: Color = resolve_color(theme, spec.background_token()).into();
    // Contract §9: color-mix(background-panel 94%, transparent) → panel @ 94% alpha.
    let bg = Color::new(panel.r, panel.g, panel.b, panel.a * 0.94);
    let border: Color = resolve_color(theme, spec.border_token()).into();
    let title_color: Color = resolve_color(theme, spec.title_color_token()).into();
    let subtitle_color: Color = resolve_color(theme, spec.subtitle_color_token()).into();

    // Size ladder (height + title/subtitle font) and density ladder
    // (region gaps + padding) all carried on the spec.
    let min_height = rem_to_px(spec.min_height_rem());
    let title_font = rem_to_px(spec.title_size_rem());
    let subtitle_font = rem_to_px(spec.subtitle_size_rem());
    let grid_gap = rem_to_px(spec.gap_rem());
    let region_gap = rem_to_px(spec.region_gap_rem());
    let pad_y = rem_to_px(spec.pad_y_rem());
    let pad_x = rem_to_px(spec.pad_x_rem());

    // ── Identity region (grid column 1: minmax(0, 1fr)) ──────────
    // Grows to fill; min-width 0 so the subtitle can truncate.
    let mut identity_region = ui_element::div()
        .flex_row()
        .items_center()
        .gap(region_gap)
        .grow()
        .min_w_0();

    if let Some(custom) = identity {
        identity_region = identity_region.child(custom);
    } else if spec.title.is_some() {
        // Default title group: title + optional subtitle.
        let mut title_group = ui_element::div()
            .flex_row()
            .items_center()
            .gap(region_gap)
            .min_w_0();

        if let Some(ref title) = spec.title {
            title_group = title_group.child(
                ui_element::label(title)
                    .text_color(title_color)
                    .text_size(title_font)
                    .text_weight(600)
                    .line_height(1.2)
                    .whitespace_nowrap(),
            );
        }

        if let Some(ref subtitle) = spec.subtitle {
            title_group = title_group.child(
                ui_element::label(subtitle)
                    .text_color(subtitle_color)
                    .text_size(subtitle_font)
                    .line_height(1.2)
                    .whitespace_nowrap()
                    .text_ellipsis(),
            );
        }

        identity_region = identity_region.child(title_group);
    }

    // ── Root shell ───────────────────────────────────────────────
    // `.w_full()` fills the parent's width (the contract grid column track);
    // `.min_h()` is the contract min-height — intentionally NOT `.grow()`,
    // which would zero the min-height (CSS min-height:0 trick) and force a
    // vertical stretch the shell must not have.
    let mut header = ui_element::div()
        .bg(bg)
        .border_b_1()
        .border_color(border)
        .w_full()
        .min_h(min_height)
        .flex_row()
        .items_center()
        .gap(grid_gap)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .child(identity_region);

    // ── Actions region (grid column 2: auto) ─────────────────────
    if let Some(actions) = actions {
        header = header.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(region_gap)
                .flex_shrink_0()
                .child(actions),
        );
    }

    // ── Utility region (grid column 3: auto, justify-end) ────────
    if let Some(utility) = utility {
        header = header.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .justify_end()
                .gap(region_gap)
                .flex_shrink_0()
                .child(utility),
        );
    }

    header
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::button::js_button;
    use crate::icon_button::js_icon_button;
    use crate::render_probe::probe;
    use poodle_specs::{ButtonSpec, ButtonVariant, IconButtonSpec};

    fn test_theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_title_subtitle_and_action_label() {
        let theme = test_theme();
        let spec = AppHeaderSpec::new()
            .with_title("Poodle Studio")
            .with_subtitle("Untitled Project");

        // Actions cluster: a single ghost button labeled "New".
        let actions = ui_element::div().flex_row().gap(4.0).child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Ghost)
                .with_label("New"),
            &theme,
        ));
        // Utility cluster: a settings icon button.
        let utility = ui_element::div().flex_row().gap(4.0).child(js_icon_button(
            &IconButtonSpec::new().with_icon("settings"),
            &theme,
        ));

        let el = js_app_header_with_slots(&spec, &theme, None, Some(actions), Some(utility));
        let tree = probe(&el, 800.0, 64.0);

        // Not title-only: title + subtitle + an action label all render.
        assert!(tree.has_text("Poodle Studio"), "title should render");
        assert!(tree.has_text("Untitled Project"), "subtitle should render");
        assert!(tree.has_text("New"), "action button label should render");
    }

    #[test]
    fn shell_has_background_and_bottom_border() {
        let theme = test_theme();
        let el = js_app_header(&AppHeaderSpec::new().with_title("App"), &theme);
        assert!(el.style.background.is_some(), "shell should have a fill");
        // `border_b_1()` is a per-edge bottom border (border_widths[2]).
        assert!(
            el.style.border_widths[2] > 0.0,
            "shell should have a bottom border"
        );
    }

    #[test]
    fn size_ladder_changes_min_height() {
        use poodle_specs::ControlSize;
        let theme = test_theme();
        let sm = js_app_header(&AppHeaderSpec::new().with_title("A").with_size(ControlSize::Sm), &theme);
        let xl = js_app_header(&AppHeaderSpec::new().with_title("A").with_size(ControlSize::Xl), &theme);
        assert_ne!(
            sm.layout.min_size.height, xl.layout.min_size.height,
            "sm and xl should resolve different shell heights"
        );
    }
}
