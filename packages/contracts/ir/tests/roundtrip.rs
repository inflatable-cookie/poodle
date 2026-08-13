//! Round-trip and validation tests for the IR schema core.
//!
//! Per batch card 011: round-trip fixtures proving JSON preserves meaning and
//! ordering; one negative test per validation rule; and a test asserting a
//! value outside a component's permitted subset of a shared type is rejected.
//!
//! Fixtures are minimal and synthetic (card scope: "fixtures may be minimal
//! and synthetic") — they model the vocabulary, not real Button/RangeSlider/
//! TextInput definitions.

use std::collections::{BTreeMap, BTreeSet};

use poodle_ir::{
    validate, A11yRole, Accessibility, AriaMapping, AttributeForm, Axes, AxisValues, Capability,
    CapabilityProvision, CapabilityRequirement, CapabilityRuntimeStatus, ComponentDefinition,
    ComponentGroup, ComponentInstance, ConformanceVector, ContractRef, ControlDensity, ControlRule,
    ControlSize, ControlledState, DensityAdjustment, DensityAxis, EmissionPolicy, Event, EventKind,
    EventPayload, EventTiming, Extension, Finding, FindingKind, FiringPhase, GateTier, Identifier,
    IrModel, KeyChord, KeyboardCommand, Layer, MetricValue, Modifier, NameRule, NameSource,
    NativeAttr, NavSection, NavSectionKind, OrderingConstraint, Orientation, OrientationAxis,
    ParityHarness, Part, PartKind, PayloadKind, PermittedSubset, PreviewState, Prop, PropBinding,
    PropType, RecipeHookRef, RecipeLink, RecipeLinkKind, RouteState, RuntimeTarget, Scene,
    SceneAxis, SceneAxisKind, SceneLayout, SearchConfig, SearchField, SharedEnumMember, SharedType,
    SizeAxis, SizeRole, SizeStep, SpecimenEntry, SpecimenRegistry, SpecimenTabs, StateAttribute,
    TokenGroup, TokenRef, Value, VectorStep, VectorStepKind, VisualFieldKind, VisualGate,
    VisualState, VisualStateField, IR_SCHEMA_VERSION,
};

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

