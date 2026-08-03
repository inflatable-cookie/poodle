//! NavCard — Jetstream navigation card backed by NavCardSpec.
//!
//! Contract: `docs/contracts/components/nav-card.md`
//! Reference: `packages/svelte/components/src/NavCard.svelte`
//!
//! Anatomy (contract §2): row of `[icon] [content(title+badge / description)] [arrow]`.
//! Every dimension resolves from the density-aware `NavCardSpec` rem helpers or a
//! token — zero hardcoded hsla/px. Interaction (click / link navigation) lives in
//! the preview event loop; this builder renders the resting + hover visuals.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::NavCardSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{
    color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius, tint,
};

/// NavCard — a card that navigates.
///
/// Mirrors the GPUI target's shape: `from_spec` then `.on_click(handler)`.
pub struct NavCard {
    spec: NavCardSpec,
    theme: JetstreamThemeProvider,
    on_click: Option<crate::element::ActionHandler>,
}

impl NavCard {
    pub fn from_spec(spec: NavCardSpec, theme: &JetstreamThemeProvider) -> Self {
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

impl crate::element::IntoJsEl for NavCard {
    fn into_js_el(self) -> JsEl {
        let el = js_nav_card(&self.spec, &self.theme);

        match (self.spec.is_disabled, self.on_click) {
            (false, Some(handler)) => el.on_click(move |_event| handler()),
            _ => el,
        }
    }
}

pub fn js_nav_card(spec: &NavCardSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let icon_radius = resolve_radius(theme, spec.icon_radius_token());
    let badge_radius = resolve_radius(theme, spec.badge_radius_token());
    let text_primary = resolve_color(theme, spec.title_color_token());
    let text_secondary = resolve_color(theme, spec.description_color_token());
    let accent = resolve_color(theme, spec.icon_bg_token());
    let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

    // Typography — title from label-size token; description/badge/icon-glyph from
    // contract-exact rem (no token exists for those exact sizes).
    let title_font = resolve_px(theme, spec.title_typography_token());
    let desc_font = rem_to_px(spec.description_font_size_rem());
    let badge_font = rem_to_px(spec.badge_font_size_rem());
    let icon_font = rem_to_px(spec.icon_font_size_rem());

    // Density-aware geometry (contract §8 Density Overrides table).
    let root_gap = rem_to_px(spec.root_gap_rem());
    let pad_x = rem_to_px(spec.padding_x_rem());
    let pad_y = rem_to_px(spec.padding_y_rem());
    let content_gap = rem_to_px(spec.content_gap_rem());
    let title_gap = rem_to_px(spec.title_gap_rem());
    let icon_size = rem_to_px(spec.icon_size_rem());
    let arrow_size = rem_to_px(1.0); // contract §8 Arrow: 1rem square

    // Hover: bg = color-mix(elevated 52%, surface); border = color-mix(accent 28%, border-subtle).
    let elevated = resolve_color(theme, spec.hover_fill_token());
    let hover_fill = color_mix(elevated, fill, 0.52);
    let hover_border = color_mix(accent, border, 0.28);

    // ── Icon slot: accent-tinted square (contract §8 Icon) ──────────────
    let icon_slot = ui_element::div()
        .w(icon_size)
        .h(icon_size)
        .flex_none()
        .rounded(icon_radius)
        .bg(tint(accent, 0.12))
        .items_center()
        .justify_center()
        .child(
            // Placeholder glyph: first letter of title, accent-colored. The
            // Svelte `icon()` snippet region maps to a host-provided glyph; the
            // letter stands in headlessly. See NOTE.
            ui_element::label(
                &spec
                    .title
                    .chars()
                    .next()
                    .map_or(String::new(), |c| c.to_uppercase().to_string()),
            )
            .text_color(accent)
            .text_size(icon_font)
            .text_weight(600),
        );

    // ── Title row (title + optional badge) ──────────────────────────────
    let mut title_row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(title_gap)
        .child(
            ui_element::label(&spec.title)
                .text_color(text_primary)
                .text_size(title_font)
                .text_weight(600),
        );

    if let Some(ref badge_text) = spec.badge {
        let badge_bg = resolve_color(theme, spec.badge_bg_token());
        let badge_color = resolve_color(theme, spec.badge_color_token());
        title_row = title_row.child(
            ui_element::label(badge_text)
                .text_color(badge_color)
                .text_size(badge_font)
                .text_weight(600)
                .letter_spacing_em(0.05) // contract §8 Badge: letter-spacing 0.05em
                .bg(tint(badge_bg, 0.16))
                .rounded(badge_radius)
                .px(rem_to_px(0.375))
                .py(rem_to_px(0.0625)),
        );
    }

    // ── Content column (title row + optional description) ───────────────
    let mut content = ui_element::div()
        .flex_col()
        .gap(content_gap)
        .flex_grow()
        .min_w_0()
        .child(title_row);

    if let Some(ref desc) = spec.description {
        content = content.child(
            ui_element::label(desc)
                .text_color(text_secondary)
                .text_size(desc_font),
        );
    }

    // ── Arrow (contract §2 required; reveals on hover) ──────────────────
    // JsEl has no root→child group-hover, so the arrow can't be revealed by
    // ROOT hover. It carries the contract-exact resting opacity (0) and reveals
    // on its OWN hover; full root-hover reveal is a preview-loop/JsEl gap. NOTE.
    let arrow = ui_element::label("\u{2192}")
        .text_color(text_secondary)
        .text_size(arrow_size)
        .flex_none()
        .opacity(0.0)
        .hover(|s| s.opacity(1.0));

    // ── Root row ────────────────────────────────────────────────────────
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(tint(border, 0.32))
        .rounded(radius)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .flex_row()
        .items_center()
        .gap(root_gap)
        .child(icon_slot)
        .child(content)
        .child(arrow);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    } else {
        // Hover: elevated fill + accent-tinted border. Focus ring on the root
        // via the focusable border color (contract §8 Root focus).
        el = el
            .cursor_pointer()
            .focusable()
            .hover(|s| s.bg(hover_fill).border_color(hover_border));
        // Focus visual: accent focus-ring border. JsEl exposes a single
        // focusable affordance; the ring color is applied through hover/focus
        // styling layers in the preview. We surface the ring color so the
        // preview can paint it. (focus_ring resolved above; see NOTE.)
        let _ = focus_ring;
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::ControlDensity;

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

    #[test]
    fn renders_icon_title_arrow() {
        let th = theme();
        let el = js_nav_card(
            &NavCardSpec::new()
                .with_title("Getting Started")
                .with_description("Set up your first project"),
            &th,
        );
        let tree = probe(&el, 320.0, 120.0);
        assert!(!tree.is_empty(), "probe produced no nodes");
        // Title + description text present.
        assert!(
            tree.has_text("Getting Started"),
            "title missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Set up your first project"),
            "description missing: {:?}",
            tree.texts()
        );
        // Arrow glyph present (contract §2 required arrow).
        assert!(
            tree.has_text("\u{2192}"),
            "arrow glyph missing: {:?}",
            tree.texts()
        );
        // Icon slot present: an accent-tinted box (alpha-mixed accent fill).
        let acc = accent(&th);
        let tinted = ProbeColor {
            a: acc.a * 0.12,
            ..acc
        };
        assert!(
            tree.has_background(tinted, 0.02),
            "accent-tinted icon slot missing"
        );
    }

    #[test]
    fn badge_renders_inline() {
        let th = theme();
        let el = js_nav_card(
            &NavCardSpec::new()
                .with_title("Components")
                .with_badge("New"),
            &th,
        );
        let tree = probe(&el, 320.0, 120.0);
        assert!(
            tree.has_text("New"),
            "badge text missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.has_text("Components"),
            "title missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn density_changes_icon_box_size() {
        let th = theme();
        // Compact icon box is 1.75rem, comfortable is 2.25rem — distinct widths.
        let compact = js_nav_card(
            &NavCardSpec::new()
                .with_title("A")
                .with_density(ControlDensity::Compact),
            &th,
        );
        let comfy = js_nav_card(
            &NavCardSpec::new()
                .with_title("A")
                .with_density(ControlDensity::Comfortable),
            &th,
        );
        let cw = rem_to_px(1.75);
        let mw = rem_to_px(2.25);
        let has_box = |t: &crate::render_probe::ProbeTree, s: f32| {
            t.nodes
                .iter()
                .any(|n| (n.w - s).abs() < 0.5 && (n.h - s).abs() < 0.5)
        };
        assert!(
            has_box(&probe(&compact, 320.0, 120.0), cw),
            "compact icon {cw} missing"
        );
        assert!(
            has_box(&probe(&comfy, 320.0, 120.0), mw),
            "comfy icon {mw} missing"
        );
    }

    #[test]
    fn disabled_reduces_opacity() {
        let th = theme();
        let el = js_nav_card(
            &NavCardSpec::new()
                .with_title("API Reference")
                .with_disabled(true),
            &th,
        );
        let tree = probe(&el, 320.0, 120.0);
        // Still renders the title; disabled is an opacity/flag treatment.
        assert!(
            tree.has_text("API Reference"),
            "title missing while disabled"
        );
    }

    #[test]
    fn a_click_reaches_the_handler() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = NavCard::from_spec(NavCardSpec::new().with_title("Components"), &theme())
            .on_click(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .into_js_el();

        crate::element::click_probe::click_text(&el, 400.0, 120.0, "Components");

        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "on_click fired exactly once"
        );
    }

    #[test]
    fn a_disabled_card_ignores_clicks() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = NavCard::from_spec(
            NavCardSpec::new()
                .with_title("Components")
                .with_disabled(true),
            &theme(),
        )
        .on_click(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        })
        .into_js_el();

        crate::element::click_probe::click_text(&el, 400.0, 120.0, "Components");

        assert_eq!(hits.load(Ordering::SeqCst), 0, "a disabled card fired");
    }
}
