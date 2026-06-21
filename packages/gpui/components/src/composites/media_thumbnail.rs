//! MediaThumbnail — real GPUI component backed by MediaThumbnailSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{AspectRatio, MediaFit, MediaFrameWidth, MediaKind, MediaPresentation, MediaState, MediaThumbnailSpec};
use poodle_specs::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::presentation::rem_to_px;
use crate::primitives::{Icon, Spinner};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Base frame width (rem) used to derive an intrinsic height from the aspect
/// ratio when no explicit pixel width is given. GPUI has no CSS `aspect-ratio`,
/// so the frame grows to its parent (`w_full`) while the ratio drives a
/// min-height. The `xl` preset (contract `frameWidth="xl"` → `min(100%, 24rem)`)
/// uses 24rem; `fill` uses a 20rem reference so the height is reasonable.
const FRAME_FILL_REF_REM: f32 = 20.0;
const FRAME_XL_REM: f32 = 24.0;

/// A real GPUI media thumbnail component backed by `MediaThumbnailSpec`.
///
/// Renders a framed preview surface with aspect-ratio-derived height, a
/// fallback icon placeholder, optional badge and play indicator, loading/error
/// state posture, and an optional caption.
pub struct MediaThumbnail {
    spec: MediaThumbnailSpec,
    theme: GpuiThemeProvider,
    image_content: Option<AnyElement>,
}

impl std::ops::Deref for MediaThumbnail {
    type Target = MediaThumbnailSpec;
    fn deref(&self) -> &MediaThumbnailSpec {
        &self.spec
    }
}

impl MediaThumbnail {
    pub fn new(kind: MediaKind, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: MediaThumbnailSpec::new(kind),
            theme: theme.clone(),
            image_content: None,
        }
    }

    pub fn from_spec(spec: MediaThumbnailSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            image_content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn kind(mut self, v: MediaKind) -> Self {
        self.spec.kind = v;
        self
    }
    pub fn state(mut self, v: MediaState) -> Self {
        self.spec.state = v;
        self
    }
    pub fn aspect_ratio(mut self, v: AspectRatio) -> Self {
        self.spec.aspect_ratio = v;
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = Some(v.into());
        self
    }
    pub fn meta(mut self, v: impl Into<String>) -> Self {
        self.spec.meta = Some(v.into());
        self
    }
    pub fn badge_label(mut self, v: impl Into<String>) -> Self {
        self.spec.badge_label = Some(v.into());
        self
    }
    pub fn state_title(mut self, v: impl Into<String>) -> Self {
        self.spec.state_title = Some(v.into());
        self
    }
    pub fn state_message(mut self, v: impl Into<String>) -> Self {
        self.spec.state_message = Some(v.into());
        self
    }
    pub fn show_caption(mut self, v: bool) -> Self {
        self.spec.show_caption = v;
        self
    }
    pub fn presentation(mut self, v: MediaPresentation) -> Self {
        self.spec.presentation = v;
        self
    }
    pub fn fit(mut self, v: MediaFit) -> Self {
        self.spec.fit = v;
        self
    }

    pub fn with_image(mut self, image: impl IntoElement) -> Self {
        self.image_content = Some(image.into_any_element());
        self
    }
}

