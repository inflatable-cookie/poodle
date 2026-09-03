//! Dialog — a modal surface over a scrim.
//!
//! Contract: `docs/contracts/components/dialog.md`
//! Ported from: `packages/jetstream/components/src/dialog.rs`. The event is
//! request-close, not open-change: the component cannot close itself; it
//! reports that a dismissal route was taken and the host decides.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodePosition, NodeRole,
};
use poodle_specs::{DialogSpec, SemanticControlSizeRole};

use crate::context::RenderContext;
use crate::presentation::{
    control_height_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};

pub fn dialog(
    spec: &DialogSpec,
    ctx: &RenderContext<'_>,
    children: Vec<Node>,
    actions: Option<Node>,
    on_request_close: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    dialog_with_slots(spec, ctx, children, actions, None, None, on_request_close)
}

/// Render a dialog with optional custom header and footer slots.
///
/// The base `dialog` entry point keeps the contract's default anatomy. This
/// variant is used by the GPUI compatibility bridge for the legacy
/// `with_header`/`with_footer` slots while preserving the same surface and
/// dismissal wiring.
pub fn dialog_with_slots(
    spec: &DialogSpec,
    ctx: &RenderContext<'_>,
    children: Vec<Node>,
    actions: Option<Node>,
    header_override: Option<Node>,
    footer_override: Option<Node>,
    on_request_close: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let theme = ctx.theme();
    let title_font = rem_to_px(1.0_f32.max(size_font_rem(effective_size) + 0.1875));
    let body_font = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(density));
    let space_y = rem_to_px(panel_space_y_rem(density));

    let fill = theme.resolve_color(spec.surface_fill_token());
    let backdrop_fill = theme.resolve_color(spec.backdrop_fill_token());
    let border = theme.resolve_color("color.border.default");
    let radius = theme.resolve_radius("radius.surface");
    let title_color = theme.resolve_color("color.text.primary");
    let desc_color = theme.resolve_color("color.text.secondary");
    let muted_color = theme.resolve_color("color.text.secondary");

    // Contract §8 section spacing.
    let header_gap = rem_to_px(0.375);
    let header_mb = theme.resolve_space("space.stack.md");
    let actions_gap = theme.resolve_space("space.inline.sm");
    let actions_mt = theme.resolve_space("space.stack.lg");
    let chrome_size = resolve_semantic_size(effective_size, SemanticControlSizeRole::Chrome);
    let close_dim = rem_to_px(control_height_rem(chrome_size));

    let width_rem = spec.surface_width_rem();

    // ── Panel ──
    let mut panel = Node::container();
    panel.id = Some("poodle-dialog-surface".to_string());
    {
        let s = &mut panel.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.max_height = Some(rem_to_px(42.0));
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        // Token-accurate elevation.dialog (modal tier).
        s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_DIALOG);
        if width_rem.is_finite() {
            s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(width_rem));
        } else {
            // Full — fill the overlay, constrained by the centering parent.
            s.descriptor.layout.width = LayoutSizing::Grow;
        }
    }

    // ── Bare mode: children fill the panel directly ──
    if spec.bare {
        for child in children {
            panel = panel.child(child);
        }
        return backdrop(backdrop_fill, spec, panel, on_request_close);
    }

    {
        let pad = &mut panel.style.descriptor.layout.spacing.padding;
        pad.left = space_x;
        pad.right = space_x;
        pad.top = space_y;
        pad.bottom = space_y;
    }

    // ── Header: title/description left, optional close right ──
    let has_header = header_override.is_some()
        || spec.title.is_some()
        || spec.description.is_some()
        || spec.show_close_button;
    if has_header {
        let mut header_col = if let Some(header) = header_override {
            header
        } else {
            let mut header_col = Node::container();
            {
                let s = &mut header_col.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = header_gap;
                s.descriptor.layout.width = LayoutSizing::Grow;
            }
            if let Some(ref title) = spec.title {
                let mut t = Node::text(title);
                t.style.descriptor.text_color = Some(title_color);
                t.style.text_size = Some(title_font);
                t.style.text_weight = Some(600);
                header_col = header_col.child(t);
            }
            if let Some(ref description) = spec.description {
                let mut d = Node::text(description);
                d.style.descriptor.text_color = Some(desc_color);
                d.style.text_size = Some(body_font);
                header_col = header_col.child(d);
            }
            header_col
        };
        header_col.style.descriptor.layout.width = LayoutSizing::Grow;

        let mut header_row = Node::container();
        {
            let s = &mut header_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
            s.descriptor.layout.spacing.gap = actions_gap;
            s.descriptor.layout.spacing.margin.bottom = header_mb;
        }
        let mut header_row = header_row.child(header_col);

        if spec.show_close_button {
            let icon_size = rem_to_px(size_font_rem(chrome_size));
            let mut close = Node::button("");
            close.a11y.label = Some(spec.close_label.clone());
            close.id = Some("poodle-dialog-close".to_string());
            {
                let s = &mut close.style;
                s.descriptor.layout.width = LayoutSizing::Fixed(close_dim);
                s.descriptor.layout.height = LayoutSizing::Fixed(close_dim);
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                let r = theme.resolve_radius("radius.control");
                s.descriptor.corner_radii.top_left = r;
                s.descriptor.corner_radii.top_right = r;
                s.descriptor.corner_radii.bottom_right = r;
                s.descriptor.corner_radii.bottom_left = r;
                s.descriptor.cursor = CursorHint::Pointer;
                s.focus = Some(poodle_node::StylePatch {
                    background: None,
                    border_color: Some(theme.resolve_color("color.accent.focusRing")),
                    text_color: None,
                    opacity: None,
                });
            }
            close.interaction.focusable = true;
            let mut x = Node::icon("x", icon_size);
            x.style.descriptor.text_color = Some(muted_color);
            let mut close = close.child(x);

            // The explicit dismissal route, whatever dismiss_on_backdrop says.
            if let Some(handler) = &on_request_close {
                let handler = Arc::clone(handler);
                close.interaction.on_activate = Some(Arc::new(move || handler()));
            }
            header_row = header_row.child(close);
        }
        panel = panel.child(header_row);
    }

    // ── Body ──
    if !children.is_empty() {
        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.min_width = Some(0.0);
            s.self_stretch = true;
        }
        let mut body = body;
        for child in children {
            body = body.child(child);
        }
        panel = panel.child(body);
    }

    // ── Actions: end-justified row ──
    if let Some(footer_el) = footer_override {
        panel = panel.child(footer_el);
    } else if let Some(actions_el) = actions {
        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.main = MainAxisAlignment::End;
            s.descriptor.layout.spacing.gap = actions_gap;
            s.descriptor.layout.spacing.margin.top = actions_mt;
            s.self_stretch = true;
        }
        panel = panel.child(row.child(actions_el));
    }

    backdrop(backdrop_fill, spec, panel, on_request_close)
}

