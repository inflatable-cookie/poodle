//! BlockEditor — Jetstream block editor backed by BlockEditorSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::BlockEditorSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_radius, tint};

pub fn js_block_editor(spec: &BlockEditorSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // Size-driven control size from contract
    let control_size = rem_to_px(match effective_size {
        poodle_primitives::ControlSize::Xs => 1.25,
        poodle_primitives::ControlSize::Sm => 1.5,
        poodle_primitives::ControlSize::Md => 1.75,
        poodle_primitives::ControlSize::Lg => 2.0,
        poodle_primitives::ControlSize::Xl => 2.25,
    });

    // Density-driven spacing from contract
    let (shell_x, shell_y, stack_gap, toolbar_y, toolbar_x, content_x, content_y) = match spec.density {
        poodle_primitives::ControlDensity::Compact => (0.625, 0.625, 0.375, 0.1875, 0.25, 0.375, 0.25),
        poodle_primitives::ControlDensity::Default => (0.75, 0.75, 0.5, 0.25, 0.375, 0.5, 0.375),
        poodle_primitives::ControlDensity::Comfortable => (1.0, 1.0, 0.625, 0.3125, 0.5, 0.625, 0.5),
    };

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "semantic.radius.surface");
    let ctrl_radius = resolve_radius(theme, "semantic.radius.control");
    let text_primary = resolve_color(theme, "semantic.color.text.primary");
    let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
    let elevated = resolve_color(theme, "semantic.color.background.elevated");

    let block_bg = tint(elevated, 0.42);

    // Root container
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_col().gap(rem_to_px(stack_gap))
        .pl(rem_to_px(shell_x)).pr(rem_to_px(shell_x))
        .pt(rem_to_px(shell_y)).pb(rem_to_px(shell_y));

    if spec.is_disabled {
        el = el.opacity(0.48);
    }

    // Render placeholder blocks based on block_count
    let count = if spec.block_count == 0 { 1 } else { spec.block_count };
    for i in 0..count {
        let mut block = ui_element::div()
            .flex_col()
            .rounded(ctrl_radius)
            .bg(block_bg);

        // Toolbar row
        let mut toolbar = ui_element::div()
            .flex_row().items_center()
            .gap(rem_to_px(0.125))
            .pl(rem_to_px(toolbar_x)).pr(rem_to_px(toolbar_x))
            .pt(rem_to_px(toolbar_y)).pb(rem_to_px(toolbar_y));

        // Toolbar left: drag grip + type select
        let toolbar_left = ui_element::div()
            .flex_row().items_center().gap(rem_to_px(0.125))
            .child(
                ui_element::div()
                    .w(control_size).h(control_size)
                    .rounded(ctrl_radius)
                    .items_center().justify_center()
            )
            .child(
                ui_element::label("paragraph")
                    .text_color(text_secondary)
                    .text_size(rem_to_px(0.8125))
            );
        toolbar = toolbar.child(toolbar_left);

        toolbar = toolbar.child(ui_element::div().grow());

        // Toolbar right: move up, move down, add, remove
        let mut toolbar_right = ui_element::div()
            .flex_row().items_center().gap(rem_to_px(0.125));

        toolbar_right = toolbar_right.child(
            ui_element::button("\u{2191}")
                .w(control_size).h(control_size)
                .text_color(text_secondary).text_size(rem_to_px(0.75))
                .rounded(ctrl_radius)
        );
        toolbar_right = toolbar_right.child(
            ui_element::button("\u{2193}")
                .w(control_size).h(control_size)
                .text_color(text_secondary).text_size(rem_to_px(0.75))
                .rounded(ctrl_radius)
        );
        toolbar_right = toolbar_right.child(
            ui_element::button("+")
                .w(control_size).h(control_size)
                .text_color(text_secondary).text_size(rem_to_px(0.75))
                .rounded(ctrl_radius)
        );
        if count > 1 {
            toolbar_right = toolbar_right.child(
                ui_element::button("\u{00D7}")
                    .w(control_size).h(control_size)
                    .text_color(text_secondary).text_size(rem_to_px(0.75))
                    .rounded(ctrl_radius)
            );
        }
        toolbar = toolbar.child(toolbar_right);
        block = block.child(toolbar);

        // Content area
        let content = ui_element::div()
            .pl(rem_to_px(content_x)).pr(rem_to_px(content_x))
            .pt(rem_to_px(content_y)).pb(rem_to_px(content_y))
            .min_h(rem_to_px(1.5))
            .child(
                ui_element::label(&format!("Block {}", i + 1))
                    .text_color(text_primary)
                    .text_size(rem_to_px(0.875))
            );
        block = block.child(content);

        el = el.child(block);
    }

    el
}
