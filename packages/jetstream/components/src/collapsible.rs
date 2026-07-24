//! Collapsible — Jetstream collapsible container backed by CollapsibleSpec.
//!
//! Contract: `docs/contracts/components/collapsible.md`
//! Reference: `packages/svelte/components/src/Collapsible.svelte`
//!
//! ALL dimensions/colors resolve from tokens. Anatomy: a `1fr auto` trigger
//! grid (heading block grows, chevron indicator trailing), with the heading
//! stacking title over description. JsEl has no CSS grid, so `1fr auto` is
//! emulated with `flex_row` + a `flex_1` heading block + a `flex_shrink_0`
//! indicator. JsEl has no rotation, so the chevron swaps icon name (Tier-3,
//! matching the other Jetstream disclosure components).

use jetstream_ui::ui_element::{self, BoxShadow, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CollapsibleSpec, ControlDensity, ControlSize};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius, tint};

pub fn js_collapsible(spec: &CollapsibleSpec, theme: &JetstreamThemeProvider, content: Option<JsEl>) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let is_open = spec.current_open();

    // Contract §8 root: gap = space.stack.md when open, 0 when closed.
    let open_gap = resolve_px(theme, "space.stack.md");
    let root_gap = if is_open { open_gap } else { 0.0 };
    // Contract §8 trigger gap (1fr ↔ auto) = space.inline.md.
    let trigger_gap = resolve_px(theme, "space.inline.md");
    // Contract §8 heading gap (title ↔ description) = space.inline.sm.
    let heading_gap = resolve_px(theme, "space.inline.sm");

    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    // Contract §8 title font-size scales per-size (xs 0.8125 … xl 1.125rem).
    let title_font = rem_to_px(match effective_size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.0625,
        ControlSize::Xl => 1.125,
    });
    // Contract §8 description font-size scales per-size (xs 0.6875 … xl 0.9375rem).
    let desc_font = rem_to_px(size_font_rem(effective_size));
    // Contract §8 indicator font-size = 0.75rem.
    let icon_size = rem_to_px(0.75);

    // Contract §8 root surface.
    let elevated = resolve_color(theme, "color.background.elevated");
    let panel = resolve_color(theme, "color.background.panel");
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let accent_base = resolve_color(theme, spec.highlight_accent_token());
    let radius = resolve_radius(theme, "radius.surface");

    // Contract §8: background = color-mix(elevated 40%, panel).
    let root_bg = color_mix(elevated, panel, 0.40);
    // Contract §8: border = color-mix(border-subtle 36%, transparent).
    let root_border = tint(border_subtle, spec.border_subtle_alpha());
    // Contract §8 highlighted: border accent-base 55% (+ accent-base 12% halo —
    // the halo box-shadow has no JsEl primitive; the accent border is applied).
    let highlight_border = tint(accent_base, spec.highlight_border_alpha());

    // Contract §8 root padding: Y = 0.625rem (fixed), X = density (0.5/1.0/1.0rem).
    let pad_y = rem_to_px(0.625);
    let pad_x = rem_to_px(match spec.density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.0,
    });

    // Contract §8 Root box-shadow: `inset 0 0.0625rem 0 color-mix(text-inverse
    // 8%, transparent)` — a top highlight in the inverse text color at 8% alpha.
    // (The highlighted-state `0 0 0 0.125rem accent@12%` halo is a focus ring,
    // carried by the accent border above, not a treatment shadow.)
    let text_inverse = resolve_color(theme, "color.text.inverse");
    let root_highlight = glam::Vec4::new(text_inverse.x, text_inverse.y, text_inverse.z, 0.08);

    let mut outer = ui_element::div()
        .flex_col()
        .gap(root_gap)
        .min_w_0()
        .self_stretch()
        .px(pad_x)
        .py(pad_y)
        .bg(root_bg)
        .border(1.0)
        .border_color(if spec.highlighted {
            highlight_border
        } else {
            root_border
        })
        .rounded(radius)
        .shadow_layers(vec![BoxShadow {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: root_highlight,
            inset: true,
        }]);

    // ── Trigger: 1fr heading block + auto chevron (emulated grid) ──
    let mut heading = ui_element::div()
        .flex_col()
        .gap(heading_gap)
        .min_w_0()
        .flex_1();

    if let Some(ref title) = spec.title {
        heading = heading.child(
            ui_element::label(title)
                .text_color(text_primary)
                .text_size(title_font)
                .text_weight(700) // Contract §8 title weight 700
                .line_height(1.2),
        );
    }
    // Contract: description renders in the heading whenever present (closed or
    // open), stacked under the title — not beside the chevron.
    if let Some(ref description) = spec.description {
        heading = heading.child(
            ui_element::label(description)
                .text_color(text_secondary)
                .text_size(desc_font)
                .line_height(1.45),
        );
    }

    let chevron_icon = if is_open { "chevron-down" } else { "chevron-right" };
    let indicator = ui_element::icon(chevron_icon)
        .w(icon_size)
        .h(icon_size)
        .flex_shrink_0()
        .text_color(text_secondary);

    let trigger = ui_element::div()
        .flex_row()
        .items_center()
        .gap(trigger_gap)
        .w_full()
        .focusable()
        .cursor_pointer()
        .child(heading)
        .child(indicator);

    outer = outer.child(trigger);

    // ── Content region (only when open) ──
    // Contract §8 Content: min-width 0, padding-top 0.125rem.
    if is_open {
        if let Some(content_el) = content {
            let content_wrapper = ui_element::div()
                .flex_col()
                .min_w_0()
                .self_stretch()
                .pt(rem_to_px(0.125))
                .child(content_el);
            outer = outer.child(content_wrapper);
        }
    }

    // Disabled state: reduce opacity of the whole element (contract §8 Root disabled).
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        outer = outer.opacity(opacity);
    }

    outer
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> CollapsibleSpec {
        CollapsibleSpec::new()
            .with_title("Project settings")
            .with_description("Configure build options and deploy targets.")
    }

    /// Trigger heading (title + description) and the chevron indicator render.
    #[test]
    fn trigger_heading_and_chevron_render() {
        let el = js_collapsible(&spec(), &theme(), None);
        let tree = probe(&el, 400.0, 200.0);
        assert!(tree.has_text("Project settings"), "title missing: {:?}", tree.texts());
        assert!(
            tree.has_text("Configure build options and deploy targets."),
            "description missing (must render in heading even when closed): {:?}",
            tree.texts()
        );
        assert!(tree.count_kind("Icon") >= 1, "chevron indicator missing");
    }

    /// Open collapsible shows its body content; closed does not.
    #[test]
    fn open_shows_body() {
        let body = ui_element::label("Body content");

        let closed = js_collapsible(&spec(), &theme(), Some(ui_element::label("Body content")));
        let closed_tree = probe(&closed, 400.0, 200.0);
        assert!(
            !closed_tree.has_text("Body content"),
            "closed collapsible must not render body"
        );

        let open = js_collapsible(&spec().with_open(true), &theme(), Some(body));
        let open_tree = probe(&open, 400.0, 200.0);
        assert!(
            open_tree.has_text("Body content"),
            "open collapsible must render body: {:?}",
            open_tree.texts()
        );
    }

    /// Highlighted root differs from the default root (accent border applied).
    #[test]
    fn highlighted_differs_from_default() {
        let th = theme();
        let default_el = js_collapsible(&spec(), &th, None);
        let highlighted_el = js_collapsible(&spec().with_highlighted(true), &th, None);

        let default_border = default_el.style.border_color;
        let highlighted_border = highlighted_el.style.border_color;
        assert!(
            default_border != highlighted_border,
            "highlighted border must differ from default: {:?} vs {:?}",
            default_border,
            highlighted_border
        );

        // The highlighted border is the accent base tinted to 55%.
        let accent = resolve_color(&th, spec().highlight_accent_token());
        let expected = tint(accent, spec().highlight_border_alpha());
        let got = highlighted_border.expect("highlighted border set");
        assert!(
            (got.r - expected.x).abs() < 1e-4
                && (got.g - expected.y).abs() < 1e-4
                && (got.b - expected.z).abs() < 1e-4
                && (got.a - expected.w).abs() < 1e-4,
            "highlighted border must be accent-base @55%: {:?} vs {:?}",
            got,
            expected
        );
    }
}
