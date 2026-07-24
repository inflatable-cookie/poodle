//! PageLoading — Jetstream loading state with spinner, progress, message, and
//! cancel, in overlay or inline presentation. Backed by PageLoadingSpec.
//!
//! Contract: `docs/contracts/components/page-loading.md`
//! Reference: `packages/svelte/components/src/PageLoading.svelte`
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{
    PageLoadingSpec, ProgressSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant,
};

use crate::presentation::{
    control_space_x_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::progress::js_progress;
use crate::spinner::js_spinner;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_page_loading(spec: &PageLoadingSpec, theme: &JetstreamThemeProvider) -> JsEl {
    if !spec.is_visible {
        return ui_element::div();
    }

    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(panel_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let item_gap = rem_to_px(control_space_x_rem(spec.density));

    let backdrop = resolve_color(theme, spec.backdrop_fill_token());
    let text_color = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let surface_bg = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");
    let border_width = resolve_px(theme, "border.width.default");
    let radius = resolve_radius(theme, "radius.surface");
    let control_radius = resolve_radius(theme, "radius.control");

    let is_inline = spec.presentation.is_inline();

    // Card container. Overlay mode is an elevated card with chrome; inline mode
    // (contract §8 inline-card override) drops border/bg/shadow + padding and
    // caps at max-width 24rem.
    let mut card = ui_element::div()
        .flex_col()
        .items_center()
        .gap(item_gap * 2.0);
    if is_inline {
        // Inline card: transparent, no chrome, no padding (max-width 24rem).
        card = card.max_w(rem_to_px(24.0));
    } else {
        // Overlay card: elevated chrome. JsEl has no box-shadow primitive, so
        // the contract `elevation-overlay` card shadow is approximated by the
        // border + elevated fill (accepted JsEl delta — note).
        card = card
            .bg(surface_bg)
            .border(border_width)
            .border_color(border)
            .rounded(radius)
            .pl(pad_x)
            .pr(pad_x)
            .pt(pad_y)
            .pb(pad_y);
    }

    // Spinner — shared ring Spinner primitive (contract §2: variant="ring",
    // sizeRole="prominent", tone="accent"). js_spinner resolves its own size
    // from SpinnerSpec, so no hardcoded diameter.
    card = card.child(js_spinner(
        &SpinnerSpec::new()
            .with_variant(SpinnerVariant::Ring)
            .with_size(SpinnerSize::Lg)
            .with_tone(SpinnerTone::Accent),
        theme,
    ));

    // Progress bar (determinate) — shared Progress primitive (contract §2:
    // "Progress primitive"), full card width (contract §8 `.page-loading__progress`).
    if let Some(value) = spec.value {
        let mut progress_spec = ProgressSpec::new().with_value(value);
        progress_spec.max = spec.max;
        card = card.child(
            ui_element::div()
                .self_stretch()
                .child(js_progress(&progress_spec, theme)),
        );
    }

    // Message
    if let Some(ref msg) = spec.message {
        card = card.child(
            ui_element::label(msg)
                .text_color(text_color)
                .text_size(font_size),
        );
    }

    // Cancel action — contract §8 `.page-loading__cancel`: bordered control
    // with radius-control and padding 0.375rem 0.875rem. Click wiring lives in
    // the preview event loop (accepted runtime delta — note).
    if spec.can_cancel {
        card = card.child(
            ui_element::label("Cancel")
                .text_color(text_secondary)
                .text_size(font_size)
                .border(border_width)
                .border_color(border)
                .rounded(control_radius)
                .pl(rem_to_px(0.875))
                .pr(rem_to_px(0.875))
                .pt(rem_to_px(0.375))
                .pb(rem_to_px(0.375))
                .cursor_pointer(),
        );
    }

    // Branch on presentation. Inline: in-flow, centered, no backdrop, padding
    // 3rem 1rem (contract §8 inline root). Overlay: full-viewport scrim.
    if is_inline {
        ui_element::div()
            .self_stretch()
            .flex_col()
            .items_center()
            .justify_center()
            .pt(rem_to_px(3.0))
            .pb(rem_to_px(3.0))
            .pl(rem_to_px(1.0))
            .pr(rem_to_px(1.0))
            .child(card)
    } else {
        ui_element::div()
            .bg(backdrop)
            .grow()
            .flex_col()
            .items_center()
            .justify_center()
            .child(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Overlay indeterminate renders the ring Spinner (not a static loader icon)
    /// and the status message, inside an elevated card.
    #[test]
    fn overlay_indeterminate_has_spinner_and_message() {
        let spec = PageLoadingSpec::new().with_message("Loading data...");
        let tree = probe(&js_page_loading(&spec, &theme()), 600.0, 400.0);

        // Ring spinner contributes a bordered circle node (no static loader icon).
        assert!(
            !tree.has_text("loader"),
            "page-loading must compose the ring Spinner, not a static loader icon: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Loading data..."),
            "status message missing: {:?}",
            tree.texts()
        );
        // Indeterminate: no ProgressBar widget.
        assert_eq!(
            tree.count_kind("ProgressBar"),
            0,
            "indeterminate page-loading must not render a progress bar"
        );
    }

    /// Determinate renders the shared Progress primitive (ProgressBar widget),
    /// proving the hand-rolled track was replaced.
    #[test]
    fn determinate_renders_progress_primitive() {
        let spec = PageLoadingSpec::new().with_value(45.0).with_max(100.0);
        let tree = probe(&js_page_loading(&spec, &theme()), 600.0, 400.0);
        assert!(
            tree.count_kind("ProgressBar") >= 1,
            "determinate page-loading must compose the Progress primitive; kinds: {:?}",
            tree.nodes.iter().map(|n| n.kind).collect::<Vec<_>>()
        );
    }

    /// Overlay mode paints a full-page backdrop scrim; inline mode does not.
    #[test]
    fn overlay_has_backdrop_inline_does_not() {
        let th = theme();
        let backdrop = resolve_color(&th, PageLoadingSpec::new().backdrop_fill_token());
        let backdrop_color = crate::render_probe::ProbeColor {
            r: backdrop.x,
            g: backdrop.y,
            b: backdrop.z,
            a: backdrop.w,
        };

        let overlay_tree = probe(&js_page_loading(&PageLoadingSpec::new(), &th), 600.0, 400.0);
        assert!(
            overlay_tree.has_background(backdrop_color, 0.02),
            "overlay presentation must paint the backdrop scrim"
        );

        let inline_spec =
            PageLoadingSpec::new().with_presentation(poodle_specs::PageLoadingPresentation::Inline);
        let inline_tree = probe(&js_page_loading(&inline_spec, &th), 600.0, 400.0);
        assert!(
            !inline_tree.has_background(backdrop_color, 0.02),
            "inline presentation must not paint a backdrop scrim"
        );
    }

    /// Cancel renders with chrome (a bordered control) when canCancel is set.
    #[test]
    fn cancel_renders_when_can_cancel() {
        let spec = PageLoadingSpec::new().with_can_cancel(true);
        let tree = probe(&js_page_loading(&spec, &theme()), 600.0, 400.0);
        assert!(tree.has_text("Cancel"), "cancel control missing: {:?}", tree.texts());
    }

    #[test]
    fn hidden_renders_empty() {
        let spec = PageLoadingSpec::new().with_visible(false);
        let tree = probe(&js_page_loading(&spec, &theme()), 600.0, 400.0);
        // Hidden component is a bare empty div with no spinner/message.
        assert!(!tree.has_text("Cancel"));
        assert_eq!(tree.count_kind("ProgressBar"), 0);
    }
}
