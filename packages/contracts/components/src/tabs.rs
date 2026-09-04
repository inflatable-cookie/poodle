use poodle_tokens::semantic;

use crate::types::{
    ControlDensity, ControlSize, Orientation, SemanticControlSizeRole, TabActivationMode,
    TabDefinition, TabVariant,
};

/// How a `Tabs` strip responds to running out of width.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TabsOverflowStrategy {
    /// One threshold: the whole strip becomes a menu. The historical behaviour.
    #[default]
    Collapse,
    /// Give up decoration before collapsing — see `TabsShedPart`.
    Shed,
}

/// How a `Tabs` fills its container. Matches the Svelte/React `layout` prop
/// (`"auto" | "fill"`, default `"auto"`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TabsLayout {
    /// Natural-height grid. The historical behaviour.
    #[default]
    Auto,
    /// The root takes its container's block size and the active panel fills
    /// and scrolls within it; the strip keeps its natural height. Requires a
    /// sized container. Orientation-independent.
    Fill,
}

/// Selection treatment on the active tab. Matches the Svelte/React
/// `activeFill` prop (`"none" | "tint" | "solid"`, default `"tint"`). Shared
/// type — see `docs/contracts/004-shared-control-types.md`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ActiveFill {
    /// No selection fill — selection is carried by the edge and the selected
    /// text colour alone. The off value of the fill axis, symmetric with
    /// `ActiveEdge::None`.
    None,
    /// Accent-tinted selection on the active tab.
    #[default]
    Tint,
    /// Fully accent-filled active tab with an inverse foreground.
    Solid,
}

/// Selection edge on the active control. Matches the Svelte/React
/// `activeEdge` prop (`"none" | "outline" | "underline"`, default `"none"`).
/// Shared type — see `docs/contracts/004-shared-control-types.md`.
///
/// One enum, not booleans: `outline` and `underline` are both borders on the
/// active control and conflict on the same property, so a boolean pair would
/// admit nonsense combinations. Exactly one value applies.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ActiveEdge {
    /// No edge drawn.
    #[default]
    None,
    /// Accent border around the active control.
    Outline,
    /// Accent edge along the inline-end side (bottom horizontal, right vertical).
    Underline,
}

