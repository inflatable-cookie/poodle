//! Jetstream theme provider — resolves Pug token paths to typed values
//! compatible with Jetstream's Vec4 colors and f32 pixel values.
//!
//! g08.002: Token bridge between Pug's semantic token system and Jetstream's
//! theme model. Uses the same resolution strategy as GPUI (parse string values
//! directly, match against typed constants, fall back to safe defaults).

use std::collections::HashMap;

use pug_adapter::ThemeProvider;
use pug_tokens::typed::{self, ColorValue};
use pug_tokens::themes::ThemeDefinition;

/// Theme provider for the Jetstream rendering adapter.
///
/// Resolves Pug semantic token strings to typed values. Supports runtime
/// theme switching by applying color overrides from a `ThemeDefinition`.
#[derive(Debug, Clone)]
pub struct JetstreamThemeProvider {
    scale_factor: f32,
    /// Color overrides from the active theme definition. Keys are semantic
    /// token paths (e.g. "semantic.color.background.canvas"), values are
    /// pre-parsed RGBA colors.
    color_overrides: HashMap<String, ColorValue>,
}

impl Default for JetstreamThemeProvider {
    fn default() -> Self {
        Self::from_theme(&pug_tokens::themes::LIGHT)
    }
}

impl JetstreamThemeProvider {
    /// Create a provider from a theme definition, pre-parsing all color
    /// overrides so that resolve_color is fast at runtime.
    pub fn from_theme(theme: &ThemeDefinition) -> Self {
        let mut color_overrides = HashMap::new();
        for &(key, value) in theme.overrides {
            // Only cache color overrides.
            if key.contains("color.") {
                if let Some(hex) = value.strip_prefix('#') {
                    color_overrides.insert(key.to_string(), parse_hex_color(hex));
                } else if let Some(inner) = value.strip_prefix("rgba(").and_then(|s| s.strip_suffix(')')) {
                    color_overrides.insert(key.to_string(), parse_rgba(inner));
                }
            }
        }
        Self {
            scale_factor: 1.0,
            color_overrides,
        }
    }

    pub fn with_scale_factor(mut self, factor: f32) -> Self {
        self.scale_factor = factor;
        self
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    /// Look up a semantic token in the theme's color overrides.
    /// Tokens use short names like "background.canvas"; override keys use
    /// full paths like "semantic.color.background.canvas".
    fn match_override(&self, token: &str) -> Option<ColorValue> {
        // Try to find the token in our pre-parsed override map.
        // The override keys are "semantic.color.<segment>" and the tokens
        // passed in contain the segment (e.g. "background.canvas").
        for (key, color) in &self.color_overrides {
            // Strip "semantic.color." prefix to get the segment.
            if let Some(segment) = key.strip_prefix("semantic.color.") {
                if token.contains(segment) {
                    return Some(*color);
                }
            }
        }
        None
    }
}

impl ThemeProvider for JetstreamThemeProvider {
    fn resolve_color(&self, token: &str) -> ColorValue {
        // Strategy 1: Parse hex color strings directly
        if let Some(hex) = token.strip_prefix('#') {
            return parse_hex_color(hex);
        }

        // Strategy 2: Parse rgba() function strings
        if let Some(inner) = token.strip_prefix("rgba(").and_then(|s| s.strip_suffix(')')) {
            return parse_rgba(inner);
        }

        // Strategy 3: Check theme overrides (dark/light/loophole-studio)
        if let Some(color) = self.match_override(token) {
            return color;
        }

        // Strategy 4: Match against known typed semantic constants (light defaults)
        if let Some(color) = match_semantic_color(token) {
            return color;
        }

        // Fallback: opaque black
        ColorValue(0.0, 0.0, 0.0, 1.0)
    }

    fn resolve_space(&self, token: &str) -> f32 {
        // Parse rem values
        if let Some(rem_str) = token.strip_suffix("rem") {
            if let Ok(rem) = rem_str.parse::<f32>() {
                return rem * 16.0 * self.scale_factor;
            }
        }

        // Parse px values
        if let Some(px_str) = token.strip_suffix("px") {
            if let Ok(px) = px_str.parse::<f32>() {
                return px * self.scale_factor;
            }
        }

        // Parse plain numbers
        if let Ok(val) = token.parse::<f32>() {
            return val * self.scale_factor;
        }

        0.0
    }

    fn resolve_radius(&self, token: &str) -> f32 {
        self.resolve_space(token)
    }

    fn resolve_border_width(&self, token: &str) -> f32 {
        self.resolve_space(token)
    }

    fn resolve_opacity(&self, token: &str) -> f32 {
        if let Ok(val) = token.parse::<f32>() {
            return val;
        }
        1.0
    }
}

fn parse_hex_color(hex: &str) -> ColorValue {
    let hex = hex.trim();
    let chars: Vec<char> = hex.chars().collect();

    match chars.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
            ColorValue(r, g, b, 1.0)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f32 / 255.0;
            ColorValue(r, g, b, a)
        }
        _ => ColorValue(0.0, 0.0, 0.0, 1.0),
    }
}

