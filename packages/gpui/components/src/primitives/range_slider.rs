//! RangeSlider — real GPUI component backed by RangeSliderSpec.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, Orientation, RangeSliderSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

static RANGE_SLIDER_ID_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// A real GPUI dual-thumb range slider component backed by `RangeSliderSpec`.
pub struct RangeSlider {
    spec: RangeSliderSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for RangeSlider {
    type Target = RangeSliderSpec;
    fn deref(&self) -> &RangeSliderSpec {
        &self.spec
    }
}

impl RangeSlider {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: RangeSliderSpec::default(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: RangeSliderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn low(mut self, v: f64) -> Self {
        self.spec.low = v;
        self
    }
    pub fn high(mut self, v: f64) -> Self {
        self.spec.high = v;
        self
    }
    pub fn min(mut self, v: f64) -> Self {
        self.spec.min = v;
        self
    }
    pub fn max(mut self, v: f64) -> Self {
        self.spec.max = v;
        self
    }
    pub fn step(mut self, v: f64) -> Self {
        self.spec.step = v;
        self
    }
    pub fn orientation(mut self, v: Orientation) -> Self {
        self.spec.orientation = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
}

impl IntoElement for RangeSlider {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let accent = resolve_color(theme, spec.range_fill_token());
        let border = resolve_color(theme, "color.border.default");
        let surface_bg = resolve_color(theme, spec.track_fill_token());
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let stack_gap = resolve_px(theme, "space.stack.sm");

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // Track dimensions scale with effective size
        let track_height_f: f32 = match effective_size {
            ControlSize::Xs => 4.0,
            ControlSize::Sm => 5.0,
            ControlSize::Md => 6.0,
            ControlSize::Lg => 7.0,
            ControlSize::Xl => 8.0,
        };
        let track_height = px(track_height_f);
        let track_radius = px(track_height_f / 2.0);

        // Thumb dimensions
        let thumb_f = theme.resolve_space("size.icon.md");
        let thumb_size = px(thumb_f);
        let thumb_radius = px(thumb_f / 2.0);
        let label_font_size = px(rem_to_px(size_font_rem(effective_size)));

        let norm_low = spec.normalized_low().clamp(0.0, 1.0) as f32;
        let norm_high = spec.normalized_high().clamp(0.0, 1.0) as f32;

        // Track with filled range between low and high thumbs
        let track = div()
            .w_full()
            .h(track_height)
            .rounded(track_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(border.opacity(0.3))
            .relative()
            // Filled range segment
            .child(
                div()
                    .absolute()
                    .top_0()
                    .h_full()
                    .rounded(track_radius)
                    .bg(accent)
                    .left(relative(norm_low))
                    .w(relative(norm_high - norm_low)),
            )
            // Low thumb
            .child(
                div()
                    .absolute()
                    .top(px(-(thumb_f - track_height_f) / 2.0))
                    .left(relative(norm_low))
                    .ml(px(-(thumb_f / 2.0)))
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(accent)
                    .border_1()
                    .border_color(accent)
                    .shadow(vec![gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.2),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(3.0),
                        spread_radius: px(0.0),
                    }]),
            )
            // High thumb
            .child(
                div()
                    .absolute()
                    .top(px(-(thumb_f - track_height_f) / 2.0))
                    .left(relative(norm_high))
                    .ml(px(-(thumb_f / 2.0)))
                    .w(thumb_size)
                    .h(thumb_size)
                    .rounded(thumb_radius)
                    .bg(accent)
                    .border_1()
                    .border_color(accent)
                    .shadow(vec![gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.2),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(3.0),
                        spread_radius: px(0.0),
                    }]),
            );

        // Labels showing low/high values
        let labels = div()
            .flex()
            .items_center()
            .justify_between()
            .text_size(label_font_size)
            .text_color(text_secondary)
            .child(format!("{:.0}", spec.clamped_low()))
            .child(format!("{:.0}", spec.clamped_high()));

        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        let slider_id = SharedString::from(format!(
            "poodle-range-slider-{}",
            RANGE_SLIDER_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));

        let mut wrapper = div()
            .id(slider_id)
            .focusable()
            .w_full()
            .flex()
            .flex_col()
            .gap(stack_gap)
            .child(track)
            .child(labels);

        wrapper = wrapper.focus(move |s| {
            s.border_color(focus_ring)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
        });

        if spec.is_disabled {
            wrapper = wrapper
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        wrapper.into_any_element()
    }
}
