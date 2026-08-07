//! GPUI backend for Poodle's render vocabulary: interpret a [`poodle_node::Node`]
//! tree as GPUI elements.
//!
//! This crate is the GPUI half of the inversion `g12.019` completes. Poodle's
//! components (`poodle-render`) emit `Spec + Theme → Node` trees and know
//! nothing of GPUI; this adapter translates that vocabulary into GPUI 0.2.2's
//! fluent element API. The transcription source is the Jetstream backend
//! (`jetstream-poodle/src/lib.rs`), whose channel walk this mirrors channel by
//! channel; where GPUI has no equivalent channel the gap is documented inline
//! and in the crate's channel table (see `docs/roadmaps/g12/019-gpui-node-backend.md`).
//!
//! What this backend owns (and the vocabulary correctly does not): text
//! measurement and shaping (GPUI's text system), hit-testing and event
//! dispatch, icon rasterisation (SVG via the app's asset source), animation
//! clocks.
//!
//! Color: the vocabulary is sRGB and GPUI's `Hsla`/`Rgba` are sRGB, so the
//! conversion at this edge is a raw passthrough — the same path the old GPUI
//! tier used (`poodle-gpui-components::theme_ext::resolve_color`,
//! `poodle-gpui::style_map::GpuiColor::from`). No transfer function applies;
//! alpha is coverage and passes through. All mixing happened render-side
//! (`poodle-render::color`); nodes carry final values.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use gpui::{
    div, img, linear_color_stop, linear_gradient, point, px, relative, svg, AnyElement, App,
    AppContext,
    ClickEvent, CursorStyle, Div, ElementId, Hsla, InteractiveElement, IntoElement, KeyDownEvent,
    MouseButton, ParentElement, SharedString, Stateful, StatefulInteractiveElement,
    StyleRefinement, Styled, StyledImage, Window,
};
use poodle_node::{
    AnimEasing, AnimLoop, AnimProperty, ColorValue, CrossAxisAlignment, CursorHint, DropEdge,
    FontFamily, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
    NodeAnimation, NodeDragEvent, NodeDragPhase, NodeDropEvent, NodeKey, NodeKind, NodeModifiers,
    NodePoint, NodePosition, NodeRole, StylePatch, TextAlign,
};

/// sRGB passthrough — the exact conversion the old GPUI tier performed.
/// gpui's `Rgba` channels are sRGB; the round trip through `Hsla` is what
/// `theme_ext::resolve_color` did for every token the old tier resolved.
pub fn color(c: ColorValue) -> Hsla {
    gpui::Rgba {
        r: c.0,
        g: c.1,
        b: c.2,
        a: c.3,
    }
    .into()
}

/// Deterministic per-tree ids for nodes that need element state (interaction)
/// but declare none. Tree order is stable across frames for a stable tree, so
/// a counter keeps the same node on the same id between rebuilds.
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn element_id(node: &Node) -> ElementId {
    if let Some(id) = &node.id {
        return ElementId::Name(SharedString::from(id.clone()));
    }
    if let Some(anim) = &node.style.animation {
        // Vocabulary: an animation's key becomes the id when none is set —
        // nodes sharing a key share a clock.
        return ElementId::Name(SharedString::from(anim.key.clone()));
    }
    ElementId::Name(SharedString::from(format!(
        "poodle-node-{}",
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    )))
}

