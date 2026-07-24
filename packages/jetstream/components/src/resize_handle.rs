//! ResizeHandle — Jetstream resize handle backed by ResizeHandleSpec.

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{Orientation, ResizeHandleSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_opacity, tint};

pub fn js_resize_handle(spec: &ResizeHandleSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Contract idle line: color-mix(border-default 82%, transparent).
    let handle_color = tint(resolve_color(theme, spec.border_color_token()), 0.82);
    // Contract §8 hover/dragging: line recolors to accent-base.
    let hover_color = resolve_color(theme, spec.hover_color_token());
    let is_disabled = spec.is_disabled;

    // Contract: visual affordance 2px (0.125rem), hit target min 8px (0.5rem)
    let visual_size = rem_to_px(0.125);
    let hit_target = rem_to_px(0.5);

    // The affordance line. Contract §8 hover/dragging recolors it to accent-base;
    // wired on the line itself (JsEl has no group-hover). The line spans the full
    // length of the hit target so its hover region matches the handle.
    let build_line = |line: JsEl| -> JsEl {
        if is_disabled {
            line.bg(handle_color)
        } else {
            line.bg(handle_color)
                .hover(move |s| s.bg(hover_color))
                .active(move |s| s.bg(hover_color))
        }
    };

    let mut el = match spec.orientation {
        Orientation::Horizontal => {
            // Contract §7: horizontal orientation = vertical line.
            // Hit target: width 0.5rem (fixed), height 100% (stretch to parent —
            // NOT flex-grow, which would fill the whole row). col-resize cursor.
            // Line: width 0.125rem (fixed), height 100%.
            ui_element::div()
                .w(hit_target).self_stretch().flex_shrink_0()
                .flex_col().items_center().justify_center()
                .cursor_col_resize()
                .child(build_line(
                    ui_element::div().w(visual_size).h_full().rounded(999.0),
                ))
        }
        Orientation::Vertical => {
            // Contract §7: vertical orientation = horizontal line.
            // Hit target: height 0.5rem (fixed), width 100% (stretch). row-resize.
            // Line: height 0.125rem (fixed), width 100%.
            ui_element::div()
                .h(hit_target).self_stretch().flex_shrink_0()
                .flex_row().items_center().justify_center()
                .cursor_row_resize()
                .child(build_line(
                    ui_element::div().h(visual_size).w_full().rounded(999.0),
                ))
        }
    };

    if is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        // Contract §8 disabled: default cursor + 0.4 opacity, no interaction.
        el = el.opacity(opacity).cursor_default();
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeColor};
    use poodle_specs::Orientation;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// Contract orientation convention (matches GPUI/Svelte):
    /// `Horizontal` → a *vertical* line (narrow width, full height), so the line
    /// is taller than it is wide. `Vertical` → a *horizontal* line (full width,
    /// short height). Regression guard against the old inverted mapping.
    #[test]
    fn horizontal_renders_vertical_line() {
        // Place the handle in a row between two panes (its real composition) so
        // the grow axis is constrained; the handle keeps its 8px cross-axis.
        let handle = js_resize_handle(
            &ResizeHandleSpec::new().with_orientation(Orientation::Horizontal),
            &theme(),
        );
        let row = ui_element::div().flex_row().w(200.0).h(100.0).child(handle);
        let tree = probe(&row, 200.0, 100.0);

        // tree.nodes[0] = row wrapper, [1] = handle hit target, [2] = line.
        let root = &tree.nodes[1];
        assert!(
            (root.w - 8.0).abs() < 0.01,
            "horizontal hit target should be 0.5rem (8px) wide; got {}px",
            root.w
        );
        let line = &tree.nodes[2];
        assert!(
            (line.w - 2.0).abs() < 0.01,
            "horizontal line should be 0.125rem (2px) wide; got {}px",
            line.w
        );
        assert!(
            line.h > line.w,
            "horizontal orientation must render a vertical line (h > w); got {}x{}",
            line.w,
            line.h
        );
    }

    #[test]
    fn vertical_renders_horizontal_line() {
        // Place the handle in a column between two panes (its real composition).
        let handle = js_resize_handle(
            &ResizeHandleSpec::new().with_orientation(Orientation::Vertical),
            &theme(),
        );
        let col = ui_element::div().flex_col().w(100.0).h(200.0).child(handle);
        let tree = probe(&col, 100.0, 200.0);

        let root = &tree.nodes[1];
        assert!(
            (root.h - 8.0).abs() < 0.01,
            "vertical hit target should be 0.5rem (8px) tall; got {}px",
            root.h
        );
        let line = &tree.nodes[2];
        assert!(
            (line.h - 2.0).abs() < 0.01,
            "vertical line should be 0.125rem (2px) tall; got {}px",
            line.h
        );
        assert!(
            line.w > line.h,
            "vertical orientation must render a horizontal line (w > h); got {}x{}",
            line.w,
            line.h
        );
    }

    #[test]
    fn idle_line_is_82pct_border() {
        // Contract §8 idle: color-mix(border-default 82%, transparent) — the
        // border color at 0.82 alpha.
        let th = theme();
        let spec = ResizeHandleSpec::new();
        let expected = tint(resolve_color(&th, spec.border_color_token()), 0.82);
        let el = js_resize_handle(&spec, &th);
        let tree = probe(&el, 200.0, 200.0);
        let want = ProbeColor {
            r: expected.x,
            g: expected.y,
            b: expected.z,
            a: expected.w,
        };
        assert!(
            tree.has_background(want, 0.001),
            "idle line should be 82%-alpha border-default; want {want:?}, tree: {}",
            tree.to_json()
        );
    }

    #[test]
    fn disabled_applies_opacity() {
        let th = theme();
        let el = js_resize_handle(&ResizeHandleSpec::new().with_disabled(true), &th);
        // Disabled drops the root opacity to the disabled token (0.4).
        let expected = resolve_opacity(&th, ResizeHandleSpec::new().disabled_opacity_token());
        assert_eq!(el.style.opacity, expected);
    }
}
