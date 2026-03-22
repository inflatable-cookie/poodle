//! BlockEditor — block-based content editor backed by BlockEditorSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::BlockEditorSpec;
use pug_primitives::{IconSize, IconSpec};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct BlockEditor {
    spec: BlockEditorSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for BlockEditor {
    type Target = BlockEditorSpec;
    fn deref(&self) -> &BlockEditorSpec { &self.spec }
}

impl BlockEditor {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: BlockEditorSpec::new(), theme: theme.clone(), children: Vec::new() }
    }
    pub fn from_spec(spec: BlockEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), children: Vec::new() }
    }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}

impl IntoElement for BlockEditor {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let border = resolve_color(&self.theme, self.spec.border_token());
        let gap = resolve_px(&self.theme, self.spec.block_gap_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let muted = resolve_color(&self.theme, "semantic.color.text.secondary");
        let hover_bg = resolve_color(&self.theme, "semantic.color.bg.hover");

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .flex().flex_col().gap(gap)
            .px(px(12.0)).py(px(8.0))
            .min_h(px(120.0));

        // Wrap each child block with a drag handle and remove button
        for child in self.children {
            let block_row = div()
                .flex().flex_row().items_center().gap(px(6.0))
                .group("block-row")
                // Drag handle
                .child(
                    div()
                        .cursor(CursorStyle::PointingHand)
                        .flex().items_center().justify_center()
                        .w(px(20.0)).h(px(20.0)).rounded(px(4.0))
                        .hover(|s| s.bg(hover_bg))
                        .child(
                            Icon::from_spec(
                                IconSpec::new("grip-vertical").with_size(IconSize::Sm),
                                &self.theme,
                            ).with_color(muted)
                        )
                )
                // Block content
                .child(div().flex_grow().child(child))
                // Remove button
                .child(
                    div()
                        .cursor(CursorStyle::PointingHand)
                        .flex().items_center().justify_center()
                        .w(px(20.0)).h(px(20.0)).rounded(px(4.0))
                        .hover(|s| s.bg(hover_bg))
                        .child(
                            Icon::from_spec(
                                IconSpec::new("x").with_size(IconSize::Sm),
                                &self.theme,
                            ).with_color(muted)
                        )
                );
            el = el.child(block_row);
        }

        // "Add block" button at the bottom
        el = el.child(
            div()
                .flex().flex_row().items_center().justify_center().gap(px(4.0))
                .py(px(6.0))
                .cursor(CursorStyle::PointingHand)
                .rounded(px(4.0))
                .hover(|s| s.bg(hover_bg))
                .child(
                    Icon::from_spec(
                        IconSpec::new("plus").with_size(IconSize::Sm),
                        &self.theme,
                    ).with_color(muted)
                )
                .child(
                    div().text_xs().text_color(muted).child("Add block")
                )
        );

        if self.spec.is_disabled {
            el = el.opacity(resolve_opacity(&self.theme, "semantic.state.opacity.disabled"));
        }
        el.into_any_element()
    }
}