/// Builds a valid, synthetic model exercising every vocabulary module.
fn sample_model() -> IrModel {
    let shared_tone = SharedType {
        id: Identifier::new("control-tone"),
        name: "ControlTone".to_owned(),
        description: "Synthetic shared tone type; canonical four-member union modeled on \
                      docs/contracts/004-shared-control-types.md."
            .to_owned(),
        canonical_ref: ContractRef::new("docs/contracts/004-shared-control-types.md", Some("tone")),
        members: vec![
            SharedEnumMember {
                id: Identifier::new("default"),
                name: "Default".to_owned(),
                description: "Neutral tone.".to_owned(),
            },
            SharedEnumMember {
                id: Identifier::new("danger"),
                name: "Danger".to_owned(),
                description: "Destructive tone.".to_owned(),
            },
            SharedEnumMember {
                id: Identifier::new("warning"),
                name: "Warning".to_owned(),
                description: "Caution tone.".to_owned(),
            },
            SharedEnumMember {
                id: Identifier::new("success"),
                name: "Success".to_owned(),
                description: "Positive tone.".to_owned(),
            },
        ],
    };

    let shared_polarity = SharedType {
        id: Identifier::new("polarity"),
        name: "Polarity".to_owned(),
        description: "Synthetic polarity type for the two-segment fill geometry (RNG-04)."
            .to_owned(),
        canonical_ref: ContractRef::new("docs/contracts/components/range-slider.md", Some("§3")),
        members: vec![
            SharedEnumMember {
                id: Identifier::new("unipolar"),
                name: "Unipolar".to_owned(),
                description: "Single segment, no negative fill (RNG-24).".to_owned(),
            },
            SharedEnumMember {
                id: Identifier::new("bipolar"),
                name: "Bipolar".to_owned(),
                description: "Window split at the center reference (RNG-24).".to_owned(),
            },
        ],
    };

    let component = ComponentDefinition {
        id: Identifier::new("sample-button"),
        name: "SampleButton".to_owned(),
        layer: Layer::Foundation,
        contract: ContractRef::new("docs/contracts/components/button.md", Some("§3")),
        description: "Synthetic pilot component exercising the shared-type layer.".to_owned(),
        props: vec![
            Prop {
                id: Identifier::new("tone"),
                name: "tone".to_owned(),
                prop_type: PropType::Shared(Identifier::new("control-tone")),
                default: Some(Value::member("default")),
                required: false,
                web_only: false,
                description: "Tone; permitted subset of control-tone (BTN-02, R6.2).".to_owned(),
                permitted_subset: Some(PermittedSubset::new(
                    "control-tone",
                    ["default", "danger", "warning"],
                )),
            },
            Prop {
                id: Identifier::new("loading"),
                name: "loading".to_owned(),
                prop_type: PropType::Bool,
                default: Some(Value::Bool(false)),
                required: false,
                web_only: false,
                description: "Loading state; suppresses activation (BTN-08).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("label"),
                name: "label".to_owned(),
                prop_type: PropType::String,
                default: Some(Value::string("")),
                required: false,
                web_only: false,
                description: "Content label (BTN-16).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("pressed"),
                name: "pressed".to_owned(),
                prop_type: PropType::Bool,
                default: None,
                required: false,
                web_only: false,
                description: "Controlled toggle state (BTN-14).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("defaultPressed"),
                name: "defaultPressed".to_owned(),
                prop_type: PropType::Bool,
                default: Some(Value::Bool(false)),
                required: false,
                web_only: false,
                description: "Uncontrolled toggle seed (BTN-14).".to_owned(),
                permitted_subset: None,
            },
            Prop {
                id: Identifier::new("formaction"),
                name: "formaction".to_owned(),
                prop_type: PropType::String,
                default: None,
                required: false,
                web_only: true,
                description: "Web-only form-override family member (BTN-06, CROSS-03).".to_owned(),
                permitted_subset: None,
            },
        ],
        controlled_state: vec![ControlledState {
            id: Identifier::new("pressed"),
            controlled: Identifier::new("pressed"),
            seed: Identifier::new("defaultPressed"),
            rule: ControlRule::DoNotMix,
            description: "Toggle pair; binding both is rejected (BTN-14, CROSS-04).".to_owned(),
        }],
        events: vec![
            Event {
                id: Identifier::new("pressed-change"),
                name: "onPressedChange".to_owned(),
                kind: EventKind::PressedChange,
                payload: Some(EventPayload {
                    name: "pressed".to_owned(),
                    kind: PayloadKind::Bool,
                }),
                timing: EventTiming {
                    phase: FiringPhase::DuringInteraction,
                    debounce_ms: None,
                    flush_on_blur: false,
                    ordering: vec![OrderingConstraint {
                        before: Identifier::new("pressed-change"),
                        after: Identifier::new("activation"),
                        reason: "onPressedChange fires before onClick (B §5, CROSS-06).".to_owned(),
                    }],
                },
                description: "Toggle state change (BTN-14).".to_owned(),
            },
            Event {
                id: Identifier::new("activation"),
                name: "onClick".to_owned(),
                kind: EventKind::Activation,
                payload: None,
                timing: EventTiming {
                    phase: FiringPhase::OnRelease,
                    debounce_ms: None,
                    flush_on_blur: false,
                    ordering: Vec::new(),
                },
                description: "Activation (CROSS-05).".to_owned(),
            },
        ],
        parts: vec![
            Part {
                id: Identifier::new("root"),
                name: "Root".to_owned(),
                parent: None,
                kind: PartKind::Static,
                description: "Synthetic root part (CROSS-12).".to_owned(),
            },
            Part {
                id: Identifier::new("label"),
                name: "Label".to_owned(),
                parent: Some(Identifier::new("root")),
                kind: PartKind::Static,
                description: "Content label part (BTN-17).".to_owned(),
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
                description: "Always emitted while loading (BTN-08, BTN-18).".to_owned(),
            },
            StateAttribute {
                id: Identifier::new("data-tone"),
                name: "data-tone".to_owned(),
                form: AttributeForm::Valued,
                emission: EmissionPolicy::OmitWhenDefault,
                source: Some(Identifier::new("tone")),
                description: "Omitted for the default tone (BTN-18).".to_owned(),
            },
            StateAttribute {
                id: Identifier::new("--poodle-sample-fill-start"),
                name: "--poodle-sample-fill-start".to_owned(),
                form: AttributeForm::Valued,
                emission: EmissionPolicy::Always,
                source: Some(Identifier::new("enabled")),
                description: "Synthetic computed custom property sourced from a VisualState \
                              field, following the RNG-17 fill-geometry pattern."
                    .to_owned(),
            },
        ],
        axes: Axes {
            size: Some(SizeAxis {
                explicit: None,
                size_role: SizeRole::Control,
                ladder: vec![
                    SizeStep {
                        size: ControlSize::Sm,
                        metrics: BTreeMap::new(),
                        description: "Placeholder rung.".to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Md,
                        metrics: BTreeMap::new(),
                        description: "Placeholder rung.".to_owned(),
                    },
                ],
            }),
            density: Some(DensityAxis {
                explicit: None,
                adjustments: vec![DensityAdjustment {
                    density: ControlDensity::Compact,
                    applies_to: None,
                    inline: Some(MetricValue::Rem(-0.125)),
                    block: None,
                    description: "Compact inline padding delta (TXT-15).".to_owned(),
                }],
            }),
            orientation: Some(OrientationAxis {
                default: Orientation::Horizontal,
                values: vec![Orientation::Horizontal],
            }),
        },
        tokens: vec![TokenRef {
            path: "color.accent.base".to_owned(),
            group: TokenGroup::Semantic,
            description: "Synthetic accent fill reference (CROSS-09).".to_owned(),
        }],
        recipe_hooks: vec![RecipeHookRef {
            hook: "--poodle-recipe-sample-fill".to_owned(),
            chain: vec![
                RecipeLink {
                    kind: RecipeLinkKind::RecipeHook,
                    target: "--poodle-recipe-sample-fill".to_owned(),
                },
                RecipeLink {
                    kind: RecipeLinkKind::Token,
                    target: "color.accent.base".to_owned(),
                },
            ],
            description: "Synthetic fill hook chain (CROSS-09).".to_owned(),
        }],
        accessibility: Accessibility {
            role: A11yRole::Button,
            name_rule: NameRule::FromContent,
            name_source: Some(NameSource::Content),
            aria: vec![AriaMapping {
                aria_attr: "aria-pressed".to_owned(),
                source: Identifier::new("pressed"),
                description: "Toggle state mapped to aria-pressed (BTN-14).".to_owned(),
            }],
            native: vec![NativeAttr {
                name: "disabled".to_owned(),
                description: "Native disabled attribute (BTN-07).".to_owned(),
            }],
            description: "Native button role (BTN-21).".to_owned(),
        },
        capabilities: vec![
            CapabilityRequirement {
                capability: Capability::Focus,
                purpose: "Activation delivery is adapter-owned (BTN-20, CROSS-17).".to_owned(),
                // Pre-g13.018 shape: no per-runtime provision rows.
                runtimes: Vec::new(),
            },
            CapabilityRequirement {
                capability: Capability::Timers,
                purpose: "Timing for debounced effects (TXT-11, CROSS-17).".to_owned(),
                runtimes: Vec::new(),
            },
        ],
        keyboard: vec![KeyboardCommand {
            id: Identifier::new("activate"),
            keys: vec![
                KeyChord {
                    key: "Enter".to_owned(),
                    modifiers: BTreeSet::new(),
                },
                KeyChord {
                    key: " ".to_owned(),
                    modifiers: BTreeSet::new(),
                },
            ],
            action: "activate".to_owned(),
            effect: "emit activation".to_owned(),
            requires: Some(Capability::Focus),
            description: "Enter/Space activate (BTN-20).".to_owned(),
        }],
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
        extensions: vec![Extension {
            id: Identifier::new("jetstream-clear-only"),
            owning_runtime: RuntimeTarget::Jetstream,
            reason: "Host owns the editor; only pointer-reachable parts wire events (TXT-31)."
                .to_owned(),
            parity_effect: "Typing and key events have no route; value feeds back through the \
                           spec (TXT-31)."
                .to_owned(),
            evidence_surface: "docs/parity/text-input.md".to_owned(),
            removal_condition: "Removed when the Jetstream host editor gains typed event routes."
                .to_owned(),
            description: "Synthetic extension recording the EXT class (spec 063 escape-hatch \
                          rules)."
                .to_owned(),
        }],
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
                description: "Step snapping anchored at min (CROSS-19).".to_owned(),
            },
            VectorStep {
                id: Identifier::new("commit-on-release"),
                name: "Commit on release".to_owned(),
                kind: VectorStepKind::EffectIntent,
                description: "Change during interaction, commit on release (RNG-11).".to_owned(),
            },
        ],
        description: "Synthetic conformance vector (CROSS-18).".to_owned(),
    };

    let scene = Scene {
        id: Identifier::new("sample-scene"),
        name: "Sample scene".to_owned(),
        description: "Synthetic specimen scene (CROSS-21).".to_owned(),
        instances: vec![ComponentInstance {
            component: Identifier::new("sample-button"),
            bindings: vec![
                PropBinding {
                    prop: Identifier::new("tone"),
                    value: Value::member("danger"),
                    description: Some("Valid subset member.".to_owned()),
                },
                PropBinding {
                    prop: Identifier::new("loading"),
                    value: Value::Bool(false),
                    description: None,
                },
            ],
            caption: Some("Danger tone".to_owned()),
        }],
        axes: vec![
            SceneAxis {
                kind: SceneAxisKind::Theme,
                values: AxisValues::Named(
                    ["clay", "cobalt"]
                        .into_iter()
                        .map(Identifier::new)
                        .collect(),
                ),
                description: "Theme presets (SHELL-01).".to_owned(),
            },
            SceneAxis {
                kind: SceneAxisKind::Size,
                values: AxisValues::Named(
                    ["sm", "md", "lg"]
                        .into_iter()
                        .map(Identifier::new)
                        .collect(),
                ),
                description: "Control sizes (SHELL-02).".to_owned(),
            },
            SceneAxis {
                kind: SceneAxisKind::Density,
                values: AxisValues::Named(
                    ["compact", "default", "comfortable"]
                        .into_iter()
                        .map(Identifier::new)
                        .collect(),
                ),
                description: "Densities (SHELL-03).".to_owned(),
            },
            SceneAxis {
                kind: SceneAxisKind::Contrast,
                values: AxisValues::Continuous {
                    min: 0.4,
                    max: 1.6,
                    default: 1.0,
                },
                description: "Neutral-contrast override (CROSS-10, SHELL-04).".to_owned(),
            },
        ],
        layout: Some(SceneLayout {
            sections: vec![NavSection {
                title: "Components".to_owned(),
                kind: NavSectionKind::Components,
                groups: vec![ComponentGroup {
                    title: "Controls".to_owned(),
                    components: vec![Identifier::new("sample-button")],
                }],
            }],
            route_state: RouteState {
                persisted: vec!["hash".to_owned(), "query".to_owned()],
            },
        }),
        tabs: Some(SpecimenTabs {
            tabs: vec![
                Identifier::new("examples"),
                Identifier::new("sizes"),
                Identifier::new("densities"),
            ],
        }),
        search: Some(SearchConfig {
            case_insensitive: true,
            fields: vec![SearchField::DisplayName, SearchField::Description],
        }),
        preview_state: Some(PreviewState {
            theme: Some(Identifier::new("clay")),
            density: Some(Identifier::new("default")),
            control_size: Some(Identifier::new("md")),
            contrast: Some(1.0),
        }),
        parity: Some(ParityHarness {
            defaults: PreviewState {
                theme: Some(Identifier::new("clay")),
                density: Some(Identifier::new("default")),
                control_size: Some(Identifier::new("md")),
                contrast: Some(1.0),
            },
            review_route_presets: vec![Identifier::new("review-default")],
            targets: vec![RuntimeTarget::Svelte, RuntimeTarget::React],
            visual_gates: vec![
                VisualGate {
                    tier: GateTier::Smoke,
                    axes: vec![SceneAxisKind::Size, SceneAxisKind::Density],
                },
                VisualGate {
                    tier: GateTier::Axis,
                    axes: vec![SceneAxisKind::Theme, SceneAxisKind::Contrast],
                },
                VisualGate {
                    tier: GateTier::Sweep,
                    axes: Vec::new(),
                },
            ],
            native_visual_baseline: true,
        }),
        captures: vec![Identifier::new("sample-button-default")],
    };

    IrModel {
        schema_version: IR_SCHEMA_VERSION,
        shared_types: vec![shared_tone, shared_polarity],
        components: vec![component],
        conformance_vectors: vec![vector],
        scenes: vec![scene],
        specimen_registry: Some(SpecimenRegistry {
            entries: vec![SpecimenEntry {
                id: Identifier::new("sample-button"),
                component: Identifier::new("sample-button"),
                scenes: vec![Identifier::new("sample-scene")],
            }],
        }),
    }
}

