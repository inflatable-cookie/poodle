//! Projection from renderer-neutral node styles into GPUI refinements.

use super::*;

// ── Layout ──────────────────────────────────────────────────────────

pub(super) fn apply_layout<E: Styled>(mut el: E, node: &Node) -> E {
    let style = &node.style;
    let d = &style.descriptor;

    // Direction is ALWAYS emitted. Node containers default to Column while a
    // bare gpui `div()` defaults to `display: block` and `flex-direction:
    // row` — the "silent-Row" trap from the Jetstream ports. Silence here
    // would lay every Column node out as a row.
    el = match d.layout.direction {
        LayoutDirection::Row => el.flex().flex_row(),
        LayoutDirection::Column => el.flex().flex_col(),
    };
    record_probe_channel("layout.intent.direction");
    match d.layout.width {
        // `.flex_grow()` is a flex property, not a width — matching the
        // Jetstream `el.grow()` mapping, which is also a flex property.
        LayoutSizing::Grow => el = el.flex_grow(),
        LayoutSizing::Fixed(w) => el = el.w(px(w)),
        LayoutSizing::Fit => {}
        LayoutSizing::Constrained { min, max } => {
            if let Some(min) = min {
                el = el.min_w(px(min));
            }
            if let Some(max) = max {
                el = el.max_w(px(max));
            }
        }
    }
    match d.layout.height {
        LayoutSizing::Fixed(h) => el = el.h(px(h)),
        // Height-grow has no ported call site yet; when one appears it gets a
        // dedicated mapping, not a guess (same punt as the Jetstream backend).
        LayoutSizing::Grow | LayoutSizing::Fit => {}
        LayoutSizing::Constrained { min, max } => {
            if let Some(min) = min {
                el = el.min_h(px(min));
            }
            if let Some(max) = max {
                el = el.max_h(px(max));
            }
        }
    }
    if style.fill_width {
        el = el.w_full();
    }
    if style.fill_height {
        el = el.h_full();
    }
    if style.flex_none {
        el = el.flex_none();
    }
    if style.self_stretch {
        // No fluent self-stretch in gpui 0.2.2; set the refinement field.
        el.style().align_self = Some(gpui::AlignSelf::Stretch);
    }
    if let Some(grow) = style.flex_grow {
        // Raw factor (fractional splits); `.flex_grow()` is the 1.0 case.
        el.style().flex_grow = Some(grow);
        record_probe_channel("layout.geometry.flex-grow");
    }
    if style.flex_fill {
        // Jetstream maps flex_fill to its grow() — grow + shrink, no stretch.
        // gpui's `.flex_grow()` leaves shrink at its 1.0 default and does not
        // touch align-self, which is exactly that.
        el = el.flex_grow();
    }
    if style.flex_shrink_zero {
        el = el.flex_shrink_0();
    }
    if let Some(pct) = style.flex_basis_pct {
        el = el.flex_basis(relative(pct));
    } else if let Some(basis) = style.flex_basis {
        el = el.flex_basis(px(basis));
    }
    if let Some(pct) = style.width_pct {
        el = el.w(relative(pct));
    }
    if style.flex_wrap {
        el = el.flex_wrap();
    }
    let pad = d.layout.spacing.padding;
    if pad.left != 0.0 {
        el = el.pl(px(pad.left));
    }
    if pad.right != 0.0 {
        el = el.pr(px(pad.right));
    }
    if pad.top != 0.0 {
        el = el.pt(px(pad.top));
    }
    if pad.bottom != 0.0 {
        el = el.pb(px(pad.bottom));
    }
    if d.layout.spacing.gap != 0.0 {
        el = el.gap(px(d.layout.spacing.gap));
        record_probe_channel("layout.intent.gap");
    }
    let margin = d.layout.spacing.margin;
    if margin.left != 0.0 {
        el = el.ml(px(margin.left));
    }
    if margin.right != 0.0 {
        el = el.mr(px(margin.right));
    }
    if margin.top != 0.0 {
        el = el.mt(px(margin.top));
    }
    if margin.bottom != 0.0 {
        el = el.mb(px(margin.bottom));
    }
    match d.layout.alignment.cross {
        CrossAxisAlignment::Center => el = el.items_center(),
        CrossAxisAlignment::Start => el = el.items_start(),
        CrossAxisAlignment::End => el = el.items_end(),
        // Stretch is taffy's default: silence is the faithful emission.
        CrossAxisAlignment::Stretch => {}
    }
    match d.layout.alignment.main {
        MainAxisAlignment::Center => el = el.justify_center(),
        MainAxisAlignment::SpaceBetween => el = el.justify_between(),
        MainAxisAlignment::End => el = el.justify_end(),
        MainAxisAlignment::Start => {}
    }
    // Per-axis overflow — gpui supports each axis independently, so the
    // Jetstream backend's combo chain becomes two direct emissions. Scroll
    // state lives on stateful elements (see `needs_state`); setting the
    // style fields directly is what the scroll helpers do internally.
    match d.layout.overflow_x {
        LayoutOverflow::Hidden => el = el.overflow_x_hidden(),
        LayoutOverflow::Scroll => el.style().overflow.x = Some(gpui::Overflow::Scroll),
        LayoutOverflow::Visible => {}
    }
    match d.layout.overflow_y {
        LayoutOverflow::Hidden => el = el.overflow_y_hidden(),
        LayoutOverflow::Scroll => el.style().overflow.y = Some(gpui::Overflow::Scroll),
        LayoutOverflow::Visible => {}
    }
    if let Some(v) = style.min_width {
        el = el.min_w(px(v));
        record_probe_channel("layout.geometry.min-width");
    }
    if let Some(v) = style.max_width {
        el = el.max_w(px(v));
        record_probe_channel("layout.geometry.max-width");
    }
    if let Some(v) = style.min_height {
        el = el.min_h(px(v));
    }
    if let Some(v) = style.max_height {
        el = el.max_h(px(v));
    }
    el
}

