//! IconButton — a button whose whole label is its glyph.
//!
//! Contract: `docs/contracts/components/icon-button.md`
//! Ported from: `packages/jetstream/components/src/icon_button.rs`, with the
//! metrics and state recipes re-transcribed from the axis-faithful old GPUI
//! tier (`packages/gpui/components/src/primitives/icon_button.rs`) in g12.019:
//! the square is the axis-layered `size.control.height` token plus the
//! per-size offset, the glyph is a token-resolved `IconSize` stop, and the
//! variant×tone fill/border/hover/active/pressed recipes are the button
//! family's — not the old fixed per-size tables.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeAnimation, NodeRole, NodeToggled, StylePatch,
};
use poodle_specs::{ButtonTone, ButtonVariant, IconButtonSpec, IconSize};

use crate::color::{mix_srgb, BLACK, TRANSPARENT, WHITE};
use crate::context::RenderContext;
use crate::presentation::{rem_to_px, resolve_supporting_visual_size, size_height_offset_rem};

/// Loading-spinner diameter in px. The old GPUI tier renders a ring spinner
/// at `SpinnerSize::Sm` (12px, fixed — it does not track the control size);
/// the node vocabulary has no spinner kind, so the old tier's `spinner.svg`
/// asset is carried as an animated icon node.
const LOADING_SPINNER_PX: f32 = 12.0;

/// Command helper used by composites. Pressed-change reporting lives on
/// [`icon_button_with_handlers`].
pub fn icon_button(
    spec: &IconButtonSpec,
    ctx: &RenderContext<'_>,
    on_click: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    icon_button_with_handlers(
        spec,
        ctx,
        IconButtonHandlers {
            on_click,
            on_pressed_change: None,
        },
    )
}