fn kinds(findings: &[Finding]) -> Vec<FindingKind> {
    findings.iter().map(|f| f.kind).collect()
}

// ---------------------------------------------------------------------------
// Round-trip and ordering
// ---------------------------------------------------------------------------

#[test]
fn valid_model_has_no_findings() {
    let findings = validate(&sample_model());
    assert!(
        findings.is_empty(),
        "valid synthetic model must pass validation, got: {findings:#?}"
    );
}

#[test]
fn json_round_trip_preserves_meaning_and_ordering() {
    let model = sample_model();

    let first = serde_json::to_string_pretty(&model).expect("serialize");
    let second = serde_json::to_string_pretty(&model).expect("serialize again");

    // Deterministic output: same model serializes to the same bytes (IR-07).
    assert_eq!(first, second, "serialization must be deterministic");

    let parsed: IrModel = serde_json::from_str(&first).expect("deserialize");
    assert_eq!(parsed, model, "round trip must preserve meaning");

    // Schema version is present and round-trips.
    assert_eq!(parsed.schema_version, IR_SCHEMA_VERSION);
    assert!(
        first.starts_with(&format!(
            "{{\n  \"schema_version\": {IR_SCHEMA_VERSION},\n  \"shared_types\":"
        )) || first.starts_with(&format!(
            "{{\n  \"schema_version\": {},\n  \"shared_types\":",
            IR_SCHEMA_VERSION
        )),
        "schema_version must be the first serialized key, got: {}",
        first.lines().take(4).collect::<Vec<_>>().join("\n")
    );

    // Collection ordering is deterministic: the permitted subset is a
    // BTreeSet and must serialize sorted, never authoring order.
    let compact = serde_json::to_string(&model).expect("compact serialize");
    assert!(
        compact.contains("\"members\":[\"danger\",\"default\",\"warning\"]"),
        "permitted subset must serialize sorted, got: {compact}"
    );
}