// ── Position ────────────────────────────────────────────────────────

pub(super) fn apply_position<E: Styled>(mut el: E, node: &Node) -> E {
    match node.position {
        NodePosition::InFlow => {}
        // gpui's default position IS relative; emit anyway for parity with
        // the walk (and to pin the semantic against default drift).
        NodePosition::Relative => el = el.relative(),
        NodePosition::Absolute {
            top,
            left,
            right,
            bottom,
        } => {
            el = el.absolute();
            if let Some(v) = top {
                el = el.top(px(v));
            }
            if let Some(v) = left {
                el = el.left(px(v));
            }
            if let Some(v) = right {
                el = el.right(px(v));
            }
            if let Some(v) = bottom {
                el = el.bottom(px(v));
            }
        }
    }
    if matches!(node.position, NodePosition::Relative) {
        record_probe_channel("layout.position.relative");
    }
    el
}

// ── Paint ───────────────────────────────────────────────────────────

pub(super) fn apply_paint<E: Styled>(mut el: E, node: &Node) -> E {
    let style = &node.style;
    let d = &style.descriptor;

    // Background and gradient are independent channels (a toast paints a
    // tint fill AND a fade gradient over it), so both apply when set.
    if let Some(bg) = d.background {
        el = el.bg(color(bg));
        record_probe_channel("surface.channels.background");
    }
    if let Some((angle, stops)) = &style.gradient {
        // gpui 0.2.2 `linear_gradient` takes exactly two stops. Two-stop
        // gradients (the only kind with a ported call site) map exactly;
        // longer stop lists keep their endpoints and drop the middle —
        // flagged as an approximation for the first component that needs it.
        if let (Some(first), Some(last)) = (stops.first(), stops.last()) {
            el = el.bg(linear_gradient(
                *angle,
                linear_color_stop(color(first.0), first.1),
                linear_color_stop(color(last.0), last.1),
            ));
        }
    }

    // gpui 0.2.2's fluent border widths are fixed steps (border_1, border_2,
    // …); arbitrary widths go straight to the refinement fields. Per-side
    // widths compose with the uniform width exactly as the Jetstream walk's
    // `border_widths[i] = w` did.
    if d.border.width != 0.0 {
        let style_ref = el.style();
        let w = Some(px(d.border.width).into());
        style_ref.border_widths.top = w;
        style_ref.border_widths.right = w;
        style_ref.border_widths.bottom = w;
        style_ref.border_widths.left = w;
    }
    if let Some(w) = style.border_top_width {
        el.style().border_widths.top = Some(px(w).into());
    }
    if let Some(w) = style.border_right_width {
        el.style().border_widths.right = Some(px(w).into());
    }
    if let Some(w) = style.border_bottom_width {
        el.style().border_widths.bottom = Some(px(w).into());
    }
    if let Some(w) = style.border_left_width {
        el.style().border_widths.left = Some(px(w).into());
    }

    // Colour accompanies any border — a border with no colour emission is
    // invisible. Per-side colour overrides win over the uniform colour.
    //
    // APPROXIMATION: gpui 0.2.2 has a single `border_color` — no per-side
    // colours. When exactly one side overrides and the uniform colour is
    // unset (the ring-spinner arc, the remediation left accent, the active
    // tab underline), the override becomes the element's one border colour,
    // which is pixel-exact when only that side has a width. Multiple
    // simultaneous side colours have no faithful mapping; last write wins.
    let transparent = ColorValue(0.0, 0.0, 0.0, 0.0);
    let uniform_color_set = d.border.color != transparent;
    let has_border = d.border.width != 0.0
        || style.border_bottom_width.is_some()
        || style.border_right_width.is_some()
        || style.border_top_width.is_some()
        || style.border_left_width.is_some();
    if has_border && uniform_color_set {
        el = el.border_color(color(d.border.color));
        record_probe_channel("surface.channels.border");
    }
    if let Some(top) = style.border_color_top {
        el = el.border_color(color(top));
    }
    if let Some(left) = style.border_color_left {
        el = el.border_color(color(left));
        record_probe_channel("surface.extended.side-border");
    }
    if let Some(bottom) = style.border_color_bottom {
        el = el.border_color(color(bottom));
    }

    // UNIMPLEMENTED: `grayscale` — gpui 0.2.2 has no filter channel. A
    // not-live card's washed-out treatment renders in full colour.
    // UNIMPLEMENTED: `border_dashed` IS supported (BorderStyle) — mapped:
    if style.border_dashed {
        el = el.border_dashed();
    }

    let radii = d.corner_radii;
    if radii.top_left != 0.0 {
        el.style().corner_radii.top_left = Some(px(radii.top_left).into());
    }
    if radii.top_right != 0.0 {
        el.style().corner_radii.top_right = Some(px(radii.top_right).into());
    }
    if radii.bottom_right != 0.0 {
        el.style().corner_radii.bottom_right = Some(px(radii.bottom_right).into());
    }
    if radii.bottom_left != 0.0 {
        el.style().corner_radii.bottom_left = Some(px(radii.bottom_left).into());
    }

    if !style.shadow_layers.is_empty() {
        // APPROXIMATION: crates.io gpui 0.2.2 `BoxShadow` has no inset flag,
        // so inset (highlight) layers are dropped; drop layers map exactly.
        // g15.045 projected them through the fork's added `inset` field;
        // g16.005 restored the published crate identity, and the flag went
        // with it. Reinstate the projection only if upstream publishes it.
        let shadows = style
            .shadow_layers
            .iter()
            .filter(|l| !l.inset)
            .map(|l| gpui::BoxShadow {
                color: color(l.color),
                offset: point(px(l.offset_x), px(l.offset_y)),
                blur_radius: px(l.blur),
                spread_radius: px(l.spread),
            })
            .collect::<Vec<_>>();
        if !shadows.is_empty() {
            el = el.shadow(shadows);
        }
    } else if let Some(shadow) = &d.shadow {
        el = el.shadow(vec![gpui::BoxShadow {
            color: color(shadow.color),
            offset: point(px(shadow.offset_x), px(shadow.offset_y)),
            blur_radius: px(shadow.blur),
            spread_radius: px(0.0),
        }]);
        record_probe_channel("surface.extended.shadow");
    }

    if d.opacity != 1.0 {
        el = el.opacity(d.opacity);
        record_probe_channel("surface.channels.opacity");
    }
    if !d.visible {
        el = el.invisible();
    }
    el
}

