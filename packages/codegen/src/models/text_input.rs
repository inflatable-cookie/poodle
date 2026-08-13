//! The authored TextInput definition — `g13.007`'s environment-boundary
//! proof, defined once in Rust (spec 063 "Authoring Form": ordinary Rust
//! types and constructor helpers, no macros), serialized to the JSON
//! fixture the pipeline consumes (`ir:build` / `ir:check` via
//! `load_and_validate`), and emitted to both web packages through the
//! `text-input-ts` target.
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
//! # R2 — the typed capability boundary (this card's deliverable)
//!
//! The definition declares the six environment capabilities the milestone
//! names — focus, selection, composition/IME, clipboard, measurement, and
//! native text editing — plus the component-owned timers. Each is a typed
//! [`Capability`] with a `purpose` naming what it means and which runtime
//! owns it. The boundary is **declared, never implemented**: no runtime may
//! read machine state from drawing code (`IR-06`), and the adapters keep
//! focus/IME/selection/clipboard/measurement (`IR-05`).
//!
//! - **focus** → [`Capability::Focus`]: the web DOM owns focus on the
//!   native input (`autofocus`, the imperative `focus()` method); the Rust
//!   backends own focus observation (`on_focus_change`) and caret drawing,
//!   with `isFocused` a host-driven prop (`T §6` Caret Ownership, `TXT-21`).
//! - **selection** → [`Capability::TextEditing`] + [`Capability::Measurement`]:
//!   there is **no** `Capability::Selection` — selection is not a first-class
//!   capability name in the inventory (`CROSS-17`), and the ownership split
//!   is the milestone's asymmetry: the web DOM owns selection entirely (the
//!   web components contain zero selection code — measured), while on the
//!   Rust targets the host owns the caret position (`selectionStart`/
//!   `selectionEnd` controlled props, `onSelectionChange`), the backend owns
//!   drawing it (glyph measurement), and the shared edit model owns the
//!   semantics (`edit_transition`, `word_range_at`, `selected_text`). The
//!   capability is typed (the two enums); the per-runtime ownership is
//!   prose in `purpose` — there is no typed per-runtime ownership field in
//!   the IR. Recorded for `g13.008` (R3 question 2).
//! - **composition/IME** → [`Capability::Ime`]: the web DOM owns composition
//!   natively (no composition listeners in the components — a composition
//!   sequence must not fire intermediate `onValueChange`; the runtime
//!   filters input events by `isComposing`); the Rust backends register a
//!   platform text input handler with a UTF-16 boundary and backend-owned
//!   marked range (`T §6`, `TXT-24`).
//! - **clipboard** → [`Capability::Clipboard`]: the web DOM owns copy/cut/
//!   paste natively; the Rust backends own the platform clipboard and the
//!   shared edit model owns paste landing (`insert_transition`) and the
//!   copy/cut source (`selected_text`) (`T §6`, `TXT-23`).
//! - **measurement** → [`Capability::Measurement`]: the browser measures
//!   natively; the Rust backends measure shaped glyphs
//!   (`shape_line`/`x_for_index`, `closest_index_for_x`) for caret placement
//!   (`T §6`, `TXT-21/22`).
//! - **native text editing** → [`Capability::TextEditing`]: the web
//!   runtime's editing model **is the browser** (there is no TS text
//!   machine — see R5); the Rust targets drive the shared headless edit
//!   model from key events (`edit_transition`) and insertions
//!   (`insert_transition`).
//!
//! The web half records the boundary now; card 049 consumes it for the
//! natives. No stop condition was reached: every capability has a typed
//! [`Capability`] name (selection rides on `TextEditing` + `Measurement` —
//! recorded, not routed around), and declaring the boundary generates no
//! lifecycle code.
//!
//! # R3 — the asymmetry question (answered in the batch log)
//!
//! The module records what the definition can and cannot express; the
//! batch log answers R3's three questions with the measured numbers.
//!
//! # R5 — the editing model stays hand-written
//!
//! `packages/contracts/headless/src/text_input.rs` is not going into the
//! IR, and no TS counterpart gets invented (the card forbids it — the
//! absence of `packages/core/src/text-input.ts` is by design, b047
//! baselined `rs:text_input` as *correctly different*). The definition
//! declares the semantics through the `text-input` conformance vector
//! (step intents, `CROSS-18`) and names the evidence: the headless edit
//! model's own unit tests. `machines.json` carries **no text key** — the
//! vector is a fixed target (R5) and the text machine is
//! unit-test-pinned only; that gap is recorded in the vector description
//! for `g13.008`.
//!
//! # R4 — this is re-plumbing
//!
//! The 49 web props keep their names, types, and defaults; the 3
//! contract-documented data attributes (`data-size`, `data-density`,
//! `data-validation-state`) keep their values — and `data-type`, the
//! fourth emitted attribute (documented only in the corpus row TXT-18,
//! not in the contract prose), keeps its value too. `text-input.css` is
//! untouched. The web surface is verified by `svelte:surface-audit`,
//! `docs:contract-drift`, the parity test's class-set diff, and the
//! existing TextInput tests.
//!
//! # Vocabulary notes recorded for g13.008
//!
//! - **The controlled pair is DoNotMix, unlike RangeSlider.** The contract
//!   says "do not mix controlled and uncontrolled modes simultaneously"
//!   (T §3), so `controlled_state` carries the pair with `DoNotMix` — the
//!   shape RangeSlider deliberately avoided because React's pair is
//!   controlled-wins (b045 vocabulary note). TextInput's contract rule is
//!   the IR's only rule.
//! - **The 49 web props + 3 Rust-only props.** The card counts 49 props
//!   from the Svelte `Props` interface. The contract §3 table also
//!   documents `selectionStart`/`selectionEnd`/`isFocused` as **Rust
//!   targets only** (`TXT-29` carries them in `TextInputSpec`); the IR has
//!   no rust-only flag, so the definition records them as portable props
//!   with the "Rust targets only" note — 52 props total, the card's 49-web
//!   count preserved and the trio recorded as the Rust extension of the
//!   portable surface.
//! - **Four emitted data attributes, three contract-documented.** The DOM
//!   emits `data-validation-state`, `data-size`, `data-density`, and
//!   `data-type`. The contract §9 documents three (the card's counting
//!   authority); the corpus row TXT-18 documents the fourth. The definition
//!   carries all four emitted names.
//! - **The web emits five TXT-16 style props; React emits three.** Svelte
//!   emits `--poodle-text-input-clear-inset-inline-end` and
//!   `--poodle-text-input-trailing-inset-inline-end` (positioning the
//!   search/validation overlay chrome); React leaves the CSS fallbacks
//!   (`0.5rem`) — a pre-existing web-side DOM asymmetry, recorded in the
//!   R7 inventory (React reads the three shared style props from the
//!   artifact; the two overlay insets are Svelte-only emissions).
//! - **Two callback surfaces have no EventKind.** `onValidationChange`
//!   (payload `{status, valid, message}` — `PayloadKind::ValidationStatus`
//!   exists but no `EventKind` uses it) and `onKeyDown` (native passthrough)
//!   cannot be typed as events; `onFocus`/`onBlur` map to `FocusChange`.
//!   Recorded as a finding for `g13.008`, not routed around.
//! - **IME filtering is the one behavior the wiring adds.** The measured
//!   table showed zero composition handling in the web runtimes; the
//!   card's required IME test exposed that the components fired
//!   intermediate `onValueChange` during composition (the browser fires
//!   `input` events with `isComposing: true` for each composition update).
//!   The wiring adds a one-line `isComposing` filter to `handleInput` in
//!   both runtimes — the composition itself stays 100% browser-native (no
//!   `compositionstart`/`update`/`end` interception), preserving the
//!   "handled not at all" shape the measured table records. No prop,
//!   attribute, or pixel moves.
//! - **The text machine is not vector-pinned.** `machines.json` has no
//!   `text` key (fixed target, R5); the edit model is pinned by the
//!   headless unit tests only. The `slider` vector thinness finding (b045
//!   R2.2) generalizes to: *no text vector exists at all*.

use poodle_ir::{
    A11yRole, Accessibility, AriaMapping, AttributeForm, Axes, Capability, CapabilityRequirement,
    ComponentDefinition, ConformanceVector, ContractRef, ControlDensity, ControlSize,
    DensityAdjustment, DensityAxis, EmissionPolicy, Event, EventKind, EventPayload, EventTiming,
    Extension, FiringPhase, Identifier, IrModel, KeyChord, KeyboardCommand, Layer,
    MetricValue, Modifier, NameRule, NameSource, NativeAttr, Part, PartKind, PayloadKind,
    PermittedSubset, Prop, PropType, RecipeHookRef, RecipeLink, RecipeLinkKind, RuntimeTarget,
    SharedEnumMember, SharedType, SizeAxis, SizeRole, SizeStep, StateAttribute, TokenGroup,
    TokenRef, Value, VectorStep, VectorStepKind, VisualFieldKind, VisualState, VisualStateField,
};