// ---------------------------------------------------------------------------
// One negative test per validation rule
// ---------------------------------------------------------------------------

#[test]
fn rejects_duplicate_ids() {
    let mut model = sample_model();
    let mut duplicate = model.shared_types[0].clone();
    duplicate.id = Identifier::new("control-tone");
    model.shared_types.push(duplicate);

    let findings = validate(&model);
    let duplicate_findings: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::DuplicateId)
        .collect();
    assert!(
        !duplicate_findings.is_empty(),
        "expected a DuplicateId finding"
    );
    assert!(
        duplicate_findings
            .iter()
            .any(|f| f.identifier == "control-tone" && f.message.contains("duplicate")),
        "finding must carry the offending identifier and an actionable message: {duplicate_findings:#?}"
    );
}

#[test]
fn rejects_invalid_references() {
    let mut model = sample_model();
    model.components[0].props[0].prop_type =
        PropType::Shared(Identifier::new("missing-shared-type"));

    let findings = validate(&model);
    let invalid: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::InvalidReference)
        .collect();
    assert!(!invalid.is_empty(), "expected an InvalidReference finding");
    assert!(
        invalid
            .iter()
            .any(|f| f.identifier.contains("tone") && f.message.contains("missing-shared-type")),
        "finding must name the offending prop and the missing reference: {invalid:#?}"
    );
}

