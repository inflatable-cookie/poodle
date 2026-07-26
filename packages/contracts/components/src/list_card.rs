use poodle_tokens::semantic;
use crate::types::{ControlDensity, ControlSize, MenuEntry, SemanticControlSizeRole};

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

/// What opens a ListCard's context menu.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ListCardContextMenuTrigger {
    /// A right-click anywhere on the card.
    #[default]
    Context,
    /// A click on the leading slot, for a card whose row is otherwise a link.
    Leading,
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
    /// Presentation axes (contract §3): size is intrinsic, density is sibling
    /// spacing, size_role resolves size from the inherited presentation.
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
    /// Context-menu entries; empty means the card has no context menu.
    pub context_menu_items: Vec<MenuEntry>,
    /// What opens the context menu: a right-click, or the leading slot.
    pub context_menu_trigger: ListCardContextMenuTrigger,
    /// Accessible name for the context menu.
    pub context_menu_aria_label: Option<String>,
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
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
            context_menu_items: Vec::new(),
            context_menu_trigger: ListCardContextMenuTrigger::Context,
            context_menu_aria_label: None,
        }
    }
}

impl ListCardSpec {
    pub fn with_context_menu_items(mut self, items: Vec<MenuEntry>) -> Self {
        self.context_menu_items = items;
        self
    }

    pub fn with_context_menu_trigger(mut self, trigger: ListCardContextMenuTrigger) -> Self {
        self.context_menu_trigger = trigger;
        self
    }

    pub fn with_context_menu_aria_label(mut self, label: impl Into<String>) -> Self {
        self.context_menu_aria_label = Some(label.into());
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }

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
    /// The size ladder the leading box sits on: the resolved control size,
    /// shifted by `leading_size_offset` and clamped to the `xs`→`xl` span.
    ///
    /// Mirrors the Svelte `offsetControlSize(resolvedSize, leadingSizeOffset)`.
    pub fn resolved_leading_size(&self) -> ControlSize {
        const LADDER: [ControlSize; 5] = [
            ControlSize::Xs,
            ControlSize::Sm,
            ControlSize::Md,
            ControlSize::Lg,
            ControlSize::Xl,
        ];
        let resolved = crate::types::resolve_semantic_control_size(self.size, self.size_role);
        let base = LADDER.iter().position(|s| *s == resolved).unwrap_or(2) as i32;
        let index = (base + self.leading_size_offset).clamp(0, LADDER.len() as i32 - 1);
        LADDER[index as usize]
    }

    /// Leading box edge in rem.
    ///
    /// This used to derive from `leading_shape` alone, so a `ListCard` rendered
    /// at the same box size whatever `size` the host asked for — the prop was
    /// carried and ignored. The ladder below is the `data-leading-size` table
    /// from `list-card.css`.
    pub fn leading_size_rem(&self) -> f32 {
        match self.resolved_leading_size() {
            ControlSize::Xs => 1.75,
            ControlSize::Sm => 2.0,
            ControlSize::Md => 2.25,
            ControlSize::Lg => 2.75,
            ControlSize::Xl => 3.0,
        }
    }

    /// Leading glyph size in rem — the same ladder's icon row.
    pub fn leading_icon_size_rem(&self) -> f32 {
        match self.resolved_leading_size() {
            ControlSize::Xs => 0.875,
            ControlSize::Sm => 1.0,
            ControlSize::Md => 1.125,
            ControlSize::Lg => 1.375,
            ControlSize::Xl => 1.5,
        }
    }

    /// Leading text font-size in rem — the ladder's font row. Was a flat
    /// `0.875rem`, which is the `md` cell.
    pub fn leading_font_size_rem(&self) -> f32 {
        match self.resolved_leading_size() {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.75,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 1.0,
            ControlSize::Xl => 1.125,
        }
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

#[cfg(test)]
mod leading_size_tests {
    use super::*;

    /// The leading box used to derive from `leading_shape` alone, so `size` was
    /// carried and ignored — every card drew the same box. These values are the
    /// `data-leading-size` ladder from `list-card.css`.
    #[test]
    fn the_box_follows_the_size_ladder() {
        let cases = [
            (ControlSize::Xs, 1.75, 0.875, 0.6875),
            (ControlSize::Sm, 2.0, 1.0, 0.75),
            (ControlSize::Md, 2.25, 1.125, 0.875),
            (ControlSize::Lg, 2.75, 1.375, 1.0),
            (ControlSize::Xl, 3.0, 1.5, 1.125),
        ];

        for (size, box_rem, icon_rem, font_rem) in cases {
            let spec = ListCardSpec::new().with_size(size);
            assert_eq!(spec.leading_size_rem(), box_rem, "{size:?} box");
            // `leading_icon_size_rem` has no in-repo caller — the leading slot
            // is host-provided — so the test is what keeps it from drifting
            // away from the stylesheet.
            assert_eq!(spec.leading_icon_size_rem(), icon_rem, "{size:?} icon");
            assert_eq!(spec.leading_font_size_rem(), font_rem, "{size:?} font");
        }
    }

    /// The offset walks the same ladder and clamps at both ends, so a large
    /// offset can never fall off the scale.
    #[test]
    fn the_offset_walks_and_clamps() {
        let up = ListCardSpec::new()
            .with_size(ControlSize::Md)
            .with_leading_size_offset(1);
        assert_eq!(up.resolved_leading_size(), ControlSize::Lg);

        let down = ListCardSpec::new()
            .with_size(ControlSize::Md)
            .with_leading_size_offset(-1);
        assert_eq!(down.resolved_leading_size(), ControlSize::Sm);

        let past_the_top = ListCardSpec::new()
            .with_size(ControlSize::Xl)
            .with_leading_size_offset(5);
        assert_eq!(past_the_top.resolved_leading_size(), ControlSize::Xl);

        let past_the_bottom = ListCardSpec::new()
            .with_size(ControlSize::Xs)
            .with_leading_size_offset(-5);
        assert_eq!(past_the_bottom.resolved_leading_size(), ControlSize::Xs);
    }

    /// `size_role` shifts the base before the offset applies.
    #[test]
    fn the_size_role_shifts_the_base() {
        let chrome = ListCardSpec::new()
            .with_size(ControlSize::Md)
            .with_size_role(SemanticControlSizeRole::Chrome);
        assert_eq!(chrome.resolved_leading_size(), ControlSize::Sm);
    }
}

#[cfg(test)]
mod context_menu_tests {
    use super::*;

    /// Neither native target draws a ListCard context menu yet. The spec
    /// carrying the entries is the precondition for that, not the feature —
    /// without it a renderer had nothing to read.
    #[test]
    fn a_card_has_no_context_menu_until_it_is_given_entries() {
        let plain = ListCardSpec::new();
        assert!(plain.context_menu_items.is_empty());
        assert_eq!(
            plain.context_menu_trigger,
            ListCardContextMenuTrigger::Context,
        );
    }

    /// The leading trigger exists for a card whose whole row is already a link,
    /// where a right-click is the browser's, not the component's.
    #[test]
    fn the_trigger_can_move_to_the_leading_slot() {
        let spec = ListCardSpec::new()
            .with_context_menu_trigger(ListCardContextMenuTrigger::Leading);
        assert_eq!(
            spec.context_menu_trigger,
            ListCardContextMenuTrigger::Leading,
        );
    }
}
