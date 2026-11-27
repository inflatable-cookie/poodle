use std::fmt::Write;

use poodle_adapter::ThemeProvider;
use poodle_node::{ColorValue, DismissReason, LayoutSizing, NodeKey, NodeKind, NodeRole};
use poodle_render::{select_option_id, select_search_focus_id, select_trigger_focus_id};

use super::*;

struct FixtureTheme;

impl ThemeProvider for FixtureTheme {
    fn try_resolve_color(&self, token: &str) -> Option<ColorValue> {
        match token {
            "color.status.danger" => Some(ColorValue(0.9, 0.2, 0.1, 1.0)),
            "color.status.success" => Some(ColorValue(0.1, 0.7, 0.4, 1.0)),
            _ => Some(ColorValue(0.1, 0.2, 0.3, 1.0)),
        }
    }

    fn resolve_space(&self, token: &str) -> f32 {
        match token {
            "size.control.height" => 32.0,
            "space.control.x" => 12.0,
            _ => 8.0,
        }
    }

    fn resolve_border_width(&self, _: &str) -> f32 {
        1.0
    }

    fn resolve_radius(&self, _: &str) -> f32 {
        4.0
    }

    fn try_resolve_opacity(&self, _: &str) -> Option<f32> {
        None
    }
}

fn by_id<'a>(node: &'a Node, id: &str) -> &'a Node {
    node.find(&|node| node.id.as_deref() == Some(id))
        .unwrap_or_else(|| panic!("missing {id}"))
}

fn has_text(node: &Node, text: &str) -> bool {
    node.texts().contains(&text)
}

fn activate(node: &Node) {
    node.interaction
        .on_activate
        .as_ref()
        .expect("activation handler")();
}

fn snapshot_node(output: &mut String, node: &Node, depth: usize) {
    let indent = "  ".repeat(depth);
    let layout = &node.style.descriptor.layout;
    let kind = match &node.kind {
        NodeKind::Container => "Box".into(),
        NodeKind::Text { content } => format!("Text {content:?}"),
        NodeKind::Icon { name, .. } => format!("Icon {name}"),
        NodeKind::Button { label } => format!("Button {label:?}"),
        NodeKind::Input { value, placeholder } => {
            format!("Input {value:?} placeholder={placeholder:?}")
        }
        NodeKind::ProgressRing { .. } => "ProgressRing".into(),
        _ => panic!("unrecorded specimen node kind"),
    };
    write!(output, "{indent}{kind}").unwrap();
    if let Some(role) = node.a11y.role {
        write!(output, " role={role:?}").unwrap();
    }
    if let Some(label) = &node.a11y.label {
        write!(output, " label={label:?}").unwrap();
    }
    for (name, value) in &node.roles {
        write!(output, " {name}={value}").unwrap();
    }
    if let Some(id) = &node.id {
        write!(output, " id={id}").unwrap();
    }
    if let Some(toggled) = node.a11y.toggled {
        write!(output, " toggled={toggled:?}").unwrap();
    }
    if let Some(expanded) = node.a11y.expanded {
        write!(output, " expanded={expanded}").unwrap();
    }
    if node.a11y.selected == Some(true) {
        output.push_str(" selected");
    }
    if node.interaction.disabled {
        output.push_str(" disabled");
    }
    if node.interaction.on_activate.is_some() {
        output.push_str(" clickable");
    }
    if node.interaction.on_edit_insert.is_some() {
        output.push_str(" editable");
    }
    if let LayoutSizing::Fixed(height) = layout.height {
        write!(output, " height={height}").unwrap();
    }
    if matches!(
        node.a11y.role,
        Some(NodeRole::Button | NodeRole::ColumnHeader)
    ) {
        let padding = layout.spacing.padding;
        write!(output, " padding={},{}", padding.left, padding.top).unwrap();
    }
    if matches!(node.kind, NodeKind::Text { .. }) {
        match node.style.descriptor.text_color {
            Some(ColorValue(0.9, 0.2, 0.1, 1.0)) => output.push_str(" tone=danger"),
            Some(ColorValue(0.1, 0.7, 0.4, 1.0)) => output.push_str(" tone=success"),
            _ => {}
        }
    }
    output.push('\n');
    for child in &node.children {
        snapshot_node(output, child, depth + 1);
    }
}

