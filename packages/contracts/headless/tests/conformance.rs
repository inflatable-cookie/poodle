//! Cross-runtime conformance: runs the shared vectors in
//! `vectors/machines.json` against the Rust machines. The TypeScript core
//! runs the same vectors (packages/core/test/conformance.test.ts).
//!
//! Effects are serialized back to the vector JSON shape and compared
//! order-sensitively.

use serde_json::{json, Value};

use poodle_headless::audio::*;
use poodle_headless::checkbox::*;
use poodle_headless::disclosure::*;
use poodle_headless::drag_drop::*;
use poodle_headless::edit::*;
use poodle_headless::hover::*;
use poodle_headless::menu::*;
use poodle_headless::modal::*;
use poodle_headless::select::{
    select_transition, SelectContext as SelectMachineContext, SelectEffect as SelectMachineEffect,
    SelectEvent as SelectMachineEvent, SelectOptionState,
};
use poodle_headless::single_select::*;
use poodle_headless::slider::*;
use poodle_headless::switch::*;
use poodle_headless::tabs::*;
use poodle_headless::toggle_group::*;

fn vectors() -> Value {
    let raw = include_str!("../vectors/machines.json");
    serde_json::from_str(raw).expect("vectors parse")
}

fn b(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn f(value: &Value, key: &str) -> f64 {
    value.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

fn s<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or("")
}

fn opt_string(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_string)
}

fn opt_usize(value: &Value, key: &str) -> Option<usize> {
    match value.get(key) {
        None | Some(Value::Null) => None,
        Some(entry) => entry.as_u64().map(|n| n as usize),
    }
}

fn options_from(value: &Value) -> Vec<SelectOption> {
    value["options"]
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .map(|entry| SelectOption {
                    value: s(entry, "value").to_string(),
                    disabled: b(entry, "disabled"),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Canonicalize numbers (all become f64) so `300` and `300.0` compare equal
/// across the two runtimes' serializers.
fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Number(number) => json!(number.as_f64().unwrap_or(0.0)),
        Value::Array(entries) => Value::Array(entries.iter().map(canonicalize).collect()),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, entry)| (key.clone(), canonicalize(entry)))
                .collect(),
        ),
        other => other.clone(),
    }
}

fn assert_case(
    machine: &str,
    case: &Value,
    actual_effects: Vec<Value>,
    actual_state: Option<&str>,
    actual_context: Option<Value>,
) {
    let name = s(case, "name");
    let expect = &case["expect"];

    if let Some(expected_state) = expect.get("state").and_then(Value::as_str) {
        assert_eq!(
            actual_state,
            Some(expected_state),
            "{machine}/{name}: state"
        );
    }

    let expected_effects: Vec<Value> = expect["effects"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .iter()
        .map(canonicalize)
        .collect();
    let actual_effects: Vec<Value> = actual_effects.iter().map(canonicalize).collect();
    assert_eq!(
        actual_effects, expected_effects,
        "{machine}/{name}: effects"
    );

    if let (Some(expected_context), Some(actual)) = (
        expect.get("context").and_then(Value::as_object),
        actual_context,
    ) {
        for (key, expected_value) in expected_context {
            assert_eq!(
                canonicalize(&actual[key]),
                canonicalize(expected_value),
                "{machine}/{name}: context.{key}"
            );
        }
    }
}

#[test]
fn checkbox_conformance() {
    for case in vectors()["checkbox"].as_array().unwrap() {
        let ctx = &case["context"];
        let context = CheckboxContext {
            checked: b(ctx, "checked"),
            mixed: b(ctx, "mixed"),
            disabled: b(ctx, "disabled"),
            read_only: b(ctx, "readOnly"),
        };
        let event = match s(&case["event"], "type") {
            "TOGGLE" => CheckboxEvent::Toggle {
                next_checked: b(&case["event"], "nextChecked"),
            },
            "SET_CHECKED" => CheckboxEvent::SetChecked {
                checked: b(&case["event"], "checked"),
            },
            other => panic!("unknown checkbox event {other}"),
        };

        let (next, effects) = checkbox_transition(context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                CheckboxEffect::RevertNativeChecked => json!({ "type": "revertNativeChecked" }),
                CheckboxEffect::EmitCheckedChange { checked } => {
                    json!({ "type": "emitCheckedChange", "checked": checked })
                }
            })
            .collect();

        assert_case(
            "checkbox",
            case,
            effects,
            None,
            Some(json!({ "checked": next.checked })),
        );
    }
}

#[test]
fn modal_conformance() {
    for case in vectors()["modal"].as_array().unwrap() {
        let ctx = &case["context"];
        let context = ModalContext {
            dismiss_on_escape: b(ctx, "dismissOnEscape"),
            dismiss_on_backdrop: b(ctx, "dismissOnBackdrop"),
        };
        let state = if s(case, "state") == "open" {
            ModalState::Open
        } else {
            ModalState::Closed
        };
        let event = match s(&case["event"], "type") {
            "OPEN" => ModalEvent::Open,
            "CLOSE" => ModalEvent::Close,
            "REQUEST_CLOSE" => ModalEvent::RequestClose,
            "ESCAPE" => ModalEvent::Escape,
            "BACKDROP_CLICK" => ModalEvent::BackdropClick,
            other => panic!("unknown modal event {other}"),
        };

        let (next_state, effects) = modal_transition(state, context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                ModalEffect::EmitOpenChange { open } => {
                    json!({ "type": "emitOpenChange", "open": open })
                }
                ModalEffect::EmitRequestClose => json!({ "type": "emitRequestClose" }),
                ModalEffect::SaveFocusAndEnter => json!({ "type": "saveFocusAndEnter" }),
                ModalEffect::LockBodyScroll => json!({ "type": "lockBodyScroll" }),
                ModalEffect::UnlockBodyScroll => json!({ "type": "unlockBodyScroll" }),
                ModalEffect::RestoreFocus => json!({ "type": "restoreFocus" }),
            })
            .collect();
        let state_name = if next_state == ModalState::Open {
            "open"
        } else {
            "closed"
        };

        assert_case("modal", case, effects, Some(state_name), None);
    }
}

#[test]
fn hover_conformance() {
    for case in vectors()["hover"].as_array().unwrap() {
        let ctx = &case["context"];
        let context = HoverContext {
            open_delay_ms: f(ctx, "openDelayMs"),
            close_delay_ms: f(ctx, "closeDelayMs"),
        };
        let state = match s(case, "state") {
            "opening" => HoverState::Opening,
            "open" => HoverState::Open,
            "closing" => HoverState::Closing,
            _ => HoverState::Closed,
        };
        let event = match s(&case["event"], "type") {
            "ENTER" => HoverEvent::Enter,
            "LEAVE" => HoverEvent::Leave,
            "TIMER_FIRE" => HoverEvent::TimerFire,
            "DISMISS" => HoverEvent::Dismiss,
            "SET_OPEN" => HoverEvent::SetOpen {
                open: b(&case["event"], "open"),
            },
            other => panic!("unknown hover event {other}"),
        };

        let (next_state, effects) = hover_transition(state, context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                HoverEffect::StartTimer { ms } => json!({ "type": "startTimer", "ms": ms }),
                HoverEffect::ClearTimer => json!({ "type": "clearTimer" }),
                HoverEffect::EmitOpenChange { open } => {
                    json!({ "type": "emitOpenChange", "open": open })
                }
            })
            .collect();
        let state_name = match next_state {
            HoverState::Closed => "closed",
            HoverState::Opening => "opening",
            HoverState::Open => "open",
            HoverState::Closing => "closing",
        };

        assert_case("hover", case, effects, Some(state_name), None);
    }
}

