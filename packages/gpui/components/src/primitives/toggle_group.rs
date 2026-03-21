//! ToggleGroup — a group of toggle buttons with shared selection state.

use gpui::*;
use pug_gpui::GpuiThemeProvider;

use super::toggle::Toggle;

/// A group of toggle buttons, rendering each with its pressed state.
pub struct ToggleGroup {
    theme: GpuiThemeProvider,
    items: Vec<ToggleGroupItem>,
    gap: f32,
}

/// An item in a toggle group.
pub struct ToggleGroupItem {
    pub label: String,
    pub is_pressed: bool,
}

impl ToggleGroupItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            is_pressed: false,
        }
    }

    pub fn with_pressed(mut self, is_pressed: bool) -> Self {
        self.is_pressed = is_pressed;
        self
    }
}

impl ToggleGroup {
    pub fn new(items: Vec<ToggleGroupItem>, theme: &GpuiThemeProvider) -> Self {
        Self {
            theme: theme.clone(),
            items,
            gap: 2.0,
        }
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl IntoElement for ToggleGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let mut el = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(self.gap));

        for item in self.items {
            let toggle = Toggle::new(item.label, theme).with_pressed(item.is_pressed);
            el = el.child(toggle);
        }

        el.into_any_element()
    }
}
