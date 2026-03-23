use gpui::*;
use pug_adapter::ThemeProvider;
use pug_composites::{
    PickerShellSpec, PickerVariant, BrowseState, SelectionMode,
    RelationPickerSpec, PickerItemSpec,
};
use pug_gpui_components::{PickerShell, RelationPicker, TextInput};
use pug_primitives::TextInputSpec;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let text_primary = theme.resolve_color("semantic.color.text.primary");

    let items = vec![
        PickerItemSpec::new("btn", "Button"),
        PickerItemSpec::new("chk", "Checkbox"),
        PickerItemSpec::new("sel", "Select"),
        PickerItemSpec::new("dlg", "Dialog"),
        PickerItemSpec::new("tbl", "Table"),
    ];

    div().flex().flex_col().gap(px(24.0))
        // --- Inline variant (ready) ---
        .child(section_label("PICKER SHELL: INLINE (READY)", text_secondary))
        .child(
            PickerShell::from_spec(
                PickerShellSpec::new("Select a component")
                    .with_description("Browse and select from available components.")
                    .with_variant(PickerVariant::Inline)
                    .with_state(BrowseState::Ready)
                    .with_result_count(5),
                theme,
            )
            .with_search(
                TextInput::from_spec(
                    TextInputSpec::new().with_placeholder("Search components..."),
                    theme,
                ).with_id("picker-search-1")
            )
            .with_results(
                div().flex().flex_col()
                    .child(result_row("Button", "Primitive", text_primary, text_secondary))
                    .child(result_row("Checkbox", "Primitive", text_primary, text_secondary))
                    .child(result_row("Select", "Primitive", text_primary, text_secondary))
                    .child(result_row("Dialog", "Overlay", text_primary, text_secondary))
                    .child(result_row("Table", "Composite", text_primary, text_secondary))
            )
        )

        // --- Loading state ---
        .child(section_label("PICKER SHELL: LOADING", text_secondary))
        .child(
            PickerShell::from_spec(
                PickerShellSpec::new("Select an item")
                    .with_variant(PickerVariant::Inline)
                    .with_state(BrowseState::Loading),
                theme,
            )
            .with_results(div())
        )

        // --- No results ---
        .child(section_label("PICKER SHELL: NO RESULTS", text_secondary))
        .child(
            PickerShell::from_spec(
                PickerShellSpec::new("Select an item")
                    .with_variant(PickerVariant::Inline)
                    .with_state(BrowseState::NoResults)
                    .with_query("xyzzy"),
                theme,
            )
            .with_results(div())
        )

        // --- Multi-select with selection count ---
        .child(section_label("PICKER SHELL: MULTI-SELECT", text_secondary))
        .child(
            PickerShell::from_spec(
                PickerShellSpec::new("Select components")
                    .with_variant(PickerVariant::Inline)
                    .with_selection_mode(SelectionMode::Multiple)
                    .with_state(BrowseState::Ready)
                    .with_result_count(5)
                    .with_selected_count(2),
                theme,
            )
            .with_results(
                div().flex().flex_col()
                    .child(result_row("Button", "Selected", text_primary, text_secondary))
                    .child(result_row("Dialog", "Selected", text_primary, text_secondary))
                    .child(result_row("Table", "", text_primary, text_secondary))
            )
        )

        // --- RelationPicker ---
        .child(section_label("RELATION PICKER", text_secondary))
        .child(
            RelationPicker::from_spec(
                RelationPickerSpec::new(items)
                    .with_selected_ids(vec!["btn".to_string(), "dlg".to_string()])
                    .with_selection_mode(SelectionMode::Multiple)
                    .with_state(BrowseState::Ready),
                theme,
            )
        )
}

fn result_row(
    label: &str,
    meta: &str,
    primary: pug_tokens::typed::ColorValue,
    secondary: pug_tokens::typed::ColorValue,
) -> Div {
    let mut row = div()
        .flex().items_center().justify_between()
        .px(px(12.0)).py(px(8.0))
        .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.06)))
        .cursor(CursorStyle::PointingHand)
        .child(
            div().text_sm().text_color(color_to_hsla(primary))
                .child(label.to_string())
        );
    if !meta.is_empty() {
        row = row.child(
            div().text_xs().text_color(color_to_hsla(secondary))
                .child(meta.to_string())
        );
    }
    row
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
