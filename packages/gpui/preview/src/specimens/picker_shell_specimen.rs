use crate::node_compat::{Button, Eyebrow, FormActions, PickerShell, TextInput};
use gpui::*;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{BrowseState, PickerShellSpec, PickerVariant, SelectionMode};
use poodle_specs::{ButtonSpec, ButtonVariant, EyebrowSpec, FormActionsSpec, TextInputSpec};

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_primary = theme.resolve_color("color.text.primary");
    let hover_bg = theme.resolve_color("color.background.hover");

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Inline variant (ready) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Inline variant (ready)"),
                    theme,
                ))
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
                        )
                        .with_id("picker-search-1"),
                    )
                    .with_results(
                        Node::container()
                            .child(result_row(
                                "Button",
                                "Primitive",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Checkbox",
                                "Primitive",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Select",
                                "Primitive",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Dialog",
                                "Overlay",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Table",
                                "Composite",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            )),
                    ),
                ),
        )
        // --- Loading state ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading"),
                    theme,
                ))
                .child(
                    PickerShell::from_spec(
                        PickerShellSpec::new("Select an item")
                            .with_variant(PickerVariant::Inline)
                            .with_state(BrowseState::Loading),
                        theme,
                    )
                    .with_results(Node::container()),
                ),
        )
        // --- No results ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("No results"),
                    theme,
                ))
                .child(
                    PickerShell::from_spec(
                        PickerShellSpec::new("Select an item")
                            .with_variant(PickerVariant::Inline)
                            .with_state(BrowseState::NoResults)
                            .with_query("xyzzy"),
                        theme,
                    )
                    .with_results(Node::container()),
                ),
        )
        // --- Multiple selection ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Multiple selection"),
                    theme,
                ))
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
                        Node::container()
                            .child(result_row(
                                "Button",
                                "Selected",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Dialog",
                                "Selected",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Table",
                                "",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            )),
                    ),
                ),
        )
        // --- Popover variant (search + body + footer) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Popover variant"),
                    theme,
                ))
                .child(
                    PickerShell::from_spec(
                        PickerShellSpec::new("Add component")
                            .with_description("Pick a component to insert.")
                            .with_variant(PickerVariant::Popover)
                            .with_state(BrowseState::Ready)
                            .with_result_count(3)
                            .with_selected_count(1),
                        theme,
                    )
                    .with_search(
                        TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Search components..."),
                            theme,
                        )
                        .with_id("picker-search-popover"),
                    )
                    .with_body(
                        Node::container()
                            .child(result_row(
                                "Button",
                                "Primitive",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Checkbox",
                                "Primitive",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Select",
                                "Primitive",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            )),
                    )
                    .with_footer(picker_footer(theme, "popover")),
                ),
        )
        // --- Modal variant (search + body + footer) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Modal variant"),
                    theme,
                ))
                .child(
                    PickerShell::from_spec(
                        PickerShellSpec::new("Select a relation")
                            .with_description("Choose one or more related records.")
                            .with_variant(PickerVariant::Modal)
                            .with_state(BrowseState::Ready)
                            .with_result_count(3)
                            .with_selected_count(2),
                        theme,
                    )
                    .with_search(
                        TextInput::from_spec(
                            TextInputSpec::new().with_placeholder("Search records..."),
                            theme,
                        )
                        .with_id("picker-search-modal"),
                    )
                    .with_body(
                        Node::container()
                            .child(result_row(
                                "Acme Corp",
                                "Selected",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Globex",
                                "Selected",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            ))
                            .child(result_row(
                                "Initech",
                                "",
                                text_primary,
                                text_secondary,
                                hover_bg,
                            )),
                    )
                    .with_footer(picker_footer(theme, "modal")),
                ),
        )
        // --- Error state ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Error"),
                    theme,
                ))
                .child(
                    PickerShell::from_spec(
                        PickerShellSpec::new("Select an item")
                            .with_variant(PickerVariant::Inline)
                            .with_state(BrowseState::Error),
                        theme,
                    )
                    .with_body(Node::container()),
                ),
        )
        // --- Empty state ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Empty"),
                    theme,
                ))
                .child(
                    PickerShell::from_spec(
                        PickerShellSpec::new("Select an item")
                            .with_variant(PickerVariant::Inline)
                            .with_state(BrowseState::Empty),
                        theme,
                    )
                    .with_body(Node::container()),
                ),
        )
}

/// Confirm/cancel footer built from the real `FormActions` primitive (end-aligned
/// secondary Cancel + primary Add). Matches the picker footer snippet contract.
fn picker_footer(theme: &GpuiThemeProvider, suffix: &str) -> FormActions {
    FormActions::from_spec(FormActionsSpec::new(), theme)
        .with_action(
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Cancel"),
                theme,
            )
            .with_id(format!("picker-cancel-{suffix}")),
        )
        .with_action(
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Primary)
                    .with_label("Add"),
                theme,
            )
            .with_id(format!("picker-confirm-{suffix}")),
        )
}

fn result_row(
    label: &str,
    meta: &str,
    primary: poodle_tokens::typed::ColorValue,
    secondary: poodle_tokens::typed::ColorValue,
    hover: poodle_tokens::typed::ColorValue,
) -> Node {
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.padding.left = 12.0;
        s.descriptor.layout.spacing.padding.right = 12.0;
        s.descriptor.layout.spacing.padding.top = 8.0;
        s.descriptor.layout.spacing.padding.bottom = 8.0;
        s.descriptor.cursor = poodle_node::CursorHint::Pointer;
        s.fill_width = true;
        s.hover = Some(poodle_node::StylePatch {
            background: Some(hover),
            ..Default::default()
        });
    }
    let mut left = Node::text(label);
    left.style.text_size = Some(14.0);
    left.style.descriptor.text_color = Some(primary);
    row = row.child(left);
    if !meta.is_empty() {
        let mut right = Node::text(meta);
        right.style.text_size = Some(12.0);
        right.style.descriptor.text_color = Some(secondary);
        row = row.child(right);
    }
    row
}
