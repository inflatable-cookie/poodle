use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone)]
pub struct BreadcrumbItem {
    pub value: String,
    pub label: String,
    pub href: Option<String>,
    pub is_current: bool,
    /// Named icon rendered before the visible label, inside the same navigation
    /// target. Named icons are the portable cross-runtime contract.
    pub icon: Option<String>,
    /// Hide the visible label while keeping [`Self::label`] as the item's
    /// accessible name. Only ever valid together with [`Self::icon`]; construct
    /// it atomically with [`BreadcrumbItem::with_icon_only`].
    pub icon_only: bool,
}

impl BreadcrumbItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            href: None,
            is_current: false,
            icon: None,
            icon_only: false,
        }
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_is_current(mut self, is_current: bool) -> Self {
        self.is_current = is_current;
        self
    }

    /// An icon before the visible label. The label still renders.
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// A visually icon-only crumb that keeps its label as the accessible name.
    ///
    /// The icon and the flag are set together, so normal construction cannot
    /// produce the `icon_only` + no-icon state. A renderer handed that state
    /// directly must fall back to the label rather than paint a blank crumb.
    pub fn with_icon_only(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self.icon_only = true;
        self
    }

    /// Whether the visible label renders. False only for a well-formed
    /// icon-only crumb; a malformed `icon_only` item with no icon still shows
    /// its label.
    pub fn shows_label(&self) -> bool {
        !self.icon_only || self.icon.is_none()
    }
}

/// Sentinel value carried by the synthesized ellipsis crumb that replaces the
/// collapsed middle items when `max_visible_items` truncates the trail. Matches
/// the Svelte `value: "__ellipsis__"` marker — the crumb is non-interactive.
pub const ELLIPSIS_VALUE: &str = "__ellipsis__";

#[derive(Clone)]
pub struct BreadcrumbsSpec {
    pub items: Vec<BreadcrumbItem>,
    pub aria_label: String,
    pub max_visible_items: Option<usize>,
    /// When true (default), the last visible item is treated as the current
    /// page (`aria-current="page"`) even without `is_current`. Set false to opt
    /// out. Mirrors the Svelte `forceLastItemCurrent` prop.
    pub force_last_item_current: bool,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    /// `None` inherits from the presentation context; an explicit value always wins.
    pub density: Option<ControlDensity>,
}

impl BreadcrumbsSpec {
    pub fn new(items: Vec<BreadcrumbItem>) -> Self {
        Self {
            items,
            aria_label: "Breadcrumb".to_string(),
            max_visible_items: None,
            force_last_item_current: true,
            size: None,
            size_role: SemanticControlSizeRole::Chrome,
            density: None,
        }
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = aria_label.into();
        self
    }

    pub fn with_max_visible_items(mut self, max: usize) -> Self {
        self.max_visible_items = Some(max);
        self
    }

    pub fn with_force_last_item_current(mut self, force: bool) -> Self {
        self.force_last_item_current = force;
        self
    }

    /// Resolve the items to actually render, applying the ellipsis-collapse
    /// when `max_visible_items` is set and the trail exceeds it. Returns
    /// `first + ellipsis + last (max - 1)` to match the Svelte truncation rule.
    pub fn visible_items(&self) -> Vec<BreadcrumbItem> {
        match self.max_visible_items {
            Some(max) if max >= 2 && self.items.len() > max => {
                let mut visible = Vec::with_capacity(max + 1);
                visible.push(self.items[0].clone());
                visible.push(BreadcrumbItem {
                    value: ELLIPSIS_VALUE.to_string(),
                    label: "\u{2026}".to_string(),
                    href: None,
                    is_current: false,
                    // The synthetic ellipsis never inherits icon presentation
                    // from the authored items it replaces.
                    icon: None,
                    icon_only: false,
                });
                for item in self.items.iter().skip(self.items.len() - (max - 1)) {
                    visible.push(item.clone());
                }
                visible
            }
            _ => self.items.clone(),
        }
    }

    /// Whether the visible item at `index` (within `visible_items()`) should
    /// render as the current page. True when the item is explicitly current, or
    /// it is the last visible item and `force_last_item_current` is set. The
    /// ellipsis sentinel is never current.
    pub fn is_current_at(&self, item: &BreadcrumbItem, index: usize, visible_len: usize) -> bool {
        if item.value == ELLIPSIS_VALUE {
            return false;
        }
        item.is_current || (self.force_last_item_current && index + 1 == visible_len)
    }

    pub fn text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn current_text_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    pub fn separator_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    pub fn hover_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_SM
    }

    /// Spacing between an item's icon and its label, inside the crumb. Tighter
    /// than the crumb/separator gap so glyph and text read as one target.
    pub fn icon_gap_token(&self) -> &'static str {
        semantic::SPACE_INLINE_XS
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_icon_keeps_the_label_visible() {
        let item = BreadcrumbItem::new("home", "Home").with_icon("home");
        assert_eq!(item.icon.as_deref(), Some("home"));
        assert!(!item.icon_only);
        assert!(item.shows_label());
    }

    #[test]
    fn with_icon_only_sets_icon_and_flag_together() {
        let item = BreadcrumbItem::new("home", "Home").with_icon_only("home");
        assert_eq!(item.icon.as_deref(), Some("home"));
        assert!(item.icon_only);
        assert!(!item.shows_label());
    }

    #[test]
    fn a_malformed_icon_only_item_still_shows_its_label() {
        let mut item = BreadcrumbItem::new("home", "Home");
        item.icon_only = true;
        assert!(item.shows_label());
    }

    #[test]
    fn the_synthetic_ellipsis_never_inherits_icon_presentation() {
        let spec = BreadcrumbsSpec::new(vec![
            BreadcrumbItem::new("home", "Home").with_icon_only("home"),
            BreadcrumbItem::new("workspace", "Workspace").with_icon("folder"),
            BreadcrumbItem::new("projects", "Projects").with_icon("folder"),
            BreadcrumbItem::new("poodle", "Poodle").with_icon("package"),
        ])
        .with_max_visible_items(3);

        let visible = spec.visible_items();
        let ellipsis = visible
            .iter()
            .find(|item| item.value == ELLIPSIS_VALUE)
            .expect("truncation synthesizes an ellipsis crumb");

        assert!(ellipsis.icon.is_none());
        assert!(!ellipsis.icon_only);
        // Retained authored items keep theirs.
        assert_eq!(visible[0].icon.as_deref(), Some("home"));
        assert!(visible[0].icon_only);
        assert_eq!(visible[3].icon.as_deref(), Some("package"));
    }
}
