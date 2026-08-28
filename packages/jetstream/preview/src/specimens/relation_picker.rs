//! RelationPicker specimen — entity relationship picker.
//!
//! Mirrors the GPUI specimen's full contract coverage: multiple / single
//! selection, loading, drill-down (Category → Subcategory → Items), semantic
//! presentation, filters + footer note + selection summary, empty state, and
//! Sizes / Densities sweeps. Every picker is a real `js_relation_picker` over
//! the shared `PickerShell` / `SelectionSummary` / `Select` composites — no
//! hand-rolled boxes.

use crate::compat::js_relation_picker;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{
    BrowseState, ControlDensity, ControlSize, DrillDownConfig, DrillDownItem, DrillDownLeafGroup,
    DrillDownLevel, PickerFilterConfig, PickerFilterOption, PickerItemSpec, RelationPickerSpec,
    SelectionMode, SemanticControlSizeRole,
};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

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
                    DrillDownItem::new("controls", "Controls")
                        .with_description("Interactive controls")
                        .with_count(2),
                    DrillDownItem::new("display", "Display")
                        .with_description("Display components")
                        .with_count(2),
                    DrillDownItem::new("data", "Data")
                        .with_description("Data display composites")
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
        ],
    );

    let mut root = div().flex_col().gap(24.0);

    // ── Multiple selection (chips via selection summary) ──
    root = root.child(group(
        "Multiple selection",
        secondary,
        js_relation_picker(
            &RelationPickerSpec::new(items())
                .with_selected_ids(vec!["btn".to_string(), "dlg".to_string()])
                .with_selection_mode(SelectionMode::Multiple)
                .with_state(BrowseState::Ready),
            theme,
        ),
    ));

    // ── Single selection ──
    root = root.child(group(
        "Single selection",
        secondary,
        js_relation_picker(
            &RelationPickerSpec::new(items())
                .with_selected_ids(vec!["sel".to_string()])
                .with_selection_mode(SelectionMode::Single)
                .with_state(BrowseState::Ready),
            theme,
        ),
    ));

    // ── Loading state ──
    root = root.child(group(
        "Loading state",
        secondary,
        js_relation_picker(
            &RelationPickerSpec::new(items())
                .with_selection_mode(SelectionMode::Multiple)
                .with_state(BrowseState::Loading),
            theme,
        ),
    ));

    // ── Drill-down (Category → Subcategory → Items) ──
    // Path drilled one level in so the breadcrumb + next-level rows render.
    root = root.child(group(
        "Drill-down (Category \u{2192} Subcategory \u{2192} Items)",
        secondary,
        js_relation_picker(
            &RelationPickerSpec::new(Vec::new())
                .with_drill_down(drill_config)
                .with_drill_down_path(vec!["primitives".to_string()])
                .with_selection_mode(SelectionMode::Multiple)
                .with_state(BrowseState::Ready),
            theme,
        ),
    ));

    // ── Semantic presentation (chrome size role, comfortable density) ──
    root = root.child(group(
        "Semantic presentation",
        secondary,
        js_relation_picker(
            &RelationPickerSpec::new(items())
                .with_selected_ids(vec!["btn".to_string()])
                .with_selection_mode(SelectionMode::Multiple)
                .with_state(BrowseState::Ready)
                .with_size(ControlSize::Sm)
                .with_size_role(SemanticControlSizeRole::Chrome)
                .with_density(ControlDensity::Comfortable),
            theme,
        ),
    ));

    // ── Filters + footer note + selection summary ──
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
    root = root.child(group(
        "Filters, footer note, and selection summary",
        secondary,
        js_relation_picker(
            &RelationPickerSpec::new(items())
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
        ),
    ));

    // ── Empty state ──
    root = root.child(group(
        "Empty state",
        secondary,
        js_relation_picker(
            &RelationPickerSpec::new(Vec::new())
                .with_title("Select components")
                .with_selection_mode(SelectionMode::Multiple)
                .with_state(BrowseState::Empty),
            theme,
        ),
    ));

    // ── Sizes (xs–xl) ──
    root = root.child(label("Sizes").text_color(secondary).text_size(11.0));
    for (name, size) in [
        ("XS", ControlSize::Xs),
        ("SM", ControlSize::Sm),
        ("MD", ControlSize::Md),
        ("LG", ControlSize::Lg),
        ("XL", ControlSize::Xl),
    ] {
        root = root.child(group(
            name,
            secondary,
            js_relation_picker(
                &RelationPickerSpec::new(items())
                    .with_title("Select components")
                    .with_description("Choose related components.")
                    .with_selected_ids(vec!["btn".to_string()])
                    .with_selection_mode(SelectionMode::Multiple)
                    .with_state(BrowseState::Ready)
                    .with_size(size),
                theme,
            ),
        ));
    }

    // ── Densities ──
    root = root.child(label("Densities").text_color(secondary).text_size(11.0));
    for (name, density) in [
        ("Compact", ControlDensity::Compact),
        ("Default", ControlDensity::Default),
        ("Comfortable", ControlDensity::Comfortable),
    ] {
        root = root.child(group(
            name,
            secondary,
            js_relation_picker(
                &RelationPickerSpec::new(items())
                    .with_title("Select components")
                    .with_description("Choose related components.")
                    .with_selected_ids(vec!["btn".to_string()])
                    .with_selection_mode(SelectionMode::Multiple)
                    .with_state(BrowseState::Ready)
                    .with_density(density),
                theme,
            ),
        ));
    }

    root
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