#[test]
fn single_select_conformance() {
    for case in vectors()["singleSelect"].as_array().unwrap() {
        let ctx = &case["context"];
        let context = SingleSelectContext {
            value: opt_string(ctx, "value"),
            options: options_from(ctx),
            disabled: b(ctx, "disabled"),
        };
        let event = match s(&case["event"], "type") {
            "SELECT" => SingleSelectEvent::Select {
                value: s(&case["event"], "value").to_string(),
            },
            "SET_VALUE" => SingleSelectEvent::SetValue {
                value: opt_string(&case["event"], "value"),
            },
            other => panic!("unknown singleSelect event {other}"),
        };

        let (next, effects) = single_select_transition(context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                SingleSelectEffect::EmitValueChange { value } => {
                    json!({ "type": "emitValueChange", "value": value })
                }
            })
            .collect();

        assert_case(
            "singleSelect",
            case,
            effects,
            None,
            Some(json!({ "value": next.value })),
        );
    }
}

#[test]
fn slider_conformance() {
    for case in vectors()["slider"].as_array().unwrap() {
        let ctx = &case["context"];
        let context = SliderContext {
            value: f(ctx, "value"),
            min: f(ctx, "min"),
            max: f(ctx, "max"),
            step: f(ctx, "step"),
            disabled: b(ctx, "disabled"),
        };
        let event = match s(&case["event"], "type") {
            "INPUT" => SliderEvent::Input {
                raw: f(&case["event"], "raw"),
            },
            "COMMIT" => SliderEvent::Commit {
                raw: f(&case["event"], "raw"),
            },
            "SET_VALUE" => SliderEvent::SetValue {
                value: f(&case["event"], "value"),
            },
            other => panic!("unknown slider event {other}"),
        };

        let (next, effects) = slider_transition(context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                SliderEffect::EmitValueChange { value } => {
                    json!({ "type": "emitValueChange", "value": value })
                }
                SliderEffect::EmitValueCommit { value } => {
                    json!({ "type": "emitValueCommit", "value": value })
                }
            })
            .collect();

        assert_case(
            "slider",
            case,
            effects,
            None,
            Some(json!({ "value": next.value })),
        );
    }
}

/// Shared step-quantization tie-law vectors: `sliderSnap` drives
/// `snap_to_step` directly so the portable half-toward-+infinity law stays
/// pinned even where the min clamp would mask a drifted tie law.
#[test]
fn slider_snap_conformance() {
    for case in vectors()["sliderSnap"].as_array().unwrap() {
        let ctx = &case["context"];
        let value = snap_to_step(f(&case["event"], "raw"), f(ctx, "min"), f(ctx, "step"));
        assert_case(
            "sliderSnap",
            case,
            Vec::new(),
            None,
            Some(json!({ "value": value })),
        );
    }
}

#[test]
fn range_slider_conformance() {
    for case in vectors()["rangeSlider"].as_array().unwrap() {
        let ctx = &case["context"];
        let pair = ctx["value"]
            .as_array()
            .map(|entries| {
                (
                    entries[0].as_f64().unwrap_or(0.0),
                    entries[1].as_f64().unwrap_or(0.0),
                )
            })
            .unwrap_or((0.0, 0.0));
        let context = RangeSliderContext {
            value: pair,
            min: f(ctx, "min"),
            max: f(ctx, "max"),
            step: f(ctx, "step"),
            disabled: b(ctx, "disabled"),
        };
        let event = match s(&case["event"], "type") {
            "INPUT" => RangeSliderEvent::Input {
                thumb: range_thumb(&case["event"]),
                raw: f(&case["event"], "raw"),
            },
            "COMMIT" => RangeSliderEvent::Commit {
                thumb: range_thumb(&case["event"]),
                raw: f(&case["event"], "raw"),
            },
            "SET_VALUE" => {
                let value = &case["event"]["value"];
                RangeSliderEvent::SetValue {
                    value: (
                        value[0].as_f64().unwrap_or(0.0),
                        value[1].as_f64().unwrap_or(0.0),
                    ),
                }
            }
            other => panic!("unknown range slider event {other}"),
        };

        let (next, effects) = range_slider_transition(context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                RangeSliderEffect::EmitValueChange { value } => json!({
                    "type": "emitValueChange",
                    "value": [value.0, value.1]
                }),
                RangeSliderEffect::EmitValueCommit { value } => json!({
                    "type": "emitValueCommit",
                    "value": [value.0, value.1]
                }),
            })
            .collect();

        assert_case(
            "rangeSlider",
            case,
            effects,
            None,
            Some(json!({ "value": [next.value.0, next.value.1] })),
        );
    }
}

fn range_thumb(event: &Value) -> RangeThumb {
    match s(event, "thumb") {
        "lower" => RangeThumb::Lower,
        "upper" => RangeThumb::Upper,
        other => panic!("unknown range slider thumb {other}"),
    }
}

#[test]
fn menu_conformance() {
    for case in vectors()["menu"].as_array().unwrap() {
        let context = MenuContext {
            disabled: b(&case["context"], "disabled"),
        };
        let state = if s(case, "state") == "open" {
            MenuState::Open
        } else {
            MenuState::Closed
        };
        let event = match s(&case["event"], "type") {
            "TOGGLE" => MenuEvent::Toggle,
            "OPEN" => MenuEvent::Open,
            "CLOSE" => MenuEvent::Close,
            "ESCAPE" => MenuEvent::Escape,
            "OUTSIDE_INTERACT" => MenuEvent::OutsideInteract,
            "ACTION" => MenuEvent::Action {
                value: s(&case["event"], "value").to_string(),
            },
            other => panic!("unknown menu event {other}"),
        };

        let (next_state, effects) = menu_transition(state, context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                MenuEffect::EmitOpenChange { open } => {
                    json!({ "type": "emitOpenChange", "open": open })
                }
                MenuEffect::EmitAction { value } => json!({ "type": "emitAction", "value": value }),
                MenuEffect::FocusFirstItem => json!({ "type": "focusFirstItem" }),
            })
            .collect();
        let state_name = if next_state == MenuState::Open {
            "open"
        } else {
            "closed"
        };

        assert_case("menu", case, effects, Some(state_name), None);
    }
}

#[test]
fn disclosure_conformance() {
    for case in vectors()["disclosure"].as_array().unwrap() {
        let ctx = &case["context"];
        let context = DisclosureContext {
            open: b(ctx, "open"),
            disabled: b(ctx, "disabled"),
        };
        let event = match s(&case["event"], "type") {
            "TOGGLE" => DisclosureEvent::Toggle,
            "SET_OPEN" => DisclosureEvent::SetOpen {
                open: b(&case["event"], "open"),
            },
            other => panic!("unknown disclosure event {other}"),
        };

        let (next, effects) = disclosure_transition(context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                DisclosureEffect::EmitOpenChange { open } => {
                    json!({ "type": "emitOpenChange", "open": open })
                }
            })
            .collect();

        assert_case(
            "disclosure",
            case,
            effects,
            None,
            Some(json!({ "open": next.open })),
        );
    }
}

