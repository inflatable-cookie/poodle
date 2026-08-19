//! GPUI theme integration.
//!
//! Implements the `ThemeProvider` trait from `poodle-adapter` by resolving
//! semantic token paths to typed values using the `poodle-tokens::typed` module.
//!
//! The token paths passed to `resolve_*` methods are semantic path strings
//! (e.g., `"color.accent.base"`, `"radius.control"`).
//! The provider checks theme overrides first, then falls back to the
//! typed constant defaults from the light theme baseline.

use poodle_adapter::ThemeProvider;
use poodle_tokens::typed::{self, ColorValue};

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
    /// Neutral-contrast knob (mirrors the CSS `--poodle-contrast` axis).
    /// 0.4 = flat … 0.5 = library default … 1 = full theme ramp.
    pub contrast: f32,
    /// OKLab lightness of the active theme's canvas (contrast pivot).
    contrast_anchor_l: f64,
}

impl Default for GpuiThemeProvider {
    fn default() -> Self {
        let mut provider = Self {
            scale_factor: 1.0,
            overrides: Vec::new(),
            theme_name: "default".to_string(),
            contrast: 0.5,
            contrast_anchor_l: 0.0,
        };
        provider.recompute_contrast_anchor();
        provider
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
    pub fn with_theme(mut self, theme: &poodle_tokens::themes::ThemeDefinition) -> Self {
        self.overrides = theme.overrides.to_vec();
        self.theme_name = theme.name.to_string();
        self.recompute_contrast_anchor();
        self
    }

    /// Set the neutral-contrast knob (see the `contrast` field).
    pub fn with_contrast(mut self, contrast: f32) -> Self {
        self.contrast = contrast;
        self
    }

    /// Recompute the contrast pivot from the active theme's canvas color.
    fn recompute_contrast_anchor(&mut self) {
        let canvas = self.resolve_color_value_raw("color.background.canvas");
        self.contrast_anchor_l = poodle_headless::color::oklab_lightness(
            canvas.0 as f64,
            canvas.1 as f64,
            canvas.2 as f64,
        );
    }

    /// Apply density overrides on top of the current theme.
    ///
    /// Density overrides adjust spacing tokens (panel padding, control
    /// spacing, stack gaps) and control height. They layer on top of
    /// theme overrides — later overrides win when tokens conflict.
    pub fn with_density(mut self, density: &poodle_tokens::density::DensityDefinition) -> Self {
        self.overrides.extend_from_slice(density.overrides);
        self
    }

    /// Apply control-size overrides on top of the current theme.
    ///
    /// Control-size overrides adjust control height, min-width, and
    /// default icon size for the given size stop.
    pub fn with_control_size(
        mut self,
        size: &poodle_tokens::density::ControlSizeDefinition,
    ) -> Self {
        self.overrides.extend_from_slice(size.overrides);
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
        let raw = self.resolve_color_value_raw(token);
        if (self.contrast - 1.0).abs() < f32::EPSILON
            || !poodle_headless::color::is_contrast_scaled_token(token)
        {
            return raw;
        }
        let (r, g, b, a) = poodle_headless::color::apply_neutral_contrast(
            raw.0 as f64,
            raw.1 as f64,
            raw.2 as f64,
            raw.3 as f64,
            self.contrast_anchor_l,
            self.contrast as f64,
        );
        ColorValue(r as f32, g as f32, b as f32, a as f32)
    }

    fn resolve_color_value_raw(&self, token: &str) -> ColorValue {
        // 1. Check theme overrides (token is a semantic path like "color.accent.base")
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
            "color.background.canvas" => typed::semantic::COLOR_BACKGROUND_CANVAS,
            "color.background.surface" => typed::semantic::COLOR_BACKGROUND_SURFACE,
            "color.background.panel" => typed::semantic::COLOR_BACKGROUND_PANEL,
            "color.background.elevated" => typed::semantic::COLOR_BACKGROUND_ELEVATED,
            "color.background.overlay" => typed::semantic::COLOR_BACKGROUND_OVERLAY,
            "color.text.primary" => typed::semantic::COLOR_TEXT_PRIMARY,
            "color.text.secondary" => typed::semantic::COLOR_TEXT_SECONDARY,
            "color.text.tertiary" => typed::semantic::COLOR_TEXT_TERTIARY,
            "color.text.inverse" => typed::semantic::COLOR_TEXT_INVERSE,
            "color.border.subtle" => typed::semantic::COLOR_BORDER_SUBTLE,
            "color.border.default" => typed::semantic::COLOR_BORDER_DEFAULT,
            "color.border.strong" => typed::semantic::COLOR_BORDER_STRONG,
            "color.accent.base" => typed::semantic::COLOR_ACCENT_BASE,
            "color.accent.hover" => typed::semantic::COLOR_ACCENT_HOVER,
            "color.accent.focusRing" => typed::semantic::COLOR_ACCENT_FOCUS_RING,
            "color.status.success" => typed::semantic::COLOR_STATUS_SUCCESS,
            "color.status.warning" => typed::semantic::COLOR_STATUS_WARNING,
            "color.status.danger" => typed::semantic::COLOR_STATUS_DANGER,
            "color.status.info" => typed::semantic::COLOR_STATUS_INFO,
            "color.icon.primary" => typed::semantic::COLOR_ICON_PRIMARY,
            "color.icon.muted" => typed::semantic::COLOR_ICON_MUTED,
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
            "space.stack.sm" => typed::semantic::SPACE_STACK_SM.as_f32(),
            "space.stack.md" => typed::semantic::SPACE_STACK_MD.as_f32(),
            "space.stack.lg" => typed::semantic::SPACE_STACK_LG.as_f32(),
            "space.inline.xs" => typed::semantic::SPACE_INLINE_XS.as_f32(),
            "space.inline.sm" => typed::semantic::SPACE_INLINE_SM.as_f32(),
            "space.inline.md" => typed::semantic::SPACE_INLINE_MD.as_f32(),
            "space.inline.lg" => typed::semantic::SPACE_INLINE_LG.as_f32(),
            "space.panel.x" => typed::semantic::SPACE_PANEL_X.as_f32(),
            "space.panel.y" => typed::semantic::SPACE_PANEL_Y.as_f32(),
            "space.control.x" => typed::semantic::SPACE_CONTROL_X.as_f32(),
            "space.control.y" => typed::semantic::SPACE_CONTROL_Y.as_f32(),
            "space.button.gap" => typed::semantic::SPACE_BUTTON_GAP.as_f32(),
            "space.button.iconInset" => typed::semantic::SPACE_BUTTON_ICON_INSET.as_f32(),
            "size.control.height" => typed::semantic::SIZE_CONTROL_HEIGHT.as_f32(),
            "size.control.minWidth" => typed::semantic::SIZE_CONTROL_MIN_WIDTH.as_f32(),
            "size.icon.sm" => typed::semantic::SIZE_ICON_SM.as_f32(),
            "size.icon.md" => typed::semantic::SIZE_ICON_MD.as_f32(),
            "size.icon.lg" => typed::semantic::SIZE_ICON_LG.as_f32(),
            "size.panel.header" => typed::semantic::SIZE_PANEL_HEADER.as_f32(),
            "size.list.grid.minItemWidth" => {
                typed::semantic::SIZE_LIST_GRID_MIN_ITEM_WIDTH.as_f32()
            }
            "icon.size.default" => typed::semantic::ICON_SIZE_DEFAULT.as_f32(),
            // Typography sizes
            "typography.body.size" => typed::semantic::TYPOGRAPHY_BODY_SIZE.as_f32(),
            "typography.body.lineHeight" => typed::semantic::TYPOGRAPHY_BODY_LINE_HEIGHT.as_f32(),
            "typography.label.size" => typed::semantic::TYPOGRAPHY_LABEL_SIZE.as_f32(),
            "typography.label.lineHeight" => typed::semantic::TYPOGRAPHY_LABEL_LINE_HEIGHT.as_f32(),
            "typography.caption.size" => typed::semantic::TYPOGRAPHY_CAPTION_SIZE.as_f32(),
            "typography.caption.lineHeight" => {
                typed::semantic::TYPOGRAPHY_CAPTION_LINE_HEIGHT.as_f32()
            }
            "typography.counter.size" => typed::semantic::TYPOGRAPHY_COUNTER_SIZE.as_f32(),
            "typography.heading.size" => typed::semantic::TYPOGRAPHY_HEADING_SIZE.as_f32(),
            "typography.heading.lineHeight" => {
                typed::semantic::TYPOGRAPHY_HEADING_LINE_HEIGHT.as_f32()
            }
            "typography.code.size" => typed::semantic::TYPOGRAPHY_CODE_SIZE.as_f32(),
            "typography.code.lineHeight" => typed::semantic::TYPOGRAPHY_CODE_LINE_HEIGHT.as_f32(),
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
            "border.width.default" => typed::semantic::BORDER_WIDTH_DEFAULT.as_f32(),
            "border.width.focus" => typed::semantic::BORDER_WIDTH_FOCUS.as_f32(),
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
            "radius.control" => typed::semantic::RADIUS_CONTROL.as_f32(),
            "radius.surface" => typed::semantic::RADIUS_SURFACE.as_f32(),
            "radius.pill" => typed::semantic::RADIUS_PILL.as_f32(),
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
            "state.opacity.disabled" => typed::semantic::STATE_OPACITY_DISABLED,
            "state.opacity.muted" => typed::semantic::STATE_OPACITY_MUTED,
            _ => token.parse::<f32>().unwrap_or(1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::ThemeProvider;
    use poodle_tokens::semantic;

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
        let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
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
        let dark = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
        let light = GpuiThemeProvider::default();
        let dark_accent = dark.resolve_color(semantic::COLOR_ACCENT_BASE);
        let light_accent = light.resolve_color(semantic::COLOR_ACCENT_BASE);
        // Dark gold vs light blue — they must differ
        assert!((dark_accent.0 - light_accent.0).abs() > 0.1);
    }
}

#[cfg(test)]
mod contrast_tests {
    use super::*;

    #[test]
    fn neutral_contrast_scales_dark_surface() {
        let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
        // default contrast 0.5 — surface sits halfway (in oklab L) to canvas
        let surface = theme.resolve_color_value("color.background.surface");
        let full = theme.clone().with_contrast(1.0);
        let literal = full.resolve_color_value("color.background.surface");
        assert!(
            (literal.0 - 21.0 / 255.0).abs() < 0.002,
            "k=1 is the literal"
        );
        assert!(surface.0 < literal.0, "toned-down surface is darker");
        // reference value from the shared conformance math
        assert!(
            (surface.0 - 17.073 / 255.0).abs() < 0.002,
            "surface r {}",
            surface.0
        );
        assert!(
            (surface.2 - 22.943 / 255.0).abs() < 0.002,
            "surface b {}",
            surface.2
        );
    }

    #[test]
    fn accent_and_text_are_untouched() {
        let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
        let full = theme.clone().with_contrast(1.0);
        for token in [
            "color.accent.base",
            "color.text.primary",
            "color.background.overlay",
        ] {
            let a = theme.resolve_color_value(token);
            let b = full.resolve_color_value(token);
            assert_eq!(a.0, b.0, "{token}");
            assert_eq!(a.3, b.3, "{token} alpha");
        }
    }

    #[test]
    fn translucent_borders_scale_alpha() {
        let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
        let border = theme.resolve_color_value("color.border.default");
        assert!((border.3 - 0.11).abs() < 0.001, "alpha {}", border.3);
    }
}