/// Interpret one node (and its subtree) as a GPUI element.
pub fn to_gpui(node: &Node) -> AnyElement {
    match &node.kind {
        NodeKind::Container => build_box(node, div()),
        NodeKind::Text { content } => build_box(node, div().child(content.clone())),
        // GPUI has no native button element; the old tier's buttons are styled
        // divs too, so the label-child div is the faithful mapping. Same for
        // Input: a real GPUI text field is an `Editor` entity, which a pure
        // `&Node -> element` function cannot create. A childless input renders
        // its intrinsic value/placeholder; composite inputs supply styled
        // children (affixes, icons, count) and the backend avoids duplicating
        // the value. Caret/selection/IME remain a backend gap.
        NodeKind::Button { label } => {
            let el = if label.is_empty() {
                div()
            } else if matches!(
                node.a11y.role,
                Some(poodle_node::NodeRole::Button | poodle_node::NodeRole::RadioButton)
            ) {
                // The old GPUI button and radio segment place their labels
                // directly in the styled control. The generic wrapper changes
                // intrinsic text measurement and centering.
                div().child(label.clone())
            } else {
                div().child(
                    div()
                        .whitespace_nowrap()
                        .min_w(px(0.0))
                        .child(label.clone()),
                )
            };
            build_box(node, el)
        }
        NodeKind::Input { value, placeholder } => {
            let el = if node.children.is_empty() {
                let text = if value.is_empty() { placeholder } else { value };
                div().child(text.clone())
            } else {
                div()
            };
            build_box(node, el)
        }
        NodeKind::Progress { fraction } => {
            // The node styles the track; the backend fills `fraction` of it.
            // Fill colour comes from `text_color` when the component set one —
            // the vocabulary carries no dedicated fill channel (Jetstream's
            // progress widget supplies its own). UNPROVEN: no gated specimen
            // exercises this yet (progress is skipped as non-deterministic).
            let fill_color = node
                .style
                .descriptor
                .text_color
                .map(color)
                .unwrap_or_else(|| gpui::white());
            let fill = div()
                .h_full()
                .w(relative(fraction.clamp(0.0, 1.0)))
                .rounded_full()
                .bg(fill_color);
            build_box(node, div().child(fill))
        }
        NodeKind::Icon { name, size } => {
            // Same path convention as the old tier's Icon: the app owns the
            // asset source; the name is the contract. svg() renders tinted by
            // `text_color`, which the style walk supplies.
            let el = svg()
                .path(SharedString::from(format!("assets/icons/{name}.svg")))
                .size(px(*size))
                .flex_shrink_0();
            build_svg_leaf(node, el)
        }
        NodeKind::Image { source } => {
            // Vocabulary: fits by covering the box (object-fit: cover).
            let el = img(source.clone()).object_fit(gpui::ObjectFit::Cover);
            build_leaf(node, el)
        }
    }
}

/// Leaves (svg, img) implement `Styled` but not `InteractiveElement`/
/// `ParentElement` in gpui 0.2.2. A leaf node that declares interaction,
/// state patches, or children is wrapped in a div that carries them — the
/// leaf keeps its own sizing and colour.
fn build_leaf<E>(node: &Node, el: E) -> AnyElement
where
    E: Styled + IntoElement + 'static,
{
    let el = apply_layout(el, node);
    let el = apply_paint(el, node);
    let el = apply_text(el, node);
    let el = apply_cursor(el, node);
    let needs_wrapper = node.style.hover.is_some()
        || node.style.active.is_some()
        || node.interaction.focusable
        || node.interaction.on_activate.is_some()
        || node.interaction.on_text_change.is_some()
        || node.interaction.on_drag.is_some()
        || !node.children.is_empty();
    if !needs_wrapper {
        return maybe_animated(el, node);
    }
    let wrapped = div().child(el);
    build_box(node, wrapped)
}

/// SVG leaves can carry the vocabulary's rotation channel directly. GPUI
/// exposes transforms on SVG elements, but not on generic Styled elements.
fn build_svg_leaf(node: &Node, el: gpui::Svg) -> AnyElement {
    use gpui::{AnimationExt, Transformation};

    let el = apply_layout(el, node);
    let el = apply_paint(el, node);
    let el = apply_text(el, node);
    let el = apply_cursor(el, node);
    let needs_wrapper = node.style.hover.is_some()
        || node.style.active.is_some()
        || node.interaction.focusable
        || node.interaction.on_activate.is_some()
        || node.interaction.on_text_change.is_some()
        || node.interaction.on_drag.is_some()
        || !node.children.is_empty();
    if needs_wrapper {
        return build_box(node, div().child(el));
    }

    let Some(anim) = &node.style.animation else {
        return el.into_any_element();
    };
    if sample_property(anim, AnimProperty::Rotate, 0.0).is_none() {
        return maybe_animated(el, node);
    }

    let anim = anim.clone();
    let id = element_id(node);
    el.with_animation(id, gpui_animation(&anim), move |svg, t| {
        let radians = sample_property(&anim, AnimProperty::Rotate, t).unwrap_or(0.0);
        svg.with_transformation(Transformation::rotate(gpui::radians(radians)))
    })
    .into_any_element()
}

