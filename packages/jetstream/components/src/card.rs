//! Card — Jetstream card component backed by CardSpec.
//!
//! Contract: `docs/contracts/components/card.md`
//! Reference: `packages/svelte/components/src/Card.svelte`
//!
//! Resolves variant fill/border via `Color::mix`; density drives internal
//! padding, gap, and footer spacing. Renders the optional overflow-clipped
//! media region with inset radius.

use jetstream_ui::ui_element::{self, BoxShadow, JsEl};
use jetstream_ui::Color;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CardLayout, CardSpec, CardVariant};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_card(spec: &CardSpec, theme: &JetstreamThemeProvider, children: Vec<JsEl>) -> JsEl {
    let radius = resolve_radius(theme, spec.radius_token());
    let border_width = resolve_px(theme, spec.border_width_token());

    // Density/layout-aware spacing (contract §8 density table). Default-density
    // values map to tokens; compact/comfortable use contract-exact rem fallbacks.
    let gap = rem_to_px(spec.gap_rem());
    let padding_x = rem_to_px(spec.padding_x_rem());
    let padding_y = rem_to_px(spec.padding_y_rem());

    let panel: Color = resolve_color(theme, "color.background.panel").into();
    let elevated: Color = resolve_color(theme, "color.background.elevated").into();

    // Card fill (contract §8):
    //   default/outlined: color-mix(panel 10%, elevated)
    //   elevated:         color-mix(elevated 98%, panel)
    let fill = match spec.variant {
        CardVariant::Elevated => elevated.mix_srgb(panel, 0.98),
        _ => panel.mix_srgb(elevated, 0.10),
    };

    // Hover fill (contract §8): color-mix(elevated 94%, panel).
    let hover_fill = elevated.mix_srgb(panel, 0.94);

    let mut el = ui_element::div()
        .bg(fill)
        .rounded(radius)
        .pl(padding_x)
        .pr(padding_x)
        .pt(padding_y)
        .pb(padding_y)
        .gap(gap)
        .overflow_hidden();

    match spec.layout {
        CardLayout::Horizontal => {
            el = el.flex_row();
        }
        _ => {
            el = el.flex_col();
        }
    }

    // Border. Outlined/default carry a subtle/default border; elevated gets a
    // border too. Selected uses the accent border color.
    if let Some(selected_token) = spec.selected_border_token() {
        let selected_color = resolve_color(theme, selected_token);
        let sel_border_w = resolve_px(theme, "border.width.focus");
        el = el.border(sel_border_w).border_color(selected_color);
    } else if let Some(border_token) = spec.border_token() {
        let border_color = resolve_color(theme, border_token);
        el = el.border(border_width).border_color(border_color);
    }

    // Variant box-shadow treatment (contract §8/§10, dark mode) via shadow_layers:
    //   elevated → two stacked outset drops + an inset top highlight + a 1px outset
    //              perimeter ring (the contract 4-layer stack);
    //   default/outlined → a subtle inset 1px perimeter ring;
    //   selected → adds an accent inset ring on top of the variant base.
    let with_a = |c: glam::Vec4, a: f32| glam::Vec4::new(c.x, c.y, c.z, a);
    let black = |a: f32| glam::Vec4::new(0.0, 0.0, 0.0, a);
    let mut shadows: Vec<BoxShadow> = if matches!(spec.variant, CardVariant::Elevated) {
        let inv = resolve_color(theme, "color.text.inverse");
        let bd = resolve_color(theme, "color.border.default");
        vec![
            BoxShadow {
                offset_x: 0.0,
                offset_y: rem_to_px(1.125),
                blur: rem_to_px(2.5),
                spread: 0.0,
                color: black(0.38),
                inset: false,
            },
            BoxShadow {
                offset_x: 0.0,
                offset_y: rem_to_px(0.375),
                blur: rem_to_px(0.875),
                spread: 0.0,
                color: black(0.24),
                inset: false,
            },
            BoxShadow {
                offset_x: 0.0,
                offset_y: rem_to_px(0.0625),
                blur: 0.0,
                spread: 0.0,
                color: with_a(inv, 0.10),
                inset: true,
            },
            BoxShadow {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: rem_to_px(0.0625),
                color: with_a(bd, 0.12),
                inset: false,
            },
        ]
    } else {
        let bs = resolve_color(theme, "color.border.subtle");
        vec![BoxShadow {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: rem_to_px(0.0625),
            color: with_a(bs, 0.18),
            inset: true,
        }]
    };
    if spec.selected_border_token().is_some() {
        let accent = resolve_color(theme, "color.accent.base");
        shadows.push(BoxShadow {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: rem_to_px(0.0625),
            color: with_a(accent, 0.5),
            inset: true,
        });
    }
    el = el.shadow_layers(shadows);

    // Interactive cards get a hover fill + pointer cursor.
    if spec.is_interactive {
        el = el.hover(move |s| s.bg(hover_fill)).cursor_pointer();
    }

    // ── Media region ─────────────────────────────────────────
    // Overflow-clipped with inset radius (contract §8: radius - 0.1875rem).
    // When `has_media` is set, the FIRST child is treated as media content.
    let mut children = children.into_iter();
    if spec.has_media {
        if let Some(media) = children.next() {
            let media_radius = (radius - rem_to_px(spec.media_radius_inset_rem())).max(0.0);
            let mut media_region = ui_element::div()
                .overflow_hidden()
                .rounded(media_radius)
                .child(media);
            // Horizontal layout: media occupies a fixed 8rem leading column.
            if matches!(spec.layout, CardLayout::Horizontal) {
                media_region = media_region.w(rem_to_px(8.0));
            }
            el = el.child(media_region);
        }
    }

    for child in children {
        el = el.child(child);
    }

    let el = crate::aria::with_aria_label(el, spec.aria_label.as_deref());
    // Contract: an interactive card is `role="button"`. A non-interactive one
    // is structure and must stay structure, or every card in a list announces
    // itself as pressable.
    if spec.is_interactive {
        el.aria_role(jetstream_ui::accesskit::Role::Button)
    } else {
        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::ControlDensity;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn body() -> Vec<JsEl> {
        vec![ui_element::label("Body").text_size(13.0)]
    }

    #[test]
    fn renders_body() {
        let tree = probe(&js_card(&CardSpec::new(), &theme(), body()), 400.0, 200.0);
        assert!(!tree.is_empty(), "card produced no nodes");
        assert!(tree.has_text("Body"), "body missing: {:?}", tree.texts());
    }

    #[test]
    fn density_changes_inner_padding() {
        // Inline padding (content x-offset): contract density table is
        // compact 0.75rem < default 1rem == comfortable 1rem.
        let probe_x = |density: ControlDensity| -> f32 {
            let spec = CardSpec::new().with_density(density);
            let tree = probe(&js_card(&spec, &theme(), body()), 400.0, 200.0);
            // First child (the body label) — its x is the card's inline padding.
            tree.nodes[1].x
        };
        let compact = probe_x(ControlDensity::Compact);
        let default = probe_x(ControlDensity::Default);
        let comfortable = probe_x(ControlDensity::Comfortable);
        assert!(
            compact < default,
            "compact padding ({compact}) not tighter than default ({default})"
        );
        assert!(
            (default - comfortable).abs() < 0.5,
            "default and comfortable inline padding both equal 1rem per the density table"
        );
    }

    #[test]
    fn density_changes_gap() {
        // Internal gap (sibling spacing): compact 0.625rem < default 0.75rem
        // < comfortable 1rem. Probe two stacked children and read the second's
        // y-offset, which grows with the gap.
        let probe_gap_y = |density: ControlDensity| -> f32 {
            let spec = CardSpec::new().with_density(density);
            let kids = vec![
                ui_element::label("A").text_size(13.0),
                ui_element::label("B").text_size(13.0),
            ];
            let tree = probe(&js_card(&spec, &theme(), kids), 400.0, 400.0);
            // nodes: [root, A, B]. B.y - A.y - A.h ≈ gap.
            let a = &tree.nodes[1];
            let b = &tree.nodes[2];
            b.y - (a.y + a.h)
        };
        let compact = probe_gap_y(ControlDensity::Compact);
        let default = probe_gap_y(ControlDensity::Default);
        let comfortable = probe_gap_y(ControlDensity::Comfortable);
        assert!(
            compact < default,
            "compact gap ({compact}) not < default ({default})"
        );
        assert!(
            default < comfortable,
            "default gap ({default}) not < comfortable ({comfortable})"
        );
    }

    #[test]
    fn density_changes_inner_padding_exact() {
        // Contract-exact: compact 0.75rem (12px), default 1rem (16px),
        // comfortable 1rem (16px) inline padding.
        let probe_x = |density: ControlDensity| -> f32 {
            let spec = CardSpec::new().with_density(density);
            probe(&js_card(&spec, &theme(), body()), 400.0, 200.0).nodes[1].x
        };
        assert!((probe_x(ControlDensity::Compact) - 12.0).abs() < 0.5);
        assert!((probe_x(ControlDensity::Default) - 16.0).abs() < 0.5);
    }

    #[test]
    fn media_region_renders_clipped() {
        // With has_media, the first child is wrapped in a clipped media region.
        let media = ui_element::label("IMG");
        let spec = CardSpec::new().with_media(true);
        let tree = probe(&js_card(&spec, &theme(), vec![media]), 400.0, 200.0);
        assert!(
            tree.has_text("IMG"),
            "media content missing: {:?}",
            tree.texts()
        );
        // Media wrapper adds an extra Panel node above the content depth.
        assert!(
            tree.count_kind("Panel") >= 2,
            "media region wrapper missing"
        );
    }
}
