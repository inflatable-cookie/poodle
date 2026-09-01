//! SplitButton — a primary action with a menu of alternates.
//!
//! Contract: `docs/contracts/components/split-button.md`
//! Ported from: `packages/jetstream/components/src/split_button.rs`. The
//! recipe below now transcribes the old GPUI tier
//! (`packages/gpui/components/src/primitives/split_button.rs`) exactly —
//! axis-faithful metrics, its variant×tone color resolution, its state
//! patches, its menu anatomy — per the g12.019 recipe correction.
//!
//! Menu open/close, click-outside and keyboard navigation are host-owned;
//! the menu panel renders from `spec.is_open` only.

use std::sync::Arc;

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeAnimation, NodeRole, StylePatch,
};
use poodle_specs::{ButtonVariant, SplitButtonSpec, SplitMenuItem};

use crate::color::{mix_srgb, with_alpha, BLACK};
use crate::context::RenderContext;
use crate::presentation::{
    rem_to_px, size_font_rem, size_height_offset_rem, size_padding_x_offset_rem,
    split_button_chevron_size_rem, split_button_toggle_width_rem,
};

/// Host callbacks: primary half, chevron half, and menu-item value.
#[derive(Default)]
pub struct SplitButtonHandlers {
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_dropdown: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_action: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

/// Resolved variant × tone color set for the split-button halves. The old
/// GPUI tier resolves the spec's variant×tone tokens and transforms only
/// Primary (darkened border) and Ghost (alpha-scaled surface/subtle) —
/// every other combination wears the resolved tokens straight.
struct SplitColors {
    fill: ColorValue,
    border: ColorValue,
    text: ColorValue,
}

fn resolve_split_colors(spec: &SplitButtonSpec, ctx: &RenderContext<'_>) -> SplitColors {
    let base_fill = ctx.theme().resolve_color(spec.fill_token());
    let base_border = ctx.theme().resolve_color(spec.border_token());
    let text = ctx.theme().resolve_color(spec.text_token());

    match spec.variant {
        // Primary: border = fill mixed 84% toward black. The old tier's
        // `color_mix_black` preserves the fill's alpha while `mix_srgb`
        // lerps it toward opaque, so restore it (accent/status fills are
        // opaque today, but keep the exact recipe).
        ButtonVariant::Primary => SplitColors {
            fill: base_fill,
            border: with_alpha(mix_srgb(base_fill, BLACK, 0.84), base_fill.3),
            text,
        },
        // Ghost: surface@42% fill, border-subtle@72% border whatever the
        // tone — the tone only recolors the text (via `text_token`).
        ButtonVariant::Ghost => {
            let surface = ctx.theme().resolve_color("color.background.surface");
            let border_subtle = ctx.theme().resolve_color("color.border.subtle");
            SplitColors {
                fill: with_alpha(surface, surface.3 * 0.42),
                border: with_alpha(border_subtle, border_subtle.3 * 0.72),
                text,
            }
        }
        // Secondary (any tone) and the legacy Danger variant: the resolved
        // tokens straight — no status-tint mixes.
        _ => SplitColors {
            fill: base_fill,
            border: base_border,
            text,
        },
    }
}

pub fn split_button(
    spec: &SplitButtonSpec,
    ctx: &RenderContext<'_>,
    handlers: SplitButtonHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    // Axis-faithful metrics (g12.019 recipe correction): the axis-layered
    // token plus the per-size offset — the old GPUI tier's form — not the
    // fixed per-size/per-density tables (`control_height_rem` /
    // `control_space_x_rem`), which ignore the theme's density/control-size
    // layering. At base tokens (the Jetstream provider, no axes) md/default
    // reproduces the old fixed values; under a preview axis the control now
    // follows the axis like Svelte does.
    let height = ctx
        .theme()
        .resolve_space(spec.control_height_token(ctx.base_size(spec.size)))
        + rem_to_px(size_height_offset_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = ctx.theme().resolve_space("space.control.x")
        + rem_to_px(size_padding_x_offset_rem(effective_size));
    // Contract §8 Chevron / Toggle half: the old tier reads these two from
    // the per-size tables, not token+offset.
    let chevron_size = rem_to_px(split_button_chevron_size_rem(effective_size));
    let toggle_w = rem_to_px(split_button_toggle_width_rem(effective_size));
    // Contract §8 Divider: 60% of control height, centered.
    let divider_h = height * 0.6;
    // Contract §8 Primary half: spinner↔label gap = space.inline.sm.
    let primary_gap = ctx.theme().resolve_space("space.inline.sm");

    let colors = resolve_split_colors(spec, ctx);
    let elevated = ctx.theme().resolve_color("color.background.elevated");
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    // The old tier's state recipes: hover/active mix the fill toward
    // elevated; hover also mixes the border toward text-primary (painted on
    // the primary half only).
    let hover_fill = mix_srgb(colors.fill, elevated, 0.84);
    let active_fill = mix_srgb(colors.fill, elevated, 0.72);
    let hover_border = mix_srgb(colors.border, text_primary, 0.78);

    // Contract §8 Divider: the spec's separator token (border-subtle, full
    // strength).
    let divider_color = ctx.theme().resolve_color(spec.separator_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());

    let is_unavailable = spec.is_unavailable();
    let label = spec.label.as_deref().unwrap_or("");

    // ── Root row ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
    }

    // ── Primary half ──
    let mut primary = Node::button("");
    // The caption is a child so layout places it beside the spinner; name
    // the button from the spec.
    primary.a11y.label = Some(spec.label.clone().unwrap_or_default());
    {
        let s = &mut primary.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.min_width = Some(rem_to_px(4.0)); // contract §7: min-width 4rem flat
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        // Left-rounded only.
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.bottom_left = radius;
        s.descriptor.background = Some(colors.fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = colors.border;
        s.descriptor.text_color = Some(colors.text);
        s.text_size = Some(font_size);
        s.text_weight = Some(500);
        s.line_height = Some(1.0);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = primary_gap;
    }
    primary.interaction.focusable = true;

    if !is_unavailable {
        // Primary-half hover shifts fill AND border; active shifts fill.
        primary.style.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: Some(hover_border),
            text_color: None,
            opacity: None,
        });
        primary.style.active = Some(StylePatch {
            background: Some(active_fill),
            border_color: None,
            text_color: None,
            opacity: None,
        });
        primary.style.descriptor.cursor = CursorHint::Pointer;

        if let Some(handler) = &handlers.on_click {
            let handler = Arc::clone(handler);
            primary.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    // Contract §4/§8: loading shows the spinner before the label. The old
    // tier paints `Spinner` ring/sm in the half's text color — a 12px
    // (0.75rem) glyph at every control size, rotating on an 0.8s loop.
    let mut primary = primary;
    if spec.is_loading {
        let mut spin = Node::icon("spinner", rem_to_px(0.75));
        spin.style.descriptor.text_color = Some(colors.text);
        spin.style.animation = crate::motion::loop_animation_for_policy(
            ctx.motion_policy(),
            NodeAnimation::spin("poodle-spinner-ring", 0.8),
            ctx.first_frame_committed(),
        );
        primary = primary.child(spin);
    }
    if !label.is_empty() {
        let mut caption = Node::text(label);
        caption.style.text_size = Some(font_size);
        caption.style.text_weight = Some(500);
        caption.style.descriptor.text_color = Some(colors.text);
        // The old tier's label wrapper: nowrap, allowed to shrink to zero.
        caption.style.no_wrap = true;
        caption.style.min_width = Some(0.0);
        primary = primary.child(caption);
    }

    let mut root = root.child(primary);

    // ── Divider (60% height, vertically centered by the row) ──
    let mut divider = Node::container();
    {
        let s = &mut divider.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(1.0);
        s.descriptor.layout.height = LayoutSizing::Fixed(divider_h);
        s.descriptor.background = Some(divider_color);
    }
    root = root.child(divider);

    // ── Toggle half (fixed per-size width, zero padding) ──
    let mut toggle = Node::button("");
    // Chevron-only: nothing in its subtree carries text.
    toggle.a11y.label = Some("More actions".to_string());
    {
        let s = &mut toggle.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.descriptor.layout.width = LayoutSizing::Fixed(toggle_w);
        // Right-rounded only.
        let c = &mut s.descriptor.corner_radii;
        c.top_right = radius;
        c.bottom_right = radius;
        s.descriptor.background = Some(colors.fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = colors.border;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    toggle.interaction.focusable = true;
    let mut chevron = Node::icon("chevron-down", chevron_size);
    chevron.style.descriptor.text_color = Some(colors.text);
    let mut toggle = toggle.child(chevron);

    if !is_unavailable {
        // The old tier wires only a fill hover on the toggle half — no
        // border shift, no active look.
        toggle.style.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: None,
            text_color: None,
            opacity: None,
        });
        toggle.style.descriptor.cursor = CursorHint::Pointer;

        if let Some(handler) = &handlers.on_dropdown {
            let handler = Arc::clone(handler);
            toggle.interaction.on_activate = Some(Arc::new(move || handler()));
        }
    }

    root = root.child(toggle);

    // ── Disabled / loading: dim the whole control, bar the cursor ──
    if is_unavailable {
        root.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
        root.style.descriptor.cursor = CursorHint::NotAllowed;
    }

    // ── Menu overlay (rendered when open) ──
    // Stacked below the row inside a column wrapper.
    if spec.is_open && !spec.items.is_empty() {
        let menu_fill = ctx.theme().resolve_color(spec.overlay_fill_token());
        let menu_border = ctx.theme().resolve_color("color.border.default");
        let menu_radius = ctx.theme().resolve_radius("radius.surface");
        let item_text = ctx.theme().resolve_color("color.text.primary");
        let accent = ctx.theme().resolve_color("color.accent.base");
        // The old tier's menu chrome: vertical padding and the top offset
        // both read space.inline.sm; items read space.inline.md
        // horizontally, space.control.y vertically.
        let menu_pad_y = ctx.theme().resolve_space("space.inline.sm");
        let item_pad_x = ctx.theme().resolve_space("space.inline.md");
        let item_pad_y = ctx.theme().resolve_space("space.control.y");
        // Items are full radius.control rounded and body-sized.
        let item_radius = ctx.theme().resolve_radius("radius.control");
        let item_font = ctx.theme().resolve_space("typography.body.size");
        // Item hover: accent at absolute 8% alpha.
        let item_hover = with_alpha(accent, 0.08);

        // Contract: the dropdown is a `menu` of `menuitem`s.
        let mut menu = Node::container();
        menu.a11y.role = Some(NodeRole::Menu);
        {
            let s = &mut menu.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.min_width = Some(rem_to_px(12.0)); // contract §8 menu min-width
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = menu_pad_y;
            pad.bottom = menu_pad_y;
            s.descriptor.layout.spacing.margin.top = menu_pad_y;
            // Token-accurate `elevation.overlay` (single layer, spread 0 —
            // the shared mapping both backends implement).
            s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
            s.descriptor.background = Some(menu_fill);
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = menu_border;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = menu_radius;
            c.top_right = menu_radius;
            c.bottom_right = menu_radius;
            c.bottom_left = menu_radius;
        }

        for item in &spec.items {
            match item {
                SplitMenuItem::Action {
                    value,
                    label,
                    is_disabled,
                } => {
                    let mut item_el = Node::button(label);
                    item_el.a11y.role = Some(NodeRole::MenuItem);
                    {
                        let s = &mut item_el.style;
                        let pad = &mut s.descriptor.layout.spacing.padding;
                        pad.left = item_pad_x;
                        pad.right = item_pad_x;
                        pad.top = item_pad_y;
                        pad.bottom = item_pad_y;
                        let c = &mut s.descriptor.corner_radii;
                        c.top_left = item_radius;
                        c.top_right = item_radius;
                        c.bottom_right = item_radius;
                        c.bottom_left = item_radius;
                        s.text_size = Some(item_font);
                        s.descriptor.text_color = Some(item_text);
                        s.descriptor.layout.direction = LayoutDirection::Row;
                        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    }
                    item_el.interaction.focusable = true;
                    if *is_disabled {
                        item_el.style.descriptor.opacity =
                            ctx.theme().resolve_opacity(spec.disabled_opacity_token());
                        item_el.interaction.disabled = true;
                    } else {
                        item_el.style.hover = Some(StylePatch {
                            background: Some(item_hover),
                            border_color: None,
                            text_color: None,
                            opacity: None,
                        });
                        item_el.style.descriptor.cursor = CursorHint::Pointer;

                        if let Some(handler) = &handlers.on_action {
                            let handler = Arc::clone(handler);
                            let value = value.clone();
                            item_el.interaction.on_activate =
                                Some(Arc::new(move || handler(&value)));
                        }
                    }
                    menu = menu.child(item_el);
                }
                SplitMenuItem::Separator => {
                    let mut sep = Node::container();
                    // Contract §6: menu separators are announced.
                    sep.a11y.role = Some(NodeRole::Splitter);
                    {
                        let s = &mut sep.style;
                        // Explicit Row (see switch.rs).
                        s.descriptor.layout.direction = LayoutDirection::Row;
                        s.fill_width = true;
                        s.descriptor.layout.height = LayoutSizing::Fixed(1.0);
                        // The old tier insets separators by space.inline.sm
                        // on every side.
                        let m = &mut s.descriptor.layout.spacing.margin;
                        m.top = menu_pad_y;
                        m.bottom = menu_pad_y;
                        m.left = menu_pad_y;
                        m.right = menu_pad_y;
                        s.descriptor.background = Some(divider_color);
                    }
                    menu = menu.child(sep);
                }
            }
        }

        // Contract `dismissOnOutsideInteract` (default `true`): a *refusal*
        // flag — native overlays dismiss on outside interact by default. The
        // refusal rides the surface's interaction as an inert activation: a
        // host implementing outside-dismissal must not dismiss a menu surface
        // carrying this marker (see menu.rs for the full contract note).
        if !spec.dismiss_on_outside_interact {
            menu.interaction.on_activate = Some(Arc::new(|| {}));
        }

        // Wrap the row + menu in a column so the menu stacks beneath.
        let mut wrapper = Node::container();
        wrapper.style.descriptor.layout.direction = LayoutDirection::Column;
        root = wrapper.child(root).child(menu);
    }

    // The visible label is the accessible name unless something overrides it.
    if let Some(label) = spec.aria_label.as_deref().or(spec.label.as_deref()) {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_specs::{ButtonTone, ControlSize};

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> SplitButtonSpec {
        SplitButtonSpec::new().with_label("Save").with_items(vec![
            SplitMenuItem::action("save-as", "Save As…"),
            SplitMenuItem::separator(),
            SplitMenuItem::action("export", "Export").with_disabled(true),
        ])
    }

    fn primary_half(node: &Node) -> &Node {
        node.find(&|n| {
            matches!(&n.kind, poodle_node::NodeKind::Button { .. })
                && n.a11y.label.as_deref() == Some("Save")
        })
        .expect("primary half")
    }

    fn toggle_half(node: &Node) -> &Node {
        node.find(&|n| {
            matches!(&n.kind, poodle_node::NodeKind::Button { .. })
                && n.a11y.label.as_deref() == Some("More actions")
        })
        .expect("toggle half")
    }

    #[test]
    fn half_metrics_follow_the_axis_faithful_recipe() {
        // height = size.control.height token (36px at base) + per-size offset
        let height_cases = [
            (ControlSize::Xs, 28.0),
            (ControlSize::Sm, 30.0),
            (ControlSize::Md, 36.0),
            (ControlSize::Lg, 42.0),
            (ControlSize::Xl, 44.0),
        ];
        // toggle width = the per-size table (1.75–2.5rem)
        let toggle_cases = [
            (ControlSize::Xs, 28.0),
            (ControlSize::Sm, 30.0),
            (ControlSize::Md, 32.0),
            (ControlSize::Lg, 36.0),
            (ControlSize::Xl, 40.0),
        ];
        for ((size, expected_h), (_, expected_w)) in height_cases.iter().zip(toggle_cases.iter()) {
            let spec = spec().with_size(*size);
            let theme = theme();
            let ctx = RenderContext::new(&theme);
            let node = split_button(&spec, &ctx, SplitButtonHandlers::default());
            match primary_half(&node).style.descriptor.layout.height {
                LayoutSizing::Fixed(h) => assert_eq!(h, *expected_h, "height for {size:?}"),
                ref other => panic!("expected fixed height, got {other:?}"),
            }
            let toggle = toggle_half(&node);
            match toggle.style.descriptor.layout.height {
                LayoutSizing::Fixed(h) => assert_eq!(h, *expected_h, "toggle height {size:?}"),
                ref other => panic!("expected fixed toggle height, got {other:?}"),
            }
            match toggle.style.descriptor.layout.width {
                LayoutSizing::Fixed(w) => assert_eq!(w, *expected_w, "toggle width {size:?}"),
                ref other => panic!("expected fixed toggle width, got {other:?}"),
            }
        }

        // pad_x = space.control.x token + per-size offset (0 at md).
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let base_pad = theme.resolve_space("space.control.x");
        let node = split_button(&spec(), &ctx, SplitButtonHandlers::default());
        let pad = &primary_half(&node).style.descriptor.layout.spacing.padding;
        assert_eq!(pad.left, base_pad);
        assert_eq!(pad.right, base_pad);
        // sm sits one offset stop down (−0.125rem).
        let node = split_button(
            &spec().with_size(ControlSize::Sm),
            &ctx,
            SplitButtonHandlers::default(),
        );
        let pad = &primary_half(&node).style.descriptor.layout.spacing.padding;
        assert_eq!(pad.left, base_pad - 2.0);
    }

    #[test]
    fn chevron_size_follows_the_per_size_table() {
        let cases = [
            (ControlSize::Xs, 10.0),
            (ControlSize::Sm, 11.0),
            (ControlSize::Md, 12.0),
            (ControlSize::Lg, 13.0),
            (ControlSize::Xl, 14.0),
        ];
        for (size, expected) in cases {
            let spec = spec().with_size(size);
            let theme = theme();
            let ctx = RenderContext::new(&theme);
            let node = split_button(&spec, &ctx, SplitButtonHandlers::default());
            let chevron = node
                .find(
                    &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "chevron-down"),
                )
                .expect("chevron icon");
            match &chevron.kind {
                poodle_node::NodeKind::Icon { size: px, .. } => {
                    assert_eq!(*px, expected, "chevron size for {size:?}")
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn variant_colors_match_the_old_gpui_tier() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let accent = theme.resolve_color("color.accent.base");
        let surface = theme.resolve_color("color.background.surface");
        let border_subtle = theme.resolve_color("color.border.subtle");
        let danger = theme.resolve_color("color.status.danger");
        let text_primary = theme.resolve_color("color.text.primary");
        let text_inverse = theme.resolve_color("color.text.inverse");

        // Primary default: accent fill, fill↔black 84% border, inverse text.
        let node = split_button(
            &spec().with_variant(ButtonVariant::Primary),
            &ctx,
            SplitButtonHandlers::default(),
        );
        let d = &primary_half(&node).style.descriptor;
        assert_eq!(d.background, Some(accent));
        assert_eq!(
            d.border.color,
            with_alpha(mix_srgb(accent, BLACK, 0.84), accent.3)
        );
        assert_eq!(d.text_color, Some(text_inverse));

        // Ghost: surface@42% / border-subtle@72% / primary text, any tone.
        let node = split_button(
            &spec()
                .with_variant(ButtonVariant::Ghost)
                .with_tone(ButtonTone::Danger),
            &ctx,
            SplitButtonHandlers::default(),
        );
        let d = &primary_half(&node).style.descriptor;
        assert_eq!(d.background, Some(with_alpha(surface, surface.3 * 0.42)));
        assert_eq!(
            d.border.color,
            with_alpha(border_subtle, border_subtle.3 * 0.72)
        );
        assert_eq!(d.text_color, Some(danger));

        // Secondary + danger tone: the tokens straight — surface fill,
        // status border, primary text (no status-tint mixes).
        let node = split_button(
            &spec().with_tone(ButtonTone::Danger),
            &ctx,
            SplitButtonHandlers::default(),
        );
        let d = &primary_half(&node).style.descriptor;
        assert_eq!(d.background, Some(surface));
        assert_eq!(d.border.color, danger);
        assert_eq!(d.text_color, Some(text_primary));

        // Legacy Danger variant: danger fill AND border, inverse text.
        let node = split_button(
            &spec().with_variant(ButtonVariant::Danger),
            &ctx,
            SplitButtonHandlers::default(),
        );
        let d = &primary_half(&node).style.descriptor;
        assert_eq!(d.background, Some(danger));
        assert_eq!(d.border.color, danger);
        assert_eq!(d.text_color, Some(text_inverse));
    }

    #[test]
    fn state_patches_match_the_old_tier() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let elevated = theme.resolve_color("color.background.elevated");
        let surface = theme.resolve_color("color.background.surface");
        let border_default = theme.resolve_color("color.border.default");
        let text_primary = theme.resolve_color("color.text.primary");

        let node = split_button(&spec(), &ctx, SplitButtonHandlers::default());
        let hover_fill = mix_srgb(surface, elevated, 0.84);
        let active_fill = mix_srgb(surface, elevated, 0.72);
        let hover_border = mix_srgb(border_default, text_primary, 0.78);

        // Primary half: hover shifts fill + border, active shifts fill.
        let primary = primary_half(&node);
        let hover = primary.style.hover.expect("primary hover patch");
        assert_eq!(hover.background, Some(hover_fill));
        assert_eq!(hover.border_color, Some(hover_border));
        let active = primary.style.active.expect("primary active patch");
        assert_eq!(active.background, Some(active_fill));

        // Toggle half: fill-only hover, no active look.
        let toggle = toggle_half(&node);
        let hover = toggle.style.hover.expect("toggle hover patch");
        assert_eq!(hover.background, Some(hover_fill));
        assert_eq!(hover.border_color, None);
        assert!(toggle.style.active.is_none(), "toggle has no active patch");
    }

    #[test]
    fn divider_reads_the_separator_token_at_full_strength() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let subtle = theme.resolve_color("color.border.subtle");
        let node = split_button(&spec(), &ctx, SplitButtonHandlers::default());
        let divider = node
            .find(
                &|n| matches!(n.style.descriptor.layout.width, LayoutSizing::Fixed(w) if w == 1.0),
            )
            .expect("divider");
        assert_eq!(divider.style.descriptor.background, Some(subtle));
        // 60% of the 36px md height.
        match divider.style.descriptor.layout.height {
            LayoutSizing::Fixed(h) => assert_eq!(h, 36.0 * 0.6),
            ref other => panic!("expected fixed divider height, got {other:?}"),
        }
    }

    #[test]
    fn handlers_route_to_their_halves_and_items() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        let sink = Arc::clone(&seen);
        let drop_sink = Arc::clone(&seen);
        let action_sink = Arc::clone(&seen);
        let handlers = SplitButtonHandlers {
            on_click: Some(Arc::new(move || sink.lock().unwrap().push("click".into()))),
            on_dropdown: Some(Arc::new(move || {
                drop_sink.lock().unwrap().push("dropdown".into())
            })),
            on_action: Some(Arc::new(move |v: &str| {
                action_sink.lock().unwrap().push(v.into())
            })),
        };
        let spec = spec().with_open(true);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = split_button(&spec, &ctx, handlers);

        (primary_half(&node)
            .interaction
            .on_activate
            .as_ref()
            .unwrap())();
        (toggle_half(&node).interaction.on_activate.as_ref().unwrap())();

        let item = node
            .find(&|n| {
                n.a11y.role == Some(NodeRole::MenuItem) && n.interaction.on_activate.is_some()
            })
            .expect("enabled menu item is activatable");
        (item.interaction.on_activate.as_ref().unwrap())();

        assert_eq!(
            seen.lock().unwrap().as_slice(),
            ["click", "dropdown", "save-as"]
        );
    }

    #[test]
    fn unavailable_control_dims_and_drops_activation() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let handlers = SplitButtonHandlers {
            on_click: Some(Arc::new(|| {})),
            on_dropdown: Some(Arc::new(|| {})),
            ..Default::default()
        };
        let spec = SplitButtonSpec {
            is_disabled: true,
            ..spec()
        };
        let node = split_button(&spec, &ctx, handlers);
        let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
        assert_eq!(node.style.descriptor.opacity, disabled_opacity);
        assert_eq!(node.style.descriptor.cursor, CursorHint::NotAllowed);
        assert!(primary_half(&node).interaction.on_activate.is_none());
        assert!(toggle_half(&node).interaction.on_activate.is_none());
        assert!(primary_half(&node).style.hover.is_none());
    }

    #[test]
    fn loading_shows_the_old_tiers_spinner_glyph() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let text_primary = theme.resolve_color("color.text.primary");
        let spec = SplitButtonSpec {
            is_loading: true,
            ..spec()
        };
        let node = split_button(&spec, &ctx, SplitButtonHandlers::default());
        let spinner = node
            .find(
                &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "spinner"),
            )
            .expect("loading spinner glyph");
        // SpinnerSize::Sm: 12px at every control size, half's text color,
        // 0.8s spin.
        match &spinner.kind {
            poodle_node::NodeKind::Icon { size, .. } => assert_eq!(*size, 12.0),
            _ => unreachable!(),
        }
        assert_eq!(spinner.style.descriptor.text_color, Some(text_primary));
        assert!(
            spinner.style.animation.is_none(),
            "loading loops wait for the first committed frame"
        );
        let after = split_button(
            &spec,
            &ctx.with_first_frame_committed(true),
            SplitButtonHandlers::default(),
        );
        assert!(after
            .find(
                &|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "spinner"),
            )
            .expect("loading spinner glyph")
            .style
            .animation
            .is_some());
        // Loading is unavailable: dimmed, no activation.
        assert!(primary_half(&node).interaction.on_activate.is_none());
    }

    #[test]
    fn open_menu_matches_the_old_tiers_anatomy() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let gap_sm = theme.resolve_space("space.inline.sm");
        let gap_md = theme.resolve_space("space.inline.md");
        let control_y = theme.resolve_space("space.control.y");
        let body_size = theme.resolve_space("typography.body.size");
        let radius_control = theme.resolve_radius("radius.control");
        let accent = theme.resolve_color("color.accent.base");
        let subtle = theme.resolve_color("color.border.subtle");
        let elevated = theme.resolve_color("color.background.elevated");
        let border_default = theme.resolve_color("color.border.default");
        let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");

        let spec = spec().with_open(true);
        let node = split_button(&spec, &ctx, SplitButtonHandlers::default());
        let menu = node
            .find(&|n| n.a11y.role == Some(NodeRole::Menu))
            .expect("menu panel");

        // Panel chrome: elevated fill, default border, overlay elevation,
        // 12rem floor, space.inline.sm vertical padding + top offset.
        assert_eq!(menu.style.descriptor.background, Some(elevated));
        assert_eq!(menu.style.descriptor.border.color, border_default);
        assert_eq!(
            menu.style.descriptor.shadow,
            Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY)
        );
        assert_eq!(menu.style.min_width, Some(192.0));
        let pad = &menu.style.descriptor.layout.spacing.padding;
        assert_eq!(pad.top, gap_sm);
        assert_eq!(pad.bottom, gap_sm);
        assert_eq!(menu.style.descriptor.layout.spacing.margin.top, gap_sm);

        // Enabled item: inline.md / control.y padding, body font, full
        // control radius, accent@8% hover, no min-height floor.
        let item = node
            .find(&|n| n.a11y.role == Some(NodeRole::MenuItem) && !n.interaction.disabled)
            .expect("enabled item");
        let pad = &item.style.descriptor.layout.spacing.padding;
        assert_eq!(pad.left, gap_md);
        assert_eq!(pad.right, gap_md);
        assert_eq!(pad.top, control_y);
        assert_eq!(pad.bottom, control_y);
        assert_eq!(item.style.text_size, Some(body_size));
        assert_eq!(item.style.descriptor.corner_radii.top_left, radius_control);
        assert_eq!(item.style.min_height, None);
        assert_eq!(
            item.style.hover.expect("item hover").background,
            Some(with_alpha(accent, 0.08))
        );

        // Disabled item: dimmed, inert, no hover.
        let disabled = node
            .find(&|n| n.a11y.role == Some(NodeRole::MenuItem) && n.interaction.disabled)
            .expect("disabled item");
        assert_eq!(disabled.style.descriptor.opacity, disabled_opacity);
        assert!(disabled.interaction.on_activate.is_none());
        assert!(disabled.style.hover.is_none());

        // Separator: 1px, space.inline.sm margins on every side, separator
        // token at full strength.
        let sep = node
            .find(&|n| n.a11y.role == Some(NodeRole::Splitter))
            .expect("menu separator");
        match sep.style.descriptor.layout.height {
            LayoutSizing::Fixed(h) => assert_eq!(h, 1.0),
            ref other => panic!("expected fixed separator height, got {other:?}"),
        }
        let m = &sep.style.descriptor.layout.spacing.margin;
        assert_eq!(
            (m.top, m.bottom, m.left, m.right),
            (gap_sm, gap_sm, gap_sm, gap_sm)
        );
        assert_eq!(sep.style.descriptor.background, Some(subtle));
    }

    #[test]
    fn outside_interact_refusal_marks_the_open_menu() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        // Web default `true` + open: the menu surface carries no marker.
        let node = split_button(
            &spec().with_open(true),
            &ctx,
            SplitButtonHandlers::default(),
        );
        let menu_node = node
            .find(&|n| n.a11y.role == Some(NodeRole::Menu))
            .expect("open menu");
        assert!(menu_node.interaction.on_activate.is_none());

        // Refusal: the open menu surface carries the inert activation marker
        // a host keys outside-dismissal on.
        let refusing = spec()
            .with_open(true)
            .with_dismiss_on_outside_interact(false);
        let node = split_button(&refusing, &ctx, SplitButtonHandlers::default());
        let menu_node = node
            .find(&|n| n.a11y.role == Some(NodeRole::Menu))
            .expect("open menu");
        assert!(menu_node.interaction.on_activate.is_some());
    }
}
