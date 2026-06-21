use poodle_tokens::semantic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeadingShape {
    Circle,
    RoundedSquare,
}

impl Default for LeadingShape {
    fn default() -> Self {
        Self::Circle
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeadingFill {
    Tint,
    Solid,
}

impl Default for LeadingFill {
    fn default() -> Self {
        Self::Tint
    }
}

/// Card layout mode. Contract §3/§4: `compact` is a dense single-line variant;
/// `stacked` is a square-ish tile with leading on top and a bottom utility rail.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListCardLayout {
    Default,
    Compact,
    Stacked,
}

impl Default for ListCardLayout {
    fn default() -> Self {
        Self::Default
    }
}

/// Selection-indicator mode. Contract §3: `Checkbox` renders a checkbox
/// selection indicator when the card is selectable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectionIndicator {
    None,
    Checkbox,
}

impl Default for SelectionIndicator {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug)]
pub struct ListCardSpec {
    pub title: String,
    pub subtitle: Option<String>,
    pub meta: Option<String>,
    pub leading_shape: LeadingShape,
    pub leading_fill: LeadingFill,
    /// Relative leading-size step offset from the resolved card size.
    /// Contract §3 `leadingSizeOffset` (default 0): rounded to whole steps and
    /// clamped to the `xs`→`xl` ladder; shifts the leading block + inner icon
    /// together without changing title/meta typography. Each step is 0.25rem on
    /// the leading box (the ladder's nominal step), clamped so the box stays
    /// positive.
    pub leading_size_offset: i32,
    pub accent_color: Option<String>,
    pub is_interactive: bool,
    pub is_disabled: bool,
    pub is_not_live: bool,
    pub sash: Option<String>,
    pub sash_color: Option<String>,
    pub aria_label: Option<String>,
    /// Optional navigation target. When set, the card implicitly
    /// becomes interactive and the consumer is expected to treat a
    /// click as navigation to this href. Matches Svelte `href` prop.
    pub href: Option<String>,
    /// When true the card renders a leading checkbox slot indicating
    /// that it participates in a multi-select group.
    pub is_selectable: bool,
    /// Current selection state when `is_selectable` is true.
    pub is_selected: bool,
    /// When true the card renders a trailing drag handle used to
    /// reorder items in a list.
    pub show_reorder_handle: bool,
    /// Layout mode. Contract §3: default / compact / stacked.
    pub layout: ListCardLayout,
    /// Accent emphasis state. Contract §3/§4/§8 Root highlighted: tints border,
    /// paints an accent-to-transparent gradient, and adds an inset accent ring.
    pub is_highlighted: bool,
    /// Selection-indicator mode. Contract §3: when `Checkbox` and selectable,
    /// renders a checkbox selection indicator.
    pub selection_indicator: SelectionIndicator,
}

impl Default for ListCardSpec {
    fn default() -> Self {
        Self {
            title: String::new(),
            subtitle: None,
            meta: None,
            leading_shape: LeadingShape::Circle,
            leading_fill: LeadingFill::Tint,
            leading_size_offset: 0,
            accent_color: None,
            is_interactive: false,
            is_disabled: false,
            is_not_live: false,
            sash: None,
            sash_color: None,
            aria_label: None,
            href: None,
            is_selectable: false,
            is_selected: false,
            show_reorder_handle: false,
            layout: ListCardLayout::Default,
            is_highlighted: false,
            selection_indicator: SelectionIndicator::None,
        }
    }
}

