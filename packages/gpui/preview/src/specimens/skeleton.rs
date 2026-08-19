//! Skeleton specimen — every shape + every preset via the REAL `Skeleton`
//! builder. No hand-rolled preset layout: each preset group calls
//! `Skeleton::from_spec(SkeletonSpec::new().with_preset(...))` so the
//! component's own preset-rendering path is what's under test.

use crate::node_compat::{Eyebrow, Skeleton};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;

use poodle_specs::{EyebrowSpec, SkeletonPreset, SkeletonSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Basic shapes (contract §13: line / circle / block) ---
        .child(group(
            theme,
            "Basic shapes",
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(12.0))
                .items_center()
                .child(Skeleton::from_spec(
                    SkeletonSpec::new().with_shape("line").with_width("12rem"),
                    theme,
                ))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new()
                        .with_shape("circle")
                        .with_width("2.5rem")
                        .with_height("2.5rem"),
                    theme,
                ))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new()
                        .with_shape("block")
                        .with_width("8rem")
                        .with_height("3rem"),
                    theme,
                )),
        ))
        // --- Preset: avatar-line ---
        .child(group(
            theme,
            "Preset: avatar-line",
            Skeleton::from_spec(
                SkeletonSpec::new().with_preset(SkeletonPreset::AvatarLine),
                theme,
            ),
        ))
        // --- Preset: list-item (x3) ---
        .child(group(
            theme,
            "Preset: list-item (×3)",
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new().with_preset(SkeletonPreset::ListItem),
                    theme,
                ))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new().with_preset(SkeletonPreset::ListItem),
                    theme,
                ))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new().with_preset(SkeletonPreset::ListItem),
                    theme,
                )),
        ))
        // --- Preset: table-row (x3) ---
        .child(group(
            theme,
            "Preset: table-row (×3)",
            div()
                .flex()
                .flex_col()
                .w(px(rem_px(37.5)))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new().with_preset(SkeletonPreset::TableRow),
                    theme,
                ))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new().with_preset(SkeletonPreset::TableRow),
                    theme,
                ))
                .child(Skeleton::from_spec(
                    SkeletonSpec::new().with_preset(SkeletonPreset::TableRow),
                    theme,
                )),
        ))
        // --- Preset: card (2-up) ---
        .child(group(
            theme,
            "Preset: card",
            div()
                .flex()
                .flex_row()
                .gap(px(16.0))
                .child(div().flex_1().child(Skeleton::from_spec(
                    SkeletonSpec::new().with_preset(SkeletonPreset::Card),
                    theme,
                )))
                .child(div().flex_1().child(Skeleton::from_spec(
                    SkeletonSpec::new().with_preset(SkeletonPreset::Card),
                    theme,
                ))),
        ))
        // --- Preset: detail-section (lines=4) ---
        .child(group(
            theme,
            "Preset: detail-section",
            Skeleton::from_spec(
                SkeletonSpec::new()
                    .with_preset(SkeletonPreset::DetailSection)
                    .with_lines(4),
                theme,
            ),
        ))
        // --- Static (no animation) ---
        .child(group(
            theme,
            "Static (no animation)",
            Skeleton::from_spec(
                SkeletonSpec::new()
                    .with_shape("line")
                    .with_width("10rem")
                    .with_animated(false),
                theme,
            ),
        ))
}

/// Eyebrow-labelled section wrapper.
fn group(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(content)
}

/// rem → logical px (16px root) for container sizing in the specimen only.
fn rem_px(rem: f32) -> f32 {
    rem * 16.0
}
