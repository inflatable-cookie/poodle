//! Skeleton specimen — all shapes and all five presets.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::skeleton::js_skeleton;
use poodle_jetstream_components::presentation::rem_to_px;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{SkeletonPreset, SkeletonSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // ── Basic shapes ─────────────────────────────────────────────────
        .child(group("Rectangle (default)", secondary,
            div().flex_col().gap(rem_to_px(0.5)).w(rem_to_px(18.75))
                .child(js_skeleton(&SkeletonSpec::new(), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_width("75%"), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_width("50%"), theme))
        ))
        .child(group("Circle", secondary,
            div().flex_row().gap(rem_to_px(0.75)).items_center()
                .child(js_skeleton(
                    &SkeletonSpec::new()
                        .with_shape("circle")
                        .with_width("2rem")
                        .with_height("2rem"),
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
                        .with_shape("circle")
                        .with_width("3rem")
                        .with_height("3rem"),
                    theme,
                ))
        ))
        .child(group("Text lines", secondary,
            div().flex_col().gap(rem_to_px(0.375)).w(rem_to_px(18.75))
                .child(js_skeleton(&SkeletonSpec::new().with_shape("text"), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_shape("text").with_width("80%"), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_shape("text").with_width("60%"), theme))
        ))

        // ── Presets ───────────────────────────────────────────────────────
        .child(group("Preset: TableRow", secondary,
            div().flex_col().gap(rem_to_px(0.25)).w(rem_to_px(37.5))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::TableRow), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::TableRow), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::TableRow), theme))
        ))
        .child(group("Preset: Card", secondary,
            div().flex_row().gap(rem_to_px(1.0))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::Card), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::Card), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::Card), theme))
        ))
        .child(group("Preset: ListItem", secondary,
            div().flex_col().gap(rem_to_px(0.25)).w(rem_to_px(25.0))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::ListItem), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::ListItem), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::ListItem), theme))
        ))
        .child(group("Preset: DetailSection", secondary,
            div().flex_col().gap(rem_to_px(0.5)).w(rem_to_px(25.0))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::DetailSection), theme))
                .child(js_skeleton(&SkeletonSpec::new()
                    .with_preset(SkeletonPreset::DetailSection)
                    .with_lines(5),
                    theme,
                ))
        ))
        .child(group("Preset: AvatarLine", secondary,
            div().flex_col().gap(rem_to_px(0.25)).w(rem_to_px(25.0))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::AvatarLine), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::AvatarLine), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_preset(SkeletonPreset::AvatarLine), theme))
        ))

        // ── Animated = false ──────────────────────────────────────────────
        .child(group("Non-animated (static fill)", secondary,
            div().flex_col().gap(rem_to_px(0.5)).w(rem_to_px(18.75))
                .child(js_skeleton(&SkeletonSpec::new().with_animated(false), theme))
                .child(js_skeleton(&SkeletonSpec::new().with_animated(false), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
