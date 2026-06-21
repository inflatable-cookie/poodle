//! IconButton — Jetstream icon-only button backed by IconButtonSpec.
//!
//! Contract: `docs/contracts/components/icon-button.md`
//! Reference: `packages/svelte/components/src/IconButton.svelte`
//!
//! Resolves variant (ghost/primary/secondary), tone (default/danger/success),
//! pressed, loading, disabled, focus, hover, and the per-size square from tokens.
//! No hardcoded hsla; the only literals are contract-exact rem deltas fed through
//! `rem_to_px` (allowed per CLAUDE.md).
//!
//! ## Accepted limits (probe-verified only)
//! - **No ARIA channel**: `aria-label`/`aria-pressed`/`aria-busy` are carried on
//!   the spec but the `JsEl` builder has no accessibility metadata sink.
//! - **No tooltip**: the contract tooltip surface is host/overlay-driven; the
//!   `tooltip` text is carried on the spec for consumer wiring.
//! - **Click/keyboard activation** lives in the preview `main.rs` event loop.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ButtonTone, ButtonVariant, ControlSize, IconButtonSpec};

use crate::presentation::{
    control_height_rem, rem_to_px, resolve_semantic_size, resolve_supporting_visual_size,
    size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Per-size square delta in rem, applied to the md control-height (contract §7).
/// xs −0.25, sm −0.375, md 0, lg +0.375, xl +0.5. Distinct from the Button
/// height-offset scale (which uses xs −0.5).
fn icon_button_size_delta_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.25,
        ControlSize::Sm => -0.375,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.375,
        ControlSize::Xl => 0.5,
    }
}

