//! Skeleton — real GPUI component backed by SkeletonSpec.

use std::time::Duration;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{SkeletonPreset, SkeletonSpec};

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI skeleton placeholder component backed by `SkeletonSpec`.
pub struct Skeleton {
    spec: SkeletonSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Skeleton {
    type Target = SkeletonSpec;
    fn deref(&self) -> &SkeletonSpec {
        &self.spec
    }
}

impl Skeleton {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SkeletonSpec::new(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: SkeletonSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn shape(mut self, v: impl Into<String>) -> Self {
        self.spec.shape = v.into();
        self
    }
    pub fn width(mut self, v: impl Into<String>) -> Self {
        self.spec.width = Some(v.into());
        self
    }
    pub fn height(mut self, v: impl Into<String>) -> Self {
        self.spec.height = Some(v.into());
        self
    }
    pub fn animated(mut self, v: bool) -> Self {
        self.spec.is_animated = v;
        self
    }
    pub fn preset(mut self, v: SkeletonPreset) -> Self {
        self.spec.preset = Some(v);
        self
    }
    pub fn lines(mut self, v: u32) -> Self {
        self.spec.lines = v;
        self
    }
}

impl IntoElement for Skeleton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let pill_radius = resolve_radius(theme, "radius.pill");
        let default_height = resolve_px(theme, spec.default_height_token());
        let gap_sm = resolve_px(theme, "space.stack.sm");
        let gap_md = resolve_px(theme, "space.stack.md");

        // ── Preset mode: compound multi-child layouts ──────────
        if let Some(ref preset) = spec.preset {
            let bone =
                |w: Pixels, h: Pixels, r: Pixels| -> Div { div().w(w).h(h).rounded(r).bg(fill) };

            let animated = spec.is_animated;
            let wrap = |inner: Div| -> AnyElement {
                if animated {
                    inner
                        .with_animation(
                            "skeleton-preset-shimmer",
                            Animation::new(Duration::from_millis(1500))
                                .repeat()
                                .with_easing(gpui::ease_in_out),
                            |el, delta| el.opacity(0.3 + delta * 0.4),
                        )
                        .into_any_element()
                } else {
                    inner.opacity(0.5).into_any_element()
                }
            };

            return match preset {
                SkeletonPreset::TableRow => {
                    let row = div()
                        .flex()
                        .flex_row()
                        .gap(gap_md)
                        .w_full()
                        .items_center()
                        .child(bone(px(120.0), default_height, radius))
                        .child(bone(px(80.0), default_height, radius))
                        .child(
                            div()
                                .flex_grow()
                                .child(bone(px(200.0), default_height, radius)),
                        )
                        .child(bone(px(60.0), default_height, radius));
                    wrap(row)
                }
                SkeletonPreset::Card => {
                    let card = div()
                        .flex()
                        .flex_col()
                        .gap(gap_sm)
                        .w_full()
                        .child(bone(px(9999.0), px(120.0), radius)) // image area
                        .child(bone(px(180.0), default_height, radius)) // title
                        .child(bone(px(240.0), default_height, radius)); // description
                    wrap(card)
                }
                SkeletonPreset::ListItem => {
                    let item = div()
                        .flex()
                        .flex_row()
                        .gap(gap_md)
                        .w_full()
                        .items_center()
                        .child(bone(px(32.0), px(32.0), pill_radius)) // avatar
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(gap_sm)
                                .flex_grow()
                                .child(bone(px(160.0), default_height, radius))
                                .child(bone(px(100.0), default_height * 0.85, radius)),
                        );
                    wrap(item)
                }
                SkeletonPreset::DetailSection => {
                    let mut section = div().flex().flex_col().gap(gap_sm).w_full().child(bone(
                        px(140.0),
                        default_height * 1.2,
                        radius,
                    )); // heading
                    for _ in 0..spec.lines {
                        section = section.child(bone(px(9999.0), default_height, radius));
                    }
                    wrap(section)
                }
                SkeletonPreset::AvatarLine => {
                    let row = div()
                        .flex()
                        .flex_row()
                        .gap(gap_md)
                        .items_center()
                        .child(bone(px(40.0), px(40.0), pill_radius)) // avatar circle
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(gap_sm)
                                .child(bone(px(120.0), default_height, radius))
                                .child(bone(px(80.0), default_height * 0.85, radius)),
                        );
                    wrap(row)
                }
            };
        }

        // ── Single-shape mode ──────────────────────────────────
        let mut el = div().rounded(radius).bg(fill);

        // Dimensions
        if let Some(ref w) = spec.width {
            if let Ok(val) = w.parse::<f32>() {
                el = el.w(px(val));
            } else {
                el = el.w_full();
            }
        } else {
            el = el.w_full();
        }

        if let Some(ref h) = spec.height {
            if let Ok(val) = h.parse::<f32>() {
                el = el.h(px(val));
            } else {
                el = el.h(default_height);
            }
        } else {
            el = el.h(default_height);
        }

        // Shimmer animation or static opacity
        if spec.is_animated {
            el.with_animation(
                "skeleton-shimmer",
                Animation::new(Duration::from_millis(1500))
                    .repeat()
                    .with_easing(gpui::ease_in_out),
                |el, delta| {
                    // Pulse opacity between 0.3 and 0.7
                    let opacity = 0.3 + delta * 0.4;
                    el.opacity(opacity)
                },
            )
            .into_any_element()
        } else {
            el.opacity(0.5).into_any_element()
        }
    }
}