impl IntoElement for MediaThumbnail {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // Frame background: contract §9 is a radial+panel gradient. GPUI has no
        // radial-gradient element, so we paint the flat panel base layer and
        // note the gradient as preview-loop.
        let frame_bg = resolve_color(theme, spec.frame_panel_token());
        let border = resolve_color(theme, spec.frame_border_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let placeholder_color = resolve_color(theme, spec.placeholder_icon_token());

        let body_size = resolve_px(theme, "typography.body.size");
        let label_size = resolve_px(theme, "typography.label.size");
        let caption_size = resolve_px(theme, "typography.caption.size");

        // Frame radius: calc(radius.surface − 0.125rem).
        let frame_radius = (resolve_radius(theme, spec.frame_radius_token())
            - px(rem_to_px(spec.frame_radius_inset_rem())))
        .max(px(0.0));

        let gap_root = resolve_px(theme, "space.stack.sm");
        let gap_state = gap_root;
        let badge_inset = px(rem_to_px(if spec.is_compact() { 0.5 } else { 0.625 }));
        let badge_pad_x = px(rem_to_px(0.625));
        let play_inset = px(rem_to_px(0.625));
        let play_size = px(rem_to_px(2.0));
        let placeholder_pad_y = resolve_px(theme, "space.panel.y");
        let placeholder_pad_x = resolve_px(theme, "space.panel.x");
        let badge_radius = resolve_radius(theme, spec.badge_radius_token());
        let play_radius = resolve_radius(theme, spec.play_radius_token());

        // Frame width / height: derive a reference width then compute height
        // from the aspect ratio (contract §9 Aspect Ratios). `Fill`/`Xl` grow
        // to the parent (`w_full`) with a ratio-derived min-height.
        let (ref_width_px, explicit_width): (f32, Option<Pixels>) = match spec.frame_width {
            MediaFrameWidth::Fill => (rem_to_px(FRAME_FILL_REF_REM), None),
            MediaFrameWidth::Xl => (rem_to_px(FRAME_XL_REM), None),
            MediaFrameWidth::Px(w) => (w, Some(px(w))),
        };
        let derived_h = spec.frame_height_for_width(ref_width_px);
        let frame_h = spec
            .frame_min_height
            .map(|m| m.max(derived_h))
            .unwrap_or(derived_h);
        let frame_h = spec
            .frame_max_height
            .map(|m| frame_h.min(m))
            .unwrap_or(frame_h);

        let root_gap = if spec.is_compact() { px(0.0) } else { gap_root };
        let mut container = div().flex().flex_col().gap(root_gap).overflow_hidden();

        // ── Frame ──────────────────────────────────────────────
        let mut frame = div()
            .map(|el| match explicit_width {
                Some(w) => el.w(w),
                None => el.w_full(),
            })
            .h(px(frame_h))
            .rounded(frame_radius)
            .bg(frame_bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .relative();

        if spec.shows_fallback_copy() {
            // ── State display (loading / error / empty) ─────────
            let state_bg = resolve_color(theme, "color.background.surface").opacity(0.78);
            let mut state = div()
                .w_full()
                .h_full()
                .flex()
                .flex_col()
                .gap(gap_state)
                .map(|el| {
                    if spec.is_compact() {
                        el.items_center().justify_center().p(px(rem_to_px(0.875)))
                    } else {
                        el.items_start()
                            .justify_end()
                            .px(placeholder_pad_x)
                            .py(placeholder_pad_y)
                    }
                })
                .bg(state_bg);

            if spec.state == MediaState::Loading {
                state = state.child(Spinner::from_spec(
                    SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Grid)
                        .with_size(if spec.is_compact() {
                            SpinnerSize::Sm
                        } else {
                            SpinnerSize::Md
                        })
                        .with_tone(SpinnerTone::Accent),
                    theme,
                ));
            }

            state = state.child(
                div()
                    .text_size(body_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(spec.resolved_state_title().to_string()),
            );

            if spec.state_message_visible() {
                state = state.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(spec.resolved_state_message().unwrap().to_string()),
                );
            }

            frame = frame.child(state);
        } else if let Some(image) = self.image_content {
            frame = frame.child(image);
        } else {
            // ── Placeholder fallback icon (contract §9) ─────────
            frame = frame.child(
                div()
                    .w_full()
                    .h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .px(placeholder_pad_x)
                    .py(placeholder_pad_y)
                    .child(
                        Icon::new(spec.fallback_icon(), theme)
                            .with_px_size(rem_to_px(1.75))
                            .with_color(placeholder_color),
                    ),
            );
        }

        // ── Play indicator (audio/video, contract §3) ──────────
        if spec.shows_play_indicator() {
            let play_bg = resolve_color(theme, spec.play_fill_token()).opacity(0.78);
            let play_color = resolve_color(theme, spec.play_color_token());
            frame = frame.child(
                div()
                    .absolute()
                    .left(play_inset)
                    .bottom(play_inset)
                    .w(play_size)
                    .h(play_size)
                    .rounded(play_radius)
                    .bg(play_bg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Icon::new(spec.play_indicator_icon(), theme)
                            .with_px_size(rem_to_px(0.9375))
                            .with_color(play_color),
                    ),
            );
        }

        // ── Badge overlay (contract §9) ────────────────────────
        if let Some(ref badge_label) = spec.badge_label {
            let badge_bg = resolve_color(theme, spec.badge_fill_token()).opacity(0.74);
            let badge_color = resolve_color(theme, spec.badge_text_token());
            frame = frame.child(
                div()
                    .absolute()
                    .top(badge_inset)
                    .right(badge_inset)
                    .h(px(rem_to_px(1.5)))
                    .px(badge_pad_x)
                    .rounded(badge_radius)
                    .bg(badge_bg)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(rem_to_px(0.6875)))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(badge_color)
                    .child(badge_label.to_uppercase()),
            );
        }

        container = container.child(frame);

        // ── Caption (non-compact, contract §3/§9) ──────────────
        if spec.caption_visible() {
            let mut caption = div()
                .w_full()
                .flex()
                .flex_col()
                .gap(px(rem_to_px(0.125)));

            if let Some(ref title) = spec.title {
                caption = caption.child(
                    div()
                        .text_size(caption_size.max(px(rem_to_px(0.875))))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(text_primary)
                        .overflow_x_hidden()
                        .child(title.clone()),
                );
            }

            if let Some(ref meta) = spec.meta {
                caption = caption.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(meta.clone()),
                );
            }

            container = container.child(caption);
        }

        container.into_any_element()
    }
}
