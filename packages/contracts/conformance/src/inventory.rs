use poodle_node::{
    Interaction, Node, NodeA11y, NodeDragSource, NodeDropTarget, NodeKind, NodeRole,
};

use crate::digest;

// No rest patterns. Adding a field to any of these records requires choosing
// its case or its classification. The same macro is exercised by the
// compile-fail test below.
macro_rules! fields {
    ($ty:ident { $($field:ident => $class:literal),+ $(,)? }) => {{
        let _exhaustive = |$ty { $($field: _,)+ }: $ty| {};
        &[$((concat!(stringify!($ty), ".", stringify!($field)), $class)),+]
    }};
}

pub(crate) fn fields_inventory() -> impl Iterator<Item = (&'static str, &'static str)> {
    let node = fields!(Node {
        kind => "kind inventory", a11y => "accessibility inventory",
        interaction => "interaction inventory", id => "fixture target identity",
        runtime_id => "backend state identity, deferred", tooltip => "tooltip dispatch, deferred",
        tooltip_bubble => "tooltip dispatch, deferred", tooltip_open_change => "tooltip dispatch, deferred",
        style => "representative layout only", position => "representative layout only",
        datetime => "semantic metadata, deferred", roles => "diagnostics",
        caret => "editing, deferred", children => "fixture tree",
    });
    let a11y = fields!(NodeA11y {
        role => "a11y.role", label => "a11y.label", expanded => "a11y.expanded",
        selected => "a11y.selected", multiselectable => "a11y.multiselectable",
        hidden => "a11y.hidden", initial_focus => "a11y.initial_focus", tab_index => "a11y.tab_index",
        controls => "a11y.controls", has_popup => "a11y.has_popup", labelled_by => "a11y.labelled_by",
        orientation => "a11y.orientation", toggled => "a11y.toggled", level => "a11y.level",
        value => "a11y.value", value_min => "a11y.value_min", value_max => "a11y.value_max",
        value_text => "a11y.value_text", invalid => "a11y.invalid", busy => "a11y.busy",
        described_by => "a11y.described_by",
    });
    let interaction = fields!(Interaction {
        on_activate => "dispatch.activate",
        on_text_change => "dispatch deferred", on_submit => "dispatch deferred",
        on_cancel => "dispatch deferred", on_edit_key => "dispatch deferred",
        on_select_range => "dispatch deferred", on_edit_insert => "dispatch deferred",
        on_focus_change => "dispatch deferred", on_drag => "dispatch deferred",
        on_scrub => "dispatch deferred", on_continuous_value => "dispatch deferred",
        on_wheel => "dispatch deferred", on_scroll => "dispatch deferred",
        on_double_activate => "dispatch deferred", on_activate_modified => "dispatch deferred",
        on_context => "dispatch deferred", on_dismiss => "dispatch deferred",
        on_key => "dispatch deferred", on_key_activate => "dispatch deferred",
        on_key_up => "dispatch deferred",
        focusable => "focus fixture setup", disabled => "dispatch deferred",
        copy_text => "dispatch deferred", scrub_axis => "dispatch deferred",
        request_focus => "dispatch deferred", dismiss_layer => "dispatch deferred",
        refuses_outside_dismiss => "dispatch deferred", trap_focus => "dispatch deferred",
        drag_source => "drag registration inventory", drop_target => "drop registration inventory",
    });
    let source = fields!(NodeDragSource {
        source_id => "drag dispatch deferred", subject => "drag dispatch deferred",
        allowed_operations => "drag dispatch deferred", operation => "drag dispatch deferred",
        disabled => "drag dispatch deferred", label => "drag dispatch deferred",
        instructions => "drag dispatch deferred", keyboard_order => "drag dispatch deferred",
        owns_announcements => "drag dispatch deferred", cross_window_source_bridge => "host bridge deferred",
        file_export_bridge => "host bridge deferred", on_drag_start => "drag dispatch deferred",
        on_drag_end => "drag dispatch deferred",
    });
    let target = fields!(NodeDropTarget {
        target_id => "drop dispatch deferred", accepted_kinds => "drop dispatch deferred",
        disabled => "drop dispatch deferred", priority => "drop dispatch deferred",
        label => "drop dispatch deferred", resolve_position => "drop dispatch deferred",
        keyboard_order => "drop dispatch deferred", resolve_keyboard_position => "drop dispatch deferred",
        inbound_files => "host bridge deferred", can_drop => "drop dispatch deferred",
        on_drop => "drop dispatch deferred", on_intent => "drop dispatch deferred",
        on_intent_cleared => "drop dispatch deferred",
    });
    [node.as_slice(), a11y, interaction, source, target]
        .into_iter()
        .flatten()
        .copied()
}

macro_rules! roles {
    ($($variant:ident),+ $(,)?) => {
        pub(crate) const ROLES: &[NodeRole] = &[$(NodeRole::$variant),+];
        pub(crate) fn role_name(role: NodeRole) -> &'static str {
            match role { $(NodeRole::$variant => stringify!($variant),)+ }
        }
    };
}