fn check_snapshot(specimen: Specimen, expected: &str) {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);
    let mut actual = String::new();
    let mut render = |title: &str, session: &Session, view| {
        writeln!(actual, "[{title}]").unwrap();
        let node = session.render(specimen, view, &ctx);
        snapshot_node(&mut actual, &node, 0);
    };
    for pane in specimen.panes() {
        match pane {
            Pane::Examples => render("Examples", &session, View::Examples),
            Pane::Sizes(rows) => {
                for axis_row in *rows {
                    render(
                        &format!("Sizes / {}", axis_row.label),
                        &session,
                        View::Size(axis_row.value),
                    );
                }
            }
            Pane::Densities(rows) => {
                for axis_row in *rows {
                    render(
                        &format!("Densities / {}", axis_row.label),
                        &session,
                        View::Density(axis_row.value),
                    );
                }
            }
        }
    }
    if specimen == Specimen::Select {
        // Closed triggers alone cannot prove option labels, descriptions,
        // disabled options or the search editor's anatomy.
        for scope in ["select-rich", "select-native-grouped", "select-freeform"] {
            let node = session.render(specimen, View::Examples, &ctx);
            activate(by_id(&node, &select_trigger_focus_id(scope)));
            session.drain();
            render(
                &format!("Examples / {scope} open"),
                &session,
                View::Examples,
            );
            let node = session.render(specimen, View::Examples, &ctx);
            activate(by_id(&node, &select_trigger_focus_id(scope)));
            session.drain();
        }
    }
    if std::env::var_os("POODLE_UPDATE_SPECIMEN_SNAPSHOTS").is_some() {
        let path = format!("snapshots/{}.txt", specimen.slug());
        std::fs::write(path, actual).unwrap();
    } else {
        assert_eq!(
            actual.lines().collect::<Vec<_>>(),
            expected.lines().collect::<Vec<_>>()
        );
    }
}

#[test]
fn button_panes() {
    check_snapshot(Specimen::Button, include_str!("../snapshots/button.txt"));
}

#[test]
fn select_panes() {
    check_snapshot(Specimen::Select, include_str!("../snapshots/select.txt"));
}

#[test]
fn data_table_panes() {
    check_snapshot(
        Specimen::DataTable,
        include_str!("../snapshots/data-table.txt"),
    );
}

#[test]
fn button_reports_activations_and_keeps_toggle_states_independent() {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);
    let node = session.render(Specimen::Button, View::Examples, &ctx);
    for id in ["save-changes", "primary", "bookmark", "sections", "publish"] {
        activate(by_id(&node, &format!("specimen-button-{id}")));
    }
    assert_eq!(session.drain(), (true, vec![]));

    let node = session.render(Specimen::Button, View::Examples, &ctx);
    assert!(has_text(&node, "Clicks: 2"));
    assert!(has_text(&node, "Last clicked: Primary"));
    assert!(has_text(&node, "Publish (formaction)"));
    assert_eq!(
        by_id(&node, "specimen-button-sections").a11y.expanded,
        Some(true)
    );
    assert!(has_text(&node, "Bookmarked"));

    activate(by_id(&node, "specimen-button-bookmark"));
    session.drain();
    let node = session.render(Specimen::Button, View::Examples, &ctx);
    assert!(has_text(&node, "Bookmark"));
    assert!(has_text(&node, "Clicks: 2"));
    assert_eq!(
        by_id(&node, "specimen-button-sections").a11y.expanded,
        Some(true)
    );
    assert_eq!(session.drain(), (false, vec![]));
}

