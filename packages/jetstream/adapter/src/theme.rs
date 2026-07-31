//! Jetstream theme provider — resolves Poodle token paths to typed values
//! compatible with Jetstream's Vec4 colors and f32 pixel values.
//!
//! g08.002: Token bridge between Poodle's semantic token system and Jetstream's
//! theme model. Uses the same resolution strategy as GPUI (parse string values
//! directly, match against typed constants, fall back to safe defaults).

use std::collections::HashMap;

use poodle_adapter::ThemeProvider;
use poodle_tokens::typed::{self, ColorValue};
use poodle_tokens::themes::ThemeDefinition;

/// Theme provider for the Jetstream rendering adapter.
///
/// Resolves Poodle semantic token strings to typed values. Supports runtime
/// theme switching by applying color overrides from a `ThemeDefinition`,
/// and density/control-size overrides from `DensityDefinition` /
/// `ControlSizeDefinition`.
#[derive(Debug, Clone)]
pub struct JetstreamThemeProvider {
    scale_factor: f32,
    /// Color overrides from the active theme definition. Keys are semantic
    /// token paths (e.g. "color.background.canvas"), values are
    /// pre-parsed RGBA colors.
    color_overrides: HashMap<String, ColorValue>,
    /// Space/size overrides from density and control-size definitions.
    /// Keys are semantic token paths (e.g. "space.panel.x"),
    /// values are raw CSS strings (e.g. "0.75rem") parsed at resolve time.
    space_overrides: HashMap<String, String>,
    /// Neutral-contrast knob (mirrors the CSS `--poodle-contrast` axis).
    /// 0.4 = flat … 0.5 = library default … 1 = full theme ramp.
    pub contrast: f32,
    /// OKLab lightness of the active theme's canvas (contrast pivot).
    contrast_anchor_l: f64,
}

impl Default for JetstreamThemeProvider {
    fn default() -> Self {
        Self::from_theme(&poodle_tokens::themes::ICEBERG)
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
        let mut provider = Self {
            scale_factor: 1.0,
            color_overrides,
            space_overrides: HashMap::new(),
            contrast: 0.5,
            contrast_anchor_l: 0.0,
        };
        let canvas = provider.resolve_color_raw("color.background.canvas");
        provider.contrast_anchor_l = poodle_headless::color::oklab_lightness(
            canvas.0 as f64,
            canvas.1 as f64,
            canvas.2 as f64,
        );
        provider
    }

    /// Set the neutral-contrast knob (see the `contrast` field).
    pub fn with_contrast(mut self, contrast: f32) -> Self {
        self.contrast = contrast;
        self
    }

    fn resolve_color_raw(&self, token: &str) -> ColorValue {
        if let Some(hex) = token.strip_prefix('#') {
            return parse_hex_color(hex);
        }
        if let Some(inner) = token.strip_prefix("rgba(").and_then(|s| s.strip_suffix(')')) {
            return parse_rgba(inner);
        }
        if let Some(color) = self.match_override(token) {
            return color;
        }
        if let Some(color) = match_semantic_color(token) {
            return color;
        }
        ColorValue(0.0, 0.0, 0.0, 1.0)
    }

    pub fn with_scale_factor(mut self, factor: f32) -> Self {
        self.scale_factor = factor;
        self
    }

    /// Apply density overrides on top of the current theme.
    ///
    /// Density overrides adjust spacing tokens (panel padding, control
    /// spacing, stack gaps) and control height. They layer on top of
    /// theme overrides — later calls win when tokens conflict.
    pub fn with_density(mut self, density: &poodle_tokens::density::DensityDefinition) -> Self {
        for &(key, value) in density.overrides {
            self.space_overrides.insert(key.to_string(), value.to_string());
        }
        self
    }