/// Container-shaped nodes: the full channel walk. Interaction that needs
/// element state (click, drag, focus) forces a stateful div — gpui 0.2.2
/// gates its listener model behind `Stateful`.
fn build_box(node: &Node, base: Div) -> AnyElement {
    if needs_state(node) {
        let el = base.id(element_id(node));
        let el = apply_shared(el, node);
        let el = apply_listeners(el, node);
        maybe_animated(el, node)
    } else {
        let el = apply_shared(base, node);
        maybe_animated(el, node)
    }
}

fn needs_state(node: &Node) -> bool {
    node.interaction.focusable
        || node.interaction.on_activate.is_some()
        || node.interaction.on_activate_modified.is_some()
        || node.interaction.on_context.is_some()
        || node.interaction.on_key.is_some()
        || node.interaction.drag_payload.is_some()
        || node.interaction.drop_zone
        || node.interaction.on_text_change.is_some()
        || node.interaction.on_drag.is_some()
        || node.id.is_some()
        // `active` style patches and scroll overflow live on gpui 0.2.2's
        // StatefulInteractiveElement — both need element state.
        || node.style.active.is_some()
        || node.style.descriptor.layout.overflow_x == LayoutOverflow::Scroll
        || node.style.descriptor.layout.overflow_y == LayoutOverflow::Scroll
}

/// The channels every box gets, in the Jetstream walk's order: layout,
/// position, paint, text, cursor, state patches, children.
fn apply_shared<E>(el: E, node: &Node) -> E
where
    E: Styled + InteractiveElement + ParentElement + 'static,
{
    let el = apply_layout(el, node);
    let el = apply_position(el, node);
    let el = apply_paint(el, node);
    let el = apply_text(el, node);
    let el = apply_cursor(el, node);
    let el = apply_state_patches(el, node);
    apply_children(el, node)
}

// ── Layout ──────────────────────────────────────────────────────────

