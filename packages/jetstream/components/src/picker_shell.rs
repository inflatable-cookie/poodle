//! PickerShell — Jetstream picker shell backed by PickerShellSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{BrowseState, PickerShellSpec};

use crate::spinner::js_spinner;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_picker_shell(
    spec: &PickerShellSpec,
    theme: &JetstreamThemeProvider,
    toolbar: Option<JsEl>,
    selection: Option<JsEl>,
    body: Option<JsEl>,
    state_content: Option<JsEl>,
    footer: Option<JsEl>,
) -> JsEl {
    let panel_x = resolve_px(theme, "space.panel.x");
    let panel_y = resolve_px(theme, "space.panel.y");
    let gap_sm = resolve_px(theme, "space.inline.sm");
    let gap_md = resolve_px(theme, "space.inline.md");
    let stack_sm = resolve_px(theme, "space.stack.sm");
    let stack_md = resolve_px(theme, "space.stack.md");
    let label_size = resolve_px(theme, "typography.label.size");
    let body_size = resolve_px(theme, "typography.body.size");
    let fill = resolve_color(theme, "color.background.panel");
    let surface = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");

    let mut shell = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border)
        .rounded(radius)
        .flex_col()
        .gap(stack_md)
        .pl(panel_x)
        .pr(panel_x)
        .pt(panel_y)
        .pb(panel_y);

    let mut title_block = ui_element::div()
        .flex_col()
        .gap(stack_sm)
        .child(
            ui_element::label(&spec.title)
                .text_color(text_primary)
                .text_size(20.0)
                .text_weight(600),
        );

    if let Some(description) = spec.description.as_ref() {
        title_block = title_block.child(
            ui_element::label(description)
                .text_color(text_secondary)
                .text_size(label_size),
        );
    }

    let mut meta = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap_sm)
        .child(
            ui_element::label(&spec.selected_count_text())
                .text_color(text_secondary)
                .text_size(label_size),
        );

    if let Some(result_text) = spec.result_count_text() {
        meta = meta.child(
            ui_element::label(&result_text)
                .text_color(text_secondary)
                .text_size(label_size),
        );
    }

    shell = shell.child(
        ui_element::div()
            .flex_row()
            .justify_between()
            .items_start()
            .gap(gap_md)
            .child(title_block)
            .child(meta),
    );

    if let Some(toolbar) = toolbar {
        shell = shell.child(toolbar);
    }

    if let Some(selection) = selection {
        shell = shell.child(selection);
    }

    if let Some(status_text) = spec.status_text.as_ref() {
        shell = shell.child(
            ui_element::label(status_text)
                .text_color(text_secondary)
                .text_size(label_size)
                .opacity(0.0),
        );
    }

    if spec.state == BrowseState::Ready {
        if let Some(body) = body {
            shell = shell.child(body);
        }
    } else if let Some(state_content) = state_content {
        shell = shell.child(state_content);
    } else {
        let mut state = ui_element::div()
            .flex_col()
            .gap(stack_sm)
            .pl(panel_x)
            .pr(panel_x)
            .pt(panel_y * 1.5)
            .pb(panel_y * 1.5)
            .border(1.0)
            .border_color(border)
            .rounded(radius)
            .bg(surface);

        if spec.state == BrowseState::Loading {
            state = state.child(js_spinner(
                &poodle_specs::SpinnerSpec::new()
                    .with_variant(poodle_specs::SpinnerVariant::Grid)
                    .with_size(poodle_specs::SpinnerSize::Md)
                    .with_tone(poodle_specs::SpinnerTone::Accent),
                theme,
            ));
        }

        state = state.child(
            ui_element::label(spec.effective_state_title())
                .text_color(text_primary)
                .text_size(body_size)
                .text_weight(600),
        );

        if let Some(message) = spec.effective_state_message() {
            state = state.child(
                ui_element::label(message)
                    .text_color(text_secondary)
                    .text_size(label_size),
            );
        }

        shell = shell.child(state);
    }

    if let Some(footer) = footer {
        shell = shell.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .gap(resolve_px(theme, spec.footer_gap_token()))
                .child(footer),
        );
    }

    shell
}
