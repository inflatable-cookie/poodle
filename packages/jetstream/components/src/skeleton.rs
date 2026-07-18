//! Skeleton — Jetstream placeholder component backed by SkeletonSpec.
//!
//! Skeletons breathe via a looping opacity pulse (`skeleton_pulse`) — the
//! engine persists animation clocks across immediate-mode rebuilds, keyed by
//! the element id. The contract's travelling gradient sweep still isn't
//! representable (needs animated gradient stops); the pulse is the stand-in.
//! Column-context line widths (60/40%, 80/100/60%) are true percentages via
//! `.w_pct`. TableRow cells stay proportional flex-basis weights on purpose —
//! the contract's 40/60/60/20% sum to 180%, i.e. they're relative weights in a
//! gapped row, not absolute percentages.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{SkeletonPreset, SkeletonSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

/// Parse a CSS-style dimension string ("2rem", "200px") into logical pixels.
/// Returns None if the string is not a recognised form.
fn parse_dim(s: &str) -> Option<f32> {
    if let Some(rem_str) = s.strip_suffix("rem") {
        rem_str.trim().parse::<f32>().ok().map(rem_to_px)
    } else if let Some(px_str) = s.strip_suffix("px") {
        px_str.trim().parse::<f32>().ok()
    } else {
        s.trim().parse::<f32>().ok()
    }
}

/// Contract §8 shimmer fill: flat mid-tone between the base outer stop
/// (`background-elevated 88%`) and the centre highlight (`background-surface
/// 92%, white`). JsEl has no gradient, so one flat tone stands in. Public so
/// consumers rendering skeleton rows (e.g. loading panels) can match the exact
/// resolved fill instead of guessing the legacy `fill_token()`.
pub fn shimmer_fill(spec: &SkeletonSpec, theme: &JetstreamThemeProvider) -> glam::Vec4 {
    let base = resolve_color(theme, spec.shimmer_base_token());
    let base = glam::Vec4::new(base.x, base.y, base.z, base.w * 0.88);
    let surface = resolve_color(theme, spec.shimmer_highlight_token());
    let white = glam::Vec4::new(1.0, 1.0, 1.0, 1.0);
    let highlight = color_mix(surface, white, 0.92);
    color_mix(highlight, base, 0.5)
}

/// Build a single skeleton shape element (fill + radius already resolved by caller).
fn single_shape(fill: glam::Vec4, radius: f32, spec: &SkeletonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let _ = theme;
    let parsed_w = spec.width.as_deref().and_then(parse_dim);
    let parsed_h = spec.height.as_deref().and_then(parse_dim);

    let mut el = ui_element::div().bg(fill).rounded(radius);

    match spec.shape.as_str() {
        // Contract §7: circle resolves 2.5rem × 2.5rem default.
        "circle" => {
            let side = parsed_w.or(parsed_h).unwrap_or(rem_to_px(2.5));
            el = el.w(side).h(side);
            // radius_token() resolves RADIUS_PILL so rounded() gives a full circle.
        }
        // Contract §7: block resolves 100% × 6rem default.
        "block" => {
            el = el.h(parsed_h.unwrap_or(rem_to_px(6.0)));
            match parsed_w {
                Some(w) => el = el.w(w),
                // Full width without cross-axis stretch (keeps the fixed height).
                None => el = el.w_full(),
            }
        }
        // Contract §7: line (and legacy "text"/"rectangle") resolves 100% × 0.875rem.
        _ => {
            el = el.h(parsed_h.unwrap_or(rem_to_px(0.875)));
            match parsed_w {
                Some(w) => el = el.w(w),
                None => el = el.w_full(),
            }
        }
    }

    el
}