fn apply_layout<E: Styled>(mut el: E, node: &Node) -> E {
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
    }
    if let Some(v) = style.max_width {
        el = el.max_w(px(v));
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

fn apply_position<E: Styled>(mut el: E, node: &Node) -> E {
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
    el
}

// ── Paint ───────────────────────────────────────────────────────────

fn apply_paint<E: Styled>(mut el: E, node: &Node) -> E {
    let style = &node.style;
    let d = &style.descriptor;

    // Background and gradient are independent channels (a toast paints a
    // tint fill AND a fade gradient over it), so both apply when set.
    if let Some(bg) = d.background {
        el = el.bg(color(bg));
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
    }
    if let Some(top) = style.border_color_top {
        el = el.border_color(color(top));
    }
    if let Some(left) = style.border_color_left {
        el = el.border_color(color(left));
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
        // APPROXIMATION: gpui 0.2.2 `BoxShadow` has no inset flag, so inset
        // (highlight) layers are dropped; drop layers map exactly.
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
    }

    if d.opacity != 1.0 {
        el = el.opacity(d.opacity);
    }
    if !d.visible {
        el = el.invisible();
    }
    el
}

// ── Text ────────────────────────────────────────────────────────────

fn apply_text<E: Styled>(mut el: E, node: &Node) -> E {
    let style = &node.style;
    let d = &style.descriptor;

    if let Some(c) = d.text_color {
        el = el.text_color(color(c));
    }
    if let Some(size) = style.text_size {
        el = el.text_size(px(size));
    }
    if let Some(weight) = style.text_weight {
        el = el.font_weight(gpui::FontWeight(weight as f32));
    }
    if style.text_italic {
        el = el.italic();
    }
    if let Some(lh) = style.line_height {
        // Vocabulary: a multiple of the font size → relative length.
        el = el.line_height(relative(lh));
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

fn apply_cursor<E: Styled>(mut el: E, node: &Node) -> E {
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
    el
}

fn apply_patch(mut s: StyleRefinement, patch: StylePatch) -> StyleRefinement {
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

fn apply_state_patches<E: InteractiveElement>(mut el: E, node: &Node) -> E {
    // Disabled nodes keep their baked style and get no patches — the
    // vocabulary's contract is "renders in the disabled state the style
    // already describes".
    if node.interaction.disabled {
        return el;
    }
    if let Some(patch) = &node.style.hover {
        let patch = *patch;
        el = el.hover(move |s| apply_patch(s, patch));
    }
    el
}

fn apply_listeners(mut el: Stateful<Div>, node: &Node) -> Stateful<Div> {
    if node.interaction.focusable {
        el = el.focusable();
    }
    if !node.interaction.disabled {
        if let Some(patch) = &node.style.active {
            let patch = *patch;
            el = el.active(move |s| apply_patch(s, patch));
        }
    }
    if node.interaction.disabled {
        return el;
    }
    if let Some(handler) = &node.interaction.on_activate {
        let click = handler.clone();
        el = el.on_click(
            move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                click();
                // Node handlers carry no context, so they cannot notify the
                // entity that owns the state they mutated; a repaint is what
                // lets the host observe the mutation on the next frame.
                cx.refresh_windows();
            },
        );
        // Activation includes Enter/Space on a focused node — the same
        // semantics the old tier wired per component.
        if node.interaction.focusable {
            let key = handler.clone();
            el = el.on_key_down(move |event: &KeyDownEvent, _window, _cx| {
                if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                    key();
                }
            });
        }
    }
    if let NodeKind::Input { value, .. } = &node.kind {
        let current_value = value.clone();
        let change = node.interaction.on_text_change.clone();
        let submit = node.interaction.on_submit.clone();
        let cancel = node.interaction.on_cancel.clone();
        if change.is_some() || submit.is_some() || cancel.is_some() {
            el = el.on_key_down(move |event: &KeyDownEvent, _window, cx| {
                let key = event.keystroke.key.as_str();
                if matches!(key, "enter" | "tab") {
                    if let Some(handler) = &submit {
                        handler();
                        cx.refresh_windows();
                        return;
                    }
                } else if key == "escape" {
                    if let Some(handler) = &cancel {
                        handler();
                        cx.refresh_windows();
                        return;
                    }
                }

                if let Some(change) = &change {
                    if let Some(next) = replacement_for_key(
                        &current_value,
                        key,
                        event.keystroke.modifiers.platform,
                        event.keystroke.modifiers.control,
                    ) {
                        change(&next);
                        cx.refresh_windows();
                    }
                }
            });
        }
    }
    if let Some(handler) = &node.interaction.on_drag {
        // gpui 0.2.2's fluent surface has mouse-down and mouse-move but no
        // mouse-up listener (the same delta the old tier's slider records),
        // so NodeDragPhase::End is never emitted. Deltas are per-frame, from
        // the last reported pointer position — the vocabulary's contract.
        let last: Rc<RefCell<Option<(f32, f32)>>> = Rc::new(RefCell::new(None));
        let last_down = last.clone();
        let last_move = last.clone();
        let down = handler.clone();
        let mv = handler.clone();
        el = el
            .on_mouse_down(MouseButton::Left, move |event, _window, _cx| {
                *last_down.borrow_mut() = Some((event.position.x.into(), event.position.y.into()));
                down(&NodeDragEvent {
                    phase: NodeDragPhase::Start,
                    delta_x: 0.0,
                    delta_y: 0.0,
                });
            })
            .on_mouse_move(move |event, _window, cx| {
                if event.pressed_button != Some(MouseButton::Left) {
                    return;
                }
                let pos: (f32, f32) = (event.position.x.into(), event.position.y.into());
                let mut last = last_move.borrow_mut();
                if let Some(prev) = *last {
                    mv(&NodeDragEvent {
                        phase: NodeDragPhase::Move,
                        delta_x: pos.0 - prev.0,
                        delta_y: pos.1 - prev.1,
                    });
                    cx.refresh_windows();
                }
                *last = Some(pos);
            });
    }
    el = apply_selection_listeners(el, node);
    apply_drop_listeners(el, node)
}

/// Collapse gpui's platform modifier pair onto the vocabulary's single
/// `accel` flag, so components never branch on the host OS.
fn node_modifiers(m: &gpui::Modifiers) -> NodeModifiers {
    NodeModifiers {
        shift: m.shift,
        accel: m.platform || m.control,
        alt: m.alt,
    }
}

fn node_key(key: &str) -> Option<NodeKey> {
    Some(match key {
        "up" => NodeKey::ArrowUp,
        "down" => NodeKey::ArrowDown,
        "left" => NodeKey::ArrowLeft,
        "right" => NodeKey::ArrowRight,
        "home" => NodeKey::Home,
        "end" => NodeKey::End,
        "space" => NodeKey::Space,
        "f2" => NodeKey::F2,
        _ => return None,
    })
}

/// Modifier-aware activation, secondary activation, and navigation keys.
fn apply_selection_listeners(mut el: Stateful<Div>, node: &Node) -> Stateful<Div> {
    if let Some(handler) = &node.interaction.on_activate_modified {
        let click = handler.clone();
        el = el.on_click(move |event: &ClickEvent, _window, cx| {
            click(node_modifiers(&event.modifiers()));
            cx.refresh_windows();
        });
    }
    if let Some(handler) = &node.interaction.on_context {
        let ctx = handler.clone();
        el = el.on_mouse_down(MouseButton::Right, move |event, _window, cx| {
            ctx(NodePoint {
                x: event.position.x.into(),
                y: event.position.y.into(),
            });
            cx.refresh_windows();
        });
    }
    if let Some(handler) = &node.interaction.on_key {
        let keys = handler.clone();
        el = el.on_key_down(move |event: &KeyDownEvent, _window, cx| {
            if let Some(key) = node_key(event.keystroke.key.as_str()) {
                keys(key, node_modifiers(&event.keystroke.modifiers));
                cx.refresh_windows();
            }
        });
    }
    el
}

/// Drag sources and drop zones.
///
/// The edge is derived here, from the zone's own bounds, and only the
/// resulting `DropEdge` reaches the component — the vocabulary's rule that a
/// component never sees layout stays intact.
fn apply_drop_listeners(mut el: Stateful<Div>, node: &Node) -> Stateful<Div> {
    if let Some(payload) = &node.interaction.drag_payload {
        let payload = NodeDragPayload {
            id: payload.clone(),
        };
        el = el.on_drag(payload, |_payload, _offset, _window, cx| {
            // gpui requires a preview entity; the drop indicator is drawn by
            // the component from its own `on_drop_hover` state, so this is
            // deliberately empty.
            cx.new(|_| EmptyDragPreview)
        });
    }
    if !node.interaction.drop_zone {
        return el;
    }
    // A branch zone accepts an "inside" drop; a leaf only takes before/after.
    let accepts_inside = node.a11y.role == Some(NodeRole::TreeItem) || node.children.is_empty();
    if let Some(handler) = &node.interaction.on_drop_hover {
        let hover = handler.clone();
        el = el.on_drag_move::<NodeDragPayload>(move |event, _window, cx| {
            let height = f32::from(event.bounds.size.height).max(1.0);
            let rel = f32::from(event.event.position.y - event.bounds.origin.y) / height;
            hover(&NodeDropEvent {
                payload: event.drag(cx).id.clone(),
                edge: edge_for(rel, accepts_inside),
            });
            cx.refresh_windows();
        });
    }
    if let Some(handler) = &node.interaction.on_drop {
        let drop = handler.clone();
        el = el.on_drop::<NodeDragPayload>(move |payload, _window, cx| {
            drop(&NodeDropEvent {
                payload: payload.id.clone(),
                // The last hover already told the component where the
                // indicator sits; a drop reuses it rather than recomputing
                // from a position gpui does not hand to `on_drop`.
                edge: DropEdge::default(),
            });
            cx.refresh_windows();
        });
    }
    el
}

/// Split a zone's height into before / inside / after bands. A zone that
/// cannot take an inside drop splits in half instead of thirds.
fn edge_for(rel: f32, accepts_inside: bool) -> DropEdge {
    if accepts_inside {
        if rel < 0.25 {
            DropEdge::Before
        } else if rel > 0.75 {
            DropEdge::After
        } else {
            DropEdge::Inside
        }
    } else if rel < 0.5 {
        DropEdge::Before
    } else {
        DropEdge::After
    }
}

/// The dragged node's opaque id, carried through gpui's drag channel.
#[derive(Clone, Debug)]
struct NodeDragPayload {
    id: String,
}

/// gpui requires a preview entity for every drag; components draw their own
/// indicator, so this renders nothing.
struct EmptyDragPreview;

impl gpui::Render for EmptyDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div()
    }
}

