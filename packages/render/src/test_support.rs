//! Token-backed theme used by renderer unit tests.

use poodle_adapter::ThemeProvider;
use poodle_headless::color::{
    apply_neutral_contrast, hex_to_rgb, is_contrast_scaled_token, is_valid_hex, oklab_lightness,
};
use poodle_style::StyleDescriptor;
use poodle_tokens::{
    semantic as tokens,
    themes::{ThemeDefinition, ECLIPSE},
    typed::{semantic as typed, ColorValue},
};

#[derive(Debug)]
pub(crate) struct TestThemeProvider {
    overrides: &'static [(&'static str, &'static str)],
    canvas_lightness: f64,
}

impl TestThemeProvider {
    pub(crate) fn from_theme(theme: &ThemeDefinition) -> Self {
        let mut provider = Self {
            overrides: theme.overrides,
            canvas_lightness: 0.0,
        };
        let canvas = provider
            .raw_color(tokens::COLOR_BACKGROUND_CANVAS)
            .expect("theme defines a canvas color");
        provider.canvas_lightness =
            oklab_lightness(canvas.0.into(), canvas.1.into(), canvas.2.into());
        provider
    }

    fn raw_color(&self, token: &str) -> Option<ColorValue> {
        if token.starts_with('#') || token.starts_with("rgba(") {
            return parse_color(token);
        }

        let path = token.strip_prefix("color.").unwrap_or(token);
        self.overrides
            .iter()
            .find(|(key, _)| key.strip_prefix("color.") == Some(path))
            .and_then(|(_, value)| parse_color(value))
            .or_else(|| semantic_color(path))
    }
}

pub(crate) fn test_theme() -> TestThemeProvider {
    TestThemeProvider::from_theme(&ECLIPSE)
}

impl ThemeProvider for TestThemeProvider {
    fn resolve_color(&self, token: &str) -> ColorValue {
        self.try_resolve_color(token)
            .unwrap_or(ColorValue(0.0, 0.0, 0.0, 1.0))
    }

    fn try_resolve_color(&self, token: &str) -> Option<ColorValue> {
        let color = self.raw_color(token)?;
        if !is_contrast_scaled_token(token) {
            return Some(color);
        }
        let (r, g, b, a) = apply_neutral_contrast(
            color.0.into(),
            color.1.into(),
            color.2.into(),
            color.3.into(),
            self.canvas_lightness,
            0.5,
        );
        Some(ColorValue(r as f32, g as f32, b as f32, a as f32))
    }

    fn resolve_space(&self, token: &str) -> f32 {
        if let Some(rem) = token.strip_suffix("rem") {
            return rem.parse::<f32>().unwrap_or_default() * 16.0;
        }
        let token = token.strip_suffix("px").unwrap_or(token);
        semantic_number(token).unwrap_or_default()
    }

    fn resolve_border_width(&self, token: &str) -> f32 {
        self.resolve_space(token)
    }

    fn resolve_radius(&self, token: &str) -> f32 {
        self.resolve_space(token)
    }

    fn resolve_opacity(&self, token: &str) -> f32 {
        self.try_resolve_opacity(token)
            .unwrap_or_else(|| StyleDescriptor::new().opacity)
    }

    fn try_resolve_opacity(&self, token: &str) -> Option<f32> {
        semantic_number(token)
    }
}

fn parse_color(value: &str) -> Option<ColorValue> {
    if let Some(inner) = value
        .strip_prefix("rgba(")
        .and_then(|s| s.strip_suffix(')'))
    {
        let channels: Vec<_> = inner.split(',').map(str::trim).collect();
        let [r, g, b, a] = channels.as_slice() else {
            return None;
        };
        return Some(ColorValue(
            r.parse::<f32>().ok()? / 255.0,
            g.parse::<f32>().ok()? / 255.0,
            b.parse::<f32>().ok()? / 255.0,
            a.parse().ok()?,
        ));
    }
    if !is_valid_hex(value) {
        return None;
    }
    let (rgb, alpha) = hex_to_rgb(value)?;
    Some(ColorValue(
        f32::from(rgb.r) / 255.0,
        f32::from(rgb.g) / 255.0,
        f32::from(rgb.b) / 255.0,
        alpha.unwrap_or(1.0) as f32,
    ))
}