#[test]
fn switch_conformance() {
    for case in vectors()["switch"].as_array().unwrap() {
        let ctx = &case["context"];
        let context = SwitchContext {
            checked: b(ctx, "checked"),
            disabled: b(ctx, "disabled"),
            read_only: b(ctx, "readOnly"),
        };
        let event = match s(&case["event"], "type") {
            "TOGGLE" => SwitchEvent::Toggle {
                next_checked: b(&case["event"], "nextChecked"),
            },
            "SET_CHECKED" => SwitchEvent::SetChecked {
                checked: b(&case["event"], "checked"),
            },
            other => panic!("unknown switch event {other}"),
        };

        let (next, effects) = switch_transition(context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                SwitchEffect::RevertNativeChecked => json!({ "type": "revertNativeChecked" }),
                SwitchEffect::EmitCheckedChange { checked } => {
                    json!({ "type": "emitCheckedChange", "checked": checked })
                }
            })
            .collect();

        assert_case(
            "switch",
            case,
            effects,
            None,
            Some(json!({ "checked": next.checked })),
        );
    }
}

fn toggle_value_from(value: &Value, mode: SelectionMode) -> ToggleGroupValue {
    match value {
        Value::Array(entries) => ToggleGroupValue::Multiple(
            entries
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect(),
        ),
        Value::String(single) => ToggleGroupValue::Single(Some(single.clone())),
        _ => match mode {
            SelectionMode::Multiple => ToggleGroupValue::Multiple(vec![]),
            SelectionMode::Single => ToggleGroupValue::Single(None),
        },
    }
}

fn toggle_value_to_json(value: &ToggleGroupValue) -> Value {
    match value {
        ToggleGroupValue::Single(Some(single)) => json!(single),
        ToggleGroupValue::Single(None) => Value::Null,
        ToggleGroupValue::Multiple(entries) => json!(entries),
    }
}

#[test]
fn toggle_group_conformance() {
    for case in vectors()["toggleGroup"].as_array().unwrap() {
        let ctx = &case["context"];
        let mode = if s(ctx, "selectionMode") == "multiple" {
            SelectionMode::Multiple
        } else {
            SelectionMode::Single
        };
        let context = ToggleGroupContext {
            value: toggle_value_from(&ctx["value"], mode),
            options: options_from(ctx),
            selection_mode: mode,
            allow_deactivation: b(ctx, "allowDeactivation"),
            disabled: b(ctx, "disabled"),
        };
        let event = match s(&case["event"], "type") {
            "TOGGLE" => ToggleGroupEvent::Toggle {
                value: s(&case["event"], "value").to_string(),
            },
            other => panic!("unknown toggleGroup event {other}"),
        };

        let (next, effects) = toggle_group_transition(context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                ToggleGroupEffect::EmitValueChange { value } => {
                    json!({ "type": "emitValueChange", "value": toggle_value_to_json(value) })
                }
            })
            .collect();

        assert_case(
            "toggleGroup",
            case,
            effects,
            None,
            Some(json!({ "value": toggle_value_to_json(&next.value) })),
        );
    }
}

#[test]
fn tabs_conformance() {
    for case in vectors()["tabs"].as_array().unwrap() {
        let ctx = &case["context"];
        let items: Vec<TabsItem> = ctx["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| TabsItem {
                value: s(item, "value").to_string(),
                disabled: b(item, "disabled"),
                closable: b(item, "closable"),
            })
            .collect();
        let context = TabsContext {
            items,
            value: opt_string(ctx, "value"),
            focus_index: f(ctx, "focusIndex") as usize,
            activation_mode: if s(ctx, "activationMode") == "manual" {
                ActivationMode::Manual
            } else {
                ActivationMode::Automatic
            },
            reorderable: b(ctx, "reorderable"),
        };
        let event_value = &case["event"];
        let from_index = event_value
            .get("fromIndex")
            .and_then(Value::as_u64)
            .map(|index| index as usize);
        let event = match s(event_value, "type") {
            "SELECT" => TabsEvent::Select {
                value: s(event_value, "value").to_string(),
            },
            "FOCUS_MOVE" => TabsEvent::FocusMove {
                direction: match s(event_value, "direction") {
                    "prev" => FocusDirection::Prev,
                    "first" => FocusDirection::First,
                    "last" => FocusDirection::Last,
                    _ => FocusDirection::Next,
                },
                from_index,
            },
            "ACTIVATE" => TabsEvent::Activate { index: from_index },
            "CLOSE" => TabsEvent::Close {
                value: s(event_value, "value").to_string(),
            },
            "REORDER_STEP" => TabsEvent::ReorderStep {
                direction: event_value
                    .get("direction")
                    .and_then(Value::as_i64)
                    .unwrap_or(1) as i32,
                from_index,
            },
            "REORDER" => TabsEvent::Reorder {
                from_index: f(event_value, "fromIndex") as usize,
                to_index: f(event_value, "toIndex") as usize,
            },
            other => panic!("unknown tabs event {other}"),
        };

        let (next, effects) = tabs_transition(context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                TabsEffect::EmitValueChange { value } => {
                    json!({ "type": "emitValueChange", "value": value })
                }
                TabsEffect::EmitReorder { order } => {
                    json!({ "type": "emitReorder", "order": order })
                }
                TabsEffect::EmitClose { value } => json!({ "type": "emitClose", "value": value }),
                TabsEffect::FocusTab { index } => json!({ "type": "focusTab", "index": index }),
            })
            .collect();

        let name = s(case, "name");

        if let Some(order) = case["expect"].get("order").and_then(Value::as_array) {
            let actual_order: Vec<Value> =
                next.items.iter().map(|item| json!(item.value)).collect();
            assert_eq!(&actual_order, order, "tabs/{name}: order");
        }

        assert_case(
            "tabs",
            case,
            effects,
            None,
            Some(json!({ "value": next.value, "focusIndex": next.focus_index })),
        );
    }
}

