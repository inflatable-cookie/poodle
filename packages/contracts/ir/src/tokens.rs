//! Token and recipe references, resolved against `poodle-tokens`.
//!
//! Serves `CROSS-09` (theme axis with semantic token paths resolved per
//! theme; the recipe-hook override chain `--poodle-recipe-*` → component var
//! → token) and the per-component token rows (`BTN-22`, `RNG-21`, `TXT-27`),
//! per spec 063 "Component IR" ("semantic token and appearance-recipe hook
//! references") and `docs/architecture/007-appearance-recipe-contract.md`.
//!
//! `poodle-tokens` is this crate's only in-repo dependency; [`TokenRef`]
//! paths are validated against the actual generated semantic constants, so a
//! token rename or removal breaks validation, not silently the fixture.

use serde::{Deserialize, Serialize};

/// A declared semantic token reference (`CROSS-09`). `path` must resolve
/// against the `poodle-tokens` semantic registry
/// ([`semantic_token_paths`]) or validation reports an `InvalidToken`
/// finding with the offending path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenRef {
    /// Token path as authored in `poodle-tokens`, e.g. `color.accent.base`,
    /// `radius.control`, `typography.label.size` (`BTN-22`, `RNG-21`,
    /// `TXT-27`).
    pub path: String,
    /// Token group the path belongs to.
    pub group: TokenGroup,
    /// Why the component references this token, citing the contract section.
    pub description: String,
}

/// Token group, mirroring the `poodle-tokens` module surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenGroup {
    /// Semantic token path (`poodle_tokens::semantic::*`), e.g.
    /// `color.accent.base` (`CROSS-09`).
    #[serde(rename = "semantic")]
    Semantic,
    /// Typed primitive value (`poodle_tokens::typed::*`).
    #[serde(rename = "primitive")]
    Primitive,
    /// Theme preset (`poodle_tokens::themes::*`), e.g. `clay`, `cobalt`
    /// (`SHELL-01`).
    #[serde(rename = "theme")]
    Theme,
    /// Density or control-size definition (`poodle_tokens::density::*`),
    /// e.g. `compact`, `xs` (`CROSS-08`, `SHELL-03`).
    #[serde(rename = "density")]
    Density,
    /// Token metadata (`poodle_tokens::metadata::*`).
    #[serde(rename = "metadata")]
    Metadata,
}

/// A recipe-hook reference — the `--poodle-recipe-*` override chain
/// (`CROSS-09`; `B/R/T §8` recipe hooks;
/// `docs/architecture/007-appearance-recipe-contract.md`).
///
/// The chain is declared first-class: recipe hook → component variable →
/// token, in resolution order, e.g. the Button fill chain
/// `--poodle-recipe-button-fill` → `--poodle-button-fill` →
/// `color.accent.base` (`BTN-22`), the RangeSlider 11-hook family
/// (`RNG-21`), the TextInput fill/border/shadow family (`TXT-27`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeHookRef {
    /// The recipe hook name, e.g. `--poodle-recipe-button-fill` (`BTN-22`).
    pub hook: String,
    /// The override chain in resolution order (`CROSS-09`; the documented
    /// "recipe hook → component var → token" precedence).
    pub chain: Vec<RecipeLink>,
    /// What the hook controls, citing the contract section.
    pub description: String,
}

/// One link in a recipe override chain (`CROSS-09`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeLink {
    /// Link kind.
    pub kind: RecipeLinkKind,
    /// Target name: `--poodle-recipe-*` hook, component custom property, or
    /// token path.
    pub target: String,
}

/// Kind of a recipe override link (`CROSS-09`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecipeLinkKind {
    /// The `--poodle-recipe-*` hook itself.
    #[serde(rename = "recipe-hook")]
    RecipeHook,
    /// The component-level custom property, e.g. `--poodle-button-fill`.
    #[serde(rename = "component-variable")]
    ComponentVariable,
    /// A semantic token path resolved against `poodle-tokens`.
    #[serde(rename = "token")]
    Token,
}

