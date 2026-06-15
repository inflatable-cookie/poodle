use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{PillAppearance, PillSpec, PillTone, TokenInputSpec};

use crate::presentation::{control_height_rem, control_space_x_rem, rem_to_px};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_radius};
use crate::Pill;

pub struct TokenInput {
    spec: TokenInputSpec,
    theme: GpuiThemeProvider,
}

impl TokenInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(TokenInputSpec::new(), theme)
    }

    pub fn from_spec(spec: TokenInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for TokenInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let surface = resolve_color(theme, "color.background.surface");
        let border = resolve_color(theme, "color.border.subtle");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let fill = color_mix(surface, gpui::transparent_black(), 0.96);
        let opacity = if self.spec.disabled {
            resolve_opacity(theme, "state.opacity.disabled")
        } else {
            1.0
        };

        let mut row = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(rem_to_px(0.375)));
        for token in &self.spec.values {
            row = row.child(Pill::from_spec(
                PillSpec::new()
                    .with_label(token.clone())
                    .with_tone(PillTone::Neutral)
                    .with_appearance(PillAppearance::Subtle),
                theme,
            ));
        }
        row = row.child(
            div()
                .text_color(text_secondary)
                .child(self.spec.placeholder.clone().unwrap_or_default()),
        );

        div()
            .min_h(px(rem_to_px(control_height_rem(self.spec.size))))
            .px(px(rem_to_px(control_space_x_rem(self.spec.density))))
            .py(px(rem_to_px(0.3125)))
            .rounded(resolve_radius(theme, "radius.control"))
            .border_1()
            .border_color(border)
            .bg(fill)
            .opacity(opacity)
            .child(row)
            .into_any_element()
    }
}