pub fn js_skeleton(spec: &SkeletonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = shimmer_fill(spec, theme);
    let radius = resolve_radius(theme, spec.radius_token());
    let pill_radius = resolve_radius(theme, "radius.pill");
    let surface_radius = resolve_radius(theme, "radius.surface");

    // Contract preset gaps — exact rem (no space token maps these precisely).
    let gap_075 = rem_to_px(0.75);
    let gap_0625 = rem_to_px(0.625);
    let gap_05 = rem_to_px(0.5);
    let gap_0375 = rem_to_px(0.375);
    let gap_025 = rem_to_px(0.25);

    let line_h = rem_to_px(0.875); // contract base line / cell height
    let line_sm_h = rem_to_px(0.6875); // contract skeleton--line-sm

    // Fixed-width skeleton block.
    let rect = |w_px: f32, h_px: f32| -> JsEl {
        ui_element::div().bg(fill).rounded(radius).w(w_px).h(h_px)
    };
    // Proportional-width cell for ROW contexts: `pct` (0..1) drives a flex-basis
    // seed inside a growing flex row, approximating the contract's percentage
    // widths (table-row cells). Main-axis grow only — height stays fixed.
    let cell_pct = |pct: f32, h_px: f32| -> JsEl {
        ui_element::div()
            .bg(fill)
            .rounded(radius)
            .h(h_px)
            .flex_grow()
            .flex_basis(pct * 100.0)
    };
    // Percentage-width line for COLUMN contexts (list-text / card body) — the
    // contract's per-line widths (60/40%, 80/100/60%) are now true percentages.
    let line_pct = |pct: f32, h_px: f32| -> JsEl {
        ui_element::div().bg(fill).rounded(radius).w_pct(pct).h(h_px)
    };
    let line_full = |h_px: f32| -> JsEl { line_pct(1.0, h_px) };
    // Circle skeleton block using the resolved pill radius (not a literal).
    let circle = |side: f32| -> JsEl {
        ui_element::div().bg(fill).rounded(pill_radius).w(side).h(side)
    };

    let Some(ref preset) = spec.preset else {
        return single_shape(fill, radius, spec, theme)
            .id("poodle-skeleton")
            .animation(skeleton_pulse());
    };

    let built = match preset {
        // Contract: avatar 2.25rem circle + single 10rem line, row, 0.75rem gap.
        SkeletonPreset::AvatarLine => ui_element::div()
            .flex_row()
            .items_center()
            .gap(gap_075)
            .child(circle(rem_to_px(2.25)))
            .child(rect(rem_to_px(10.0), line_h)),

        // Contract: avatar 2.25rem circle + primary line 60% + line-sm 40%.
        SkeletonPreset::ListItem => {
            let text_col = ui_element::div()
                .flex_col()
                .gap(gap_0375)
                .flex_grow()
                .child(line_pct(0.60, line_h)) // primary line (contract 60%)
                .child(line_pct(0.40, line_sm_h)); // secondary line-sm (contract 40%)
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(gap_075)
                .child(circle(rem_to_px(2.25)))
                .child(text_col)
        }

        // Contract: 4 cells, height 0.875rem, widths 40/60/60/20%, 0.75rem gap.
        SkeletonPreset::TableRow => ui_element::div()
            .flex_row()
            .gap(gap_075)
            .child(cell_pct(0.40, line_h))
            .child(cell_pct(0.60, line_h))
            .child(cell_pct(0.60, line_h))
            .child(cell_pct(0.20, line_h)),

        // Contract: block-header 6rem + body 3 lines (80/100/60%) + 2 footer pills.
        SkeletonPreset::Card => {
            let block_radius = (surface_radius - rem_to_px(0.375)).max(0.0);
            let header = ui_element::div()
                .bg(fill)
                .rounded(block_radius)
                .h(rem_to_px(6.0))
                .w_full();
            let body = ui_element::div()
                .flex_col()
                .gap(gap_0375)
                .child(line_pct(0.80, line_h)) // contract 80%
                .child(line_full(line_h)) // contract 100%
                .child(line_pct(0.60, line_h)); // contract 60%
            let pill = || {
                ui_element::div()
                    .bg(fill)
                    .rounded(pill_radius)
                    .w(rem_to_px(3.5))
                    .h(rem_to_px(1.25))
            };
            let footer = ui_element::div()
                .flex_row()
                .gap(gap_05)
                .pt(gap_025)
                .child(pill())
                .child(pill());
            ui_element::div()
                .flex_col()
                .gap(gap_075)
                .p(rem_to_px(1.0))
                .child(header)
                .child(body)
                .child(footer)
        }

        // Contract: heading 8rem×1rem + `lines` rows of label(6rem) + value(max 14rem).
        SkeletonPreset::DetailSection => {
            let mut col = ui_element::div()
                .flex_col()
                .gap(gap_0625)
                .child(rect(rem_to_px(8.0), rem_to_px(1.0))); // heading

            for _ in 0..spec.lines {
                let label = ui_element::div()
                    .bg(fill)
                    .rounded(radius)
                    .w(rem_to_px(6.0))
                    .h(rem_to_px(0.75))
                    .flex_shrink_0();
                let value = ui_element::div()
                    .bg(fill)
                    .rounded(radius)
                    .h(rem_to_px(0.75))
                    .flex_grow()
                    .max_w(rem_to_px(14.0));
                let row = ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(rem_to_px(1.0))
                    .child(label)
                    .child(value);
                col = col.child(row);
            }
            col
        }
    };
    // Shimmer approximation: the whole skeleton breathes (opacity pulse) —
    // node opacity cascades multiplicatively to every shape. The id keys the
    // engine's animation clock across immediate-mode rebuilds.
    built.id("poodle-skeleton").animation(skeleton_pulse())
}

