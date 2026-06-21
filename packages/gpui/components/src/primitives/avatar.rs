use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::AvatarSpec;

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
        let spec = &self.spec;
        let size = px(rem_to_px(spec.size_rem()));
        let font_size = px(rem_to_px(spec.font_size_rem()));

        // Tone colors resolved through the spec's token targets + mix ratio
        // (contract §3 + Avatar.svelte), not inlined token strings.
        let base = resolve_color(theme, spec.background_base_token());
        let mix = resolve_color(theme, spec.background_mix_token());
        let bg = color_mix(base, mix, spec.background_mix_ratio());
        let fg = resolve_color(theme, spec.color_token());

        // Circle = border-radius:50% → half the box size; rounded = radius token.
        let radius = if spec.is_circle() {
            px(rem_to_px(spec.circle_radius_rem()))
        } else {
            resolve_radius(theme, spec.radius_token())
        };

        let mut root = div()
            .size(size)
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .flex_none()
            .rounded(radius)
            .bg(bg);

        if spec.has_image() {
            // Image fills the square and uses object-fit: cover (contract §3).
            // Decode/load is owned by the GPUI asset pipeline.
            let src = spec.src.clone().unwrap_or_default();
            root = root.child(
                img(src)
                    .size_full()
                    .object_fit(ObjectFit::Cover)
                    .into_any_element(),
            );
        } else {
            // Initials (or "?") fallback frame.
            root = root
                .text_color(fg)
                .text_size(font_size)
                .font_weight(FontWeight::SEMIBOLD)
                .child(spec.fallback_text());
        }

        root.into_any_element()
    }
}
