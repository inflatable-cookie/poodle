//! BlockEditor — block-based content editor backed by BlockEditorSpec.

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_composites::BlockEditorSpec;
use flint_primitives::{IconSize, IconSpec};
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

/// Build a small toolbar icon button (20x20, rounded, hover-bg).
fn toolbar_icon(theme: &GpuiThemeProvider, icon_name: &str, muted: Hsla, hover_bg: Hsla) -> Div {
    div()
        .cursor(CursorStyle::PointingHand)
        .flex().items_center().justify_center()
        .w(px(20.0)).h(px(20.0)).rounded(px(4.0))
        .hover(move |s| s.bg(hover_bg))
        .child(
            Icon::from_spec(
                IconSpec::new(icon_name).with_size(IconSize::Sm),
                theme,
            ).with_color(muted)
        )
}

impl IntoElement for BlockEditor {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let fill = resolve_color(theme, self.spec.fill_token());
        let border = resolve_color(theme, self.spec.border_token());
        let gap = resolve_px(theme, self.spec.block_gap_token());
        let radius = resolve_radius(theme, "semantic.radius.surface");
        let muted = resolve_color(theme, "semantic.color.text.secondary");
        let hover_bg = resolve_color(theme, "semantic.color.background.elevated");
        let separator = resolve_color(theme, "semantic.color.border.subtle");

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .flex().flex_col().gap(gap)
            .px(px(12.0)).py(px(8.0))
            .min_h(px(120.0));

        let block_count = self.children.len();

        // Wrap each child block with a hover-revealed toolbar
        for (i, child) in self.children.into_iter().enumerate() {
            let block_row = div()
                .flex().flex_row().items_start().gap(px(4.0))
                .group("block-row")
                .py(px(4.0))
                // Left toolbar: drag handle, block type indicator, move up, move down
                .child(
                    div()
                        .flex().flex_row().items_center().gap(px(2.0))
                        .opacity(0.0)
                        .group_hover("block-row", |s| s.opacity(1.0))
                        // Drag handle
                        .child(toolbar_icon(theme,"grip-vertical", muted, hover_bg))
                        // Block type indicator (generic block icon)
                        .child(toolbar_icon(theme,"square", muted, hover_bg))
                        // Separator
                        .child(
                            div().w(px(1.0)).h(px(14.0)).bg(separator).mx(px(2.0))
                        )
                        // Move up (disabled style if first block)
                        .child({
                            let btn = toolbar_icon(theme,"chevron-up", muted, hover_bg);
                            if i == 0 { btn.opacity(0.35) } else { btn }
                        })
                        // Move down (disabled style if last block)
                        .child({
                            let btn = toolbar_icon(theme,"chevron-down", muted, hover_bg);
                            if i == block_count - 1 { btn.opacity(0.35) } else { btn }
                        })
                )
                // Block content
                .child(div().flex_grow().child(child))
                // Right toolbar: add below, remove
                .child(
                    div()
                        .flex().flex_row().items_center().gap(px(2.0))
                        .opacity(0.0)
                        .group_hover("block-row", |s| s.opacity(1.0))
                        // Add block below
                        .child(toolbar_icon(theme,"plus", muted, hover_bg))
                        // Remove block
                        .child(toolbar_icon(theme,"trash-2", muted, hover_bg))
                );
            el = el.child(block_row);
        }

        // "Add block" button at the bottom
        el = el.child(
            div()
                .flex().flex_row().items_center().justify_center().gap(px(6.0))
                .py(px(8.0))
                .cursor(CursorStyle::PointingHand)
                .rounded(px(4.0))
                .border_1().border_color(separator)
                .hover(|s| s.bg(hover_bg))
                .child(
                    Icon::from_spec(
                        IconSpec::new("plus").with_size(IconSize::Sm),
                        theme,
                    ).with_color(muted)
                )
                .child(
                    div().text_size(px(12.0)).text_color(muted).child("Add block")
                )
        );

        if self.spec.is_disabled {
            el = el.opacity(resolve_opacity(theme, "semantic.state.opacity.disabled"));
        }
        el.into_any_element()
    }
}
