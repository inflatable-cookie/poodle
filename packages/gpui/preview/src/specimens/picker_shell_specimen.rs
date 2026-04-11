use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_specs::{PickerShellSpec, PickerVariant, BrowseState, SelectionMode};
use poodle_specs::{TextInputSpec, EyebrowSpec};
use poodle_gpui_components::{PickerShell, TextInput, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::style_bridge::color_to_hsla;

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_primary = theme.resolve_color("color.text.primary");
    let hover_bg = theme.resolve_color("color.background.hover");

    div().flex().flex_col().gap(px(24.0))
        // --- Inline variant (ready) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Inline variant (ready)"), theme))
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
                            .child(result_row("Button", "Primitive", text_primary, text_secondary, hover_bg))
                            .child(result_row("Checkbox", "Primitive", text_primary, text_secondary, hover_bg))
                            .child(result_row("Select", "Primitive", text_primary, text_secondary, hover_bg))
                            .child(result_row("Dialog", "Overlay", text_primary, text_secondary, hover_bg))
                            .child(result_row("Table", "Composite", text_primary, text_secondary, hover_bg))
                    )
                )
        )
        // --- Loading state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Loading"), theme))
                .child(
                    PickerShell::from_spec(
                        PickerShellSpec::new("Select an item")
                            .with_variant(PickerVariant::Inline)
                            .with_state(BrowseState::Loading),
                        theme,
                    )
                    .with_results(div())
                )
        )
        // --- No results ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("No results"), theme))
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
        )
        // --- Multiple selection ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Multiple selection"), theme))
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
                            .child(result_row("Button", "Selected", text_primary, text_secondary, hover_bg))
                            .child(result_row("Dialog", "Selected", text_primary, text_secondary, hover_bg))
                            .child(result_row("Table", "", text_primary, text_secondary, hover_bg))
                    )
                )
        )
}

fn result_row(
    label: &str, meta: &str,
    primary: poodle_tokens::typed::ColorValue,
    secondary: poodle_tokens::typed::ColorValue,
    hover: poodle_tokens::typed::ColorValue,
) -> Div {
    let mut row = div()
        .flex().items_center().justify_between()
        .px(px(12.0)).py(px(8.0))
        .hover(|s| s.bg(color_to_hsla(hover)))
        .cursor(CursorStyle::PointingHand)
        .child(div().text_sm().text_color(color_to_hsla(primary)).child(label.to_string()));
    if !meta.is_empty() {
        row = row.child(div().text_xs().text_color(color_to_hsla(secondary)).child(meta.to_string()));
    }
    row
}