#[test]
fn select_replays_query_highlight_caret_commit_and_dismiss() {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);
    let scope = "select-searchable";
    let trigger_id = select_trigger_focus_id(scope);
    let search_id = select_search_focus_id(scope);
    let render = |session: &Session| session.render(Specimen::Select, View::Examples, &ctx);

    activate(by_id(&render(&session), &trigger_id));
    assert_eq!(
        session.drain(),
        (true, vec![Effect::Focus(search_id.clone())])
    );

    let node = render(&session);
    let editor = by_id(&node, &search_id);
    editor.interaction.on_edit_insert.as_ref().unwrap()("s");
    assert_eq!(session.drain(), (true, vec![]));
    let node = render(&session);
    let editor = by_id(&node, &search_id);
    assert!(has_text(editor, "s"));
    assert_eq!(editor.caret.unwrap().selection, (1, 1));
    assert!(node
        .find(&|node| node.id.as_deref() == Some(&select_option_id(scope, "vue")))
        .is_none());
    editor.interaction.on_key.as_ref().unwrap()(NodeKey::ArrowDown, Default::default());
    assert_eq!(session.drain(), (true, vec![]));
    let node = render(&session);
    let highlighted = by_id(&node, &select_option_id(scope, "solid"));
    assert!(highlighted.style.descriptor.background.is_some());
    assert_eq!(highlighted.a11y.selected, Some(false));

    by_id(&node, &search_id)
        .interaction
        .on_edit_key
        .as_ref()
        .unwrap()("left", Default::default());
    assert_eq!(session.drain(), (true, vec![]));
    let node = render(&session);
    let editor = by_id(&node, &search_id);
    assert_eq!(editor.caret.unwrap().selection, (0, 0));
    editor.interaction.on_submit.as_ref().unwrap()();
    assert_eq!(
        session.drain(),
        (true, vec![Effect::Focus(trigger_id.clone())])
    );
    let node = render(&session);
    assert!(has_text(&node, "Selected: solid"));
    assert_eq!(by_id(&node, &trigger_id).a11y.expanded, Some(false));

    let freeform = select_trigger_focus_id("select-freeform");
    let freeform_search = select_search_focus_id("select-freeform");
    activate(by_id(&node, &freeform));
    assert_eq!(
        session.drain(),
        (true, vec![Effect::Focus(freeform_search.clone())])
    );
    by_id(&render(&session), &freeform_search)
        .interaction
        .on_edit_insert
        .as_ref()
        .unwrap()("custom");
    session.drain();
    let node = render(&session);
    by_id(&node, &freeform_search)
        .interaction
        .on_submit
        .as_ref()
        .unwrap()();
    assert_eq!(
        session.drain(),
        (true, vec![Effect::Focus(freeform.clone())])
    );
    assert!(has_text(
        &render(&session),
        "value: custom · query: custom · open: false"
    ));

    activate(by_id(&render(&session), &freeform));
    session.drain();
    by_id(&render(&session), &freeform)
        .interaction
        .on_dismiss
        .as_ref()
        .unwrap()(DismissReason::Escape);
    assert_eq!(session.drain(), (true, vec![Effect::Focus(freeform)]));
    assert!(has_text(&render(&session), "Selected: solid"));
}

#[test]
fn data_table_reorders_rows_and_keeps_selection_by_id() {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);
    let render = |session: &Session| session.render(Specimen::DataTable, View::Examples, &ctx);
    let node = render(&session);
    let table = by_id(&node, "specimen-data-table-live");
    let find_row = |name| {
        table
            .find(&|node| node.a11y.role == Some(NodeRole::Row) && has_text(node, name))
            .unwrap()
    };

    activate(find_row("Alice Chen"));
    activate(find_row("Bob Martinez"));
    let name_header = table
        .find(&|node| node.a11y.label.as_deref() == Some("Name"))
        .unwrap();
    activate(name_header);
    assert_eq!(session.drain(), (true, vec![]));

    let node = render(&session);
    assert!(has_text(&node, "Clicked row 2"));
    assert!(has_text(&node, "2 of 5 selected"));
    let table = by_id(&node, "specimen-data-table-live");
    let row_order = [
        "Eve Nakamura",
        "Dan Okoro",
        "Carol Patel",
        "Bob Martinez",
        "Alice Chen",
    ];
    for (row, name) in table.children.iter().skip(1).zip(row_order) {
        assert!(has_text(row, name), "sorted row should contain {name}");
        assert_eq!(
            row.a11y.selected,
            Some(matches!(name, "Alice Chen" | "Bob Martinez"))
        );
    }

    activate(&table.children[4]); // Bob, after the rows were reordered.
    let role_header = table
        .find(&|node| node.a11y.label.as_deref() == Some("Role"))
        .unwrap();
    activate(role_header);
    session.drain();
    let node = render(&session);
    assert!(has_text(&node, "1 of 5 selected"));
    let table = by_id(&node, "specimen-data-table-live");
    assert!(has_text(&table.children[1], "Bob Martinez"));
    assert_eq!(table.children[1].a11y.selected, Some(false));
}