fn select_options_from(value: &Value) -> Vec<SelectOptionState> {
    value["options"]
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .map(|entry| SelectOptionState {
                    value: s(entry, "value").to_string(),
                    label: s(entry, "label").to_string(),
                    disabled: b(entry, "disabled"),
                })
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn select_conformance() {
    for case in vectors()["select"].as_array().unwrap() {
        let ctx = &case["context"];
        let context = SelectMachineContext {
            value: s(ctx, "value").to_string(),
            open: b(ctx, "open"),
            query: s(ctx, "query").to_string(),
            highlighted_value: opt_string(ctx, "highlightedValue"),
            options: select_options_from(ctx),
            clear_value: s(ctx, "clearValue").to_string(),
            searchable: b(ctx, "searchable"),
            freeform: b(ctx, "freeform"),
            disabled: b(ctx, "disabled"),
        };
        let event_value = &case["event"];
        let event = match s(event_value, "type") {
            "OPEN" => SelectMachineEvent::Open,
            "CLOSE" => SelectMachineEvent::Close,
            "TOGGLE" => SelectMachineEvent::Toggle,
            "QUERY" => SelectMachineEvent::Query {
                query: s(event_value, "query").to_string(),
            },
            "HIGHLIGHT" => SelectMachineEvent::Highlight {
                value: s(event_value, "value").to_string(),
            },
            "HIGHLIGHT_PREV" => SelectMachineEvent::HighlightPrev,
            "HIGHLIGHT_NEXT" => SelectMachineEvent::HighlightNext,
            "HIGHLIGHT_FIRST" => SelectMachineEvent::HighlightFirst,
            "HIGHLIGHT_LAST" => SelectMachineEvent::HighlightLast,
            "COMMIT_HIGHLIGHTED" => SelectMachineEvent::CommitHighlighted,
            "COMMIT_OPTION" => SelectMachineEvent::CommitOption {
                value: s(event_value, "value").to_string(),
            },
            "COMMIT_FREEFORM" => SelectMachineEvent::CommitFreeform,
            "CLEAR" => SelectMachineEvent::Clear,
            "OPTIONS_CHANGED" => SelectMachineEvent::OptionsChanged {
                options: select_options_from(event_value),
            },
            other => panic!("unknown select event {other}"),
        };

        let (next, effects) = select_transition(context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                SelectMachineEffect::OpenChanged { open } => {
                    json!({ "type": "openChanged", "open": open })
                }
                SelectMachineEffect::QueryChanged { query } => {
                    json!({ "type": "queryChanged", "query": query })
                }
                SelectMachineEffect::ValueChanged { value } => {
                    json!({ "type": "valueChanged", "value": value })
                }
            })
            .collect();

        assert_case(
            "select",
            case,
            effects,
            None,
            Some(json!({
                "value": next.value,
                "open": next.open,
                "query": next.query,
                "highlightedValue": next.highlighted_value,
            })),
        );
    }
}

// ---------------------------------------------------------------------------
// Drag-and-drop semantic kernel.
//
// The drag session is the one machine whose claims are about ordering across a
// whole lifecycle rather than a single transition, so its cases are step
// sequences. Every case starts at `Idle` with no session; each step asserts the
// resulting phase, the effects that step emitted in order, and — where the case
// pins it — a subset of the resulting session. The TypeScript core runs the
// same shape (packages/core/test/conformance.test.ts).
// ---------------------------------------------------------------------------

fn drag_operation(value: &Value) -> DragOperation {
    match value.as_str().expect("drag operation") {
        "move" => DragOperation::Move,
        "copy" => DragOperation::Copy,
        "link" => DragOperation::Link,
        other => panic!("unknown drag operation {other}"),
    }
}

fn drag_operation_json(operation: DragOperation) -> Value {
    json!(match operation {
        DragOperation::Move => "move",
        DragOperation::Copy => "copy",
        DragOperation::Link => "link",
    })
}

fn drag_subject(value: &Value) -> DragSubject {
    DragSubject {
        kind: s(value, "kind").to_string(),
        id: s(value, "id").to_string(),
    }
}

fn drag_subject_json(subject: &DragSubject) -> Value {
    json!({ "kind": subject.kind, "id": subject.id })
}

fn drop_intent(value: &Value) -> DropIntent {
    DropIntent {
        target_id: s(value, "targetId").to_string(),
        position: s(value, "position").to_string(),
        operation: drag_operation(&value["operation"]),
        destination: None,
    }
}

fn drop_intent_json(intent: &DropIntent) -> Value {
    json!({
        "targetId": intent.target_id,
        "position": intent.position,
        "operation": drag_operation_json(intent.operation),
    })
}

fn drag_cancel_reason(value: &Value) -> DragCancelReason {
    match value.as_str().expect("cancel reason") {
        "preparation-declined" => DragCancelReason::PreparationDeclined,
        "preparation-failed" => DragCancelReason::PreparationFailed,
        "superseded" => DragCancelReason::Superseded,
        "escape" => DragCancelReason::Escape,
        "explicit" => DragCancelReason::Explicit,
        "source-lost" => DragCancelReason::SourceLost,
        "target-lost" => DragCancelReason::TargetLost,
        "transport-lost" => DragCancelReason::TransportLost,
        "window-lost" => DragCancelReason::WindowLost,
        other => panic!("unknown drag cancel reason {other}"),
    }
}

/// The host-authoritative outcome a cross-window bridge reports, decoded from
/// the same JSON shape `drag_outcome_json` emits.
fn drag_terminal_outcome(value: &Value) -> DragTerminalOutcome {
    match s(value, "status") {
        "committed" => DragTerminalOutcome::Committed {
            intent: drop_intent(&value["intent"]),
        },
        "rejected" => DragTerminalOutcome::Rejected {
            reason: opt_string(value, "reason"),
        },
        "failed" => DragTerminalOutcome::Failed {
            reason: opt_string(value, "reason"),
        },
        "cancelled" => DragTerminalOutcome::Cancelled {
            reason: drag_cancel_reason(&value["reason"]),
        },
        other => panic!("unknown drag terminal status {other}"),
    }
}

fn drag_event(value: &Value) -> DragSessionEvent {
    let session_id = s(value, "sessionId").to_string();

    match s(value, "type") {
        "PREPARE" => DragSessionEvent::Prepare {
            session_id,
            source_id: s(value, "sourceId").to_string(),
            subject: drag_subject(&value["subject"]),
            operation: drag_operation(&value["operation"]),
            allowed_operations: value["allowedOperations"]
                .as_array()
                .expect("allowedOperations")
                .iter()
                .map(drag_operation)
                .collect(),
        },
        "PREPARED" => DragSessionEvent::Prepared { session_id },
        "PREPARE_DECLINED" => DragSessionEvent::PrepareDeclined { session_id },
        "PREPARE_FAILED" => DragSessionEvent::PrepareFailed { session_id },
        "ACTIVATE" => DragSessionEvent::Activate { session_id },
        "TARGET_INTENT" => DragSessionEvent::TargetIntent {
            session_id,
            intent: drop_intent(&value["intent"]),
        },
        "TARGET_CLEARED" => DragSessionEvent::TargetCleared { session_id },
        "OPERATION_CHANGED" => DragSessionEvent::OperationChanged {
            session_id,
            operation: drag_operation(&value["operation"]),
        },
        "DROP_REQUESTED" => DragSessionEvent::DropRequested { session_id },
        "DROP_COMMITTED" => DragSessionEvent::DropCommitted {
            session_id,
            intent: drop_intent(&value["intent"]),
        },
        "DROP_REJECTED" => DragSessionEvent::DropRejected {
            session_id,
            reason: opt_string(value, "reason"),
        },
        "DROP_FAILED" => DragSessionEvent::DropFailed {
            session_id,
            reason: opt_string(value, "reason"),
        },
        "ESCAPE" => DragSessionEvent::Escape { session_id },
        "CANCEL" => DragSessionEvent::Cancel { session_id },
        "SOURCE_LOST" => DragSessionEvent::SourceLost { session_id },
        "TARGET_LOST" => DragSessionEvent::TargetLost {
            session_id,
            target_id: s(value, "targetId").to_string(),
        },
        "TRANSPORT_LOST" => DragSessionEvent::TransportLost { session_id },
        "WINDOW_LOST" => DragSessionEvent::WindowLost { session_id },
        "HOST_TERMINAL" => DragSessionEvent::HostTerminal {
            session_id,
            outcome: drag_terminal_outcome(&value["outcome"]),
        },
        "RESET" => DragSessionEvent::Reset { session_id },
        other => panic!("unknown dragDrop event {other}"),
    }
}

