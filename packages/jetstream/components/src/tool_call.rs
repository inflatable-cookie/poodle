//! ToolCall — one row of agent work, backed by `ToolCallSpec`.
//!
//! Contract: `docs/contracts/components/tool-call.md`. Every dimension resolves
//! from the spec's ladder; the only literal is the hairline the contract states
//! as an absolute.

use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::Color;
use poodle_headless::agent_transcript::ToolCallStatus;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ToolCallSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_radius};

pub fn js_tool_call(spec: &ToolCallSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let label_color: Color = resolve_color(theme, spec.label_token()).into();
    let detail_color: Color = resolve_color(theme, spec.detail_token()).into();
    let icon_color: Color = resolve_color(theme, spec.icon_token()).into();
    let success: Color = resolve_color(theme, spec.success_token()).into();
    let danger: Color = resolve_color(theme, spec.danger_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let icon_size = rem_to_px(spec.icon_size_rem());
    let row_height = rem_to_px(spec.row_height_rem());
    let pad_y = rem_to_px(spec.padding_block_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());
    let gap = rem_to_px(spec.gap_rem());

    // Only the label takes the danger colour, never the detail. The detail is
    // already the dimmest thing in the row, and colouring it red as well makes a
    // failed row read as a block of alarm rather than a line you can scan.
    let label_color = match spec.status {
        ToolCallStatus::Error => danger,
        _ => label_color,
    };
    let status_color = match spec.status {
        ToolCallStatus::Error => danger,
        ToolCallStatus::Success => success,
        ToolCallStatus::Running => icon_color,
    };

    let mut row = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .min_h(row_height)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .rounded(radius)
        .child(
            ui_element::icon(spec.resolved_icon())
                .w(icon_size)
                .h(icon_size)
                .text_color(icon_color),
        )
        .child(
            ui_element::label(spec.label.clone())
                .text_size(font_size)
                .text_color(label_color)
                .flex_shrink_0(),
        );

    if let Some(detail) = &spec.detail {
        // `min_w_0` is load-bearing: without it the detail refuses to shrink
        // below its content width and a long command pushes the status
        // indicator out of the row.
        row = row.child(
            ui_element::label(detail.clone())
                .text_size(font_size)
                .text_color(detail_color)
                .opacity(crate::theme_ext::resolve_opacity(theme, spec.detail_opacity_token()))
                .grow()
                .min_w_0(),
        );
    } else {
        row = row.child(ui_element::div().grow());
    }

    if spec.has_output() {
        row = row.child(
            ui_element::icon("chevron-down")
                .w(icon_size)
                .h(icon_size)
                .text_color(detail_color),
        );
    }

    row = row.child(
        ui_element::icon(spec.status_icon())
            .w(icon_size)
            .h(icon_size)
            .text_color(status_color)
            .flex_shrink_0(),
    );

    // Status reaches assistive technology through the name; colour and glyph do
    // not.
    let mut root = ui_element::div()
        .flex_col()
        .w_full()
        .aria_role(jetstream_ui::accesskit::Role::ListItem)
        .aria_label(spec.accessible_name())
        .child(row);

    if spec.has_output() && spec.is_expanded {
        if let Some(output) = &spec.output {
            root = root.child(
                ui_element::label(output.clone())
                    .text_size(font_size)
                    .text_color(detail_color)
                    .pl(pad_x + icon_size + gap),
            );
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn the_detail_reaches_the_render() {
        let spec = ToolCallSpec::new("a", "Ran command").with_detail("bun test");
        let tree = crate::render_probe::probe(&js_tool_call(&spec, &theme()), 720.0, 64.0);

        assert!(tree.has_text("Ran command"), "{:?}", tree.texts());
        assert!(tree.has_text("bun test"), "{:?}", tree.texts());
    }

    #[test]
    fn output_is_only_built_when_open() {
        let base = ToolCallSpec::new("a", "Ran command").with_output("272 pass");
        let closed = crate::render_probe::probe(&js_tool_call(&base, &theme()), 720.0, 64.0);
        let open = crate::render_probe::probe(
            &js_tool_call(&base.clone().with_expanded(true), &theme()),
            720.0,
            128.0,
        );

        assert!(!closed.has_text("272 pass"), "{:?}", closed.texts());
        assert!(open.has_text("272 pass"), "{:?}", open.texts());
    }
}
