//! ReorderableList — drag-to-reorder list backed by ReorderableListSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};
use poodle_composites::ReorderableListSpec;
use crate::presentation::{resolve_semantic_size, control_space_x_rem, rem_to_px};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px};

pub struct ReorderableList {
    spec: ReorderableListSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for ReorderableList {
    type Target = ReorderableListSpec;
    fn deref(&self) -> &ReorderableListSpec { &self.spec }
}

impl ReorderableList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ReorderableListSpec::new(), theme: theme.clone(), children: Vec::new() }
    }
    pub fn from_spec(spec: ReorderableListSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), children: Vec::new() }
    }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }
}

impl IntoElement for ReorderableList {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let _effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let item_gap = rem_to_px(control_space_x_rem(spec.density));

        let _fill = resolve_color(theme, spec.fill_token());
        let gap = resolve_px(theme, spec.item_gap_token());
        let handle_color = resolve_color(theme, spec.handle_color_token());
        let drag_bg = resolve_color(theme, "color.surface.raised");

        let mut el = div().flex().flex_col().gap(gap);
        for (idx, child) in self.children.into_iter().enumerate() {
            let grip_icon = Icon::from_spec(
                IconSpec::new("grip-vertical").with_size(IconSize::Sm),
                theme,
            ).with_color(handle_color);

            let handle = div()
                .flex()
                .items_center()
                .justify_center()
                .cursor(CursorStyle::ClosedHand)
                .px(px(2.0))
                .child(grip_icon);

            let row_id = SharedString::from(format!("poodle-reorder-item-{}", idx));
            let focus_ring = resolve_color(theme, "color.accent.focusRing");
            let row = div()
                .id(row_id)
                .focusable()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(item_gap))
                .px(px(4.0))
                .py(px(2.0))
                .rounded(px(4.0))
                .hover(|s| s.bg(drag_bg))
                .focus(move |s| s.border_color(focus_ring))
                .child(handle)
                .child(child);
            el = el.child(row);
        }
        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "state.opacity.disabled");
            el = el.opacity(opacity);
        }
        el.into_any_element()
    }
}
