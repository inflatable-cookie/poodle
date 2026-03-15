//! PugPanelSurface — real GPUI component backed by PanelSurfaceSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_workstation::{PanelBodyPadding, PanelScrollMode, PanelSurfaceSpec};

use crate::theme_ext::resolve_color;

/// A real GPUI panel surface container backed by `PanelSurfaceSpec`.
///
/// Provides a panel content area with configurable padding, elevation,
/// scroll mode, and optional header slot.
pub struct PugPanelSurface {
    spec: PanelSurfaceSpec,
    theme: GpuiThemeProvider,
    /// Slot for the panel header.
    header: Option<AnyElement>,
    /// Slot for the panel body content.
    content: Option<AnyElement>,
}

impl PugPanelSurface {
    pub fn new(spec: PanelSurfaceSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            header: None,
            content: None,
        }
    }

    pub fn with_header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for PugPanelSurface {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let bg = resolve_color(theme, spec.background_token());
        let border = resolve_color(theme, "semantic.color.border.default");

        let mut surface = div()
            .flex()
            .flex_col()
            .size_full()
            .bg(bg)
            .border_1()
            .border_color(border);

        // Elevation via shadow
        if spec.is_elevated {
            surface = surface.shadow_md();
        }

        // Header slot
        if spec.has_header {
            if let Some(header) = self.header {
                surface = surface.child(header);
            }
        }

        // Body with padding and scroll
        let padding_scale = spec.resolved_padding_scale();
        let panel_inset = padding_scale.panel_inset();

        let mut body = div().flex_1();

        // Apply padding from the resolved scale
        if let Some(h_token) = panel_inset.horizontal {
            let h_px = theme.resolve_space(h_token);
            body = body.px(px(h_px));
        }
        if let Some(v_token) = panel_inset.vertical {
            let v_px = theme.resolve_space(v_token);
            body = body.py(px(v_px));
        }

        // Scroll mode
        match spec.scroll_mode {
            PanelScrollMode::Panel => {
                body = body.overflow_y_hidden();
            }
            PanelScrollMode::Content => {
                // Content scrolls within itself; body clips overflow.
                body = body.overflow_hidden();
            }
        }

        if let Some(content) = self.content {
            body = body.child(content);
        }

        surface = surface.child(body);

        surface.into_any_element()
    }
}
