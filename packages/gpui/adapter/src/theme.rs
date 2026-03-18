//! GPUI theme integration.
//!
//! Implements the `ThemeProvider` trait from `pug-adapter` by resolving
//! semantic token paths to typed values using the `pug-tokens::typed` module.
//!
//! The token paths passed to `resolve_*` methods are semantic path strings
//! (e.g., `"semantic.color.accent.base"`, `"semantic.radius.control"`).
//! The provider checks theme overrides first, then falls back to the
//! typed constant defaults from the light theme baseline.

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

    /// Resolve a color token to a `ColorValue`.
    ///
    /// Resolution order:
    /// 1. Theme overrides (keyed by semantic path)
    /// 2. Typed constant defaults (light theme baseline)
    /// 3. Direct hex/rgba parsing (for inline values)
    /// 4. Black fallback
    pub fn resolve_color_value(&self, token: &str) -> ColorValue {
        // 1. Check theme overrides (token is a semantic path like "semantic.color.accent.base")
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
        // 2. Fall back to typed constant defaults (light theme baseline)
        match token {
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
            // 3. Direct hex/rgba parsing (for inline color values)
            _ => {
                if let Some(color) = Self::parse_hex_color(token) {
                    return color;
                }
                if let Some(color) = Self::parse_rgba_color(token) {
                    return color;
                }
                ColorValue(0.0, 0.0, 0.0, 1.0)
            }
        }
    }

    /// Resolve a space/size token to a pixel value.
    ///
    /// Checks theme overrides for density/size mode overrides,
    /// then falls back to typed constant defaults.
    pub fn resolve_space_value(&self, token: &str) -> f32 {
        // Check overrides (density modes can override spacing)
        for &(path, value) in &self.overrides {
            if path == token {
                if let Some(px) = Self::parse_dimension(value) {
                    return px;
                }
            }
        }
        // Fall back to typed constant defaults
        match token {
            "semantic.space.stack.sm" => typed::semantic::SPACE_STACK_SM.as_f32(),
            "semantic.space.stack.md" => typed::semantic::SPACE_STACK_MD.as_f32(),
            "semantic.space.stack.lg" => typed::semantic::SPACE_STACK_LG.as_f32(),
            "semantic.space.inline.sm" => typed::semantic::SPACE_INLINE_SM.as_f32(),
            "semantic.space.inline.md" => typed::semantic::SPACE_INLINE_MD.as_f32(),
            "semantic.space.inline.lg" => typed::semantic::SPACE_INLINE_LG.as_f32(),
            "semantic.space.panel.x" => typed::semantic::SPACE_PANEL_X.as_f32(),
            "semantic.space.panel.y" => typed::semantic::SPACE_PANEL_Y.as_f32(),
            "semantic.space.control.x" => typed::semantic::SPACE_CONTROL_X.as_f32(),
            "semantic.space.control.y" => typed::semantic::SPACE_CONTROL_Y.as_f32(),
            "semantic.size.control.height" => typed::semantic::SIZE_CONTROL_HEIGHT.as_f32(),
            "semantic.size.control.minWidth" => typed::semantic::SIZE_CONTROL_MIN_WIDTH.as_f32(),
            "semantic.size.icon.sm" => typed::semantic::SIZE_ICON_SM.as_f32(),
            "semantic.size.icon.md" => typed::semantic::SIZE_ICON_MD.as_f32(),
            "semantic.size.icon.lg" => typed::semantic::SIZE_ICON_LG.as_f32(),
            "semantic.size.panel.header" => typed::semantic::SIZE_PANEL_HEADER.as_f32(),
            "semantic.icon.size.default" => typed::semantic::ICON_SIZE_DEFAULT.as_f32(),
            // Direct dimension parsing for inline values
            _ => Self::parse_dimension(token).unwrap_or(0.0),
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
        // Check overrides
        for &(path, value) in &self.overrides {
            if path == token {
                if let Some(px) = Self::parse_dimension(value) {
                    return px;
                }
            }
        }
        match token {
            "semantic.border.width.default" => typed::semantic::BORDER_WIDTH_DEFAULT.as_f32(),
            "semantic.border.width.focus" => typed::semantic::BORDER_WIDTH_FOCUS.as_f32(),
            _ => Self::parse_dimension(token).unwrap_or(0.0),
        }
    }

    fn resolve_radius(&self, token: &str) -> f32 {
        // Check overrides
        for &(path, value) in &self.overrides {
            if path == token {
                if let Some(px) = Self::parse_dimension(value) {
                    return px;
                }
            }
        }
        match token {
            "semantic.radius.control" => typed::semantic::RADIUS_CONTROL.as_f32(),
            "semantic.radius.surface" => typed::semantic::RADIUS_SURFACE.as_f32(),
            "semantic.radius.pill" => typed::semantic::RADIUS_PILL.as_f32(),
            _ => Self::parse_dimension(token).unwrap_or(0.0),
        }
    }

    fn resolve_opacity(&self, token: &str) -> f32 {
        // Check overrides
        for &(path, value) in &self.overrides {
            if path == token {
                if let Ok(v) = value.parse::<f32>() {
                    return v;
                }
            }
        }
        match token {
            "semantic.state.opacity.disabled" => typed::semantic::STATE_OPACITY_DISABLED,
            "semantic.state.opacity.muted" => typed::semantic::STATE_OPACITY_MUTED,
            _ => token.parse::<f32>().unwrap_or(1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use pug_adapter::ThemeProvider;
    use pug_tokens::semantic;

    use super::*;

    #[test]
    fn resolves_color_from_default_baseline() {
        let theme = GpuiThemeProvider::default();
        let color = theme.resolve_color(semantic::COLOR_ACCENT_BASE);
        // Light baseline: #2d86f3 → approximately (0.176, 0.525, 0.953, 1.0)
        assert!((color.0 - 0.176).abs() < 0.01);
        assert!((color.1 - 0.525).abs() < 0.01);
        assert!((color.2 - 0.953).abs() < 0.01);
        assert_eq!(color.3, 1.0);
    }

    #[test]
    fn dark_theme_overrides_accent_color() {
        let theme = GpuiThemeProvider::new().with_theme(&pug_tokens::themes::DARK);
        let color = theme.resolve_color(semantic::COLOR_ACCENT_BASE);
        // Dark theme: #f0b24d → approximately (0.941, 0.698, 0.302, 1.0)
        assert!((color.0 - 0.941).abs() < 0.01, "r={}", color.0);
        assert!((color.1 - 0.698).abs() < 0.01, "g={}", color.1);
        assert!((color.2 - 0.302).abs() < 0.01, "b={}", color.2);
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
    fn resolves_space_tokens() {
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

    #[test]
    fn theme_switching_changes_colors() {
        let dark = GpuiThemeProvider::new().with_theme(&pug_tokens::themes::DARK);
        let light = GpuiThemeProvider::default();
        let dark_accent = dark.resolve_color(semantic::COLOR_ACCENT_BASE);
        let light_accent = light.resolve_color(semantic::COLOR_ACCENT_BASE);
        // Dark gold vs light blue — they must differ
        assert!((dark_accent.0 - light_accent.0).abs() > 0.1);
    }
}
