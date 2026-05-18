//! PickerShell specimen — generic picker container.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::picker_shell::js_picker_shell;
use poodle_jetstream_components::presentation::{rem_to_px, size_font_rem};
use poodle_jetstream_components::selection_summary::js_selection_summary;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{
    BrowseState, ButtonSpec, ButtonVariant, ControlSize, PickerShellSpec, RemediationAction,
    SelectionSummaryItem, SelectionSummarySpec,
};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_primary = resolve_color(theme, "color.text.primary");

    div().flex_col().gap(24.0)
        .child(group("With content", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Select an item")
                    .with_result_count(3)
                    .with_selected_count(2)
                    .with_status_text("3 results, 2 selected"),
                theme,
                Some(
                    div()
                        .border(1.0)
                        .border_color(resolve_color(theme, "color.border.subtle"))
                        .rounded(rem_to_px(0.375))
                        .pl(rem_to_px(0.75))
                        .pr(rem_to_px(0.75))
                        .pt(rem_to_px(0.5))
                        .pb(rem_to_px(0.5))
                        .child(label("Search items…").text_color(secondary).text_size(13.0)),
                ),
                Some(
                    js_selection_summary(
                        &SelectionSummarySpec::new(vec![
                            SelectionSummaryItem::new("a", "Option A"),
                            SelectionSummaryItem::new("b", "Option B"),
                        ])
                        .with_clear_action(RemediationAction::new("clear", "Clear")),
                        theme,
                    ),
                ),
                Some(
                    div().flex_col().gap(4.0)
                        .child(label("Option A").text_color(text_primary).text_size(body_font))
                        .child(label("Option B").text_color(text_primary).text_size(body_font))
                        .child(label("Option C").text_color(text_primary).text_size(body_font))
                ),
                None,
                Some(
                    div()
                        .flex_row()
                        .justify_end()
                        .gap(8.0)
                        .child(js_button(
                            &ButtonSpec::new().with_variant(ButtonVariant::Ghost).with_label("Cancel"),
                            theme,
                        ))
                        .child(js_button(
                            &ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Confirm"),
                            theme,
                        )),
                ),
            )
        ))
        .child(group("Empty", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Choose item").with_state(BrowseState::Empty),
                theme,
                None,
                None,
                None,
                None,
                None,
            )
        ))
        .child(group("Loading", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Search media")
                    .with_state(BrowseState::Loading)
                    .with_state_message("Fetching the latest results."),
                theme,
                None,
                None,
                None,
                None,
                None,
            )
        ))
        .child(group("Error", secondary,
            js_picker_shell(
                &PickerShellSpec::new("Search media")
                    .with_state(BrowseState::Error)
                    .with_state_message("The request failed. Try again in a moment."),
                theme,
                None,
                None,
                None,
                None,
                None,
            )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