#[test]
fn rejects_impossible_prop_bindings() {
    let mut model = sample_model();
    model.scenes[0].instances[0].bindings.push(PropBinding {
        prop: Identifier::new("no-such-prop"),
        value: Value::Bool(true),
        description: None,
    });

    let findings = validate(&model);
    let impossible: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::ImpossibleBinding)
        .collect();
    assert!(
        !impossible.is_empty(),
        "expected an ImpossibleBinding finding"
    );
    assert!(
        impossible
            .iter()
            .any(|f| f.identifier.contains("no-such-prop")),
        "finding must name the offending binding: {impossible:#?}"
    );
}

#[test]
fn rejects_unsupported_parent_cycles() {
    let mut model = sample_model();
    // label -> root, spinner -> root. Make label and spinner point at each
    // other through root: root.parent = spinner creates the cycle
    // root -> spinner -> root.
    let root = model.components[0]
        .parts
        .iter_mut()
        .find(|p| p.id.as_str() == "root")
        .expect("root part");
    root.parent = Some(Identifier::new("spinner"));

    let findings = validate(&model);
    let cycles: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::Cycle)
        .collect();
    assert!(!cycles.is_empty(), "expected a Cycle finding");
    assert!(
        cycles.iter().any(|f| f.message.contains("cycle")),
        "finding must explain the cycle: {cycles:#?}"
    );
}

