//! TimeAgo — real GPUI component backed by TimeAgoSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::TimeAgoSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI relative time display component backed by `TimeAgoSpec`.
pub struct TimeAgo {
    spec: TimeAgoSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for TimeAgo {
    type Target = TimeAgoSpec;
    fn deref(&self) -> &TimeAgoSpec { &self.spec }
}

impl TimeAgo {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: TimeAgoSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: TimeAgoSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn timestamp(mut self, v: impl Into<String>) -> Self { self.spec.timestamp = v.into(); self }
    pub fn live(mut self, v: bool) -> Self { self.spec.live = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }

}

impl IntoElement for TimeAgo {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_color = resolve_color(theme, spec.text_color_token());

        // Display the timestamp as-is; in a real implementation this would
        // compute relative time from the current instant.
        let display = if spec.timestamp.is_empty() {
            "just now".to_string()
        } else {
            spec.timestamp.clone()
        };

        div()
            .text_xs()
            .text_color(text_color)
            .child(display)
            .into_any_element()
    }
}