/// A declared metric value on an axis ladder or density adjustment
/// (`CROSS-07`, `CROSS-08`; `BTN-23`, `BTN-05`, `RNG-09`, `TXT-15`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricValue {
    /// A rem dimension, e.g. `0.125` for `0.125rem` (`BTN-23` fixed rem
    /// heights; `TXT-15` density deltas).
    #[serde(rename = "rem")]
    Rem(f64),
    /// A pixel dimension, e.g. focus-ring widths.
    #[serde(rename = "px")]
    Px(f64),
    /// A unitless fraction, e.g. `0.5` opacity or ratio (`BTN-10` chevron
    /// opacity `0.5`).
    #[serde(rename = "fraction")]
    Fraction(f64),
    /// A text value, e.g. a font-family name or easing curve.
    #[serde(rename = "text")]
    Text(String),
}

/// Registry of valid semantic token paths, transcribed from the generated
/// `poodle-tokens` semantic constants. Used by
/// [`validate`](crate::validate) to resolve [`TokenRef`] paths (`CROSS-09`
/// "typed, resolved against `poodle-tokens`"). Compile-checked: a token
/// constant rename breaks this crate.
pub fn semantic_token_paths() -> Vec<&'static str> {
    vec![
        poodle_tokens::semantic::BORDER_WIDTH_DEFAULT,
        poodle_tokens::semantic::BORDER_WIDTH_FOCUS,
        poodle_tokens::semantic::COLOR_BACKGROUND_CANVAS,
        poodle_tokens::semantic::COLOR_BACKGROUND_SURFACE,
        poodle_tokens::semantic::COLOR_BACKGROUND_PANEL,
        poodle_tokens::semantic::COLOR_BACKGROUND_ELEVATED,
        poodle_tokens::semantic::COLOR_BACKGROUND_OVERLAY,
        poodle_tokens::semantic::COLOR_TEXT_PRIMARY,
        poodle_tokens::semantic::COLOR_TEXT_SECONDARY,
        poodle_tokens::semantic::COLOR_TEXT_TERTIARY,
        poodle_tokens::semantic::COLOR_TEXT_INVERSE,
        poodle_tokens::semantic::COLOR_BORDER_SUBTLE,
        poodle_tokens::semantic::COLOR_BORDER_DEFAULT,
        poodle_tokens::semantic::COLOR_BORDER_STRONG,
        poodle_tokens::semantic::COLOR_ACCENT_BASE,
        poodle_tokens::semantic::COLOR_ACCENT_HOVER,
        poodle_tokens::semantic::COLOR_ACCENT_FOCUS_RING,
        poodle_tokens::semantic::COLOR_STATUS_SUCCESS,
        poodle_tokens::semantic::COLOR_STATUS_WARNING,
        poodle_tokens::semantic::COLOR_STATUS_DANGER,
        poodle_tokens::semantic::COLOR_STATUS_INFO,
        poodle_tokens::semantic::COLOR_ICON_PRIMARY,
        poodle_tokens::semantic::COLOR_ICON_MUTED,
        poodle_tokens::semantic::ELEVATION_SURFACE,
        poodle_tokens::semantic::ELEVATION_OVERLAY,
        poodle_tokens::semantic::ELEVATION_DIALOG,
        poodle_tokens::semantic::ICON_SIZE_DEFAULT,
        poodle_tokens::semantic::ICON_STROKE_DEFAULT,
        poodle_tokens::semantic::MOTION_DURATION_INTERACTION,
        poodle_tokens::semantic::MOTION_DURATION_OVERLAY,
        poodle_tokens::semantic::MOTION_EASING_STANDARD,
        poodle_tokens::semantic::MOTION_EASING_EMPHASIZED,
        poodle_tokens::semantic::OVERLAY_Z_MENU,
        poodle_tokens::semantic::OVERLAY_Z_DIALOG,
        poodle_tokens::semantic::OVERLAY_Z_TOAST,
        poodle_tokens::semantic::OVERLAY_SCRIM_OPACITY,
        poodle_tokens::semantic::RADIUS_CONTROL,
        poodle_tokens::semantic::RADIUS_SURFACE,
        poodle_tokens::semantic::RADIUS_PILL,
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
        poodle_tokens::semantic::SPACE_BUTTON_GAP,
        poodle_tokens::semantic::SPACE_BUTTON_ICON_INSET,
        poodle_tokens::semantic::STATE_OPACITY_DISABLED,
        poodle_tokens::semantic::STATE_OPACITY_MUTED,
        poodle_tokens::semantic::TYPOGRAPHY_BODY_FAMILY,
        poodle_tokens::semantic::TYPOGRAPHY_BODY_SIZE,
        poodle_tokens::semantic::TYPOGRAPHY_BODY_LINE_HEIGHT,
        poodle_tokens::semantic::TYPOGRAPHY_BODY_WEIGHT,
        poodle_tokens::semantic::TYPOGRAPHY_LABEL_FAMILY,
        poodle_tokens::semantic::TYPOGRAPHY_LABEL_SIZE,
        poodle_tokens::semantic::TYPOGRAPHY_LABEL_LINE_HEIGHT,
        poodle_tokens::semantic::TYPOGRAPHY_LABEL_WEIGHT,
        poodle_tokens::semantic::TYPOGRAPHY_COUNTER_SIZE,
        poodle_tokens::semantic::TYPOGRAPHY_CAPTION_FAMILY,
        poodle_tokens::semantic::TYPOGRAPHY_CAPTION_SIZE,
        poodle_tokens::semantic::TYPOGRAPHY_CAPTION_LINE_HEIGHT,
        poodle_tokens::semantic::TYPOGRAPHY_CAPTION_WEIGHT,
        poodle_tokens::semantic::TYPOGRAPHY_HEADING_FAMILY,
        poodle_tokens::semantic::TYPOGRAPHY_HEADING_SIZE,
        poodle_tokens::semantic::TYPOGRAPHY_HEADING_LINE_HEIGHT,
        poodle_tokens::semantic::TYPOGRAPHY_HEADING_WEIGHT,
        poodle_tokens::semantic::TYPOGRAPHY_CODE_FAMILY,
        poodle_tokens::semantic::TYPOGRAPHY_CODE_ADJUSTMENT_RATIO,
        poodle_tokens::semantic::TYPOGRAPHY_CODE_SIZE,
        poodle_tokens::semantic::TYPOGRAPHY_CODE_LINE_HEIGHT,
        poodle_tokens::semantic::TYPOGRAPHY_CODE_WEIGHT,
    ]
}

