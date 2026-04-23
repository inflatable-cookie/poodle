//! Skeleton — Jetstream placeholder component backed by SkeletonSpec.
//!
//! Jetstream cannot animate, so skeletons render as static gray boxes.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, SkeletonPreset, SkeletonSpec};

use crate::presentation::{control_height_rem, rem_to_px};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Parse a CSS-style dimension string ("2rem", "200px") into logical pixels.
/// Returns None if the string is not a recognised form.
fn parse_dim(s: &str) -> Option<f32> {
    if let Some(rem_str) = s.strip_suffix("rem") {
        rem_str.trim().parse::<f32>().ok().map(rem_to_px)
    } else if let Some(px_str) = s.strip_suffix("px") {
        px_str.trim().parse::<f32>().ok()
    } else {
        None
    }
}

/// Build a single skeleton shape element (fill + radius already resolved by caller).
fn single_shape(fill: glam::Vec4, radius: f32, spec: &SkeletonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let default_height = resolve_px(theme, spec.default_height_token());

    let parsed_w = spec.width.as_deref().and_then(parse_dim);
    let parsed_h = spec.height.as_deref().and_then(parse_dim);

    let mut el = ui_element::div()
        .bg(fill)
        .rounded(radius);

    match spec.shape.as_str() {
        "circle" => {
            // Width == height; use parsed height if provided, else default_height
            let side = parsed_h.unwrap_or(default_height);
            el = el.w(side).h(side);
            // radius_token already resolves to RADIUS_PILL so rounded() gives full circle
        }
        "text" => {
            // Narrow height, grows to fill width
            let h = parsed_h.unwrap_or(rem_to_px(0.75));
            el = el.h(h);
            if let Some(w) = parsed_w {
                el = el.w(w);
            } else {
                el = el.grow();
            }
        }
        _ => {
            // rectangle (default)
            if let Some(h) = parsed_h {
                el = el.h(h);
            } else {
                el = el.min_h(default_height);
            }
            if let Some(w) = parsed_w {
                el = el.w(w);
            } else {
                el = el.grow();
            }
        }
    }

    el
}

pub fn js_skeleton(spec: &SkeletonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let fill = resolve_color(theme, spec.fill_token());
    let radius = resolve_radius(theme, spec.radius_token());
    // Shared gap for preset layouts
    let gap_sm = resolve_px(theme, "space.stack.sm");
    let gap_md = resolve_px(theme, "space.inline.sm");

    // Helper: a basic rectangle skeleton block with shared fill/radius
    let rect = |w_px: Option<f32>, h_px: f32| -> JsEl {
        let mut el = ui_element::div().bg(fill).rounded(radius).h(h_px);
        match w_px {
            Some(w) => el = el.w(w),
            None => el = el.grow(),
        }
        el
    };

    // Helper: a circle skeleton block
    let circle = |side: f32| -> JsEl {
        ui_element::div().bg(fill).rounded(9999.0).w(side).h(side)
    };

    if let Some(ref preset) = spec.preset {
        match preset {
            SkeletonPreset::AvatarLine => {
                // flex-row: circle avatar + flex-col of two text lines
                let avatar_side = rem_to_px(2.0);
                let text_col = ui_element::div()
                    .flex_col()
                    .gap(gap_sm)
                    .child(rect(Some(rem_to_px(8.0)), rem_to_px(0.875)))  // title ~80%
                    .child(rect(Some(rem_to_px(5.0)), rem_to_px(0.75)));  // subtitle ~50%

                ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(gap_md)
                    .child(circle(avatar_side))
                    .child(text_col)
            }

            SkeletonPreset::ListItem => {
                // flex-row: small icon circle + growing text line
                let icon_side = rem_to_px(1.25);
                ui_element::div()
                    .flex_row()
                    .items_center()
                    .gap(gap_md)
                    .child(circle(icon_side))
                    .child(rect(None, rem_to_px(0.875)))
            }

            SkeletonPreset::TableRow => {
                // flex-row: 4 equal-width growing blocks at control height
                let row_h = rem_to_px(control_height_rem(ControlSize::Md));
                ui_element::div()
                    .flex_row()
                    .gap(gap_md)
                    .child(rect(None, row_h))
                    .child(rect(None, row_h))
                    .child(rect(None, row_h))
                    .child(rect(None, row_h))
            }

            SkeletonPreset::Card => {
                // flex-col: tall image area + title line + subtitle line
                ui_element::div()
                    .flex_col()
                    .gap(gap_sm)
                    .child(rect(None, rem_to_px(8.0)))          // image area
                    .child(rect(None, rem_to_px(1.0)))          // title
                    .child(rect(Some(rem_to_px(6.0)), rem_to_px(0.75))) // subtitle
            }

            SkeletonPreset::DetailSection => {
                // flex-col: header line + spec.lines text lines
                let mut col = ui_element::div()
                    .flex_col()
                    .gap(gap_sm)
                    .child(rect(Some(rem_to_px(5.0)), rem_to_px(1.0))); // header

                for _ in 0..spec.lines {
                    col = col.child(rect(None, rem_to_px(0.875)));
                }
                col
            }
        }
    } else {
        single_shape(fill, radius, spec, theme)
    }
}