#[test]
fn axis_rows_are_unique_stable_and_renderable_for_every_specimen() {
    let session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);
    for specimen in Specimen::ALL {
        assert_eq!(Specimen::from_slug(specimen.slug()), Some(specimen));
        assert!(!specimen.title().is_empty());
        let examples = session.render(specimen, View::Examples, &ctx);
        assert_eq!(
            examples.id.as_deref(),
            Some(format!("specimen-{}-examples", specimen.slug()).as_str())
        );
        for pane in specimen.panes() {
            match pane {
                Pane::Examples => {}
                Pane::Sizes(rows) => {
                    assert!(!rows.is_empty(), "{} Sizes pane is empty", specimen.slug());
                    let mut seen = std::collections::BTreeSet::new();
                    for axis_row in *rows {
                        assert!(
                            seen.insert(axis_row.value),
                            "{} duplicate size axis value {}",
                            specimen.slug(),
                            axis_row.value
                        );
                        let _ = session.render(specimen, View::Size(axis_row.value), &ctx);
                    }
                }
                Pane::Densities(rows) => {
                    assert!(
                        !rows.is_empty(),
                        "{} Densities pane is empty",
                        specimen.slug()
                    );
                    let mut seen = std::collections::BTreeSet::new();
                    for axis_row in *rows {
                        assert!(
                            seen.insert(axis_row.value),
                            "{} duplicate density axis value {}",
                            specimen.slug(),
                            axis_row.value
                        );
                        let _ = session.render(specimen, View::Density(axis_row.value), &ctx);
                    }
                }
            }
        }
    }
}

#[test]
fn select_reduces_batched_intents_in_order_before_repaint() {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);
    let scope = "select-searchable";
    let trigger_id = select_trigger_focus_id(scope);
    let search_id = select_search_focus_id(scope);

    // Open the searchable select first so the search editor is mounted.
    activate(by_id(
        &session.render(Specimen::Select, View::Examples, &ctx),
        &trigger_id,
    ));
    assert_eq!(
        session.drain(),
        (true, vec![Effect::Focus(search_id.clone())])
    );

    // Fire multiple inputs against the SAME painted tree without draining or
    // repainting in between: insert "s", insert "o", ArrowDown, Enter.
    let painted = session.render(Specimen::Select, View::Examples, &ctx);
    let editor = by_id(&painted, &search_id);
    let insert = editor.interaction.on_edit_insert.as_ref().unwrap();
    insert("s");
    insert("o");
    editor.interaction.on_key.as_ref().unwrap()(NodeKey::ArrowDown, Default::default());
    editor.interaction.on_submit.as_ref().unwrap()();

    assert_eq!(
        session.drain(),
        (true, vec![Effect::Focus(trigger_id.clone())])
    );
    let after = session.render(Specimen::Select, View::Examples, &ctx);
    assert!(has_text(&after, "Selected: solid"));
    assert_eq!(by_id(&after, &trigger_id).a11y.expanded, Some(false));
}

#[test]
fn select_reduces_closed_to_open_batch_before_repaint() {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);
    let scope = "select-native";
    let trigger_id = select_trigger_focus_id(scope);

    // On a closed Select, End and Enter alone remain inert.
    let closed = session.render(Specimen::Select, View::Examples, &ctx);
    let trigger = by_id(&closed, &trigger_id);
    assert_eq!(trigger.a11y.expanded, Some(false));
    trigger.interaction.on_key.as_ref().unwrap()(NodeKey::End, Default::default());
    trigger.interaction.on_submit.as_ref().unwrap()();
    assert_eq!(session.drain(), (true, vec![]));
    let still_closed = session.render(Specimen::Select, View::Examples, &ctx);
    assert!(!has_text(&still_closed, "Selected: elderberry"));
    assert_eq!(by_id(&still_closed, &trigger_id).a11y.expanded, Some(false));

    // Starting from the same closed tree, batch ArrowDown (opens) + End
    // (jumps to last option) + Enter (commits last option and closes) without
    // draining or repainting in between.
    let trigger = by_id(&still_closed, &trigger_id);
    let on_key = trigger.interaction.on_key.as_ref().unwrap();
    on_key(NodeKey::ArrowDown, Default::default());
    on_key(NodeKey::End, Default::default());
    trigger.interaction.on_submit.as_ref().unwrap()();
    assert_eq!(
        session.drain(),
        (
            true,
            vec![
                Effect::Focus(trigger_id.clone()),
                Effect::Focus(trigger_id.clone()),
            ]
        )
    );

    let after = session.render(Specimen::Select, View::Examples, &ctx);
    assert!(has_text(&after, "Selected: elderberry"));
    assert_eq!(by_id(&after, &trigger_id).a11y.expanded, Some(false));
}

