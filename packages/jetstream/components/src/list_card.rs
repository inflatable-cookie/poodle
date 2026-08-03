//! ListCard — Jetstream list card backed by ListCardSpec.
//!
//! Contract: `docs/contracts/components/list-card.md`
//! Reference: `packages/svelte/components/src/ListCard.svelte`
//! Mirrors the GPUI build-out (`packages/gpui/components/src/primitives/list_card.rs`).
//!
//! Anatomy (contract §2):
//! `[sash?] [selection?] [leading?] [body(header(title + accessories/badges) / subtitle / footer?)] [meta?] [actions?] [trailing?] [handle?]`.
//! Every dimension resolves from a token or a contract-exact rem via the spec
//! helpers — zero hardcoded hsla; rem literals are contract values resolved by
//! `rem_to_px`. Interaction (click/keyboard) lives in the preview event loop.
//!
//! Host-snippet slots (contract §2 / §3) — `leading`, `badges`, `corner`,
//! `footer`, `actions`, `trailing` — are content the host composes (`JsEl`), not
//! spec fields, so they arrive through [`js_list_card_with_slots`];
//! [`js_list_card`] delegates with empty slots for back-compat.

use jetstream_ui::ui_element::{self, BorderStyle, BoxShadow, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{LeadingFill, LeadingShape, ListCardLayout, ListCardSpec, SelectionIndicator};

use crate::presentation::rem_to_px;
use crate::theme_ext::{
    color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius, tint,
};

/// Render a list card with no host slots (derived leading glyph, no
/// badges/footer/actions/trailing). Back-compat entry — existing callers are
/// unchanged.
/// ListCard — one row-card in a list.
///
/// Mirrors the GPUI target's `on_click`. The whole card is the hit target, and
/// only an interactive card fires — `is_interactive` or an `href` is what makes
/// it one, exactly the condition that already draws the pointer cursor.
pub struct ListCard {
    spec: ListCardSpec,
    theme: JetstreamThemeProvider,
    on_click: Option<crate::element::ActionHandler>,
}

impl ListCard {
    pub fn from_spec(spec: ListCardSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_click = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for ListCard {
    fn into_js_el(self) -> JsEl {
        let interactive =
            (self.spec.is_interactive || self.spec.href.is_some()) && !self.spec.is_disabled;
        let el = js_list_card(&self.spec, &self.theme);

        match (interactive, self.on_click) {
            (true, Some(handler)) => el.on_click(move |_event| handler()),
            _ => el,
        }
    }
}

pub fn js_list_card(spec: &ListCardSpec, theme: &JetstreamThemeProvider) -> JsEl {
    js_list_card_with_slots(spec, theme, None, Vec::new(), None, None, None, None)
}

/// Render a list card with the contract's optional host-snippet slots.
///
/// - `leading` — avatar/icon/thumbnail content; rendered inside the styled
///   leading box and **overriding** the derived first-letter glyph (contract §2
///   Leading; mirrors GPUI `with_leading`).
/// - `badges` — pills/badges rendered in the header-accessories cluster inline
///   next to the title (contract §2 Badges / §8 Header Accessories).
/// - `corner` — supplementary header-corner content rendered alongside badges in
///   the header-accessories cluster, painted at the tertiary text color
///   (contract §2 Corner / §8 Badges-and-Corner).
/// - `footer` — counter row rendered in the body column below the subtitle with
///   the contract `margin-top 0.125rem` (contract §2 Footer / §8 Footer).
/// - `actions` — explicit action triggers in the right-edge lane after meta
///   (contract §2 Actions).
/// - `trailing` — exclusive right-edge lane; when present it **replaces** both
///   meta and actions so the card has a single trailing lane (contract §3
///   `trailing` / §7 right-edge composition).
pub fn js_list_card_with_slots(
    spec: &ListCardSpec,
    theme: &JetstreamThemeProvider,
    leading: Option<JsEl>,
    badges: Vec<JsEl>,
    footer: Option<JsEl>,
    actions: Option<JsEl>,
    trailing: Option<JsEl>,
    corner: Option<JsEl>,
) -> JsEl {
    let surface = resolve_color(theme, spec.fill_token());
    let text_primary = resolve_color(theme, spec.title_color_token());
    let border_subtle = resolve_color(theme, spec.border_token());
    let border_default = resolve_color(theme, spec.hover_border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_secondary = resolve_color(theme, spec.subtitle_color_token());
    let accent = resolve_color(theme, spec.accent_base_token());
    let on_accent = resolve_color(theme, spec.on_accent_color_token());
    let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

    // Svelte: fill = color-mix(surface 88%, text-primary); hover = 82%.
    let fill = color_mix(surface, text_primary, 0.88);
    let hover_fill = color_mix(surface, text_primary, 0.82);
    // Svelte: border = color-mix(border-subtle 18%, transparent); hover = default 52%.
    let border = tint(border_subtle, 0.18);
    let hover_border = tint(border_default, 0.52);

    // Spacing — contract §8 Root: padding 0.625rem 0.75rem, gap 0.75rem.
    let pad_x = resolve_px(theme, "space.inline.md"); // 0.75rem
    let pad_y = resolve_px(theme, "space.stack.sm"); // 0.5rem — see NOTE (contract 0.625rem)
    let gap = resolve_px(theme, "space.inline.md"); // 0.75rem

    // Typography — title from label-size token; subtitle/meta 0.75rem (no token).
    let title_font = resolve_px(theme, "typography.label.size");
    let small_font = rem_to_px(spec.small_font_size_rem());

    let is_compact = spec.layout == ListCardLayout::Compact;
    let is_stacked = spec.layout == ListCardLayout::Stacked;

    // ── Leading: shape-sized square, tint/solid fill (contract §7/§8) ───
    let leading_size = rem_to_px(spec.leading_size_rem());
    let leading_font = rem_to_px(spec.leading_font_size_rem());
    let leading_radius = match spec.leading_shape {
        LeadingShape::Circle => leading_size / 2.0,
        LeadingShape::RoundedSquare => resolve_radius(theme, spec.leading_radius_token()),
    };
    let leading_bg = match spec.leading_fill {
        LeadingFill::Tint => tint(accent, spec.leading_tint_ratio()),
        LeadingFill::Solid => accent,
    };
    let leading_icon_color = match spec.leading_fill {
        LeadingFill::Tint => accent,
        LeadingFill::Solid => on_accent,
    };

    // The styled leading box always paints the shape/tint/solid fill. Its child
    // is the host `leading` slot when supplied (contract §2 Leading — avatar /
    // icon / thumbnail), otherwise the derived first-letter glyph. The slot
    // overrides the derived avatar, mirroring GPUI `with_leading`.
    let leading_inner = leading.unwrap_or_else(|| {
        ui_element::label(
            &spec
                .title
                .chars()
                .next()
                .map_or(String::new(), |c| c.to_uppercase().to_string()),
        )
        .text_color(leading_icon_color)
        .text_size(leading_font)
        .text_weight(600)
    });

    let leading_el = ui_element::div()
        .size(leading_size)
        .flex_none()
        .rounded(leading_radius)
        .bg(leading_bg)
        .items_center()
        .justify_center()
        .overflow_hidden()
        .text_color(leading_icon_color)
        .child(leading_inner);

    // ── Body: header (title + badges) + optional subtitle + optional footer ──
    let mut body = ui_element::div()
        .flex_col()
        .gap(rem_to_px(spec.body_gap_rem()))
        .flex_grow()
        .min_w_0();

    // Header row — contract §8 Header: gap 0.375rem. Title truncates (flex 1,
    // min-width 0); the badges cluster is shrink-proof beside it.
    let title_el = ui_element::label(&spec.title)
        .text_color(text_primary)
        .text_size(title_font)
        .text_weight(500)
        .text_ellipsis()
        .flex_grow()
        .min_w_0();

    // Header-accessories cluster holds badges and/or corner content beside the
    // title (contract §2 HeaderAccessories). It renders only when at least one
    // of those slots is supplied.
    if badges.is_empty() && corner.is_none() {
        body = body.child(title_el);
    } else {
        // Header-accessories cluster (contract §8): shrink-proof, wraps, gap
        // space.inline.sm; the badges/corner groups each use gap space.inline.xs.
        let mut accessories = ui_element::div()
            .flex_row()
            .flex_none()
            .items_center()
            .flex_wrap()
            .gap(resolve_px(theme, "space.inline.sm"));

        if !badges.is_empty() {
            // Badges group — gap space.inline.xs (contract §8 Badges-and-Corner).
            accessories = accessories.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(resolve_px(theme, "space.inline.xs"))
                    .children(badges),
            );
        }

        if let Some(corner) = corner {
            // Corner group — gap space.inline.xs, tertiary text color
            // (contract §8 Badges-and-Corner: "Corner additionally sets color:
            // var(--poodle-color-text-tertiary)").
            let text_tertiary = resolve_color(theme, "color.text.tertiary");
            accessories = accessories.child(
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(resolve_px(theme, "space.inline.xs"))
                    .text_color(text_tertiary)
                    .child(corner),
            );
        }

        body = body.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(rem_to_px(spec.header_gap_rem()))
                .child(title_el)
                .child(accessories),
        );
    }

    if let Some(ref subtitle) = spec.subtitle {
        body = body.child(
            ui_element::label(subtitle)
                .text_color(text_secondary)
                .text_size(small_font)
                .text_ellipsis(),
        );
    }

    // Footer — contract §8 Footer: gap 0.5rem, margin-top 0.125rem. The footer
    // slot is host-composed (e.g. ListCardCounters); the row wrapper supplies
    // the contract spacing.
    if let Some(footer) = footer {
        body = body.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(rem_to_px(spec.footer_gap_rem()))
                .mt(rem_to_px(0.125))
                .child(footer),
        );
    }

    // ── Right-edge lanes (contract §7) ──────────────────────────────────
    // `trailing` is exclusive: when present it replaces both meta and actions
    // so the card has a single trailing lane. Otherwise meta + actions compose.
    let has_trailing = trailing.is_some();

    // Meta (right-aligned). NOTE: tabular-nums has no JsEl API.
    let meta_el = (!has_trailing)
        .then(|| spec.meta.as_ref())
        .flatten()
        .map(|m| {
            ui_element::label(m)
                .text_color(text_secondary)
                .text_size(small_font)
                .flex_none()
        });

    // Actions lane (contract §2 Actions) — explicit action triggers, shrink-proof,
    // suppressed when `trailing` claims the right edge.
    let actions_el = actions.filter(|_| !has_trailing).map(|a| {
        ui_element::div()
            .flex_row()
            .flex_none()
            .items_center()
            .gap(resolve_px(theme, "space.inline.xs"))
            .child(a)
    });

    // Trailing lane (contract §8 Trailing) — exclusive, shrink-proof.
    let trailing_el = trailing.map(|t| {
        ui_element::div()
            .flex_row()
            .flex_none()
            .items_center()
            .child(t)
    });

    // ── Selection indicator (checkbox box) — contract §3/§8 ─────────────
    let selection_el = (spec.is_selectable
        && spec.selection_indicator == SelectionIndicator::Checkbox)
        .then(|| {
            let box_size = resolve_px(theme, spec.selection_indicator_size_token());
            let pill = resolve_radius(theme, spec.leading_radius_token());
            let (box_bg, box_border) = if spec.is_selected {
                (accent, accent)
            } else {
                (surface, border_subtle)
            };
            ui_element::div()
                .size(box_size)
                .flex_none()
                .rounded(pill)
                .border(1.0)
                .border_color(box_border)
                .bg(box_bg)
        });

    // ── Reorder handle (two columns of dots) — contract §2 Handle ───────
    let handle_el = spec.show_reorder_handle.then(|| {
        let dot = resolve_px(theme, "space.inline.xs") / 2.0; // 0.125rem dot
        let dot_gap = rem_to_px(0.125);
        let handle_color = text_secondary;
        let col = || {
            ui_element::div()
                .flex_col()
                .gap(dot_gap)
                .child(ui_element::div().size(dot).rounded(dot).bg(handle_color))
                .child(ui_element::div().size(dot).rounded(dot).bg(handle_color))
                .child(ui_element::div().size(dot).rounded(dot).bg(handle_color))
        };
        ui_element::div()
            .flex_row()
            .flex_none()
            .items_center()
            .gap(dot_gap)
            .opacity(0.6)
            .child(col())
            .child(col())
    });

    // ── Sash ribbon (top-left). NOTE: JsEl has no transform/rotate, so the ──
    //    diagonal rotate(-45deg) is approximated by a top-left block. ───────
    let sash_el = spec.sash.as_ref().map(|sash_text| {
        let sash_bg = spec
            .sash_color
            .as_ref()
            .and_then(|c| crate::theme_ext::hex_to_rgb255(c))
            .map(|rgb| crate::theme_ext::rgb255_to_vec4(rgb, 1.0))
            .unwrap_or_else(|| resolve_color(theme, spec.sash_bg_token()));
        ui_element::label(&sash_text.to_uppercase())
            .absolute()
            .top(0.0)
            .left(0.0)
            .px(rem_to_px(0.375))
            .py(rem_to_px(0.0625))
            .bg(sash_bg)
            .text_color(on_accent)
            .text_size(rem_to_px(0.5625))
            .text_weight(700)
    });

    // ── Root row (or column when stacked) ───────────────────────────────
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .gap(if is_compact { gap / 2.0 } else { gap });

    el = if is_stacked {
        el.flex_col().items_start()
    } else {
        el.flex_row().items_center()
    };

    // Highlighted: accent-tinted border + inset accent ring (contract §8 Root
    // highlighted, independent of selection). The accent-to-transparent gradient
    // is approximated by compositing the accent at 10% over the base fill (JsEl
    // can't paint a gradient overlay); the ring is now a real spread-based inset
    // box-shadow rather than a border stand-in, so it hugs the rounded edge
    // without affecting layout.
    if spec.is_highlighted {
        // Contract: box-shadow inset 0 0 0 0.0625rem color-mix(accent 12%, transparent).
        let ring_spread = rem_to_px(0.0625);
        let ring_color = tint(accent, 0.12);
        el = el
            .border_color(tint(accent, 0.34))
            .bg(color_mix(accent, fill, 0.10))
            .shadow_layers(vec![BoxShadow {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: ring_spread,
                color: ring_color,
                inset: true,
            }]);
    }

    if let Some(sel) = selection_el {
        el = el.child(sel);
    }
    // Active — the card you are currently on. A bar down the leading edge and
    // nothing else: `is_selected` already owns the loud treatment, and this
    // state is always on for one card in a list, so at that weight it would
    // shout permanently. Contract §4.
    //
    // An inset shadow, not a child element. A child rectangle cannot follow the
    // card's corner radius and juts out squarely top and bottom left; an inset
    // shadow is clipped by the radius, so the bar curves with the card the way
    // a real border does.
    if spec.is_active {
        let bar = BoxShadow {
            offset_x: rem_to_px(spec.active_bar_width_rem()),
            offset_y: 0.0,
            blur: 0.0,
            spread: 0.0,
            color: accent,
            inset: true,
        };
        // Preserve whatever `selected` or `highlighted` already put there;
        // replacing the list would drop their rings.
        let mut layers = el.style.shadow_layers.clone();
        layers.push(bar);
        el = el.shadow_layers(layers);
    }

    el = el.child(leading_el).child(body);

    // Right edge: meta + actions, OR the exclusive trailing lane (contract §7).
    if let Some(m) = meta_el {
        el = el.child(m);
    }
    if let Some(a) = actions_el {
        el = el.child(a);
    }
    if let Some(t) = trailing_el {
        el = el.child(t);
    }

    if let Some(handle) = handle_el {
        el = el.child(handle);
    }

    if let Some(sash) = sash_el {
        el = el.relative().overflow_hidden().child(sash);
    }

    // Not-live: dashed 0.1875rem border at border-default @ 72%, grayscale(1), and
    // reduced opacity (0.72) (contract §8). Grayscale/opacity restore on hover is a
    // hover-state refinement (JsStyleOverride has no grayscale lane) — base state applied.
    if spec.is_not_live {
        el = el
            .border(rem_to_px(0.1875))
            .border_style(BorderStyle::Dashed)
            .border_color(tint(border_default, 0.72))
            .grayscale(1.0)
            .opacity(spec.not_live_opacity());
    }

    // Disabled: token opacity.
    if spec.is_disabled {
        el = el.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    // Interactive: hover background + border, pointer + focusable. Not-live
    // cards stay interactive (contract §8) but restore their dashed border to
    // full border-default on hover instead of the generic 52% tint.
    if (spec.is_interactive || spec.href.is_some()) && !spec.is_disabled {
        let hover_border = if spec.is_not_live {
            border_default
        } else {
            hover_border
        };
        el = el
            .cursor_pointer()
            .focusable()
            .hover(move |s| s.bg(hover_fill).border_color(hover_border));
        let _ = focus_ring; // surfaced for the preview focus layer; see NOTE.
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {

    /// The active bar is an inset shadow so the card's radius clips it. A child
    /// rectangle cannot follow the corners and juts out squarely, which is what
    /// the first attempt did.
    #[test]
    fn active_draws_an_inset_bar_that_the_radius_can_clip() {
        let th = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let plain = js_list_card(&ListCardSpec::new().with_title("Row"), &th);
        let active = js_list_card(
            &ListCardSpec::new().with_title("Row").with_active(true),
            &th,
        );

        assert!(
            active.style.shadow_layers.len() > plain.style.shadow_layers.len(),
            "active should add a shadow layer"
        );
        let bar = active.style.shadow_layers.last().unwrap();
        assert!(
            bar.inset,
            "the bar must be inset, or the radius cannot clip it"
        );
        assert!(
            bar.offset_x > 0.0,
            "the bar is offset from the leading edge"
        );
        assert_eq!(bar.blur, 0.0, "a bar, not a glow");
    }

    /// Active and selected are orthogonal, so marking a selected card active
    /// must not drop the ring selection already painted.
    #[test]
    fn active_preserves_the_selected_ring() {
        let th = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let selected = js_list_card(
            &ListCardSpec::new()
                .with_title("Row")
                .with_selectable(true)
                .with_selected(true),
            &th,
        );
        let both = js_list_card(
            &ListCardSpec::new()
                .with_title("Row")
                .with_selectable(true)
                .with_selected(true)
                .with_active(true),
            &th,
        );
        assert!(both.style.shadow_layers.len() > selected.style.shadow_layers.len());
    }
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::{LeadingFill, LeadingShape, SelectionIndicator};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn accent(th: &JetstreamThemeProvider) -> ProbeColor {
        let c = resolve_color(th, "color.accent.base");
        ProbeColor {
            r: c.x,
            g: c.y,
            b: c.z,
            a: c.w,
        }
    }

    fn spec() -> ListCardSpec {
        ListCardSpec::new()
            .with_title("design-system-v2.figma")
            .with_subtitle("Edited 2 hours ago")
            .with_meta("14.2 MB")
            .with_interactive(true)
    }

    #[test]
    fn renders_title_subtitle_meta() {
        let th = theme();
        let tree = probe(&js_list_card(&spec(), &th), 360.0, 96.0);
        assert!(!tree.is_empty(), "probe produced no nodes");
        assert!(
            tree.has_text("design-system-v2.figma"),
            "title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Edited 2 hours ago"),
            "subtitle missing: {:?}",
            tree.texts()
        );
        assert!(tree.has_text("14.2 MB"), "meta missing: {:?}", tree.texts());
    }

    /// Shape is a radius, not a size.
    ///
    /// This test used to assert circle = 2rem and rounded-square = 2.75rem,
    /// which are simply two different steps of the size ladder — it had
    /// conflated shape with size and had been failing ever since. The Svelte
    /// reference settles it: `data-leading-shape` sets `border-radius` and
    /// nothing else, while the box is sized from `data-leading-size`.
    #[test]
    fn leading_shape_does_not_change_the_box_size() {
        let th = theme();
        let edge = rem_to_px(spec().leading_size_rem());
        let has_box = |t: &crate::render_probe::ProbeTree| {
            t.nodes
                .iter()
                .any(|n| (n.w - edge).abs() < 0.5 && (n.h - edge).abs() < 0.5)
        };

        for shape in [LeadingShape::Circle, LeadingShape::RoundedSquare] {
            let tree = probe(
                &js_list_card(&spec().with_leading_shape(shape), &th),
                360.0,
                96.0,
            );
            assert!(
                has_box(&tree),
                "{shape:?} leading box should be {edge}px square"
            );
        }
    }

    /// `leadingSizeOffset` steps along the control-size ladder.
    ///
    /// The expected values are read from the spec rather than written as
    /// literals: the previous version hardcoded a 2rem base, which is the `sm`
    /// step, while the default control size is `md`. Every assertion was
    /// therefore one step out.
    #[test]
    fn leading_size_offset_shifts_leading_box() {
        let th = theme();
        let has_box = |t: &crate::render_probe::ProbeTree, edge: f32| {
            t.nodes
                .iter()
                .any(|n| (n.w - edge).abs() < 0.5 && (n.h - edge).abs() < 0.5)
        };

        for offset in [-1, 0, 1] {
            let s = spec().with_leading_size_offset(offset);
            let edge = rem_to_px(s.leading_size_rem());
            let tree = probe(&js_list_card(&s, &th), 360.0, 96.0);
            assert!(
                has_box(&tree, edge),
                "offset {offset} should give a {edge}px leading box"
            );
        }

        // …and the ladder must actually move, or the loop above would pass on
        // a component that ignored the offset entirely.
        let smaller = spec().with_leading_size_offset(-1).leading_size_rem();
        let base = spec().leading_size_rem();
        let bigger = spec().with_leading_size_offset(1).leading_size_rem();
        assert!(
            smaller < base && base < bigger,
            "{smaller} < {base} < {bigger}"
        );
    }

    #[test]
    fn sash_ribbon_renders_text() {
        let th = theme();
        let tree = probe(&js_list_card(&spec().with_sash("Free"), &th), 360.0, 96.0);
        assert!(
            tree.has_text("FREE"),
            "sash ribbon text missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn empty_meta_and_subtitle_omitted() {
        let th = theme();
        // A bare title-only card renders just the title (plus leading glyph),
        // proving optional regions are correctly omitted (empty-state shape).
        let tree = probe(
            &js_list_card(&ListCardSpec::new().with_title("Read-only item"), &th),
            360.0,
            96.0,
        );
        assert!(
            tree.has_text("Read-only item"),
            "title missing: {:?}",
            tree.texts()
        );
        // No meta text like a size string leaks in.
        assert!(!tree.has_text("14.2 MB"), "unexpected meta on bare card");
    }

    #[test]
    fn default_no_slots_still_renders_title_subtitle() {
        // Back-compat: js_list_card (no slots) still renders title + subtitle.
        let th = theme();
        let tree = probe(&js_list_card(&spec(), &th), 360.0, 96.0);
        assert!(
            tree.has_text("design-system-v2.figma"),
            "title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Edited 2 hours ago"),
            "subtitle missing: {:?}",
            tree.texts()
        );
        // Derived first-letter glyph present when no leading slot is supplied.
        assert!(
            tree.has_text("D"),
            "derived leading glyph missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn leading_slot_overrides_derived_avatar() {
        let th = theme();
        // A leading slot replaces the derived first-letter glyph (contract §2).
        let leading = ui_element::label("LEAD");
        let tree = probe(
            &js_list_card_with_slots(
                &spec(),
                &th,
                Some(leading),
                Vec::new(),
                None,
                None,
                None,
                None,
            ),
            360.0,
            96.0,
        );
        assert!(
            tree.has_text("LEAD"),
            "leading slot content missing: {:?}",
            tree.texts()
        );
        // The derived first-letter glyph ("D") must be gone — the slot overrides it.
        assert!(
            !tree.has_text("D"),
            "derived avatar glyph should be overridden by the leading slot: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn footer_slot_renders() {
        let th = theme();
        let footer = ui_element::label("24 docs");
        let tree = probe(
            &js_list_card_with_slots(
                &spec(),
                &th,
                None,
                Vec::new(),
                Some(footer),
                None,
                None,
                None,
            ),
            360.0,
            96.0,
        );
        assert!(
            tree.has_text("24 docs"),
            "footer slot content missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn badges_slot_renders_in_header() {
        let th = theme();
        let badges = vec![ui_element::label("New"), ui_element::label("Review")];
        let tree = probe(
            &js_list_card_with_slots(&spec(), &th, None, badges, None, None, None, None),
            360.0,
            96.0,
        );
        assert!(
            tree.has_text("New"),
            "first badge missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Review"),
            "second badge missing: {:?}",
            tree.texts()
        );
        // Title still present alongside the badges.
        assert!(
            tree.has_text("design-system-v2.figma"),
            "title missing with badges"
        );
    }

    #[test]
    fn trailing_slot_renders() {
        let th = theme();
        let trailing = ui_element::label("Open");
        let tree = probe(
            &js_list_card_with_slots(
                &spec(),
                &th,
                None,
                Vec::new(),
                None,
                None,
                Some(trailing),
                None,
            ),
            360.0,
            96.0,
        );
        assert!(
            tree.has_text("Open"),
            "trailing slot content missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn trailing_is_exclusive_and_replaces_meta_and_actions() {
        // Contract §7: trailing replaces meta + actions. The card still has meta
        // text in its spec, but the trailing lane suppresses it.
        let th = theme();
        let trailing = ui_element::label("Trailing");
        let actions = ui_element::label("Action");
        let tree = probe(
            &js_list_card_with_slots(
                &spec(), // spec carries meta "14.2 MB"
                &th,
                None,
                Vec::new(),
                None,
                Some(actions),
                Some(trailing),
                None,
            ),
            360.0,
            96.0,
        );
        assert!(
            tree.has_text("Trailing"),
            "trailing missing: {:?}",
            tree.texts()
        );
        assert!(
            !tree.has_text("14.2 MB"),
            "meta should be suppressed by trailing"
        );
        assert!(
            !tree.has_text("Action"),
            "actions should be suppressed by trailing"
        );
    }

    #[test]
    fn corner_slot_renders_in_header() {
        // Contract §2 Corner: supplementary header-corner content renders in the
        // header-accessories cluster beside the title (alongside any badges).
        let th = theme();
        let corner = ui_element::label("v2.1");
        let tree = probe(
            &js_list_card_with_slots(
                &spec(),
                &th,
                None,
                Vec::new(),
                None,
                None,
                None,
                Some(corner),
            ),
            360.0,
            96.0,
        );
        assert!(
            tree.has_text("v2.1"),
            "corner slot content missing: {:?}",
            tree.texts()
        );
        // Title still present alongside the corner content.
        assert!(
            tree.has_text("design-system-v2.figma"),
            "title missing with corner: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn corner_and_badges_render_together_in_header() {
        // Both badges and corner share the header-accessories cluster (contract §2).
        let th = theme();
        let badges = vec![ui_element::label("New")];
        let corner = ui_element::label("v2.1");
        let tree = probe(
            &js_list_card_with_slots(&spec(), &th, None, badges, None, None, None, Some(corner)),
            360.0,
            96.0,
        );
        assert!(tree.has_text("New"), "badge missing: {:?}", tree.texts());
        assert!(tree.has_text("v2.1"), "corner missing: {:?}", tree.texts());
    }

    #[test]
    fn an_interactive_card_reports_a_click() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = ListCard::from_spec(spec(), &theme())
            .on_click(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 480.0, 120.0, "design-system-v2.figma");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_click fired exactly once"
        );
    }

    /// A card that is not interactive draws no pointer cursor, and must not
    /// fire either — the same condition governs both.
    #[test]
    fn a_static_card_ignores_clicks() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = ListCard::from_spec(
            ListCardSpec::new().with_title("design-system-v2.figma"),
            &theme(),
        )
        .on_click(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .into_js_el();

        crate::element::click_probe::click_text(&el, 480.0, 120.0, "design-system-v2.figma");

        assert_eq!(hits.load(Ordering::SeqCst), 0, "a static card fired");
    }
}
