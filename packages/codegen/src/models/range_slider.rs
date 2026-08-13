//! The authored RangeSlider definition — `g13.006`'s stateful proof,
//! defined once in Rust (spec 063 "Authoring Form": ordinary Rust types and
//! constructor helpers, no macros), serialized to the JSON fixture the
//! pipeline consumes (`ir:build` / `ir:check` via `load_and_validate`), and
//! emitted to both web packages through the `range-slider-ts` target.
//!
//! # Placement — pilot-scoped
//!
//! Same ruling as `g13-b035` R1 and `g13-b041` R1: `poodle-ir` is **lib
//! only, no `[[bin]]`**, pure serializable data plus validation — an
//! authored *instance* is content, not schema. This module lives in
//! `poodle-codegen`, reachable from the existing bin, and no new crate
//! exists. Where production models are authored is a `g13.008` question; do
//! not mistake this boundary for settled.
//!
//! # R1 — the IR declares the machine, it does not absorb it
//!
//! The slider machine already exists twice, hand-written in TS
//! (`packages/core/src/slider.ts`) and Rust
//! (`packages/contracts/headless/src/slider.rs`), and is already pinned by a
//! shared conformance vector (`slider` in
//! `packages/contracts/headless/vectors/machines.json`). Neither machine is
//! ported into the IR, and no state-machine encoding is invented to make it
//! generated. This definition declares everything a declaration *can*
//! carry — props, parts, state attributes, axes, recipe hooks,
//! accessibility, the VisualState projection shape, the gesture intent —
//! and names the machine semantics through `conformance` (`CROSS-18`).
//!
//! # R2 — the three things Button could not test
//!
//! 1. **Repeated anatomy.** The IR's only repetition mechanism is
//!    `PartKind::Repeated { over }`, and validation requires `over` to be a
//!    `List`-typed prop ("a repeated node needs a list source"). The
//!    RangeSlider value is a `Pair`, not a `List` — the two thumbs are a
//!    fixed `[lower, upper]` pair, and the contract's own anatomy (§2)
//!    defines Lower Control and Upper Control as distinct parts with
//!    distinct semantics (per-thumb aria labels, clamp bounds, Home/End
//!    behavior). Even over a list, a `Repeated` part yields **identical
//!    instances** — the expression vocabulary has no per-item index or
//!    identity operand, so "first repetition is lower, second is upper"
//!    cannot be declared. The two thumbs are therefore recorded as two
//!    distinct parts here, and the renderer hard-codes "two". The
//!    `Repeated` kind's own doc comment names "the two RangeSlider thumbs"
//!    as its motivating example; that example does not apply — the kind
//!    needs a list source the component does not have, and per-instance
//!    identity is not expressible. This is the finding `g13.006` exists to
//!    produce; it is recorded in the batch log for `g13.008`.
//! 2. **Value-dependent geometry.** Fill geometry is arithmetic over the
//!    value pair (`norm * 100`, negative/positive span splits) — excluded
//!    from the expression vocabulary by design. Spec 063's sanctioned
//!    escape is the **VisualState projection**: this definition declares
//!    the fourteen machine fields (`RNG-16`) and the seven computed custom
//!    properties (`--poodle-range-start/end/center/…`, `RNG-17`) as
//!    state-derived attributes whose `value` names the projection field;
//!    the shared machine (`rangeSliderVisualState`) computes the numbers;
//!    drawing consumes them. No runtime-specific value path: both web
//!    runtimes call the same shared machine.
//! 3. **Gesture effects.** Begin/move/end semantics and nearer-thumb
//!    selection are implemented by the hand-written
//!    `rangeSliderControlTransition` (`POINTER_BEGIN`/`MOVE`/`END`,
//!    `activeThumb`). This definition declares the gesture **intent**: the
//!    value-change/value-commit events with their firing phases, the
//!    adapter capabilities (pointer capture, per-thumb focus), the keyboard
//!    table, and the conformance reference to the `slider` vector the
//!    machines honor. The transitions themselves stay machine-owned (R1).
//!
//! # R5 — the conformance vector is a fixed target
//!
//! `packages/contracts/headless/vectors/machines.json` pins the slider
//! machine and must pass unedited against both machines. This model's
//! `conformance_vectors` carries a declarative `slider` vector so the
//! component's `conformance` reference resolves in the model (`CROSS-18`):
//! it names the machine semantics as step intents and cites the executable
//! vector file as evidence — it does not duplicate the vector's data.
//!
//! # R4 — this is re-plumbing
//!
//! The 18 web props keep their names, types, and defaults; the 8 `data-*`
//! attributes keep their names and values; `range-slider.css` is untouched.
//! The web surface is verified by `svelte:surface-audit`,
//! `docs:contract-drift`, the parity test's class-set diff, and the
//! existing RangeSlider tests.
//!
//! # Vocabulary notes recorded for g13.008
//!
//! - **The value pair is controlled-wins, not do-not-mix.** The IR's only
//!   `ControlRule` is `DoNotMix`; React's `value`/`defaultValue` pair is
//!   controlled-wins (`value !== undefined` wins, the same shape as
//!   Button's toggle pair, b041). The pair is recorded through the props
//!   (`defaultValue` marked web-only) and the machine's normalize path
//!   instead of `controlled_state`.
//! - **`law`'s default is an opaque object literal.** `AudioValueLaw`
//!   defaults to `{ type: "linear" }`, which no `Value` variant can carry
//!   (there is no object literal). The prop is `Opaque` with `default:
//!   None`; the default is named in the description — the same pattern
//!   b041 used for nullable shared props.
//! - **The geometry hooks are style properties, not `data-*` attributes.**
//!   The seven `--poodle-range-*` custom properties are declared through
//!   the same `StateAttribute` mechanism (RNG-17) but emitted by the
//!   `range-slider-ts` target as `styleProps` (inline-style names), not as
//!   DOM attributes; the components read them into `rangeStyle`.

use poodle_ir::{
    A11yRole, Accessibility, AriaMapping, AttributeForm, Axes, Capability, CapabilityRequirement,
    ComponentDefinition, ConformanceVector, ContractRef, ControlDensity, ControlSize,
    DensityAdjustment, DensityAxis, EmissionPolicy, Event, EventKind, EventPayload, EventTiming,
    Extension, FiringPhase, Identifier, IrModel, KeyChord, KeyboardCommand, Layer,
    MetricValue, Modifier, NameRule, NameSource, NativeAttr, Orientation, OrientationAxis, Part,
    PartKind, PayloadKind, PermittedSubset, Prop, PropType, RecipeHookRef, RecipeLink,
    RecipeLinkKind, RuntimeTarget, SharedEnumMember, SharedType, SizeAxis, SizeRole, SizeStep,
    StateAttribute, TokenGroup, TokenRef, Value, VectorStep, VectorStepKind, VisualFieldKind,
    VisualState, VisualStateField,
};