#[test]
fn text_input_reduces_batched_intents_in_order_before_repaint() {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);

    let painted = session.render(Specimen::TextInput, View::Examples, &ctx);
    let name = by_id(&painted, "poodle-input-name-field");
    let insert = name.interaction.on_edit_insert.as_ref().unwrap();
    insert("J");
    insert("a");
    insert("n");
    insert("e");
    assert_eq!(session.drain(), (true, vec![]));

    let after = session.render(Specimen::TextInput, View::Examples, &ctx);
    let name = by_id(&after, "poodle-input-name-field");
    assert!(has_text(name, "Jane"));
    assert_eq!(name.caret.unwrap().selection, (4, 4));

    // Replace invalid email with a valid one and verify validation error clears.
    assert!(has_text(&after, "Please enter a valid email address."));
    by_id(&after, "poodle-input-email-field")
        .interaction
        .on_text_change
        .as_ref()
        .unwrap()("jane@example.com");
    assert_eq!(session.drain(), (true, vec![]));
    let after = session.render(Specimen::TextInput, View::Examples, &ctx);
    assert!(!has_text(&after, "Please enter a valid email address."));
}

#[test]
fn number_input_reduces_batched_intents_in_order_before_repaint() {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);

    // Click increment twice against the same painted tree: 1 -> 2 -> 3.
    let painted = session.render(Specimen::NumberInput, View::Examples, &ctx);
    let inc = by_id(&painted, "poodle-number-input-quantity-inc");
    activate(inc);
    activate(inc);
    assert_eq!(session.drain(), (true, vec![]));

    let after = session.render(Specimen::NumberInput, View::Examples, &ctx);
    assert!(has_text(&after, "Quantity: 3"));

    // Type "4", "2" and submit against the same painted tree (replacing via
    // backspace first): "3" -> "" -> "4" -> "42" -> commit 42.
    let field = by_id(&after, "poodle-number-input-quantity");
    field.interaction.on_edit_key.as_ref().unwrap()("backspace", Default::default());
    let insert = field.interaction.on_edit_insert.as_ref().unwrap();
    insert("4");
    insert("2");
    field.interaction.on_submit.as_ref().unwrap()();
    assert_eq!(session.drain(), (true, vec![]));

    let after = session.render(Specimen::NumberInput, View::Examples, &ctx);
    assert!(has_text(&after, "Quantity: 42"));
}

#[test]
fn code_input_reduces_batched_intents_in_order_before_repaint() {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);

    let painted = session.render(Specimen::CodeInput, View::Examples, &ctx);
    let code = by_id(&painted, "specimen-code-input-code");
    let row = code
        .find(&|node| node.interaction.on_edit_key.is_some())
        .expect("editable code row");
    let key = row.interaction.on_edit_key.as_ref().unwrap();
    for digit in ["1", "2", "3", "4", "5", "6"] {
        key(digit, Default::default());
    }
    assert_eq!(session.drain(), (true, vec![]));

    let after = session.render(Specimen::CodeInput, View::Examples, &ctx);
    assert!(has_text(&after, "Code complete!"));
    assert!(after
        .find(&|node| node.a11y.label.as_deref() == Some("Code check passed"))
        .is_some());
}