impl ListCardSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn with_leading_shape(mut self, shape: LeadingShape) -> Self {
        self.leading_shape = shape;
        self
    }

    pub fn with_leading_fill(mut self, fill: LeadingFill) -> Self {
        self.leading_fill = fill;
        self
    }

    /// Contract §3 `leadingSizeOffset`: relative leading-size step offset.
    pub fn with_leading_size_offset(mut self, offset: i32) -> Self {
        self.leading_size_offset = offset;
        self
    }

    pub fn with_interactive(mut self, interactive: bool) -> Self {
        self.is_interactive = interactive;
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn with_not_live(mut self, not_live: bool) -> Self {
        self.is_not_live = not_live;
        self
    }

    pub fn with_sash(mut self, sash: impl Into<String>) -> Self {
        self.sash = Some(sash.into());
        self
    }

    pub fn with_sash_color(mut self, color: impl Into<String>) -> Self {
        self.sash_color = Some(color.into());
        self
    }

    pub fn with_accent_color(mut self, color: impl Into<String>) -> Self {
        self.accent_color = Some(color.into());
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        // A card with an href is implicitly interactive.
        self.is_interactive = true;
        self
    }

    pub fn with_selectable(mut self, selectable: bool) -> Self {
        self.is_selectable = selectable;
        self
    }

    pub fn with_selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    pub fn with_reorder_handle(mut self, show: bool) -> Self {
        self.show_reorder_handle = show;
        self
    }

    pub fn with_layout(mut self, layout: ListCardLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_highlighted(mut self, highlighted: bool) -> Self {
        self.is_highlighted = highlighted;
        self
    }

    pub fn with_selection_indicator(mut self, indicator: SelectionIndicator) -> Self {
        self.selection_indicator = indicator;
        self
    }

    pub fn has_href(&self) -> bool {
        self.href.is_some()
    }

    // ── Token methods ──────────────────────────────────────────

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn hover_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn hover_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_DEFAULT
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn title_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn subtitle_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn meta_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn leading_tint_bg_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn leading_solid_bg_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    /// Title color token. Contract §8 Title: `color.text.primary`.
    pub fn corner_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_TERTIARY
    }

    /// Default sash ribbon background. Contract §8 Sash: positive/green.
    pub fn sash_bg_token(&self) -> &'static str {
        semantic::COLOR_STATUS_SUCCESS
    }

    /// Sash text + solid-leading icon color. Contract §8: `#fff`.
    /// No pure-white semantic token exists; the inverse-text token is the
    /// closest token surface for on-accent foreground. See NOTE in list_card.rs.
    pub fn on_accent_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_INVERSE
    }

    /// Accent base used for highlighted ring/gradient + leading. Contract §8.
    pub fn accent_base_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Leading radius token for rounded-square shape. Contract §8.
    pub fn leading_radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    /// Pill radius for circle leading + checkbox indicator. Contract §8.
    pub fn pill_radius_token(&self) -> &'static str {
        semantic::RADIUS_PILL
    }

    /// Selection-indicator box edge. Contract §8 Selection Indicator (size icon md).
    pub fn selection_indicator_size_token(&self) -> &'static str {
        semantic::SIZE_ICON_MD
    }

    // ── Exact dimensions (contract §7/§8) ──────────────────────────────
    // No tokens exist for these contract-exact rem values; the rem literal is
    // the contract source of truth and `rem_to_px` resolves it at the call site.

    /// Leading square edge length in rem. Contract §7: circle 2rem,
    /// rounded-square 2.75rem. Compact layout shrinks one step.
    /// `leadingSizeOffset` (contract §3) shifts the box by whole 0.25rem steps,
    /// clamped to the `xs`→`xl` ladder span (±2 steps from the shape base, box
    /// kept ≥ 1rem so it never collapses).
    pub fn leading_size_rem(&self) -> f32 {
        let base = match self.leading_shape {
            LeadingShape::Circle => 2.0,
            LeadingShape::RoundedSquare => 2.75,
        };
        let compact_adjust = if self.layout == ListCardLayout::Compact {
            -0.25
        } else {
            0.0
        };
        let step = self.leading_size_offset.clamp(-2, 2) as f32;
        (base + compact_adjust + step * 0.25).max(1.0)
    }

    /// Leading icon glyph font-size in rem. Contract §8 Leading: `0.875rem`.
    pub fn leading_font_size_rem(&self) -> f32 {
        0.875
    }

    /// Body column gap in rem. Contract §8 Body: `0.0625rem`.
    pub fn body_gap_rem(&self) -> f32 {
        0.0625
    }

    /// Header row gap in rem. Contract §8 Header: `0.375rem`.
    pub fn header_gap_rem(&self) -> f32 {
        0.375
    }

    /// Footer row gap in rem. Contract §8 Footer: `0.5rem`.
    pub fn footer_gap_rem(&self) -> f32 {
        0.5
    }

    /// Subtitle / meta font-size in rem. Contract §8: `0.75rem`.
    pub fn small_font_size_rem(&self) -> f32 {
        0.75
    }

    /// Tint ratio for leading background. Contract §8 Leading: `accent 12%`.
    pub fn leading_tint_ratio(&self) -> f32 {
        0.12
    }

    /// Not-live opacity. Contract §4/§8: `0.72`.
    pub fn not_live_opacity(&self) -> f32 {
        0.72
    }
}