/// A part a `Tabs` strip may give up to keep its labels.
///
/// Labels are deliberately absent: shedding one would leave a tab as an unnamed
/// glyph, and any level that did drop them would need forced tooltips.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TabsShedPart {
    Icon,
    Count,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TabsSpec {
    pub tabs: Vec<TabDefinition>,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub variant: TabVariant,
    /// Selection edge on the active tab — see `ActiveEdge`. Matches Svelte
    /// `activeEdge` (default `"none"`).
    pub active_edge: ActiveEdge,
    /// Selection treatment on the active tab: none (no fill), tint, or fully
    /// accent-filled. Matches Svelte `activeFill` (default `"tint"`).
    pub active_fill: ActiveFill,
    pub orientation: Orientation,
    /// Fill-layout seam — see `TabsLayout`. Matches Svelte `layout`
    /// (default `"auto"`). Contract §7.
    pub layout: TabsLayout,
    pub activation_mode: TabActivationMode,
    pub aria_label: Option<String>,
    /// When true, tabs can be reordered via drag. Defers to the
    /// consumer to actually commit the new order. Matches Svelte
    /// `reorderable` prop.
    pub is_reorderable: bool,
    /// When true, the Card variant renders the bottom border
    /// line under the whole tab list. When false (the default since g13-020)
    /// the list renders flush to its container — useful for titlebars and
    /// toolbars. Matches Svelte `bordered` prop (default false).
    pub is_bordered: bool,
    /// When true (and orientation is horizontal), tabs flex to fill the
    /// row at equal widths with centered labels. Matches Svelte
    /// `fullWidth` prop / `data-full-width` (default false). Contract §8
    /// Full-width table.
    pub is_full_width: bool,
    /// Optional key used to persist the active tab across sessions.
    /// The consumer owns the actual storage; this field carries the
    /// key so the render surface can expose a stable id for tests /
    /// state.
    pub history_key: Option<String>,
    /// The tab currently being dragged, if any. Transient host-set state
    /// during a reorder drag. Drives the drag-source visual (opacity 0.4).
    /// Mirrors the tree's `drag_value` convention.
    pub drag_value: Option<String>,
    /// The tab currently under the drag (the drop target), if any. Transient
    /// host-set state during a reorder drag. Drives the drop-target visual
    /// (inset accent ring). Mirrors the tree's `drop_target_value` convention.
    pub drop_target_value: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Whether hovering a tab shows its label as a tooltip.
    pub shows_tooltips: bool,
    /// Whether the strip collapses into a menu when it overflows.
    pub collapse_when_overflow: bool,
    /// What to do as the strip stops fitting.
    ///
    /// `Collapse` is the single threshold into a menu. `Shed` gives up
    /// decoration first — full, then without icons, then without counts —
    /// keeping labels at every level. Contract §3.
    pub overflow_strategy: TabsOverflowStrategy,
    /// Which parts to give up, in order, under `Shed`.
    ///
    /// Icons first by default: an icon usually repeats what the label already
    /// says, where a count carries information the label does not.
    pub shed: Vec<TabsShedPart>,
    /// Label for the overflow-collapse control.
    pub collapse_label: Option<String>,
    /// The semantic drag family this strip belongs to.
    ///
    /// `None` means a family scoped to the mounted Tabs instance, so ordinary
    /// tab sets never accept one another's rows even under one controller. An
    /// owning composite sets an explicit kind to place the strip in a shared
    /// family — DockRegion uses `poodle.dock-panel` — without taking over
    /// reorder, which stays Tabs' own. Registration ids remain instance-scoped
    /// either way; this is subject vocabulary, not identity.
    pub drag_subject_kind: Option<String>,
}

impl Default for TabsSpec {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            value: None,
            default_value: None,
            variant: TabVariant::Card,
            active_edge: ActiveEdge::None,
            active_fill: ActiveFill::Tint,
            orientation: Orientation::Horizontal,
            layout: TabsLayout::Auto,
            activation_mode: TabActivationMode::Automatic,
            aria_label: None,
            is_reorderable: false,
            is_bordered: false,
            is_full_width: false,
            history_key: None,
            drag_value: None,
            drop_target_value: None,
            size: None,
            size_role: SemanticControlSizeRole::Chrome,
            density: None,
            shows_tooltips: false,
            collapse_when_overflow: false,
            overflow_strategy: TabsOverflowStrategy::Collapse,
            shed: vec![TabsShedPart::Icon, TabsShedPart::Count],
            collapse_label: None,
            drag_subject_kind: None,
        }
    }
}

