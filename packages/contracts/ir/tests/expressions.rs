//! Bounded expression vocabulary tests (batch card 012).
//!
//! Per the card's Conformance Set: one test per conformance-set row (the
//! twelve real pilot derivations that must be expressible), at least one
//! test proving an arithmetic or call-shaped expression cannot be
//! constructed or does not type-check, JSON round-trip and ordering for
//! expressions, and negative tests for the expression finding kinds and the
//! slot contradiction rules.
//!
//! Fixtures are minimal and synthetic (card scope) — they model the
//! vocabulary against the pilot derivations, not real component definitions.
//!
//! Transcriptions (recorded in the batch log): `pressedControlled` becomes
//! `is_present(pressed)` (a bound controlled prop is present), and
//! `uncontrolledPressed` becomes the `defaultPressed` seed prop; the
//! `Boolean(slot)` calls of `BTN-17` become slot-presence operands
//! (`Expr::slot`), which is what the spec means by "a slot's presence".

use poodle_ir::{
    validate, A11yRole, Accessibility, AttributeForm, Axes, ComponentDefinition, ConformanceVector,
    ContractRef, ControlRule, ControlSize, ControlledState, EmissionPolicy, Event, EventKind,
    EventPayload, EventTiming, Expr, FindingKind, FiringPhase, Identifier, IrModel, Layer,
    NameRule, NameSource, NativeAttr, Part, PartKind, PayloadKind, PermittedSubset, Prop, PropType,
    RuntimeTarget, SharedEnumMember, SharedType, SizeAxis, SizeRole, SizeStep, StateAttribute,
    Value, VectorStep, VectorStepKind, VisualFieldKind, VisualState, VisualStateField,
    IR_SCHEMA_VERSION,
};

// ---------------------------------------------------------------------------
// The twelve conformance-set derivations
// ---------------------------------------------------------------------------

/// `disabled || loading` — Button `isUnavailable`, `CROSS-20` (row 1; `or`).
fn row1_is_unavailable() -> Expr {
    Expr::or(Expr::prop("disabled"), Expr::prop("loading"))
}

/// `pressed !== null || defaultPressed !== null` — Button `isToggle`,
/// `BTN-14` (row 2; `is_present`, `or`). The `!== null` checks are the
/// nullability group's `is_present`.
fn row2_is_toggle() -> Expr {
    Expr::or(
        Expr::is_present(Expr::prop("pressed")),
        Expr::is_present(Expr::prop("defaultPressed")),
    )
}

/// `!children` — Button `iconOnly`, `BTN-09` (row 3; slot presence, `not`).
/// The `children` part is the label slot (`BTN-16` "children = label").
fn row3_icon_only() -> Expr {
    Expr::not(Expr::slot("children"))
}

/// `Boolean(leading) || Boolean(leadingIcon) || loading` — Button
/// `hasLeading`, `BTN-17` (row 4; slot presence, `or`). The excluded
/// `Boolean(...)` calls are slot-presence operands.
fn row4_has_leading() -> Expr {
    Expr::or(
        Expr::or(Expr::slot("leading"), Expr::slot("leading-icon")),
        Expr::prop("loading"),
    )
}

/// `pressedControlled ? pressed === true : uncontrolledPressed` — Button
/// `currentPressed`, `CROSS-04` (row 5; `if`/`then`/`else`, `eq`).
fn row5_current_pressed() -> Expr {
    Expr::if_then_else(
        Expr::is_present(Expr::prop("pressed")),
        Expr::eq(Expr::prop("pressed"), Expr::boolean(true)),
        Expr::prop("defaultPressed"),
    )
}

/// `size ?? resolveSemanticControlSize(...)` — all three pilots, `CROSS-07`
/// (row 6; `coalesce`, axis-resolution operand). The second operand is the
/// resolved size axis value.
fn row6_size_fallback() -> Expr {
    Expr::coalesce(Expr::prop("size"), Expr::axis("size"))
}

/// `type === "search"` — TextInput `isSearch`, `TXT-08` (row 7; `eq`
/// against a shared-type member).
fn row7_is_search() -> Expr {
    Expr::eq(
        Expr::prop("type"),
        Expr::member("text-input-type", "search"),
    )
}

/// `type === "multiline" || (type === "text" && rows !== null && rows > 1)`
/// — TextInput `isMultiline`, `TXT-06` (row 8; `or`, `and`, `is_present`,
/// `gt`).
fn row8_is_multiline() -> Expr {
    Expr::or(
        Expr::eq(
            Expr::prop("type"),
            Expr::member("text-input-type", "multiline"),
        ),
        Expr::and(
            Expr::eq(Expr::prop("type"), Expr::member("text-input-type", "text")),
            Expr::and(
                Expr::is_present(Expr::prop("rows")),
                Expr::gt(Expr::prop("rows"), Expr::int(1)),
            ),
        ),
    )
}

/// `isSearch && showClearButton && !disabled && !readOnly &&
/// currentValue.length > 0` — TextInput `canClear`, `TXT-08` (row 9; `and`,
/// `not`, `is_empty`). `currentValue` is the declared `value` controlled
/// state (`TXT-02`); `length > 0` is `not(is_empty(...))`.
fn row9_can_clear() -> Expr {
    Expr::and(
        Expr::and(
            Expr::and(Expr::visual("isSearch"), Expr::prop("showClearButton")),
            Expr::and(
                Expr::not(Expr::prop("disabled")),
                Expr::not(Expr::prop("readOnly")),
            ),
        ),
        Expr::not(Expr::is_empty(Expr::state("value"))),
    )
}

/// `maxLength !== null && charCount > maxLength` — TextInput char-over,
/// `TXT-14` (row 10; `is_present`, `and`, `gt`).
fn row10_char_over() -> Expr {
    Expr::and(
        Expr::is_present(Expr::prop("maxLength")),
        Expr::gt(Expr::visual("charCount"), Expr::prop("maxLength")),
    )
}