#[test]
fn rejects_missing_accessibility_data() {
    let mut model = sample_model();
    model.components[0].accessibility.name_rule = NameRule::Required;
    model.components[0].accessibility.name_source = None;

    let findings = validate(&model);
    let missing: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::MissingAccessibility)
        .collect();
    assert!(
        !missing.is_empty(),
        "expected a MissingAccessibility finding"
    );
    assert!(
        missing.iter().any(|f| f.identifier == "sample-button"),
        "finding must name the offending component: {missing:#?}"
    );
}

#[test]
fn rejects_undeclared_capabilities() {
    let mut model = sample_model();
    // Declared: Focus, Timers. A command requiring Clipboard is undeclared.
    model.components[0].keyboard.push(KeyboardCommand {
        id: Identifier::new("paste"),
        keys: vec![KeyChord {
            key: "v".to_owned(),
            modifiers: BTreeSet::from([Modifier::Meta]),
        }],
        action: "paste".to_owned(),
        effect: "platform clipboard paste".to_owned(),
        requires: Some(Capability::Clipboard),
        description: "Synthetic clipboard command (TXT-23).".to_owned(),
    });

    let findings = validate(&model);
    let undeclared: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::UndeclaredCapability)
        .collect();
    assert!(
        !undeclared.is_empty(),
        "expected an UndeclaredCapability finding"
    );
    assert!(
        undeclared.iter().any(|f| f.identifier.contains("paste")),
        "finding must name the offending command: {undeclared:#?}"
    );
}

#[test]
fn rejects_schema_version_mismatch() {
    let mut model = sample_model();
    model.schema_version = IR_SCHEMA_VERSION + 7;

    let findings = validate(&model);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::SchemaVersion),
        "expected a SchemaVersion finding"
    );
}