// ── Text ────────────────────────────────────────────────────────────

pub(super) fn apply_text<E: Styled>(mut el: E, node: &Node) -> E {
    let style = &node.style;
    let d = &style.descriptor;

    if let Some(c) = d.text_color {
        el = el.text_color(color(c));
        record_probe_channel("surface.channels.text");
    }
    if let Some(size) = style.text_size {
        el = el.text_size(px(size));
        record_probe_channel("content.typography.size");
    }
    if let Some(weight) = style.text_weight {
        el = el.font_weight(gpui::FontWeight(weight as f32));
        record_probe_channel("content.typography.weight");
    }
    if style.text_italic {
        el = el.italic();
    }
    if let Some(lh) = style.line_height {
        // Vocabulary: a multiple of the font size → relative length.
        el = el.line_height(relative(lh));
        record_probe_channel("content.typography.line-height");
    }
    if style.text_wrap {
        el = el.whitespace_normal();
    }
    if let Some(family) = style.font_family {
        // Sans is the app's default (the preview sets Inter at the root), so
        // silence is faithful; Mono names gpui's system monospace stack, the
        // same string the old tier used.
        if let FontFamily::Mono = family {
            el = el.font_family("monospace");
        }
    }
    if style.text_ellipsis {
        el = el.text_ellipsis();
    }
    if style.text_underline {
        el = el.underline();
    }
    if let Some(underline_color) = style.text_underline_color {
        el = el.text_decoration_color(color(underline_color));
    }
    if style.no_wrap {
        el = el.whitespace_nowrap();
    }
    // UNIMPLEMENTED: `letter_spacing_em` — gpui 0.2.2 text styles have no
    // letter-spacing channel.
    // Button labels are centered by the flex main-axis channel. The old GPUI
    // tier did not also apply text alignment to the label wrapper.
    if !matches!(node.kind, NodeKind::Button { .. }) {
        match style.text_align {
            Some(TextAlign::Center) => el = el.text_center(),
            Some(TextAlign::Right) => el = el.text_right(),
            Some(TextAlign::Left) | None => {}
        }
    }
    el
}