#[test]
fn first_batch_controls_update_session_state() {
    let mut session = Session::default();
    let ctx = RenderContext::new(&FixtureTheme);

    // Checkbox
    let node = session.render(Specimen::Checkbox, View::Examples, &ctx);
    activate(by_id(&node, "checkbox-marketing"));
    assert_eq!(session.drain(), (true, vec![]));
    let node = session.render(Specimen::Checkbox, View::Examples, &ctx);
    assert_eq!(
        by_id(&node, "checkbox-marketing").a11y.toggled,
        Some(poodle_node::NodeToggled::True)
    );

    // Switch
    let node = session.render(Specimen::Switch, View::Examples, &ctx);
    activate(by_id(&node, "switch-auto-save"));
    assert_eq!(session.drain(), (true, vec![]));
    let node = session.render(Specimen::Switch, View::Examples, &ctx);
    assert_eq!(
        by_id(&node, "switch-auto-save").a11y.toggled,
        Some(poodle_node::NodeToggled::True)
    );

    // Radio
    let node = session.render(Specimen::Radio, View::Examples, &ctx);
    activate(by_id(&node, "radio-express"));
    assert_eq!(session.drain(), (true, vec![]));
    let node = session.render(Specimen::Radio, View::Examples, &ctx);
    assert_eq!(
        by_id(&node, "radio-express").a11y.toggled,
        Some(poodle_node::NodeToggled::True)
    );

    // RadioGroup
    let node = session.render(Specimen::RadioGroup, View::Examples, &ctx);
    let enterprise = node
        .find(&|n| {
            n.a11y.role == Some(NodeRole::RadioButton)
                && n.interaction.on_activate.is_some()
                && has_text(n, "Enterprise")
        })
        .expect("enterprise radio option");
    activate(enterprise);
    assert_eq!(session.drain(), (true, vec![]));
    let node = session.render(Specimen::RadioGroup, View::Examples, &ctx);
    assert!(has_text(&node, "Selected: enterprise"));

    // TriStateSwitch
    let node = session.render(Specimen::TriStateSwitch, View::Examples, &ctx);
    let include_btn = node
        .find(&|n| {
            n.interaction.on_activate.is_some()
                && n.runtime_id
                    .as_deref()
                    .is_some_and(|id| id.contains("tri-state-filter") && id.contains("included"))
        })
        .expect("included option");
    activate(include_btn);
    assert_eq!(session.drain(), (true, vec![]));
    let node = session.render(Specimen::TriStateSwitch, View::Examples, &ctx);
    assert!(has_text(&node, "Value: included"));

    // SegmentedControl (including icon-only)
    let node = session.render(Specimen::SegmentedControl, View::Examples, &ctx);
    let table_seg = node
        .find(&|n| n.interaction.on_activate.is_some() && has_text(n, "Table"))
        .expect("table segment");
    activate(table_seg);
    assert_eq!(session.drain(), (true, vec![]));
    let node = session.render(Specimen::SegmentedControl, View::Examples, &ctx);
    assert!(has_text(&node, "Selected: table"));

    // ToggleGroup
    let node = session.render(Specimen::ToggleGroup, View::Examples, &ctx);
    let board = node
        .find(&|n| n.interaction.on_activate.is_some() && has_text(n, "Board"))
        .expect("board toggle");
    activate(board);
    assert_eq!(session.drain(), (true, vec![]));
    let node = session.render(Specimen::ToggleGroup, View::Examples, &ctx);
    assert!(has_text(&node, "View: board"));

    // IconButton
    let node = session.render(Specimen::IconButton, View::Examples, &ctx);
    activate(by_id(&node, "specimen-icon-button-add"));
    activate(by_id(&node, "specimen-icon-button-bold"));
    assert_eq!(session.drain(), (true, vec![]));
    let node = session.render(Specimen::IconButton, View::Examples, &ctx);
    assert!(has_text(&node, "Last action: Add"));
    assert_eq!(
        by_id(&node, "specimen-icon-button-bold").a11y.toggled,
        Some(poodle_node::NodeToggled::True)
    );

    // SplitButton
    let node = session.render(Specimen::SplitButton, View::Examples, &ctx);
    let save_root = by_id(&node, "specimen-split-button-save");
    let primary = save_root
        .find(&|n| n.interaction.on_activate.is_some() && has_text(n, "Save"))
        .expect("save primary button");
    activate(primary);
    assert_eq!(session.drain(), (true, vec![]));
    let node = session.render(Specimen::SplitButton, View::Examples, &ctx);
    assert!(has_text(&node, "Last action: Save"));

    // Rating
    let node = session.render(Specimen::Rating, View::Examples, &ctx);
    let clear_star = node
        .find(&|n| {
            n.interaction.on_activate.is_some()
                && n.id
                    .as_deref()
                    .is_some_and(|id| id.contains("specimen-whole"))
        })
        .expect("whole rating star");
    activate(clear_star);
    assert_eq!(session.drain(), (true, vec![]));
}
