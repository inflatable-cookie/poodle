//! Skeleton — real GPUI component backed by SkeletonSpec.

use std::time::Duration;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{SkeletonPreset, SkeletonSpec};

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_px, resolve_radius};

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

        // Contract §8 shimmer base/highlight resolved from spec token methods.
        // GPUI fills are flat (no CSS gradient), so the static fill is the mid-tone
        // between the contract's outer base (background-elevated 88%) and centre
        // highlight (background-surface 92% white). Known delta: a 3-stop moving
        // gradient is not rendered; the pulse below is the approved animation delta.
        let shimmer_base = {
            let c = resolve_color(theme, spec.shimmer_base_token());
            Hsla { a: c.a * 0.88, ..c }
        };
        let shimmer_highlight = {
            let surface = resolve_color(theme, spec.shimmer_highlight_token());
            // color-mix(surface 92%, white)
            color_mix(surface, white(), 0.92)
        };
        let fill = color_mix(shimmer_highlight, shimmer_base, 0.5);

        // Per-shape radius (contract §8): line→control, circle→pill, block→surface.
        let radius = resolve_radius(theme, spec.radius_token());
        let pill_radius = resolve_radius(theme, "radius.pill");
        let default_height = resolve_px(theme, spec.default_height_token());

        // Contract preset gaps (exact rem, no space-token maps these precisely).
        let line_height = px(rem_to_px(0.875)); // contract base line / cell / list line
        let line_sm_height = px(rem_to_px(0.6875)); // contract skeleton--line-sm

        // ── Preset mode: compound multi-child layouts ──────────
        if let Some(ref preset) = spec.preset {
            // bone with explicit pixel width
            let bone = |w: Pixels, h: Pixels, r: Pixels| -> Div { div().w(w).h(h).rounded(r).bg(fill) };
            // bone with a fractional (percentage) width — contract widths are %.
            let bone_pct =
                |frac: f32, h: Pixels, r: Pixels| -> Div { div().w(relative(frac)).h(h).rounded(r).bg(fill) };

            let animated = spec.is_animated;
            let wrap = |inner: Div| -> AnyElement {
                if animated {
                    inner
                        .with_animation(
                            "skeleton-preset-shimmer",
                            // Contract: 1.6s linear infinite. Technique delta: pulse, not sweep.
                            Animation::new(Duration::from_millis(1600))
                                .repeat()
                                .with_easing(gpui::linear),
                            |el, delta| el.opacity(0.5 + delta * 0.3),
                        )
                        .into_any_element()
                } else {
                    inner.into_any_element()
                }
            };

            return match preset {
                SkeletonPreset::TableRow => {
                    // Contract: 4 cells, height 0.875rem, widths 40/60/60/20%.
                    let row = div()
                        .flex()
                        .flex_row()
                        .gap(px(rem_to_px(0.75)))
                        .py(px(rem_to_px(0.625)))
                        .w_full()
                        .items_center()
                        .child(bone_pct(0.40, line_height, radius))
                        .child(bone_pct(0.60, line_height, radius))
                        .child(bone_pct(0.60, line_height, radius))
                        .child(bone_pct(0.20, line_height, radius));
                    wrap(row)
                }
                SkeletonPreset::Card => {
                    // Contract: block-header 6rem, body 3 lines 80/100/60%, 2 footer pills.
                    let block_radius = (resolve_radius(theme, "radius.surface")
                        - px(rem_to_px(0.375)))
                    .max(px(0.0));
                    let pill_w = px(rem_to_px(3.5));
                    let pill_h = px(rem_to_px(1.25));
                    let card = div()
                        .flex()
                        .flex_col()
                        .gap(px(rem_to_px(0.75)))
                        .p(px(rem_to_px(1.0)))
                        .w_full()
                        .child(
                            div()
                                .w_full()
                                .h(px(rem_to_px(6.0)))
                                .rounded(block_radius)
                                .bg(fill),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(rem_to_px(0.375)))
                                .w_full()
                                .child(bone_pct(0.80, line_height, radius))
                                .child(bone_pct(1.0, line_height, radius))
                                .child(bone_pct(0.60, line_height, radius)),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(rem_to_px(0.5)))
                                .pt(px(rem_to_px(0.25)))
                                .child(bone(pill_w, pill_h, pill_radius))
                                .child(bone(pill_w, pill_h, pill_radius)),
                        );
                    wrap(card)
                }
                SkeletonPreset::ListItem => {
                    // Contract: avatar 2.25rem circle, line 60%, line-sm 40%.
                    let avatar = px(rem_to_px(2.25));
                    let item = div()
                        .flex()
                        .flex_row()
                        .gap(px(rem_to_px(0.75)))
                        .py(px(rem_to_px(0.5)))
                        .w_full()
                        .items_center()
                        .child(bone(avatar, avatar, pill_radius))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(rem_to_px(0.375)))
                                .flex_grow()
                                .child(bone_pct(0.60, line_height, radius))
                                .child(bone_pct(0.40, line_sm_height, radius)),
                        );
                    wrap(item)
                }
                SkeletonPreset::DetailSection => {
                    // Contract: heading 8rem×1rem, then `lines` rows of label(6rem)+value(max 14rem).
                    let mut section = div()
                        .flex()
                        .flex_col()
                        .gap(px(rem_to_px(0.625)))
                        .w_full()
                        .child(
                            // heading: 8rem wide, 1rem tall, 0.25rem bottom margin
                            div()
                                .w(px(rem_to_px(8.0)))
                                .h(px(rem_to_px(1.0)))
                                .rounded(radius)
                                .bg(fill),
                        );
                    for _ in 0..spec.lines {
                        section = section.child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(rem_to_px(1.0)))
                                .items_center()
                                .child(
                                    // label: 6rem, 0.75rem tall, no shrink
                                    div()
                                        .w(px(rem_to_px(6.0)))
                                        .h(px(rem_to_px(0.75)))
                                        .flex_shrink_0()
                                        .rounded(radius)
                                        .bg(fill),
                                )
                                .child(
                                    // value: flex, max-width 14rem, 0.75rem tall
                                    div()
                                        .flex_grow()
                                        .max_w(px(rem_to_px(14.0)))
                                        .h(px(rem_to_px(0.75)))
                                        .rounded(radius)
                                        .bg(fill),
                                ),
                        );
                    }
                    wrap(section)
                }
                SkeletonPreset::AvatarLine => {
                    // Contract: avatar 2.25rem circle + single line 10rem.
                    let avatar = px(rem_to_px(2.25));
                    let row = div()
                        .flex()
                        .flex_row()
                        .gap(px(rem_to_px(0.75)))
                        .items_center()
                        .child(bone(avatar, avatar, pill_radius))
                        .child(bone(px(rem_to_px(10.0)), line_height, radius));
                    wrap(row)
                }
            };
        }

        // ── Single-shape mode ──────────────────────────────────
        // Contract §7 resolved defaults: line 100%×0.875rem, circle 2.5rem²,
        // block 100%×6rem. Width/height props override (parsed as rem or raw px).
        let is_circle = spec.shape == "circle";
        let is_block = spec.shape == "block";

        let mut el = div().rounded(radius).bg(fill);

        // Width
        if let Some(w_px) = spec.width.as_deref().and_then(parse_dim) {
            el = el.w(px(w_px));
        } else if is_circle {
            el = el.w(px(rem_to_px(2.5)));
        } else {
            el = el.w_full();
        }

        // Height
        if let Some(h_px) = spec.height.as_deref().and_then(parse_dim) {
            el = el.h(px(h_px));
        } else if is_circle {
            el = el.h(px(rem_to_px(2.5)));
        } else if is_block {
            el = el.h(px(rem_to_px(6.0)));
        } else {
            el = el.h(default_height);
        }

        // Shimmer animation (technique delta: pulse, not gradient sweep) or static.
        if spec.is_animated {
            el.with_animation(
                "skeleton-shimmer",
                // Contract: 1.6s linear infinite.
                Animation::new(Duration::from_millis(1600))
                    .repeat()
                    .with_easing(gpui::linear),
                |el, delta| el.opacity(0.5 + delta * 0.3),
            )
            .into_any_element()
        } else {
            el.into_any_element()
        }
    }
}

/// Pure white in GPUI Hsla (for the contract's `color-mix(... white)` centre stop).
fn white() -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 1.0,
    }
}

/// Parse a CSS-style dimension ("2rem", "200px", or a bare number as px) into
/// logical pixels. Returns None for unrecognised forms.
fn parse_dim(s: &str) -> Option<f32> {
    let s = s.trim();
    if let Some(rem) = s.strip_suffix("rem") {
        rem.trim().parse::<f32>().ok().map(rem_to_px)
    } else if let Some(p) = s.strip_suffix("px") {
        p.trim().parse::<f32>().ok()
    } else {
        s.parse::<f32>().ok()
    }
}
