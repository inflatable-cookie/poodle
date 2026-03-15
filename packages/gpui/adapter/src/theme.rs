//! GPUI theme integration.
//!
//! Implements the `ThemeProvider` trait from `pug-adapter` by resolving
//! string token paths to typed values using the `pug-tokens::typed` module.
//!
//! The token paths passed to `resolve_*` methods are the `&'static str`
//! constants from `pug_tokens::semantic` (e.g., `"#2d86f3"`, `"0.0625rem"`).
//! This provider parses them into the corresponding typed values.
//!
//! In a full GPUI integration, this would also consult the active GPUI
//! `Theme` struct for runtime theme overrides. The current implementation
//! resolves directly from the static typed token constants.

use pug_adapter::ThemeProvider;
use pug_tokens::typed::{self, ColorValue};

/// GPUI theme provider backed by the typed token module.
///
/// Resolves token string values to typed numeric values suitable for
/// GPUI's styling API. All colors become `[f32; 4]` RGBA, all sizes
/// become `f32` pixel values.
#[derive(Debug, Clone)]
pub struct GpuiThemeProvider {
    /// Scale factor for pixel values (default 1.0, set to 2.0 for Retina).
    pub scale_factor: f32,
    /// Active theme overrides (semantic path → value string).
    overrides: Vec<(&'static str, &'static str)>,
    /// Active theme name.
    pub theme_name: String,
}

impl Default for GpuiThemeProvider {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            overrides: Vec::new(),
            theme_name: "default".to_string(),
        }
    }
}