roles! {
    Alert, AlertDialog, Banner, Button, Cell, CheckBox, ColumnHeader, ComboBox, ContentInfo,
    Dialog, Figure, Grid, Group, Heading, SearchBox, Label, Link, List, ListItem, ListBox,
    ListBoxOption, Log, Image, Menu, MenuBar, MenuItem, MenuItemCheckBox, MenuItemRadio,
    Meter, Navigation, Presentation, Splitter, Slider, ProgressIndicator, RadioGroup, RadioButton,
    Region, Row, SpinButton, Status, Switch, Table, RowHeader, Tab, TabList, TabPanel, TextInput,
    Toolbar, Tooltip, Tree, TreeItem,
}

macro_rules! kinds {
    ($($variant:ident { $($field:ident),* }),+ $(,)?) => {
        pub(crate) const KINDS: &[&str] = &[$(stringify!($variant)),+];
        pub(crate) fn kind_name(kind: &NodeKind) -> &'static str {
            match kind {
                $(NodeKind::$variant { $($field: _,)* } => stringify!($variant),)+
            }
        }
    };
}

kinds! {
    Container {}, Text { content }, Button { label }, Input { value, placeholder },
    Progress { fraction }, ProgressRing { fraction }, Icon { name, size },
    ResolvedIconGeometry { size, frame }, Image { source },
}

/// SHA-256 of the declarations and classifications covered by the pilot.
pub fn vocabulary_fingerprint() -> String {
    let node = include_str!("../../node/src/lib.rs");
    let drag = include_str!("../../node/src/drag.rs");
    // Hash declaration bodies, with comments and formatting removed.
    let mut vocabulary = String::new();
    for (source, name) in [
        (node, "struct Node"),
        (node, "enum NodeKind"),
        (node, "enum NodeRole"),
        (node, "struct NodeA11y"),
        (node, "struct Interaction"),
        (drag, "struct NodeDragSource"),
        (drag, "struct NodeDropTarget"),
    ] {
        let (_, body) = source
            .split_once(&format!("pub {name} {{\n"))
            .expect("inventoried vocabulary declaration");
        let (body, _) = body
            .split_once("\n}")
            .expect("end of vocabulary declaration");
        vocabulary.push_str(name);
        for line in body
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
        {
            vocabulary.extend(line.chars().filter(|c| !c.is_whitespace()));
        }
        vocabulary.push('\n');
    }
    vocabulary.push_str(&KINDS.join("\n"));
    vocabulary.push('\n');
    for (field, class) in fields_inventory() {
        vocabulary.push_str(field);
        vocabulary.push('=');
        vocabulary.push_str(class);
        vocabulary.push('\n');
    }
    digest(vocabulary.as_bytes())
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_declared_case_is_in_the_manifest() {
        use super::{fields_inventory, role_name, KINDS, ROLES};
        use std::collections::BTreeSet;

        let cases: Vec<_> = crate::cases().collect();
        let ids: BTreeSet<_> = cases.iter().map(|case| case.id.as_str()).collect();
        assert_eq!(cases.len(), ids.len(), "duplicate fixture ID");
        for id in KINDS.iter().map(|kind| format!("kind.{kind}")).chain(
            ROLES
                .iter()
                .map(|&role| format!("role.{}", role_name(role))),
        ) {
            assert!(ids.contains(id.as_str()), "missing vocabulary fixture {id}");
        }
        for (_, classification) in fields_inventory() {
            if classification.starts_with("a11y.") || classification.starts_with("dispatch.") {
                assert!(
                    ids.contains(classification),
                    "missing field fixture {classification}"
                );
            }
        }
    }

    #[test]
    fn new_field_fails_the_inventory_pattern() {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let source = include_str!("inventory.rs");
        let (_, macro_body) = source
            .split_once("macro_rules! fields")
            .expect("field macro");
        let (macro_body, _) = macro_body
            .split_once("pub(crate) fn")
            .expect("end of field macro");
        let planted = "pub planted_omission: bool,";
        for (extra, classification, compiles) in [
            ("", "", true),
            (planted, "", false),
            (planted, ", planted_omission => \"deferred\"", true),
        ] {
            let mut compiler = Command::new("rustc")
                .args([
                    "--edition=2021",
                    "--crate-type=lib",
                    "--emit=metadata=-",
                    "--error-format=json",
                    "-",
                ])
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()
                .expect("start rustc for exhaustive inventory test");
            write!(
                compiler.stdin.take().expect("rustc stdin"),
                "macro_rules! fields{macro_body}\npub struct Dummy {{ pub known: bool, {extra} }}\nfn check() {{ let _ = fields!(Dummy {{ known => \"deferred\"{classification} }}); }}"
            )
            .expect("write dummy vocabulary");
            let output = compiler.wait_with_output().expect("rustc result");
            let errors = String::from_utf8_lossy(&output.stderr);
            assert_eq!(output.status.success(), compiles, "{errors}");
            if !compiles {
                assert!(
                    (errors.contains("E0027") && errors.contains("planted_omission"))
                        || errors.contains("pattern requires `..` due to inaccessible fields"),
                    "{errors}"
                );
            }
        }
    }
}
