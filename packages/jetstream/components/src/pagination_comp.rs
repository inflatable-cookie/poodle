//! Pagination — Jetstream pagination backed by PaginationSpec.
//!
//! Contract: `docs/contracts/components/pagination.md`
//! Reference: `packages/svelte/components/src/Pagination.svelte`
//!
//! Anatomy (per contract §2):
//! ```text
//! [Root]
//!   ├── [Info]               (optional — show_info && total > 0)
//!   └── [Controls Wrapper]
//!         ├── [Limit Selector] (optional — show_limit_selector)
//!         └── [Controls]
//!               ├── First   (full variant)
//!               ├── Previous
//!               ├── Pages / Summary  (variant-specific center)
//!               ├── Next
//!               └── Last    (full variant)
//! ```
//! Click / navigation lives in the preview event loop; this builds the visual tree.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlDensity, PageItem, PaginationSpec, PaginationVariant};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

/// Inter-control / inter-page gap in px for a density.
///
/// Contract §8 Density: compact `3px`, default `0.25rem`, comfortable `0.375rem`.
fn density_gap_px(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 3.0,
        ControlDensity::Default => rem_to_px(0.25),
        ControlDensity::Comfortable => rem_to_px(0.375),
    }
}

pub fn js_pagination(spec: &PaginationSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    // Contract: controls/pages gap is density-driven (not pad_x).
    let gap = density_gap_px(spec.density);
    let radius = resolve_radius(theme, spec.radius_token());

    let text_primary = resolve_color(theme, spec.button_text_token());
    let text_secondary = resolve_color(theme, spec.ellipsis_color_token());
    let border_color = resolve_color(theme, spec.button_border_token());
    let surface = resolve_color(theme, spec.button_fill_token());
    let accent = resolve_color(theme, spec.current_fill_token());

    // Current-page button: 18% accent bg, border at 42% accent / 58% default.
    let current_bg = tint(accent, 0.18);
    let current_border: Color = {
        let a: Color = accent.into();
        let b: Color = border_color.into();
        a.mix_srgb(b, 0.58)
    };

    // Border at 78% opacity (matches Svelte `color-mix … 78%`).
    let button_border = tint(border_color, 0.78);

    let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

    // ── root: info row + controls-wrapper stacked. ─────────────────────────────
    let mut root = ui_element::div().flex_col().gap(resolve_px(theme, "space.inline.sm"));

    // Chrome root treatment: padding + top border + elevated 92% background.
    // `resolved_chrome` applies the contract's precedence — `standalone` only
    // overrides when the host set it, otherwise `chrome` decides.
    if spec.resolved_chrome() {
        let elevated = resolve_color(theme, "color.background.elevated");
        root = root
            .bg(tint(elevated, 0.92))
            .border_t_1()
            .border_color(border_color)
            .px(resolve_px(theme, "space.panel.x"))
            .py(resolve_px(theme, "space.control.y"));
    }

    if spec.is_loading {
        root = root.opacity(disabled_opacity);
    }

    // ── info row ───────────────────────────────────────────────────────────────
    if spec.show_info {
        if let Some(text) = spec.info_string() {
            root = root.child(
                ui_element::label(&text)
                    .text_size(font_size)
                    .text_color(text_secondary),
            );
        }
    }

    // ── controls-wrapper: limit selector + controls. ───────────────────────────
    let mut wrapper = ui_element::div()
        .flex_row()
        .items_center()
        .gap(resolve_px(theme, "space.inline.md"));

    // Limit selector (all variants per contract §2 — "Show [n] per page").
    if spec.show_limit_selector && !spec.limit_options.is_empty() {
        wrapper = wrapper.child(build_limit_selector(
            spec, theme, height, font_size, pad_x, radius, gap,
        ));
    }

    // Controls row.
    let mut controls = ui_element::div().flex_row().items_center().gap(gap);

    // Helper: build a page-item / nav button (square-min-width, content-driven width).
    let make_button = |label: &str, is_current: bool, is_disabled: bool| {
        let bg: Color = if is_current {
            current_bg.into()
        } else {
            surface.into()
        };
        let bc: Color = if is_current {
            current_border
        } else {
            button_border.into()
        };

        let mut btn = ui_element::button(label)
            // Contract: min-width = control-height, height = control-height,
            // content-driven width with padding 0 control-x.
            .min_w(height)
            .h(height)
            .px(pad_x)
            .rounded(radius)
            .border_1()
            .border_color(bc)
            .bg(bg)
            .text_color(text_primary)
            .text_size(font_size)
            .text_weight(600)
            .flex_row()
            .items_center()
            .justify_center()
            .focusable()
            .cursor_pointer();

        if is_disabled || spec.is_loading {
            btn = btn.opacity(disabled_opacity).disabled(true);
        }

        btn
    };

    let is_full = spec.variant == PaginationVariant::Full;
    let prev_disabled = spec.is_first_page();
    let next_disabled = spec.is_last_page();

    // First button (`««`) — full variant only.
    if is_full {
        controls = controls.child(make_button("««", false, prev_disabled));
    }

    // Previous button — text "Prev" for simple, chevron icon otherwise.
    if spec.variant == PaginationVariant::Simple {
        controls = controls.child(make_button("Prev", false, prev_disabled));
    } else {
        controls = controls.child(arrow_button(
            "chevron-left",
            prev_disabled || spec.is_loading,
            height,
            font_size,
            radius,
            button_border,
            surface,
            text_primary,
            disabled_opacity,
            pad_x,
        ));
    }

    // Center content — variant-specific.
    match spec.variant {
        PaginationVariant::Numbered => {
            let mut pages_row = ui_element::div().flex_row().items_center().gap(gap);
            for item in spec.visible_pages() {
                match item {
                    PageItem::Page(n) => {
                        let is_current = n == spec.current_page;
                        pages_row = pages_row.child(make_button(&n.to_string(), is_current, false));
                    }
                    PageItem::Ellipsis => {
                        // Non-interactive ellipsis; contract min-width 1.5rem.
                        pages_row = pages_row.child(
                            ui_element::div()
                                .min_w(rem_to_px(1.5))
                                .h(height)
                                .flex_row()
                                .items_center()
                                .justify_center()
                                .child(
                                    ui_element::label("…")
                                        .text_size(font_size)
                                        .text_color(text_secondary),
                                ),
                        );
                    }
                }
            }
            controls = controls.child(pages_row);
        }
        PaginationVariant::Full => {
            // Contract: full center summary = "Page X of Y".
            controls = controls.child(
                ui_element::label(spec.full_summary())
                    .text_size(font_size)
                    .text_color(text_secondary)
                    .px(rem_to_px(0.5)),
            );
        }
        PaginationVariant::Simple => {
            // Contract: simple center summary = item range "X–Y of Z".
            controls = controls.child(
                ui_element::label(spec.simple_summary())
                    .text_size(font_size)
                    .text_color(text_secondary)
                    .px(rem_to_px(0.5)),
            );
        }
    }

    // Next button — text "Next" for simple, chevron icon otherwise.
    if spec.variant == PaginationVariant::Simple {
        controls = controls.child(make_button("Next", false, next_disabled));
    } else {
        controls = controls.child(arrow_button(
            "chevron-right",
            next_disabled || spec.is_loading,
            height,
            font_size,
            radius,
            button_border,
            surface,
            text_primary,
            disabled_opacity,
            pad_x,
        ));
    }

    // Last button (`»»`) — full variant only.
    if is_full {
        controls = controls.child(make_button("»»", false, next_disabled));
    }

    wrapper = wrapper.child(controls);
    root.child(wrapper)
}