impl GpuiThemeProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_scale_factor(mut self, factor: f32) -> Self {
        self.scale_factor = factor;
        self
    }

    /// Apply a theme definition's overrides.
    pub fn with_theme(mut self, theme: &pug_tokens::themes::ThemeDefinition) -> Self {
        self.overrides = theme.overrides.to_vec();
        self.theme_name = theme.name.to_string();
        self
    }

    /// Parse a CSS hex color string into a `ColorValue`.
    ///
    /// Handles `#RRGGBB` and `#RRGGBBAA` formats.
    fn parse_hex_color(hex: &str) -> Option<ColorValue> {
        let hex = hex.strip_prefix('#')?;
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
            Some(ColorValue(r, g, b, 1.0))
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0;
            Some(ColorValue(r, g, b, a))
        } else {
            None
        }
    }

    /// Parse a CSS `rgba(r, g, b, a)` string into a `ColorValue`.
    fn parse_rgba_color(s: &str) -> Option<ColorValue> {
        let inner = s.strip_prefix("rgba(")?.strip_suffix(')')?;
        let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
        if parts.len() != 4 {
            return None;
        }
        let r = parts[0].parse::<f32>().ok()? / 255.0;
        let g = parts[1].parse::<f32>().ok()? / 255.0;
        let b = parts[2].parse::<f32>().ok()? / 255.0;
        let a = parts[3].parse::<f32>().ok()?;
        Some(ColorValue(r, g, b, a))
    }

    /// Parse a CSS dimension string (`rem` or `px`) into pixel value.
    ///
    /// Base size: 16px per rem.
    fn parse_dimension(s: &str) -> Option<f32> {
        if let Some(rem_str) = s.strip_suffix("rem") {
            rem_str.parse::<f32>().ok().map(|v| v * 16.0)
        } else if let Some(px_str) = s.strip_suffix("px") {
            px_str.parse::<f32>().ok()
        } else {
            s.parse::<f32>().ok()
        }
    }

    /// Resolve a color token string to a `ColorValue`.
    ///
    /// Checks theme overrides first, then accepts hex/rgba formats,
    /// then falls back to typed constant lookup.
    pub fn resolve_color_value(&self, token: &str) -> ColorValue {
        // Check theme overrides first (token is a semantic path like "semantic.color.accent.base")
        for &(path, value) in &self.overrides {
            if path == token {
                if let Some(color) = Self::parse_hex_color(value) {
                    return color;
                }
                if let Some(color) = Self::parse_rgba_color(value) {
                    return color;
                }
            }
        }
        if let Some(color) = Self::parse_hex_color(token) {
            return color;
        }
        if let Some(color) = Self::parse_rgba_color(token) {
            return color;
        }
        // Match by semantic path name (e.g., "semantic.color.background.canvas")
        // or by raw token value (e.g., "#e7eef5"). Both forms are used: the
        // preview app passes paths, while spec resolved_*_token() methods emit
        // raw values.
        match token {
            // Path-based lookup
            "semantic.color.background.canvas" => typed::semantic::COLOR_BACKGROUND_CANVAS,
            "semantic.color.background.surface" => typed::semantic::COLOR_BACKGROUND_SURFACE,
            "semantic.color.background.panel" => typed::semantic::COLOR_BACKGROUND_PANEL,
            "semantic.color.background.elevated" => typed::semantic::COLOR_BACKGROUND_ELEVATED,
            "semantic.color.background.overlay" => typed::semantic::COLOR_BACKGROUND_OVERLAY,
            "semantic.color.text.primary" => typed::semantic::COLOR_TEXT_PRIMARY,
            "semantic.color.text.secondary" => typed::semantic::COLOR_TEXT_SECONDARY,
            "semantic.color.text.inverse" => typed::semantic::COLOR_TEXT_INVERSE,
            "semantic.color.border.subtle" => typed::semantic::COLOR_BORDER_SUBTLE,
            "semantic.color.border.default" => typed::semantic::COLOR_BORDER_DEFAULT,
            "semantic.color.border.strong" => typed::semantic::COLOR_BORDER_STRONG,
            "semantic.color.accent.base" => typed::semantic::COLOR_ACCENT_BASE,
            "semantic.color.accent.hover" => typed::semantic::COLOR_ACCENT_HOVER,
            "semantic.color.accent.focusRing" => typed::semantic::COLOR_ACCENT_FOCUS_RING,
            "semantic.color.status.success" => typed::semantic::COLOR_STATUS_SUCCESS,
            "semantic.color.status.warning" => typed::semantic::COLOR_STATUS_WARNING,
            "semantic.color.status.danger" => typed::semantic::COLOR_STATUS_DANGER,
            "semantic.color.icon.primary" => typed::semantic::COLOR_ICON_PRIMARY,
            "semantic.color.icon.muted" => typed::semantic::COLOR_ICON_MUTED,
            // Raw value lookup (hex/rgba values emitted by spec token methods)
            t if t == pug_tokens::semantic::COLOR_BACKGROUND_CANVAS => typed::semantic::COLOR_BACKGROUND_CANVAS,
            t if t == pug_tokens::semantic::COLOR_BACKGROUND_SURFACE => typed::semantic::COLOR_BACKGROUND_SURFACE,
            t if t == pug_tokens::semantic::COLOR_BACKGROUND_PANEL => typed::semantic::COLOR_BACKGROUND_PANEL,
            t if t == pug_tokens::semantic::COLOR_BACKGROUND_ELEVATED => typed::semantic::COLOR_BACKGROUND_ELEVATED,
            t if t == pug_tokens::semantic::COLOR_BACKGROUND_OVERLAY => typed::semantic::COLOR_BACKGROUND_OVERLAY,
            t if t == pug_tokens::semantic::COLOR_TEXT_PRIMARY => typed::semantic::COLOR_TEXT_PRIMARY,
            t if t == pug_tokens::semantic::COLOR_TEXT_SECONDARY => typed::semantic::COLOR_TEXT_SECONDARY,
            t if t == pug_tokens::semantic::COLOR_TEXT_INVERSE => typed::semantic::COLOR_TEXT_INVERSE,
            t if t == pug_tokens::semantic::COLOR_BORDER_SUBTLE => typed::semantic::COLOR_BORDER_SUBTLE,
            t if t == pug_tokens::semantic::COLOR_BORDER_DEFAULT => typed::semantic::COLOR_BORDER_DEFAULT,
            t if t == pug_tokens::semantic::COLOR_BORDER_STRONG => typed::semantic::COLOR_BORDER_STRONG,
            t if t == pug_tokens::semantic::COLOR_ACCENT_BASE => typed::semantic::COLOR_ACCENT_BASE,
            t if t == pug_tokens::semantic::COLOR_ACCENT_HOVER => typed::semantic::COLOR_ACCENT_HOVER,
            t if t == pug_tokens::semantic::COLOR_ACCENT_FOCUS_RING => typed::semantic::COLOR_ACCENT_FOCUS_RING,
            t if t == pug_tokens::semantic::COLOR_STATUS_SUCCESS => typed::semantic::COLOR_STATUS_SUCCESS,
            t if t == pug_tokens::semantic::COLOR_STATUS_WARNING => typed::semantic::COLOR_STATUS_WARNING,
            t if t == pug_tokens::semantic::COLOR_STATUS_DANGER => typed::semantic::COLOR_STATUS_DANGER,
            t if t == pug_tokens::semantic::COLOR_ICON_PRIMARY => typed::semantic::COLOR_ICON_PRIMARY,
            t if t == pug_tokens::semantic::COLOR_ICON_MUTED => typed::semantic::COLOR_ICON_MUTED,
            _ => ColorValue(0.0, 0.0, 0.0, 1.0),
        }
    }

    /// Resolve a space/size token string to a pixel value.
    pub fn resolve_space_value(&self, token: &str) -> f32 {
        if let Some(px) = Self::parse_dimension(token) {
            return px;
        }
        // Match known semantic space tokens
        match token {
            t if t == pug_tokens::semantic::SPACE_STACK_SM => typed::semantic::SPACE_STACK_SM.as_f32(),
            t if t == pug_tokens::semantic::SPACE_STACK_MD => typed::semantic::SPACE_STACK_MD.as_f32(),
            t if t == pug_tokens::semantic::SPACE_STACK_LG => typed::semantic::SPACE_STACK_LG.as_f32(),
            t if t == pug_tokens::semantic::SPACE_INLINE_SM => typed::semantic::SPACE_INLINE_SM.as_f32(),
            t if t == pug_tokens::semantic::SPACE_INLINE_MD => typed::semantic::SPACE_INLINE_MD.as_f32(),
            t if t == pug_tokens::semantic::SPACE_INLINE_LG => typed::semantic::SPACE_INLINE_LG.as_f32(),
            t if t == pug_tokens::semantic::SPACE_PANEL_X => typed::semantic::SPACE_PANEL_X.as_f32(),
            t if t == pug_tokens::semantic::SPACE_PANEL_Y => typed::semantic::SPACE_PANEL_Y.as_f32(),
            t if t == pug_tokens::semantic::SPACE_CONTROL_X => typed::semantic::SPACE_CONTROL_X.as_f32(),
            t if t == pug_tokens::semantic::SPACE_CONTROL_Y => typed::semantic::SPACE_CONTROL_Y.as_f32(),
            t if t == pug_tokens::semantic::SIZE_CONTROL_HEIGHT => typed::semantic::SIZE_CONTROL_HEIGHT.as_f32(),
            t if t == pug_tokens::semantic::SIZE_CONTROL_MIN_WIDTH => typed::semantic::SIZE_CONTROL_MIN_WIDTH.as_f32(),
            t if t == pug_tokens::semantic::SIZE_ICON_SM => typed::semantic::SIZE_ICON_SM.as_f32(),
            t if t == pug_tokens::semantic::SIZE_ICON_MD => typed::semantic::SIZE_ICON_MD.as_f32(),
            t if t == pug_tokens::semantic::SIZE_ICON_LG => typed::semantic::SIZE_ICON_LG.as_f32(),
            t if t == pug_tokens::semantic::SIZE_PANEL_HEADER => typed::semantic::SIZE_PANEL_HEADER.as_f32(),
            t if t == pug_tokens::semantic::ICON_SIZE_DEFAULT => typed::semantic::ICON_SIZE_DEFAULT.as_f32(),
            _ => 0.0,
        }
    }
}