    /// Apply control-size overrides on top of the current theme.
    ///
    /// Control-size overrides adjust control height, min-width, and
    /// default icon size for the given size stop.
    pub fn with_control_size(mut self, size: &poodle_tokens::density::ControlSizeDefinition) -> Self {
        for &(key, value) in size.overrides {
            self.space_overrides.insert(key.to_string(), value.to_string());
        }
        self
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    /// Look up a semantic token in the theme's color overrides.
    /// Tokens use short names like "background.canvas"; override keys use
    /// full paths like "color.background.canvas".
    fn match_override(&self, token: &str) -> Option<ColorValue> {
        // Try to find the token in our pre-parsed override map.
        // The override keys are "color.<segment>" and the tokens
        // passed in contain the segment (e.g. "background.canvas").
        for (key, color) in &self.color_overrides {
            // Strip "color." prefix to get the segment.
            if let Some(segment) = key.strip_prefix("color.") {
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
        let raw = self.resolve_color_raw(token);
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

    fn resolve_space(&self, token: &str) -> f32 {
        // Strategy 0: Check density/control-size overrides first (highest priority)
        if let Some(value) = self.space_overrides.get(token) {
            if let Some(rem_str) = value.strip_suffix("rem") {
                if let Ok(rem) = rem_str.parse::<f32>() {
                    return rem * 16.0 * self.scale_factor;
                }
            }
            if let Some(px_str) = value.strip_suffix("px") {
                if let Ok(px) = px_str.parse::<f32>() {
                    return px * self.scale_factor;
                }
            }
            if let Ok(val) = value.parse::<f32>() {
                return val * self.scale_factor;
            }
        }

        // Strategy 1: Look up semantic space/size/radius tokens
        if let Some(val) = match_semantic_space(token) {
            return val;
        }

        // Strategy 2: Parse rem values
        if let Some(rem_str) = token.strip_suffix("rem") {
            if let Ok(rem) = rem_str.parse::<f32>() {
                return rem * 16.0 * self.scale_factor;
            }
        }

        // Strategy 3: Parse px values
        if let Some(px_str) = token.strip_suffix("px") {
            if let Ok(px) = px_str.parse::<f32>() {
                return px * self.scale_factor;
            }
        }

        // Strategy 4: Parse plain numbers
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
        // Look up semantic tokens first
        if let Some(val) = match_semantic_space(token) {
            return val;
        }
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
        t if t.contains("text.tertiary") => Some(typed::semantic::COLOR_TEXT_TERTIARY),
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

/// Match semantic space/size/radius tokens to typed values.
fn match_semantic_space(token: &str) -> Option<f32> {
    use poodle_tokens::typed;
    match token {
        // Size tokens
        t if t.contains("size.control.height") => Some(typed::semantic::SIZE_CONTROL_HEIGHT.0),
        t if t.contains("size.control.minWidth") => Some(typed::semantic::SIZE_CONTROL_MIN_WIDTH.0),
        t if t.contains("size.icon.xs") => Some(typed::semantic::SIZE_ICON_XS.0),
        t if t.contains("size.icon.sm") => Some(typed::semantic::SIZE_ICON_SM.0),
        t if t.contains("size.icon.md") => Some(typed::semantic::SIZE_ICON_MD.0),
        t if t.contains("size.icon.lg") => Some(typed::semantic::SIZE_ICON_LG.0),
        t if t.contains("size.icon.xl") => Some(typed::semantic::SIZE_ICON_XL.0),
        t if t.contains("size.panel.header") => Some(typed::semantic::SIZE_PANEL_HEADER.0),
        t if t.contains("size.list.grid.minItemWidth") => {
            Some(typed::semantic::SIZE_LIST_GRID_MIN_ITEM_WIDTH.0)
        },
        // Overlay dimensions. These were missing, and a token this function does
        // not know resolves to 0 — so `Select`'s panel got `max-height: 0`,
        // collapsed to its padding, and painted rows it could not hit-test. The
        // exhaustiveness test below is what stops that recurring.
        t if t.contains("size.menu.maxHeight") => Some(typed::semantic::SIZE_MENU_MAX_HEIGHT.0),
        t if t.contains("size.menu.minWidth") => Some(typed::semantic::SIZE_MENU_MIN_WIDTH.0),
        t if t.contains("size.popover.maxWidth") => Some(typed::semantic::SIZE_POPOVER_MAX_WIDTH.0),
        t if t.contains("size.hoverCard.maxWidth") => {
            Some(typed::semantic::SIZE_HOVER_CARD_MAX_WIDTH.0)
        },
        t if t.contains("size.select.minWidth") => Some(typed::semantic::SIZE_SELECT_MIN_WIDTH.0),
        t if t.contains("size.dateTimeRangePicker.minWidth") => {
            Some(typed::semantic::SIZE_DATE_TIME_RANGE_PICKER_MIN_WIDTH.0)
        },
        t if t.contains("size.fileUpload.dropZoneMinHeight") => {
            Some(typed::semantic::SIZE_FILE_UPLOAD_DROP_ZONE_MIN_HEIGHT.0)
        },
        // Space tokens
        t if t.contains("space.stack.sm") => Some(typed::semantic::SPACE_STACK_SM.0),
        t if t.contains("space.stack.md") => Some(typed::semantic::SPACE_STACK_MD.0),
        t if t.contains("space.stack.lg") => Some(typed::semantic::SPACE_STACK_LG.0),
        t if t.contains("space.inline.xs") => Some(typed::semantic::SPACE_INLINE_XS.0),
        t if t.contains("space.inline.sm") => Some(typed::semantic::SPACE_INLINE_SM.0),
        t if t.contains("space.inline.md") => Some(typed::semantic::SPACE_INLINE_MD.0),
        t if t.contains("space.inline.lg") => Some(typed::semantic::SPACE_INLINE_LG.0),
        t if t.contains("space.panel.x") => Some(typed::semantic::SPACE_PANEL_X.0),
        t if t.contains("space.panel.y") => Some(typed::semantic::SPACE_PANEL_Y.0),
        t if t.contains("space.control.x") => Some(typed::semantic::SPACE_CONTROL_X.0),
        t if t.contains("space.control.y") => Some(typed::semantic::SPACE_CONTROL_Y.0),
        t if t.contains("space.button.gap") => Some(typed::semantic::SPACE_BUTTON_GAP.0),
        t if t.contains("space.button.iconInset") => Some(typed::semantic::SPACE_BUTTON_ICON_INSET.0),
        // Radius tokens
        t if t.contains("radius.control") => Some(typed::semantic::RADIUS_CONTROL.0),
        t if t.contains("radius.surface") => Some(typed::semantic::RADIUS_SURFACE.0),
        t if t.contains("radius.pill") => Some(typed::semantic::RADIUS_PILL.0),
        // Border width tokens
        t if t.contains("border.width.default") || t.contains("borderWidth.default") => Some(typed::semantic::BORDER_WIDTH_DEFAULT.0),
        t if t.contains("border.width.focus") || t.contains("borderWidth.focus") => Some(typed::semantic::BORDER_WIDTH_FOCUS.0),
        // Typography sizes / line-heights — read from typed semantic constants,
        // never hardcoded. Order longest/most-specific paths first so e.g.
        // `typography.label.lineHeight` does not get swallowed by the
        // `typography.label.size` arm (`contains` matches substrings).
        t if t.contains("typography.body.lineHeight") => Some(typed::semantic::TYPOGRAPHY_BODY_LINE_HEIGHT.0),
        t if t.contains("typography.body.size") => Some(typed::semantic::TYPOGRAPHY_BODY_SIZE.0),
        t if t.contains("typography.label.lineHeight") => Some(typed::semantic::TYPOGRAPHY_LABEL_LINE_HEIGHT.0),
        t if t.contains("typography.label.weight") => Some(typed::semantic::TYPOGRAPHY_LABEL_WEIGHT),
        t if t.contains("typography.label.size") => Some(typed::semantic::TYPOGRAPHY_LABEL_SIZE.0),
        t if t.contains("typography.caption.lineHeight") => Some(typed::semantic::TYPOGRAPHY_CAPTION_LINE_HEIGHT.0),
        t if t.contains("typography.caption.weight") => Some(typed::semantic::TYPOGRAPHY_CAPTION_WEIGHT),
        t if t.contains("typography.caption.size") => Some(typed::semantic::TYPOGRAPHY_CAPTION_SIZE.0),
        t if t.contains("typography.counter.size") => Some(typed::semantic::TYPOGRAPHY_COUNTER_SIZE.0),
        t if t.contains("typography.heading.lineHeight") => Some(typed::semantic::TYPOGRAPHY_HEADING_LINE_HEIGHT.0),
        t if t.contains("typography.heading.weight") => Some(typed::semantic::TYPOGRAPHY_HEADING_WEIGHT),
        t if t.contains("typography.heading.size") => Some(typed::semantic::TYPOGRAPHY_HEADING_SIZE.0),
        t if t.contains("typography.heading") => Some(typed::semantic::TYPOGRAPHY_HEADING_SIZE.0),
        t if t.contains("typography.code.lineHeight") => Some(typed::semantic::TYPOGRAPHY_CODE_LINE_HEIGHT.0),
        t if t.contains("typography.code.size") => Some(typed::semantic::TYPOGRAPHY_CODE_SIZE.0),
        // Opacity tokens
        t if t.contains("state.opacity.disabled") => Some(typed::semantic::STATE_OPACITY_DISABLED),
        t if t.contains("state.opacity.muted") => Some(typed::semantic::STATE_OPACITY_MUTED),
        _ => None,
    }
}

// ── sRGB → linear color conversion ──

/// Convert a single sRGB component to linear light.
///
/// Uses the official sRGB transfer function (IEC 61966-2-1).
pub fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

impl JetstreamThemeProvider {
    /// Resolve a Poodle semantic token to a `glam::Vec4` color in **linear** space.
    ///
    /// Poodle tokens store colors in sRGB gamma space (matching CSS). This converts
    /// RGB channels to linear for correct rendering on an sRGB surface. Alpha is
    /// left unchanged.
    pub fn resolve_linear_color(&self, token: &str) -> glam::Vec4 {
        let c = self.resolve_color(token);
        glam::Vec4::new(
            srgb_to_linear(c.0),
            srgb_to_linear(c.1),
            srgb_to_linear(c.2),
            c.3,
        )
    }

    /// Resolve a space/size token to logical pixels.
    pub fn resolve_space_px(&self, token: &str) -> f32 {
        self.resolve_space(token)
    }

    /// Resolve a radius token to logical pixels.
    pub fn resolve_radius_px(&self, token: &str) -> f32 {
        self.resolve_radius(token)
    }

    /// Resolve an opacity token to a float.
    pub fn resolve_opacity_value(&self, token: &str) -> f32 {
        self.resolve_opacity(token)
    }
}

#[cfg(test)]
mod tests {
    use poodle_adapter::ThemeProvider;
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

    #[test]
    fn resolves_typography_size_tokens_nonzero() {
        let theme = JetstreamThemeProvider::default();
        // Regression: `typography.caption.size` previously had no match arm and
        // resolved to 0.0, breaking weekday captions / tab count badges / menu meta.
        assert_eq!(theme.resolve_space("typography.caption.size"), 11.0);
        assert_eq!(theme.resolve_space("typography.label.size"), 13.0);
        assert_eq!(theme.resolve_space("typography.body.size"), 14.0);
        assert_eq!(theme.resolve_space("typography.heading.size"), 16.0);
        assert_eq!(theme.resolve_space("typography.counter.size"), 12.0);
        assert_eq!(theme.resolve_space("typography.code.size"), 13.0);
        // Weight tokens resolve too (used by label/heading callers).
        assert_eq!(theme.resolve_space("typography.label.weight"), 500.0);
        // None of the typography size tokens should resolve to 0.
        for t in [
            "typography.caption.size",
            "typography.label.size",
            "typography.body.size",
            "typography.heading.size",
            "typography.counter.size",
        ] {
            assert!(theme.resolve_space(t) > 0.0, "{t} resolved to 0");
        }
    }

    #[test]
    fn typography_line_height_not_swallowed_by_size_arm() {
        // `contains` matching must not let `...label.size` capture `...label.lineHeight`.
        let theme = JetstreamThemeProvider::default();
        assert_eq!(theme.resolve_space("typography.label.lineHeight"), 16.0);
        assert_eq!(theme.resolve_space("typography.body.lineHeight"), 20.0);
        assert_eq!(theme.resolve_space("typography.caption.lineHeight"), 16.0);
    }

    #[test]
    fn resolve_linear_color_converts_srgb() {
        let theme = JetstreamThemeProvider::default();
        let linear = theme.resolve_linear_color("color.text.primary");
        // Linear values should be lower than sRGB for values > 0.04045.
        let srgb = theme.resolve_color("color.text.primary");
        assert!(linear.x <= srgb.0 || srgb.0 <= 0.04045);
        assert!(linear.w > 0.0); // alpha should be preserved
    }

    #[test]
    fn srgb_to_linear_edge_cases() {
        assert!((super::srgb_to_linear(0.0)).abs() < 0.001);
        assert!((super::srgb_to_linear(1.0) - 1.0).abs() < 0.001);
        // Mid-range: 0.5 sRGB ≈ 0.214 linear
        assert!((super::srgb_to_linear(0.5) - 0.214).abs() < 0.01);
    }
}

#[cfg(test)]
mod contrast_tests {
    use super::*;
    use poodle_adapter::ThemeProvider;

    #[test]
    fn neutral_contrast_matches_gpui_reference() {
        let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let surface = theme.resolve_color("color.background.surface");
        assert!((surface.0 - 17.073 / 255.0).abs() < 0.002, "surface r {}", surface.0);
        let border = theme.resolve_color("color.border.default");
        assert!((border.3 - 0.11).abs() < 0.001, "border alpha {}", border.3);
        let full = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE).with_contrast(1.0);
        let literal = full.resolve_color("color.background.surface");
        assert!((literal.0 - 21.0 / 255.0).abs() < 0.002, "k=1 literal");
        let accent = theme.resolve_color("color.accent.base");
        let accent_full = full.resolve_color("color.accent.base");
        assert_eq!(accent.0, accent_full.0, "accent untouched");
    }
    /// Every semantic size and space token has to resolve to something.
    ///
    /// `resolve_space` returns `0.0` for a token it does not recognise, which
    /// is silent and produces a plausible-looking layout: `Select`'s panel had
    /// `max-height: 0` for months, collapsed to its 8px of padding, and painted
    /// option rows outside its own rect — visible, correctly labelled for
    /// assistive technology, and impossible to click, because hit-testing
    /// requires every ancestor to contain the point.
    ///
    /// Seven tokens were missing when this test was written. It exists so the
    /// next one added to `poodle-tokens` cannot reach a component as a zero.
    #[test]
    fn every_semantic_size_and_space_token_resolves() {
        let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);

        let tokens = [
            poodle_tokens::semantic::SIZE_CONTROL_HEIGHT,
            poodle_tokens::semantic::SIZE_CONTROL_MIN_WIDTH,
            poodle_tokens::semantic::SIZE_ICON_XS,
            poodle_tokens::semantic::SIZE_ICON_SM,
            poodle_tokens::semantic::SIZE_ICON_MD,
            poodle_tokens::semantic::SIZE_ICON_LG,
            poodle_tokens::semantic::SIZE_ICON_XL,
            poodle_tokens::semantic::SIZE_PANEL_HEADER,
            poodle_tokens::semantic::SIZE_LIST_GRID_MIN_ITEM_WIDTH,
            poodle_tokens::semantic::SIZE_MENU_MAX_HEIGHT,
            poodle_tokens::semantic::SIZE_MENU_MIN_WIDTH,
            poodle_tokens::semantic::SIZE_POPOVER_MAX_WIDTH,
            poodle_tokens::semantic::SIZE_HOVER_CARD_MAX_WIDTH,
            poodle_tokens::semantic::SIZE_SELECT_MIN_WIDTH,
            poodle_tokens::semantic::SIZE_DATE_TIME_RANGE_PICKER_MIN_WIDTH,
            poodle_tokens::semantic::SIZE_FILE_UPLOAD_DROP_ZONE_MIN_HEIGHT,
            poodle_tokens::semantic::SPACE_STACK_SM,
            poodle_tokens::semantic::SPACE_STACK_MD,
            poodle_tokens::semantic::SPACE_STACK_LG,
            poodle_tokens::semantic::SPACE_INLINE_XS,
            poodle_tokens::semantic::SPACE_INLINE_SM,
            poodle_tokens::semantic::SPACE_INLINE_MD,
            poodle_tokens::semantic::SPACE_INLINE_LG,
            poodle_tokens::semantic::SPACE_PANEL_X,
            poodle_tokens::semantic::SPACE_PANEL_Y,
            poodle_tokens::semantic::SPACE_CONTROL_X,
            poodle_tokens::semantic::SPACE_CONTROL_Y,
        ];

        let dead: Vec<&str> = tokens
            .iter()
            .copied()
            .filter(|token| theme.resolve_space(token) <= 0.0)
            .collect();

        assert!(
            dead.is_empty(),
            "these tokens resolve to 0 — a component asking for one gets a silent zero: {dead:?}"
        );
    }

}