/// The scrim and the panel on it. The panel takes an inert handler when the
/// backdrop dismisses, so inside-clicks end there instead of bubbling to the
/// scrim — pressing "Save" must not dismiss the dialog.
fn backdrop(
    backdrop_fill: poodle_node::ColorValue,
    spec: &DialogSpec,
    mut panel: Node,
    on_request_close: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let mut root = Node::container();
    root.id = Some("poodle-dialog-backdrop".to_string());
    root.position = NodePosition::Absolute {
        top: Some(0.0),
        left: Some(0.0),
        right: Some(0.0),
        bottom: Some(0.0),
    };
    {
        let s = &mut root.style;
        // Explicit Row (see switch.rs): the old backdrop relied on the default.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.background = Some(backdrop_fill);
        s.overlay = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }

    // Dismiss layer registration for Escape and outside dismissal stack.
    // The panel surface is the containment boundary (contract §8 / Dialog.svelte);
    // the full-screen backdrop is not part of the layer so outside clicks can fire.
    let layer_id = "poodle-dialog-layer".to_string();
    if on_request_close.is_some() {
        panel.interaction.dismiss_layer = Some(layer_id);
        panel.interaction.on_activate = Some(Arc::new(|| {}));
    }

    if let (true, Some(handler)) = (spec.effective_dismiss_on_backdrop(), &on_request_close) {
        let handler = Arc::clone(handler);
        root.interaction.on_activate = Some(Arc::new(move || handler()));
    }

    if let Some(handler) = on_request_close {
        let dismiss_on_escape = spec.dismiss_on_escape;
        if dismiss_on_escape {
            panel.interaction.on_dismiss = Some(Arc::new(move |reason| match reason {
                poodle_node::DismissReason::Escape if dismiss_on_escape => {
                    handler();
                }
                _ => {}
            }));
        }
    }

    let mut root = root.child(panel);
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::Dialog);
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::DialogWidth;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestTheme;
    impl poodle_adapter::ThemeProvider for TestTheme {
        fn resolve_color(&self, _: &str) -> poodle_node::ColorValue {
            poodle_node::ColorValue(0.1, 0.2, 0.3, 1.0)
        }
        fn resolve_space(&self, _: &str) -> f32 {
            8.0
        }
        fn resolve_border_width(&self, _: &str) -> f32 {
            1.0
        }
        fn resolve_radius(&self, _: &str) -> f32 {
            6.0
        }
        fn resolve_opacity(&self, _: &str) -> f32 {
            1.0
        }
    }

    #[test]
    fn renders_dialog_backdrop_and_surface() {
        let theme = TestTheme;
        let ctx = RenderContext::new(&theme);
        let spec = DialogSpec::new()
            .with_title("Test Title")
            .with_description("Test Description")
            .with_show_close_button(true);

        let node = dialog(&spec, &ctx, vec![Node::text("Body content")], None, None);

        assert_eq!(node.id.as_deref(), Some("poodle-dialog-backdrop"));
        assert!(node.style.overlay);
        assert_eq!(node.a11y.role, Some(NodeRole::Dialog));
        assert!(node.interaction.dismiss_layer.is_none());

        assert_eq!(node.children.len(), 1);
        let panel = &node.children[0];
        assert_eq!(panel.id.as_deref(), Some("poodle-dialog-surface"));
        assert!(panel.interaction.dismiss_layer.is_none());
        assert!(panel.interaction.on_activate.is_none());

        let with_close = dialog(
            &spec,
            &ctx,
            vec![Node::text("Body content")],
            None,
            Some(Arc::new(|| {})),
        );
        let with_close_panel = &with_close.children[0];
        assert_eq!(
            with_close_panel.interaction.dismiss_layer.as_deref(),
            Some("poodle-dialog-layer")
        );
        assert!(with_close_panel.interaction.on_activate.is_some());
    }

    #[test]
    fn dismissal_wiring_for_backdrop_and_escape() {
        let theme = TestTheme;
        let ctx = RenderContext::new(&theme);

        let call_count = Arc::new(AtomicUsize::new(0));
        let count_clone = Arc::clone(&call_count);
        let on_close = Arc::new(move || {
            count_clone.fetch_add(1, Ordering::SeqCst);
        });

        let spec = DialogSpec::new()
            .with_dismiss_on_backdrop(true)
            .with_dismiss_on_escape(true);

        let node = dialog(&spec, &ctx, vec![], None, Some(on_close));

        // Backdrop activation on root
        assert!(node.interaction.on_activate.is_some());
        if let Some(act) = &node.interaction.on_activate {
            act();
            assert_eq!(call_count.load(Ordering::SeqCst), 1);
        }

        // On dismiss (Escape) on panel
        let panel = &node.children[0];
        assert!(panel.interaction.on_dismiss.is_some());
        if let Some(dismiss) = &panel.interaction.on_dismiss {
            dismiss(poodle_node::DismissReason::Escape);
            assert_eq!(call_count.load(Ordering::SeqCst), 2);
        }

        // Escape disabled omits dismissal
        let escape_disabled_spec = DialogSpec::new()
            .with_dismiss_on_escape(false);
        let escape_disabled_node = dialog(&escape_disabled_spec, &ctx, vec![], None, Some(Arc::new(|| {})));
        let escape_disabled_panel = &escape_disabled_node.children[0];
        assert!(escape_disabled_panel.interaction.on_dismiss.is_none());
    }

    #[test]
    fn dismiss_on_backdrop_disabled_omits_root_activation() {
        let theme = TestTheme;
        let ctx = RenderContext::new(&theme);

        let on_close = Arc::new(|| {});
        let spec = DialogSpec::new().with_dismiss_on_backdrop(false);

        let node = dialog(&spec, &ctx, vec![], None, Some(on_close));
        assert!(node.interaction.on_activate.is_none());
    }

    #[test]
    fn bare_dialog_mode_mounts_children_directly_into_panel() {
        let theme = TestTheme;
        let ctx = RenderContext::new(&theme);

        let spec = DialogSpec::new().with_bare(true);
        let node = dialog(
            &spec,
            &ctx,
            vec![Node::text("Custom Modal View")],
            None,
            None,
        );

        let panel = &node.children[0];
        assert_eq!(panel.id.as_deref(), Some("poodle-dialog-surface"));
        assert_eq!(panel.children.len(), 1);
    }

    #[test]
    fn width_presets_apply_to_panel_sizing() {
        let theme = TestTheme;
        let ctx = RenderContext::new(&theme);

        let sm_node = dialog(
            &DialogSpec::new().with_width(DialogWidth::Sm),
            &ctx,
            vec![],
            None,
            None,
        );
        assert_eq!(
            sm_node.children[0].style.descriptor.layout.width,
            LayoutSizing::Fixed(rem_to_px(24.0))
        );

        let full_node = dialog(
            &DialogSpec::new().with_width(DialogWidth::Full),
            &ctx,
            vec![],
            None,
            None,
        );
        assert_eq!(
            full_node.children[0].style.descriptor.layout.width,
            LayoutSizing::Grow
        );
    }
}
