//! Cross-runtime conformance: runs the shared vectors in
//! `vectors/machines.json` against the Rust machines. The TypeScript core
//! runs the same vectors (packages/core/test/conformance.test.ts).
//!
//! Effects are serialized back to the vector JSON shape and compared
//! order-sensitively.

use serde_json::{json, Value};

use poodle_headless::checkbox::*;
use poodle_headless::disclosure::*;
use poodle_headless::hover::*;
use poodle_headless::menu::*;
use poodle_headless::modal::*;
use poodle_headless::popover::*;
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
fn popover_conformance() {
    for case in vectors()["popover"].as_array().unwrap() {
        let ctx = &case["context"];
        let initial_focus = match s(ctx, "initialFocus") {
            "content" => PopoverInitialFocus::Content,
            "none" => PopoverInitialFocus::None,
            _ => PopoverInitialFocus::FirstFocusable,
        };
        let context = PopoverContext {
            disabled: b(ctx, "disabled"),
            dismiss_on_outside_interact: b(ctx, "dismissOnOutsideInteract"),
            initial_focus,
        };
        let state = if s(case, "state") == "open" {
            PopoverState::Open
        } else {
            PopoverState::Closed
        };
        let event = match s(&case["event"], "type") {
            "TOGGLE" => PopoverEvent::Toggle,
            "OPEN" => PopoverEvent::Open,
            "CLOSE" => PopoverEvent::Close,
            "ESCAPE" => PopoverEvent::Escape,
            "OUTSIDE_INTERACT" => PopoverEvent::OutsideInteract,
            other => panic!("unknown popover event {other}"),
        };

        let (next_state, effects) = popover_transition(state, context, event);
        let effects = effects
            .iter()
            .map(|effect| match effect {
                PopoverEffect::EmitOpenChange { open } => {
                    json!({ "type": "emitOpenChange", "open": open })
                }
                PopoverEffect::FocusOnOpen { strategy } => json!({
                    "type": "focusOnOpen",
                    "strategy": match strategy {
                        PopoverInitialFocus::FirstFocusable => "first-focusable",
                        PopoverInitialFocus::Content => "content",
                        PopoverInitialFocus::None => "none",
                    },
                }),
                PopoverEffect::RestoreTriggerFocus => json!({ "type": "restoreTriggerFocus" }),
            })
            .collect();
        let state_name = if next_state == PopoverState::Open {
            "open"
        } else {
            "closed"
        };

        assert_case("popover", case, effects, Some(state_name), None);
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