/// Registry of valid theme preset names, from the generated
/// `poodle-tokens` theme definitions (`SHELL-01` theme selection; `CROSS-09`
/// theme axis).
pub fn theme_names() -> Vec<&'static str> {
    vec![
        poodle_tokens::themes::CLAY.name,
        poodle_tokens::themes::COBALT.name,
        poodle_tokens::themes::ECLIPSE.name,
        poodle_tokens::themes::FOREST.name,
        poodle_tokens::themes::GRAPHITE.name,
        poodle_tokens::themes::HORNET.name,
        poodle_tokens::themes::ICEBERG.name,
        poodle_tokens::themes::MEADOW.name,
        poodle_tokens::themes::MIDNIGHT.name,
        poodle_tokens::themes::NORD.name,
        poodle_tokens::themes::ROSE.name,
        poodle_tokens::themes::SOLARIZED.name,
    ]
}

/// Registry of valid density preset names, from the generated `poodle-tokens`
/// density definitions (`SHELL-03`; `CROSS-08`).
pub fn density_names() -> Vec<&'static str> {
    vec![
        poodle_tokens::density::COMPACT.name,
        poodle_tokens::density::DEFAULT.name,
        poodle_tokens::density::COMFORTABLE.name,
    ]
}

/// Registry of valid control-size names, from the generated `poodle-tokens`
/// control-size definitions (`SHELL-02`; `CROSS-07`).
pub fn control_size_names() -> Vec<&'static str> {
    vec![
        poodle_tokens::density::CONTROL_SIZE_XS.name,
        poodle_tokens::density::CONTROL_SIZE_SM.name,
        poodle_tokens::density::CONTROL_SIZE_MD.name,
        poodle_tokens::density::CONTROL_SIZE_LG.name,
        poodle_tokens::density::CONTROL_SIZE_XL.name,
    ]
}
