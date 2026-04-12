//! BlockEditor — block-based content editor backed by BlockEditorSpec.

use crate::presentation::{
    control_space_x_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::BlockEditorSpec;
use poodle_specs::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};

pub struct BlockEditor {
    spec: BlockEditorSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for BlockEditor {
    type Target = BlockEditorSpec;
    fn deref(&self) -> &BlockEditorSpec {
        &self.spec
    }
}

impl BlockEditor {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: BlockEditorSpec::new(),
            theme: theme.clone(),
            children: Vec::new(),
        }
    }
    pub fn from_spec(spec: BlockEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
}

/// Build a small toolbar icon button (20x20, rounded, hover-bg).
fn toolbar_icon(
    theme: &GpuiThemeProvider,
    icon_name: &str,
    muted: Hsla,
    hover_bg: Hsla,
    radius: Pixels,
) -> Div {
    div()
        .cursor(CursorStyle::PointingHand)
        .flex()
        .items_center()
        .justify_center()
        .w(px(20.0))
        .h(px(20.0))
        .rounded(radius)
        .hover(move |s| s.bg(hover_bg))
        .child(
            Icon::from_spec(IconSpec::new(icon_name).with_size(IconSize::Sm), theme)
                .with_color(muted),
        )
}

impl IntoElement for BlockEditor {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let _effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let pad_x = rem_to_px(panel_space_x_rem(self.spec.density));
        let pad_y = rem_to_px(panel_space_y_rem(self.spec.density));
        let _item_gap = rem_to_px(control_space_x_rem(self.spec.density));

        let fill = resolve_color(theme, self.spec.fill_token());
        let border = resolve_color(theme, self.spec.border_token());
        let gap = resolve_px(theme, self.spec.block_gap_token());
        let radius = resolve_radius(theme, "radius.surface");
        let radius_control = resolve_radius(theme, "radius.control");
        let caption_size = resolve_px(theme, "typography.caption.size");
        let label_size = resolve_px(theme, "typography.label.size");
        let body_size = resolve_px(theme, "typography.body.size");
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");
        let muted = resolve_color(theme, "color.text.secondary");
        let text_primary = resolve_color(theme, "color.text.primary");
        let hover_bg = resolve_color(theme, "color.background.elevated");
        let separator = resolve_color(theme, "color.border.subtle");

        // When spec.blocks is populated, render them as text children using the
        // consumer-provided block type definitions to show the type label.
        // Otherwise fall back to self.children (legacy API).
        let use_spec_blocks = !self.spec.blocks.is_empty();
        let children: Vec<AnyElement> = if use_spec_blocks {
            self.spec
                .blocks
                .iter()
                .map(|block| {
                    // Find the block type definition for display
                    let type_label = self
                        .spec
                        .block_types
                        .iter()
                        .find(|t| t.block_type == block.block_type)
                        .map(|t| t.label.clone())
                        .unwrap_or_else(|| block.block_type.clone());
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_size(caption_size)
                                .text_color(muted)
                                .child(type_label),
                        )
                        .child(
                            div()
                                .text_size(body_size)
                                .text_color(text_primary)
                                .child(block.content.clone()),
                        )
                        .into_any_element()
                })
                .collect()
        } else {
            self.children
        };

        let mut el = div()
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .flex()
            .flex_col()
            .gap(gap)
            .px(px(pad_x))
            .py(px(pad_y))
            .min_h(px(120.0));

        let block_count = children.len();

        // Wrap each child block with a hover-revealed toolbar
        for (i, child) in children.into_iter().enumerate() {
            let block_row = div()
                .flex()
                .flex_row()
                .items_start()
                .gap(gap_sm)
                .group("block-row")
                .py(gap_sm)
                // Left toolbar: drag handle, block type indicator, move up, move down
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(2.0))
                        .opacity(0.0)
                        .group_hover("block-row", |s| s.opacity(1.0))
                        // Drag handle
                        .child(toolbar_icon(
                            theme,
                            "grip-vertical",
                            muted,
                            hover_bg,
                            radius_control,
                        ))
                        // Block type indicator (generic block icon)
                        .child(toolbar_icon(
                            theme,
                            "square",
                            muted,
                            hover_bg,
                            radius_control,
                        ))
                        // Separator
                        .child(div().w(px(1.0)).h(px(14.0)).bg(separator).mx(px(2.0)))
                        // Move up (disabled style if first block)
                        .child({
                            let btn =
                                toolbar_icon(theme, "chevron-up", muted, hover_bg, radius_control);
                            if i == 0 {
                                btn.opacity(0.35)
                            } else {
                                btn
                            }
                        })
                        // Move down (disabled style if last block)
                        .child({
                            let btn = toolbar_icon(
                                theme,
                                "chevron-down",
                                muted,
                                hover_bg,
                                radius_control,
                            );
                            if i == block_count - 1 {
                                btn.opacity(0.35)
                            } else {
                                btn
                            }
                        }),
                )
                // Block content
                .child(div().flex_grow().child(child))
                // Right toolbar: add below, remove
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(2.0))
                        .opacity(0.0)
                        .group_hover("block-row", |s| s.opacity(1.0))
                        // Add block below
                        .child(toolbar_icon(theme, "plus", muted, hover_bg, radius_control))
                        // Remove block
                        .child(toolbar_icon(
                            theme,
                            "trash-2",
                            muted,
                            hover_bg,
                            radius_control,
                        )),
                );
            el = el.child(block_row);
        }

        // "Add block" button at the bottom
        el = el.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_center()
                .gap(gap_md)
                .py(gap_md)
                .cursor(CursorStyle::PointingHand)
                .rounded(radius_control)
                .border_1()
                .border_color(separator)
                .hover(|s| s.bg(hover_bg))
                .child(
                    Icon::from_spec(IconSpec::new("plus").with_size(IconSize::Sm), theme)
                        .with_color(muted),
                )
                .child(
                    div()
                        .text_size(label_size)
                        .text_color(muted)
                        .child("Add block"),
                ),
        );

        if self.spec.is_disabled {
            el = el.opacity(resolve_opacity(theme, "state.opacity.disabled"));
        }
        el.into_any_element()
    }
}