fn drag_phase_json(phase: DragSessionPhase) -> Value {
    json!(match phase {
        DragSessionPhase::Idle => "idle",
        DragSessionPhase::Preparing => "preparing",
        DragSessionPhase::Armed => "armed",
        DragSessionPhase::Dragging => "dragging",
        DragSessionPhase::Dropping => "dropping",
        DragSessionPhase::Ended => "ended",
        DragSessionPhase::Cancelled => "cancelled",
    })
}

fn drag_cancel_reason_json(reason: DragCancelReason) -> Value {
    json!(match reason {
        DragCancelReason::PreparationDeclined => "preparation-declined",
        DragCancelReason::PreparationFailed => "preparation-failed",
        DragCancelReason::Superseded => "superseded",
        DragCancelReason::Escape => "escape",
        DragCancelReason::Explicit => "explicit",
        DragCancelReason::SourceLost => "source-lost",
        DragCancelReason::TargetLost => "target-lost",
        DragCancelReason::TransportLost => "transport-lost",
        DragCancelReason::WindowLost => "window-lost",
    })
}

fn drag_announcement_json(kind: DragAnnouncementKind) -> Value {
    json!(match kind {
        DragAnnouncementKind::Pickup => "pickup",
        DragAnnouncementKind::IntentChanged => "intentChanged",
        DragAnnouncementKind::IntentCleared => "intentCleared",
        DragAnnouncementKind::Dropped => "dropped",
        DragAnnouncementKind::Rejected => "rejected",
        DragAnnouncementKind::Failed => "failed",
        DragAnnouncementKind::Cancelled => "cancelled",
    })
}

/// An absent reason is an absent key, matching the TypeScript effect exactly.
fn drag_status_json(status: &str, reason: &Option<String>) -> Value {
    match reason {
        Some(reason) => json!({ "status": status, "reason": reason }),
        None => json!({ "status": status }),
    }
}

fn drag_outcome_json(outcome: &DragTerminalOutcome) -> Value {
    match outcome {
        DragTerminalOutcome::Committed { intent } => {
            json!({ "status": "committed", "intent": drop_intent_json(intent) })
        }
        DragTerminalOutcome::Rejected { reason } => drag_status_json("rejected", reason),
        DragTerminalOutcome::Failed { reason } => drag_status_json("failed", reason),
        DragTerminalOutcome::Cancelled { reason } => {
            json!({ "status": "cancelled", "reason": drag_cancel_reason_json(*reason) })
        }
    }
}

fn drag_effect_json(effect: &DragSessionEffect) -> Value {
    match effect {
        DragSessionEffect::PrepareSession {
            session_id,
            source_id,
            subject,
        } => json!({
            "type": "prepareSession",
            "sessionId": session_id,
            "sourceId": source_id,
            "subject": drag_subject_json(subject),
        }),
        DragSessionEffect::EmitDragStart {
            session_id,
            source_id,
            subject,
            operation,
        } => json!({
            "type": "emitDragStart",
            "sessionId": session_id,
            "sourceId": source_id,
            "subject": drag_subject_json(subject),
            "operation": drag_operation_json(*operation),
        }),
        DragSessionEffect::RequestDrop { session_id, intent } => json!({
            "type": "requestDrop",
            "sessionId": session_id,
            "intent": drop_intent_json(intent),
        }),
        DragSessionEffect::EmitDropResult {
            session_id,
            outcome,
        } => json!({
            "type": "emitDropResult",
            "sessionId": session_id,
            "outcome": drag_outcome_json(outcome),
        }),
        DragSessionEffect::Announce { kind } => {
            json!({ "type": "announce", "kind": drag_announcement_json(*kind) })
        }
        DragSessionEffect::ReturnFocus {
            session_id,
            subject,
        } => json!({
            "type": "returnFocus",
            "sessionId": session_id,
            "subject": drag_subject_json(subject),
        }),
        DragSessionEffect::CleanupSession { session_id } => {
            json!({ "type": "cleanupSession", "sessionId": session_id })
        }
    }
}

fn drag_session_json(session: &Option<DragSession>) -> Value {
    match session {
        None => Value::Null,
        Some(session) => json!({
            "sessionId": session.session_id,
            "sourceId": session.source_id,
            "subject": drag_subject_json(&session.subject),
            "operation": drag_operation_json(session.operation),
            "allowedOperations": session
                .allowed_operations
                .iter()
                .map(|operation| drag_operation_json(*operation))
                .collect::<Vec<Value>>(),
            "intent": session
                .intent
                .as_ref()
                .map_or(Value::Null, drop_intent_json),
        }),
    }
}

fn drop_target_candidate(value: &Value) -> DropTargetCandidate {
    let eligibility = &value["eligibility"];

    DropTargetCandidate {
        target_id: s(value, "targetId").to_string(),
        depth: f(value, "depth") as i32,
        order: f(value, "order") as i32,
        priority: value.get("priority").and_then(Value::as_f64).unwrap_or(0.0) as i32,
        contains_point: b(value, "containsPoint"),
        eligibility: if b(eligibility, "accepted") {
            DropEligibility::Accepted {
                intent: drop_intent(&eligibility["intent"]),
            }
        } else {
            DropEligibility::Rejected {
                reason: opt_string(eligibility, "reason"),
            }
        },
    }
}

#[test]
fn edit_conformance() {
    for case in vectors()["edit"].as_array().unwrap() {
        let ctx = &case["context"];
        let context = EditLabelContext {
            value: s(ctx, "value").to_string(),
            draft: s(ctx, "draft").to_string(),
            disabled: b(ctx, "disabled"),
            max_length: opt_usize(ctx, "maxLength"),
        };
        let state = match s(case, "state") {
            "editing" => EditLabelState::Editing,
            _ => EditLabelState::View,
        };
        let event = match s(&case["event"], "type") {
            "START_EDIT" => EditLabelEvent::StartEdit,
            "SET_DRAFT" => EditLabelEvent::SetDraft {
                draft: s(&case["event"], "draft").to_string(),
            },
            "COMMIT" => EditLabelEvent::Commit,
            "COMMIT_BLUR" => EditLabelEvent::CommitBlur,
            "CANCEL" => EditLabelEvent::Cancel,
            "REPLACE_VALUE" => EditLabelEvent::ReplaceValue {
                value: s(&case["event"], "value").to_string(),
            },
            "SET_DISABLED" => EditLabelEvent::SetDisabled {
                disabled: b(&case["event"], "disabled"),
            },
            "TEARDOWN" => EditLabelEvent::Teardown,
            other => panic!("unknown edit event {other}"),
        };

        let (next_state, next, effects) = edit_label_transition(state, context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                EditLabelEffect::EmitEditStart => json!({ "type": "emitEditStart" }),
                EditLabelEffect::FocusInput => json!({ "type": "focusInput" }),
                EditLabelEffect::EmitCommit {
                    value,
                    previous_value,
                    restore_focus,
                } => json!({
                    "type": "emitCommit",
                    "value": value,
                    "previousValue": previous_value,
                    "restoreFocus": restore_focus,
                }),
                EditLabelEffect::EmitCancel { restore_focus } => json!({
                    "type": "emitCancel",
                    "restoreFocus": restore_focus,
                }),
            })
            .collect();
        let state_name = match next_state {
            EditLabelState::View => "view",
            EditLabelState::Editing => "editing",
        };

        assert_case(
            "edit",
            case,
            effects,
            Some(state_name),
            Some(json!({
                "value": next.value,
                "draft": next.draft,
                "disabled": next.disabled,
                "maxLength": next.max_length,
            })),
        );
    }
}

