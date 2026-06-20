//! ToastStack — Jetstream toast notification stack backed by ToastStackSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::ToastStackSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub fn js_toast_stack(spec: &ToastStackSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_y = rem_to_px(panel_space_y_rem(spec.density) - 0.25);
    let stack_gap = resolve_px(theme, spec.gap_token());
    let item_gap = rem_to_px(0.5);

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let title_color = resolve_color(theme, spec.title_color_token());
    let message_color = resolve_color(theme, spec.message_color_token());
    let dismiss_color = resolve_color(theme, spec.dismiss_color_token());

    let mut el = ui_element::div().flex_col().gap(stack_gap);

    for toast in &spec.toasts {
        let tone_color = resolve_color(theme, spec.tone_color(&toast.tone));

        // Leading tone accent bar.
        let accent_bar = ui_element::div()
            .w(rem_to_px(0.1875))
            .self_stretch()
            .rounded(999.0)
            .bg(tone_color);

        // Title + optional message column.
        let mut content = ui_element::div()
            .flex_col()
            .gap(rem_to_px(0.125))
            .grow()
            .child(
                ui_element::label(toast.title.as_str())
                    .text_color(title_color)
                    .text_size(font_size)
                    .text_weight(600),
            );
        if let Some(message) = &toast.message {
            content = content.child(
                ui_element::label(message.as_str())
                    .text_color(message_color)
                    .text_size(rem_to_px(0.8125)),
            );
        }

        let mut toast_el = ui_element::div()
            .bg(fill)
            .border(1.0)
            .border_color(border)
            .rounded(radius)
            .pl(pad_x)
            .pr(pad_x)
            .pt(pad_y)
            .pb(pad_y)
            .flex_row()
            .items_center()
            .gap(item_gap)
            .child(accent_bar)
            .child(content);

        // Optional action label (tone-colored), then the dismiss affordance.
        if let Some(action) = &toast.action_label {
            toast_el = toast_el.child(
                ui_element::label(action.as_str())
                    .text_color(tone_color)
                    .text_size(font_size)
                    .text_weight(600),
            );
        }
        toast_el = toast_el.child(
            ui_element::label("\u{00d7}") // × — dismiss; click handled in preview loop
                .text_color(dismiss_color)
                .text_size(font_size),
        );

        el = el.child(toast_el);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::Toast;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn renders_title_message_and_action() {
        let spec = ToastStackSpec::new().with_toasts(vec![Toast::new("t1", "Saved")
            .with_message("Your changes were saved")
            .with_action_label("Undo")]);
        let tree = probe(&js_toast_stack(&spec, &theme()), 360.0, 120.0);
        assert!(tree.has_text("Saved"), "title missing: {:?}", tree.texts());
        assert!(tree.has_text("Your changes were saved"), "message missing");
        assert!(tree.has_text("Undo"), "action missing");
        assert!(tree.has_text("\u{00d7}"), "dismiss missing");
    }
}