fn replacement_for_key(
    current: &str,
    key: &str,
    platform_modifier: bool,
    control_modifier: bool,
) -> Option<String> {
    if key == "backspace" {
        let mut chars: Vec<char> = current.chars().collect();
        chars.pop();
        Some(chars.into_iter().collect())
    } else if key.chars().count() == 1 && !platform_modifier && !control_modifier {
        Some(format!("{current}{key}"))
    } else {
        None
    }
}

fn apply_children<E: ParentElement>(mut el: E, node: &Node) -> E {
    for child in &node.children {
        el = el.child(to_gpui(child));
    }
    el
}

// ── Animation ───────────────────────────────────────────────────────

/// Sample one animated property at cycle position `t` (0.0..=1.0):
/// piecewise-linear between the keyframes that declare it, clamped at the
/// ends. Pure — the unit-tested half of the animation channel.
fn sample_property(anim: &NodeAnimation, prop: AnimProperty, t: f32) -> Option<f32> {
    let mut keys: Vec<(f32, f32)> = anim
        .keyframes
        .iter()
        .filter_map(|k| {
            k.values
                .iter()
                .find(|(p, _)| *p == prop)
                .map(|(_, v)| (k.at, *v))
        })
        .collect();
    if keys.is_empty() {
        return None;
    }
    keys.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let t = t.clamp(0.0, 1.0);
    if t <= keys[0].0 {
        return Some(keys[0].1);
    }
    if t >= keys[keys.len() - 1].0 {
        return Some(keys[keys.len() - 1].1);
    }
    for pair in keys.windows(2) {
        let (t0, v0) = pair[0];
        let (t1, v1) = pair[1];
        if t >= t0 && t <= t1 {
            let span = t1 - t0;
            let f = if span > 0.0 { (t - t0) / span } else { 0.0 };
            return Some(v0 + (v1 - v0) * f);
        }
    }
    Some(keys[keys.len() - 1].1)
}