/// Ping-pong opacity pulse standing in for the contract's shimmer sweep
/// (a travelling gradient needs animated gradient stops — not yet available).
fn skeleton_pulse() -> jetstream_ui::Animation {
    use jetstream_ui::{AnimatableProperty, Animation, Easing, Keyframe, LoopMode};
    Animation {
        keyframes: vec![
            Keyframe { at: 0.0, values: vec![(AnimatableProperty::Opacity, 1.0)] },
            Keyframe { at: 1.0, values: vec![(AnimatableProperty::Opacity, 0.55)] },
        ],
        duration_secs: 1.1,
        easing: Easing::EaseInOut,
        loop_mode: LoopMode::PingPong,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;

    /// Skeletons breathe: the root declares a looping opacity pulse with a
    /// stable id (the engine's animation-clock key).
    #[test]
    fn skeleton_carries_pulse_animation() {
        let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK);
        let el = js_skeleton(&SkeletonSpec::new(), &theme);
        assert!(el.animation.is_some(), "skeleton pulses");
        assert!(el.id.is_some(), "animated element carries a stable id");
    }

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn spec() -> SkeletonSpec {
        SkeletonSpec::new()
    }

    #[test]
    fn line_shape_resolves_default_height_0875rem() {
        let th = theme();
        let s = spec().with_shape("line");
        let tree = probe(&js_skeleton(&s, &th), 200.0, 60.0);
        let node = &tree.nodes[0];
        assert!((node.h - rem_to_px(0.875)).abs() < 0.5, "line height = {}", node.h);
    }

    #[test]
    fn circle_shape_is_square_at_25rem_default() {
        let th = theme();
        let s = spec().with_shape("circle");
        let tree = probe(&js_skeleton(&s, &th), 200.0, 200.0);
        let n = &tree.nodes[0];
        assert!((n.w - n.h).abs() < 0.5, "circle not square: {}x{}", n.w, n.h);
        assert!((n.w - rem_to_px(2.5)).abs() < 0.5, "circle side = {}", n.w);
    }

    #[test]
    fn block_shape_resolves_6rem_height() {
        let th = theme();
        let s = spec().with_shape("block");
        let tree = probe(&js_skeleton(&s, &th), 200.0, 200.0);
        assert!((tree.nodes[0].h - rem_to_px(6.0)).abs() < 0.5, "block height = {}", tree.nodes[0].h);
    }

    #[test]
    fn explicit_width_height_override_defaults() {
        let th = theme();
        let s = spec().with_shape("line").with_width("8rem").with_height("3rem");
        let tree = probe(&js_skeleton(&s, &th), 400.0, 200.0);
        let n = &tree.nodes[0];
        assert!((n.w - rem_to_px(8.0)).abs() < 0.5, "w = {}", n.w);
        assert!((n.h - rem_to_px(3.0)).abs() < 0.5, "h = {}", n.h);
    }

    #[test]
    fn avatar_line_preset_is_avatar_plus_single_line() {
        let th = theme();
        let s = spec().with_preset(SkeletonPreset::AvatarLine);
        let tree = probe(&js_skeleton(&s, &th), 400.0, 80.0);
        // root + avatar circle + one line = 3 panels.
        assert_eq!(tree.count_kind("Panel"), 3, "{}", tree.to_json());
        // avatar is a 2.25rem square.
        let av = rem_to_px(2.25);
        assert!(
            tree.nodes.iter().any(|n| (n.w - av).abs() < 0.5 && (n.h - av).abs() < 0.5),
            "missing 2.25rem avatar"
        );
    }

    #[test]
    fn list_item_preset_has_avatar_and_two_lines() {
        let th = theme();
        let s = spec().with_preset(SkeletonPreset::ListItem);
        let tree = probe(&js_skeleton(&s, &th), 400.0, 80.0);
        // root + avatar + text col + 2 lines = 5 panels.
        assert_eq!(tree.count_kind("Panel"), 5, "{}", tree.to_json());
        // one line uses the line-sm height.
        assert!(
            tree.nodes.iter().any(|n| (n.h - rem_to_px(0.6875)).abs() < 0.5),
            "missing line-sm (0.6875rem)"
        );
    }

    #[test]
    fn table_row_preset_has_four_cells() {
        let th = theme();
        let s = spec().with_preset(SkeletonPreset::TableRow);
        let tree = probe(&js_skeleton(&s, &th), 400.0, 40.0);
        // root + 4 cells = 5 panels.
        assert_eq!(tree.count_kind("Panel"), 5, "{}", tree.to_json());
        // all cells at the 0.875rem line height.
        let cells = tree.nodes.iter().filter(|n| (n.h - rem_to_px(0.875)).abs() < 0.5).count();
        assert_eq!(cells, 4, "expected 4 cells at 0.875rem");
    }

    #[test]
    fn card_preset_has_header_three_body_lines_and_two_pills() {
        let th = theme();
        let s = spec().with_preset(SkeletonPreset::Card);
        let tree = probe(&js_skeleton(&s, &th), 320.0, 240.0);
        // root + header + body-col + 3 body lines + footer-row + 2 pills = 9 panels.
        assert_eq!(tree.count_kind("Panel"), 9, "{}", tree.to_json());
        // header is 6rem tall.
        assert!(
            tree.nodes.iter().any(|n| (n.h - rem_to_px(6.0)).abs() < 0.5),
            "missing 6rem block header"
        );
        // two pills at 3.5rem × 1.25rem.
        let pills = tree
            .nodes
            .iter()
            .filter(|n| (n.w - rem_to_px(3.5)).abs() < 0.5 && (n.h - rem_to_px(1.25)).abs() < 0.5)
            .count();
        assert_eq!(pills, 2, "expected 2 footer pills");
    }

    #[test]
    fn detail_section_preset_emits_lines_rows() {
        let th = theme();
        let s = spec().with_preset(SkeletonPreset::DetailSection).with_lines(4);
        let tree = probe(&js_skeleton(&s, &th), 400.0, 240.0);
        // heading at 8rem wide.
        assert!(
            tree.nodes.iter().any(|n| (n.w - rem_to_px(8.0)).abs() < 0.5),
            "missing 8rem heading"
        );
        // 4 label cells at 6rem wide (flex-shrink-0).
        let labels = tree.nodes.iter().filter(|n| (n.w - rem_to_px(6.0)).abs() < 0.5).count();
        assert_eq!(labels, 4, "expected 4 detail-row labels");
    }

    #[test]
    fn per_shape_radius_resolves_correctly() {
        assert_eq!(
            spec().with_shape("circle").radius_token(),
            poodle_tokens::semantic::RADIUS_PILL
        );
        assert_eq!(
            spec().with_shape("line").radius_token(),
            poodle_tokens::semantic::RADIUS_CONTROL
        );
        assert_eq!(
            spec().with_shape("block").radius_token(),
            poodle_tokens::semantic::RADIUS_SURFACE
        );
    }
}
