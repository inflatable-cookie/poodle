use gpui::*;
use poodle_components::{
    BrowseState, DrillDownConfig, DrillDownItem, DrillDownLeafGroup, DrillDownLevel,
    PickerItemSpec, RelationPickerSpec, SelectionMode,
};
use poodle_components::{ControlDensity, ControlSize, EyebrowSpec, SemanticControlSizeRole};
use poodle_gpui_components::{DrillEnterArgs, Eyebrow, RelationPicker};
use crate::app_state::AppState;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let items = || vec![
        PickerItemSpec::new("btn", "Button"),
        PickerItemSpec::new("chk", "Checkbox"),
        PickerItemSpec::new("sel", "Select"),
        PickerItemSpec::new("dlg", "Dialog"),
        PickerItemSpec::new("tbl", "Table"),
    ];

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
                vec![
                    PickerItemSpec::new("dd-formlayout", "FormLayout")
                        .with_description("Responsive form grid"),
                ],
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

    div().flex().flex_col().gap(px(24.0))
        // --- Multiple selection ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Multiple selection"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(items())
                            .with_selected_ids(vec!["btn".to_string(), "dlg".to_string()])
                            .with_selection_mode(SelectionMode::Multiple)
                            .with_state(BrowseState::Ready),
                        theme,
                    )
                )
        )
        // --- Single selection ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Single selection"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(items())
                            .with_selected_ids(vec!["sel".to_string()])
                            .with_selection_mode(SelectionMode::Single)
                            .with_state(BrowseState::Ready),
                        theme,
                    )
                )
        )
        // --- Loading state ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Loading state"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(items())
                            .with_selection_mode(SelectionMode::Multiple)
                            .with_state(BrowseState::Loading),
                        theme,
                    )
                )
        )
        // --- Drill-down (Category → Subcategory → Items) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Drill-down (Category \u{2192} Subcategory \u{2192} Items)"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(Vec::new())
                            .with_drill_down(drill_config)
                            .with_drill_down_path(drill_path)
                            .with_selection_mode(SelectionMode::Multiple)
                            .with_state(BrowseState::Ready),
                        theme,
                    )
                    .on_drill_enter(cx.listener(|this, args: &DrillEnterArgs, _w, cx| {
                        let current = this
                            .state
                            .specimens
                            .text
                            .get("relation-picker-drill-path")
                            .cloned()
                            .unwrap_or_default();
                        let next = if current.is_empty() {
                            args.item_id.clone()
                        } else {
                            format!("{}/{}", current, args.item_id)
                        };
                        this.state.specimens.text.insert("relation-picker-drill-path".to_string(), next);
                        cx.notify();
                    }))
                    .on_breadcrumb_click(cx.listener(|this, depth: &usize, _w, cx| {
                        let current = this
                            .state
                            .specimens
                            .text
                            .get("relation-picker-drill-path")
                            .cloned()
                            .unwrap_or_default();
                        let segs: Vec<&str> = if current.is_empty() {
                            Vec::new()
                        } else {
                            current.split('/').collect()
                        };
                        let kept: Vec<&str> = segs.into_iter().take(*depth).collect();
                        let next = kept.join("/");
                        this.state.specimens.text.insert("relation-picker-drill-path".to_string(), next);
                        cx.notify();
                    }))
                )
        )
        // --- Semantic presentation (chrome size role, comfortable density) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Semantic presentation"), theme))
                .child(
                    RelationPicker::from_spec(
                        RelationPickerSpec::new(items())
                            .with_selected_ids(vec!["btn".to_string()])
                            .with_selection_mode(SelectionMode::Multiple)
                            .with_state(BrowseState::Ready)
                            .with_size(ControlSize::Sm)
                            .with_size_role(SemanticControlSizeRole::Chrome)
                            .with_density(ControlDensity::Comfortable),
                        theme,
                    )
                )
        )
}
