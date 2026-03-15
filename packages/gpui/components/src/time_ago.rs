//! PugTimeAgo — real GPUI component backed by TimeAgoSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::TimeAgoSpec;

use crate::theme_ext::resolve_color;

/// A real GPUI relative time display component backed by `TimeAgoSpec`.
pub struct PugTimeAgo {
    spec: TimeAgoSpec,
    theme: GpuiThemeProvider,
}

impl PugTimeAgo {
    pub fn new(spec: TimeAgoSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugTimeAgo {
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