/// `showValidationStatus && effectiveValidationState !== "none"` — TextInput
/// indicator, `TXT-12` (row 11; `and`, `ne`). `"none"` is a member of the
/// `validation-state` shared type, and `effectiveValidationState` is a
/// VisualState field of that type.
fn row11_validation_indicator() -> Expr {
    Expr::and(
        Expr::prop("showValidationStatus"),
        Expr::ne(
            Expr::visual("effectiveValidationState"),
            Expr::member("validation-state", "none"),
        ),
    )
}

/// `data-tone` omitted when default — Button `BTN-18` (row 12; attribute
/// emission condition). The omission is the emission condition
/// `tone != default`.
fn row12_data_tone_condition() -> Expr {
    Expr::ne(Expr::prop("tone"), Expr::member("control-tone", "default"))
}

/// The TextInput prop-default expression (spec 063 "prop default" slot;
/// `CROSS-02`): the value-change debounce defaults to the validation
/// debounce when set, else 0 (`TXT-11`, `TXT-12`).
fn text_input_debounce_default() -> Expr {
    Expr::coalesce(Expr::prop("validationDebounce"), Expr::int(0))
}

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

/// Builds a valid, synthetic model exercising every conformance-set row and
/// every sanctioned expression slot.
fn expression_fixture() -> IrModel {
    let shared_tone = SharedType {
        id: Identifier::new("control-tone"),
        name: "ControlTone".to_owned(),
        description: "Synthetic shared tone type (004-shared-control-types).".to_owned(),
        canonical_ref: ContractRef::new("docs/contracts/004-shared-control-types.md", Some("tone")),
        members: ["default", "danger", "warning", "success"]
            .into_iter()
            .map(|m| SharedEnumMember {
                id: Identifier::new(m),
                name: m.to_owned(),
                description: "Synthetic tone member.".to_owned(),
            })
            .collect(),
    };

    let shared_size = SharedType {
        id: Identifier::new("control-size"),
        name: "ControlSize".to_owned(),
        description: "Synthetic xs–xl size ladder type (CROSS-07).".to_owned(),
        canonical_ref: ContractRef::new("docs/contracts/components/button.md", Some("§7")),
        members: ["xs", "sm", "md", "lg", "xl"]
            .into_iter()
            .map(|m| SharedEnumMember {
                id: Identifier::new(m),
                name: m.to_owned(),
                description: "Synthetic size rung.".to_owned(),
            })
            .collect(),
    };

    let shared_type = SharedType {
        id: Identifier::new("text-input-type"),
        name: "TextInputType".to_owned(),
        description: "Synthetic input type modes (TXT-06).".to_owned(),
        canonical_ref: ContractRef::new("docs/contracts/components/text-input.md", Some("§3")),
        members: ["text", "multiline", "search", "slug"]
            .into_iter()
            .map(|m| SharedEnumMember {
                id: Identifier::new(m),
                name: m.to_owned(),
                description: "Synthetic input mode.".to_owned(),
            })
            .collect(),
    };

    let shared_validation = SharedType {
        id: Identifier::new("validation-state"),
        name: "ValidationState".to_owned(),
        description: "Synthetic validation state (TXT-12).".to_owned(),
        canonical_ref: ContractRef::new("docs/contracts/components/text-input.md", Some("§3")),
        members: ["none", "valid", "invalid", "pending"]
            .into_iter()
            .map(|m| SharedEnumMember {
                id: Identifier::new(m),
                name: m.to_owned(),
                description: "Synthetic validation status.".to_owned(),
            })
            .collect(),
    };

    let button = ComponentDefinition {
        id: Identifier::new("sample-button"),
        name: "SampleButton".to_owned(),
        layer: Layer::Foundation,
        contract: ContractRef::new("docs/contracts/components/button.md", Some("§3")),
        description: "Synthetic button exercising the expression vocabulary.".to_owned(),
        props: vec![
            Prop {
                id: Identifier::new("tone"),
                name: "tone".to_owned(),
                prop_type: PropType::Shared(Identifier::new("control-tone")),
                default: Some(Value::member("default")),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Tone (BTN-02).".to_owned(),
                permitted_subset: Some(PermittedSubset::new(
                    "control-tone",
                    ["default", "danger", "warning", "success"],
                )),
            },
            Prop {
                id: Identifier::new("loading"),
                name: "loading".to_owned(),
                prop_type: PropType::Bool,
                default: Some(Value::Bool(false)),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Loading state (BTN-08).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("disabled"),
                name: "disabled".to_owned(),
                prop_type: PropType::Bool,
                default: Some(Value::Bool(false)),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Disabled state (BTN-07).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("pressed"),
                name: "pressed".to_owned(),
                prop_type: PropType::Bool,
                default: None,
                default_expr: None,
                required: false,
                web_only: false,
                description: "Controlled toggle state; nullable (BTN-14).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("defaultPressed"),
                name: "defaultPressed".to_owned(),
                prop_type: PropType::Bool,
                default: Some(Value::Bool(false)),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Uncontrolled toggle seed (BTN-14).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("size"),
                name: "size".to_owned(),
                prop_type: PropType::Shared(Identifier::new("control-size")),
                default: None,
                default_expr: None,
                required: false,
                web_only: false,
                description: "Explicit size override; nullable (CROSS-07).".to_owned(),
                permitted_subset: Some(PermittedSubset::new(
                    "control-size",
                    ["xs", "sm", "md", "lg", "xl"],
                )),
            },
        ],
        controlled_state: vec![ControlledState {
            id: Identifier::new("pressed"),
            controlled: Identifier::new("pressed"),
            seed: Identifier::new("defaultPressed"),
            rule: ControlRule::DoNotMix,
            description: "Toggle pair (BTN-14, CROSS-04).".to_owned(),
        }],
        events: vec![Event {
            id: Identifier::new("activation"),
            name: "onClick".to_owned(),
            kind: EventKind::Activation,
            payload: None,
            timing: EventTiming {
                phase: FiringPhase::OnRelease,
                ..EventTiming::default()
            },
            description: "Activation (CROSS-05).".to_owned(),
        }],
        parts: vec![
            Part {
                id: Identifier::new("root"),
                name: "Root".to_owned(),
                parent: None,
                kind: PartKind::Static,
                description: "Synthetic root part (CROSS-12).".to_owned(),
            },
            Part {
                id: Identifier::new("children"),
                name: "Children".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::Static,
                description: "Label slot; presence is `children` (BTN-16).".to_owned(),
            },
            Part {
                id: Identifier::new("leading"),
                name: "Leading".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::Static,
                description: "Leading slot (BTN-17).".to_owned(),
            },
            Part {
                id: Identifier::new("leading-icon"),
                name: "Leading icon".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::Static,
                description: "Leading icon slot (BTN-09, BTN-17).".to_owned(),
            },
            Part {
                id: Identifier::new("spinner"),
                name: "Spinner".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::Conditional {
                    when: Identifier::new("loading"),
                    description: "Spinner shown when loading (BTN-08, BTN-17).".to_owned(),
                },
                description: "Conditional loading spinner (BTN-17).".to_owned(),
            },
        ],
        attributes: vec![
            StateAttribute {
                id: Identifier::new("data-loading"),
                name: "data-loading".to_owned(),
                form: AttributeForm::PresenceOnly,
                emission: EmissionPolicy::Always,
                source: Some(Identifier::new("loading")),
                condition: None,
                value: None,
                description: "Always emitted while loading (BTN-08, BTN-18).".to_owned(),
            },
            StateAttribute {
                id: Identifier::new("data-tone"),
                name: "data-tone".to_owned(),
                form: AttributeForm::Valued,
                emission: EmissionPolicy::OmitWhenDefault,
                source: Some(Identifier::new("tone")),
                condition: Some(row12_data_tone_condition()),
                value: None,
                description: "Omitted for the default tone; the omission is the emission \
                              condition (BTN-18)."
                    .to_owned(),
            },
            StateAttribute {
                id: Identifier::new("data-unavailable"),
                name: "data-unavailable".to_owned(),
                form: AttributeForm::PresenceOnly,
                emission: EmissionPolicy::Always,
                source: None,
                condition: Some(row1_is_unavailable()),
                value: None,
                description: "Emitted when disabled or loading (CROSS-20).".to_owned(),
            },
            StateAttribute {
                id: Identifier::new("data-pressed"),
                name: "data-pressed".to_owned(),
                form: AttributeForm::PresenceOnly,
                emission: EmissionPolicy::Always,
                source: None,
                condition: Some(row2_is_toggle()),
                value: None,
                description: "Emitted only when the button is a toggle (BTN-14, BTN-18)."
                    .to_owned(),
            },
            StateAttribute {
                id: Identifier::new("data-icon-only"),
                name: "data-icon-only".to_owned(),
                form: AttributeForm::PresenceOnly,
                emission: EmissionPolicy::Always,
                source: None,
                condition: Some(row3_icon_only()),
                value: None,
                description: "Emitted when the children slot is empty (BTN-09, BTN-18).".to_owned(),
            },
            StateAttribute {
                id: Identifier::new("data-has-leading"),
                name: "data-has-leading".to_owned(),
                form: AttributeForm::PresenceOnly,
                emission: EmissionPolicy::Always,
                source: None,
                condition: Some(row4_has_leading()),
                value: None,
                description: "Emitted when a leading slot or icon is present, or while loading \
                              (BTN-17, BTN-18)."
                    .to_owned(),
            },
            StateAttribute {
                id: Identifier::new("data-current-pressed"),
                name: "data-current-pressed".to_owned(),
                form: AttributeForm::Valued,
                emission: EmissionPolicy::Always,
                source: None,
                condition: None,
                value: Some(row5_current_pressed()),
                description: "Valued with the effective pressed state (CROSS-04).".to_owned(),
            },
        ],
        axes: Axes {
            size: Some(SizeAxis {
                explicit: None,
                size_role: SizeRole::Control,
                fallback: Some(row6_size_fallback()),
                ladder: vec![
                    SizeStep {
                        size: ControlSize::Sm,
                        metrics: Default::default(),
                        description: "Placeholder rung.".to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Md,
                        metrics: Default::default(),
                        description: "Placeholder rung.".to_owned(),
                    },
                ],
            }),
            density: None,
            orientation: None,
        },
        tokens: Vec::new(),
        recipe_hooks: Vec::new(),
        accessibility: Accessibility {
            role: A11yRole::Button,
            name_rule: NameRule::FromContent,
            name_source: Some(NameSource::Content),
            aria: Vec::new(),
            native: vec![NativeAttr {
                name: "disabled".to_owned(),
                description: "Native disabled attribute (BTN-07).".to_owned(),
            }],
            description: "Native button role (BTN-21).".to_owned(),
        },
        capabilities: Vec::new(),
        keyboard: Vec::new(),
        visual_state: vec![VisualState {
            id: Identifier::new("sample-button-state"),
            name: "SampleButtonVisualState".to_owned(),
            fields: vec![
                VisualStateField {
                    id: Identifier::new("enabled"),
                    name: "enabled".to_owned(),
                    kind: VisualFieldKind::Bool,
                    description: "Disabled/loading unification (CROSS-20).".to_owned(),
                },
                VisualStateField {
                    id: Identifier::new("loading"),
                    name: "loading".to_owned(),
                    kind: VisualFieldKind::Bool,
                    description: "Loading projection (BTN-19).".to_owned(),
                },
            ],
            description: "Synthetic projection shape (CROSS-14).".to_owned(),
        }],
        conformance: vec![Identifier::new("sample-vector")],
        extensions: Vec::new(),
    };

    let text_input = ComponentDefinition {
        id: Identifier::new("sample-text-input"),
        name: "SampleTextInput".to_owned(),
        layer: Layer::Foundation,
        contract: ContractRef::new("docs/contracts/components/text-input.md", Some("§3")),
        description: "Synthetic text input exercising the expression vocabulary.".to_owned(),
        props: vec![
            Prop {
                id: Identifier::new("value"),
                name: "value".to_owned(),
                prop_type: PropType::String,
                default: None,
                default_expr: None,
                required: false,
                web_only: false,
                description: "Controlled value; nullable (TXT-02).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("defaultValue"),
                name: "defaultValue".to_owned(),
                prop_type: PropType::String,
                default: Some(Value::string("")),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Uncontrolled seed (TXT-02).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("type"),
                name: "type".to_owned(),
                prop_type: PropType::Shared(Identifier::new("text-input-type")),
                default: Some(Value::member("text")),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Input type mode (TXT-06).".to_owned(),
                permitted_subset: Some(PermittedSubset::new(
                    "text-input-type",
                    ["text", "multiline", "search", "slug"],
                )),
            },
            Prop {
                id: Identifier::new("rows"),
                name: "rows".to_owned(),
                prop_type: PropType::Number,
                default: None,
                default_expr: None,
                required: false,
                web_only: false,
                description: "Multiline row count; nullable (TXT-06, TXT-07).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("maxLength"),
                name: "maxLength".to_owned(),
                prop_type: PropType::Number,
                default: None,
                default_expr: None,
                required: false,
                web_only: false,
                description: "Maximum length; nullable (TXT-14).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("showClearButton"),
                name: "showClearButton".to_owned(),
                prop_type: PropType::Bool,
                default: Some(Value::Bool(true)),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Clear-button visibility (TXT-08).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("showValidationStatus"),
                name: "showValidationStatus".to_owned(),
                prop_type: PropType::Bool,
                default: Some(Value::Bool(true)),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Validation indicator visibility (TXT-12).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("disabled"),
                name: "disabled".to_owned(),
                prop_type: PropType::Bool,
                default: Some(Value::Bool(false)),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Disabled state (TXT-05).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("readOnly"),
                name: "readOnly".to_owned(),
                prop_type: PropType::Bool,
                default: Some(Value::Bool(false)),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Read-only mode (TXT-05).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("validationDebounce"),
                name: "validationDebounce".to_owned(),
                prop_type: PropType::Number,
                default: Some(Value::number(300.0)),
                default_expr: None,
                required: false,
                web_only: false,
                description: "Validation debounce in milliseconds (TXT-12).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("debounce"),
                name: "debounce".to_owned(),
                prop_type: PropType::Number,
                default: None,
                default_expr: Some(text_input_debounce_default()),
                required: false,
                web_only: false,
                description: "Value-change debounce; defaults by expression (TXT-11).".to_owned(),
                permitted_subset: None,
            },
        ],
        controlled_state: vec![ControlledState {
            id: Identifier::new("value"),
            controlled: Identifier::new("value"),
            seed: Identifier::new("defaultValue"),
            rule: ControlRule::DoNotMix,
            description: "Controlled value pair (TXT-02, CROSS-04).".to_owned(),
        }],
        events: vec![Event {
            id: Identifier::new("value-change"),
            name: "onValueChange".to_owned(),
            kind: EventKind::ValueChange,
            payload: Some(EventPayload {
                name: "value".to_owned(),
                kind: PayloadKind::String,
            }),
            timing: EventTiming::default(),
            description: "Value change during interaction (TXT-13).".to_owned(),
        }],
        parts: vec![
            Part {
                id: Identifier::new("root"),
                name: "Root".to_owned(),
                parent: None,
                kind: PartKind::Static,
                description: "Synthetic root part (CROSS-12).".to_owned(),
            },
            Part {
                id: Identifier::new("field"),
                name: "Field".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::Static,
                description: "Field part (TXT-17).".to_owned(),
            },
            Part {
                id: Identifier::new("search-affordance"),
                name: "Search affordance".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::ConditionalExpr {
                    when: row7_is_search(),
                    description: "Automatic leading search icon in search mode (TXT-08, TXT-17)."
                        .to_owned(),
                },
                description: "Search affordance (TXT-17).".to_owned(),
            },
            Part {
                id: Identifier::new("input-control"),
                name: "Input control".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::ConditionalExpr {
                    when: row8_is_multiline(),
                    description: "Multiline textarea when the mode or row count demands it \
                                  (TXT-06, TXT-17)."
                        .to_owned(),
                },
                description: "Input control part (TXT-17).".to_owned(),
            },
            Part {
                id: Identifier::new("clear-button"),
                name: "Clear button".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::ConditionalExpr {
                    when: row9_can_clear(),
                    description: "Clear button gated by canClear (TXT-08, TXT-17).".to_owned(),
                },
                description: "Clear button part (TXT-17).".to_owned(),
            },
            Part {
                id: Identifier::new("validation-indicator"),
                name: "Validation indicator".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::ConditionalExpr {
                    when: row11_validation_indicator(),
                    description: "Indicator shown when status is visible and not none \
                                  (TXT-12, TXT-17)."
                        .to_owned(),
                },
                description: "Validation indicator part (TXT-17).".to_owned(),
            },
        ],
        attributes: vec![
            StateAttribute {
                id: Identifier::new("data-validation-state"),
                name: "data-validation-state".to_owned(),
                form: AttributeForm::Valued,
                emission: EmissionPolicy::Always,
                source: Some(Identifier::new("effectiveValidationState")),
                condition: None,
                value: None,
                description: "Sourced from the projection field (TXT-18).".to_owned(),
            },
            StateAttribute {
                id: Identifier::new("data-char-over"),
                name: "data-char-over".to_owned(),
                form: AttributeForm::PresenceOnly,
                emission: EmissionPolicy::Always,
                source: None,
                condition: Some(row10_char_over()),
                value: None,
                description: "Emitted when the character count exceeds maxLength (TXT-14)."
                    .to_owned(),
            },
            StateAttribute {
                id: Identifier::new("--poodle-text-input-control-padding-start"),
                name: "--poodle-text-input-control-padding-start".to_owned(),
                form: AttributeForm::Valued,
                emission: EmissionPolicy::Always,
                source: Some(Identifier::new("controlPaddingStart")),
                condition: None,
                value: None,
                description: "Adornment-padding reservation sourced from the projection field \
                              (TXT-16; the arithmetic is a projection concern, not an \
                              expression)."
                    .to_owned(),
            },
        ],
        axes: Axes::default(),
        tokens: Vec::new(),
        recipe_hooks: Vec::new(),
        accessibility: Accessibility {
            role: A11yRole::Textbox,
            name_rule: NameRule::NeverPlaceholder,
            name_source: None,
            aria: Vec::new(),
            native: vec![NativeAttr {
                name: "readonly".to_owned(),
                description: "Native readonly (TXT-05).".to_owned(),
            }],
            description: "Native textbox role (TXT-26).".to_owned(),
        },
        capabilities: Vec::new(),
        keyboard: Vec::new(),
        visual_state: vec![VisualState {
            id: Identifier::new("text-input-state"),
            name: "TextInputVisualState".to_owned(),
            fields: vec![
                VisualStateField {
                    id: Identifier::new("isSearch"),
                    name: "isSearch".to_owned(),
                    kind: VisualFieldKind::Bool,
                    description: "Search-mode projection (TXT-08).".to_owned(),
                },
                VisualStateField {
                    id: Identifier::new("charCount"),
                    name: "charCount".to_owned(),
                    kind: VisualFieldKind::Number,
                    description: "Character count projection (TXT-14).".to_owned(),
                },
                VisualStateField {
                    id: Identifier::new("effectiveValidationState"),
                    name: "effectiveValidationState".to_owned(),
                    kind: VisualFieldKind::Enum(Identifier::new("validation-state")),
                    description: "Effective validation state projection (TXT-12).".to_owned(),
                },
                VisualStateField {
                    id: Identifier::new("controlPaddingStart"),
                    name: "controlPaddingStart".to_owned(),
                    kind: VisualFieldKind::Number,
                    description: "Adornment-padding projection; the arithmetic lives here, not \
                                  in an expression (TXT-16)."
                        .to_owned(),
                },
            ],
            description: "Synthetic projection shape (CROSS-14).".to_owned(),
        }],
        conformance: Vec::new(),
        extensions: Vec::new(),
    };

    let vector = ConformanceVector {
        id: Identifier::new("sample-vector"),
        name: "sample".to_owned(),
        applies_to: vec![RuntimeTarget::Svelte, RuntimeTarget::React],
        steps: vec![
            VectorStep {
                id: Identifier::new("snap-to-step"),
                name: "Snap to step".to_owned(),
                kind: VectorStepKind::Guard,
                guard: None,
                description: "Step snapping anchored at min; a machine guard, not an expression \
                              (CROSS-19)."
                    .to_owned(),
            },
            VectorStep {
                id: Identifier::new("commit-on-release"),
                name: "Commit on release".to_owned(),
                kind: VectorStepKind::EffectIntent,
                guard: Some(Expr::not(Expr::boolean(false))),
                description: "Change during interaction, commit on release (RNG-11); the guard \
                              is a literal expression — vectors name no component state."
                    .to_owned(),
            },
        ],
        description: "Synthetic conformance vector with a literal guard (CROSS-18).".to_owned(),
    };

    IrModel {
        schema_version: IR_SCHEMA_VERSION,
        shared_types: vec![shared_tone, shared_size, shared_type, shared_validation],
        components: vec![button, text_input],
        conformance_vectors: vec![vector],
        scenes: Vec::new(),
        specimen_registry: None,
    }
}