/// The governing contract, cited by the component and every definition row.
const CONTRACT: &str = "docs/contracts/components/range-slider.md";

/// The cross-component control types (004), referenced by the shared types
/// this model declares.
const SHARED_CONTRACT: &str = "docs/contracts/004-shared-control-types.md";

/// The executable conformance-vector evidence (R5 — a fixed target).
const VECTOR_EVIDENCE: &str = "packages/contracts/headless/vectors/machines.json";

fn ident(value: &str) -> Identifier {
    Identifier::new(value)
}

fn contract_ref(section: &str) -> ContractRef {
    ContractRef::new(CONTRACT, Some(section))
}

fn shared_contract_ref() -> ContractRef {
    ContractRef::new(SHARED_CONTRACT, None::<&str>)
}

/// Builds a [`SharedType`] member (`g13-b003` R6.1).
fn member(name: &str, description: &str) -> SharedEnumMember {
    SharedEnumMember {
        id: ident(name),
        name: name.to_owned(),
        description: description.to_owned(),
    }
}

/// Builds a [`SharedType`] from member ids, one line per member.
fn shared_type(
    shared_id: &str,
    rust_name: &str,
    canonical: ContractRef,
    members: &[(&str, &str)],
    description: &str,
) -> SharedType {
    SharedType {
        id: ident(shared_id),
        name: rust_name.to_owned(),
        description: description.to_owned(),
        canonical_ref: canonical,
        members: members
            .iter()
            .map(|(member_id, member_desc)| member(member_id, member_desc))
            .collect(),
    }
}

/// The permitted subset of a shared type for a prop (`g13-b003` R6.2).
fn subset(shared_id: &str, member_ids: &[&str]) -> PermittedSubset {
    PermittedSubset::new(shared_id, member_ids.iter().copied())
}

/// Builds a [`Prop`]. `default` is `None` when the contract default is null
/// on a shared-typed prop, or an opaque object no `Value` variant can carry
/// (see module notes).
#[allow(clippy::too_many_arguments)]
fn prop(
    id: &str,
    prop_type: PropType,
    default: Option<Value>,
    web_only: bool,
    description: &str,
) -> Prop {
    let permitted_subset = match &prop_type {
        PropType::Shared(shared_id) => {
            Some(subset(shared_id.as_str(), &all_members(shared_id.as_str())))
        }
        _ => None,
    };
    Prop {
        id: ident(id),
        name: id.to_owned(),
        prop_type,
        default,
        required: false,
        web_only,
        description: description.to_owned(),
        permitted_subset,
    }
}

/// Every member of a shared type — the permitted subset for a prop that
/// carries the full domain (validation requires a subset on shared props).
fn all_members(shared_id: &str) -> Vec<&'static str> {
    match shared_id {
        "slider-variant" => vec!["standard", "embedded"],
        "slider-polarity" => vec!["unipolar", "bipolar"],
        "slider-thumb" => vec!["lower", "upper"],
        "control-size" => vec!["xs", "sm", "md", "lg", "xl"],
        "control-density" => vec!["compact", "default", "comfortable"],
        "control-size-role" => vec!["chrome", "control", "prominent"],
        "orientation" => vec!["horizontal", "vertical"],
        other => panic!("no member list for shared type '{other}'"),
    }
}

fn shared(shared_id: &str) -> PropType {
    PropType::Shared(ident(shared_id))
}

/// Builds a boolean prop with a `false` default.
fn bool_prop(id: &str, description: &str) -> Prop {
    prop(
        id,
        PropType::Bool,
        Some(Value::boolean(false)),
        false,
        description,
    )
}

/// A valued state attribute deriving from a prop or VisualState field
/// (`CROSS-13`). `source` names the field the value derives from; the
/// emitted vocabulary is the attribute row itself, never an expression
/// tree (g13.017).
fn valued_attribute(
    id: &str,
    name: &str,
    source: &str,
    emission: EmissionPolicy,
    description: &str,
) -> StateAttribute {
    StateAttribute {
        id: ident(id),
        name: name.to_owned(),
        form: AttributeForm::Valued,
        emission,
        source: Some(ident(source)),
        description: description.to_owned(),
    }
}

/// A VisualState projection field (`CROSS-14`, `RNG-16`).
fn visual_field(
    id: &str,
    name: &str,
    kind: VisualFieldKind,
    description: &str,
) -> VisualStateField {
    VisualStateField {
        id: ident(id),
        name: name.to_owned(),
        kind,
        description: description.to_owned(),
    }
}

/// Builds one recipe-hook override chain (`CROSS-09`, `RNG-21`): hook →
/// terminal token. The RangeSlider stylesheet consumes each hook with an
/// inline token fallback (`var(--poodle-recipe-range-slider-*, <token>)`),
/// so the chain has no intermediate component variable — unlike Button's
/// chains, which run hook → component variable → token (BTN-22).
fn recipe_hook(hook: &str, token: &str, description: &str) -> RecipeHookRef {
    RecipeHookRef {
        hook: hook.to_owned(),
        chain: vec![
            RecipeLink {
                kind: RecipeLinkKind::RecipeHook,
                target: hook.to_owned(),
            },
            RecipeLink {
                kind: RecipeLinkKind::Token,
                target: token.to_owned(),
            },
        ],
        description: description.to_owned(),
    }
}

