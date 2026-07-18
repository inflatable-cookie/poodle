//! Skeleton specimen — every shape and all five presets via the REAL
//! `js_skeleton` builder. Presets exercise the component's own preset path;
//! single shapes use rem widths (JsEl's `parse_dim` has no `%` support, so
//! percentage strings would silently fall back to full width).

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::presentation::rem_to_px;
use poodle_jetstream_components::skeleton::js_skeleton;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{SkeletonPreset, SkeletonSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // ── Basic shapes (contract §13: line / circle / block) ───────────────
        .child(group("Basic shapes", secondary,
            div().flex_row().gap(rem_to_px(0.75)).items_center()
                .child(js_skeleton(
                    &SkeletonSpec::new().with_shape("line").with_width("12rem"),
                    theme,
                ))
                .child(js_skeleton(
                    &SkeletonSpec::new()
                        .with_shape("circle")
                        .with_width("2.5rem")
                        .with_height("2.5rem"),
                    theme,
                ))
                .child(js_skeleton(
                    &SkeletonSpec::new()
                        .with_shape("block")
                        .with_width("8rem")
                        .with_height("3rem"),
                    theme,
                ))
        ))
        // ── Partial-width lines (rem widths — % would render full) ───────────
        .child(group("Partial-width lines", secondary,
            div().flex_col().gap(rem_to_px(0.375)).w(rem_to_px(18.0))
                .child(js_skeleton(&SkeletonSpec::new().with_shape("line").with_width("18rem"), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_shape("line").with_width("13.5rem"), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_shape("line").with_width("9rem"), theme))
        ))

        // ── Presets ──────────────────────────────────────────────────────────
        .child(group("Preset: AvatarLine", secondary,
            div().flex_col().gap(rem_to_px(0.25)).w(rem_to_px(25.0))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::AvatarLine), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::AvatarLine), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::AvatarLine), theme))
        ))
        .child(group("Preset: ListItem (\u{00d7}3)", secondary,
            div().flex_col().gap(rem_to_px(0.25)).w(rem_to_px(25.0))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::ListItem), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::ListItem), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::ListItem), theme))
        ))
        .child(group("Preset: TableRow (\u{00d7}3)", secondary,
            div().flex_col().gap(rem_to_px(0.25)).w(rem_to_px(37.5))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::TableRow), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::TableRow), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::TableRow), theme))
        ))
        .child(group("Preset: Card", secondary,
            div().flex_row().gap(rem_to_px(1.0))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::Card), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::Card), theme))
        ))
        .child(group("Preset: DetailSection", secondary,
            div().flex_col().gap(rem_to_px(0.5)).w(rem_to_px(25.0))
                .child(js_skeleton(&SkeletonSpec::new()
                    .with_preset(SkeletonPreset::DetailSection)
                    .with_lines(4),
                    theme,
                ))
        ))

        // ── Static (no shimmer — JsEl has no animation regardless) ───────────
        .child(group("Static (no animation)", secondary,
            div().flex_col().gap(rem_to_px(0.5)).w(rem_to_px(18.0))
                .child(js_skeleton(
                    &SkeletonSpec::new()
                        .with_shape("line")
                        .with_width("10rem")
                        .with_animated(false),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