pub fn js_icon_button(spec: &IconButtonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let tone = spec.tone;

    // ── Square size: md control-height ± contract per-size delta ──
    let md_height = rem_to_px(control_height_rem(ControlSize::Md));
    let size_px = md_height + rem_to_px(icon_button_size_delta_rem(effective_size));

    // ── Glyph tracks the supporting-visual size (contract §13). ──
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));

    let radius = resolve_radius(theme, "radius.control");
    let is_pressed = spec.is_pressed.unwrap_or(false);
    let is_unavailable = spec.is_disabled || spec.is_loading;

    // ── Shared color tokens ──
    let surface: Color = resolve_color(theme, "color.background.surface").into();
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();
    let accent: Color = resolve_color(theme, "color.accent.base").into();
    let danger: Color = resolve_color(theme, "color.status.danger").into();
    let border_default: Color = resolve_color(theme, "color.border.default").into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let text_inverse: Color = resolve_color(theme, "color.text.inverse").into();

    // Status-tone family (icon-button.md §8 Tone: danger / Tone: success).
    // `Default` carries no status tint. The `Danger` compat variant maps to danger.
    let status: Option<Color> = match (spec.variant, tone) {
        (ButtonVariant::Danger, _) => Some(danger),
        (_, ButtonTone::Danger) => Some(danger),
        (_, ButtonTone::Success) => {
            Some(resolve_color(theme, "color.status.success").into())
        }
        (_, ButtonTone::Warning) => {
            Some(resolve_color(theme, "color.status.warning").into())
        }
        (_, ButtonTone::Default) => None,
    };

    // ── Variant × tone fill / border / text (contract §8) ──
    let (mut fill, mut border, mut text_color) = match (spec.variant, status) {
        // Ghost base: transparent fill + border; default text primary.
        (ButtonVariant::Ghost, None) => (Color::TRANSPARENT, Color::TRANSPARENT, text_primary),
        // Ghost danger/success: transparent fill/border, status-colored glyph.
        (ButtonVariant::Ghost, Some(s)) => (Color::TRANSPARENT, Color::TRANSPARENT, s),
        // Primary base: accent fill, accent-84%-black border, inverse text.
        (ButtonVariant::Primary, None) => {
            (accent, accent.mix(Color::BLACK, 0.84), text_inverse)
        }
        // Primary danger/success: solid status fill, status-84%-black border, inverse text.
        (ButtonVariant::Primary, Some(s)) => {
            (s, s.mix(Color::BLACK, 0.84), text_inverse)
        }
        // Danger compat variant ≈ primary status (status is Some(danger) here).
        (ButtonVariant::Danger, Some(s)) => {
            (s, s.mix(Color::BLACK, 0.84), text_inverse)
        }
        (ButtonVariant::Danger, None) => {
            // Unreachable: Danger variant always yields Some(danger) above.
            (danger, danger.mix(Color::BLACK, 0.84), text_inverse)
        }
        // Secondary base: surface fill, border-default, text primary.
        (ButtonVariant::Secondary, None) => (surface, border_default, text_primary),
        // Secondary danger/success: status-16%-surface fill, status-46%-border, text primary.
        (ButtonVariant::Secondary, Some(s)) => (
            s.mix(surface, 0.16),
            s.mix(border_default, 0.46),
            text_primary,
        ),
    };

    // ── Pressed (non-primary): solid accent treatment (contract §8) ──
    if is_pressed && !matches!(spec.variant, ButtonVariant::Primary) {
        fill = accent;
        border = accent.mix(Color::BLACK, 0.85);
        text_color = text_inverse;
    }

    // ── Hover: color-mix(fill 76%, elevated); border 74% toward text-primary ──
    let hover_fill = fill.mix(elevated, 0.76);
    let hover_border = border.mix(text_primary, 0.74);
    let active_fill = fill.mix(elevated, 0.64);

    let icon_name = spec.icon.as_deref().unwrap_or("help-circle");

    // ── Root (square button) ──
    let mut el = ui_element::button("")
        .h(size_px)
        .w(size_px)
        .rounded(radius)
        .bg(fill)
        .text_color(text_color)
        .border(1.0)
        .border_color(border)
        .flex_row()
        .items_center()
        .justify_center()
        .focusable();

    // ── Glyph or spinner (mutually exclusive; contract §2) ──
    if spec.is_loading {
        el = el.child(
            ui_element::icon("loader")
                .w(icon_size)
                .h(icon_size)
                .text_color(text_color),
        );
    } else {
        el = el.child(
            ui_element::icon(icon_name)
                .w(icon_size)
                .h(icon_size)
                .text_color(text_color),
        );
    }

    // ── Interactive vs disabled ──
    if is_unavailable {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity).disabled(true);
    } else {
        el = el
            .cursor_pointer()
            .hover(move |s| s.bg(hover_fill).border_color(hover_border))
            .active(move |s| s.bg(active_fill));
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn ghost_default_has_transparent_fill() {
        let th = theme();
        let el = js_icon_button(&IconButtonSpec::new().with_icon("plus"), &th);
        // Ghost: transparent fill + transparent border.
        let bg = el.style.background.expect("bg set");
        assert!(bg.a < 0.01, "ghost fill should be transparent, got a={}", bg.a);
        let border = el.style.border_color.expect("border color set");
        assert!(border.a < 0.01, "ghost border should be transparent");
    }

    #[test]
    fn primary_has_accent_fill_and_visible_border() {
        let th = theme();
        let accent: Color = resolve_color(&th, "color.accent.base").into();
        let el = js_icon_button(
            &IconButtonSpec::new()
                .with_icon("plus")
                .with_variant(ButtonVariant::Primary),
            &th,
        );
        let bg = el.style.background.expect("bg set");
        assert!(
            (bg.r - accent.r).abs() < 0.02 && (bg.g - accent.g).abs() < 0.02,
            "primary fill should be accent base"
        );
        assert!(el.style.border_width > 0.0, "primary has a border");
    }

    #[test]
    fn secondary_differs_from_ghost_fill() {
        let th = theme();
        let ghost = js_icon_button(&IconButtonSpec::new().with_icon("a"), &th);
        let secondary = js_icon_button(
            &IconButtonSpec::new()
                .with_icon("a")
                .with_variant(ButtonVariant::Secondary),
            &th,
        );
        let g = ghost.style.background.expect("ghost bg");
        let s = secondary.style.background.expect("secondary bg");
        // Ghost transparent vs secondary opaque surface — alpha must differ.
        assert!(g.a < 0.01 && s.a > 0.5, "secondary fill must be opaque surface");
    }

    #[test]
    fn danger_tone_recolors_ghost_text() {
        let th = theme();
        let danger: Color = resolve_color(&th, "color.status.danger").into();
        let el = js_icon_button(
            &IconButtonSpec::new()
                .with_icon("trash")
                .with_tone(ButtonTone::Danger),
            &th,
        );
        // Ghost danger keeps transparent fill but danger glyph text.
        let txt = el.style.text_color.expect("text color set");
        assert!(
            (txt.r - danger.r).abs() < 0.02,
            "ghost danger text should be status-danger"
        );
    }

    #[test]
    fn success_tone_recolors_ghost_glyph() {
        let th = theme();
        let success: Color = resolve_color(&th, "color.status.success").into();
        let el = js_icon_button(
            &IconButtonSpec::new()
                .with_icon("check")
                .with_tone(ButtonTone::Success),
            &th,
        );
        // Ghost success keeps transparent fill but a success-colored glyph.
        let txt = el.style.text_color.expect("text color set");
        assert!(
            (txt.r - success.r).abs() < 0.02 && (txt.g - success.g).abs() < 0.02,
            "ghost success glyph should be status-success, got {txt:?}"
        );
        let bg = el.style.background.expect("bg set");
        assert!(bg.a < 0.01, "ghost success fill stays transparent");
    }

    #[test]
    fn primary_success_fills_with_status_success() {
        let th = theme();
        let success: Color = resolve_color(&th, "color.status.success").into();
        let el = js_icon_button(
            &IconButtonSpec::new()
                .with_icon("check")
                .with_variant(ButtonVariant::Primary)
                .with_tone(ButtonTone::Success),
            &th,
        );
        let bg = el.style.background.expect("bg set");
        assert!(
            (bg.r - success.r).abs() < 0.02
                && (bg.g - success.g).abs() < 0.02
                && (bg.b - success.b).abs() < 0.02,
            "primary success fill should be status-success, got {bg:?}"
        );
    }

    #[test]
    fn pressed_non_primary_uses_accent_fill() {
        let th = theme();
        let accent: Color = resolve_color(&th, "color.accent.base").into();
        let el = js_icon_button(
            &IconButtonSpec::new().with_icon("pin").with_pressed(true),
            &th,
        );
        let bg = el.style.background.expect("bg set");
        assert!(
            (bg.r - accent.r).abs() < 0.02 && (bg.a - 1.0).abs() < 0.02,
            "pressed ghost should fill with accent base"
        );
    }

    #[test]
    fn loading_swaps_glyph_for_spinner() {
        let th = theme();
        let el = js_icon_button(
            &IconButtonSpec::new().with_icon("refresh-cw").with_loading(true),
            &th,
        );
        let tree = probe(&el, 80.0, 80.0);
        // Spinner uses the shared "loader" icon; the original glyph is replaced.
        assert!(tree.has_text("loader"), "loading should render the loader spinner glyph");
        assert!(
            !tree.has_text("refresh-cw"),
            "glyph must be replaced while loading"
        );
        assert!(el.style.disabled, "loading suppresses activation");
    }

    #[test]
    fn disabled_reduces_opacity_via_token() {
        let th = theme();
        let el = js_icon_button(
            &IconButtonSpec::new().with_icon("ban").with_disabled(true),
            &th,
        );
        let expected = resolve_opacity(&th, "state.opacity.disabled");
        assert!(el.style.disabled);
        assert!((el.style.opacity - expected).abs() < 0.001, "disabled opacity from token");
    }

    #[test]
    fn square_size_tracks_control_size() {
        let th = theme();
        let sm = probe(
            &js_icon_button(
                &IconButtonSpec::new().with_icon("star").with_size(ControlSize::Sm),
                &th,
            ),
            80.0,
            80.0,
        );
        let lg = probe(
            &js_icon_button(
                &IconButtonSpec::new().with_icon("star").with_size(ControlSize::Lg),
                &th,
            ),
            80.0,
            80.0,
        );
        let (sm_root, lg_root) = (&sm.nodes[0], &lg.nodes[0]);
        // Square: width == height, and lg larger than sm.
        assert!((sm_root.w - sm_root.h).abs() < 0.01, "sm is square");
        assert!((lg_root.w - lg_root.h).abs() < 0.01, "lg is square");
        assert!(lg_root.w > sm_root.w, "lg larger than sm");
    }

    #[test]
    fn probe_renders_glyph() {
        let th = theme();
        let el = js_icon_button(&IconButtonSpec::new().with_icon("settings"), &th);
        let tree = probe(&el, 80.0, 80.0);
        assert!(tree.has_text("settings"), "icon glyph renders: {:?}", tree.texts());
        assert_eq!(tree.count_kind("Icon"), 1, "exactly one glyph");
    }
}