#[test]
fn rejects_unresolvable_token_reference() {
    let mut model = sample_model();
    model.components[0].tokens[0].path = "color.does.not.exist".to_owned();

    let findings = validate(&model);
    assert!(
        findings.iter().any(|f| {
            f.kind == FindingKind::InvalidReference && f.message.contains("color.does.not.exist")
        }),
        "expected an InvalidReference finding naming the token path: {findings:#?}"
    );
}

#[test]
fn rejects_do_not_mix_controlled_pair() {
    let mut model = sample_model();
    model.scenes[0].instances[0].bindings.push(PropBinding {
        prop: Identifier::new("pressed"),
        value: Value::Bool(true),
        description: None,
    });
    model.scenes[0].instances[0].bindings.push(PropBinding {
        prop: Identifier::new("defaultPressed"),
        value: Value::Bool(true),
        description: None,
    });

    let findings = validate(&model);
    let impossible: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::ImpossibleBinding)
        .collect();
    assert!(
        impossible
            .iter()
            .any(|f| f.message.contains("mutually exclusive")),
        "expected a do-not-mix finding: {impossible:#?}"
    );
}

// ---------------------------------------------------------------------------
// The reason this card exists: permitted subsets
// ---------------------------------------------------------------------------

#[test]
fn rejects_value_outside_permitted_subset() {
    let mut model = sample_model();
    // The shared type defines `success`; the component permits only
    // [default, danger, warning] (the motivating ButtonTone::Success case).
    model.scenes[0].instances[0].bindings.push(PropBinding {
        prop: Identifier::new("tone"),
        value: Value::member("success"),
        description: None,
    });

    let findings = validate(&model);
    let violations: Vec<_> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::PermittedSubsetViolation)
        .collect();
    assert!(
        !violations.is_empty(),
        "a member outside the permitted subset must be rejected (g13-b003 R6.2)"
    );
    assert!(
        violations.iter().any(|f| {
            f.identifier.contains("tone")
                && f.message.contains("success")
                && f.message.contains("permitted")
        }),
        "finding must name the offending binding, the value, and the permitted set: {violations:#?}"
    );
}

#[test]
fn rejects_default_outside_permitted_subset() {
    let mut model = sample_model();
    model.components[0].props[0].default = Some(Value::member("success"));

    let findings = validate(&model);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::PermittedSubsetViolation),
        "a default outside the permitted subset must be rejected: {findings:#?}"
    );
}

#[test]
fn reports_all_findings_at_once_not_first() {
    let mut model = sample_model();
    // Three independent violations in one model.
    model.schema_version = 99;
    model.components[0].props[0].prop_type =
        PropType::Shared(Identifier::new("missing-shared-type"));
    model.components[0].accessibility.name_rule = NameRule::Required;
    model.components[0].accessibility.name_source = None;

    let findings = validate(&model);
    let kinds = kinds(&findings);
    assert!(
        kinds.contains(&FindingKind::SchemaVersion)
            && kinds.contains(&FindingKind::InvalidReference)
            && kinds.contains(&FindingKind::MissingAccessibility),
        "validation must report every finding at once, got: {kinds:#?}"
    );
    assert!(
        findings.len() >= 3,
        "expected at least three findings, got {}",
        findings.len()
    );
}

// ---------------------------------------------------------------------------
// g13.018 — the identified-instance anatomy and the per-runtime capability
// provisions (both amendments are vocabulary; these are the validation
// rules that keep the vocabulary honest)
// ---------------------------------------------------------------------------

/// A complete per-runtime provision list: all four runtimes, each with a
/// provision and a reason.
fn four_runtime_status() -> Vec<CapabilityRuntimeStatus> {
    vec![
        CapabilityRuntimeStatus {
            runtime: RuntimeTarget::Svelte,
            provision: CapabilityProvision::Provided,
            reason: "svelte provides it".to_owned(),
        },
        CapabilityRuntimeStatus {
            runtime: RuntimeTarget::React,
            provision: CapabilityProvision::Delegated,
            reason: "react delegates it".to_owned(),
        },
        CapabilityRuntimeStatus {
            runtime: RuntimeTarget::Gpui,
            provision: CapabilityProvision::Provided,
            reason: "gpui provides it".to_owned(),
        },
        CapabilityRuntimeStatus {
            runtime: RuntimeTarget::Jetstream,
            provision: CapabilityProvision::Absent,
            reason: "jetstream lacks it, with a reason".to_owned(),
        },
    ]
}