/// The 11 recipe hooks of `range-slider.css` (RNG-21; the contract §8
/// recipe-hook table). Each hook's fallback token is transcribed from the
/// stylesheet's `var()` fallback.
fn recipe_hooks() -> Vec<RecipeHookRef> {
    vec![
        recipe_hook(
            "--poodle-recipe-range-slider-track-fill",
            "color.background.surface",
            "Track background; surface 88% color-mix base (R §8, RNG-21).",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-fill-fill",
            "color.accent.base",
            "Positive selected-fill segment (R §8, RNG-21).",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-fill-negative",
            "color.status.danger",
            "Negative selected-fill segment — the bipolar side of the origin \
             (R §8, RNG-21; the acceptance line 'negative/positive fill geometry and \
             recipe roles remain exact').",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-track-border",
            "color.border.default",
            "Control border (R §8, RNG-21).",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-center-fill",
            "color.border.strong",
            "Bipolar center reference marker (R §8, RNG-21).",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-control-fill",
            "color.background.canvas",
            "Standard-variant control fill; transparent in the embedded variant \
             (R §8, RNG-21).",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-control-track-fill",
            "color.background.canvas",
            "Native input runnable-track fill; transparent (R §8, RNG-21).",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-control-thumb-fill",
            "color.background.elevated",
            "Thumb fill (R §8, RNG-21).",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-control-thumb-shadow",
            "color.background.elevated",
            "Thumb shadow — a black 18% color-mix with no single semantic token; \
             the elevated fallback names the family (R §8, RNG-21).",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-focus-ring",
            "color.accent.focusRing",
            "Focus ring for controls and embedded stops (R §8, RNG-21).",
        ),
        recipe_hook(
            "--poodle-recipe-range-slider-focus-control-thumb-shadow",
            "color.accent.focusRing",
            "Compound focus ring on the native thumb; focusRing 32% color-mix \
             (R §8, RNG-21).",
        ),
    ]
}

/// Builds a semantic [`TokenRef`] (`CROSS-09`, `RNG-21`).
fn token(path: &str, description: &str) -> TokenRef {
    TokenRef {
        path: path.to_owned(),
        group: TokenGroup::Semantic,
        description: description.to_owned(),
    }
}

/// The per-rung size metrics (R §8, RNG-09): control min-height, track
/// thickness, and thumb diameter in rem, from the contract's size table.
fn size_metrics(
    min_height: f64,
    track_thickness: f64,
    thumb_diameter: f64,
) -> std::collections::BTreeMap<String, MetricValue> {
    let mut metrics = std::collections::BTreeMap::new();
    metrics.insert("min-height".to_owned(), MetricValue::Rem(min_height));
    metrics.insert(
        "track-thickness".to_owned(),
        MetricValue::Rem(track_thickness),
    );
    metrics.insert(
        "thumb-diameter".to_owned(),
        MetricValue::Rem(thumb_diameter),
    );
    metrics
}