/// The governing contract, cited by the component and every definition row.
const CONTRACT: &str = "docs/contracts/components/text-input.md";

/// The cross-component control types (004), referenced by the shared types
/// this model declares.
const SHARED_CONTRACT: &str = "docs/contracts/004-shared-control-types.md";

/// The executable conformance-vector evidence (R5 — a fixed target). The
/// text machine is **not** pinned here: `machines.json` carries no `text`
/// key (GAP-01), and the vectors are a fixed target — the edit model's
/// evidence is its own unit tests (see `text_input_model`).
const VECTOR_EVIDENCE: &str = "packages/contracts/headless/vectors/machines.json";

/// The shared edit model — the hand-written machine this definition
/// declares but does not absorb (R5).
const EDIT_MODEL: &str = "packages/contracts/headless/src/text_input.rs";

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
/// on a shared-typed prop, or an opaque payload no `Value` variant can
/// carry (see module notes).
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
        "text-input-type" => vec!["text", "multiline", "search", "slug"],
        "validation-state" => vec!["none", "invalid", "valid", "pending"],
        "input-mode" => vec![
            "none", "search", "text", "tel", "url", "email", "numeric", "decimal",
        ],
        "resize-direction" => vec!["vertical", "horizontal", "both", "none"],
        "enter-key-hint" => vec!["enter", "done", "go", "next", "previous", "search", "send"],
        "autocorrect-mode" => vec!["on", "off"],
        "control-size" => vec!["xs", "sm", "md", "lg", "xl"],
        "control-density" => vec!["compact", "default", "comfortable"],
        "control-size-role" => vec!["chrome", "control", "prominent"],
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

/// A VisualState projection field (`CROSS-14`, `TXT-19`).
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

/// Builds one recipe-hook override chain (`CROSS-09`, `TXT-27`).
fn recipe_hook(hook: &str, chain: Vec<RecipeLink>, description: &str) -> RecipeHookRef {
    RecipeHookRef {
        hook: hook.to_owned(),
        chain,
        description: description.to_owned(),
    }
}

/// A recipe hook whose fallback is a terminal token, transcribed from the
/// stylesheet's `var()` fallback.
fn recipe_hook_token(hook: &str, token: &str, description: &str) -> RecipeHookRef {
    recipe_hook(
        hook,
        vec![
            RecipeLink {
                kind: RecipeLinkKind::RecipeHook,
                target: hook.to_owned(),
            },
            RecipeLink {
                kind: RecipeLinkKind::Token,
                target: token.to_owned(),
            },
        ],
        description,
    )
}

/// The 6 recipe hooks of `text-input.css` (TXT-27; the contract §8
/// recipe-hook table). Each hook's fallback is transcribed from the
/// stylesheet's `var()` fallback; the focus-variant hooks fall back through
/// the component variable chain (hook → component variable → recipe hook →
/// token), the Button-style chain.
fn recipe_hooks() -> Vec<RecipeHookRef> {
    vec![
        recipe_hook_token(
            "--poodle-recipe-text-input-fill",
            "color.background.surface",
            "Field fill; surface 88% color-mix base (T §8, TXT-27).",
        ),
        recipe_hook(
            "--poodle-recipe-text-input-fill-focus",
            vec![
                RecipeLink {
                    kind: RecipeLinkKind::RecipeHook,
                    target: "--poodle-recipe-text-input-fill-focus".to_owned(),
                },
                RecipeLink {
                    kind: RecipeLinkKind::ComponentVariable,
                    target: "--poodle-text-input-fill".to_owned(),
                },
                RecipeLink {
                    kind: RecipeLinkKind::RecipeHook,
                    target: "--poodle-recipe-text-input-fill".to_owned(),
                },
                RecipeLink {
                    kind: RecipeLinkKind::Token,
                    target: "color.background.surface".to_owned(),
                },
            ],
            "Focus fill; falls back through the component variable to the fill \
             recipe (T §8, TXT-27).",
        ),
        recipe_hook_token(
            "--poodle-recipe-text-input-border",
            "color.border.default",
            "Field border (T §8, TXT-27).",
        ),
        recipe_hook_token(
            "--poodle-recipe-text-input-border-focus",
            "color.accent.focusRing",
            "Focus border (T §8, TXT-27).",
        ),
        recipe_hook(
            "--poodle-recipe-text-input-shadow",
            vec![RecipeLink {
                kind: RecipeLinkKind::RecipeHook,
                target: "--poodle-recipe-text-input-shadow".to_owned(),
            }],
            "Resting shadow; the fallback is the literal `none` — no semantic \
             token exists, so the chain records the hook alone (T §8, TXT-27).",
        ),
        recipe_hook_token(
            "--poodle-recipe-text-input-shadow-focus",
            "color.accent.focusRing",
            "Focus shadow — `0 0 0 var(--poodle-border-width-focus) color-mix(... \
             focusRing 28%, transparent)`; the focusRing path names the family \
             (T §8, TXT-27).",
        ),
    ]
}

/// Builds a semantic [`TokenRef`] (`CROSS-09`, `TXT-27`).
fn token(path: &str, description: &str) -> TokenRef {
    TokenRef {
        path: path.to_owned(),
        group: TokenGroup::Semantic,
        description: description.to_owned(),
    }
}

/// The per-rung size metrics (T §8, TXT-15): min-height, inline padding,
/// and font-size. The contract's table expresses min-height and padding as
/// token-relative `calc()` expressions, so the metrics are recorded as
/// `Text` values transcribed from the table; the font-size rungs that are
/// fixed rems are recorded as `Rem`.
fn size_metrics(
    min_height: &str,
    padding: &str,
    font_size: MetricValue,
) -> std::collections::BTreeMap<String, MetricValue> {
    let mut metrics = std::collections::BTreeMap::new();
    metrics.insert(
        "min-height".to_owned(),
        MetricValue::Text(min_height.to_owned()),
    );
    metrics.insert("padding".to_owned(), MetricValue::Text(padding.to_owned()));
    metrics.insert("font-size".to_owned(), font_size);
    metrics
}