impl ThemeProvider for GpuiThemeProvider {
    fn resolve_color(&self, token: &str) -> ColorValue {
        self.resolve_color_value(token)
    }

    fn resolve_space(&self, token: &str) -> f32 {
        self.resolve_space_value(token)
    }

    fn resolve_border_width(&self, token: &str) -> f32 {
        if let Some(px) = Self::parse_dimension(token) {
            return px;
        }
        match token {
            t if t == pug_tokens::semantic::BORDER_WIDTH_DEFAULT => typed::semantic::BORDER_WIDTH_DEFAULT.as_f32(),
            t if t == pug_tokens::semantic::BORDER_WIDTH_FOCUS => typed::semantic::BORDER_WIDTH_FOCUS.as_f32(),
            _ => 0.0,
        }
    }

    fn resolve_radius(&self, token: &str) -> f32 {
        if let Some(px) = Self::parse_dimension(token) {
            return px;
        }
        match token {
            t if t == pug_tokens::semantic::RADIUS_CONTROL => typed::semantic::RADIUS_CONTROL.as_f32(),
            t if t == pug_tokens::semantic::RADIUS_SURFACE => typed::semantic::RADIUS_SURFACE.as_f32(),
            t if t == pug_tokens::semantic::RADIUS_PILL => typed::semantic::RADIUS_PILL.as_f32(),
            _ => 0.0,
        }
    }

