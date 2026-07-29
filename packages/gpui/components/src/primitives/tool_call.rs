//! ToolCall — one row of agent work, backed by `ToolCallSpec`.
//!
//! Contract: `docs/contracts/components/tool-call.md`. Every dimension resolves
//! from the spec's size/density ladder; colours resolve from tokens.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_specs::ToolCallSpec;

use crate::presentation::rem_to_px;
use crate::primitives::icon::Icon;
use crate::theme_ext::resolve_color;

pub struct ToolCall {
    spec: ToolCallSpec,
    theme: GpuiThemeProvider,
}

impl ToolCall {
    pub fn from_spec(spec: ToolCallSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for ToolCall {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let label_color = resolve_color(theme, spec.label_token());
        let detail_color = resolve_color(theme, spec.detail_token());
        let icon_color = resolve_color(theme, spec.icon_token());
        let success = resolve_color(theme, spec.success_token());
        let danger = resolve_color(theme, spec.danger_token());

        let font_size = px(rem_to_px(spec.font_size_rem()));
        let icon_size = px(rem_to_px(spec.icon_size_rem()));
        let row_height = px(rem_to_px(spec.row_height_rem()));
        let pad_y = px(rem_to_px(spec.padding_block_rem()));
        let pad_x = px(rem_to_px(spec.padding_inline_rem()));
        let gap = px(rem_to_px(spec.gap_rem()));

        // Only the label takes the danger colour, never the detail. The detail
        // is already the dimmest thing in the row, and colouring it red as well
        // makes a failed row read as a block of alarm rather than a line you
        // can scan.
        let label_color = match spec.status {
            ToolCallStatus::Error => danger,
            _ => label_color,
        };
        let status_color = match spec.status {
            ToolCallStatus::Error => danger,
            ToolCallStatus::Success => success,
            ToolCallStatus::Running => icon_color,
        };

        let mut row = div()
            .flex()
            .w_full()
            .items_center()
            .gap(gap)
            .min_h(row_height)
            .px(pad_x)
            .py(pad_y)
            .text_size(font_size)
            .child(
                Icon::new(spec.resolved_icon(), theme)
                    .with_px_size(rem_to_px(spec.icon_size_rem()))
                    .with_color(icon_color)
                    .into_any_element(),
            )
            .child(div().flex_shrink_0().text_color(label_color).child(spec.label.clone()));

        if let Some(detail) = &spec.detail {
            // `min_w_0` is load-bearing: without it the detail refuses to shrink
            // below its content width and a long command pushes the status
            // indicator out of the row entirely.
            row = row.child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .text_color(detail_color)
                    .child(detail.clone()),
            );
        } else {
            row = row.child(div().flex_1());
        }

        if spec.has_output() {
            row = row.child(
                Icon::new("chevron-down", theme)
                    .with_px_size(rem_to_px(spec.icon_size_rem()))
                    .with_color(detail_color)
                    .into_any_element(),
            );
        }

        row = row.child(
            div().flex_shrink_0().child(
                Icon::new(spec.status_icon(), theme)
                    .with_px_size(rem_to_px(spec.icon_size_rem()))
                    .with_color(status_color)
                    .into_any_element(),
            ),
        );

        let mut root = div().flex().flex_col().w_full().child(row);

        // Only built when open: a transcript of a thousand rows must not build
        // a thousand output blocks for output nobody opened.
        if spec.has_output() && spec.is_expanded {
            if let Some(output) = &spec.output {
                root = root.child(
                    div()
                        .pl(pad_x + icon_size + gap)
                        .text_size(font_size)
                        .text_color(detail_color)
                        .child(output.clone()),
                );
            }
        }

        root.into_any_element()
    }
}