fn kinds(findings: &[poodle_ir::Finding]) -> Vec<FindingKind> {
    findings.iter().map(|f| f.kind).collect()
}

// ---------------------------------------------------------------------------
// Conformance set — the twelve pilot derivations (one test per row)
// ---------------------------------------------------------------------------

#[test]
fn row_or_is_unavailable_expressible() {
    let model = expression_fixture();
    let findings = validate(&model);
    assert!(
        findings.is_empty(),
        "fixture must validate clean, got: {findings:#?}"
    );
    let attr = model.components[0]
        .attributes
        .iter()
        .find(|a| a.id.as_str() == "data-unavailable")
        .expect("data-unavailable attribute");
    assert_eq!(
        attr.condition.as_ref(),
        Some(&row1_is_unavailable()),
        "CROSS-20 isUnavailable must be the emission condition"
    );
}

#[test]
fn row_is_present_or_is_toggle_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let attr = model.components[0]
        .attributes
        .iter()
        .find(|a| a.id.as_str() == "data-pressed")
        .expect("data-pressed attribute");
    assert_eq!(
        attr.condition.as_ref(),
        Some(&row2_is_toggle()),
        "BTN-14 isToggle must be the emission condition"
    );
}

#[test]
fn row_not_slot_icon_only_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let attr = model.components[0]
        .attributes
        .iter()
        .find(|a| a.id.as_str() == "data-icon-only")
        .expect("data-icon-only attribute");
    assert_eq!(
        attr.condition.as_ref(),
        Some(&row3_icon_only()),
        "BTN-09 iconOnly must be the emission condition"
    );
}