/// Host callbacks. Toggle activation reports the inverse pressed value first,
/// then invokes the command callback. Command-only activation never
/// manufactures a pressed-change event.
#[derive(Default)]
pub struct IconButtonHandlers {
    pub on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_pressed_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

/// Build an icon-button node with command and optional pressed-change handlers.
pub fn icon_button_with_handlers(
    spec: &IconButtonSpec,
    ctx: &RenderContext<'_>,
    handlers: IconButtonHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let tone = spec.tone;

    // Axis-faithful square (g12.019 recipe correction): the axis-layered
    // token plus the per-size offset — the old GPUI tier's form — not the
    // fixed md height plus an icon-button delta (`control_height_rem(Md)` /
    // `icon_button_size_delta_rem`), which ignore the theme's
    // density/control-size layering. At base tokens (the Jetstream provider,
    // no axes) md/default reproduces the old fixed 36px square.
    let size_px = ctx
        .theme()
        .resolve_space(spec.control_height_token(ctx.base_size(spec.size)))
        + rem_to_px(size_height_offset_rem(effective_size));

    // Glyph (contract §13): the old tier's
    // `IconSize::from(resolve_supporting_visual_size(..))` — one stop smaller
    // than the control, resolved through the `size.icon.*` tokens (the
    // IconSize ladder only has sm/md/lg stops) — not the per-size font ladder.
    let icon_size = ctx
        .theme()
        .resolve_space(IconSize::from(resolve_supporting_visual_size(effective_size)).size_token());

    let radius = ctx.theme().resolve_radius("radius.control");
    let is_pressed = spec.current_pressed();
    let is_toggle = spec.is_toggle_mode();
    let is_unavailable = spec.is_disabled || spec.is_loading;

    let elevated = ctx.theme().resolve_color("color.background.elevated");
    let accent = ctx.theme().resolve_color("color.accent.base");
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_inverse = ctx.theme().resolve_color("color.text.inverse");

    // ── Variant × tone (contract §8, the button-family recipes) ──
    let base_fill = ctx.theme().resolve_color(spec.variant.fill_token(tone));
    let base_border = ctx.theme().resolve_color(spec.variant.border_token(tone));
    let text_color = ctx.theme().resolve_color(spec.variant.text_token(tone));

    let (fill, border) = match spec.variant {
        // Primary: the fill carries the tone; the border is that fill
        // darkened toward black.
        ButtonVariant::Primary => (base_fill, mix_srgb(base_fill, BLACK, 0.84)),
        ButtonVariant::Ghost => (TRANSPARENT, TRANSPARENT),
        // Toned secondary: color-mix(status 16%, surface) fill and
        // color-mix(status 46%, border-default) border (icon-button.md §8
        // Tone: danger / success / warning). Default secondary uses the
        // token values.
        ButtonVariant::Secondary => match tone {
            ButtonTone::Danger | ButtonTone::Success | ButtonTone::Warning => {
                let status_token = match tone {
                    ButtonTone::Success => "color.status.success",
                    ButtonTone::Warning => "color.status.warning",
                    _ => "color.status.danger",
                };
                let status = ctx.theme().resolve_color(status_token);
                let surface = ctx.theme().resolve_color("color.background.surface");
                let border_default = ctx.theme().resolve_color("color.border.default");
                (
                    mix_srgb(status, surface, 0.16),
                    mix_srgb(status, border_default, 0.46),
                )
            }
            ButtonTone::Default => (base_fill, base_border),
        },
        // Danger (and any other solid variant): the raw tokens — the border
        // is NOT darkened; that recipe belongs to primary alone.
        _ => (base_fill, base_border),
    };

    // ── Pressed (contract §8 "Root — Pressed") ──
    // Non-primary variants get a solid-accent treatment: fill accent-base,
    // border accent-base 85% black, inverse text. Primary keeps its own
    // variant styling when pressed.
    let pressed_active = is_pressed && !matches!(spec.variant, ButtonVariant::Primary);
    let (fill, border, text_color) = if pressed_active {
        (accent, mix_srgb(accent, BLACK, 0.85), text_inverse)
    } else {
        (fill, border, text_color)
    };

    // Hover = color-mix(fill 76%, elevated), active = color-mix(fill 64%,
    // elevated); a pressed hover is color-mix(white 12%, accent-base).
    // Hover border = border 74% toward text-primary.
    let hover_fill = if pressed_active {
        mix_srgb(WHITE, accent, 0.12)
    } else {
        mix_srgb(fill, elevated, 0.76)
    };
    let hover_border = mix_srgb(border, text_primary, 0.74);
    let active_fill = mix_srgb(fill, elevated, 0.64);

    let icon_name = spec.icon.as_deref().unwrap_or("");

    let mut el = Node::button("");
    {
        let s = &mut el.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(size_px);
        s.descriptor.layout.width = LayoutSizing::Fixed(size_px);
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.background = Some(fill);
        s.descriptor.text_color = Some(text_color);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    el.interaction.focusable = !is_unavailable;

    // Contract §8: the shadow is `none` in every state — no inset highlight,
    // no drop. The old fixed-table port carried an inset top highlight the
    // old GPUI tier never painted.

    // Glyph or loading spinner — mutually exclusive.
    if spec.is_loading {
        let mut spinner = Node::icon("spinner", LOADING_SPINNER_PX);
        spinner.style.descriptor.text_color = Some(text_color);
        spinner.style.animation = Some(NodeAnimation::spin("poodle-spinner-ring", 0.8));
        el = el.child(spinner);
    } else if !icon_name.is_empty() {
        let mut glyph = Node::icon(icon_name, icon_size);
        glyph.style.descriptor.text_color = Some(text_color);
        el = el.child(glyph);
    }

    if is_unavailable {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
        el.style.descriptor.cursor = CursorHint::NotAllowed;
        el.interaction.disabled = true;
    } else {
        el.style.descriptor.cursor = CursorHint::Pointer;
        el.a11y.tab_index = Some(0);
        el.style.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: Some(hover_border),
            text_color: None,
            opacity: None,
        });
        el.style.active = Some(StylePatch {
            background: Some(active_fill),
            border_color: None,
            text_color: None,
            opacity: None,
        });
        el.style.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color("color.accent.focusRing"),
            width: ctx.theme().resolve_border_width("border.width.focus"),
            offset: rem_to_px(0.125),
        });
        let on_click = handlers.on_click;
        let on_pressed_change = handlers.on_pressed_change;
        if on_click.is_some() || (is_toggle && on_pressed_change.is_some()) {
            let next_pressed = !is_pressed;
            el.interaction.on_activate = Some(Arc::new(move || {
                if is_toggle {
                    if let Some(ref handler) = on_pressed_change {
                        handler(next_pressed);
                    }
                }
                if let Some(ref handler) = on_click {
                    handler();
                }
            }));
        }
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::Button);
    if is_toggle {
        el.a11y.toggled = Some(if is_pressed {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
    }
    el.a11y.expanded = spec.is_expanded;
    el.a11y.controls = spec.controls.clone();
    el.tooltip = projected_tooltip(spec);
    el
}

fn projected_tooltip(spec: &IconButtonSpec) -> Option<String> {
    match spec.tooltip.as_deref() {
        Some(text) => non_empty_text(text),
        None => spec.aria_label.as_deref().and_then(non_empty_text),
    }
}

fn non_empty_text(text: &str) -> Option<String> {
    if text.trim().is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::ColorValue;
    use poodle_node::NodeKind;
    use poodle_specs::ControlSize;

    /// The real token resolver over the ECLIPSE theme. Pure — no backend.
    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn resolve_color(theme: &poodle_jetstream::JetstreamThemeProvider, token: &str) -> ColorValue {
        poodle_adapter::ThemeProvider::resolve_color(theme, token)
    }

    fn icon_child(node: &Node) -> Option<&Node> {
        node.find(&|n| matches!(&n.kind, NodeKind::Icon { .. }))
    }

    #[test]
    fn square_follows_the_axis_faithful_recipe() {
        // size = size.control.height token (36px at base) + per-size offset.
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let cases = [
            (ControlSize::Xs, 28.0),
            (ControlSize::Sm, 30.0),
            (ControlSize::Md, 36.0),
            (ControlSize::Lg, 42.0),
            (ControlSize::Xl, 44.0),
        ];
        for (size, expected) in cases {
            let spec = IconButtonSpec::new().with_icon("plus").with_size(size);
            let node = icon_button(&spec, &ctx, None);
            match (
                node.style.descriptor.layout.width,
                node.style.descriptor.layout.height,
            ) {
                (LayoutSizing::Fixed(w), LayoutSizing::Fixed(h)) => {
                    assert_eq!(w, expected, "width for {size:?}");
                    assert_eq!(h, expected, "height for {size:?}");
                }
                ref other => panic!("expected fixed square, got {other:?}"),
            }
        }
    }

    #[test]
    fn glyph_size_resolves_through_the_icon_token_ladder() {
        // Supporting visuals are one control stop smaller; IconSize maps 1:1
        // from that stop through the five icon tokens (no endpoint collapse).
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let cases = [
            (ControlSize::Xs, "size.icon.xs"),
            (ControlSize::Sm, "size.icon.sm"),
            (ControlSize::Md, "size.icon.sm"),
            (ControlSize::Lg, "size.icon.md"),
            (ControlSize::Xl, "size.icon.lg"),
        ];
        for (size, token) in cases {
            let expected = poodle_adapter::ThemeProvider::resolve_space(&theme, token);
            let spec = IconButtonSpec::new().with_icon("plus").with_size(size);
            let node = icon_button(&spec, &ctx, None);
            let glyph = icon_child(&node).expect("glyph for a named icon");
            match &glyph.kind {
                NodeKind::Icon { name, size } => {
                    assert_eq!(name, "plus");
                    assert_eq!(*size, expected, "glyph size for {token}");
                }
                _ => panic!("expected icon glyph"),
            }
        }
    }

    #[test]
    fn ghost_default_is_fully_transparent_with_primary_text() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = icon_button(&IconButtonSpec::new().with_icon("plus"), &ctx, None);
        assert_eq!(node.style.descriptor.background, Some(TRANSPARENT));
        assert_eq!(node.style.descriptor.border.color, TRANSPARENT);
        assert_eq!(
            node.style.descriptor.text_color,
            Some(resolve_color(&theme, "color.text.primary"))
        );
    }

    #[test]
    fn ghost_toned_paints_the_tone_on_the_glyph_only() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_tone(ButtonTone::Success);
        let node = icon_button(&spec, &ctx, None);
        assert_eq!(node.style.descriptor.background, Some(TRANSPARENT));
        assert_eq!(node.style.descriptor.border.color, TRANSPARENT);
        assert_eq!(
            node.style.descriptor.text_color,
            Some(resolve_color(&theme, "color.status.success"))
        );
    }

    #[test]
    fn primary_darkens_its_own_fill_for_the_border() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let accent = resolve_color(&theme, "color.accent.base");
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_variant(ButtonVariant::Primary);
        let node = icon_button(&spec, &ctx, None);
        assert_eq!(node.style.descriptor.background, Some(accent));
        assert_eq!(
            node.style.descriptor.border.color,
            mix_srgb(accent, BLACK, 0.84)
        );
        assert_eq!(
            node.style.descriptor.text_color,
            Some(resolve_color(&theme, "color.text.inverse"))
        );
    }

    #[test]
    fn primary_toned_fill_is_the_status_color_with_a_darkened_border() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let danger = resolve_color(&theme, "color.status.danger");
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_variant(ButtonVariant::Primary)
            .with_tone(ButtonTone::Danger);
        let node = icon_button(&spec, &ctx, None);
        assert_eq!(node.style.descriptor.background, Some(danger));
        assert_eq!(
            node.style.descriptor.border.color,
            mix_srgb(danger, BLACK, 0.84)
        );
    }

    #[test]
    fn danger_variant_uses_the_raw_status_tokens() {
        // Unlike primary, the danger variant's border is the status token
        // itself — the old GPUI tier darkens primary borders only.
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let danger = resolve_color(&theme, "color.status.danger");
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_variant(ButtonVariant::Danger);
        let node = icon_button(&spec, &ctx, None);
        assert_eq!(node.style.descriptor.background, Some(danger));
        assert_eq!(node.style.descriptor.border.color, danger);
        assert_eq!(
            node.style.descriptor.text_color,
            Some(resolve_color(&theme, "color.text.inverse"))
        );
    }

    #[test]
    fn secondary_default_uses_the_token_values() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_variant(ButtonVariant::Secondary);
        let node = icon_button(&spec, &ctx, None);
        assert_eq!(
            node.style.descriptor.background,
            Some(resolve_color(&theme, "color.background.surface"))
        );
        assert_eq!(
            node.style.descriptor.border.color,
            resolve_color(&theme, "color.border.default")
        );
        assert_eq!(
            node.style.descriptor.text_color,
            Some(resolve_color(&theme, "color.text.primary"))
        );
    }

    #[test]
    fn secondary_toned_mixes_status_into_surface_and_border() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let danger = resolve_color(&theme, "color.status.danger");
        let surface = resolve_color(&theme, "color.background.surface");
        let border_default = resolve_color(&theme, "color.border.default");
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_variant(ButtonVariant::Secondary)
            .with_tone(ButtonTone::Danger);
        let node = icon_button(&spec, &ctx, None);
        assert_eq!(
            node.style.descriptor.background,
            Some(mix_srgb(danger, surface, 0.16))
        );
        assert_eq!(
            node.style.descriptor.border.color,
            mix_srgb(danger, border_default, 0.46)
        );
        assert_eq!(
            node.style.descriptor.text_color,
            Some(resolve_color(&theme, "color.text.primary"))
        );
    }

    #[test]
    fn hover_and_active_mix_the_fill_toward_elevated() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let surface = resolve_color(&theme, "color.background.surface");
        let elevated = resolve_color(&theme, "color.background.elevated");
        let border_default = resolve_color(&theme, "color.border.default");
        let text_primary = resolve_color(&theme, "color.text.primary");
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_variant(ButtonVariant::Secondary);
        let node = icon_button(&spec, &ctx, None);
        let hover = node.style.hover.expect("hover patch when enabled");
        assert_eq!(hover.background, Some(mix_srgb(surface, elevated, 0.76)));
        assert_eq!(
            hover.border_color,
            Some(mix_srgb(border_default, text_primary, 0.74))
        );
        let active = node.style.active.expect("active patch when enabled");
        assert_eq!(active.background, Some(mix_srgb(surface, elevated, 0.64)));
    }

    #[test]
    fn pressed_non_primary_gets_the_solid_accent_treatment() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let accent = resolve_color(&theme, "color.accent.base");
        let text_primary = resolve_color(&theme, "color.text.primary");
        let elevated = resolve_color(&theme, "color.background.elevated");
        let pressed_border = mix_srgb(accent, BLACK, 0.85);
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_variant(ButtonVariant::Ghost)
            .with_pressed(true);
        let node = icon_button(&spec, &ctx, None);
        assert_eq!(node.style.descriptor.background, Some(accent));
        assert_eq!(node.style.descriptor.border.color, pressed_border);
        assert_eq!(
            node.style.descriptor.text_color,
            Some(resolve_color(&theme, "color.text.inverse"))
        );
        // Pressed hover is white 12% over accent, not the elevated mix.
        let hover = node.style.hover.expect("hover patch when pressed");
        assert_eq!(hover.background, Some(mix_srgb(WHITE, accent, 0.12)));
        assert_eq!(
            hover.border_color,
            Some(mix_srgb(pressed_border, text_primary, 0.74))
        );
        let active = node.style.active.expect("active patch when pressed");
        assert_eq!(active.background, Some(mix_srgb(accent, elevated, 0.64)));
    }

    #[test]
    fn pressed_primary_keeps_its_variant_styling() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let accent = resolve_color(&theme, "color.accent.base");
        let elevated = resolve_color(&theme, "color.background.elevated");
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_variant(ButtonVariant::Primary)
            .with_pressed(true);
        let node = icon_button(&spec, &ctx, None);
        assert_eq!(node.style.descriptor.background, Some(accent));
        assert_eq!(
            node.style.descriptor.border.color,
            mix_srgb(accent, BLACK, 0.84)
        );
        let hover = node.style.hover.expect("hover patch when pressed primary");
        assert_eq!(hover.background, Some(mix_srgb(accent, elevated, 0.76)));
    }

    #[test]
    fn no_shadow_in_any_state() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        for spec in [
            IconButtonSpec::new().with_icon("plus"),
            IconButtonSpec::new()
                .with_icon("plus")
                .with_variant(ButtonVariant::Primary),
            IconButtonSpec::new()
                .with_icon("plus")
                .with_variant(ButtonVariant::Secondary)
                .with_pressed(true),
        ] {
            let node = icon_button(&spec, &ctx, None);
            assert!(
                node.style.shadow_layers.is_empty(),
                "contract §8: shadow is none in every state"
            );
        }
    }

    #[test]
    fn disabled_is_dimmed_not_allowed_and_inert() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = IconButtonSpec::new().with_icon("plus").with_disabled(true);
        let node = icon_button(
            &spec,
            &ctx,
            Some(Arc::new(|| panic!("disabled buttons do not fire"))),
        );
        assert!(node.interaction.disabled);
        assert!(!node.interaction.focusable);
        assert_eq!(node.a11y.tab_index, None);
        assert!(node.style.focus_ring.is_none());
        assert!(node.interaction.on_activate.is_none());
        assert_eq!(
            node.style.descriptor.opacity,
            poodle_adapter::ThemeProvider::resolve_opacity(&theme, "state.opacity.disabled")
        );
        assert_eq!(node.style.descriptor.cursor, CursorHint::NotAllowed);
        assert!(node.style.hover.is_none() && node.style.active.is_none());
    }

    #[test]
    fn loading_swaps_the_glyph_for_the_fixed_size_ring_spinner() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = IconButtonSpec::new()
            .with_icon("plus")
            .with_size(ControlSize::Xl)
            .with_loading(true);
        let node = icon_button(&spec, &ctx, None);
        assert!(node.interaction.disabled);
        assert!(!node.interaction.focusable);
        assert_eq!(node.a11y.tab_index, None);
        let glyph = icon_child(&node).expect("the spinner stands in for the glyph");
        match &glyph.kind {
            NodeKind::Icon { name, size } => {
                assert_eq!(name, "spinner");
                // SpinnerSize::Sm — fixed, not tracking the control size.
                assert_eq!(*size, 12.0);
            }
            _ => panic!("expected spinner icon"),
        }
        assert!(glyph.style.animation.is_some());
    }

    #[test]
    fn an_unset_icon_renders_no_glyph() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = icon_button(&IconButtonSpec::new(), &ctx, None);
        assert!(icon_child(&node).is_none(), "no icon, no glyph child");
    }

    #[test]
    fn focus_and_disclosure_semantics_reach_the_node() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = icon_button(
            &IconButtonSpec::new()
                .with_icon("chevron-down")
                .with_expanded(true)
                .with_controls("details"),
            &ctx,
            None,
        );
        assert_eq!(node.a11y.role, Some(NodeRole::Button));
        assert_eq!(node.a11y.tab_index, Some(0));
        assert!(
            node.style.focus_ring.is_some(),
            "keyboard focus has a structured ring"
        );
        assert_eq!(node.a11y.expanded, Some(true));
        assert_eq!(node.a11y.controls.as_deref(), Some("details"));
        assert!(node.a11y.toggled.is_none());
    }

    #[test]
    fn activation_fires_the_handler() {
        use std::sync::Mutex;
        let fired: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let sink = Arc::clone(&fired);
        let spec = IconButtonSpec::new().with_icon("plus");
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = icon_button(
            &spec,
            &ctx,
            Some(Arc::new(move || *sink.lock().unwrap() += 1)),
        );
        let activate = node.interaction.on_activate.expect("activatable");
        activate();
        assert_eq!(*fired.lock().unwrap(), 1);
    }

    #[test]
    fn command_only_activation_never_emits_pressed_change() {
        use std::sync::Mutex;
        let clicks: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let pressed: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(Vec::new()));
        let click_sink = Arc::clone(&clicks);
        let pressed_sink = Arc::clone(&pressed);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = icon_button_with_handlers(
            &IconButtonSpec::new()
                .with_icon("plus")
                .with_aria_label("Add"),
            &ctx,
            IconButtonHandlers {
                on_click: Some(Arc::new(move || *click_sink.lock().unwrap() += 1)),
                on_pressed_change: Some(Arc::new(move |next| {
                    pressed_sink.lock().unwrap().push(next);
                })),
            },
        );
        node.interaction.on_activate.expect("activatable")();
        assert_eq!(*clicks.lock().unwrap(), 1);
        assert!(pressed.lock().unwrap().is_empty());
        assert!(node.a11y.toggled.is_none());
    }

    #[test]
    fn toggle_activation_reports_the_inverse_before_the_command() {
        use std::sync::Mutex;
        let order: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let pressed_order = Arc::clone(&order);
        let click_order = Arc::clone(&order);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = icon_button_with_handlers(
            &IconButtonSpec::new()
                .with_icon("bold")
                .with_aria_label("Bold")
                .with_pressed(false),
            &ctx,
            IconButtonHandlers {
                on_click: Some(Arc::new(move || {
                    click_order.lock().unwrap().push("click".into());
                })),
                on_pressed_change: Some(Arc::new(move |next| {
                    pressed_order
                        .lock()
                        .unwrap()
                        .push(format!("pressed:{next}"));
                })),
            },
        );
        assert_eq!(node.a11y.toggled, Some(NodeToggled::False));
        node.interaction.on_activate.expect("activatable")();
        assert_eq!(
            *order.lock().unwrap(),
            ["pressed:true".to_string(), "click".to_string()]
        );
    }

    #[test]
    fn default_pressed_seeds_toggled_state_and_first_activation_reports_false() {
        use std::sync::Mutex;
        let next: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(None));
        let sink = Arc::clone(&next);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = IconButtonSpec::new()
            .with_icon("pin")
            .with_aria_label("Pin")
            .with_default_pressed(true);
        let node = icon_button_with_handlers(
            &spec,
            &ctx,
            IconButtonHandlers {
                on_click: None,
                on_pressed_change: Some(Arc::new(move |value| {
                    *sink.lock().unwrap() = Some(value);
                })),
            },
        );
        assert_eq!(node.a11y.toggled, Some(NodeToggled::True));
        assert_eq!(
            node.style.descriptor.background,
            Some(resolve_color(&theme, "color.accent.base"))
        );
        node.interaction.on_activate.expect("activatable")();
        assert_eq!(*next.lock().unwrap(), Some(false));
    }

    #[test]
    fn explicit_tooltip_wins_and_empty_text_is_omitted() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let explicit = icon_button(
            &IconButtonSpec::new()
                .with_icon("plus")
                .with_aria_label("Add")
                .with_tooltip("Add item"),
            &ctx,
            None,
        );
        assert_eq!(explicit.tooltip.as_deref(), Some("Add item"));

        let fallback = icon_button(
            &IconButtonSpec::new()
                .with_icon("plus")
                .with_aria_label("Add"),
            &ctx,
            None,
        );
        assert_eq!(fallback.tooltip.as_deref(), Some("Add"));

        let mut empty = IconButtonSpec::new()
            .with_icon("plus")
            .with_aria_label("Add");
        empty.tooltip = Some(String::new());
        let omitted = icon_button(&empty, &ctx, None);
        assert!(omitted.tooltip.is_none());
    }

    #[test]
    fn unavailable_targets_emit_nothing() {
        use std::sync::Mutex;
        let fired: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        for spec in [
            IconButtonSpec::new()
                .with_icon("ban")
                .with_aria_label("Block")
                .with_disabled(true)
                .with_pressed(false),
            IconButtonSpec::new()
                .with_icon("loader")
                .with_aria_label("Loading")
                .with_loading(true)
                .with_default_pressed(true),
        ] {
            let click_sink = Arc::clone(&fired);
            let pressed_sink = Arc::clone(&fired);
            let node = icon_button_with_handlers(
                &spec,
                &ctx,
                IconButtonHandlers {
                    on_click: Some(Arc::new(move || *click_sink.lock().unwrap() += 1)),
                    on_pressed_change: Some(Arc::new(move |_| {
                        *pressed_sink.lock().unwrap() += 1;
                    })),
                },
            );
            assert!(node.interaction.on_activate.is_none());
            assert!(!node.interaction.focusable);
            assert_eq!(node.a11y.tab_index, None);
        }
        assert_eq!(*fired.lock().unwrap(), 0);
    }
}