impl TabsSpec {
    pub fn new(tabs: Vec<TabDefinition>) -> Self {
        Self {
            tabs,
            ..Self::default()
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_reorderable(mut self, is_reorderable: bool) -> Self {
        self.is_reorderable = is_reorderable;
        self
    }

    pub fn with_bordered(mut self, is_bordered: bool) -> Self {
        self.is_bordered = is_bordered;
        self
    }

    pub fn with_full_width(mut self, is_full_width: bool) -> Self {
        self.is_full_width = is_full_width;
        self
    }

    /// Set the tab currently being dragged (drag-source). `None` clears it.
    pub fn with_drag_value(mut self, drag_value: Option<String>) -> Self {
        self.drag_value = drag_value;
        self
    }

    /// Set the tab currently under the drag (drop-target). `None` clears it.
    pub fn with_drop_target_value(mut self, drop_target_value: Option<String>) -> Self {
        self.drop_target_value = drop_target_value;
        self
    }

    /// True when `value` is the tab currently being dragged (drag-source).
    pub fn is_drag_value(&self, value: &str) -> bool {
        self.drag_value.as_deref() == Some(value)
    }

    /// True when `value` is the tab currently under the drag (drop-target).
    pub fn is_drop_target(&self, value: &str) -> bool {
        self.drop_target_value.as_deref() == Some(value)
    }

    /// True when the full-width flex layout applies: `fullWidth` set and the
    /// orientation is horizontal (contract §8 Full-width is non-vertical only).
    pub fn uses_full_width(&self) -> bool {
        self.is_full_width && self.orientation == Orientation::Horizontal
    }

    /// True when the tablist renders vertically (icon-only collapse, border on
    /// the inline-end edge). Contract §7 + §8 vertical tables.
    pub fn is_vertical(&self) -> bool {
        self.orientation == Orientation::Vertical
    }

    pub fn with_history_key(mut self, history_key: impl Into<String>) -> Self {
        self.history_key = Some(history_key.into());
        self
    }

    pub fn with_default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn with_variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the selection edge on the active tab (none, outline, or underline).
    pub fn with_active_edge(mut self, active_edge: ActiveEdge) -> Self {
        self.active_edge = active_edge;
        self
    }

    /// Set the selection treatment on the active tab (none, tint, or solid).
    pub fn with_active_fill(mut self, active_fill: ActiveFill) -> Self {
        self.active_fill = active_fill;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the fill-layout seam (natural-height grid, or root fills its
    /// container with a scrolling panel).
    pub fn with_layout(mut self, layout: TabsLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_activation_mode(mut self, activation_mode: TabActivationMode) -> Self {
        self.activation_mode = activation_mode;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_value(&self) -> Option<&str> {
        self.value
            .as_deref()
            .or(self.default_value.as_deref())
            .or_else(|| {
                self.tabs
                    .iter()
                    .find(|tab| !tab.is_disabled)
                    .map(|tab| tab.value.as_str())
            })
    }

    pub fn selected_tab(&self) -> Option<&TabDefinition> {
        let current = self.current_value()?;
        self.tabs.iter().find(|tab| tab.value == current)
    }

    pub fn uses_manual_activation(&self) -> bool {
        self.activation_mode == TabActivationMode::Manual
    }

    pub fn list_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    pub fn indicator_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn list_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn pill_border_opacity(&self) -> f32 {
        0.68
    }

    pub fn pill_active_bg_opacity(&self) -> f32 {
        0.18
    }

    /// Block variant: opacity of the panel background tint applied to the
    /// tab list. Matches Svelte `color-mix(... panel 90%, transparent)`.
    pub fn block_list_bg_opacity(&self) -> f32 {
        0.9
    }

    /// Block variant: opacity of the accent tint layered onto the surface
    /// for the selected item. Matches Svelte `accent 14% + surface`.
    pub fn block_selected_accent_mix(&self) -> f32 {
        0.14
    }

    /// Block variant: opacity of the accent tint on hover of a selected
    /// item. Matches Svelte `accent 18% + surface`.
    pub fn block_selected_hover_accent_mix(&self) -> f32 {
        0.18
    }

    /// Block variant: opacity of the separator border between items.
    /// Matches Svelte `border-subtle 72%`.
    pub fn block_separator_opacity(&self) -> f32 {
        0.72
    }

    /// Block variant: opacity of the elevated background used on unselected
    /// item hover. Matches Svelte `elevated 50%`.
    pub fn block_hover_bg_opacity(&self) -> f32 {
        0.5
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }

    /// Project tab labels as native hover tooltips. Vertical strips also
    /// project even when this is false, matching web `hasTooltips`.
    pub fn with_shows_tooltips(mut self, shows_tooltips: bool) -> Self {
        self.shows_tooltips = shows_tooltips;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drag_state_defaults_none() {
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A")]);
        assert!(spec.drag_value.is_none());
        assert!(spec.drop_target_value.is_none());
        assert!(!spec.is_drag_value("a"));
        assert!(!spec.is_drop_target("a"));
    }

    #[test]
    fn drag_builders_set_drag_and_drop_target() {
        let spec = TabsSpec::new(vec![
            TabDefinition::new("a", "A"),
            TabDefinition::new("b", "B"),
            TabDefinition::new("c", "C"),
        ])
        .with_drag_value(Some("a".into()))
        .with_drop_target_value(Some("c".into()));

        assert!(spec.is_drag_value("a"));
        assert!(!spec.is_drag_value("c"));
        assert!(spec.is_drop_target("c"));
        assert!(!spec.is_drop_target("a"));
    }

    #[test]
    fn drag_builders_clear_with_none() {
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A")])
            .with_drag_value(Some("a".into()))
            .with_drag_value(None)
            .with_drop_target_value(Some("a".into()))
            .with_drop_target_value(None);
        assert!(spec.drag_value.is_none());
        assert!(spec.drop_target_value.is_none());
    }

    #[test]
    fn active_decorations_default_off_and_tint() {
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A")]);
        assert_eq!(spec.active_edge, ActiveEdge::None);
        assert_eq!(spec.active_fill, ActiveFill::Tint);
        assert_eq!(spec.variant, TabVariant::Card);
        assert!(!spec.is_bordered, "bordered defaults to false (g13-020 R3)");
    }

    #[test]
    fn active_decorations_builders_set_both() {
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A")])
            .with_active_edge(ActiveEdge::Outline)
            .with_active_fill(ActiveFill::Solid);
        assert_eq!(spec.active_edge, ActiveEdge::Outline);
        assert_eq!(spec.active_fill, ActiveFill::Solid);
    }

    #[test]
    fn shows_tooltips_defaults_false_and_builder_sets_it() {
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A")]);
        assert!(!spec.shows_tooltips);
        assert!(spec.with_shows_tooltips(true).shows_tooltips);
    }

    #[test]
    fn spec_defaults_and_builder_accessors() {
        let default_spec = TabsSpec::default();
        assert!(default_spec.tabs.is_empty());
        assert_eq!(default_spec.orientation, Orientation::Horizontal);
        assert_eq!(default_spec.activation_mode, TabActivationMode::Automatic);
        assert!(!default_spec.is_reorderable);
        assert!(!default_spec.is_bordered);
        assert!(!default_spec.is_full_width);
        assert!(!default_spec.is_vertical());
        assert!(!default_spec.uses_full_width());
        assert!(!default_spec.uses_manual_activation());
        assert_eq!(default_spec.layout, TabsLayout::Auto);
        assert_eq!(default_spec.size_role, SemanticControlSizeRole::Chrome);
        assert_eq!(default_spec.overflow_strategy, TabsOverflowStrategy::Collapse);

        let custom = TabsSpec::new(vec![
            TabDefinition::new("x", "X"),
            TabDefinition::new("y", "Y"),
        ])
        .with_value("x")
        .with_default_value("y")
        .with_orientation(Orientation::Vertical)
        .with_activation_mode(TabActivationMode::Manual)
        .with_variant(TabVariant::Pill)
        .with_reorderable(true)
        .with_bordered(true)
        .with_full_width(true)
        .with_aria_label("Custom Tabs")
        .with_history_key("custom-tabs-history")
        .with_size(ControlSize::Lg)
        .with_layout(TabsLayout::Fill)
        .with_size_role(SemanticControlSizeRole::Control)
        .with_density(ControlDensity::Compact);

        assert_eq!(custom.value.as_deref(), Some("x"));
        assert_eq!(custom.default_value.as_deref(), Some("y"));
        assert_eq!(custom.orientation, Orientation::Vertical);
        assert!(custom.is_vertical());
        assert!(!custom.uses_full_width(), "vertical tabs do not use full width");
        assert!(custom.uses_manual_activation());
        assert_eq!(custom.variant, TabVariant::Pill);
        assert!(custom.is_reorderable);
        assert!(custom.is_bordered);
        assert_eq!(custom.aria_label.as_deref(), Some("Custom Tabs"));
        assert_eq!(custom.history_key.as_deref(), Some("custom-tabs-history"));
        assert_eq!(custom.size, Some(ControlSize::Lg));
        assert_eq!(custom.size_role, SemanticControlSizeRole::Control);
        assert_eq!(custom.density, Some(ControlDensity::Compact));
        assert_eq!(custom.layout, TabsLayout::Fill);
    }

    #[test]
    fn layout_defaults_auto_and_builder_sets_fill() {
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A")]);
        assert_eq!(spec.layout, TabsLayout::Auto);
        assert_eq!(spec.with_layout(TabsLayout::Fill).layout, TabsLayout::Fill);
    }

    #[test]
    fn current_value_resolution_order() {
        // 1. Explicit value wins
        let spec_val = TabsSpec::new(vec![
            TabDefinition::new("first", "First"),
            TabDefinition::new("second", "Second"),
        ])
        .with_value("second")
        .with_default_value("first");
        assert_eq!(spec_val.current_value(), Some("second"));
        assert_eq!(spec_val.selected_tab().map(|t| t.label.as_str()), Some("Second"));

        // 2. Default value applies when explicit value is None
        let spec_def = TabsSpec::new(vec![
            TabDefinition::new("first", "First"),
            TabDefinition::new("second", "Second"),
        ])
        .with_default_value("second");
        assert_eq!(spec_def.current_value(), Some("second"));

        // 3. First non-disabled tab applies when value and default_value are None
        let spec_fallback = TabsSpec::new(vec![
            TabDefinition::new("skip", "Skip").with_disabled(true),
            TabDefinition::new("first_enabled", "First Enabled"),
            TabDefinition::new("third", "Third"),
        ]);
        assert_eq!(spec_fallback.current_value(), Some("first_enabled"));

        // 4. Empty tabs returns None
        let empty_spec = TabsSpec::new(vec![]);
        assert_eq!(empty_spec.current_value(), None);
        assert!(empty_spec.selected_tab().is_none());
    }

    #[test]
    fn token_helpers_and_visual_properties() {
        let spec = TabsSpec::new(vec![TabDefinition::new("a", "A")]);
        assert_eq!(spec.list_gap_token(), semantic::SPACE_INLINE_SM);
        assert_eq!(spec.indicator_token(), semantic::COLOR_ACCENT_BASE);
        assert_eq!(spec.list_border_token(), semantic::COLOR_BORDER_SUBTLE);
        assert_eq!(spec.focus_ring_color_token(), semantic::COLOR_ACCENT_FOCUS_RING);
        assert_eq!(spec.disabled_opacity_token(), semantic::STATE_OPACITY_DISABLED);
        assert_eq!(spec.pill_border_opacity(), 0.68);
        assert_eq!(spec.pill_active_bg_opacity(), 0.18);
        assert_eq!(spec.block_list_bg_opacity(), 0.9);
        assert_eq!(spec.block_selected_accent_mix(), 0.14);
        assert_eq!(spec.block_selected_hover_accent_mix(), 0.18);
        assert_eq!(spec.block_separator_opacity(), 0.72);
        assert_eq!(spec.block_hover_bg_opacity(), 0.5);
    }

    #[test]
    fn tab_definition_builders() {
        let tab = TabDefinition::new("item", "Item")
            .with_disabled(true)
            .with_closable(true)
            .with_icon("folder")
            .with_count(42);

        assert_eq!(tab.value, "item");
        assert_eq!(tab.label, "Item");
        assert!(tab.is_disabled);
        assert!(tab.is_closable);
        assert_eq!(tab.icon.as_deref(), Some("folder"));
        assert_eq!(tab.count, Some(42));
    }
}
