use crate::menu::MenuSpec;
use crate::types::{ControlDensity, ControlSize, MenuEntry, SemanticControlSizeRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextMenuSpec {
    pub menu: MenuSpec,
    pub anchor_point: Option<(i32, i32)>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for ContextMenuSpec {
    fn default() -> Self {
        Self {
            menu: MenuSpec::default(),
            anchor_point: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Chrome,
            density: ControlDensity::Default,
        }
    }
}

impl ContextMenuSpec {
    pub fn new(items: Vec<MenuEntry>) -> Self {
        Self {
            menu: MenuSpec::new(items),
            ..Self::default()
        }
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.menu = self.menu.with_open(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.menu = self.menu.with_default_open(default_open);
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.menu = self.menu.with_aria_label(aria_label);
        self
    }

    pub fn with_anchor_point(mut self, x: i32, y: i32) -> Self {
        self.anchor_point = Some((x, y));
        self
    }

    pub fn current_open(&self) -> bool {
        self.menu.current_open()
    }

    pub fn actionable_item_count(&self) -> usize {
        self.menu.actionable_item_count()
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
}
