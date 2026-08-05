//! Dialog — a modal surface over a scrim.
//!
//! Contract: `docs/contracts/components/dialog.md`
//! Ported from: `packages/jetstream/components/src/dialog.rs`. The event is
//! request-close, not open-change: the component cannot close itself; it
//! reports that a dismissal route was taken and the host decides.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeRole,
};
use poodle_specs::{DialogSpec, SemanticControlSizeRole};

use crate::presentation::{
    control_height_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};

pub fn dialog(
    spec: &DialogSpec,
    theme: &dyn ThemeProvider,
    children: Vec<Node>,
    actions: Option<Node>,
    on_request_close: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_font = rem_to_px(1.0_f32.max(size_font_rem(effective_size) + 0.1875));
    let body_font = rem_to_px(size_font_rem(effective_size));
    let space_x = rem_to_px(panel_space_x_rem(spec.density));
    let space_y = rem_to_px(panel_space_y_rem(spec.density));

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
    let has_header = spec.title.is_some() || spec.description.is_some() || spec.show_close_button;
    if has_header {
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
    if let Some(actions_el) = actions {
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
    {
        let s = &mut root.style;
        // Explicit Row (see switch.rs): the old backdrop relied on the default.
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.background = Some(backdrop_fill);
        s.overlay = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }

    if let (true, Some(handler)) = (spec.effective_dismiss_on_backdrop(), &on_request_close) {
        let handler = Arc::clone(handler);
        root.interaction.on_activate = Some(Arc::new(move || handler()));
        panel.interaction.on_activate = Some(Arc::new(|| {}));
    }

    let mut root = root.child(panel);
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::Dialog);
    root
}
