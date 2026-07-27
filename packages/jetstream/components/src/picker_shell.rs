//! PickerShell — Jetstream picker shell backed by PickerShellSpec.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{BrowseState, PickerShellSpec};

use crate::presentation::rem_to_px;
use crate::spinner::js_spinner;
use crate::theme_ext::{color_mix, elevation_dialog, elevation_overlay, resolve_color, resolve_px, resolve_radius};

pub fn js_picker_shell(
    spec: &PickerShellSpec,
    theme: &JetstreamThemeProvider,
    toolbar: Option<JsEl>,
    selection: Option<JsEl>,
    body: Option<JsEl>,
    state_content: Option<JsEl>,
    footer: Option<JsEl>,
) -> JsEl {
    let panel_x = resolve_px(theme, "space.panel.x");
    let panel_y = resolve_px(theme, "space.panel.y");
    let gap_sm = resolve_px(theme, "space.inline.sm");
    let gap_md = resolve_px(theme, "space.inline.md");
    let stack_sm = resolve_px(theme, "space.stack.sm");
    let stack_md = resolve_px(theme, "space.stack.md");
    let label_size = resolve_px(theme, "typography.label.size");
    let body_size = resolve_px(theme, "typography.body.size");
    let panel = resolve_color(theme, "color.background.panel");
    let elevated = resolve_color(theme, "color.background.elevated");
    let surface = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let title_size = rem_to_px(1.25); // contract §8 title 1.25rem (no token resolves to this)
    let border_width = rem_to_px(0.0625); // contract border 0.0625rem
    let transparent = glam::Vec4::ZERO;

    // Contract §8: modal uses elevated background (96%); popover + inline keep
    // the panel background (94%). `color-mix(in srgb, <c> N%, transparent)`.
    let fill = if spec.is_modal() {
        color_mix(elevated, transparent, 0.96)
    } else {
        color_mix(panel, transparent, 0.94)
    };

    let mut shell = ui_element::div()
        .bg(fill)
        .border(border_width)
        .border_color(border)
        .rounded(radius)
        .flex_col()
        .gap(stack_md)
        .pl(panel_x)
        .pr(panel_x)
        .pt(panel_y)
        .pb(panel_y);

    // Variant elevation + width. Token-accurate per contract §8 (popover →
    // `elevation-overlay`, modal → `elevation-dialog`), resolved from the typed
    // semantic tokens via the runtime shadow builder (single layer, spread 0;
    // matches GPUI's mapping).
    if spec.is_popover() {
        shell = elevation_overlay(shell)
            .w_full()
            .max_w(rem_to_px(spec.popover_max_width_rem()));
    } else if spec.is_modal() {
        shell = elevation_dialog(shell);
    }

    let mut title_block = ui_element::div()
        .flex_col()
        .gap(stack_sm)
        .child(
            ui_element::label(&spec.title)
                .text_color(text_primary)
                .text_size(title_size)
                .text_weight(600),
        );

    if let Some(description) = spec.description.as_ref() {
        title_block = title_block.child(
            ui_element::label(description)
                .text_color(text_secondary)
                .text_size(label_size),
        );
    }

    let mut meta = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap_sm)
        .child(
            ui_element::label(&spec.selected_count_text())
                .text_color(text_secondary)
                .text_size(label_size),
        );

    if let Some(result_text) = spec.result_count_text() {
        meta = meta.child(
            ui_element::label(&result_text)
                .text_color(text_secondary)
                .text_size(label_size),
        );
    }

    shell = shell.child(
        ui_element::div()
            .flex_row()
            .justify_between()
            .items_start()
            .gap(gap_md)
            .child(title_block)
            .child(meta),
    );

    if let Some(toolbar) = toolbar {
        shell = shell.child(toolbar);
    }

    if let Some(selection) = selection {
        shell = shell.child(selection);
    }

    if let Some(status_text) = spec.status_text.as_ref() {
        // sr-only: keep in tree but clip to 1×1 so it claims no layout space
        // (mirrors the contract §8 sr-only clip + GPUI's 1px box). Jetstream has
        // no a11y channel, so this is visual-collapse only, not an SR live region.
        shell = shell.child(
            ui_element::div()
                .w(1.0)
                .h(1.0)
                .overflow_hidden()
                .child(
                    ui_element::label(status_text)
                        .text_color(text_secondary)
                        .text_size(label_size),
                ),
        );
    }

    if spec.state == BrowseState::Ready {
        if let Some(body) = body {
            shell = shell.child(body);
        }
    } else if let Some(state_content) = state_content {
        shell = shell.child(state_content);
    } else {
        let mut state = ui_element::div()
            .flex_col()
            .gap(stack_sm)
            .pl(panel_x)
            .pr(panel_x)
            .pt(panel_y * 1.5)
            .pb(panel_y * 1.5)
            .border(border_width)
            .border_color(border)
            .rounded(radius)
            .bg(surface);

        if spec.state == BrowseState::Loading {
            state = state.child(js_spinner(
                &poodle_specs::SpinnerSpec::new()
                    .with_variant(poodle_specs::SpinnerVariant::Grid)
                    .with_size(poodle_specs::SpinnerSize::Md)
                    .with_tone(poodle_specs::SpinnerTone::Accent),
                theme,
            ));
        }

        state = state.child(
            ui_element::label(spec.effective_state_title())
                .text_color(text_primary)
                .text_size(body_size)
                .text_weight(600),
        );

        if let Some(message) = spec.effective_state_message() {
            state = state.child(
                ui_element::label(message)
                    .text_color(text_secondary)
                    .text_size(label_size),
            );
        }

        shell = shell.child(state);
    }

    if let Some(footer) = footer {
        shell = shell.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(resolve_px(theme, spec.footer_gap_token()))
                .child(footer),
        );
    }

    crate::aria::with_aria_label(shell, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::PickerVariant;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn ready_renders_header_and_body() {
        let th = theme();
        let spec = PickerShellSpec::new("Select a component")
            .with_description("Browse components")
            .with_result_count(12);
        let body = ui_element::label("candidate-row");
        let el = js_picker_shell(&spec, &th, None, None, Some(body), None, None);
        let tree = probe(&el, 480.0, 400.0);

        // Header: title + description + meta counts.
        assert!(tree.has_text("Select a component"), "title missing: {:?}", tree.texts());
        assert!(tree.has_text("Browse components"), "description missing");
        assert!(tree.has_text("12 results"), "result count missing: {:?}", tree.texts());
        assert!(tree.has_text("0 selected"), "selection count missing");
        // Body content visible in ready state.
        assert!(tree.has_text("candidate-row"), "body content missing");
    }

    #[test]
    fn footer_slot_renders_actions() {
        let th = theme();
        let spec = PickerShellSpec::new("Pick");
        let footer = ui_element::label("Confirm");
        let el = js_picker_shell(&spec, &th, None, None, None, None, Some(footer));
        let tree = probe(&el, 480.0, 400.0);
        assert!(tree.has_text("Confirm"), "footer action missing: {:?}", tree.texts());
    }

    #[test]
    fn non_ready_state_shows_fallback_copy() {
        let th = theme();
        let spec = PickerShellSpec::new("Pick").with_state(BrowseState::NoResults);
        let body = ui_element::label("should-not-show");
        let el = js_picker_shell(&spec, &th, None, None, Some(body), None, None);
        let tree = probe(&el, 480.0, 400.0);
        // State area fallback (effective_state_*), not the body.
        assert!(tree.has_text("No results"), "fallback title missing: {:?}", tree.texts());
        assert!(tree.has_text("No results found."), "fallback message missing");
        assert!(!tree.has_text("should-not-show"), "body must be hidden when not ready");
    }

    #[test]
    fn surface_uses_panel_background_for_inline() {
        let th = theme();
        let spec = PickerShellSpec::new("Pick");
        let el = js_picker_shell(&spec, &th, None, None, None, None, None);
        let tree = probe(&el, 480.0, 400.0);
        // Root (shell surface) carries a background fill (panel @ 94%).
        let root = &tree.nodes[0];
        assert!(root.bg.is_some(), "shell surface should have a background fill");
        assert!(root.w > 0.0 && root.h > 0.0, "shell not laid out");
    }

    #[test]
    fn popover_variant_caps_width_at_32rem() {
        let th = theme();
        let spec = PickerShellSpec::new("Pick").with_variant(PickerVariant::Popover);
        let el = js_picker_shell(&spec, &th, None, None, None, None, None);
        // Wide viewport: popover must clamp to 32rem (512px), not fill 900px.
        let tree = probe(&el, 900.0, 400.0);
        let root = &tree.nodes[0];
        let cap = rem_to_px(32.0);
        assert!(
            root.w <= cap + 0.5,
            "popover width {} exceeds 32rem cap {cap}",
            root.w
        );
    }

    #[test]
    fn modal_variant_uses_elevated_background() {
        let th = theme();
        let elevated = color_mix(
            resolve_color(&th, "color.background.elevated"),
            glam::Vec4::ZERO,
            0.96,
        );
        let spec = PickerShellSpec::new("Pick").with_variant(PickerVariant::Modal);
        let el = js_picker_shell(&spec, &th, None, None, None, None, None);
        let tree = probe(&el, 480.0, 400.0);
        let root_bg = tree.nodes[0].bg.expect("modal shell should have a fill");
        let expected = crate::render_probe::ProbeColor {
            r: elevated.x,
            g: elevated.y,
            b: elevated.z,
            a: elevated.w,
        };
        assert!(
            root_bg.approx(expected, 0.02),
            "modal bg {root_bg:?} != elevated@96% {expected:?}"
        );
    }
}
