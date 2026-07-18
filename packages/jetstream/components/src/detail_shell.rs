//! DetailShell — Jetstream detail page shell backed by DetailShellSpec.
//!
//! Contract: `docs/contracts/components/detail-shell.md`
//! Reference: `packages/svelte/components/src/DetailShell.svelte`
//!
//! Anatomy (contract §3): a root `<section>` stacking an optional header region
//! (header slot OR `<h2>` title) over either the body (children, when
//! `state="ready"`) or a state region (custom `stateContent` slot OR the default
//! `<strong>` state-title + optional `<p>` state-message). The loading state
//! prepends the shared grid `Spinner`. All gaps/padding/surface resolve from
//! tokens. Root + region stacking gap = `space.stack.lg`.
//!
//! JsEl gap: no role/aria attributes (the runtime has no accessibility surface),
//! so `aria_label` is not emitted. Noted once in the parity doc. Scroll
//! ownership is approximated with `overflow_scroll` on the chosen region.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{DetailShellSpec, DetailState, ScrollOwner};
use poodle_specs::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::spinner::js_spinner;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

/// Build the Jetstream DetailShell.
///
/// `header` / `content` are the host's header and body slots; `state_content`
/// is an optional custom state-region slot (overrides the default state copy).
pub fn js_detail_shell(
    spec: &DetailShellSpec,
    theme: &JetstreamThemeProvider,
    header: Option<JsEl>,
    content: Option<JsEl>,
    state_content: Option<JsEl>,
) -> JsEl {
    let bg = resolve_color(theme, spec.body_fill_token());
    // Contract §9: root + region stacking gap = space.stack.lg.
    let stack_gap = resolve_px(theme, spec.stack_gap_token());
    let panel_x = resolve_px(theme, "space.panel.x");
    let panel_y = resolve_px(theme, "space.panel.y");
    let body_size = resolve_px(theme, "typography.body.size");
    let heading_size = resolve_px(theme, "typography.heading.size");

    let text_primary = resolve_color(theme, spec.state_title_color_token());
    let text_secondary = resolve_color(theme, spec.state_message_color_token());

    // Scroll ownership (contract §3 scrollMode): shell scrolls vs body scrolls.
    let scroll_shell = matches!(spec.scroll_owner, ScrollOwner::Shell);

    let mut shell = ui_element::div()
        .flex_col()
        .grow()
        .gap(stack_gap)
        .bg(bg)
        .when(scroll_shell, |s| s.overflow_scroll());

    // ── Header region (title OR header slot) ─────────────────────
    if spec.title.is_some() || header.is_some() {
        let mut header_region = ui_element::div()
            .flex_row()
            .items_center()
            .w_full()
            .gap(resolve_px(theme, "space.inline.sm"))
            .px(panel_x)
            .py(panel_y);

        if let Some(ref title) = spec.title {
            header_region = header_region.child(
                ui_element::label(title)
                    .text_color(text_primary)
                    .text_size(heading_size)
                    .text_weight(600)
                    .line_height(1.2),
            );
        }
        if let Some(h) = header {
            header_region = header_region.child(h);
        }
        shell = shell.child(header_region);
    }

    // ── Body / state region ──────────────────────────────────────
    if spec.state == DetailState::Ready {
        let mut body = ui_element::div()
            .flex_col()
            .grow()
            .gap(stack_gap)
            .w_full()
            .px(panel_x)
            .py(panel_y)
            .when(!scroll_shell, |s| s.overflow_scroll());
        if let Some(c) = content {
            body = body.child(c);
        }
        shell = shell.child(body);
    } else {
        // Contract §9-10: subtle surface, doubled panel-y / 1.5× panel-x padding,
        // radius-surface corners.
        let state_fill = resolve_color(theme, spec.state_fill_token());
        let state_mix = resolve_color(theme, spec.state_fill_mix_token());
        let state_bg = color_mix(state_fill, state_mix, 0.96);
        let state_radius = resolve_radius(theme, spec.state_radius_token());
        let state_pad_y = resolve_px(theme, spec.state_pad_y_token()) * 2.0;
        let state_pad_x = resolve_px(theme, spec.state_pad_x_token()) * 1.5;

        let mut region = ui_element::div()
            .flex_col()
            .grow()
            .gap(stack_gap)
            .w_full()
            .px(state_pad_x)
            .py(state_pad_y)
            .bg(state_bg)
            .rounded(state_radius);

        if let Some(custom) = state_content {
            region = region.child(custom);
        } else {
            // Loading prepends the shared grid spinner before the copy.
            if spec.state == DetailState::Loading {
                region = region.child(js_spinner(
                    &SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Grid)
                        .with_size(SpinnerSize::Md)
                        .with_tone(SpinnerTone::Accent),
                    theme,
                ));
            }
            // Default <strong> state title + optional <p> message.
            region = region.child(
                ui_element::label(spec.effective_state_title())
                    .text_color(text_primary)
                    .text_size(body_size)
                    .text_weight(600),
            );
            if let Some(ref message) = spec.state_message {
                region = region.child(
                    ui_element::label(message)
                        .text_color(text_secondary)
                        .text_size(body_size),
                );
            }
        }

        shell = shell.child(region);
    }

    shell
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    /// Ready state renders the header (title) and the body content slot.
    #[test]
    fn ready_renders_header_and_body() {
        let el = js_detail_shell(
            &DetailShellSpec::new().with_title("Account"),
            &theme(),
            None,
            Some(ui_element::label("Body content")),
            None,
        );
        let tree = probe(&el, 400.0, 300.0);
        assert!(tree.has_text("Account"), "title missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Body content"),
            "ready body slot missing: {:?}",
            tree.texts()
        );
    }

    /// Non-ready states replace the body with the state region (body slot hidden).
    #[test]
    fn non_ready_hides_body_shows_state() {
        let el = js_detail_shell(
            &DetailShellSpec::new().with_state(DetailState::Empty),
            &theme(),
            None,
            Some(ui_element::label("Body content")),
            None,
        );
        let tree = probe(&el, 400.0, 300.0);
        assert!(
            !tree.has_text("Body content"),
            "non-ready state must not render the body slot"
        );
        assert!(
            tree.has_text("Detail state"),
            "default state title fallback missing: {:?}",
            tree.texts()
        );
    }

    /// Error state shows custom state title + message.
    #[test]
    fn error_renders_custom_state_title_and_message() {
        let el = js_detail_shell(
            &DetailShellSpec::new()
                .with_state(DetailState::Error)
                .with_state_title("Failed to load")
                .with_state_message("Something went wrong. Please try again."),
            &theme(),
            None,
            None,
            None,
        );
        let tree = probe(&el, 400.0, 300.0);
        assert!(
            tree.has_text("Failed to load"),
            "custom state title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Something went wrong. Please try again."),
            "custom state message missing: {:?}",
            tree.texts()
        );
    }

    /// Loading state prepends the grid spinner before the loading copy.
    #[test]
    fn loading_prepends_spinner() {
        let el = js_detail_shell(
            &DetailShellSpec::new()
                .with_state(DetailState::Loading)
                .with_state_title("Loading"),
            &theme(),
            None,
            None,
            None,
        );
        let tree = probe(&el, 400.0, 300.0);
        assert!(tree.has_text("Loading"), "loading copy missing: {:?}", tree.texts());
        // The grid spinner is built from panels; the state region adds nodes
        // beyond just the title label, confirming the spinner was prepended.
        assert!(
            tree.count_kind("Panel") > 1,
            "grid spinner panels missing from loading state"
        );
    }

    /// A custom state-content slot overrides the default state copy.
    #[test]
    fn custom_state_content_overrides_default() {
        let el = js_detail_shell(
            &DetailShellSpec::new().with_state(DetailState::Empty),
            &theme(),
            None,
            None,
            Some(ui_element::label("Nothing here yet")),
        );
        let tree = probe(&el, 400.0, 300.0);
        assert!(
            tree.has_text("Nothing here yet"),
            "custom state content missing: {:?}",
            tree.texts()
        );
        assert!(
            !tree.has_text("Detail state"),
            "custom state content must suppress the default fallback title"
        );
    }
}