#[test]
fn row_slot_or_has_leading_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let attr = model.components[0]
        .attributes
        .iter()
        .find(|a| a.id.as_str() == "data-has-leading")
        .expect("data-has-leading attribute");
    assert_eq!(
        attr.condition.as_ref(),
        Some(&row4_has_leading()),
        "BTN-17 hasLeading must be the emission condition"
    );
}

#[test]
fn row_if_else_current_pressed_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let attr = model.components[0]
        .attributes
        .iter()
        .find(|a| a.id.as_str() == "data-current-pressed")
        .expect("data-current-pressed attribute");
    assert_eq!(
        attr.value.as_ref(),
        Some(&row5_current_pressed()),
        "CROSS-04 currentPressed must be the valued attribute's expression"
    );
}

#[test]
fn row_coalesce_axis_fallback_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let fallback = model.components[0]
        .axes
        .size
        .as_ref()
        .expect("size axis")
        .fallback
        .as_ref()
        .expect("size axis fallback expression");
    assert_eq!(
        fallback,
        &row6_size_fallback(),
        "CROSS-07 must be the axis fallback expression"
    );
}

#[test]
fn row_eq_member_is_search_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let part = model.components[1]
        .parts
        .iter()
        .find(|p| p.id.as_str() == "search-affordance")
        .expect("search-affordance part");
    let PartKind::ConditionalExpr { when, .. } = &part.kind else {
        panic!("search-affordance must be a conditional-expr part");
    };
    assert_eq!(
        when,
        &row7_is_search(),
        "TXT-08 isSearch must be the render condition"
    );
}

