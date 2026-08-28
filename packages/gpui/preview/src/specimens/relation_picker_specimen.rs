use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, RelationPicker};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    BrowseState, DrillDownConfig, DrillDownItem, DrillDownLeafGroup, DrillDownLevel,
    PickerFilterConfig, PickerFilterOption, PickerItemSpec, RelationPickerSpec, SelectionMode,
};
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec, SemanticControlSizeRole};
use std::sync::Arc;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let items = || {
        vec![
            PickerItemSpec::new("btn", "Button"),
            PickerItemSpec::new("chk", "Checkbox"),
            PickerItemSpec::new("sel", "Select"),
            PickerItemSpec::new("dlg", "Dialog"),
            PickerItemSpec::new("tbl", "Table"),
        ]
    };

    // Drill-down fixture: Categories → Subcategories → leaf primitives.
    let drill_config = DrillDownConfig::new(
        vec![
            DrillDownLevel::new(
                "category",
                "Category",
                vec![
                    DrillDownItem::new("primitives", "Primitives")
                        .with_description("Foundation-level building blocks")
                        .with_count(4),
                    DrillDownItem::new("composites", "Composites")
                        .with_description("Higher-level composed components")
                        .with_count(3),
                    DrillDownItem::new("layout", "Layout")
                        .with_description("Structural layout utilities")
                        .with_count(2),
                ],
            )
            .with_search_placeholder("Search categories\u{2026}"),
            DrillDownLevel::new(
                "subcategory",
                "Subcategory",
                vec![
                    // Primitives → Controls, Display
                    DrillDownItem::new("controls", "Controls")
                        .with_description("Interactive controls")
                        .with_count(2),
                    DrillDownItem::new("display", "Display")
                        .with_description("Display components")
                        .with_count(2),
                    // Composites → Data, Forms
                    DrillDownItem::new("data", "Data")
                        .with_description("Data display composites")
                        .with_count(2),
                    DrillDownItem::new("forms", "Forms")
                        .with_description("Form composites")
                        .with_count(1),
                    // Layout → Containers
                    DrillDownItem::new("containers", "Containers")
                        .with_description("Layout containers")
                        .with_count(2),
                ],
            )
            .with_search_placeholder("Search subcategories\u{2026}"),
        ],
        vec![
            DrillDownLeafGroup::new(
                "controls",
                vec![
                    PickerItemSpec::new("dd-btn", "Button")
                        .with_description("Primary interactive control"),
                    PickerItemSpec::new("dd-checkbox", "Checkbox")
                        .with_description("Boolean toggle control"),
                ],
            ),
            DrillDownLeafGroup::new(
                "display",
                vec![
                    PickerItemSpec::new("dd-badge", "Badge")
                        .with_description("Status indicator label"),
                    PickerItemSpec::new("dd-pill", "Pill")
                        .with_description("Removable tag element"),
                ],
            ),
            DrillDownLeafGroup::new(
                "data",
                vec![
                    PickerItemSpec::new("dd-datatable", "DataTable")
                        .with_description("Sortable data grid"),
                    PickerItemSpec::new("dd-detailsection", "DetailSection")
                        .with_description("Key-value detail view"),
                ],
            ),
            DrillDownLeafGroup::new(
                "forms",
                vec![PickerItemSpec::new("dd-formlayout", "FormLayout")
                    .with_description("Responsive form grid")],
            ),
            DrillDownLeafGroup::new(
                "containers",
                vec![
                    PickerItemSpec::new("dd-box", "Box")
                        .with_description("Generic layout container"),
                    PickerItemSpec::new("dd-surface", "Surface")
                        .with_description("Elevated content surface"),
                ],
            ),
        ],
    );

    // Read current drill-down path from specimen state. Stored as a
    // slash-separated string in `relation-picker-drill-path`.
    let drill_path: Vec<String> = state
        .specimens
        .text
        .get("relation-picker-drill-path")
        .filter(|s| !s.is_empty())
        .map(|s| s.split('/').map(|x| x.to_string()).collect())
        .unwrap_or_default();

    let mut root = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
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
                .child(RelationPicker::from_spec(
                    RelationPickerSpec::new(items())
                        .with_selected_ids(vec!["btn".to_string(), "dlg".to_string()])
                        .with_selection_mode(SelectionMode::Multiple)
                        .with_state(BrowseState::Ready),
                    theme,
                )),
        )
        // --- Single selection ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Single selection"),
                    theme,
                ))
                .child(RelationPicker::from_spec(
                    RelationPickerSpec::new(items())
                        .with_selected_ids(vec!["sel".to_string()])
                        .with_selection_mode(SelectionMode::Single)
                        .with_state(BrowseState::Ready),
                    theme,
                )),
        )
        // --- Loading state ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Loading state"),
                    theme,
                ))
                .child(RelationPicker::from_spec(
                    RelationPickerSpec::new(items())
                        .with_selection_mode(SelectionMode::Multiple)
                        .with_state(BrowseState::Loading),
                    theme,
                )),
        )
        // --- Drill-down (Category → Subcategory → Items) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new()
                        .with_content("Drill-down (Category \u{2192} Subcategory \u{2192} Items)"),
                    theme,
                ))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(Vec::new())
                            .with_drill_down(drill_config)
                            .with_drill_down_path(drill_path.clone())
                            .with_selection_mode(SelectionMode::Multiple)
                            .with_state(BrowseState::Ready),
                        theme,
                    )
                    .on_drill_enter({
                        let events = Arc::clone(&state.node_events);
                        let current_path = drill_path.join("/");
                        Arc::new(move |item_id: &str| {
                            let next = if current_path.is_empty() {
                                item_id.to_string()
                            } else {
                                format!("{current_path}/{item_id}")
                            };
                            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                                key: "relation-picker-drill-path".to_string(),
                                value: next,
                            });
                        })
                    })
                    .on_breadcrumb_click({
                        let events = Arc::clone(&state.node_events);
                        let current_path = drill_path.clone();
                        Arc::new(move |depth: usize| {
                            let next = current_path
                                .iter()
                                .take(depth)
                                .cloned()
                                .collect::<Vec<_>>()
                                .join("/");
                            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                                key: "relation-picker-drill-path".to_string(),
                                value: next,
                            });
                        })
                    }),
                ),
        )
        // --- Semantic presentation (chrome size role, comfortable density) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Semantic presentation"),
                    theme,
                ))
                .child(RelationPicker::from_spec(
                    RelationPickerSpec::new(items())
                        .with_selected_ids(vec!["btn".to_string()])
                        .with_selection_mode(SelectionMode::Multiple)
                        .with_state(BrowseState::Ready)
                        .with_size(ControlSize::Sm)
                        .with_size_role(SemanticControlSizeRole::Chrome)
                        .with_density(ControlDensity::Comfortable),
                    theme,
                )),
        );

    // --- Filters + footer note + selection summary ---
    let filters = vec![
        PickerFilterConfig::new(
            "kind",
            "Kind",
            vec![
                PickerFilterOption::new("primitive", "Primitive"),
                PickerFilterOption::new("composite", "Composite"),
            ],
        ),
        PickerFilterConfig::new(
            "status",
            "Status",
            vec![
                PickerFilterOption::new("stable", "Stable"),
                PickerFilterOption::new("beta", "Beta"),
            ],
        ),
    ];
    root = root.child(
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Eyebrow::from_spec(
                EyebrowSpec::new().with_content("Filters, footer note, and selection summary"),
                theme,
            ))
            .child(RelationPicker::from_spec(
                RelationPickerSpec::new(items())
                    .with_title("Select components")
                    .with_description("Choose related components.")
                    .with_selected_ids(vec!["btn".to_string(), "dlg".to_string()])
                    .with_selection_mode(SelectionMode::Multiple)
                    .with_state(BrowseState::Ready)
                    .with_search_placeholder("Search components\u{2026}")
                    .with_filters(filters)
                    .with_filter_value("kind", "primitive")
                    .with_footer_note("Two of five components selected.")
                    .with_show_footer(true)
                    .with_show_selection_summary(true),
                theme,
            )),
    );

    // --- Empty state ---
    root = root.child(
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Eyebrow::from_spec(
                EyebrowSpec::new().with_content("Empty state"),
                theme,
            ))
            .child(RelationPicker::from_spec(
                RelationPickerSpec::new(Vec::new())
                    .with_title("Select components")
                    .with_selection_mode(SelectionMode::Multiple)
                    .with_state(BrowseState::Empty),
                theme,
            )),
    );

    // --- Sizes (xs–xl) ---
    let mut sizes_row = div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content("Sizes"),
            theme,
        ));
    for (label, size) in [
        ("XS", ControlSize::Xs),
        ("SM", ControlSize::Sm),
        ("MD", ControlSize::Md),
        ("LG", ControlSize::Lg),
        ("XL", ControlSize::Xl),
    ] {
        sizes_row = sizes_row.child(
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(color_to_hsla(text_secondary))
                        .child(label),
                )
                .child(RelationPicker::from_spec(
                    RelationPickerSpec::new(items())
                        .with_title("Select components")
                        .with_description("Choose related components.")
                        .with_selected_ids(vec!["btn".to_string()])
                        .with_selection_mode(SelectionMode::Multiple)
                        .with_state(BrowseState::Ready)
                        .with_size(size),
                    theme,
                )),
        );
    }
    root = root.child(sizes_row);

    // --- Densities ---
    let mut densities_row = div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content("Densities"),
            theme,
        ));
    for (label, density) in [
        ("Compact", ControlDensity::Compact),
        ("Default", ControlDensity::Default),
        ("Comfortable", ControlDensity::Comfortable),
    ] {
        densities_row = densities_row.child(
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(color_to_hsla(text_secondary))
                        .child(label),
                )
                .child(RelationPicker::from_spec(
                    RelationPickerSpec::new(items())
                        .with_title("Select components")
                        .with_description("Choose related components.")
                        .with_selected_ids(vec!["btn".to_string()])
                        .with_selection_mode(SelectionMode::Multiple)
                        .with_state(BrowseState::Ready)
                        .with_density(density),
                    theme,
                )),
        );
    }
    root = root.child(densities_row);
    let examples = root.into_any_element();

    specimen_layout(
        state,
        cx,
        "relation-picker",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                RelationPicker::from_spec(
                    RelationPickerSpec::new(items())
                        .with_selected_ids(vec!["btn".to_string()])
                        .with_selection_mode(SelectionMode::Multiple)
                        .with_state(BrowseState::Ready)
                        .with_size(size),
                    theme,
                )
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                RelationPicker::from_spec(
                    RelationPickerSpec::new(items())
                        .with_selected_ids(vec!["btn".to_string()])
                        .with_selection_mode(SelectionMode::Multiple)
                        .with_state(BrowseState::Ready)
                        .with_density(density),
                    theme,
                )
                .into_any_element()
            }),
    )
}
