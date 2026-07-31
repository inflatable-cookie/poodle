//! Drawer — Jetstream slide-out panel backed by DrawerSpec.
//!
//! Contract: `docs/contracts/components/drawer.md`
//! Uses overlay() with backdrop, anchored to viewport edge.
//!
//! Anatomy (contract §2): Root → Backdrop (modal) + Surface → Header
//! (title + description) + Body + Actions (footer row). No close button —
//! the contract anatomy has no close affordance and Svelte ships none.
//!
//! Preview-loop / accepted limits:
//! - Open/close, escape/backdrop dismissal, focus trap, and scroll-lock live
//!   in the preview event loop, not the component (matches repo-wide Jetstream
//!   pattern). The backdrop renders unconditionally when modal.
//! - No ARIA channel (`role="dialog"` / `aria-modal`) — carried on the spec but
//!   not emitted.

use jetstream_ui::Color;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{DrawerEdge, DrawerSpec};

use crate::presentation::{
    drawer_title_font_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{elevation_dialog, resolve_color, resolve_px};

/// Render a Drawer surface anchored to the spec's edge.
///
/// `content` is the body region (`children` snippet); `actions` is the optional
/// footer action row (`actions` snippet). Both are caller-owned.
/// Drawer — a panel anchored to a viewport edge.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_x(handler)`.
///
/// The only dismissal route the component draws is the backdrop: the contract
/// anatomy has no close affordance, and escape is host-event-loop work.
pub struct Drawer {
    spec: DrawerSpec,
    theme: JetstreamThemeProvider,
    content: Option<JsEl>,
    actions: Option<JsEl>,
    on_request_close: Option<crate::element::ActionHandler>,
}

impl Drawer {
    pub fn from_spec(spec: DrawerSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            actions: None,
            on_request_close: None,
        }
    }

    pub fn content(mut self, content: impl crate::element::IntoJsEl) -> Self {
        self.content = Some(content.into_js_el());
        self
    }

    pub fn actions(mut self, actions: impl crate::element::IntoJsEl) -> Self {
        self.actions = Some(actions.into_js_el());
        self
    }

    /// Fires when the backdrop is clicked, if the drawer is modal and
    /// `dismiss_on_backdrop` allows it. A non-modal drawer draws no backdrop,
    /// so it has no dismissal route at all.
    pub fn on_request_close(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_request_close = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for Drawer {
    fn into_js_el(self) -> JsEl {
        build(
            &self.spec,
            &self.theme,
            self.content,
            self.actions,
            self.on_request_close,
        )
    }
}

pub fn js_drawer(spec: &DrawerSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    js_drawer_with_actions(spec, theme, content, None)
}

/// Render a Drawer with an explicit footer actions row.
pub fn js_drawer_with_actions(
    spec: &DrawerSpec,
    theme: &JetstreamThemeProvider,
    content: Option<JsEl>,
    actions: Option<JsEl>,
) -> JsEl {
    build(spec, theme, content, actions, None)
}

fn build(
    spec: &DrawerSpec,
    theme: &JetstreamThemeProvider,
    content: Option<JsEl>,
    actions: Option<JsEl>,
    on_request_close: Option<crate::element::ActionHandler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    // Contract §8 size table: header title font-size per size (md == 1rem).
    let title_font = rem_to_px(drawer_title_font_rem(effective_size));
    let body_font = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(spec.density));
    let space_y = rem_to_px(panel_space_y_rem(spec.density));
    // Contract: header internal gap 0.375rem; panel column gap + header/actions
    // margins resolve from space tokens (stack.sm / stack.md).
    let header_gap = rem_to_px(0.375);
    let panel_gap = resolve_px(theme, "space.stack.sm");
    let stack_md = resolve_px(theme, "space.stack.md");
    let actions_gap = resolve_px(theme, "space.inline.sm");

    let fill = resolve_color(theme, spec.surface_fill_token());
    let backdrop: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border = resolve_color(theme, "color.border.default");
    let title_color = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");

    // Contract §7 surface sizing — edge-anchored, NOT a per-size switch:
    //   left/right → width min(28rem, 100vw), height 100vh
    //   top/bottom → width 100vw,             height min(24rem, 100vh)
    // JsEl has no viewport-relative units; the rem ceiling resolves to a fixed
    // dimension, capped by the overlay box the drawer fills.
    let side_width = rem_to_px(28.0);
    let edge_height = rem_to_px(24.0);

    // Panel — direction-specific sizing and border edge.
    let panel = {
        // Token-accurate `elevation-dialog` (modal tier) resolved from the typed
        // semantic token via the runtime shadow builder (single layer, spread 0;
        // matches GPUI's mapping).
        let mut p = elevation_dialog(
            ui_element::div()
                // Contract: the drawer panel is a `dialog`.
                .aria_role(jetstream_ui::accesskit::Role::Dialog)
                .bg(fill)
                .flex_col()
                .gap(panel_gap)
                .pl(space_x)
                .pr(space_x)
                .pt(space_y)
                .pb(space_y),
        );

        p = match spec.edge {
            DrawerEdge::Right => p.h_full().w(side_width).border_l_1().border_color(border),
            DrawerEdge::Left => p.h_full().w(side_width).border_r_1().border_color(border),
            DrawerEdge::Bottom => p.w_full().h(edge_height).border_t_1().border_color(border),
            DrawerEdge::Top => p.w_full().h(edge_height).border_b_1().border_color(border),
        };

        p
    };

    let mut panel = panel;

    // Header: title + optional description (contract anatomy — no close button).
    // margin-bottom: space-stack-md separates the header from the body.
    if spec.title.is_some() || spec.description.is_some() {
        let mut header = ui_element::div().flex_col().gap(header_gap).mb(stack_md);

        if let Some(ref title) = spec.title {
            header = header.child(
                ui_element::label(title)
                    .text_color(title_color)
                    .text_size(title_font)
                    .text_weight(600),
            );
        }

        if let Some(ref description) = spec.description {
            header = header.child(
                ui_element::label(description)
                    .text_color(text_secondary)
                    .text_size(body_font),
            );
        }

        panel = panel.child(header);
    }

    // Body (flex-grows so the actions row pins to the panel bottom).
    if let Some(content_el) = content {
        panel = panel.child(ui_element::div().grow().child(content_el));
    }

    // Contract: Actions part — footer row, flex-end, wraps, margin-top: stack-md.
    if let Some(actions_el) = actions {
        panel = panel.child(
            ui_element::div()
                .flex_row()
                .flex_wrap()
                .items_center()
                .justify_end()
                .gap(actions_gap)
                .mt(stack_md)
                .child(actions_el),
        );
    }

    // Backdrop + panel as overlay — edge controls panel anchor position.
    let overlay = match spec.edge {
        DrawerEdge::Right => ui_element::div().bg(backdrop).overlay().flex_row().justify_end(),
        DrawerEdge::Left => ui_element::div().bg(backdrop).overlay().flex_row().justify_start(),
        DrawerEdge::Bottom => ui_element::div().bg(backdrop).overlay().flex_col().justify_end(),
        DrawerEdge::Top => ui_element::div().bg(backdrop).overlay().flex_col().justify_start(),
    };

    // Clicks bubble to the nearest clickable ancestor, so the panel takes an
    // inert handler of its own — without it, every click inside the drawer
    // would reach the backdrop and dismiss it.
    let mut overlay = overlay;
    let mut panel = panel;
    if let (true, true, Some(handler)) = (spec.is_modal, spec.dismiss_on_backdrop, &on_request_close) {
        let handler = std::sync::Arc::clone(handler);
        overlay = overlay.on_click(move |_event| handler());
        panel = panel.on_click(|_event| {});
    }

    let root = overlay.child(panel);
    crate::aria::with_aria_label(root, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> DrawerSpec {
        DrawerSpec::new()
            .with_open(true)
            .with_title("Settings")
            .with_description("Configure your preferences.")
    }

    #[test]
    fn renders_panel_header_title_and_description() {
        let th = theme();
        let body = ui_element::label("Body content");
        let actions = ui_element::label("Save");
        let el = js_drawer_with_actions(&spec(), &th, Some(body), Some(actions));
        let tree = probe(&el, 800.0, 600.0);

        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(tree.has_text("Settings"), "title missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Configure your preferences."),
            "description missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Body content"),
            "body content missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn renders_footer_actions_row() {
        let th = theme();
        let actions = ui_element::label("Save changes");
        let el = js_drawer_with_actions(&spec(), &th, None, Some(actions));
        let tree = probe(&el, 800.0, 600.0);

        assert!(
            tree.has_text("Save changes"),
            "footer action missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn renders_backdrop_with_overlay_fill() {
        let th = theme();
        let el = js_drawer(&spec(), &th, None);
        let tree = probe(&el, 800.0, 600.0);

        // Root overlay carries the resolved overlay backdrop fill.
        let backdrop_fill: Color =
            resolve_color(&th, DrawerSpec::new().backdrop_fill_token()).into();
        let expect = crate::render_probe::ProbeColor {
            r: backdrop_fill.r,
            g: backdrop_fill.g,
            b: backdrop_fill.b,
            a: backdrop_fill.a,
        };
        assert!(
            tree.has_background(expect, 0.02),
            "backdrop overlay fill missing from tree"
        );
    }

    #[test]
    fn omits_header_when_no_title_or_description() {
        let th = theme();
        let mut s = DrawerSpec::new().with_open(true);
        s.title = None;
        s.description = None;
        let body = ui_element::label("Just body");
        let el = js_drawer(&s, &th, Some(body));
        let tree = probe(&el, 800.0, 600.0);

        assert!(tree.has_text("Just body"));
        // No close-x icon anywhere (contract has no close affordance).
        assert!(
            !tree.texts().iter().any(|t| *t == "x"),
            "drawer should not render a close icon"
        );
    }

    #[test]
    fn no_close_icon_in_anatomy() {
        let th = theme();
        let el = js_drawer(&spec(), &th, Some(ui_element::label("Body")));
        let tree = probe(&el, 800.0, 600.0);
        assert_eq!(tree.count_kind("Icon"), 0, "drawer must not render any icon");
    }

    #[test]
    fn the_backdrop_requests_close() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = Drawer::from_spec(spec(), &theme())
            .on_request_close(move || { counter.fetch_add(1, Ordering::SeqCst); })
            .into_js_el();

        crate::element::click_probe::click_at(&el, 800.0, 600.0, 4.0, 300.0);

        assert_eq!(hits.load(Ordering::SeqCst), 1, "a backdrop click did not dismiss");
    }

    /// A non-modal drawer draws no backdrop at all, so it has no dismissal
    /// route — and the surface behind it stays usable, which is the point.
    #[test]
    fn a_non_modal_drawer_has_no_dismissal_route() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = Drawer::from_spec(spec().with_modal(false), &theme())
            .on_request_close(move || { counter.fetch_add(1, Ordering::SeqCst); })
            .into_js_el();

        crate::element::click_probe::click_at(&el, 800.0, 600.0, 4.0, 300.0);

        assert_eq!(hits.load(Ordering::SeqCst), 0, "a non-modal drawer dismissed");
    }

}