#[test]
fn row_multiline_or_and_present_gt_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let part = model.components[1]
        .parts
        .iter()
        .find(|p| p.id.as_str() == "input-control")
        .expect("input-control part");
    let PartKind::ConditionalExpr { when, .. } = &part.kind else {
        panic!("input-control must be a conditional-expr part");
    };
    assert_eq!(
        when,
        &row8_is_multiline(),
        "TXT-06 isMultiline must be the render condition"
    );
}

#[test]
fn row_can_clear_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let part = model.components[1]
        .parts
        .iter()
        .find(|p| p.id.as_str() == "clear-button")
        .expect("clear-button part");
    let PartKind::ConditionalExpr { when, .. } = &part.kind else {
        panic!("clear-button must be a conditional-expr part");
    };
    assert_eq!(
        when,
        &row9_can_clear(),
        "TXT-08 canClear must be the render condition"
    );
}

#[test]
fn row_char_over_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let attr = model.components[1]
        .attributes
        .iter()
        .find(|a| a.id.as_str() == "data-char-over")
        .expect("data-char-over attribute");
    assert_eq!(
        attr.condition.as_ref(),
        Some(&row10_char_over()),
        "TXT-14 char-over must be the emission condition"
    );
}

#[test]
fn row_validation_indicator_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let part = model.components[1]
        .parts
        .iter()
        .find(|p| p.id.as_str() == "validation-indicator")
        .expect("validation-indicator part");
    let PartKind::ConditionalExpr { when, .. } = &part.kind else {
        panic!("validation-indicator must be a conditional-expr part");
    };
    assert_eq!(
        when,
        &row11_validation_indicator(),
        "TXT-12 must be the render condition"
    );
}

