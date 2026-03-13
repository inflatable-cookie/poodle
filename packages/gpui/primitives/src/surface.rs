use pug_gpui_tokens::semantic;

use crate::types::{Inset, PaddingScale, SurfaceBorder, SurfaceRole, SurfaceTone};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceSpec {
    pub tone: SurfaceTone,
    pub border: SurfaceBorder,
    pub padding: PaddingScale,
    pub is_elevated: bool,
    pub role: Option<SurfaceRole>,
    pub label: Option<String>,
}

impl Default for SurfaceSpec {
    fn default() -> Self {
        Self {
            tone: SurfaceTone::Panel,
            border: SurfaceBorder::Subtle,
            padding: PaddingScale::Md,
            is_elevated: false,
            role: None,
            label: None,
        }
    }
}

impl SurfaceSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tone(mut self, tone: SurfaceTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_border(mut self, border: SurfaceBorder) -> Self {
        self.border = border;
        self
    }

    pub fn with_padding(mut self, padding: PaddingScale) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_elevation(mut self, is_elevated: bool) -> Self {
        self.is_elevated = is_elevated;
        self
    }

    pub fn with_role(mut self, role: SurfaceRole) -> Self {
        self.role = Some(role);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn resolved_background_token(&self) -> &'static str {
        self.tone.background_token()
    }

    pub fn resolved_border_color(&self) -> Option<&'static str> {
        self.border.color_token()
    }

    pub fn resolved_border_width(&self) -> Option<&'static str> {
        self.border.width_token()
    }

    pub fn resolved_shadow_token(&self) -> &'static str {
        if self.is_elevated || self.tone == SurfaceTone::Elevated {
            semantic::ELEVATION_OVERLAY
        } else {
            semantic::ELEVATION_SURFACE
        }
    }

    pub fn resolved_padding(&self) -> Inset {
        self.padding.panel_inset()
    }
}