fn semantic_color(path: &str) -> Option<ColorValue> {
    Some(match path {
        "accent.base" => typed::COLOR_ACCENT_BASE,
        "accent.hover" => typed::COLOR_ACCENT_HOVER,
        "accent.focusRing" => typed::COLOR_ACCENT_FOCUS_RING,
        "background.canvas" => typed::COLOR_BACKGROUND_CANVAS,
        "background.surface" => typed::COLOR_BACKGROUND_SURFACE,
        "background.panel" => typed::COLOR_BACKGROUND_PANEL,
        "background.overlay" => typed::COLOR_BACKGROUND_OVERLAY,
        "background.elevated" => typed::COLOR_BACKGROUND_ELEVATED,
        "text.primary" => typed::COLOR_TEXT_PRIMARY,
        "text.tertiary" => typed::COLOR_TEXT_TERTIARY,
        "text.secondary" => typed::COLOR_TEXT_SECONDARY,
        "text.inverse" => typed::COLOR_TEXT_INVERSE,
        "border.subtle" => typed::COLOR_BORDER_SUBTLE,
        "border.default" => typed::COLOR_BORDER_DEFAULT,
        "border.strong" => typed::COLOR_BORDER_STRONG,
        "status.success" => typed::COLOR_STATUS_SUCCESS,
        "status.warning" => typed::COLOR_STATUS_WARNING,
        "status.danger" => typed::COLOR_STATUS_DANGER,
        "status.info" => typed::COLOR_STATUS_INFO,
        "icon.primary" => typed::COLOR_ICON_PRIMARY,
        "icon.muted" => typed::COLOR_ICON_MUTED,
        _ => return None,
    })
}