#[test]
fn row_data_tone_omit_when_default_expressible() {
    let model = expression_fixture();
    assert!(validate(&model).is_empty(), "fixture must validate clean");
    let attr = model.components[0]
        .attributes
        .iter()
        .find(|a| a.id.as_str() == "data-tone")
        .expect("data-tone attribute");
    assert_eq!(
        attr.condition.as_ref(),
        Some(&row12_data_tone_condition()),
        "BTN-18 omit-when-default must be the emission condition"
    );
}

// ---------------------------------------------------------------------------
// The four sanctioned slots beyond the conformance rows
// ---------------------------------------------------------------------------

#[test]
fn prop_default_expression_slot_type_checks() {
    let model = expression_fixture();
    let findings = validate(&model);
    assert!(
        findings.is_empty(),
        "fixture must validate clean, got: {findings:#?}"
    );
    let debounce = model.components[1]
        .props
        .iter()
        .find(|p| p.id.as_str() == "debounce")
        .expect("debounce prop");
    assert_eq!(
        debounce.default_expr.as_ref(),
        Some(&text_input_debounce_default()),
        "the prop default slot must carry the coalesce expression"
    );
}

#[test]
fn vector_guard_condition_slot_type_checks() {
    let model = expression_fixture();
    let findings = validate(&model);
    assert!(
        findings.is_empty(),
        "fixture must validate clean, got: {findings:#?}"
    );
    let step = &model.conformance_vectors[0].steps[1];
    assert_eq!(
        step.guard,
        Some(Expr::not(Expr::boolean(false))),
        "the effect-intent guard must be the literal expression"
    );
}

// ---------------------------------------------------------------------------
// Exclusions — arithmetic and calls are not part of the language
// ---------------------------------------------------------------------------

#[test]
fn excluded_arithmetic_and_call_operators_cannot_be_constructed() {
    // Arithmetic and function calls are excluded deliberately (spec 063
    // "Excluded, deliberately"; card 012 "Fixed By Ruling"). `Expr` has no
    // variant for them, so no expression can be constructed — and any JSON
    // smuggling one in fails to deserialize, keeping the boundary closed.
    let arithmetic = r#"{"add":{"left":{"operand":{"prop":"lowerNorm"}},"right":{"operand":{"literal":{"int":100}}}}}"#;
    let call = r#"{"call":{"name":"slugify","arguments":[]}}"#;
    let indexing =
        r#"{"index":{"of":{"operand":{"prop":"value"}},"at":{"operand":{"literal":{"int":0}}}}}"#;
    for smuggled in [arithmetic, call, indexing] {
        assert!(
            serde_json::from_str::<Expr>(smuggled).is_err(),
            "excluded operator JSON must not deserialize: {smuggled}"
        );
    }
}

#[test]
fn rejects_arithmetic_shaped_usage_at_type_check() {
    let mut model = expression_fixture();
    // `visualState.lowerNorm * 100` (RNG-17) and the TXT-16 adornment
    // arithmetic need operators that do not exist; the closest vocabulary
    // shapes reject non-integer and non-collection operands, so no
    // arithmetic-shaped expression type-checks either.
    let attr = model.components[0]
        .attributes
        .iter_mut()
        .find(|a| a.id.as_str() == "data-unavailable")
        .expect("data-unavailable attribute");
    attr.condition = Some(Expr::is_empty(Expr::int(3)));
    let findings = validate(&model);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::ExpressionTypeError
                && f.identifier == "sample-button.data-unavailable"
                && f.message.contains("is_empty")),
        "is_empty on a number must be a type error: {findings:#?}"
    );

    let mut model = expression_fixture();
    let attr = model.components[0]
        .attributes
        .iter_mut()
        .find(|a| a.id.as_str() == "data-unavailable")
        .expect("data-unavailable attribute");
    attr.condition = Some(Expr::gt(Expr::string("a"), Expr::int(1)));
    let findings = validate(&model);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::ExpressionTypeError
                && f.message.contains("integers only")),
        "ordering on a string must be a type error: {findings:#?}"
    );
}