// ── helpers ────────────────────────────────────────────────────────────────────

/// Chevron arrow nav button (prev / next). Square min-width, icon-only.
#[allow(clippy::too_many_arguments)]
fn arrow_button(
    icon: &str,
    is_disabled: bool,
    height: f32,
    font_size: f32,
    radius: f32,
    button_border: glam::Vec4,
    surface: glam::Vec4,
    text_primary: glam::Vec4,
    disabled_opacity: f32,
    pad_x: f32,
) -> JsEl {
    let mut btn = ui_element::button("")
        .min_w(height)
        .h(height)
        .px(pad_x)
        .rounded(radius)
        .border_1()
        .border_color(button_border)
        .bg(surface)
        .flex_row()
        .items_center()
        .justify_center()
        .focusable()
        .cursor_pointer()
        .child(
            ui_element::icon(icon)
                .w(font_size)
                .h(font_size)
                .text_color(text_primary),
        );
    if is_disabled {
        btn = btn.opacity(disabled_opacity).disabled(true);
    }
    btn
}

/// Limit selector: "Show [n ▾] per page" — matches Svelte limit row anatomy.
/// The select is rendered as a static bordered box (interaction is preview-loop).
#[allow(clippy::too_many_arguments)]
fn build_limit_selector(
    spec: &PaginationSpec,
    theme: &JetstreamThemeProvider,
    height: f32,
    font_size: f32,
    pad_x: f32,
    radius: f32,
    gap: f32,
) -> JsEl {
    let text_secondary = resolve_color(theme, spec.ellipsis_color_token());
    let text_primary = resolve_color(theme, spec.button_text_token());
    let border_color = tint(resolve_color(theme, spec.button_border_token()), 0.78);
    let surface = resolve_color(theme, spec.button_fill_token());

    let page_size_label = spec
        .page_size
        .map(|s| s.to_string())
        .unwrap_or_else(|| spec.limit_options[0].to_string());

    // Limit row gap = 0.375rem per contract §8 Limit Selector.
    ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.375))
        .child(
            ui_element::label("Show")
                .text_size(font_size)
                .text_color(text_secondary),
        )
        .child(
            // <select> visual: bordered box + value + chevron.
            ui_element::div()
                .h(height)
                .rounded(radius)
                .border_1()
                .border_color(border_color)
                .bg(surface)
                .flex_row()
                .items_center()
                .px(pad_x)
                .gap(gap)
                .child(
                    ui_element::label(&page_size_label)
                        .text_size(font_size)
                        .text_color(text_primary),
                )
                .child(
                    ui_element::icon("chevron-down")
                        .w(font_size)
                        .h(font_size)
                        .text_color(text_secondary),
                ),
        )
        .child(
            ui_element::label("per page")
                .text_size(font_size)
                .text_color(text_secondary),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::PaginationVariant;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Numbered variant renders page-number buttons plus ellipsis truncation.
    #[test]
    fn numbered_renders_pages_and_ellipsis() {
        let th = theme();
        let spec = PaginationSpec::new()
            .with_current_page(5)
            .with_total_pages(20)
            .with_sibling_count(1)
            .with_variant(PaginationVariant::Numbered);
        let tree = probe(&js_pagination(&spec, &th), 600.0, 80.0);

        // 1 … 4 5 6 … 20 — endpoints, current, and ellipsis all present.
        assert!(tree.has_text("1"), "missing page 1: {:?}", tree.texts());
        assert!(tree.has_text("5"), "missing current page 5");
        assert!(tree.has_text("20"), "missing last page 20");
        assert!(
            tree.has_text("…"),
            "missing ellipsis truncation; texts: {:?}",
            tree.texts()
        );
    }

    /// Current page button carries the accent-18% fill.
    #[test]
    fn current_page_has_accent_fill() {
        use crate::render_probe::ProbeColor;
        let th = theme();
        let spec = PaginationSpec::new()
            .with_current_page(3)
            .with_total_pages(5)
            .with_variant(PaginationVariant::Numbered);
        let tree = probe(&js_pagination(&spec, &th), 600.0, 80.0);

        let accent = resolve_color(&th, spec.current_fill_token());
        let want = tint(accent, 0.18);
        let want = ProbeColor { r: want.x, g: want.y, b: want.z, a: want.w };
        assert!(
            tree.has_background(want, 0.02),
            "current page button missing accent-18% fill; tree: {}",
            tree.to_json()
        );
    }

    /// Full variant renders first/last guillemets + "Page X of Y" summary.
    #[test]
    fn full_variant_has_first_last_and_summary() {
        let th = theme();
        let spec = PaginationSpec::new()
            .with_current_page(1)
            .with_total_pages(7)
            .with_page_size(20)
            .with_total_items(140)
            .with_variant(PaginationVariant::Full);
        let tree = probe(&js_pagination(&spec, &th), 700.0, 80.0);

        assert!(tree.has_text("««"), "missing first button; texts: {:?}", tree.texts());
        assert!(tree.has_text("»»"), "missing last button");
        assert!(
            tree.has_text("Page 1 of 7"),
            "missing full summary; texts: {:?}",
            tree.texts()
        );
        // Full variant must NOT render numbered page buttons.
        assert!(!tree.has_text("4"), "full variant leaked numbered pages");
    }

    /// Simple variant shows item-range summary, not "Page X of Y".
    #[test]
    fn simple_variant_shows_item_range() {
        let th = theme();
        let spec = PaginationSpec::new()
            .with_current_page(3)
            .with_total_pages(10)
            .with_page_size(25)
            .with_total_items(248)
            .with_variant(PaginationVariant::Simple);
        let tree = probe(&js_pagination(&spec, &th), 600.0, 80.0);

        assert!(tree.has_text("Prev"), "missing Prev; texts: {:?}", tree.texts());
        assert!(tree.has_text("Next"), "missing Next");
        assert!(
            tree.has_text("51–75 of 248"),
            "simple summary should be item range; texts: {:?}",
            tree.texts()
        );
    }

    /// Info row computes "Showing X to Y of Z".
    #[test]
    fn info_row_shows_range_of_total() {
        let th = theme();
        let spec = PaginationSpec::new()
            .with_current_page(3)
            .with_total_pages(10)
            .with_page_size(25)
            .with_total_items(248)
            .with_show_info(true)
            .with_variant(PaginationVariant::Numbered);
        let tree = probe(&js_pagination(&spec, &th), 700.0, 100.0);
        assert!(
            tree.has_text("Showing 51 to 75 of 248"),
            "missing info row; texts: {:?}",
            tree.texts()
        );
    }

    /// Limit selector renders "Show … per page" for any variant.
    #[test]
    fn limit_selector_renders_for_numbered() {
        let th = theme();
        let spec = PaginationSpec::new()
            .with_current_page(1)
            .with_total_pages(10)
            .with_page_size(25)
            .with_show_limit_selector(true)
            .with_limit_options(vec![10, 25, 50, 100])
            .with_variant(PaginationVariant::Numbered);
        let tree = probe(&js_pagination(&spec, &th), 700.0, 100.0);
        assert!(tree.has_text("Show"), "missing limit label; texts: {:?}", tree.texts());
        assert!(tree.has_text("per page"), "missing limit suffix");
        assert!(tree.has_text("25"), "missing selected page size");
    }
}