#[test]
fn drag_drop_conformance() {
    let vectors = vectors();
    let drag_drop = &vectors["dragDrop"];

    for case in drag_drop["sessions"].as_array().unwrap() {
        let name = s(case, "name");
        let mut phase = DragSessionPhase::Idle;
        let mut context = DragSessionContext::default();

        for (index, step) in case["steps"].as_array().unwrap().iter().enumerate() {
            let event_type = s(&step["event"], "type");
            let (next_phase, next_context, effects) =
                drag_session_transition(phase, context, drag_event(&step["event"]));

            assert_eq!(
                drag_phase_json(next_phase),
                step["phase"],
                "dragDrop/{name} step {index} ({event_type}): phase"
            );

            let actual: Vec<Value> = effects.iter().map(drag_effect_json).collect();
            let expected: Vec<Value> = step["effects"].as_array().cloned().unwrap_or_default();
            assert_eq!(
                actual, expected,
                "dragDrop/{name} step {index} ({event_type}): effects"
            );

            if let Some(expected_session) = step.get("session") {
                let actual_session = drag_session_json(&next_context.session);

                if expected_session.is_null() {
                    assert_eq!(
                        actual_session,
                        Value::Null,
                        "dragDrop/{name} step {index} ({event_type}): session"
                    );
                } else {
                    for (key, value) in expected_session.as_object().expect("session object") {
                        assert_eq!(
                            &actual_session[key], value,
                            "dragDrop/{name} step {index} ({event_type}): session.{key}"
                        );
                    }
                }
            }

            phase = next_phase;
            context = next_context;
        }
    }

    for case in drag_drop["arbitration"].as_array().unwrap() {
        let name = s(case, "name");
        let candidates: Vec<DropTargetCandidate> = case["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .map(drop_target_candidate)
            .collect();
        let actual = resolve_drop_target(&candidates)
            .as_ref()
            .map_or(Value::Null, drop_intent_json);

        assert_eq!(actual, case["expect"]["intent"], "dragDrop/{name}: intent");
    }
}

// ---------------------------------------------------------------------------
// Continuous audio controls (g16.031)
//
// The continuous-audio controls make lifetime claims — one accepted begin, one
// terminal, rebase without a jump — so their cases are ordered step sequences
// over one context rather than single transitions. Every step pins the effects
// it emitted in order and, where the case claims it, a subset of the resulting
// context. The TypeScript mirror runs the same shape
// (packages/core/test/conformance.test.ts).
// ---------------------------------------------------------------------------

fn audio_law(value: Option<&Value>) -> AudioValueLaw {
    let Some(value) = value else {
        return AudioValueLaw::Linear;
    };
    match s(value, "type") {
        "linear" => AudioValueLaw::Linear,
        "logarithmic" => AudioValueLaw::Logarithmic,
        "exponential" => AudioValueLaw::Exponential {
            exponent: f(value, "exponent"),
        },
        "bipolar-center" => AudioValueLaw::BipolarCenter {
            center: f(value, "center"),
        },
        "stepped" => AudioValueLaw::Stepped {
            step: f(value, "step"),
            law: match audio_law(value.get("law")) {
                AudioValueLaw::Logarithmic => ContinuousAudioValueLaw::Logarithmic,
                AudioValueLaw::Exponential { exponent } => {
                    ContinuousAudioValueLaw::Exponential { exponent }
                }
                AudioValueLaw::BipolarCenter { center } => {
                    ContinuousAudioValueLaw::BipolarCenter { center }
                }
                _ => ContinuousAudioValueLaw::Linear,
            },
        },
        other => panic!("unknown audio law {other}"),
    }
}

fn audio_format(value: Option<&Value>) -> AudioValueFormat {
    let Some(value) = value else {
        return AudioValueFormat::Number { decimals: 2 };
    };
    let decimals = value
        .get("decimals")
        .and_then(Value::as_u64)
        .map(|places| places as usize);
    match s(value, "type") {
        "number" => AudioValueFormat::Number {
            decimals: decimals.unwrap_or(2),
        },
        "db" => AudioValueFormat::Db {
            decimals: decimals.unwrap_or(1),
        },
        "hz" => AudioValueFormat::Hz {
            decimals: decimals.unwrap_or(1),
        },
        "khz" => AudioValueFormat::Khz {
            decimals: decimals.unwrap_or(2),
        },
        "percent" => AudioValueFormat::Percent {
            decimals: decimals.unwrap_or(1),
        },
        "ratio" => AudioValueFormat::Ratio {
            decimals: decimals.unwrap_or(2),
        },
        "milliseconds" => AudioValueFormat::Milliseconds {
            decimals: decimals.unwrap_or(1),
        },
        "note" => AudioValueFormat::Note,
        "semitones" => AudioValueFormat::Semitones {
            decimals: decimals.unwrap_or(1),
        },
        other => panic!("unknown audio format {other}"),
    }
}

fn automation_state(name: &str) -> AutomationState {
    match name {
        "none" => AutomationState::None,
        "touched" => AutomationState::Touched,
        "latched" => AutomationState::Latched,
        "writing" => AutomationState::Writing,
        "read" => AutomationState::Read,
        other => panic!("unknown automation state {other}"),
    }
}

fn automation_state_json(state: AutomationState) -> Value {
    json!(match state {
        AutomationState::None => "none",
        AutomationState::Touched => "touched",
        AutomationState::Latched => "latched",
        AutomationState::Writing => "writing",
        AutomationState::Read => "read",
    })
}

fn drag_state(name: &str) -> DragState {
    match name {
        "none" => DragState::None,
        "coarse" => DragState::Coarse,
        "fine" => DragState::Fine,
        other => panic!("unknown drag state {other}"),
    }
}

fn drag_state_json(state: DragState) -> Value {
    json!(match state {
        DragState::None => "none",
        DragState::Coarse => "coarse",
        DragState::Fine => "fine",
    })
}

fn override_f(value: &Value, key: &str, current: f64) -> f64 {
    value.get(key).and_then(Value::as_f64).unwrap_or(current)
}

/// Case overrides applied over the control's default scalar context.
fn audio_value_context(value: &Value) -> AudioValueContext {
    let base = AudioValueContext::default();
    AudioValueContext {
        value: override_f(value, "value", base.value),
        min: override_f(value, "min", base.min),
        max: override_f(value, "max", base.max),
        law: value
            .get("law")
            .map_or(base.law, |law| audio_law(Some(law))),
        default_value: override_f(value, "defaultValue", base.default_value),
        keyboard_step: override_f(value, "keyboardStep", base.keyboard_step),
        format: value
            .get("format")
            .map_or(base.format, |format| audio_format(Some(format))),
        hover: b(value, "hover"),
        focus: b(value, "focus"),
        drag: value
            .get("drag")
            .and_then(Value::as_str)
            .map_or(base.drag, drag_state),
        automation: value
            .get("automation")
            .and_then(Value::as_str)
            .map_or(base.automation, automation_state),
        entry_open: b(value, "entryOpen"),
        drag_start_value: override_f(value, "dragStartValue", base.drag_start_value),
        drag_start_position: override_f(value, "dragStartPosition", base.drag_start_position),
        disabled: b(value, "disabled"),
    }
}

fn knob_context(value: &Value) -> KnobContext {
    let defaults = KnobContext::default();
    KnobContext {
        base: audio_value_context(value),
        drag_mode: match value.get("dragMode").and_then(Value::as_str) {
            Some("circular") => KnobDragMode::Circular,
            Some("vertical") | None => KnobDragMode::Vertical,
            Some(other) => panic!("unknown knob drag mode {other}"),
        },
        drag_sensitivity: override_f(value, "dragSensitivity", defaults.drag_sensitivity),
    }
}

fn fader_context(value: &Value) -> FaderContext {
    let defaults = FaderContext::default();
    FaderContext {
        base: audio_value_context(value),
        orientation: fader_orientation(value.get("orientation").and_then(Value::as_str)),
        detents: value
            .get("detents")
            .and_then(Value::as_array)
            .map(|entries| entries.iter().filter_map(Value::as_f64).collect())
            .unwrap_or(defaults.detents),
        detent_snap: override_f(value, "detentSnap", defaults.detent_snap),
    }
}

fn fader_orientation(name: Option<&str>) -> FaderOrientation {
    match name {
        Some("horizontal") => FaderOrientation::Horizontal,
        Some("vertical") | None => FaderOrientation::Vertical,
        Some(other) => panic!("unknown fader orientation {other}"),
    }
}

fn xy_pad_context(value: &Value) -> XYPadContext {
    let base = XYPadContext::default();
    XYPadContext {
        x: override_f(value, "x", base.x),
        y: override_f(value, "y", base.y),
        min_x: override_f(value, "minX", base.min_x),
        max_x: override_f(value, "maxX", base.max_x),
        min_y: override_f(value, "minY", base.min_y),
        max_y: override_f(value, "maxY", base.max_y),
        law_x: value
            .get("lawX")
            .map_or(base.law_x, |law| audio_law(Some(law))),
        law_y: value
            .get("lawY")
            .map_or(base.law_y, |law| audio_law(Some(law))),
        default_x: override_f(value, "defaultX", base.default_x),
        default_y: override_f(value, "defaultY", base.default_y),
        keyboard_step_x: override_f(value, "keyboardStepX", base.keyboard_step_x),
        keyboard_step_y: override_f(value, "keyboardStepY", base.keyboard_step_y),
        hover: b(value, "hover"),
        focus: b(value, "focus"),
        drag: value
            .get("drag")
            .and_then(Value::as_str)
            .map_or(base.drag, drag_state),
        automation: value
            .get("automation")
            .and_then(Value::as_str)
            .map_or(base.automation, automation_state),
        drag_start_x: override_f(value, "dragStartX", base.drag_start_x),
        drag_start_y: override_f(value, "dragStartY", base.drag_start_y),
        drag_start_norm_x: override_f(value, "dragStartNormX", base.drag_start_norm_x),
        drag_start_norm_y: override_f(value, "dragStartNormY", base.drag_start_norm_y),
        disabled: b(value, "disabled"),
    }
}

fn value_bound(name: &str) -> ValueBound {
    match name {
        "min" => ValueBound::Min,
        "max" => ValueBound::Max,
        other => panic!("unknown bound {other}"),
    }
}

fn xy_pad_axis(name: &str) -> XYPadAxis {
    match name {
        "x" => XYPadAxis::X,
        "y" => XYPadAxis::Y,
        other => panic!("unknown axis {other}"),
    }
}

fn audio_value_event(value: &Value) -> AudioValueEvent {
    let fine = b(value, "fine");
    match s(value, "type") {
        "HOVER" => AudioValueEvent::Hover {
            value: b(value, "value"),
        },
        "FOCUS" => AudioValueEvent::Focus {
            value: b(value, "value"),
        },
        "SET_AUTOMATION" => AudioValueEvent::SetAutomation {
            value: automation_state(s(value, "value")),
        },
        "SET_VALUE" => AudioValueEvent::SetValue {
            value: f(value, "value"),
        },
        "DRAG_BEGIN" => AudioValueEvent::DragBegin {
            position: f(value, "position"),
            fine,
        },
        "DRAG_MOVE" => AudioValueEvent::DragMove {
            position: f(value, "position"),
            fine,
        },
        "DRAG_SET_NORM" => AudioValueEvent::DragSetNorm {
            value_norm: f(value, "valueNorm"),
            fine,
        },
        "DRAG_END" => AudioValueEvent::DragEnd,
        "DRAG_CANCEL" => AudioValueEvent::DragCancel,
        "WHEEL" => AudioValueEvent::Wheel {
            direction: f(value, "direction") as i8,
            fine,
        },
        "RESET" => AudioValueEvent::Reset,
        "KEY_NUDGE" => AudioValueEvent::KeyNudge {
            direction: f(value, "direction") as i8,
            multiplier: value
                .get("multiplier")
                .and_then(Value::as_f64)
                .unwrap_or(1.0),
            fine,
        },
        "KEY_BOUND" => AudioValueEvent::KeyBound {
            bound: value_bound(s(value, "bound")),
        },
        "ENTRY_OPEN" => AudioValueEvent::EntryOpen,
        "ENTRY_CANCEL" => AudioValueEvent::EntryCancel,
        "ENTRY_COMMIT" => AudioValueEvent::EntryCommit {
            text: s(value, "text").to_string(),
        },
        other => panic!("unknown audio value event {other}"),
    }
}

fn xy_pad_event(value: &Value) -> XYPadEvent {
    let fine = b(value, "fine");
    match s(value, "type") {
        "SET_VALUES" => XYPadEvent::SetValues {
            x: f(value, "x"),
            y: f(value, "y"),
        },
        "HOVER" => XYPadEvent::Hover {
            value: b(value, "value"),
        },
        "FOCUS" => XYPadEvent::Focus {
            value: b(value, "value"),
        },
        "SET_AUTOMATION" => XYPadEvent::SetAutomation {
            value: automation_state(s(value, "value")),
        },
        "DRAG_BEGIN" => XYPadEvent::DragBegin {
            x_norm: f(value, "xNorm"),
            y_norm: f(value, "yNorm"),
            fine,
        },
        "DRAG_MOVE" => XYPadEvent::DragMove {
            x_norm: f(value, "xNorm"),
            y_norm: f(value, "yNorm"),
            fine,
        },
        "DRAG_END" => XYPadEvent::DragEnd,
        "DRAG_CANCEL" => XYPadEvent::DragCancel,
        "RESET" => XYPadEvent::Reset,
        "NUDGE" => XYPadEvent::Nudge {
            axis: xy_pad_axis(s(value, "axis")),
            direction: f(value, "direction") as i8,
            multiplier: value
                .get("multiplier")
                .and_then(Value::as_f64)
                .unwrap_or(1.0),
            fine,
        },
        "BOUND" => XYPadEvent::Bound {
            axis: xy_pad_axis(s(value, "axis")),
            bound: value_bound(s(value, "bound")),
        },
        other => panic!("unknown xy pad event {other}"),
    }
}

fn audio_value_effect_json(effect: &AudioValueEffect) -> Value {
    match effect {
        AudioValueEffect::ValueChange(value) => {
            json!({ "type": "emitValueChange", "value": value })
        }
        AudioValueEffect::ValueCommit(value) => {
            json!({ "type": "emitValueCommit", "value": value })
        }
        AudioValueEffect::GestureBegin => json!({ "type": "beginGesture" }),
        AudioValueEffect::GestureEnd => json!({ "type": "endGesture" }),
        AudioValueEffect::RequestEntryFocus => json!({ "type": "requestEntryFocus" }),
    }
}

fn xy_pad_effect_json(effect: &XYPadEffect) -> Value {
    match effect {
        XYPadEffect::ValueChange(x, y) => json!({ "type": "emitValueChange", "x": x, "y": y }),
        XYPadEffect::ValueCommit(x, y) => json!({ "type": "emitValueCommit", "x": x, "y": y }),
        XYPadEffect::GestureBegin => json!({ "type": "beginGesture" }),
        XYPadEffect::GestureEnd => json!({ "type": "endGesture" }),
    }
}

fn assert_audio_step(
    control: &str,
    name: &str,
    index: usize,
    step: &Value,
    actual_effects: Vec<Value>,
    actual_context: Value,
) {
    let event_type = s(&step["event"], "type");
    let expected: Vec<Value> = step["effects"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .iter()
        .map(canonicalize)
        .collect();
    let actual: Vec<Value> = actual_effects.iter().map(canonicalize).collect();
    assert_eq!(
        actual, expected,
        "audioControls/{control}/{name} step {index} ({event_type}): effects"
    );

    if let Some(expected_context) = step.get("context").and_then(Value::as_object) {
        for (key, value) in expected_context {
            assert_eq!(
                canonicalize(&actual_context[key]),
                canonicalize(value),
                "audioControls/{control}/{name} step {index} ({event_type}): context.{key}"
            );
        }
    }
}

fn scalar_context_json(context: &AudioValueContext) -> Value {
    json!({
        "value": context.value,
        "drag": drag_state_json(context.drag),
        "entryOpen": context.entry_open,
        "hover": context.hover,
        "focus": context.focus,
        "automation": automation_state_json(context.automation),
        "dragStartValue": context.drag_start_value,
        "dragStartPosition": context.drag_start_position,
    })
}

fn point_from(value: &Value) -> AudioPoint {
    AudioPoint {
        x: f(value, "x"),
        y: f(value, "y"),
    }
}

fn rect_from(value: &Value) -> AudioRect {
    AudioRect {
        left: f(value, "left"),
        top: f(value, "top"),
        width: f(value, "width"),
        height: f(value, "height"),
    }
}

#[test]
fn audio_controls_conformance() {
    let vectors = vectors();
    let audio = &vectors["audioControls"];

    for case in audio["knob"].as_array().unwrap() {
        let name = s(case, "name");
        let mut context = knob_context(&case["context"]);

        for (index, step) in case["steps"].as_array().unwrap().iter().enumerate() {
            let (next, effects) = knob_transition(context, audio_value_event(&step["event"]));
            assert_audio_step(
                "knob",
                name,
                index,
                step,
                effects.iter().map(audio_value_effect_json).collect(),
                scalar_context_json(&next.base),
            );
            context = next;
        }
    }

    for case in audio["fader"].as_array().unwrap() {
        let name = s(case, "name");
        let mut context = fader_context(&case["context"]);

        for (index, step) in case["steps"].as_array().unwrap().iter().enumerate() {
            let (next, effects) = fader_transition(context, audio_value_event(&step["event"]));
            assert_audio_step(
                "fader",
                name,
                index,
                step,
                effects.iter().map(audio_value_effect_json).collect(),
                scalar_context_json(&next.base),
            );
            context = next;
        }
    }

    for case in audio["xyPad"].as_array().unwrap() {
        let name = s(case, "name");
        let mut context = xy_pad_context(&case["context"]);

        for (index, step) in case["steps"].as_array().unwrap().iter().enumerate() {
            let (next, effects) = xy_pad_transition(context, xy_pad_event(&step["event"]));
            let actual_context = json!({
                "x": next.x,
                "y": next.y,
                "drag": drag_state_json(next.drag),
                "hover": next.hover,
                "focus": next.focus,
                "automation": automation_state_json(next.automation),
            });
            assert_audio_step(
                "xyPad",
                name,
                index,
                step,
                effects.iter().map(xy_pad_effect_json).collect(),
                actual_context,
            );
            context = next;
        }
    }

    let geometry = &audio["geometry"];

    for case in geometry["knob"].as_array().unwrap() {
        let name = s(case, "name");
        assert_eq!(
            knob_point_to_norm(point_from(&case["point"]), rect_from(&case["rect"])),
            f(case, "expect"),
            "audioControls/geometry/knob/{name}"
        );
    }

    for case in geometry["fader"].as_array().unwrap() {
        let name = s(case, "name");
        assert_eq!(
            fader_point_to_norm(
                point_from(&case["point"]),
                rect_from(&case["rect"]),
                fader_orientation(case.get("orientation").and_then(Value::as_str)),
            ),
            f(case, "expect"),
            "audioControls/geometry/fader/{name}"
        );
    }

    for case in geometry["xyPad"].as_array().unwrap() {
        let name = s(case, "name");
        let (x_norm, y_norm) =
            xy_pad_point_to_norm(point_from(&case["point"]), rect_from(&case["rect"]));
        assert_eq!(
            json!({ "xNorm": x_norm, "yNorm": y_norm }),
            canonicalize(&case["expect"]),
            "audioControls/geometry/xyPad/{name}"
        );
    }

    for case in geometry["hitTest"].as_array().unwrap() {
        let name = s(case, "name");
        let point = point_from(&case["point"]);
        let rect = rect_from(&case["rect"]);
        let hit = match s(case, "shape") {
            "circle" => hit_test_circle(point, rect),
            "rect" => hit_test_rect(point, rect),
            other => panic!("unknown hit-test shape {other}"),
        };
        assert_eq!(
            hit,
            b(case, "expect"),
            "audioControls/geometry/hitTest/{name}"
        );
    }
}