/// The `g13.006` RangeSlider definition — the stateful proof: controlled
/// pair state, gesture effects, two-thumb anatomy, and value-dependent
/// fill geometry (R2).
pub fn range_slider_definition() -> ComponentDefinition {
    ComponentDefinition {
        id: ident("range-slider"),
        name: "RangeSlider".to_owned(),
        layer: Layer::Foundation,
        contract: contract_ref("§3"),
        description: "A dual-thumb range control representing lower and upper numeric bounds, \
                      built on two overlapping native range inputs with a custom track and fill \
                      visualization (R §1). g13.006's stateful proof: 18 web props, 8 data-* \
                      attributes, two-thumb anatomy, unipolar/bipolar fill geometry, and a \
                      machine pinned by the shared slider conformance vector (RNG-01..29)."
            .to_owned(),

        // 16 data props + 2 callbacks (declared as events) = the 18-web-prop
        // surface. `defaultValue` (React's uncontrolled seed) is recorded
        // web-only at the end. Order is the contract's §3 table order.
        props: vec![
            prop(
                "value",
                PropType::Pair(Box::new(PropType::Number)),
                Some(Value::Pair(
                    Box::new(Value::Number(0.0)),
                    Box::new(Value::Number(100.0)),
                )),
                false,
                "Controlled lower/upper pair (R §3, RNG-01).",
            ),
            prop(
                "min",
                PropType::Number,
                Some(Value::number(0.0)),
                false,
                "Lower bound (R §3, RNG-02).",
            ),
            prop(
                "max",
                PropType::Number,
                Some(Value::number(100.0)),
                false,
                "Upper bound (R §3, RNG-02).",
            ),
            prop(
                "step",
                PropType::Number,
                Some(Value::number(1.0)),
                false,
                "Increment size (R §3, RNG-02).",
            ),
            prop(
                "variant",
                shared("slider-variant"),
                Some(Value::member("standard")),
                false,
                "Native-input control or dense composite control (R §3, RNG-03).",
            ),
            prop(
                "polarity",
                shared("slider-polarity"),
                Some(Value::member("unipolar")),
                false,
                "Ordinary range or an explicit bipolar center reference (R §3, RNG-04).",
            ),
            prop(
                "centerValue",
                PropType::Number,
                Some(Value::Null),
                false,
                "Bipolar reference; null defaults to zero when zero is inside the range, \
                 otherwise the midpoint (R §3, RNG-05).",
            ),
            prop(
                "law",
                PropType::Opaque,
                None,
                false,
                "Embedded-variant value mapping; default { type: \"linear\" } — an opaque \
                 payload whose default object literal no Value variant can carry, so the \
                 default is recorded as None and named here (R §3; b041 vocabulary-note \
                 pattern).",
            ),
            prop(
                "orientation",
                shared("orientation"),
                Some(Value::member("horizontal")),
                false,
                "Layout and interaction axis (R §3/§7, RNG-07; CROSS-11).",
            ),
            bool_prop(
                "disabled",
                "Disables interaction and applies disabled opacity (R §3, RNG-08).",
            ),
            prop(
                "ariaLabel",
                PropType::String,
                Some(Value::Null),
                false,
                "Base accessible name; per-thumb labels append \"minimum\"/\"maximum\" \
                 (R §3/§6, RNG-10).",
            ),
            prop(
                "lowerValueText",
                PropType::String,
                Some(Value::Null),
                false,
                "Human-readable text for the lower thumb (aria-valuetext) (R §3, RNG-10).",
            ),
            prop(
                "upperValueText",
                PropType::String,
                Some(Value::Null),
                false,
                "Human-readable text for the upper thumb (aria-valuetext) (R §3, RNG-10).",
            ),
            prop(
                "size",
                shared("control-size"),
                None,
                false,
                "Explicit control-size override; default null — resolves from inherited \
                 presentation plus sizeRole (R §3, RNG-06; CROSS-07).",
            ),
            prop(
                "sizeRole",
                shared("control-size-role"),
                Some(Value::member("control")),
                false,
                "Semantic size offset from inherited presentation (R §3, RNG-06; CROSS-07).",
            ),
            prop(
                "density",
                shared("control-density"),
                None,
                false,
                "Explicit density override; default null — inherited from presentation \
                 (R §3, RNG-09; CROSS-08).",
            ),
            prop(
                "defaultValue",
                PropType::Pair(Box::new(PropType::Number)),
                Some(Value::Pair(
                    Box::new(Value::Number(0.0)),
                    Box::new(Value::Number(100.0)),
                )),
                true,
                "React uncontrolled seed; the value pair is controlled-wins (value !== \
                 undefined), so the pair is recorded through the props rather than the \
                 DoNotMix controlled_state (R §3 controlled/uncontrolled; CROSS-03 \
                 web-only idiom, CROSS-04).",
            ),
        ],

        // The value pair is controlled-wins, not do-not-mix — see the module
        // notes; recorded through the props and the machine's normalize path.
        controlled_state: Vec::new(),

        // The two callbacks (R §5, RNG-11): value-change during interaction,
        // value-commit on release, pair payload reported together (RNG-27).
        events: vec![
            Event {
                id: ident("value-change"),
                name: "onValueChange".to_owned(),
                kind: EventKind::ValueChange,
                payload: Some(EventPayload {
                    name: "[lower, upper]".to_owned(),
                    kind: PayloadKind::Pair,
                }),
                timing: EventTiming {
                    phase: FiringPhase::DuringInteraction,
                    ..EventTiming::default()
                },
                description: "Either thumb changes value during interaction; the pair is \
                              reported together and lower <= upper is preserved (R §5, RNG-11)."
                    .to_owned(),
            },
            Event {
                id: ident("value-commit"),
                name: "onValueCommit".to_owned(),
                kind: EventKind::ValueCommit,
                payload: Some(EventPayload {
                    name: "[lower, upper]".to_owned(),
                    kind: PayloadKind::Pair,
                }),
                timing: EventTiming {
                    phase: FiringPhase::OnRelease,
                    ..EventTiming::default()
                },
                description: "Interaction finishes on either thumb (mouseup/touchend/keyup \
                              commit) (R §5, RNG-11)."
                    .to_owned(),
            },
        ],

        // The anatomy (R §2 + the rendered DOM): the contract's five parts
        // split into the DOM's nine — the fill renders as negative/positive
        // segments plus a center marker, and each variant's controls are
        // distinct parts (see the module notes for why the two thumbs are
        // not one Repeated part: R2.1).
        parts: vec![
            Part {
                id: ident("root"),
                name: "Root".to_owned(),
                parent: None,
                kind: PartKind::Static,
                description: "Range slider host with relative positioning; role=\"group\" \
                              (R §2, RNG-14)."
                    .to_owned(),
            },
            Part {
                id: ident("track"),
                name: "Track".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::Static,
                description: "Full available range background bar (R §2, RNG-14).".to_owned(),
            },
            Part {
                id: ident("fill-negative"),
                name: "Negative Fill".to_owned(),
                parent: Some(ident("track")),
                kind: PartKind::Static,
                description: "Bipolar selected-fill segment on the negative side of the \
                              center reference; empty in unipolar (R §2/§8, RNG-16)."
                    .to_owned(),
            },
            Part {
                id: ident("fill-positive"),
                name: "Positive Fill".to_owned(),
                parent: Some(ident("track")),
                kind: PartKind::Static,
                description: "Selected-fill segment on the positive side of the center \
                              reference — the whole window in unipolar (R §2/§8, RNG-16)."
                    .to_owned(),
            },
            Part {
                id: ident("center"),
                name: "Center".to_owned(),
                parent: Some(ident("track")),
                kind: PartKind::Static,
                description: "Bipolar center reference marker; hidden in unipolar and in the \
                              standard variant (R §2/§8)."
                    .to_owned(),
            },
            Part {
                id: ident("control-lower"),
                name: "Lower Control".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::ConditionalDocumented {
                    condition: "standard variant only".to_owned(),
                    description: "Native range input for the lower thumb (R §2, RNG-14/15)."
                        .to_owned(),
                },
                description: "Lower bound thumb input (R §2).".to_owned(),
            },
            Part {
                id: ident("control-upper"),
                name: "Upper Control".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::ConditionalDocumented {
                    condition: "standard variant only".to_owned(),
                    description: "Native range input for the upper thumb (R §2, RNG-14/15)."
                        .to_owned(),
                },
                description: "Upper bound thumb input (R §2).".to_owned(),
            },
            Part {
                id: ident("embedded-lower"),
                name: "Embedded Lower Control".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::ConditionalDocumented {
                    condition: "embedded variant only".to_owned(),
                    description: "Adapter-owned slider focus stop for the lower thumb \
                                  (R §2/§6, RNG-15)."
                        .to_owned(),
                },
                description: "Embedded lower bound focus stop (R §2).".to_owned(),
            },
            Part {
                id: ident("embedded-upper"),
                name: "Embedded Upper Control".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::ConditionalDocumented {
                    condition: "embedded variant only".to_owned(),
                    description: "Adapter-owned slider focus stop for the upper thumb \
                                  (R §2/§6, RNG-15)."
                        .to_owned(),
                },
                description: "Embedded upper bound focus stop (R §2).".to_owned(),
            },
        ],

        // The 8 data-* attributes (R §9, RNG-17) plus the 7 fill-geometry
        // custom properties (RNG-17). Names, forms, emission policies, and
        // value domains are the rendered vocabulary the `range-slider-ts`
        // artifact carries (R2); the geometry hooks are emitted as
        // styleProps, not DOM attributes.
        attributes: vec![
            valued_attribute(
                "orientation",
                "data-orientation",
                "orientation",
                EmissionPolicy::Always,
                "The orientation value; always emitted; styling hook only, not exposed to \
                 assistive technology (R §9, RNG-17).",
            ),
            valued_attribute(
                "disabled",
                "data-disabled",
                "disabled",
                EmissionPolicy::Always,
                "The disabled boolean; always emitted (R §9, RNG-17).",
            ),
            valued_attribute(
                "variant",
                "data-variant",
                "variant",
                EmissionPolicy::Always,
                "The variant value; always emitted (R §9, RNG-17).",
            ),
            valued_attribute(
                "polarity",
                "data-polarity",
                "polarity",
                EmissionPolicy::Always,
                "The polarity from the machine's visual state; always emitted (R §9, RNG-17).",
            ),
            valued_attribute(
                "fill-split",
                "data-fill-split",
                "fillSplitAtCenter",
                EmissionPolicy::Always,
                "Whether both fill segments meet at the center, so renderers square only the \
                 touching corners; always emitted (R §4/§8, RNG-16/17).",
            ),
            // data-state is runtime-derived: active while a gesture is in
            // progress (R §4, RNG-17). The `pointerActive ? "active" :
            // "idle"` selection was an expression and is gone (g13.017 R1
            // bucket 3: derivation); the domain {active, idle} is this
            // description's prose and the runtime's own projection.
            StateAttribute {
                id: ident("state"),
                name: "data-state".to_owned(),
                form: AttributeForm::Valued,
                emission: EmissionPolicy::Always,
                source: None,
                description: "Interaction state — active while a gesture is in progress; \
                              always emitted (R §4, RNG-17)."
                    .to_owned(),
            },
            valued_attribute(
                "size",
                "data-size",
                "resolvedSize",
                EmissionPolicy::Always,
                "The resolved control size (explicit or sizeRole-derived); always emitted \
                 (R §9, RNG-17; CROSS-07).",
            ),
            valued_attribute(
                "density",
                "data-density",
                "resolvedDensity",
                EmissionPolicy::Always,
                "The resolved density (explicit or inherited); always emitted (R §9, RNG-17; \
                 CROSS-08).",
            ),
            // The fill geometry (RNG-17): computed custom properties fed by
            // the machine's visual state fields. The runtime computes the
            // values (`norm * 100%` — arithmetic is not vocabulary) and
            // emits them as inline style; `source` names the field.
            valued_attribute(
                "range-start",
                "--poodle-range-start",
                "lowerNorm",
                EmissionPolicy::Always,
                "Fill start position, from the lower thumb's normalized value (R §3, RNG-17).",
            ),
            valued_attribute(
                "range-end",
                "--poodle-range-end",
                "upperNorm",
                EmissionPolicy::Always,
                "Fill end position, from the upper thumb's normalized value (R §3, RNG-17).",
            ),
            valued_attribute(
                "range-center",
                "--poodle-range-center",
                "centerNorm",
                EmissionPolicy::Always,
                "Center reference position (R §3, RNG-17).",
            ),
            valued_attribute(
                "range-negative-start",
                "--poodle-range-negative-start",
                "negativeFillStartNorm",
                EmissionPolicy::Always,
                "Negative segment start — which side of the origin the negative fill grows \
                 from (R §3, RNG-17).",
            ),
            valued_attribute(
                "range-negative-span",
                "--poodle-range-negative-span",
                "negativeFillSpanNorm",
                EmissionPolicy::Always,
                "Negative segment width — how negative fill is expressed (R §3, RNG-17).",
            ),
            valued_attribute(
                "range-positive-start",
                "--poodle-range-positive-start",
                "positiveFillStartNorm",
                EmissionPolicy::Always,
                "Positive segment start (R §3, RNG-17).",
            ),
            valued_attribute(
                "range-positive-span",
                "--poodle-range-positive-span",
                "positiveFillSpanNorm",
                EmissionPolicy::Always,
                "Positive segment width (R §3, RNG-17).",
            ),
        ],

        // Axes (CROSS-07/08/11): the size ladder with the contract's fixed
        // rem metrics (RNG-09), the density adjustments that grow only the
        // vertical hit area (the documented §8 exception), and the
        // orientation axis (RNG-07).
        axes: Axes {
            size: Some(SizeAxis {
                explicit: None,
                size_role: SizeRole::Control,
                ladder: vec![
                    SizeStep {
                        size: ControlSize::Xs,
                        metrics: size_metrics(1.25, 0.1875, 0.75),
                        description: "Extra-small rung: min-height 1.25rem, track 0.1875rem, \
                                      thumb 0.75rem (R §8, RNG-09)."
                            .to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Sm,
                        metrics: size_metrics(1.375, 0.25, 0.875),
                        description: "Small rung: min-height 1.375rem, track 0.25rem, thumb \
                                      0.875rem (R §8, RNG-09)."
                            .to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Md,
                        metrics: size_metrics(1.5, 0.375, 1.0),
                        description: "Default rung: min-height 1.5rem, track 0.375rem, thumb \
                                      1rem (R §8, RNG-09)."
                            .to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Lg,
                        metrics: size_metrics(1.625, 0.5, 1.125),
                        description: "Large rung: min-height 1.625rem, track 0.5rem, thumb \
                                      1.125rem (R §8, RNG-09)."
                            .to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Xl,
                        metrics: size_metrics(1.75, 0.625, 1.25),
                        description: "Extra-large rung: min-height 1.75rem, track 0.625rem, \
                                      thumb 1.25rem (R §8, RNG-09)."
                            .to_owned(),
                    },
                ],
            }),
            density: Some(DensityAxis {
                explicit: None,
                adjustments: vec![
                    DensityAdjustment {
                        density: ControlDensity::Compact,
                        applies_to: Some(ident("root")),
                        inline: None,
                        block: Some(MetricValue::Rem(0.25)),
                        description: "Compact adds 0.25rem vertical hit-area padding to the \
                                      root — the documented §8 exception: density grows the \
                                      grabbable margin without changing track, thumb, or \
                                      visual min-height (R §8, RNG-09)."
                            .to_owned(),
                    },
                    DensityAdjustment {
                        density: ControlDensity::Comfortable,
                        applies_to: Some(ident("root")),
                        inline: None,
                        block: Some(MetricValue::Rem(0.75)),
                        description: "Comfortable adds 0.75rem vertical hit-area padding to \
                                      the root (R §8, RNG-09)."
                            .to_owned(),
                    },
                ],
            }),
            orientation: Some(OrientationAxis {
                default: Orientation::Horizontal,
                values: vec![Orientation::Horizontal, Orientation::Vertical],
            }),
        },

        // RNG-21: the semantic tokens the appearance consumes, resolved
        // against the generated poodle-tokens registry (CROSS-09).
        tokens: vec![
            token(
                "color.background.surface",
                "Track fill color-mix base (R §8).",
            ),
            token("color.accent.base", "Positive fill segment (R §8)."),
            token(
                "color.status.danger",
                "Negative fill segment — the bipolar side of the origin (R §8).",
            ),
            token(
                "color.border.default",
                "Track border and thumb border (R §8).",
            ),
            token(
                "color.background.canvas",
                "Standard-variant control fill (R §8).",
            ),
            token("color.border.strong", "Center reference marker (R §8)."),
            token("color.accent.focusRing", "Focus rings (R §8)."),
            token("color.background.elevated", "Thumb fill (R §8)."),
            token(
                "state.opacity.disabled",
                "Disabled opacity on the root (R §8).",
            ),
        ],

        recipe_hooks: recipe_hooks(),

        // RNG-10/14/15 accessibility intent (CROSS-15): group root, two
        // per-thumb slider semantics, orientation channel on the root only.
        accessibility: Accessibility {
            role: A11yRole::Group,
            name_rule: NameRule::FromProp(ident("ariaLabel")),
            name_source: Some(NameSource::Prop(ident("ariaLabel"))),
            aria: vec![
                AriaMapping {
                    aria_attr: "aria-label".to_owned(),
                    source: ident("ariaLabel"),
                    description: "Per-thumb accessible names append \"minimum\"/\"maximum\" \
                                  to the base label, or default to \"Minimum value\"/\
                                  \"Maximum value\" (R §6, RNG-10)."
                        .to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-valuemin".to_owned(),
                    source: ident("min"),
                    description: "Lower bound on both inputs (R §6, RNG-10).".to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-valuemax".to_owned(),
                    source: ident("max"),
                    description: "Upper bound on both inputs (R §6, RNG-10).".to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-valuenow".to_owned(),
                    source: ident("value"),
                    description: "Lower value on the lower thumb, upper value on the upper \
                                  thumb (R §6, RNG-10)."
                        .to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-valuetext".to_owned(),
                    source: ident("lowerValueText"),
                    description: "Human-readable lower value (R §6, RNG-10).".to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-valuetext".to_owned(),
                    source: ident("upperValueText"),
                    description: "Human-readable upper value (R §6, RNG-10).".to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-orientation".to_owned(),
                    source: ident("orientation"),
                    description: "Required on the embedded slider focus stops — custom widgets \
                                  with no implicit orientation; NOT set on the native range \
                                  inputs (R §6, RNG-10)."
                        .to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-disabled".to_owned(),
                    source: ident("disabled"),
                    description: "Disabled state on the embedded focus stops (R §6, RNG-10)."
                        .to_owned(),
                },
            ],
            native: vec![
                NativeAttr {
                    name: "disabled".to_owned(),
                    description: "Native disabled attribute on both inputs (R §6, RNG-10)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "type".to_owned(),
                    description: "Native range input type on both controls (R §6, RNG-14)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "min".to_owned(),
                    description: "Native min on both inputs; the lower input's max clamps to \
                                  the upper value (R §3/§6, RNG-12)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "max".to_owned(),
                    description: "Native max on both inputs; the upper input's min clamps to \
                                  the lower value (R §3/§6, RNG-12)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "step".to_owned(),
                    description: "Native step on both inputs (R §6, RNG-02).".to_owned(),
                },
            ],
            description: "Group root with two related slider semantics; each thumb is \
                          individually focusable and distinguishable as lower or upper bound \
                          (R §6, RNG-10/14/15)."
                .to_owned(),
        },

        // Adapter-owned environment work (R3; RNG-13/15/20; CROSS-17): the
        // definition declares that these exist and what they mean; each
        // runtime's adapter owns the implementation.
        capabilities: vec![
            CapabilityRequirement {
                capability: Capability::PointerCapture,
                purpose: "Embedded-variant shared-root pointer capture — the full root is the \
                          pointer target and the gesture never transfers thumbs (R §3/§4, \
                          RNG-15)."
                    .to_owned(),
            },
            CapabilityRequirement {
                capability: Capability::Focus,
                purpose: "Per-thumb focus stops — Tab cycles lower, upper, and out (R §6, \
                          RNG-18/20)."
                    .to_owned(),
            },
            CapabilityRequirement {
                capability: Capability::ScrubFraction,
                purpose: "Native grab overlay reports the pointer position as a fraction \
                          (Interaction::on_scrub); the component picks the nearer thumb on \
                          the press (R §3, RNG-13)."
                    .to_owned(),
            },
        ],

        // RNG-18 keyboard table (R §6; CROSS-16).
        keyboard: vec![
            KeyboardCommand {
                id: ident("decrement-step"),
                keys: vec![
                    KeyChord {
                        key: "ArrowLeft".to_owned(),
                        modifiers: Default::default(),
                    },
                    KeyChord {
                        key: "ArrowDown".to_owned(),
                        modifiers: Default::default(),
                    },
                ],
                action: "decrement-step".to_owned(),
                effect: "Decrements the focused thumb by step through the machine — INPUT then \
                         COMMIT, preserving lower <= upper (R §6, RNG-18)."
                    .to_owned(),
                requires: None,
                description: "Arrow Left/Down decrements the focused thumb (R §6, RNG-18)."
                    .to_owned(),
            },
            KeyboardCommand {
                id: ident("increment-step"),
                keys: vec![
                    KeyChord {
                        key: "ArrowRight".to_owned(),
                        modifiers: Default::default(),
                    },
                    KeyChord {
                        key: "ArrowUp".to_owned(),
                        modifiers: Default::default(),
                    },
                ],
                action: "increment-step".to_owned(),
                effect: "Increments the focused thumb by step through the machine — INPUT then \
                         COMMIT, preserving lower <= upper (R §6, RNG-18)."
                    .to_owned(),
                requires: None,
                description: "Arrow Right/Up increments the focused thumb (R §6, RNG-18)."
                    .to_owned(),
            },
            KeyboardCommand {
                id: ident("move-to-min"),
                keys: vec![KeyChord {
                    key: "Home".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "move-to-min".to_owned(),
                effect: "Moves the focused thumb to min (lower thumb) or to the lower value \
                         (upper thumb) — the per-thumb Home bound (R §6, RNG-18)."
                    .to_owned(),
                requires: None,
                description: "Home moves the focused thumb to its lower bound (R §6, RNG-18)."
                    .to_owned(),
            },
            KeyboardCommand {
                id: ident("move-to-max"),
                keys: vec![KeyChord {
                    key: "End".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "move-to-max".to_owned(),
                effect: "Moves the focused thumb to the upper value (lower thumb) or to max \
                         (upper thumb) — the per-thumb End bound (R §6, RNG-18)."
                    .to_owned(),
                requires: None,
                description: "End moves the focused thumb to its upper bound (R §6, RNG-18)."
                    .to_owned(),
            },
            KeyboardCommand {
                id: ident("move-focus-next"),
                keys: vec![KeyChord {
                    key: "Tab".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "move-focus".to_owned(),
                effect: "Moves focus from the lower thumb to the upper thumb and onward \
                         (R §6, RNG-18/20)."
                    .to_owned(),
                requires: Some(Capability::Focus),
                description: "Tab moves focus between thumbs and out of the control (R §6, \
                              RNG-18)."
                    .to_owned(),
            },
            KeyboardCommand {
                id: ident("move-focus-previous"),
                keys: vec![KeyChord {
                    key: "Tab".to_owned(),
                    modifiers: [Modifier::Shift].into_iter().collect(),
                }],
                action: "move-focus".to_owned(),
                effect: "Moves focus from the upper thumb to the lower thumb and backward \
                         (R §6, RNG-18/20)."
                    .to_owned(),
                requires: Some(Capability::Focus),
                description: "Shift+Tab moves focus backward through the thumbs (R §6, RNG-18)."
                    .to_owned(),
            },
        ],

        // RNG-16 visual-state projection: the fields the shared machine
        // computes (rangeSliderVisualState) and drawing consumes (CROSS-14,
        // IR-06). `resolvedSize`/`resolvedDensity` are the resolution the
        // attributes derive from.
        visual_state: vec![VisualState {
            id: ident("range-slider-visual-state"),
            name: "RangeSliderVisualState".to_owned(),
            fields: vec![
                visual_field(
                    "value",
                    "value",
                    VisualFieldKind::Pair,
                    "The ordered, clamped display pair (R §4, RNG-16).",
                ),
                visual_field(
                    "lowerNorm",
                    "lowerNorm",
                    VisualFieldKind::Number,
                    "Lower thumb position normalized to [0, 1] through the law (R §4, RNG-16).",
                ),
                visual_field(
                    "upperNorm",
                    "upperNorm",
                    VisualFieldKind::Number,
                    "Upper thumb position normalized to [0, 1] through the law (R §4, RNG-16).",
                ),
                visual_field(
                    "centerNorm",
                    "centerNorm",
                    VisualFieldKind::Number,
                    "Center reference normalized to [0, 1] (R §4, RNG-16).",
                ),
                visual_field(
                    "fillStartNorm",
                    "fillStartNorm",
                    VisualFieldKind::Number,
                    "Fill start = lowerNorm (R §4, RNG-16).",
                ),
                visual_field(
                    "fillSpanNorm",
                    "fillSpanNorm",
                    VisualFieldKind::Number,
                    "Fill span = upperNorm - lowerNorm — the low-to-high selected window \
                     (R §4, RNG-16).",
                ),
                visual_field(
                    "negativeFillStartNorm",
                    "negativeFillStartNorm",
                    VisualFieldKind::Number,
                    "Negative segment start — the bipolar side of the origin (R §4, RNG-16).",
                ),
                visual_field(
                    "negativeFillSpanNorm",
                    "negativeFillSpanNorm",
                    VisualFieldKind::Number,
                    "Negative segment width; zero in unipolar (R §4, RNG-16).",
                ),
                visual_field(
                    "positiveFillStartNorm",
                    "positiveFillStartNorm",
                    VisualFieldKind::Number,
                    "Positive segment start (R §4, RNG-16).",
                ),
                visual_field(
                    "positiveFillSpanNorm",
                    "positiveFillSpanNorm",
                    VisualFieldKind::Number,
                    "Positive segment width; the whole window in unipolar (R §4, RNG-16).",
                ),
                visual_field(
                    "fillSplitAtCenter",
                    "fillSplitAtCenter",
                    VisualFieldKind::Bool,
                    "Both segments meet at the center — renderers square only the touching \
                     corners (R §4, RNG-16/23).",
                ),
                visual_field(
                    "polarity",
                    "polarity",
                    VisualFieldKind::Enum(ident("slider-polarity")),
                    "The declared polarity (R §4, RNG-16).",
                ),
                visual_field(
                    "pointerActive",
                    "pointerActive",
                    VisualFieldKind::Bool,
                    "A gesture is in progress (R §4, RNG-16).",
                ),
                visual_field(
                    "activeThumb",
                    "activeThumb",
                    VisualFieldKind::Enum(ident("slider-thumb")),
                    "Which thumb the gesture holds — selected on begin, held for the gesture \
                     (R §4, RNG-12/16).",
                ),
                visual_field(
                    "enabled",
                    "enabled",
                    VisualFieldKind::Bool,
                    "!disabled (R §4, RNG-16).",
                ),
                visual_field(
                    "resolvedSize",
                    "resolvedSize",
                    VisualFieldKind::Enum(ident("control-size")),
                    "Explicit size or sizeRole resolution (R §7, RNG-06; CROSS-07).",
                ),
                visual_field(
                    "resolvedDensity",
                    "resolvedDensity",
                    VisualFieldKind::Enum(ident("control-density")),
                    "Explicit density or inherited presentation (R §8, RNG-09; CROSS-08).",
                ),
            ],
            description: "The projection the shared machine computes (rangeSliderVisualState \
                          in core slider.ts; RangeSliderVisualState in contracts/headless \
                          slider.rs), which drawing consumes — geometry and gesture state, \
                          never machine internals (R §4; CROSS-14, RNG-16, IR-06)."
                .to_owned(),
        }],

        // R1/R5: the machine semantics are declared by the shared `slider`
        // conformance vector (CROSS-18), authored in this model so the
        // reference resolves; the executable vector stays
        // machines.json — a fixed target (R5).
        conformance: vec![ident("slider")],

        // R §12 known deltas (EXT class rows RNG-26/27) and the R §12
        // table: the web half records them now so the four-runtime
        // definition is complete; card 046 consumes them for the natives.
        extensions: vec![
            Extension {
                id: ident("gpui-native-vertical"),
                owning_runtime: RuntimeTarget::Gpui,
                reason: "Vertical orientation is implemented natively rather than via CSS \
                         rotation (R §10/§12; RNG-26)."
                    .to_owned(),
                parity_effect: "Same visual and interaction result by native means.".to_owned(),
                evidence_surface: "docs/contracts/components/range-slider.md §12".to_owned(),
                removal_condition: "None — the difference is intentional.".to_owned(),
                description: "RNG-26 GPUI native vertical.".to_owned(),
            },
            Extension {
                id: ident("jetstream-pair-reporting"),
                owning_runtime: RuntimeTarget::Jetstream,
                reason: "The pair is the value; a host told about one thumb alone would have \
                         to remember the other, so Jetstream reports (low, high) together \
                         (R §10a; RNG-27)."
                    .to_owned(),
                parity_effect: "Jetstream on_change/on_value_commit carry the pair, matching \
                                 the web events' pair payload."
                    .to_owned(),
                evidence_surface: "docs/contracts/components/range-slider.md §10a".to_owned(),
                removal_condition: "None — the difference is intentional.".to_owned(),
                description: "RNG-27 Jetstream pair reporting.".to_owned(),
            },
            Extension {
                id: ident("web-overlapping-inputs"),
                owning_runtime: RuntimeTarget::React,
                reason: "Two overlapping native range inputs is the web DOM pattern; the \
                         native targets use a single grab overlay (R §12)."
                    .to_owned(),
                parity_effect: "Same interaction and accessibility result from a different \
                                 DOM pattern."
                    .to_owned(),
                evidence_surface: "docs/contracts/components/range-slider.md §12".to_owned(),
                removal_condition: "None — the difference is intentional.".to_owned(),
                description: "R §12 two-overlapping-inputs delta (web side).".to_owned(),
            },
            Extension {
                id: ident("native-embedded-orientation-gap"),
                owning_runtime: RuntimeTarget::Gpui,
                reason: "poodle-node carries no orientation channel in its accessibility \
                         vocabulary, so neither native adapter can project the embedded \
                         aria-orientation today (R §12)."
                    .to_owned(),
                parity_effect: "Embedded aria-orientation is absent on the native targets; a \
                                 vertical embedded control announces as horizontal."
                    .to_owned(),
                evidence_surface: "docs/contracts/components/range-slider.md §12".to_owned(),
                removal_condition: "Lands with the orientation a11y field and native vertical \
                                     orientation."
                    .to_owned(),
                description: "R §12 embedded orientation-channel delta.".to_owned(),
            },
        ],
    }
}

/// The RangeSlider model — the one component, its shared types, the
/// declarative `slider` conformance vector, and nothing else (the shell
/// scene, Button, and synthetic fixtures are untouched).
pub fn range_slider_model() -> IrModel {
    IrModel {
        schema_version: poodle_ir::IR_SCHEMA_VERSION,
        shared_types: vec![
            shared_type(
                "slider-variant",
                "SliderVariant",
                contract_ref("§3"),
                &[
                    ("standard", "Two overlapping native range inputs."),
                    (
                        "embedded",
                        "Dense composite control with adapter-owned focus stops.",
                    ),
                ],
                "The RangeSlider treatment (R §3; the TS SliderVariant union).",
            ),
            shared_type(
                "slider-polarity",
                "SliderPolarity",
                contract_ref("§3"),
                &[
                    ("unipolar", "Ordinary range; no explicit center reference."),
                    (
                        "bipolar",
                        "Range with an explicit bipolar center reference.",
                    ),
                ],
                "The fill-geometry polarity (R §3; the TS SliderPolarity union).",
            ),
            shared_type(
                "slider-thumb",
                "SliderThumb",
                contract_ref("§3"),
                &[
                    ("lower", "The lower bound thumb."),
                    ("upper", "The upper bound thumb."),
                ],
                "Thumb identity — which thumb a gesture holds (R §4 component states).",
            ),
            shared_type(
                "control-size",
                "ControlSize",
                shared_contract_ref(),
                &[
                    ("xs", "Extra-small rung."),
                    ("sm", "Small rung."),
                    ("md", "Default rung."),
                    ("lg", "Large rung."),
                    ("xl", "Extra-large rung."),
                ],
                "The xs-xl control-size ladder (CROSS-07; the TS ControlSize union).",
            ),
            shared_type(
                "control-density",
                "ControlDensity",
                shared_contract_ref(),
                &[
                    ("compact", "Tighter spacing."),
                    ("default", "Standard spacing."),
                    ("comfortable", "Relaxed spacing."),
                ],
                "The compact/default/comfortable density ladder (CROSS-08; the TS \
                 ControlDensity union).",
            ),
            shared_type(
                "control-size-role",
                "SemanticControlSizeRole",
                shared_contract_ref(),
                &[
                    ("chrome", "Shell chrome size role."),
                    ("control", "Standard control size role (default)."),
                    ("prominent", "Prominent size role."),
                ],
                "Semantic size offset from inherited presentation (CROSS-07; the TS \
                 SemanticControlSizeRole union).",
            ),
            shared_type(
                "orientation",
                "Orientation",
                shared_contract_ref(),
                &[
                    (
                        "horizontal",
                        "Layout and interaction along the inline axis.",
                    ),
                    ("vertical", "Layout and interaction along the block axis."),
                ],
                "The orientation axis (CROSS-11, RNG-07; the TS Orientation union).",
            ),
        ],
        components: vec![range_slider_definition()],
        conformance_vectors: vec![ConformanceVector {
            id: ident("slider"),
            name: "slider".to_owned(),
            applies_to: vec![RuntimeTarget::Svelte, RuntimeTarget::React],
            steps: vec![
                VectorStep {
                    id: ident("input-snaps-and-clamps"),
                    name: "INPUT snaps and clamps".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "INPUT normalizes the raw value — snap to step anchored at \
                                  min, then clamp into [min, safeSliderMax] — and emits \
                                  value-change (R §3/§5; machines.json slider entry 0)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("commit-clamps-and-emits"),
                    name: "COMMIT clamps and emits".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "COMMIT normalizes the same way and emits value-commit on \
                                  release (R §3/§5; machines.json slider entry 1)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("set-value-orders-pair"),
                    name: "SET_VALUE orders the pair".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "SET_VALUE normalizes through normalizeRangeValue — the \
                                  display pair is ordered and clamped (R §3, RNG-12)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("thumb-crossing-guard"),
                    name: "a thumb cannot cross its sibling".to_owned(),
                    kind: VectorStepKind::Invariant,
                    description: "Lower clamps to [min, upper], upper to [lower, max]; \
                                  lower <= upper always holds (R §3, RNG-12)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("begin-selects-nearer-thumb"),
                    name: "POINTER_BEGIN selects the nearer thumb".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "Begin picks the thumb nearer the press fraction, holds it \
                                  for the gesture, and the gesture never transfers (R §3/§4, \
                                  RNG-12/13)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("end-commits-pair"),
                    name: "POINTER_END commits the pair".to_owned(),
                    kind: VectorStepKind::EffectIntent,
                    description: "End emits value-commit with the pair and clears the active \
                                  thumb (R §5, RNG-11)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("emit-change-commit-split"),
                    name: "change during interaction, commit on release".to_owned(),
                    kind: VectorStepKind::EffectIntent,
                    description: "INPUT emits value-change; COMMIT/POINTER_END emits \
                                  value-commit — the change/commit callback split (R §5, \
                                  RNG-11)."
                        .to_owned(),
                },
            ],
            description: format!(
                "The two-thumb slider machine semantics both web runtimes honor through the \
                 shared hand-written machines (slider.ts / slider.rs). Executable evidence: \
                 the `slider` key of {VECTOR_EVIDENCE} — a fixed target that must pass \
                 unedited (R5)."
            ),
        }],
        scenes: Vec::new(),
        specimen_registry: None,
    }
}