fn gpui_animation(anim: &NodeAnimation) -> gpui::Animation {
    let a = gpui::Animation::new(Duration::from_secs_f32(anim.duration_secs));
    // APPROXIMATION: gpui 0.2.2 animations repeat or run once; there is no
    // ping-pong mode, so PingPong degrades to Loop.
    let a = match anim.loop_mode {
        AnimLoop::Once => a,
        AnimLoop::Loop | AnimLoop::PingPong => a.repeat(),
    };
    match anim.easing {
        AnimEasing::Linear => a,
        AnimEasing::EaseIn => a.with_easing(|t| t * t),
        AnimEasing::EaseOut => a.with_easing(|t| 1.0 - (1.0 - t) * (1.0 - t)),
        AnimEasing::EaseInOut => a.with_easing(|t| {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            }
        }),
    }
}

/// Opacity is the one property gpui 0.2.2 can animate on any Styled element.
/// SVG rotation is handled above; other transform channels remain unavailable
/// on generic elements.
fn maybe_animated<E>(el: E, node: &Node) -> AnyElement
where
    E: Styled + IntoElement + 'static,
{
    use gpui::AnimationExt;
    let Some(anim) = &node.style.animation else {
        return el.into_any_element();
    };
    let anim = anim.clone();
    let id = element_id(node);
    el.with_animation(id, gpui_animation(&anim), move |el, t| {
        let mut el = el;
        if let Some(v) = sample_property(&anim, AnimProperty::Opacity, t) {
            el = el.opacity(v);
        }
        el
    })
    .into_any_element()
}

// ── Accessibility ───────────────────────────────────────────────────
//
// NodeA11y (role, label, expanded, selected, toggled, level) is intentionally
// NOT mapped: gpui 0.2.2's fluent element API exposes no accessibility
// attributes. The old GPUI tier recorded the same gap (see the note in
// `poodle-gpui-components::primitives::button`), and g12.015 holds GPUI
// accessibility upstream work deliberately. The channels are walked (read)
// here so the omission is a decision, not a drift.

#[cfg(test)]
mod tests;