#[test]
fn excluded_derivations_route_to_projection_fields_and_machines() {
    // RNG-17 (`visualState.lowerNorm * 100`), TXT-16 (adornment padding),
    // TXT-14 (`${charCount}/${maxLength}`), CROSS-19/RNG-02
    // (`safeSliderMax`, `normalizeRangeValue`), and TXT-09 (`slugify`) are
    // projection fields, conformance vectors, or machines — never
    // expressions (spec 063 escapes; card 012). The fixture proves the
    // projection-field home: the padding custom property is a valued
    // attribute sourced from a declared VisualState Number field, with no
    // expression and no arithmetic.
    let model = expression_fixture();
    let findings = validate(&model);
    assert!(
        findings.is_empty(),
        "fixture must validate clean, got: {findings:#?}"
    );
    let padding = model.components[1]
        .attributes
        .iter()
        .find(|a| a.id.as_str() == "--poodle-text-input-control-padding-start")
        .expect("padding attribute");
    assert_eq!(
        padding.value, None,
        "the padding derivation must not use an expression"
    );
    assert_eq!(
        padding.source.as_ref().map(|s| s.as_str()),
        Some("controlPaddingStart"),
        "the padding derivation must source the projection field"
    );
    assert!(
        model.components[1].visual_state[0].fields.iter().any(|f| {
            f.id.as_str() == "controlPaddingStart" && f.kind == VisualFieldKind::Number
        }),
        "controlPaddingStart must be a declared Number projection field"
    );
    // The degenerate-range guard stays a machine step, not a guard
    // expression (CROSS-19, RNG-02).
    assert_eq!(
        model.conformance_vectors[0].steps[0].guard, None,
        "the machine guard must not be an expression"
    );
}

// ---------------------------------------------------------------------------
// Expression type errors and unresolved references
// ---------------------------------------------------------------------------

#[test]
fn rejects_expression_type_errors() {
    let mut model = expression_fixture();
    let attr = model.components[0]
        .attributes
        .iter_mut()
        .find(|a| a.id.as_str() == "data-unavailable")
        .expect("data-unavailable attribute");
    attr.condition = Some(Expr::and(Expr::int(1), Expr::boolean(true)));

    let findings = validate(&model);
    let type_errors: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::ExpressionTypeError)
        .collect();
    assert!(
        !type_errors.is_empty(),
        "and with a non-boolean operand must be a type error: {findings:#?}"
    );
    assert!(
        type_errors
            .iter()
            .any(|f| f.identifier == "sample-button.data-unavailable"
                && f.message.contains("boolean")),
        "the finding must name the slot and the fix: {type_errors:#?}"
    );
}

#[test]
fn rejects_unresolved_expression_references() {
    let mut model = expression_fixture();
    let attr = model.components[0]
        .attributes
        .iter_mut()
        .find(|a| a.id.as_str() == "data-icon-only")
        .expect("data-icon-only attribute");
    attr.condition = Some(Expr::not(Expr::slot("missing-slot")));

    let findings = validate(&model);
    let unresolved: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::UnresolvedExpressionReference)
        .collect();
    assert!(
        !unresolved.is_empty(),
        "a slot operand outside the declared parts must be unresolved: {findings:#?}"
    );
    assert!(
        unresolved.iter().any(|f| {
            f.identifier == "sample-button.data-icon-only" && f.message.contains("missing-slot")
        }),
        "the finding must name the slot and the missing reference: {unresolved:#?}"
    );
}

#[test]
fn rejects_non_boolean_condition_slot() {
    let mut model = expression_fixture();
    let attr = model.components[0]
        .attributes
        .iter_mut()
        .find(|a| a.id.as_str() == "data-unavailable")
        .expect("data-unavailable attribute");
    attr.condition = Some(Expr::string("always"));

    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::ExpressionTypeError
                && f.identifier == "sample-button.data-unavailable"
                && f.message.contains("emission condition")
        }),
        "a string emission condition must be rejected: {findings:#?}"
    );
}

#[test]
fn rejects_non_boolean_part_condition() {
    let mut model = expression_fixture();
    let part = model.components[1]
        .parts
        .iter_mut()
        .find(|p| p.id.as_str() == "search-affordance")
        .expect("search-affordance part");
    let PartKind::ConditionalExpr { when, .. } = &mut part.kind else {
        panic!("search-affordance must be a conditional-expr part");
    };
    *when = Expr::int(1);

    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::ExpressionTypeError
                && f.identifier == "sample-text-input.search-affordance"
                && f.message.contains("render condition")
        }),
        "a numeric part render condition must be rejected: {findings:#?}"
    );
}

#[test]
fn rejects_eq_against_non_literal() {
    let mut model = expression_fixture();
    let attr = model.components[0]
        .attributes
        .iter_mut()
        .find(|a| a.id.as_str() == "data-current-pressed")
        .expect("data-current-pressed attribute");
    attr.value = Some(Expr::eq(
        Expr::prop("pressed"),
        Expr::prop("defaultPressed"),
    ));

    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::ExpressionTypeError && f.message.contains("literal")
        }),
        "eq against a non-literal must be rejected (spec 063: eq/ne compare against a literal \
         or shared-type member): {findings:#?}"
    );
}

#[test]
fn rejects_mismatched_coalesce_operands() {
    let mut model = expression_fixture();
    let axis = model.components[0].axes.size.as_mut().expect("size axis");
    axis.fallback = Some(Expr::coalesce(Expr::prop("size"), Expr::int(1)));

    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::ExpressionTypeError && f.message.contains("same type")
        }),
        "coalesce of a member and an integer must be rejected: {findings:#?}"
    );
}