fn parse_rgba(inner: &str) -> ColorValue {
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
    if parts.len() == 4 {
        let r = parts[0].parse::<f32>().unwrap_or(0.0) / 255.0;
        let g = parts[1].parse::<f32>().unwrap_or(0.0) / 255.0;
        let b = parts[2].parse::<f32>().unwrap_or(0.0) / 255.0;
        let a = parts[3].parse::<f32>().unwrap_or(1.0);
        ColorValue(r, g, b, a)
    } else {
        ColorValue(0.0, 0.0, 0.0, 1.0)
    }
}

fn match_semantic_color(token: &str) -> Option<ColorValue> {
    match token {
        t if t.contains("accent.base") => Some(typed::semantic::COLOR_ACCENT_BASE),
        t if t.contains("accent.hover") => Some(typed::semantic::COLOR_ACCENT_HOVER),
        t if t.contains("accent.focus") => Some(typed::semantic::COLOR_ACCENT_FOCUS_RING),
        t if t.contains("background.canvas") => Some(typed::semantic::COLOR_BACKGROUND_CANVAS),
        t if t.contains("background.surface") => Some(typed::semantic::COLOR_BACKGROUND_SURFACE),
        t if t.contains("background.panel") => Some(typed::semantic::COLOR_BACKGROUND_PANEL),
        t if t.contains("background.overlay") => Some(typed::semantic::COLOR_BACKGROUND_OVERLAY),
        t if t.contains("background.elevated") => Some(typed::semantic::COLOR_BACKGROUND_ELEVATED),
        t if t.contains("text.primary") => Some(typed::semantic::COLOR_TEXT_PRIMARY),
        t if t.contains("text.secondary") => Some(typed::semantic::COLOR_TEXT_SECONDARY),
        t if t.contains("text.inverse") => Some(typed::semantic::COLOR_TEXT_INVERSE),
        t if t.contains("border.subtle") => Some(typed::semantic::COLOR_BORDER_SUBTLE),
        t if t.contains("border.default") => Some(typed::semantic::COLOR_BORDER_DEFAULT),
        t if t.contains("border.strong") => Some(typed::semantic::COLOR_BORDER_STRONG),
        t if t.contains("status.success") => Some(typed::semantic::COLOR_STATUS_SUCCESS),
        t if t.contains("status.warning") => Some(typed::semantic::COLOR_STATUS_WARNING),
        t if t.contains("status.danger") => Some(typed::semantic::COLOR_STATUS_DANGER),
        t if t.contains("icon.primary") => Some(typed::semantic::COLOR_ICON_PRIMARY),
        t if t.contains("icon.muted") => Some(typed::semantic::COLOR_ICON_MUTED),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use pug_adapter::ThemeProvider;
    use super::*;

    #[test]
    fn resolves_hex_color_tokens() {
        let theme = JetstreamThemeProvider::default();
        let color = theme.resolve_color("#2d86f3");
        assert!((color.0 - 0.176).abs() < 0.01);
        assert!((color.1 - 0.525).abs() < 0.01);
        assert!((color.2 - 0.953).abs() < 0.01);
        assert!((color.3 - 1.0).abs() < 0.001);
    }

    #[test]
    fn resolves_rgba_color_tokens() {
        let theme = JetstreamThemeProvider::default();
        let color = theme.resolve_color("rgba(45, 134, 243, 0.5)");
        assert!((color.0 - 0.176).abs() < 0.01);
        assert!((color.3 - 0.5).abs() < 0.001);
    }

    #[test]
    fn resolves_rem_space_tokens() {
        let theme = JetstreamThemeProvider::default();
        assert!((theme.resolve_space("0.5rem") - 8.0).abs() < 0.001);
        assert!((theme.resolve_space("1rem") - 16.0).abs() < 0.001);
    }

    #[test]
    fn resolves_px_space_tokens() {
        let theme = JetstreamThemeProvider::default();
        assert!((theme.resolve_space("4px") - 4.0).abs() < 0.001);
    }

    #[test]
    fn resolves_border_width_tokens() {
        let theme = JetstreamThemeProvider::default();
        assert!((theme.resolve_border_width("0.0625rem") - 1.0).abs() < 0.001);
    }

    #[test]
    fn resolves_opacity_tokens() {
        let theme = JetstreamThemeProvider::default();
        assert!((theme.resolve_opacity("0.38") - 0.38).abs() < 0.001);
    }

    #[test]
    fn scale_factor_applies_to_space() {
        let theme = JetstreamThemeProvider::default().with_scale_factor(2.0);
        assert!((theme.resolve_space("1rem") - 32.0).abs() < 0.001);
        assert_eq!(theme.scale_factor(), 2.0);
    }

    #[test]
    fn unknown_tokens_return_safe_defaults() {
        let theme = JetstreamThemeProvider::default();
        assert_eq!(theme.resolve_color("unknown"), ColorValue(0.0, 0.0, 0.0, 1.0));
        assert_eq!(theme.resolve_space("unknown"), 0.0);
        assert_eq!(theme.resolve_opacity("unknown"), 1.0);
    }
}
