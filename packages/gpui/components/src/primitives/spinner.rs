use std::time::Duration;

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_components::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::theme_ext::resolve_color;

pub struct Spinner {
    spec: SpinnerSpec,
    theme: GpuiThemeProvider,
    color_override: Option<Hsla>,
}

impl std::ops::Deref for Spinner {
    type Target = SpinnerSpec;
    fn deref(&self) -> &SpinnerSpec { &self.spec }
}

impl Spinner {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: SpinnerSpec::new(), theme: theme.clone(), color_override: None }
    }

    pub fn from_spec(spec: SpinnerSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), color_override: None }
    }

    pub fn variant(mut self, variant: SpinnerVariant) -> Self { self.spec.variant = variant; self }
    pub fn size(mut self, size: SpinnerSize) -> Self { self.spec.size = size; self }
    pub fn tone(mut self, tone: SpinnerTone) -> Self { self.spec.tone = tone; self }
    pub fn aria_label(mut self, aria_label: impl Into<String>) -> Self { self.spec.aria_label = Some(aria_label.into()); self }

    pub fn with_color(mut self, color: Hsla) -> Self {
        self.color_override = Some(color);
        self
    }
}

impl IntoElement for Spinner {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let color = if let Some(color) = self.color_override {
            color
        } else if let Some(token) = spec.tone_color_token() {
            resolve_color(theme, token)
        } else {
            resolve_color(theme, "color.text.primary")
        };

        match spec.variant {
            SpinnerVariant::Ring => {
                let size = px(spec.size_px());
                svg()
                    .path(SharedString::from("assets/icons/spinner.svg"))
                    .size(size)
                    .flex_shrink_0()
                    .text_color(color)
                    .with_animation(
                        "spinner-ring",
                        Animation::new(Duration::from_millis(800)).repeat(),
                        |svg, delta| {
                            svg.with_transformation(
                                Transformation::rotate(gpui::radians(delta * std::f32::consts::TAU))
                            )
                        },
                    )
                    .into_any_element()
            }
            SpinnerVariant::Grid => {
                let (cell_size, gap) = match spec.size {
                    SpinnerSize::Xs => (px(2.0), px(1.0)),
                    SpinnerSize::Sm => (px(2.5), px(1.25)),
                    SpinnerSize::Md => (px(3.25), px(1.5)),
                    SpinnerSize::Lg => (px(4.5), px(2.0)),
                    SpinnerSize::Xl => (px(6.0), px(2.5)),
                };

                let mut col = div().flex().flex_col().gap(gap);

                for row_idx in 0..3 {
                    let mut row = div().flex().gap(gap);

                    for col_idx in 0..2 {
                        let active_steps: &'static [f32] = match (row_idx, col_idx) {
                            (0, 0) => &[0.0],
                            (0, 1) => &[1.0],
                            (1, 1) => &[2.0, 6.0],
                            (1, 0) => &[3.0, 7.0],
                            (2, 0) => &[4.0],
                            (2, 1) => &[5.0],
                            _ => unreachable!(),
                        };
                        row = row.child(
                            div()
                                .w(cell_size)
                                .h(cell_size)
                                .rounded(px(2.0))
                                .bg(color)
                                .with_animation(
                                    SharedString::from(format!("spinner-grid-{}-{}", row_idx, col_idx)),
                                    Animation::new(Duration::from_millis(1240))
                                        .repeat()
                                        .with_easing(gpui::linear),
                                    move |el, delta| {
                                        let step_progress = (delta * 8.0) % 8.0;
                                        let nearest = active_steps
                                            .iter()
                                            .map(|step| {
                                                let raw = (step_progress - *step).abs();
                                                raw.min(8.0 - raw)
                                            })
                                            .fold(f32::INFINITY, f32::min);
                                        let normalized = (nearest / 1.6).min(1.0);
                                        let smooth = 1.0 - (normalized * normalized * (3.0 - 2.0 * normalized));
                                        let opacity = 0.2 + smooth * 0.56;
                                        el.opacity(opacity)
                                    },
                                ),
                        );
                    }

                    col = col.child(row);
                }

                col.into_any_element()
            }
        }
    }
}