#[test]
fn accepts_the_g13_018_vocabulary() {
    let mut model = sample_model();
    // The identified-instance family: the label part names the spinner as
    // its instance (both parts exist in the sample anatomy).
    for part in &mut model.components[0].parts {
        if part.id.as_str() == "label" {
            part.kind = PartKind::Identified {
                instances: vec![Identifier::new("spinner")],
                description: "the identified label instance".to_owned(),
            };
        }
    }
    // A complete per-runtime provision on the Focus requirement.
    model.components[0].capabilities[0].runtimes = four_runtime_status();

    let findings = validate(&model);
    assert!(
        findings.is_empty(),
        "the g13.018 vocabulary must validate clean, got: {findings:#?}"
    );
}

#[test]
fn rejects_identified_instance_that_is_not_a_part() {
    let mut model = sample_model();
    for part in &mut model.components[0].parts {
        if part.id.as_str() == "label" {
            part.kind = PartKind::Identified {
                instances: vec![Identifier::new("ghost-instance")],
                description: "an instance that is not a part".to_owned(),
            };
        }
    }

    let findings = validate(&model);
    let invalid = findings
        .iter()
        .filter(|f| f.kind == FindingKind::InvalidReference)
        .collect::<Vec<_>>();
    assert!(
        !invalid.is_empty(),
        "an identified instance must be a part in the same component: {findings:#?}"
    );
    assert!(
        invalid
            .iter()
            .any(|f| f.message.contains("ghost-instance")),
        "the finding must name the missing instance: {findings:#?}"
    );
}

#[test]
fn rejects_an_empty_identified_instance_list() {
    let mut model = sample_model();
    for part in &mut model.components[0].parts {
        if part.id.as_str() == "label" {
            part.kind = PartKind::Identified {
                instances: Vec::new(),
                description: "a fixed set of zero instances".to_owned(),
            };
        }
    }

    let findings = validate(&model);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::ImpossibleBinding),
        "a fixed set of identified instances must name at least one: {findings:#?}"
    );
}

#[test]
fn rejects_an_omitted_runtime_in_a_declared_provision() {
    let mut model = sample_model();
    let mut statuses = four_runtime_status();
    statuses.pop(); // Jetstream unlisted — silence must not mean absent.
    model.components[0].capabilities[0].runtimes = statuses;

    let findings = validate(&model);
    let incomplete = findings
        .iter()
        .filter(|f| f.kind == FindingKind::IncompleteCapabilityProvision)
        .collect::<Vec<_>>();
    assert!(
        !incomplete.is_empty(),
        "a declared provision must list every runtime: {findings:#?}"
    );
    assert!(
        incomplete
            .iter()
            .any(|f| f.message.contains("Jetstream")),
        "the finding must name the omitted runtime: {findings:#?}"
    );
}

#[test]
fn rejects_an_absent_runtime_without_a_reason() {
    let mut model = sample_model();
    let mut statuses = four_runtime_status();
    for status in &mut statuses {
        if status.provision == CapabilityProvision::Absent {
            status.reason = "   ".to_owned();
        }
    }
    model.components[0].capabilities[0].runtimes = statuses;

    let findings = validate(&model);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::IncompleteCapabilityProvision),
        "an absence must carry a reason (g13.018 R3): {findings:#?}"
    );
}

#[test]
fn rejects_a_duplicate_runtime_in_a_declared_provision() {
    let mut model = sample_model();
    let mut statuses = four_runtime_status();
    statuses.push(statuses[0].clone()); // Svelte listed twice.
    model.components[0].capabilities[0].runtimes = statuses;

    let findings = validate(&model);
    assert!(
        findings
            .iter()
            .any(|f| f.kind == FindingKind::DuplicateId),
        "one row per runtime: {findings:#?}"
    );
}
