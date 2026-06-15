use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{AvatarShape, AvatarSpec, AvatarTone};

use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_radius};

pub struct Avatar {
    spec: AvatarSpec,
    theme: GpuiThemeProvider,
}

impl Avatar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(AvatarSpec::new(), theme)
    }

    pub fn from_spec(spec: AvatarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for Avatar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let size = px(rem_to_px(self.spec.size_rem()));
        let font_size = px(rem_to_px(self.spec.font_size_rem()));
        let surface = resolve_color(theme, "color.background.surface");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");
        let (bg, fg) = match self.spec.tone {
            AvatarTone::Neutral => (color_mix(surface, text_primary, 0.82), text_secondary),
            AvatarTone::Accent => (color_mix(accent, surface, 0.76), text_primary),
        };
        let radius = match self.spec.shape {
            AvatarShape::Circle => px(rem_to_px(999.0)),
            AvatarShape::Rounded => resolve_radius(theme, "radius.control"),
        };

        div()
            .size(size)
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .flex_none()
            .rounded(radius)
            .bg(bg)
            .text_color(fg)
            .text_size(font_size)
            .font_weight(FontWeight::SEMIBOLD)
            .child(self.spec.fallback_text())
            .into_any_element()
    }
}