fn semantic_number(token: &str) -> Option<f32> {
    Some(match token {
        tokens::SIZE_CONTROL_HEIGHT => typed::SIZE_CONTROL_HEIGHT.0,
        tokens::SIZE_CONTROL_MIN_WIDTH => typed::SIZE_CONTROL_MIN_WIDTH.0,
        tokens::SIZE_ICON_XS => typed::SIZE_ICON_XS.0,
        tokens::SIZE_ICON_SM => typed::SIZE_ICON_SM.0,
        tokens::SIZE_ICON_MD => typed::SIZE_ICON_MD.0,
        tokens::SIZE_ICON_LG => typed::SIZE_ICON_LG.0,
        tokens::SIZE_ICON_XL => typed::SIZE_ICON_XL.0,
        tokens::SIZE_PANEL_HEADER => typed::SIZE_PANEL_HEADER.0,
        tokens::SIZE_LIST_GRID_MIN_ITEM_WIDTH => typed::SIZE_LIST_GRID_MIN_ITEM_WIDTH.0,
        tokens::SIZE_MENU_MAX_HEIGHT => typed::SIZE_MENU_MAX_HEIGHT.0,
        tokens::SIZE_MENU_MIN_WIDTH => typed::SIZE_MENU_MIN_WIDTH.0,
        tokens::SIZE_POPOVER_MAX_WIDTH => typed::SIZE_POPOVER_MAX_WIDTH.0,
        tokens::SIZE_HOVER_CARD_MAX_WIDTH => typed::SIZE_HOVER_CARD_MAX_WIDTH.0,
        tokens::SIZE_SELECT_MIN_WIDTH => typed::SIZE_SELECT_MIN_WIDTH.0,
        tokens::SIZE_DATE_TIME_RANGE_PICKER_MIN_WIDTH => {
            typed::SIZE_DATE_TIME_RANGE_PICKER_MIN_WIDTH.0
        }
        tokens::SIZE_FILE_UPLOAD_DROP_ZONE_MIN_HEIGHT => {
            typed::SIZE_FILE_UPLOAD_DROP_ZONE_MIN_HEIGHT.0
        }
        tokens::SPACE_STACK_SM => typed::SPACE_STACK_SM.0,
        tokens::SPACE_STACK_MD => typed::SPACE_STACK_MD.0,
        tokens::SPACE_STACK_LG => typed::SPACE_STACK_LG.0,
        tokens::SPACE_INLINE_XS => typed::SPACE_INLINE_XS.0,
        tokens::SPACE_INLINE_SM => typed::SPACE_INLINE_SM.0,
        tokens::SPACE_INLINE_MD => typed::SPACE_INLINE_MD.0,
        tokens::SPACE_INLINE_LG => typed::SPACE_INLINE_LG.0,
        tokens::SPACE_PANEL_X => typed::SPACE_PANEL_X.0,
        tokens::SPACE_PANEL_Y => typed::SPACE_PANEL_Y.0,
        tokens::SPACE_CONTROL_X => typed::SPACE_CONTROL_X.0,
        tokens::SPACE_CONTROL_Y => typed::SPACE_CONTROL_Y.0,
        tokens::SPACE_BUTTON_GAP => typed::SPACE_BUTTON_GAP.0,
        tokens::SPACE_BUTTON_ICON_INSET => typed::SPACE_BUTTON_ICON_INSET.0,
        tokens::RADIUS_CONTROL => typed::RADIUS_CONTROL.0,
        tokens::RADIUS_SURFACE => typed::RADIUS_SURFACE.0,
        tokens::RADIUS_PILL => typed::RADIUS_PILL.0,
        tokens::BORDER_WIDTH_DEFAULT | "borderWidth.default" => typed::BORDER_WIDTH_DEFAULT.0,
        tokens::BORDER_WIDTH_FOCUS | "borderWidth.focus" => typed::BORDER_WIDTH_FOCUS.0,
        tokens::TYPOGRAPHY_BODY_LINE_HEIGHT => typed::TYPOGRAPHY_BODY_LINE_HEIGHT.0,
        tokens::TYPOGRAPHY_BODY_SIZE => typed::TYPOGRAPHY_BODY_SIZE.0,
        tokens::TYPOGRAPHY_LABEL_LINE_HEIGHT => typed::TYPOGRAPHY_LABEL_LINE_HEIGHT.0,
        tokens::TYPOGRAPHY_LABEL_WEIGHT => typed::TYPOGRAPHY_LABEL_WEIGHT,
        tokens::TYPOGRAPHY_LABEL_SIZE => typed::TYPOGRAPHY_LABEL_SIZE.0,
        tokens::TYPOGRAPHY_CAPTION_LINE_HEIGHT => typed::TYPOGRAPHY_CAPTION_LINE_HEIGHT.0,
        tokens::TYPOGRAPHY_CAPTION_WEIGHT => typed::TYPOGRAPHY_CAPTION_WEIGHT,
        tokens::TYPOGRAPHY_CAPTION_SIZE => typed::TYPOGRAPHY_CAPTION_SIZE.0,
        tokens::TYPOGRAPHY_COUNTER_SIZE => typed::TYPOGRAPHY_COUNTER_SIZE.0,
        tokens::TYPOGRAPHY_HEADING_LINE_HEIGHT => typed::TYPOGRAPHY_HEADING_LINE_HEIGHT.0,
        tokens::TYPOGRAPHY_HEADING_WEIGHT => typed::TYPOGRAPHY_HEADING_WEIGHT,
        tokens::TYPOGRAPHY_HEADING_SIZE => typed::TYPOGRAPHY_HEADING_SIZE.0,
        "typography.heading" => typed::TYPOGRAPHY_HEADING_SIZE.0,
        tokens::TYPOGRAPHY_CODE_LINE_HEIGHT => typed::TYPOGRAPHY_CODE_LINE_HEIGHT.0,
        tokens::TYPOGRAPHY_CODE_SIZE => typed::TYPOGRAPHY_CODE_SIZE.0,
        tokens::STATE_OPACITY_DISABLED => typed::STATE_OPACITY_DISABLED,
        tokens::STATE_OPACITY_MUTED => typed::STATE_OPACITY_MUTED,
        _ => return token.parse().ok(),
    })
}