    fn resolve_opacity(&self, token: &str) -> f32 {
        if let Ok(v) = token.parse::<f32>() {
            return v;
        }
        match token {
            t if t == pug_tokens::semantic::STATE_OPACITY_DISABLED => typed::semantic::STATE_OPACITY_DISABLED,
            t if t == pug_tokens::semantic::STATE_OPACITY_MUTED => typed::semantic::STATE_OPACITY_MUTED,
            _ => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use pug_adapter::ThemeProvider;
    use pug_tokens::semantic;

    use super::*;

    #[test]
    fn resolves_hex_color_tokens() {
        let theme = GpuiThemeProvider::default();
        let color = theme.resolve_color(semantic::COLOR_ACCENT_BASE);
        // #2d86f3 → approximately (0.176, 0.525, 0.953, 1.0)
        assert!((color.0 - 0.176).abs() < 0.01);
        assert!((color.1 - 0.525).abs() < 0.01);
        assert!((color.2 - 0.953).abs() < 0.01);
        assert_eq!(color.3, 1.0);
    }

    #[test]
    fn resolves_rgba_color_tokens() {
        let theme = GpuiThemeProvider::default();
        let color = theme.resolve_color(semantic::COLOR_BACKGROUND_OVERLAY);
        // rgba(11, 15, 20, 0.64)
        assert!(color.3 > 0.6 && color.3 < 0.7);
    }

    #[test]
    fn resolves_rem_space_tokens() {
        let theme = GpuiThemeProvider::default();
        let space = theme.resolve_space(semantic::SPACE_STACK_MD);
        // 0.75rem = 12px
        assert_eq!(space, 12.0);
    }

    #[test]
    fn resolves_border_width_tokens() {
        let theme = GpuiThemeProvider::default();
        let width = theme.resolve_border_width(semantic::BORDER_WIDTH_DEFAULT);
        // 0.0625rem = 1px
        assert_eq!(width, 1.0);
    }

    #[test]
    fn resolves_radius_tokens() {
        let theme = GpuiThemeProvider::default();
        let radius = theme.resolve_radius(semantic::RADIUS_CONTROL);
        assert_eq!(radius, 6.0);
    }

    #[test]
    fn resolves_opacity_tokens() {
        let theme = GpuiThemeProvider::default();
        let opacity = theme.resolve_opacity(semantic::STATE_OPACITY_DISABLED);
        assert_eq!(opacity, 0.48);
    }

    #[test]
    fn unknown_tokens_return_safe_defaults() {
        let theme = GpuiThemeProvider::default();
        let color = theme.resolve_color("unknown-token");
        assert_eq!(color, ColorValue(0.0, 0.0, 0.0, 1.0));
        assert_eq!(theme.resolve_space("unknown"), 0.0);
        assert_eq!(theme.resolve_border_width("unknown"), 0.0);
        assert_eq!(theme.resolve_radius("unknown"), 0.0);
        assert_eq!(theme.resolve_opacity("unknown"), 1.0);
    }

    #[test]
    fn scale_factor_is_configurable() {
        let theme = GpuiThemeProvider::new().with_scale_factor(2.0);
        assert_eq!(theme.scale_factor, 2.0);
    }
}