/// The `g13.007` TextInput definition — the environment-boundary proof:
/// 49 web props against 4 emitted data attributes (~16:1 vs Button's ~3:1,
/// the ratio this card exists to measure), a typed capability boundary for
/// focus/selection/IME/clipboard/measurement/text-editing (R2), and the
/// asymmetry between a web runtime whose editing model *is* the browser and
/// a Rust runtime that implements one (R3).
pub fn text_input_definition() -> ComponentDefinition {
    ComponentDefinition {
        id: ident("text-input"),
        name: "TextInput".to_owned(),
        layer: Layer::Foundation,
        contract: contract_ref("§3"),
        description: "A single-line text entry control with explicit value, validation, \
                      focus, and submission semantics; prefix/suffix affixes, character \
                      counting, built-in async validation status indicators, and \
                      leading/trailing affordance slots; multiline/search/slug modes \
                      (T §1). g13.007's environment-boundary proof: 49 web props against 3 \
                      contract-documented data attributes, a typed capability boundary for \
                      focus/selection/IME/clipboard/measurement/text-editing, and a \
                      hand-written editing model declared but not absorbed (TXT-01..32)."
            .to_owned(),

        // 49 web props + 3 Rust-only props (selectionStart/selectionEnd/
        // isFocused — T §3 "Rust targets only"). Order is the contract's
        // §3 table order. The 8 callbacks are declared as events below;
        // `onValidationChange` and `onKeyDown` have no EventKind and are
        // recorded as findings (see the module notes).
        props: vec![
            prop(
                "id",
                PropType::String,
                Some(Value::string("")),
                false,
                "Element id for label association; the contract marks it required while \
                 the Svelte interface defaults it to \"\" — the generated fallback id \
                 covers the missing case (T §3/§6, TXT-01).",
            ),
            prop(
                "value",
                PropType::String,
                Some(Value::Null),
                false,
                "Host-owned value when supplied; null remains a valid controlled empty \
                 state (T §3, TXT-02).",
            ),
            prop(
                "defaultValue",
                PropType::String,
                Some(Value::string("")),
                false,
                "Uncontrolled initial value; the pair is do-not-mix (T §3, TXT-02).",
            ),
            prop(
                "placeholder",
                PropType::String,
                Some(Value::Null),
                false,
                "Hint text when empty; placeholder never counts as the accessible name \
                 (T §3/§6, TXT-03).",
            ),
            prop(
                "name",
                PropType::String,
                Some(Value::Null),
                false,
                "Form submission name; native passthrough (T §3, TXT-04).",
            ),
            prop(
                "autocomplete",
                PropType::String,
                Some(Value::Null),
                false,
                "Native autocomplete attribute (T §3/§6, TXT-04).",
            ),
            bool_prop(
                "disabled",
                "Disables editing and interaction; native attribute plus disabled opacity \
                 (T §3/§4/§6, TXT-05).",
            ),
            bool_prop(
                "readOnly",
                "Allows selection without editing; native readonly (not aria-readonly) \
                 (T §3/§6, TXT-05).",
            ),
            prop(
                "autofocus",
                PropType::Bool,
                Some(Value::boolean(false)),
                true,
                "Web-only native autofocus attribute; falsy omits the attribute. Excluded \
                 from TextInputSpec alongside the other native attributes (T §3, TXT-04; \
                 CROSS-03).",
            ),
            prop(
                "selectionStart",
                PropType::Number,
                Some(Value::number(0.0)),
                false,
                "Caret/selection start, in characters. RUST TARGETS ONLY — the web DOM \
                 owns selection and the web components have no such prop (T §3/§6, \
                 TXT-21).",
            ),
            prop(
                "selectionEnd",
                PropType::Number,
                Some(Value::number(0.0)),
                false,
                "Caret/selection end; equal to selectionStart means a plain caret. RUST \
                 TARGETS ONLY (T §3/§6, TXT-21).",
            ),
            prop(
                "isFocused",
                PropType::Bool,
                Some(Value::boolean(false)),
                false,
                "Whether the field holds focus, so the caret is drawn. RUST TARGETS ONLY \
                 — the backend draws a caret only in the field that actually holds focus \
                 (T §3/§6, TXT-21).",
            ),
            bool_prop("required", "Native required attribute (T §3/§6, TXT-04)."),
            prop(
                "pattern",
                PropType::String,
                Some(Value::Null),
                false,
                "Native pattern attribute (T §3/§6, TXT-04).",
            ),
            prop(
                "spellcheck",
                PropType::Bool,
                Some(Value::Null),
                true,
                "Native spellcheck attribute; web-only surface (T §3/§6, TXT-04; CROSS-03).",
            ),
            prop(
                "autocapitalize",
                PropType::String,
                Some(Value::Null),
                true,
                "Native autocapitalize attribute; web-only surface (T §3/§6, TXT-04; \
                 CROSS-03).",
            ),
            prop(
                "autocorrect",
                shared("autocorrect-mode"),
                None,
                true,
                "Native autocorrection attribute; web-only surface. OBS-03: React's \
                 surface omits autocorrect entirely — recorded as an extension (T §3, \
                 TXT-04; CROSS-03).",
            ),
            prop(
                "enterKeyHint",
                shared("enter-key-hint"),
                None,
                true,
                "Native enterkeyhint attribute; web-only surface (T §3, TXT-04; CROSS-03).",
            ),
            prop(
                "debounce",
                PropType::Number,
                Some(Value::Null),
                false,
                "Delays onValueChange while typing; flush on blur; immediate for clear and \
                 slug source regeneration (T §3/§5, TXT-11/28; debounce_ms).",
            ),
            prop(
                "validate",
                PropType::Opaque,
                None,
                true,
                "Optional validator function; async validation orchestration is \
                 web-component-owned — TextInputSpec carries no validator (T §3, TXT-12; \
                 CROSS-03).",
            ),
            prop(
                "validationContext",
                PropType::Opaque,
                None,
                true,
                "App-owned opaque context passed to validate (T §3, TXT-12; CROSS-03).",
            ),
            prop(
                "validationKey",
                PropType::Opaque,
                None,
                true,
                "Stable value merged into the validation context so validation re-runs \
                 after identity changes (T §3, TXT-12; CROSS-03).",
            ),
            prop(
                "validationDebounce",
                PropType::Number,
                Some(Value::number(300.0)),
                true,
                "Delay before validation runs while typing; web-only (T §3, TXT-12; \
                 CROSS-03).",
            ),
            prop(
                "validateOnBlur",
                PropType::Bool,
                Some(Value::boolean(true)),
                true,
                "Whether blur triggers immediate validation; web-only (T §3, TXT-12; \
                 CROSS-03).",
            ),
            prop(
                "showValidationStatus",
                PropType::Bool,
                Some(Value::boolean(true)),
                false,
                "Whether built-in validation status chrome is shown (T §3/§4, TXT-12; \
                 shows_validation_status).",
            ),
            prop(
                "validationState",
                shared("validation-state"),
                Some(Value::member("none")),
                false,
                "Caller-owned visual and assistive validation state; built-in validation \
                 maps idle→caller/validating→pending/valid→valid/invalid→invalid (T §3/§4, \
                 TXT-12).",
            ),
            prop(
                "ariaLabel",
                PropType::String,
                Some(Value::Null),
                false,
                "Accessible name; required when no external label exists (T §3/§6, \
                 TXT-26).",
            ),
            prop(
                "describedBy",
                PropType::String,
                Some(Value::Null),
                false,
                "aria-describedby target; the built-in validation-message id joins it \
                 (T §3/§6, TXT-12/26).",
            ),
            prop(
                "inputMode",
                shared("input-mode"),
                None,
                false,
                "Virtual keyboard hint (T §3/§6, TXT-04; input_mode).",
            ),
            prop(
                "list",
                PropType::String,
                Some(Value::Null),
                true,
                "Datalist id for native suggestion lists; passed as the native list \
                 attribute; web-only surface (T §3/§6, TXT-04; CROSS-03).",
            ),
            prop(
                "type",
                shared("text-input-type"),
                Some(Value::member("text")),
                false,
                "Input type attribute; \"multiline\" renders a textarea, \"search\" search \
                 mode, \"slug\" semantic slug entry rendered as a native text input; other \
                 native HTML types pass through web-only (T §3, TXT-06; input_type).",
            ),
            prop(
                "rows",
                PropType::Number,
                Some(Value::Null),
                false,
                "Visible text rows; > 1 with the default type auto-switches to multiline; \
                 textarea defaults to 4 rows (T §3, TXT-06/07).",
            ),
            prop(
                "resize",
                shared("resize-direction"),
                Some(Value::member("vertical")),
                false,
                "Native resize handle direction in multiline mode (T §3, TXT-07).",
            ),
            prop(
                "source",
                PropType::String,
                Some(Value::Null),
                false,
                "Slug source text for auto-generation until the user meaningfully edits \
                 (T §3, TXT-09).",
            ),
            prop(
                "showClearButton",
                PropType::Bool,
                Some(Value::boolean(true)),
                false,
                "Search-mode clear button visibility; default true (T §3, TXT-08; \
                 show_clear_button).",
            ),
            prop(
                "prefix",
                PropType::String,
                Some(Value::Null),
                false,
                "Static non-editable text before the input, excluded from the editable \
                 value (T §2/§3/§6/§8, TXT-10).",
            ),
            prop(
                "suffix",
                PropType::String,
                Some(Value::Null),
                false,
                "Static non-editable text after the input, excluded from the editable \
                 value (T §2/§3/§6/§8, TXT-10).",
            ),
            prop(
                "maxLength",
                PropType::Number,
                Some(Value::Null),
                false,
                "Maximum character count (T §3, TXT-14; max_length).",
            ),
            bool_prop(
                "showCharCount",
                "Live character counter (T §3, TXT-14; show_char_count).",
            ),
            prop(
                "size",
                shared("control-size"),
                None,
                false,
                "Explicit control-size override; when null resolves from inherited \
                 presentation plus sizeRole (T §3, TXT-15; CROSS-07).",
            ),
            prop(
                "sizeRole",
                shared("control-size-role"),
                Some(Value::member("control")),
                false,
                "Semantic size offset from inherited presentation (T §3, TXT-15; \
                 CROSS-07).",
            ),
            prop(
                "density",
                shared("control-density"),
                None,
                false,
                "Explicit density override; when null inherited from presentation \
                 (T §3, TXT-15; CROSS-08).",
            ),
            prop(
                "leading",
                PropType::Opaque,
                None,
                true,
                "Leading affordance snippet — web slot only; the Rust spec's leading \
                 affordance is an icon name, a different surface (T §3, TXT-17; CROSS-03).",
            ),
            prop(
                "trailing",
                PropType::Opaque,
                None,
                true,
                "Trailing affordance snippet — web slot only (T §3, TXT-17; CROSS-03).",
            ),
        ],

        // T §3 "do not mix controlled and uncontrolled modes
        // simultaneously" (TXT-02; CROSS-04). Unlike RangeSlider's
        // controlled-wins pair (b045 note), TextInput's contract rule is
        // exactly the IR's DoNotMix rule.
        controlled_state: vec![poodle_ir::ControlledState {
            id: ident("value"),
            controlled: ident("value"),
            seed: ident("defaultValue"),
            rule: poodle_ir::ControlRule::DoNotMix,
            description: "Controlled `value` + onValueChange, or `defaultValue` seeding \
                          the uncontrolled mode; do not mix modes simultaneously (T §3, \
                          TXT-02; CROSS-04)."
                .to_owned(),
        }],

        // The expressible event surface (T §5, TXT-13). onValidationChange
        // has no EventKind (PayloadKind::ValidationStatus exists but no
        // kind uses it) and onKeyDown is a native passthrough — both are
        // recorded as findings in the module notes, not typed dishonestly.
        events: vec![
            Event {
                id: ident("value-change"),
                name: "onValueChange".to_owned(),
                kind: EventKind::ValueChange,
                payload: Some(EventPayload {
                    name: "value".to_owned(),
                    kind: PayloadKind::String,
                }),
                timing: EventTiming {
                    phase: FiringPhase::DuringInteraction,
                    flush_on_blur: true,
                    ordering: vec![poodle_ir::OrderingConstraint {
                        before: ident("value-change"),
                        after: ident("blur"),
                        reason: "Blur flushes a pending debounced value-change before the \
                                 blur passthrough fires (T §5, TXT-11/28)."
                            .to_owned(),
                    }],
                    ..EventTiming::default()
                },
                description: "Fires per input change respecting debounce; flush on blur; \
                              immediate for clear and slug source regeneration (T §5, \
                              TXT-11/13/28)."
                    .to_owned(),
            },
            Event {
                id: ident("submit"),
                name: "onSubmit".to_owned(),
                kind: EventKind::Submit,
                payload: Some(EventPayload {
                    name: "value".to_owned(),
                    kind: PayloadKind::String,
                }),
                timing: EventTiming {
                    phase: FiringPhase::DuringInteraction,
                    ..EventTiming::default()
                },
                description: "Fires on Enter, or Cmd/Ctrl+Enter in multiline mode, with \
                              the current value (T §5, TXT-13/20)."
                    .to_owned(),
            },
            Event {
                id: ident("cancel"),
                name: "onCancel".to_owned(),
                kind: EventKind::Cancel,
                payload: None,
                timing: EventTiming {
                    phase: FiringPhase::DuringInteraction,
                    ..EventTiming::default()
                },
                description: "Fires on Escape with no payload (T §5, TXT-13/20).".to_owned(),
            },
            Event {
                id: ident("clear"),
                name: "onClear".to_owned(),
                kind: EventKind::Clear,
                payload: None,
                timing: EventTiming {
                    phase: FiringPhase::Immediate,
                    ..EventTiming::default()
                },
                description: "Fires when the built-in search clear button is used; the \
                              value commits immediately (T §5, TXT-08/13)."
                    .to_owned(),
            },
            Event {
                id: ident("focus"),
                name: "onFocus".to_owned(),
                kind: EventKind::FocusChange,
                payload: None,
                timing: EventTiming {
                    phase: FiringPhase::DuringInteraction,
                    ..EventTiming::default()
                },
                description: "Native focus passthrough (T §5, TXT-13).".to_owned(),
            },
            Event {
                id: ident("blur"),
                name: "onBlur".to_owned(),
                kind: EventKind::FocusChange,
                payload: None,
                timing: EventTiming {
                    phase: FiringPhase::OnBlur,
                    ..EventTiming::default()
                },
                description: "Native blur passthrough; flushes a pending debounce and runs \
                              immediate validation when validateOnBlur (T §5, TXT-11/13/28)."
                    .to_owned(),
            },
            Event {
                id: ident("selection-change"),
                name: "onSelectionChange".to_owned(),
                kind: EventKind::SelectionChange,
                payload: Some(EventPayload {
                    name: "[anchor, head]".to_owned(),
                    kind: PayloadKind::Pair,
                }),
                timing: EventTiming {
                    phase: FiringPhase::DuringInteraction,
                    ..EventTiming::default()
                },
                description: "Caret/selection reported on Rust targets only — the host \
                              stores it like TreeSpec::focused_value; the web DOM owns \
                              selection and reports nothing (T §6, TXT-21)."
                    .to_owned(),
            },
        ],

        // The anatomy (T §2 + the rendered DOM): the contract's ten parts.
        // The optional parts carry their render conditions as documented
        // prose on the conditional part kind (g13.017 R1 bucket 2: anatomy
        // kept, expression tree gone); the input control is ONE part that
        // renders <input> or <textarea> — the multiline element variant
        // and its modifier class are a hand-written runtime branch
        // (recorded in the R7 inventory).
        parts: vec![
            Part {
                id: ident("root"),
                name: "Root".to_owned(),
                parent: None,
                kind: PartKind::Static,
                description: "Field chrome container with flex layout (T §2, TXT-17).".to_owned(),
            },
            Part {
                id: ident("prefix"),
                name: "Prefix".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::ConditionalDocumented {
                    condition: "present when the prefix prop holds a non-empty value"
                        .to_owned(),
                    description: "Non-editable text prefix with separator border \
                                  (T §2, TXT-10/17)."
                        .to_owned(),
                },
                description: "Static prefix text (T §2).".to_owned(),
            },
            Part {
                id: ident("field"),
                name: "Field".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::Static,
                description: "Relative positioning layer for the editable surface and \
                              overlaid chrome (T §2, TXT-17)."
                    .to_owned(),
            },
            Part {
                id: ident("leading-affordance"),
                name: "Leading Affordance".to_owned(),
                parent: Some(ident("field")),
                kind: PartKind::ConditionalDocumented {
                    condition: "present when the leading slot is provided or the type is \
                                search (automatic search icon)"
                        .to_owned(),
                    description: "Icon or adornment inside the editable field's leading edge \
                                  (T §2, TXT-08/17)."
                        .to_owned(),
                },
                description: "Leading affordance slot (T §2).".to_owned(),
            },
            Part {
                id: ident("input-control"),
                name: "Input Control".to_owned(),
                parent: Some(ident("field")),
                kind: PartKind::Static,
                description: "Native input element — <input> single-line or <textarea> \
                              multiline; the element switch and the --multiline modifier \
                              class are a hand-written runtime branch (T §2, TXT-06/07/17)."
                    .to_owned(),
            },
            Part {
                id: ident("trailing-affordance"),
                name: "Trailing Affordance".to_owned(),
                parent: Some(ident("field")),
                kind: PartKind::ConditionalDocumented {
                    condition: "present when the trailing slot is provided".to_owned(),
                    description: "Icon or action inside the editable field's trailing edge \
                                  (T §2, TXT-17)."
                        .to_owned(),
                },
                description: "Trailing affordance slot (T §2).".to_owned(),
            },
            Part {
                id: ident("clear-button"),
                name: "Clear Button".to_owned(),
                parent: Some(ident("field")),
                kind: PartKind::ConditionalDocumented {
                    condition: "present in search mode with a value when not \
                                disabled/read-only"
                        .to_owned(),
                    description: "Search clear action inside the field's trailing edge \
                                  (T §2, TXT-08/17)."
                        .to_owned(),
                },
                description: "Search-mode clear button (T §2).".to_owned(),
            },
            Part {
                id: ident("validation-indicator"),
                name: "Validation Indicator".to_owned(),
                parent: Some(ident("field")),
                kind: PartKind::ConditionalDocumented {
                    condition: "present when validation chrome is enabled and the effective \
                                state is not none"
                        .to_owned(),
                    description: "Pending spinner or valid/invalid status icon overlaid \
                                  inside the field (T §2, TXT-12/17/19)."
                        .to_owned(),
                },
                description: "Built-in validation indicator (T §2).".to_owned(),
            },
            Part {
                id: ident("suffix"),
                name: "Suffix".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::ConditionalDocumented {
                    condition: "present when the suffix prop holds a non-empty value"
                        .to_owned(),
                    description: "Non-editable text suffix with separator border \
                                  (T §2, TXT-10/17)."
                        .to_owned(),
                },
                description: "Static suffix text (T §2).".to_owned(),
            },
            Part {
                id: ident("char-count"),
                name: "Character Count".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::Conditional {
                    when: ident("showCharCount"),
                    description: "Live character count, optionally with max; present when \
                                  showCharCount (T §2, TXT-14/17)."
                        .to_owned(),
                },
                description: "Live character counter (T §2).".to_owned(),
            },
        ],

        // The 4 emitted data-* attributes (TXT-18; T §9 documents three —
        // data-validation-state, data-size, data-density — and the corpus
        // row documents the fourth, data-type) plus the 5 TXT-16 style
        // custom properties. Names, forms, emission policies, and value
        // domains are the rendered vocabulary the `text-input-ts` artifact
        // carries (R2); the style props are emitted as styleProps, not DOM
        // attributes.
        attributes: vec![
            valued_attribute(
                "validation-state",
                "data-validation-state",
                "effectiveValidationState",
                EmissionPolicy::Always,
                "The effective validation state — the validationStatusToState mapping \
                 when a validator is present, otherwise the caller-owned validationState; \
                 always emitted (T §9, TXT-18/19).",
            ),
            valued_attribute(
                "size",
                "data-size",
                "resolvedSize",
                EmissionPolicy::Always,
                "The resolved control size (explicit or sizeRole-derived); always emitted \
                 (T §9, TXT-15/18; CROSS-07).",
            ),
            valued_attribute(
                "density",
                "data-density",
                "resolvedDensity",
                EmissionPolicy::Always,
                "The resolved density (explicit or inherited); always emitted (T §9, \
                 TXT-15/18; CROSS-08).",
            ),
            valued_attribute(
                "type",
                "data-type",
                "type",
                EmissionPolicy::Always,
                "The type prop value; always emitted — documented in the corpus row \
                 TXT-18, not in the contract prose (T §9 counting note, TXT-18).",
            ),
            // The TXT-16 adornment-padding reservation: computed custom
            // properties fed by the runtime's derived strings (the calc()
            // arithmetic is not vocabulary); `source` names the runtime
            // field. Svelte emits all five; React emits the three shared
            // ones (the overlay insets fall back to 0.5rem in the
            // stylesheet — see the module notes and the R7 inventory).
            valued_attribute(
                "control-padding-start",
                "--poodle-text-input-control-padding-start",
                "controlPaddingStart",
                EmissionPolicy::Always,
                "Start padding reservation so text never runs under the leading \
                 affordance (T §8, TXT-16).",
            ),
            valued_attribute(
                "control-padding-end",
                "--poodle-text-input-control-padding-end",
                "controlPaddingEnd",
                EmissionPolicy::Always,
                "End padding reservation for the trailing adornment count (T §8, TXT-16).",
            ),
            valued_attribute(
                "multiline-padding-end",
                "--poodle-text-input-multiline-padding-end",
                "multilineBottomPadding",
                EmissionPolicy::Always,
                "Multiline bottom padding reservation so typed text never runs under the \
                 overlaid char count (T §8, TXT-07/16).",
            ),
            valued_attribute(
                "clear-inset-inline-end",
                "--poodle-text-input-clear-inset-inline-end",
                "clearInsetInlineEnd",
                EmissionPolicy::Always,
                "Clear button inline-end inset, pushed left of the validation indicator \
                 when both render (T §8, TXT-16).",
            ),
            valued_attribute(
                "trailing-inset-inline-end",
                "--poodle-text-input-trailing-inset-inline-end",
                "trailingInsetInlineEnd",
                EmissionPolicy::Always,
                "Trailing affordance inline-end inset (T §8, TXT-16).",
            ),
        ],

        // Axes (CROSS-07/08): the size ladder with the contract's §8 table
        // and the density adjustments with the documented §8 orthogonality
        // exception (block padding changes the effective vertical text
        // inset). No orientation axis — TextInput has none.
        axes: Axes {
            size: Some(SizeAxis {
                explicit: None,
                size_role: SizeRole::Control,
                ladder: vec![
                    SizeStep {
                        size: ControlSize::Xs,
                        metrics: size_metrics(
                            "calc(control-height - 0.5rem)",
                            "0 calc(space-control-x - 0.125rem)",
                            MetricValue::Rem(0.75),
                        ),
                        description: "Extra-small rung (T §8, TXT-15).".to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Sm,
                        metrics: size_metrics(
                            "calc(control-height - 0.375rem)",
                            "0 calc(space-control-x - 0.0625rem)",
                            MetricValue::Text("typography-body-size".to_owned()),
                        ),
                        description: "Small rung (T §8, TXT-15).".to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Md,
                        metrics: size_metrics(
                            "control-height",
                            "0 space-control-x",
                            MetricValue::Text("typography-body-size".to_owned()),
                        ),
                        description: "Default rung (T §8, TXT-15).".to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Lg,
                        metrics: size_metrics(
                            "calc(control-height + 0.375rem)",
                            "0 calc(space-control-x + 0.125rem)",
                            MetricValue::Rem(0.9375),
                        ),
                        description: "Large rung (T §8, TXT-15).".to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Xl,
                        metrics: size_metrics(
                            "calc(control-height + 0.5rem)",
                            "0 calc(space-control-x + 0.1875rem)",
                            MetricValue::Rem(1.0),
                        ),
                        description: "Extra-large rung (T §8, TXT-15).".to_owned(),
                    },
                ],
            }),
            density: Some(DensityAxis {
                explicit: None,
                adjustments: vec![
                    DensityAdjustment {
                        density: ControlDensity::Compact,
                        applies_to: Some(ident("root")),
                        inline: Some(MetricValue::Rem(-0.125)),
                        block: Some(MetricValue::Rem(-0.0625)),
                        description: "Compact shifts inline and block padding via the \
                                      density adjustment vars — the documented §8 \
                                      orthogonality exception: block padding changes the \
                                      effective vertical text inset on the control \
                                      (T §8, TXT-15)."
                            .to_owned(),
                    },
                    DensityAdjustment {
                        density: ControlDensity::Comfortable,
                        applies_to: Some(ident("root")),
                        inline: Some(MetricValue::Rem(0.125)),
                        block: Some(MetricValue::Rem(0.0625)),
                        description: "Comfortable shifts inline and block padding the other \
                                      way (T §8, TXT-15)."
                            .to_owned(),
                    },
                ],
            }),
            orientation: None,
        },

        // TXT-27: the semantic tokens the appearance consumes, resolved
        // against the generated poodle-tokens registry (CROSS-09).
        tokens: vec![
            token("color.background.surface", "Field fill (T §8)."),
            token(
                "color.border.default",
                "Field border and affix separator (T §8).",
            ),
            token(
                "color.accent.focusRing",
                "Focus border and focus shadow (T §8).",
            ),
            token(
                "color.accent.base",
                "Pending validation border and indicator (T §4/§8).",
            ),
            token(
                "color.status.danger",
                "Invalid border and indicator (T §4/§8).",
            ),
            token(
                "color.status.success",
                "Valid border and indicator (T §4/§8).",
            ),
            token("color.text.primary", "Editable value text (T §8)."),
            token("color.text.secondary", "Placeholder text (T §8)."),
            token(
                "color.text.tertiary",
                "Character count text — text-input.css references \
                                     --poodle-color-text-muted, which does not resolve \
                                     against the semantic registry (a pre-existing token \
                                     gap); the tertiary family is the intended record \
                                     (T §8, TXT-27; finding for g13.008).",
            ),
            token("color.icon.muted", "Affordance icons (T §8)."),
            token(
                "state.opacity.muted",
                "Placeholder and affix opacity (T §8).",
            ),
            token(
                "state.opacity.disabled",
                "Disabled opacity on the root (T §4/§8).",
            ),
            token(
                "radius.control",
                "Field radius — resolves directly from \
                                     --poodle-radius-control (T §8/§9/§10, TXT-27).",
            ),
        ],

        recipe_hooks: recipe_hooks(),

        // TXT-26 accessibility intent (CROSS-15): native input role, the
        // aria-label/aria-describedby/aria-invalid/aria-busy mappings, and
        // the native attribute projection. Placeholder never counts as the
        // accessible name (TXT-03).
        accessibility: Accessibility {
            role: A11yRole::Textbox,
            name_rule: NameRule::FromProp(ident("ariaLabel")),
            name_source: Some(NameSource::Prop(ident("ariaLabel"))),
            aria: vec![
                AriaMapping {
                    aria_attr: "aria-label".to_owned(),
                    source: ident("ariaLabel"),
                    description: "Accessible name from the prop; required when no external \
                                  label exists; placeholder never counts as the name \
                                  (T §6, TXT-26)."
                        .to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-describedby".to_owned(),
                    source: ident("describedBy"),
                    description: "Described-by target; the built-in validation-message id \
                                  joins it so the invalid message is announced (T §6, \
                                  TXT-12/26)."
                        .to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-invalid".to_owned(),
                    source: ident("validationState"),
                    description: "\"true\" when the effective validation state is invalid — \
                                  the runtime emits from the effective state, which may be \
                                  the built-in mapping (T §6, TXT-12/26)."
                        .to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-busy".to_owned(),
                    source: ident("validationState"),
                    description: "\"true\" when the effective validation state is pending \
                                  (T §6, TXT-12/26)."
                        .to_owned(),
                },
            ],
            native: vec![
                NativeAttr {
                    name: "id".to_owned(),
                    description: "Element id for external label association (T §6, TXT-01)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "name".to_owned(),
                    description: "Form submission name (T §6, TXT-04).".to_owned(),
                },
                NativeAttr {
                    name: "type".to_owned(),
                    description: "Native input type; \"multiline\" renders a textarea, \
                                  \"slug\" renders type=\"text\" (T §6, TXT-06/09)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "autocomplete".to_owned(),
                    description: "Native autocomplete passthrough (T §6, TXT-04).".to_owned(),
                },
                NativeAttr {
                    name: "disabled".to_owned(),
                    description: "Native disabled attribute (T §6, TXT-05).".to_owned(),
                },
                NativeAttr {
                    name: "readonly".to_owned(),
                    description: "Native readonly — NOT aria-readonly (T §6, TXT-05).".to_owned(),
                },
                NativeAttr {
                    name: "required".to_owned(),
                    description: "Native required attribute (T §6, TXT-04).".to_owned(),
                },
                NativeAttr {
                    name: "pattern".to_owned(),
                    description: "Native pattern attribute (T §6, TXT-04).".to_owned(),
                },
                NativeAttr {
                    name: "maxlength".to_owned(),
                    description: "Native maxlength from the maxLength prop (T §6, TXT-14)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "inputmode".to_owned(),
                    description: "Native inputmode (\"text\" in slug mode) (T §6, TXT-04/09)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "list".to_owned(),
                    description: "Native list attribute for datalist association (T §6, \
                                  TXT-04)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "autocapitalize".to_owned(),
                    description: "Native autocapitalize (\"off\" in slug mode) (T §6, \
                                  TXT-04/09)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "spellcheck".to_owned(),
                    description: "Native spellcheck (false in slug mode) (T §6, TXT-04/09)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "autocorrect".to_owned(),
                    description: "Native autocorrect (\"off\" in slug mode; React omits \
                                  autocorrect entirely — OBS-03) (T §6, TXT-04/09)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "enterkeyhint".to_owned(),
                    description: "Native enterkeyhint (T §6, TXT-04).".to_owned(),
                },
                NativeAttr {
                    name: "autofocus".to_owned(),
                    description: "Web-only native autofocus; falsy omits the attribute \
                                  (T §3, TXT-04)."
                        .to_owned(),
                },
            ],
            description: "Native input role; aria-label from the ariaLabel prop (required \
                          without an external label), aria-describedby joined with the \
                          validation-message id, aria-invalid/aria-busy from the effective \
                          validation state; native readonly rather than aria-readonly; \
                          placeholder never counts as the name; the validation indicator \
                          is decorative and aria-hidden (T §6, TXT-26)."
                .to_owned(),
        },

        // R2 — the typed capability boundary, the card's deliverable: the
        // six environment capabilities the milestone names plus the
        // component-owned timers, each typed and each with its per-runtime
        // ownership in prose (see the module notes for the full R2
        // answer). Declared, never implemented (IR-05/06).
        capabilities: vec![
            CapabilityRequirement {
                capability: Capability::Focus,
                purpose: "Caret focus ownership: the web DOM owns focus on the native \
                          input (autofocus, the imperative focus()); the Rust backends own \
                          focus observation (on_focus_change — only they can see a blur) \
                          and caret drawing, with isFocused a host-driven prop that does \
                          not drive the caret (T §6 Caret Ownership, TXT-21)."
                    .to_owned(),
            },
            CapabilityRequirement {
                capability: Capability::TextEditing,
                purpose: "Selection and editing semantics: the web runtime's editing \
                          model IS the browser (no TS text machine); the Rust targets \
                          drive the shared headless model (edit_transition, \
                          insert_transition, selected_text, word_range_at) from key \
                          events and insertions. The DOM owns selection on web; the host \
                          owns the caret position on Rust (selectionStart/selectionEnd, \
                          onSelectionChange) — selection is not a first-class capability \
                          name, recorded as a finding (T §6, TXT-21/22/23; IR-05)."
                    .to_owned(),
            },
            CapabilityRequirement {
                capability: Capability::Ime,
                purpose: "Composition: the web DOM owns composition natively — the \
                          components intercept no composition events, and a composition \
                          sequence must not fire intermediate onValueChange (the runtime \
                          filters input events by isComposing); the Rust backends register \
                          a platform text input handler with a UTF-16 boundary and a \
                          backend-owned marked range (T §6, TXT-24)."
                    .to_owned(),
            },
            CapabilityRequirement {
                capability: Capability::Clipboard,
                purpose: "Copy/cut/paste: the web DOM owns the clipboard natively; the \
                          Rust backends own the platform clipboard (the text comes from \
                          outside the tree) while the shared edit model owns where a \
                          paste lands (insert_transition) and what a copy/cut reads \
                          (selected_text); copying an empty selection leaves the \
                          clipboard alone (T §6, TXT-23)."
                    .to_owned(),
            },
            CapabilityRequirement {
                capability: Capability::Measurement,
                purpose: "Glyph measurement for caret placement: the browser measures \
                          natively; the Rust backends measure shaped glyphs \
                          (shape_line/x_for_index) to draw the caret at character n and \
                          answer the reverse question (closest_index_for_x makes \
                          click-to-position and drag-to-select possible) (T §6, TXT-21/22)."
                    .to_owned(),
            },
            CapabilityRequirement {
                capability: Capability::Timers,
                purpose: "Debounce and validation timing owned by the component: the web \
                          components own their setTimeout lifecycle (debounce, \
                          validationDebounce) with cleanup on destroy; the Rust targets \
                          have no timer surface today — the host drives timing (T §5, \
                          TXT-11/12/28; CROSS-17)."
                    .to_owned(),
            },
        ],

        // TXT-20 keyboard table (T §6; CROSS-16): character insertion and
        // caret/selection movement through the shared edit model (Rust) or
        // the browser (web); copy/cut/paste/select-all and undo through the
        // platform; Enter/Escape submit/cancel; Tab moves focus out.
        keyboard: vec![
            KeyboardCommand {
                id: ident("insert-character"),
                keys: vec![KeyChord {
                    key: "Character".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "insert-text".to_owned(),
                effect: "A printable character replaces the selection at the caret \
                         (T §6, TXT-20)."
                    .to_owned(),
                requires: Some(Capability::TextEditing),
                description: "Character input inserts text (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("move-caret-left"),
                keys: vec![KeyChord {
                    key: "ArrowLeft".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "move-caret".to_owned(),
                effect: "Moves the caret left; a plain arrow collapses a selection to its \
                         edge; accel moves to the start (T §6, TXT-20)."
                    .to_owned(),
                requires: Some(Capability::TextEditing),
                description: "ArrowLeft moves the caret (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("move-caret-right"),
                keys: vec![KeyChord {
                    key: "ArrowRight".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "move-caret".to_owned(),
                effect: "Moves the caret right (T §6, TXT-20).".to_owned(),
                requires: Some(Capability::TextEditing),
                description: "ArrowRight moves the caret (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("move-to-start"),
                keys: vec![KeyChord {
                    key: "Home".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "move-caret".to_owned(),
                effect: "Moves the caret to the start of the value (T §6, TXT-20).".to_owned(),
                requires: Some(Capability::TextEditing),
                description: "Home moves to the start (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("move-to-end"),
                keys: vec![KeyChord {
                    key: "End".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "move-caret".to_owned(),
                effect: "Moves the caret to the end of the value (T §6, TXT-20).".to_owned(),
                requires: Some(Capability::TextEditing),
                description: "End moves to the end (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("extend-selection-left"),
                keys: vec![KeyChord {
                    key: "ArrowLeft".to_owned(),
                    modifiers: [Modifier::Shift].into_iter().collect(),
                }],
                action: "extend-selection".to_owned(),
                effect: "Shift+ArrowLeft extends the selection leftward, keeping the \
                         anchor (T §6, TXT-20)."
                    .to_owned(),
                requires: Some(Capability::TextEditing),
                description: "Shift+Arrow extends selection (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("extend-selection-right"),
                keys: vec![KeyChord {
                    key: "ArrowRight".to_owned(),
                    modifiers: [Modifier::Shift].into_iter().collect(),
                }],
                action: "extend-selection".to_owned(),
                effect: "Shift+ArrowRight extends the selection rightward (T §6, TXT-20)."
                    .to_owned(),
                requires: Some(Capability::TextEditing),
                description: "Shift+Arrow extends selection (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("copy"),
                keys: vec![KeyChord {
                    key: "c".to_owned(),
                    modifiers: [Modifier::Meta].into_iter().collect(),
                }],
                action: "copy".to_owned(),
                effect: "Copies the selected text; an empty selection leaves the \
                         clipboard alone (T §6, TXT-20/23)."
                    .to_owned(),
                requires: Some(Capability::Clipboard),
                description: "Platform copy shortcut (T §6, TXT-20/23).".to_owned(),
            },
            KeyboardCommand {
                id: ident("cut"),
                keys: vec![KeyChord {
                    key: "x".to_owned(),
                    modifiers: [Modifier::Meta].into_iter().collect(),
                }],
                action: "cut".to_owned(),
                effect: "Copies then deletes the selection through the shared edit model \
                         (T §6, TXT-20/23)."
                    .to_owned(),
                requires: Some(Capability::Clipboard),
                description: "Platform cut shortcut (T §6, TXT-20/23).".to_owned(),
            },
            KeyboardCommand {
                id: ident("paste"),
                keys: vec![KeyChord {
                    key: "v".to_owned(),
                    modifiers: [Modifier::Meta].into_iter().collect(),
                }],
                action: "paste".to_owned(),
                effect: "Inserts clipboard text at the caret through insert_transition; \
                         a multi-line paste collapses to one line (T §6, TXT-20/23)."
                    .to_owned(),
                requires: Some(Capability::Clipboard),
                description: "Platform paste shortcut (T §6, TXT-20/23).".to_owned(),
            },
            KeyboardCommand {
                id: ident("select-all"),
                keys: vec![KeyChord {
                    key: "a".to_owned(),
                    modifiers: [Modifier::Meta].into_iter().collect(),
                }],
                action: "select-all".to_owned(),
                effect: "Selects the whole value; typing after replaces it (T §6, TXT-20)."
                    .to_owned(),
                requires: Some(Capability::TextEditing),
                description: "Platform select-all shortcut (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("undo"),
                keys: vec![KeyChord {
                    key: "z".to_owned(),
                    modifiers: [Modifier::Meta].into_iter().collect(),
                }],
                action: "undo".to_owned(),
                effect: "Steps back one undoable step; a continuous run of typing is one \
                         step (coalesces); editing after undo discards the redo tail; \
                         history is ephemeral backend-owned state (T §6, TXT-25)."
                    .to_owned(),
                requires: Some(Capability::TextEditing),
                description: "accel+Z undoes (T §6, TXT-25).".to_owned(),
            },
            KeyboardCommand {
                id: ident("redo"),
                keys: vec![KeyChord {
                    key: "z".to_owned(),
                    modifiers: [Modifier::Meta, Modifier::Shift].into_iter().collect(),
                }],
                action: "redo".to_owned(),
                effect: "Steps forward one undoable step (T §6, TXT-25).".to_owned(),
                requires: Some(Capability::TextEditing),
                description: "accel+shift+Z redoes (T §6, TXT-25).".to_owned(),
            },
            KeyboardCommand {
                id: ident("submit"),
                keys: vec![KeyChord {
                    key: "Enter".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "submit".to_owned(),
                effect: "Calls onSubmit with the current value; Cmd/Ctrl+Enter in \
                         multiline mode (T §5/§6, TXT-13/20)."
                    .to_owned(),
                requires: None,
                description: "Enter submits (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("cancel"),
                keys: vec![KeyChord {
                    key: "Escape".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "cancel".to_owned(),
                effect: "Calls onCancel (T §5/§6, TXT-13/20).".to_owned(),
                requires: None,
                description: "Escape cancels (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("move-focus-next"),
                keys: vec![KeyChord {
                    key: "Tab".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "move-focus".to_owned(),
                effect: "Moves focus out of the control; text-focused shortcut \
                         suppression while focused (T §6, TXT-20)."
                    .to_owned(),
                requires: Some(Capability::Focus),
                description: "Tab moves focus out (T §6, TXT-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("move-focus-previous"),
                keys: vec![KeyChord {
                    key: "Tab".to_owned(),
                    modifiers: [Modifier::Shift].into_iter().collect(),
                }],
                action: "move-focus".to_owned(),
                effect: "Shift+Tab moves focus backward out of the control (T §6, TXT-20)."
                    .to_owned(),
                requires: Some(Capability::Focus),
                description: "Shift+Tab moves focus backward (T §6, TXT-20).".to_owned(),
            },
        ],

        // TXT-19 visual-state projection: the fields the runtime derives
        // (resolved size/density, the validation-state mapping, the merged
        // value) and the TXT-16 padding strings, which drawing consumes
        // (CROSS-14, IR-06). The char-count--over modifier (charCount >
        // maxLength) is a value-dependent class — no sanctioned slot
        // exists, recorded in the R7 inventory.
        visual_state: vec![VisualState {
            id: ident("text-input-visual-state"),
            name: "TextInputVisualState".to_owned(),
            fields: vec![
                visual_field(
                    "currentValue",
                    "currentValue",
                    VisualFieldKind::String,
                    "The merged value — controlled value ?? \"\", or the uncontrolled \
                     state (T §3, TXT-02).",
                ),
                visual_field(
                    "effectiveValidationState",
                    "effectiveValidationState",
                    VisualFieldKind::Enum(ident("validation-state")),
                    "The validationStatusToState mapping when a validator is present, \
                     otherwise the caller-owned validationState (T §4, TXT-12).",
                ),
                visual_field(
                    "resolvedSize",
                    "resolvedSize",
                    VisualFieldKind::Enum(ident("control-size")),
                    "Explicit size or sizeRole resolution (T §7, TXT-15; CROSS-07).",
                ),
                visual_field(
                    "resolvedDensity",
                    "resolvedDensity",
                    VisualFieldKind::Enum(ident("control-density")),
                    "Explicit density or inherited presentation (T §8, TXT-15; CROSS-08).",
                ),
                visual_field(
                    "leadingContent",
                    "leadingContent",
                    VisualFieldKind::Bool,
                    "A leading affordance snippet is provided (T §2, TXT-17).",
                ),
                visual_field(
                    "trailingContent",
                    "trailingContent",
                    VisualFieldKind::Bool,
                    "A trailing affordance snippet is provided (T §2, TXT-17).",
                ),
                visual_field(
                    "charCount",
                    "charCount",
                    VisualFieldKind::Number,
                    "currentValue length; the over-limit modifier (charCount > maxLength) \
                     is a value-dependent class the vocabulary cannot express (T §8, \
                     TXT-14).",
                ),
                visual_field(
                    "controlPaddingStart",
                    "controlPaddingStart",
                    VisualFieldKind::String,
                    "Start padding reservation calc() string (T §8, TXT-16).",
                ),
                visual_field(
                    "controlPaddingEnd",
                    "controlPaddingEnd",
                    VisualFieldKind::String,
                    "End padding reservation calc() string (T §8, TXT-16).",
                ),
                visual_field(
                    "multilineBottomPadding",
                    "multilineBottomPadding",
                    VisualFieldKind::String,
                    "Multiline bottom padding calc() string (T §8, TXT-07/16).",
                ),
                visual_field(
                    "clearInsetInlineEnd",
                    "clearInsetInlineEnd",
                    VisualFieldKind::String,
                    "Clear button inset calc() string (T §8, TXT-16).",
                ),
                visual_field(
                    "trailingInsetInlineEnd",
                    "trailingInsetInlineEnd",
                    VisualFieldKind::String,
                    "Trailing affordance inset calc() string (T §8, TXT-16).",
                ),
            ],
            description: "The projection the runtime derives, which drawing consumes — \
                          value, validation state, resolved axes, and the TXT-16 padding \
                          strings; never machine internals (T §4; CROSS-14, TXT-19, \
                          IR-06)."
                .to_owned(),
        }],

        // R5: the editing model semantics are declared by the `text-input`
        // conformance vector (CROSS-18), authored in this model so the
        // reference resolves; the model stays hand-written in the headless
        // crate and `machines.json` is a fixed target with no text key
        // (the vector gap is recorded in the vector description).
        conformance: vec![ident("text-input")],

        // T §12 deltas (TXT-31, OBS-03): the Jetstream clear-only surface
        // and the React autocorrect omission. The §12 "allowed" rows
        // (native caret visuals, transition timing, recipe fallback chain,
        // affix separator) are parity freedoms recorded in the batch log's
        // R7 inventory, not extension rows.
        extensions: vec![
            Extension {
                id: ident("jetstream-clear-only"),
                owning_runtime: RuntimeTarget::Jetstream,
                reason: "Jetstream delivers pointer events only: the clear button is the \
                         only part of a field a pointer can reach, so it is the only \
                         wired event; the host owns the editor and feeds the value back \
                         through the spec (T §10a/§12, TXT-31)."
                    .to_owned(),
                parity_effect: "onValueChange, onKeyDown, onSubmit and onCancel have no \
                                 route on Jetstream; disabled/read-only fields do not \
                                 clear."
                    .to_owned(),
                evidence_surface: "docs/contracts/components/text-input.md §10a/§12".to_owned(),
                removal_condition: "None — the difference is intentional.".to_owned(),
                description: "TXT-31 Jetstream clear-only events.".to_owned(),
            },
            Extension {
                id: ident("react-omits-autocorrect"),
                owning_runtime: RuntimeTarget::React,
                reason: "React's TextInputProps surface omits the autocorrect prop the \
                         Svelte surface carries (OBS-03); the DOM asymmetry is \
                         pre-existing and unchanged (T §3, TXT-04)."
                    .to_owned(),
                parity_effect: "React never emits the native autocorrect attribute; \
                                 Svelte does when the prop is set."
                    .to_owned(),
                evidence_surface: "docs/roadmaps/g13/pilot-expressiveness-corpus.md TXT-04 \
                                    (OBS-03)"
                    .to_owned(),
                removal_condition: "Add autocorrect to the React surface (a prop-surface \
                                     change, outside this card's scope)."
                    .to_owned(),
                description: "OBS-03 React omits autocorrect.".to_owned(),
            },
        ],
    }
}

/// The TextInput model — the one component, its shared types, the
/// declarative `text-input` conformance vector, and nothing else (the
/// shell scene, Button, RangeSlider, and synthetic fixtures are untouched).
pub fn text_input_model() -> IrModel {
    IrModel {
        schema_version: poodle_ir::IR_SCHEMA_VERSION,
        shared_types: vec![
            shared_type(
                "text-input-type",
                "TextInputType",
                contract_ref("§3"),
                &[
                    ("text", "Plain single-line input (default)."),
                    (
                        "multiline",
                        "Multiline editing — renders a native textarea.",
                    ),
                    (
                        "search",
                        "Search mode — automatic leading search icon and clear button.",
                    ),
                    (
                        "slug",
                        "Semantic slug entry rendered as a native text input.",
                    ),
                ],
                "The TextInput type modes (T §3, TXT-06; the TS type union). Other \
                 native HTML input types pass through web-only.",
            ),
            shared_type(
                "validation-state",
                "ValidationState",
                contract_ref("§3"),
                &[
                    ("none", "No validation emphasis (default)."),
                    (
                        "invalid",
                        "Invalid — status-danger border and cross indicator.",
                    ),
                    (
                        "valid",
                        "Valid — status-success border and check indicator.",
                    ),
                    (
                        "pending",
                        "Pending — accent border and ring spinner indicator.",
                    ),
                ],
                "The validation-state ladder (T §3/§4, TXT-12; the TS ValidationState \
                 union).",
            ),
            shared_type(
                "input-mode",
                "InputMode",
                contract_ref("§3"),
                &[
                    ("none", "No virtual keyboard hint."),
                    ("search", "Search keyboard."),
                    ("text", "Text keyboard."),
                    ("tel", "Telephone keyboard."),
                    ("url", "URL keyboard."),
                    ("email", "Email keyboard."),
                    ("numeric", "Numeric keyboard."),
                    ("decimal", "Decimal keyboard."),
                ],
                "The inputMode value domain (T §3, TXT-04; the TS inputMode union).",
            ),
            shared_type(
                "resize-direction",
                "ResizeDirection",
                contract_ref("§3"),
                &[
                    ("vertical", "Resize handle on the vertical axis (default)."),
                    ("horizontal", "Resize handle on the horizontal axis."),
                    ("both", "Resize handle on both axes."),
                    ("none", "No resize handle."),
                ],
                "The multiline resize domain (T §3, TXT-07; the TS resize union).",
            ),
            shared_type(
                "enter-key-hint",
                "EnterKeyHint",
                contract_ref("§3"),
                &[
                    ("enter", "Enter key."),
                    ("done", "Done."),
                    ("go", "Go."),
                    ("next", "Next."),
                    ("previous", "Previous."),
                    ("search", "Search."),
                    ("send", "Send."),
                ],
                "The enterkeyhint value domain (T §3, TXT-04; the TS enterKeyHint union).",
            ),
            shared_type(
                "autocorrect-mode",
                "AutocorrectMode",
                contract_ref("§3"),
                &[
                    ("on", "Native autocorrection enabled."),
                    ("off", "Native autocorrection disabled."),
                ],
                "The autocorrect value domain (T §3, TXT-04; the TS autocorrect union).",
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
        ],
        components: vec![text_input_definition()],
        conformance_vectors: vec![ConformanceVector {
            id: ident("text-input"),
            name: "text-input".to_owned(),
            // The runtimes that implement the shared edit model are the
            // Rust targets. The web runtimes honor the same semantics
            // through the browser's native editing with no machine of their
            // own — the asymmetry this milestone measures (R3); card 049
            // proves the native half.
            applies_to: vec![RuntimeTarget::Gpui, RuntimeTarget::Jetstream],
            steps: vec![
                VectorStep {
                    id: ident("insert-at-caret"),
                    name: "character input inserts at the caret".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "A printable character replaces the selection at the \
                                  caret, not appended at the end (T §6; EDIT_MODEL \
                                  edit_transition tests)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("selection-replaces"),
                    name: "typing replaces the selection".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "Typing over a selection replaces the whole range; the \
                                  caret lands after the inserted text (T §6; edit_transition \
                                  shift-arrow tests)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("deletion-semantics"),
                    name: "backspace/delete at the caret".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "Backspace removes before the caret, delete removes at it; \
                                  at the edges both are inert but consumed — a key that is \
                                  ours but changes nothing must not fall through (T §6; \
                                  edit_transition tests)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("caret-moves-do-not-edit"),
                    name: "arrows, Home and End move without editing".to_owned(),
                    kind: VectorStepKind::Invariant,
                    description: "Left/right/home/end move the caret (or collapse a \
                                  selection to its edge) and never change the value (T §6; \
                                  edit_transition tests)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("shift-extends-selection"),
                    name: "Shift+Arrow extends the selection".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "Shift keeps the anchor and moves the head; the following \
                                  keystroke replaces the extended selection (T §6; \
                                  edit_transition tests)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("select-all-replaces"),
                    name: "select-all then type replaces everything".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "Accel+A selects the whole value; typing after replaces \
                                  it (T §6; edit_transition tests)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("paste-lands-at-caret"),
                    name: "paste lands at the caret".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "insert_transition places clipboard text at the caret, \
                                  replacing any selection; multi-line collapse is \
                                  backend-owned (T §6, TXT-23; insert_transition tests)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("word-boundaries"),
                    name: "a word is a run of alphanumerics or underscore".to_owned(),
                    kind: VectorStepKind::Guard,
                    description: "word_range_at defines double-click selection: a run of \
                                  alphanumerics/_ is a word; punctuation is its own run; \
                                  the trailing caret belongs to the word before it (T §6, \
                                  TXT-22; word_range_at tests)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("undo-coalesces-typing-runs"),
                    name: "a continuous typing run is one undo step".to_owned(),
                    kind: VectorStepKind::Transition,
                    description: "coalesces collapses consecutive single-character \
                                  insertions at the caret into one step; a deletion, a \
                                  paste, or a caret that jumped breaks the run; replacing \
                                  a selection begins its own step (T §6, TXT-25; coalesces \
                                  tests)."
                        .to_owned(),
                },
                VectorStep {
                    id: ident("submit-cancel-tab-fall-through"),
                    name: "Enter, Escape and Tab are not edit-model keys".to_owned(),
                    kind: VectorStepKind::Guard,
                    description: "edit_transition returns None for keys the model does not \
                                  own — Enter (submit), Escape (cancel), Tab (focus out) — \
                                  so they reach the host handlers (T §6, TXT-20; \
                                  edit_transition tests)."
                        .to_owned(),
                },
            ],
            description: format!(
                "The shared hand-written edit model's semantics, implemented by the Rust \
                 targets from {EDIT_MODEL} — a machine that stays hand-written (R5) and is \
                 pinned by its own unit tests. Executable vector evidence: **none** — \
                 {VECTOR_EVIDENCE} carries no `text` key (GAP-01) and the vectors are a \
                 fixed target (R5); the b045 slider-vector thinness finding generalizes: \
                 the text machine is unit-test-pinned only, in both runtimes."
            ),
        }],
        scenes: Vec::new(),
        specimen_registry: None,
    }
}