// ── Interaction ─────────────────────────────────────────────────────

pub(super) fn apply_cursor<E: Styled>(mut el: E, node: &Node) -> E {
    el = match node.style.descriptor.cursor {
        CursorHint::Pointer => el.cursor_pointer(),
        CursorHint::Text => el.cursor(CursorStyle::IBeam),
        CursorHint::NotAllowed => el.cursor(CursorStyle::OperationNotAllowed),
        CursorHint::Grab => el.cursor(CursorStyle::OpenHand),
        CursorHint::Grabbing => el.cursor(CursorStyle::ClosedHand),
        CursorHint::ColResize => el.cursor(CursorStyle::ResizeColumn),
        CursorHint::RowResize => el.cursor(CursorStyle::ResizeRow),
        CursorHint::Default => el,
    };
    if node.style.descriptor.cursor == CursorHint::Pointer {
        record_probe_channel("surface.extended.cursor");
    }
    el
}

pub(super) fn apply_patch(mut s: StyleRefinement, patch: StylePatch) -> StyleRefinement {
    if let Some(bg) = patch.background {
        s = s.bg(color(bg));
    }
    if let Some(b) = patch.border_color {
        s = s.border_color(color(b));
    }
    if let Some(t) = patch.text_color {
        s = s.text_color(color(t));
    }
    if let Some(o) = patch.opacity {
        s = s.opacity(o);
    }
    s
}

pub(super) fn apply_state_patches<E: InteractiveElement>(mut el: E, node: &Node, id: &str) -> E {
    // Disabled nodes keep their baked style and get no patches — the
    // vocabulary's contract is "renders in the disabled state the style
    // already describes".
    if node.interaction.disabled {
        return el;
    }
    if let Some(patch) = &node.style.hover {
        let patch = *patch;
        // gpui refines hover *after* focus (`div.rs`: focus_style at 2490,
        // hover_style at 2506), so a hover border silently overwrites a focus
        // ring — a focused field lost its ring the moment you moved the mouse
        // over it. Fold the focus patch back on top inside the hover closure
        // while this node actually holds focus, so the last word is focus's.
        let focus_patch = node.style.focus.filter(|_| is_focused(id));
        el = el.hover(move |s| {
            let s = apply_patch(s, patch);
            match focus_patch {
                Some(focus) => apply_patch(s, focus),
                None => s,
            }
        });
        record_probe_channel("surface.state-patches.hover");
    }
    el
}
