//! Dialog — Jetstream dialog container backed by DialogSpec.
//!
//! Contract: `docs/contracts/components/dialog.md`
//! Uses overlay() for modal rendering with backdrop.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::DialogSpec;

use crate::presentation::{
    control_height_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{elevation_dialog, resolve_color, resolve_px, resolve_radius};
use poodle_specs::SemanticControlSizeRole;

/// Dialog — a modal surface over a backdrop.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_x(handler)`.
///
/// The event is `on_request_close`, not `on_open_change`: the component does
/// not own the open state and cannot close itself. It reports that a dismissal
/// route was taken and the host decides.
pub struct Dialog {
    spec: DialogSpec,
    theme: JetstreamThemeProvider,
    children: Vec<JsEl>,
    actions: Option<JsEl>,
    on_request_close: Option<crate::element::ActionHandler>,
}

impl Dialog {
    pub fn from_spec(spec: DialogSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
            actions: None,
            on_request_close: None,
        }
    }

    pub fn child(mut self, child: impl crate::element::IntoJsEl) -> Self {
        self.children.push(child.into_js_el());
        self
    }

    pub fn actions(mut self, actions: impl crate::element::IntoJsEl) -> Self {
        self.actions = Some(actions.into_js_el());
        self
    }

    /// Fires when a dismissal route is taken — the close button, or the
    /// backdrop when `dismiss_on_backdrop` allows it.
    pub fn on_request_close(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_request_close = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for Dialog {
    fn into_js_el(self) -> JsEl {
        build(
            &self.spec,
            &self.theme,
            self.children,
            self.actions,
            self.on_request_close,
        )
    }
}

pub fn js_dialog(
    spec: &DialogSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
    actions: Option<JsEl>,
) -> JsEl {
    build(spec, theme, children, actions, None)
}

fn build(
    spec: &DialogSpec,
    theme: &JetstreamThemeProvider,
    children: Vec<JsEl>,
    actions: Option<JsEl>,
    on_request_close: Option<crate::element::ActionHandler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    // Title is 1rem at Md; body follows the size scale.
    let title_font = rem_to_px(1.0_f32.max(size_font_rem(effective_size) + 0.1875));
    let body_font = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(spec.density));
    let space_y = rem_to_px(panel_space_y_rem(spec.density));

    let fill = resolve_color(theme, spec.surface_fill_token());
    let backdrop_fill: Color = resolve_color(theme, spec.backdrop_fill_token()).into();
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");
    let title_color = resolve_color(theme, "color.text.primary");
    let desc_color = resolve_color(theme, "color.text.secondary");
    let muted_color = resolve_color(theme, "color.text.secondary");

    // Contract §8 section spacing — distinct, token-resolved gaps (not a flat 1rem):
    // header internal gap 0.375rem, header margin-bottom = space.stack.md,
    // actions internal gap = space.inline.sm, actions margin-top = space.stack.lg.
    let header_gap = rem_to_px(0.375);
    let header_mb = resolve_px(theme, "space.stack.md");
    let actions_gap = resolve_px(theme, "space.inline.sm");
    let actions_mt = resolve_px(theme, "space.stack.lg");
    // Close button is an IconButton at chrome size (one step down from the dialog).
    let chrome_size = resolve_semantic_size(effective_size, SemanticControlSizeRole::Chrome);
    let close_dim = rem_to_px(control_height_rem(chrome_size));

    // ── Width ────────────────────────────────────────────────────────────────
    // `surface_width_rem()` returns INFINITY for Full; all others are finite rem values.
    let width_rem = spec.surface_width_rem();

    // ── Panel ────────────────────────────────────────────────────────────────
    // No flat panel gap — per-section spacing comes from header margin-bottom /
    // actions margin-top (contract §8), not a uniform 1rem.
    // Token-accurate `elevation-dialog` (modal tier) resolved from the typed
    // semantic token via the runtime shadow builder (single layer, spread 0;
    // matches GPUI's mapping).
    let mut panel = elevation_dialog(
        ui_element::div()
            .bg(fill)
            .border(1.0)
            .border_color(border)
            .rounded(radius)
            .flex_col()
            // Contract §7: surface max-height min(80vh, 42rem). The 80vh term is
            // viewport-relative (the centering parent already constrains it); cap
            // at the 42rem rem-term here so tall content scrolls within the surface.
            .max_h(rem_to_px(42.0))
            .overflow_y_hidden(),
    );

    if width_rem.is_finite() {
        panel = panel.w(rem_to_px(width_rem));
    } else {
        // Full — fill the overlay (constrained by the centering flex parent)
        panel = panel.grow();
    }

    // ── Bare mode ────────────────────────────────────────────────────────────
    if spec.bare {
        // No internal padding; close button floats absolutely when needed.
        // Children fill the panel directly.
        for child in children {
            panel = panel.child(child);
        }

        return backdrop(theme, backdrop_fill, spec, panel, on_request_close);
    }

    // ── Non-bare: apply padding and build chrome ─────────────────────────────
    panel = panel.pl(space_x).pr(space_x).pt(space_y).pb(space_y);

    // Header row: title/description on the left, optional close button on the right.
    let has_header = spec.title.is_some() || spec.description.is_some() || spec.show_close_button;
    if has_header {
        let mut header_col = ui_element::div().flex_col().gap(header_gap).grow();

        if let Some(ref title) = spec.title {
            header_col = header_col.child(
                ui_element::label(title)
                    .text_color(title_color)
                    .text_size(title_font)
                    .text_weight(600),
            );
        }

        if let Some(ref description) = spec.description {
            header_col = header_col.child(
                ui_element::label(description)
                    .text_color(desc_color)
                    .text_size(body_font),
            );
        }

        let mut header_row = ui_element::div()
            .flex_row()
            .items_start()
            .justify_between()
            .gap(actions_gap)
            .mb(header_mb)
            .child(header_col);

        if spec.show_close_button {
            // IconButton at chrome size (one step down from the dialog size),
            // square per the control-height token; glyph follows the size scale.
            let icon_size = rem_to_px(size_font_rem(chrome_size));
            let mut close = ui_element::button("")
                // `close_label` has been on the spec all along, defaulting
                // to "Close dialog" as Svelte does; this component simply
                // never used it, so the glyph announced as an unnamed
                // button.
                .aria_label(spec.close_label.clone())
                .id("poodle-dialog-close")
                .w(close_dim)
                .h(close_dim)
                .flex_row()
                .items_center()
                .justify_center()
                .rounded(resolve_radius(theme, "radius.control"))
                .cursor_pointer()
                .focusable()
                .child(
                    ui_element::icon("x")
                        .w(icon_size)
                        .h(icon_size)
                        .text_color(muted_color),
                );

            // The close button dismisses whatever `dismiss_on_backdrop` says:
            // it is the explicit route, not the incidental one.
            if let Some(handler) = &on_request_close {
                let handler = std::sync::Arc::clone(handler);
                close = close.on_click(move |_event| handler());
            }

            header_row = header_row.child(close);
        }

        panel = panel.child(header_row);
    }

    // ── Body children ────────────────────────────────────────────────────────
    // Contract §8 `.dialog__body`: min-width 0 so long content can shrink/wrap.
    if !children.is_empty() {
        let mut body = ui_element::div().flex_col().min_w_0().self_stretch();
        for child in children {
            body = body.child(child);
        }
        panel = panel.child(body);
    }

    // ── Actions slot ─────────────────────────────────────────────────────────
    // Contract §8 `.dialog__actions`: flex row, wrap, end-justified,
    // gap = space.inline.sm, margin-top = space.stack.lg. No divider (Svelte
    // has none — the earlier 1px rule was a non-contract addition, removed).
    if let Some(actions_el) = actions {
        panel = panel.child(
            ui_element::div()
                .flex_row()
                .justify_end()
                .gap(actions_gap)
                .mt(actions_mt)
                .self_stretch()
                .child(actions_el),
        );
    }

    // ── Overlay wrapper ───────────────────────────────────────────────────────
    backdrop(theme, backdrop_fill, spec, panel, on_request_close)
}

/// The scrim, and the panel sitting on it.
///
/// Clicks bubble to the nearest clickable ancestor, so a backdrop handler would
/// otherwise fire for every click *inside* the dialog — pressing "Save" would
/// dismiss it. The panel takes an inert handler of its own to stop that: it
/// becomes the nearest clickable, and the click ends there.
fn backdrop(
    _theme: &JetstreamThemeProvider,
    backdrop_fill: Color,
    spec: &DialogSpec,
    panel: JsEl,
    on_request_close: Option<crate::element::ActionHandler>,
) -> JsEl {
    let mut panel = panel;
    let mut root = ui_element::div()
        .bg(backdrop_fill)
        .overlay()
        .items_center()
        .justify_center();

    if let (true, Some(handler)) = (spec.effective_dismiss_on_backdrop(), &on_request_close) {
        let handler = std::sync::Arc::clone(handler);
        root = root.on_click(move |_event| handler());
        panel = panel.on_click(|_event| {});
    }

    crate::aria::with_aria_label(root.child(panel), spec.aria_label.as_deref())
        .aria_role(jetstream_ui::accesskit::Role::Dialog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::DialogWidth;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn overlay_color() -> ProbeColor {
        let c = resolve_color(&theme(), "color.background.overlay");
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    /// Header (title + description) and a body child all reach the rendered tree.
    #[test]
    fn dialog_renders_header_and_body() {
        let spec = DialogSpec::new()
            .with_title("Confirm action")
            .with_description("Are you sure?");
        let body = ui_element::label("Body content");
        let el = js_dialog(&spec, &theme(), vec![body], None);
        let tree = probe(&el, 800.0, 600.0);

        assert!(
            tree.has_text("Confirm action"),
            "title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Are you sure?"),
            "description missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Body content"),
            "body missing: {:?}",
            tree.texts()
        );
    }

    /// Footer/actions slot renders below the body.
    #[test]
    fn dialog_renders_actions() {
        let spec = DialogSpec::new().with_title("Confirm");
        let actions = ui_element::button("Confirm");
        let el = js_dialog(&spec, &theme(), vec![], Some(actions));
        let tree = probe(&el, 800.0, 600.0);
        assert!(
            tree.has_text("Confirm"),
            "action button missing: {:?}",
            tree.texts()
        );
    }

    /// Backdrop scrim resolves from the overlay token (not a hardcoded color).
    #[test]
    fn dialog_backdrop_uses_overlay_token() {
        let spec = DialogSpec::new().with_title("X");
        let el = js_dialog(&spec, &theme(), vec![], None);
        let tree = probe(&el, 800.0, 600.0);
        assert!(
            tree.has_background(overlay_color(), 0.02),
            "backdrop overlay token not found in tree"
        );
    }

    /// Close button renders an icon and a hit-testable id when enabled.
    #[test]
    fn dialog_close_button_present() {
        let spec = DialogSpec::new()
            .with_title("X")
            .with_show_close_button(true);
        let el = js_dialog(&spec, &theme(), vec![], None);
        let tree = probe(&el, 800.0, 600.0);
        assert!(
            tree.find_token("poodle-dialog-close").is_some(),
            "close button id missing"
        );
        assert!(
            tree.has_text("x"),
            "close icon glyph missing: {:?}",
            tree.texts()
        );
    }

    /// The non-contract 1px pre-actions divider was removed: the only Panel
    /// nodes are the surface itself and (when present) section wrappers — none
    /// is a bare 1px-tall rule. We assert by checking no Panel has a ~1px height
    /// while also carrying the subtle border color as its only content.
    #[test]
    fn dialog_has_no_pre_actions_divider() {
        let spec = DialogSpec::new().with_title("X");
        let actions = ui_element::button("OK");
        let el = js_dialog(&spec, &theme(), vec![], Some(actions));
        let tree = probe(&el, 800.0, 600.0);
        let subtle = resolve_color(&theme(), "color.border.subtle");
        let subtle_pc = ProbeColor {
            r: subtle.x,
            g: subtle.y,
            b: subtle.z,
            a: subtle.w,
        };
        let has_divider = tree.nodes.iter().any(|n| {
            n.kind == "Panel"
                && n.h > 0.0
                && n.h <= 2.0
                && n.bg.is_some_and(|b| b.approx(subtle_pc, 0.02))
        });
        assert!(
            !has_divider,
            "unexpected 1px subtle-border divider still present"
        );
    }

    /// Full width fills the overlay; Sm uses the finite rem preset.
    #[test]
    fn dialog_width_presets() {
        let sm = js_dialog(
            &DialogSpec::new()
                .with_title("X")
                .with_width(DialogWidth::Sm),
            &theme(),
            vec![],
            None,
        );
        let sm_tree = probe(&sm, 1200.0, 800.0);
        // Sm surface = 24rem = 384px (the Panel child of the overlay root).
        let surface = &sm_tree.nodes[1];
        assert!(
            (surface.w - 384.0).abs() < 1.0,
            "Sm width not 384px: {}",
            surface.w
        );

        let full = js_dialog(
            &DialogSpec::new()
                .with_title("X")
                .with_width(DialogWidth::Full),
            &theme(),
            vec![],
            None,
        );
        let full_tree = probe(&full, 1200.0, 800.0);
        let full_surface = &full_tree.nodes[1];
        assert!(
            full_surface.w > 384.0,
            "Full width did not grow: {}",
            full_surface.w
        );
    }
    /// The close button is the explicit dismissal route.
    #[test]
    fn the_close_button_requests_close() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = Dialog::from_spec(
            DialogSpec::new()
                .with_title("Delete?")
                .with_show_close_button(true),
            &theme(),
        )
        .on_request_close(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .into_js_el();

        crate::element::click_probe::click_text(&el, 800.0, 600.0, "x");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_request_close fired exactly once"
        );
    }

    #[test]
    fn the_backdrop_requests_close() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = Dialog::from_spec(DialogSpec::new().with_title("Delete?"), &theme())
            .on_request_close(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        // Top-left: outside the centred panel.
        crate::element::click_probe::click_at(&el, 800.0, 600.0, 4.0, 4.0);

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "a backdrop click did not dismiss"
        );
    }

    /// The defect this arrangement exists to prevent.
    ///
    /// Clicks bubble to the nearest clickable ancestor, so without an inert
    /// handler on the panel every click *inside* the dialog would reach the
    /// backdrop — pressing "Save" would dismiss the dialog it was saving.
    #[test]
    fn a_click_inside_the_panel_does_not_dismiss() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = Dialog::from_spec(DialogSpec::new().with_title("Delete everything?"), &theme())
            .on_request_close(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 800.0, 600.0, "Delete everything?");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            0,
            "clicking the dialog dismissed it"
        );
    }

    /// `dismissOnBackdrop` is a contract prop, so it has to actually guard the
    /// route rather than only the styling.
    #[test]
    fn dismiss_on_backdrop_false_keeps_the_dialog() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);
        let closes = Arc::clone(&hits);

        let el = Dialog::from_spec(
            DialogSpec::new()
                .with_title("Delete?")
                .with_show_close_button(true)
                .with_dismiss_on_backdrop(false),
            &theme(),
        )
        .on_request_close(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .into_js_el();

        crate::element::click_probe::click_at(&el, 800.0, 600.0, 4.0, 4.0);
        assert_eq!(
            hits.load(Ordering::SeqCst),
            0,
            "the backdrop dismissed despite the guard"
        );

        // The close button still works — the guard is about the backdrop only.
        crate::element::click_probe::click_text(&el, 800.0, 600.0, "x");
        assert_eq!(
            closes.load(Ordering::SeqCst),
            1,
            "the close button stopped working"
        );
    }
}