#[test]
fn rejects_prop_default_type_mismatch() {
    let mut model = expression_fixture();
    let debounce = model.components[1]
        .props
        .iter_mut()
        .find(|p| p.id.as_str() == "debounce")
        .expect("debounce prop");
    debounce.default_expr = Some(Expr::string("fast"));

    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::ExpressionTypeError
                && f.identifier == "sample-text-input.debounce"
                && f.message.contains("declared type")
        }),
        "a string default expression on a Number prop must be rejected: {findings:#?}"
    );
}

#[test]
fn rejects_prop_with_default_and_default_expression() {
    let mut model = expression_fixture();
    let loading = model.components[0]
        .props
        .iter_mut()
        .find(|p| p.id.as_str() == "loading")
        .expect("loading prop");
    loading.default_expr = Some(Expr::boolean(true));

    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::ImpossibleBinding
                && f.identifier == "sample-button.loading"
                && f.message.contains("default")
        }),
        "a prop with both a default value and a default expression must be rejected: \
         {findings:#?}"
    );
}

#[test]
fn rejects_presence_only_attribute_with_value_expression() {
    let mut model = expression_fixture();
    let attr = model.components[0]
        .attributes
        .iter_mut()
        .find(|a| a.id.as_str() == "data-unavailable")
        .expect("data-unavailable attribute");
    attr.value = Some(Expr::boolean(true));

    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::ImpossibleBinding
                && f.identifier == "sample-button.data-unavailable"
                && f.message.contains("presence-only")
        }),
        "a presence-only attribute with a value expression must be rejected: {findings:#?}"
    );
}

#[test]
fn rejects_attribute_with_source_and_value_expression() {
    let mut model = expression_fixture();
    let attr = model.components[0]
        .attributes
        .iter_mut()
        .find(|a| a.id.as_str() == "data-tone")
        .expect("data-tone attribute");
    attr.value = Some(Expr::boolean(true));

    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::ImpossibleBinding
                && f.identifier == "sample-button.data-tone"
                && f.message.contains("source")
        }),
        "an attribute with both a source and a value expression must be rejected: {findings:#?}"
    );
}

#[test]
fn rejects_non_boolean_and_unresolvable_vector_guards() {
    let mut model = expression_fixture();
    model.conformance_vectors[0].steps[1].guard = Some(Expr::string("not-a-guard"));
    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::ExpressionTypeError
                && f.identifier == "sample-vector.commit-on-release"
        }),
        "a non-boolean guard must be rejected: {findings:#?}"
    );

    let mut model = expression_fixture();
    // Vectors are shared machine semantics and name no component state, so
    // a prop reference in a guard is unresolved (CROSS-19, RNG-02: machine
    // guards are vector machines, not expressions).
    model.conformance_vectors[0].steps[1].guard = Some(Expr::prop("loading"));
    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::UnresolvedExpressionReference
                && f.identifier == "sample-vector.commit-on-release"
        }),
        "a prop reference in a vector guard must be unresolved: {findings:#?}"
    );
}

// ---------------------------------------------------------------------------
// Findings arrive with all other findings at once
// ---------------------------------------------------------------------------

#[test]
fn expression_findings_arrive_with_all_other_findings() {
    let mut model = expression_fixture();
    // An expression type error plus an unrelated duplicate-id violation in
    // one model: both must be reported together.
    model.schema_version = 99;
    let attr = model.components[0]
        .attributes
        .iter_mut()
        .find(|a| a.id.as_str() == "data-unavailable")
        .expect("data-unavailable attribute");
    attr.condition = Some(Expr::and(Expr::int(1), Expr::boolean(true)));

    let findings = validate(&model);
    let all = kinds(&findings);
    assert!(
        all.contains(&FindingKind::SchemaVersion)
            && all.contains(&FindingKind::ExpressionTypeError),
        "expression findings must arrive alongside all other findings, got: {all:#?}"
    );
}

// ---------------------------------------------------------------------------
// Round-trip and ordering for expressions
// ---------------------------------------------------------------------------

#[test]
fn expression_json_round_trip_preserves_meaning_and_ordering() {
    let model = expression_fixture();
    let first = serde_json::to_string_pretty(&model).expect("serialize");
    let second = serde_json::to_string_pretty(&model).expect("serialize again");
    assert_eq!(first, second, "serialization must be deterministic (IR-07)");

    let parsed: IrModel = serde_json::from_str(&first).expect("deserialize");
    assert_eq!(parsed, model, "round trip must preserve meaning");

    // Operators serialize under their spec names with deterministic shapes.
    let or = serde_json::to_string(&row1_is_unavailable()).expect("serialize or");
    assert_eq!(
        or, r#"{"or":[{"operand":{"prop":"disabled"}},{"operand":{"prop":"loading"}}]}"#,
        "or must serialize under its spec name as a tagged pair"
    );
    let parsed_or: Expr = serde_json::from_str(&or).expect("deserialize or");
    assert_eq!(parsed_or, row1_is_unavailable());

    let if_expr = row5_current_pressed();
    let json = serde_json::to_string(&if_expr).expect("serialize if");
    assert!(
        json.starts_with(r#"{"if":{"condition":{"#),
        "if/then/else must serialize under the spec selection tag, got: {json}"
    );
    let parsed_if: Expr = serde_json::from_str(&json).expect("deserialize if");
    assert_eq!(parsed_if, if_expr);

    let member = row7_is_search();
    let json = serde_json::to_string(&member).expect("serialize member");
    assert!(
        json.contains(r#""member":{"shared_type":"text-input-type","member":"search"}"#),
        "a shared-type member literal must carry its shared type, got: {json}"
    );
    let parsed_member: Expr = serde_json::from_str(&json).expect("deserialize member");
    assert_eq!(parsed_member, member);
}
