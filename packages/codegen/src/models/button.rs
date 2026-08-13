//! The authored Button definition — `g13.005`'s first real component,
//! defined once in Rust (spec 063 "Authoring Form": ordinary Rust types and
//! constructor helpers, no macros), serialized to the JSON fixture the
//! pipeline consumes (`ir:build` / `ir:check` via `load_and_validate`), and
//! emitted to both web packages through the `button-ts` target.
//!
//! # Placement — pilot-scoped
//!
//! Same ruling as `g13-b035` R1: `poodle-ir` is **lib only, no `[[bin]]`**,
//! pure serializable data plus validation — an authored *instance* is
//! content, not schema. This module therefore lives in `poodle-codegen`,
//! reachable from the existing bin, and no new crate exists. Where
//! production models are authored is a `g13.008` question; do not mistake
//! this boundary for settled.
//!
//! # R2 — the artifact must carry the rendered vocabulary
//!
//! The generated artifact is not a props type: it carries the parts (with
//! their DOM classes), the eleven state attributes (with names, forms,
//! emission policies, and value domains), and the recipe hooks, and
//! Button's Svelte and React read those instead of hard-coding the
//! attribute names and values inline. The proof (card step 7): renaming an
//! attribute here moves both web previews' DOM in one `ir:build`.
//!
//! # R3 — this is re-plumbing
//!
//! All 34 web props keep their names, types, and defaults; the DOM keeps
//! its eleven attributes and their values; `button.css` is untouched. The
//! web surface is verified by `svelte:surface-audit` and
//! `docs:contract-drift`, not by this model.
//!
//! # Vocabulary notes recorded for g13.008
//!
//! - `Value::Null` is not accepted for `Shared`-typed props by
//!   `poodle-ir` validation (`value_matches_type`), so the nullable shared
//!   props (`size`, `density`, `formenctype`, `formmethod`) record
//!   `default: None` and name the null default in their description.
//! - Button's controlled/uncontrolled toggle pair (`pressed` /
//!   `defaultPressed`) is **controlled-wins**, not do-not-mix: the IR's
//!   only `ControlRule` is `DoNotMix`, so the pair is recorded through the
//!   props and the VisualState projection (`isToggle`, `currentPressed`)
//!   instead of `controlled_state`.

use poodle_ir::{
    A11yRole, Accessibility, AriaMapping, AttributeForm, Axes, Capability, CapabilityRequirement,
    ComponentDefinition, ContractRef, ControlSize, DensityAxis, EmissionPolicy, Event, EventKind,
    EventPayload, EventTiming, Extension, FiringPhase, Identifier, IrModel, KeyChord,
    KeyboardCommand, Layer, MetricValue, Modifier, NameRule, NameSource, NativeAttr,
    OrderingConstraint, Part, PartKind, PayloadKind, PermittedSubset, Prop, PropType,
    RecipeHookRef, RecipeLink, RecipeLinkKind, RuntimeTarget, SharedEnumMember, SharedType,
    SizeAxis, SizeRole, SizeStep, StateAttribute, TokenGroup, TokenRef, Value, VisualFieldKind,
    VisualState, VisualStateField,
};

/// The governing contract, cited by the component and every definition row.
const CONTRACT: &str = "docs/contracts/components/button.md";

fn ident(value: &str) -> Identifier {
    Identifier::new(value)
}

fn contract_ref(section: &str) -> ContractRef {
    ContractRef::new(CONTRACT, Some(section))
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
    members: &[(&str, &str)],
    description: &str,
) -> SharedType {
    SharedType {
        id: ident(shared_id),
        name: rust_name.to_owned(),
        description: description.to_owned(),
        canonical_ref: contract_ref("§3"),
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
/// on a shared-typed prop (see module notes).
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
        "button-variant" => vec!["primary", "secondary", "ghost"],
        "button-tone" => vec!["default", "danger", "success", "warning"],
        "control-size" => vec!["xs", "sm", "md", "lg", "xl"],
        "control-density" => vec!["compact", "default", "comfortable"],
        "control-size-role" => vec!["chrome", "control", "prominent"],
        "button-fit" => vec!["default", "content"],
        "button-type" => vec!["button", "submit", "reset"],
        "button-form-enc-type" => {
            vec![
                "application/x-www-form-urlencoded",
                "multipart/form-data",
                "text/plain",
            ]
        }
        "button-form-method" => vec!["get", "post", "dialog"],
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
/// (`CROSS-13`). `source` names the prop, controlled state, or VisualState
/// field the value derives from; the emitted vocabulary is the attribute
/// row itself (name, form, emission, value domain), never an expression
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

/// A presence-only state attribute (`CROSS-13`; `BTN-18`). When the
/// presence is emitted is documented in the description — the emission
/// condition was an expression and is gone (g13.017 R1 bucket 1; the
/// information survives as prose).
fn presence_attribute(id: &str, name: &str, description: &str) -> StateAttribute {
    StateAttribute {
        id: ident(id),
        name: name.to_owned(),
        form: AttributeForm::PresenceOnly,
        emission: EmissionPolicy::Always,
        source: None,
        description: description.to_owned(),
    }
}

/// A VisualState projection field (`CROSS-14`, `BTN-19`).
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

/// Builds one recipe-hook override chain (`CROSS-09`; `BTN-22`): recipe
/// hook → component variable → terminal token, in resolution order.
fn recipe_hook(
    hook: &str,
    component_var: &str,
    token: Option<&str>,
    description: &str,
) -> RecipeHookRef {
    let mut chain = vec![
        RecipeLink {
            kind: RecipeLinkKind::RecipeHook,
            target: hook.to_owned(),
        },
        RecipeLink {
            kind: RecipeLinkKind::ComponentVariable,
            target: component_var.to_owned(),
        },
    ];
    if let Some(token) = token {
        chain.push(RecipeLink {
            kind: RecipeLinkKind::Token,
            target: token.to_owned(),
        });
    }
    RecipeHookRef {
        hook: hook.to_owned(),
        chain,
        description: description.to_owned(),
    }
}

/// One variant×tone recipe-hook family (`BTN-22`): every property the
/// family defines and the terminal semantic token each property's fallback
/// resolves to. `prefix` joins the hook name (`--poodle-recipe-button-` +
/// prefix + property); the component variable is always
/// `--poodle-button-<property>` — the family hooks override the same
/// component variables the base family declares.
fn recipe_family(
    prefix: &str,
    properties: &[(&str, Option<&str>)],
    description: &str,
) -> Vec<RecipeHookRef> {
    properties
        .iter()
        .map(|(property, token)| {
            recipe_hook(
                &format!("--poodle-recipe-button-{prefix}{property}"),
                &format!("--poodle-button-{property}"),
                *token,
                description,
            )
        })
        .collect()
}

/// The secondary (default) variant's seven properties. Fill mixes the
/// surface toward text-primary (elevation stacking); border-hover mixes
/// toward text-primary 78% (B §8).
const SECONDARY_PROPS: &[(&str, Option<&str>)] = &[
    ("fill", Some("color.background.surface")),
    ("fill-hover", Some("color.background.surface")),
    ("fill-active", Some("color.background.surface")),
    ("border", Some("color.border.default")),
    ("border-hover", Some("color.text.primary")),
    ("text", Some("color.text.primary")),
    ("shadow", None),
];

/// Primary variant: accent fill/border family, inverse text; the shadow is
/// a white/black mix with no single semantic token (B §8).
const PRIMARY_PROPS: &[(&str, Option<&str>)] = &[
    ("fill", Some("color.accent.base")),
    ("fill-hover", Some("color.accent.base")),
    ("fill-active", Some("color.accent.base")),
    ("border", Some("color.accent.base")),
    ("text", Some("color.text.inverse")),
    ("shadow", None),
];

/// Ghost variant: transparent fill/border, no shadow (B §8).
const GHOST_PROPS: &[(&str, Option<&str>)] = &[("fill", None), ("border", None), ("shadow", None)];

/// Status tone (danger/success/warning) on the secondary variant: the
/// status color tints fill and border-hover; the idle border stays
/// border-default (B §8 status-tone structure).
fn tone_props(status: &'static str) -> Vec<(&'static str, Option<&'static str>)> {
    vec![
        ("fill", Some(status)),
        ("fill-hover", Some(status)),
        ("fill-active", Some(status)),
        ("border", Some("color.border.default")),
        ("border-hover", Some(status)),
        ("text", Some("color.text.primary")),
    ]
}

/// Status tone on the primary variant: solid status fill/border family,
/// inverse text (B §8).
fn primary_tone_props(status: &'static str) -> Vec<(&'static str, Option<&'static str>)> {
    vec![
        ("fill", Some(status)),
        ("fill-hover", Some(status)),
        ("fill-active", Some(status)),
        ("border", Some(status)),
        ("border-hover", Some(status)),
        ("text", Some("color.text.inverse")),
        ("shadow", None),
    ]
}

/// Status tone on the ghost variant: status-tinted hover/active fills,
/// transparent idle fill and border, status text (B §8).
fn ghost_tone_props(status: &'static str) -> Vec<(&'static str, Option<&'static str>)> {
    vec![
        ("fill", Some(status)),
        ("fill-hover", Some(status)),
        ("fill-active", Some(status)),
        ("border", Some(status)),
        ("border-hover", Some(status)),
        ("text", Some(status)),
        ("shadow", None),
    ]
}

/// The 76 recipe hooks of `button.css` (82 `var(--poodle-recipe-*)` uses;
/// the distinct hooks are declared once with their chain). Each family's
/// description cites the contract §8 table it transcribes.
fn recipe_hooks() -> Vec<RecipeHookRef> {
    let mut hooks = Vec::new();

    hooks.extend(recipe_family(
        "",
        SECONDARY_PROPS,
        "Secondary (default) variant override chain — elevation stacking toward text-primary (B §8).",
    ));
    hooks.extend(recipe_family(
        "primary-",
        PRIMARY_PROPS,
        "Primary variant override chain (B §8).",
    ));
    hooks.extend(recipe_family(
        "ghost-",
        GHOST_PROPS,
        "Ghost variant override chain (B §8).",
    ));

    for (tone, status) in [
        ("danger", "color.status.danger"),
        ("success", "color.status.success"),
        ("warning", "color.status.warning"),
    ] {
        hooks.extend(recipe_family(
            &format!("{tone}-"),
            &tone_props(status),
            &format!("{tone} tone override chain on the secondary variant (B §8)."),
        ));
        hooks.extend(recipe_family(
            &format!("primary-{tone}-"),
            &primary_tone_props(status),
            &format!("{tone} tone override chain on the primary variant (B §8)."),
        ));
        hooks.extend(recipe_family(
            &format!("ghost-{tone}-"),
            &ghost_tone_props(status),
            &format!("{tone} tone override chain on the ghost variant (B §8)."),
        ));
    }

    hooks
}

/// The `g13.005` Button definition — the pilot's first real component
/// (synthetic components proved the vocabulary; Button authors a value into
/// every `CROSS-*` row).
pub fn button_definition() -> ComponentDefinition {
    ComponentDefinition {
        id: ident("button"),
        name: "Button".to_owned(),
        layer: Layer::Foundation,
        contract: contract_ref("§3"),
        description: "A general action trigger for commands, confirmations, and view-level \
                      affordances (B §1). g13.005's pilot component: 34 web props, eleven \
                      data-* attributes, six anatomy parts, the full variant × tone matrix, \
                      and the recipe-hook override chain (BTN-01..23)."
            .to_owned(),

        // 27 data props + 3 snippet slots (children/leading/trailing) —
        // the IR records the four callbacks as events, so the 34-web-prop
        // surface is 30 props + 4 events. Order is the contract's prop
        // table order (B §3).
        props: vec![
            prop(
                "variant",
                shared("button-variant"),
                Some(Value::member("secondary")),
                false,
                "Visual treatment — primary/secondary/ghost (B §3).",
            ),
            prop(
                "tone",
                shared("button-tone"),
                Some(Value::member("default")),
                false,
                "Intent modifier; every tone composes with every variant (B §3; 004-shared-control-types ButtonTone).",
            ),
            prop(
                "size",
                shared("control-size"),
                None,
                false,
                "Explicit control-size override; default null — resolves from inherited \
                 presentation plus sizeRole (B §3, BTN-03; CROSS-07).",
            ),
            prop(
                "sizeRole",
                shared("control-size-role"),
                Some(Value::member("control")),
                false,
                "Semantic size offset from inherited presentation (B §3, BTN-04; CROSS-07).",
            ),
            prop(
                "density",
                shared("control-density"),
                None,
                false,
                "Explicit density override; default null — inherited from presentation \
                 (B §3, BTN-05; CROSS-08).",
            ),
            prop(
                "type",
                shared("button-type"),
                Some(Value::member("button")),
                true,
                "HTML button type (B §3; CROSS-03 web-only).",
            ),
            prop(
                "form",
                PropType::String,
                Some(Value::Null),
                true,
                "External form id to associate with (B §3; CROSS-03 web-only).",
            ),
            prop(
                "formaction",
                PropType::String,
                Some(Value::Null),
                true,
                "Per-button form submission URL override (B §3; CROSS-03 web-only).",
            ),
            prop(
                "formenctype",
                shared("button-form-enc-type"),
                None,
                true,
                "Per-button encoding override; default null (B §3; CROSS-03 web-only).",
            ),
            prop(
                "formmethod",
                shared("button-form-method"),
                None,
                true,
                "Per-button form method override; default null (B §3; CROSS-03 web-only).",
            ),
            prop(
                "formnovalidate",
                PropType::Bool,
                Some(Value::boolean(false)),
                true,
                "Skips form validation for this submit action (B §3; CROSS-03 web-only).",
            ),
            prop(
                "formtarget",
                PropType::String,
                Some(Value::Null),
                true,
                "Per-button browsing context override (B §3; CROSS-03 web-only).",
            ),
            bool_prop(
                "disabled",
                "Suppresses activation (B §3, BTN-07).",
            ),
            bool_prop(
                "loading",
                "Shows the spinner, suppresses activation, always emits data-loading (B §3, BTN-08).",
            ),
            prop(
                "leadingIcon",
                PropType::Opaque,
                Some(Value::Null),
                false,
                "Icon registry identifier for the leading icon; the component carries it \
                 without interpreting the registry (B §3; opaque payload).",
            ),
            prop(
                "trailingIcon",
                PropType::Opaque,
                Some(Value::Null),
                false,
                "Icon registry identifier for the trailing icon (B §3; opaque payload).",
            ),
            bool_prop(
                "chevron",
                "Renders the trailing disclosure chevron indicator (B §3).",
            ),
            bool_prop(
                "truncate",
                "Emits data-truncate; clips an overlong label with ellipsis (B §3, BTN-13).",
            ),
            prop(
                "fit",
                shared("button-fit"),
                Some(Value::member("default")),
                false,
                "Width mode — default or content shrink-wrap; content emits data-fit (B §3, BTN-13).",
            ),
            prop(
                "maxWidth",
                PropType::String,
                Some(Value::Null),
                false,
                "Composed into the inline style as max-width when provided (B §3, BTN-13).",
            ),
            prop(
                "pressed",
                PropType::Bool,
                Some(Value::Null),
                false,
                "Controlled toggle state; non-null activates toggle mode with aria-pressed (B §3, BTN-14).",
            ),
            prop(
                "defaultPressed",
                PropType::Bool,
                Some(Value::Null),
                false,
                "Initial pressed state for uncontrolled toggle mode (B §3, BTN-14).",
            ),
            prop(
                "ariaLabel",
                PropType::String,
                Some(Value::Null),
                false,
                "Accessible name; required when no visible label (B §3, BTN-15/21).",
            ),
            prop(
                "ariaExpanded",
                PropType::Bool,
                Some(Value::Null),
                false,
                "Disclosure-state hint for menu and accordion triggers; None omits the \
                 attribute (B §3/§6, BTN-15; portable Option<bool>).",
            ),
            prop(
                "describedBy",
                PropType::String,
                Some(Value::Null),
                false,
                "aria-describedby target (B §3/§6, BTN-15).",
            ),
            prop(
                "className",
                PropType::String,
                Some(Value::string("")),
                true,
                "Additional CSS classes on the root (B §3; CROSS-03 web-only DOM-node prop).",
            ),
            prop(
                "style",
                PropType::String,
                Some(Value::Null),
                true,
                "Inline style passthrough for dynamic sizing and CSS-variable overrides \
                 (B §3/§6; CROSS-03 web-only DOM-node prop).",
            ),
            prop(
                "children",
                PropType::Opaque,
                None,
                false,
                "Label content snippet; absence triggers icon-only mode (B §3, BTN-16).",
            ),
            prop(
                "leading",
                PropType::Opaque,
                None,
                true,
                "Custom leading content snippet, overrides leadingIcon (B §3, BTN-16; \
                 CROSS-03 web-only snippet slot).",
            ),
            prop(
                "trailing",
                PropType::Opaque,
                None,
                true,
                "Custom trailing content snippet, overrides trailingIcon (B §3, BTN-16; \
                 CROSS-03 web-only snippet slot).",
            ),
        ],

        // Button's toggle pair is controlled-wins, not do-not-mix — see the
        // module notes; recorded through props + VisualState instead.
        controlled_state: Vec::new(),

        // The four callbacks are declared as events (CROSS-05): activation,
        // focus change (x2), and pressed change, with the B §5 ordering
        // note (`onPressedChange` before `onClick`).
        events: vec![
            Event {
                id: ident("activation"),
                name: "onClick".to_owned(),
                kind: EventKind::Activation,
                payload: None,
                timing: EventTiming {
                    phase: FiringPhase::OnRelease,
                    debounce_ms: None,
                    flush_on_blur: false,
                    ordering: vec![OrderingConstraint {
                        before: ident("pressed-change"),
                        after: ident("activation"),
                        reason: "onPressedChange fires before onClick (B §5, BTN-14; CROSS-06)."
                            .to_owned(),
                    }],
                },
                description: "Activation completed; suppressed while disabled or loading \
                              (B §5, BTN-14; CROSS-20)."
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
                description: "Focus enters the root (B §5).".to_owned(),
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
                description: "Focus leaves the root (B §5).".to_owned(),
            },
            Event {
                id: ident("pressed-change"),
                name: "onPressedChange".to_owned(),
                kind: EventKind::PressedChange,
                payload: Some(EventPayload {
                    name: "pressed".to_owned(),
                    kind: PayloadKind::Bool,
                }),
                timing: EventTiming {
                    phase: FiringPhase::DuringInteraction,
                    ..EventTiming::default()
                },
                description: "Toggle state changes; fires when the button is in toggle mode, \
                              before onClick (B §5, BTN-14)."
                    .to_owned(),
            },
        ],

        // The anatomy (B §2): root plus the five conditional/static parts.
        // Slot-presence and derived-content conditions are documented
        // prose on the conditional parts (g13.017 R1 bucket 2: anatomy
        // kept, expression tree gone); the snippet and icon props are
        // opaque payloads the vocabulary cannot test directly.
        parts: vec![
            Part {
                id: ident("root"),
                name: "Root".to_owned(),
                parent: None,
                kind: PartKind::Static,
                description: "Clickable command surface; always present (B §2).".to_owned(),
            },
            Part {
                id: ident("spinner"),
                name: "Spinner".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::Conditional {
                    when: ident("loading"),
                    description: "Shared Spinner primitive, ring/sm/current, rendered when \
                                  loading (B §2, BTN-08/17)."
                        .to_owned(),
                },
                description: "Loading spinner wrapper (B §2).".to_owned(),
            },
            Part {
                id: ident("leading-icon"),
                name: "Leading Icon".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::ConditionalDocumented {
                    condition: "present when the leading snippet or leadingIcon prop is \
                                provided"
                        .to_owned(),
                    description: "Leading icon span (B §2, BTN-16/17).".to_owned(),
                },
                description: "Icon before the label (B §2).".to_owned(),
            },
            Part {
                id: ident("label"),
                name: "Label".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::ConditionalDocumented {
                    condition: "present when children content exists; absence triggers \
                                icon-only mode"
                        .to_owned(),
                    description: "Label content span (B §2, BTN-09/16/17).".to_owned(),
                },
                description: "Text content (B §2).".to_owned(),
            },
            Part {
                id: ident("trailing-icon"),
                name: "Trailing Icon".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::ConditionalDocumented {
                    condition: "present when the trailing snippet or trailingIcon prop is \
                                provided"
                        .to_owned(),
                    description: "Trailing icon span (B §2, BTN-16/17).".to_owned(),
                },
                description: "Icon after the label (B §2).".to_owned(),
            },
            Part {
                id: ident("chevron"),
                name: "Chevron".to_owned(),
                parent: Some(ident("root")),
                kind: PartKind::Conditional {
                    when: ident("chevron"),
                    description: "Disclosure indicator after all content, rendered when \
                                  chevron is true (B §2, BTN-17)."
                        .to_owned(),
                },
                description: "Trailing disclosure chevron (B §2).".to_owned(),
            },
        ],

        // The eleven data-* attributes (B §9, BTN-18). Names, forms,
        // emission policies, and value domains are the rendered vocabulary
        // the `button-ts` artifact carries (R2).
        attributes: vec![
            valued_attribute(
                "variant",
                "data-variant",
                "variant",
                EmissionPolicy::Always,
                "The variant value; always emitted (B §9, BTN-18).",
            ),
            valued_attribute(
                "tone",
                "data-tone",
                "tone",
                EmissionPolicy::OmitWhenDefault,
                "The tone value, omitted when the tone is default (B §9, BTN-18).",
            ),
            valued_attribute(
                "size",
                "data-size",
                "resolvedSize",
                EmissionPolicy::Always,
                "The resolved control size (explicit or sizeRole-derived); always emitted \
                 (B §9, BTN-18; CROSS-07).",
            ),
            valued_attribute(
                "density",
                "data-density",
                "resolvedDensity",
                EmissionPolicy::Always,
                "The resolved density (explicit or inherited); always emitted (B §9, BTN-18; \
                 CROSS-08).",
            ),
            presence_attribute(
                "icon-only",
                "data-icon-only",
                "Presence-only; emitted when there is no label content (B §9, BTN-09/18).",
            ),
            presence_attribute(
                "has-leading",
                "data-has-leading",
                "Presence-only; emitted when a leading icon/snippet is present or loading \
                 (B §9, BTN-17/18).",
            ),
            presence_attribute(
                "has-trailing",
                "data-has-trailing",
                "Presence-only; emitted when a trailing icon/snippet or the chevron is \
                 present (B §9, BTN-17/18).",
            ),
            presence_attribute(
                "truncate",
                "data-truncate",
                "Presence-only; emitted when truncate is true (B §9, BTN-18).",
            ),
            valued_attribute(
                "fit",
                "data-fit",
                "fit",
                EmissionPolicy::OmitWhenDefault,
                "The fit value, emitted only when fit is content (B §9, BTN-18).",
            ),
            valued_attribute(
                "loading",
                "data-loading",
                "loading",
                EmissionPolicy::Always,
                "The loading boolean; always emitted, even as false (B §9, BTN-08/18).",
            ),
            valued_attribute(
                "pressed",
                "data-pressed",
                "currentPressed",
                EmissionPolicy::Always,
                "The current pressed boolean, emitted only in toggle mode (B §9, BTN-14/18).",
            ),
        ],

        // Axes (CROSS-07/08): size ladder with the contract's fixed rem
        // metrics and icon-side padding adjustments (B §8, BTN-23); density
        // is attribute-only (no metric adjustments); no orientation.
        axes: Axes {
            size: Some(SizeAxis {
                explicit: None,
                size_role: SizeRole::Control,
                ladder: vec![
                    SizeStep {
                        size: ControlSize::Xs,
                        metrics: size_metrics(
                            1.5,
                            3.75,
                            0.6875,
                            1.5,
                            -0.1875,
                        ),
                        description: "Extra-small rung: height 1.5rem, min-width 3.75rem, \
                                      font-size 0.6875rem; icon-side padding reduces 0.1875rem \
                                      (B §8, BTN-23)."
                            .to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Sm,
                        metrics: size_metrics(1.75, 4.25, 0.75, 1.75, -0.25),
                        description: "Small rung: height 1.75rem, min-width 4.25rem, font-size \
                                      0.75rem; icon-side padding reduces 0.25rem (B §8, BTN-23)."
                            .to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Md,
                        metrics: size_metrics(2.25, 5.0, 0.8125, 2.25, -0.125),
                        description: "Default rung: height 2.25rem, min-width 5rem, font-size \
                                      0.8125rem; icon-side padding reduces 0.125rem (B §8, \
                                      BTN-23)."
                            .to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Lg,
                        metrics: size_metrics(2.75, 5.75, 0.875, 2.75, 0.0),
                        description: "Large rung: height 2.75rem, min-width 5.75rem, font-size \
                                      0.875rem; no icon-side padding reduction (B §8, BTN-23)."
                            .to_owned(),
                    },
                    SizeStep {
                        size: ControlSize::Xl,
                        metrics: size_metrics(3.25, 6.5, 0.9375, 3.25, 0.0625),
                        description: "Extra-large rung: height 3.25rem, min-width 6.5rem, \
                                      font-size 0.9375rem; icon-side padding adds 0.0625rem \
                                      (B §8, BTN-23)."
                            .to_owned(),
                    },
                ],
            }),
            density: Some(DensityAxis {
                explicit: None,
                adjustments: Vec::new(),
            }),
            orientation: None,
        },

        // BTN-22: the semantic tokens the appearance consumes, resolved
        // against the generated poodle-tokens registry (CROSS-09).
        tokens: vec![
            token("typography.label.family", "Label typography family (B §8)."),
            token("typography.label.size", "Label typography size (B §8)."),
            token("typography.label.weight", "Label typography weight (B §8)."),
            token("radius.control", "Root border radius (B §8)."),
            token("border.width.focus", "Focus outline width (B §8)."),
            token("color.accent.focusRing", "Focus outline color (B §8)."),
            token("state.opacity.disabled", "Disabled opacity (B §8)."),
            token("color.text.primary", "Base root text (B §8)."),
            token("color.text.inverse", "Primary variant text (B §8)."),
            token("color.accent.base", "Primary variant fill/border family (B §8)."),
            token("color.border.default", "Secondary variant border (B §8)."),
            token("color.status.danger", "Danger tone family (B §8)."),
            token("color.status.success", "Success tone family (B §8)."),
            token("color.status.warning", "Warning tone family (B §8)."),
            token(
                "color.background.surface",
                "Secondary fill mix base (elevation stacking, B §8).",
            ),
            token("size.icon.md", "Icon wrapper sizing (B §8)."),
            token("space.control.x", "Root inline padding (B §8)."),
            token("space.button.gap", "Content gap (B §8)."),
            token("space.button.iconInset", "Icon inset adjustments (B §8)."),
            token("space.inline.sm", "Chevron margin (B §8)."),
            token("motion.duration.interaction", "Transition duration (B §8)."),
            token("motion.easing.standard", "Transition easing (B §8)."),
        ],

        recipe_hooks: recipe_hooks(),

        // BTN-21 accessibility intent (CROSS-15).
        accessibility: Accessibility {
            role: A11yRole::Button,
            name_rule: NameRule::FromContent,
            name_source: Some(NameSource::Prop(ident("ariaLabel"))),
            aria: vec![
                AriaMapping {
                    aria_attr: "aria-pressed".to_owned(),
                    source: ident("pressed"),
                    description: "Toggle state; set only in toggle mode (B §6, BTN-14/21)."
                        .to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-busy".to_owned(),
                    source: ident("loading"),
                    description: "Loading signal (B §6, BTN-08/21).".to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-expanded".to_owned(),
                    source: ident("ariaExpanded"),
                    description: "Disclosure-state hint (B §6, BTN-15/21).".to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-describedby".to_owned(),
                    source: ident("describedBy"),
                    description: "Description association (B §6, BTN-15/21).".to_owned(),
                },
                AriaMapping {
                    aria_attr: "aria-label".to_owned(),
                    source: ident("ariaLabel"),
                    description: "Explicit accessible name (B §6, BTN-15/21).".to_owned(),
                },
            ],
            native: vec![
                NativeAttr {
                    name: "disabled".to_owned(),
                    description: "Native disabled when disabled or loading (B §6, BTN-07; CROSS-20)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "type".to_owned(),
                    description: "Native button type (B §6).".to_owned(),
                },
                NativeAttr {
                    name: "form".to_owned(),
                    description: "Native form override attrs pass through when provided (B §6, BTN-06)."
                        .to_owned(),
                },
                NativeAttr {
                    name: "formaction".to_owned(),
                    description: "Native form override attr (B §6, BTN-06).".to_owned(),
                },
                NativeAttr {
                    name: "formenctype".to_owned(),
                    description: "Native form override attr (B §6, BTN-06).".to_owned(),
                },
                NativeAttr {
                    name: "formmethod".to_owned(),
                    description: "Native form override attr (B §6, BTN-06).".to_owned(),
                },
                NativeAttr {
                    name: "formnovalidate".to_owned(),
                    description: "Native form override attr (B §6, BTN-06).".to_owned(),
                },
                NativeAttr {
                    name: "formtarget".to_owned(),
                    description: "Native form override attr (B §6, BTN-06).".to_owned(),
                },
            ],
            description: "Native button role; name from content with ariaLabel as the explicit \
                          source — icon-only buttons require one (B §6, BTN-21)."
                .to_owned(),
        },

        // The only declared adapter capability is keyboard focus delivery
        // (BTN-20); everything else is native button behavior.
        capabilities: vec![CapabilityRequirement {
            capability: Capability::Focus,
            purpose: "Tab/Shift+Tab focus movement and focus-ring delivery (B §6, BTN-20; \
                      CROSS-17)."
                .to_owned(),
        }],

        // BTN-20 keyboard table (CROSS-16).
        keyboard: vec![
            KeyboardCommand {
                id: ident("activate"),
                keys: vec![
                    KeyChord {
                        key: "Enter".to_owned(),
                        modifiers: Default::default(),
                    },
                    KeyChord {
                        key: "Space".to_owned(),
                        modifiers: Default::default(),
                    },
                ],
                action: "activate".to_owned(),
                effect: "Native button activation; suppressed while disabled or loading \
                         (B §6, BTN-20; CROSS-20)."
                    .to_owned(),
                requires: None,
                description: "Enter/Space activate the button (B §6, BTN-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("move-focus-next"),
                keys: vec![KeyChord {
                    key: "Tab".to_owned(),
                    modifiers: Default::default(),
                }],
                action: "move-focus".to_owned(),
                effect: "Moves focus to the next focusable element (B §6, BTN-20).".to_owned(),
                requires: Some(Capability::Focus),
                description: "Tab moves focus onward (B §6, BTN-20).".to_owned(),
            },
            KeyboardCommand {
                id: ident("move-focus-previous"),
                keys: vec![KeyChord {
                    key: "Tab".to_owned(),
                    modifiers: [Modifier::Shift].into_iter().collect(),
                }],
                action: "move-focus".to_owned(),
                effect: "Moves focus to the previous focusable element (B §6, BTN-20)."
                    .to_owned(),
                requires: Some(Capability::Focus),
                description: "Shift+Tab moves focus backward (B §6, BTN-20).".to_owned(),
            },
        ],

        // BTN-19 visual-state projection: the derived values the web
        // components' `$derived` compute, declared so drawing consumes
        // serializable state (CROSS-14, IR-06).
        visual_state: vec![VisualState {
            id: ident("button-visual-state"),
            name: "ButtonVisualState".to_owned(),
            fields: vec![
                visual_field(
                    "iconOnly",
                    "iconOnly",
                    VisualFieldKind::Bool,
                    "No children content — icon-only mode (B §4/§9, BTN-09).",
                ),
                visual_field(
                    "leadingContent",
                    "leadingContent",
                    VisualFieldKind::Bool,
                    "Leading snippet or leadingIcon prop present (B §2, BTN-16/17).",
                ),
                visual_field(
                    "trailingContent",
                    "trailingContent",
                    VisualFieldKind::Bool,
                    "Trailing snippet or trailingIcon prop present (B §2, BTN-16/17).",
                ),
                visual_field(
                    "hasLeading",
                    "hasLeading",
                    VisualFieldKind::Bool,
                    "leadingContent or loading — drives data-has-leading (B §9, BTN-17).",
                ),
                visual_field(
                    "hasTrailing",
                    "hasTrailing",
                    VisualFieldKind::Bool,
                    "trailingContent or chevron — drives data-has-trailing (B §9, BTN-17).",
                ),
                visual_field(
                    "isToggle",
                    "isToggle",
                    VisualFieldKind::Bool,
                    "pressed or defaultPressed non-null — toggle mode (B §3/§9, BTN-14).",
                ),
                visual_field(
                    "isUnavailable",
                    "isUnavailable",
                    VisualFieldKind::Bool,
                    "disabled or loading — activation suppressed (CROSS-20, BTN-07/08).",
                ),
                visual_field(
                    "currentPressed",
                    "currentPressed",
                    VisualFieldKind::Bool,
                    "Controlled ? pressed : uncontrolledPressed (B §3, BTN-14; CROSS-04).",
                ),
                visual_field(
                    "resolvedSize",
                    "resolvedSize",
                    VisualFieldKind::Enum(ident("control-size")),
                    "Explicit size or sizeRole resolution (B §7, BTN-03/04; CROSS-07).",
                ),
                visual_field(
                    "resolvedDensity",
                    "resolvedDensity",
                    VisualFieldKind::Enum(ident("control-density")),
                    "Explicit density or inherited presentation (B §8, BTN-05; CROSS-08).",
                ),
                visual_field(
                    "resolvedStyle",
                    "resolvedStyle",
                    VisualFieldKind::String,
                    "Inline style plus max-width composition (B §3/§9, BTN-13).",
                ),
            ],
            description: "The derived values the web runtimes compute and drawing consumes \
                          (B §4/§9; CROSS-14, BTN-19)."
                .to_owned(),
        }],

        conformance: Vec::new(),

        // B §12 known deltas (EXT class rows BTN-26/27/29): the web half
        // records them now so the four-runtime definition is complete; card
        // 042 consumes them for the natives.
        extensions: vec![
            Extension {
                id: ident("gpui-translate-y-omitted"),
                owning_runtime: RuntimeTarget::Gpui,
                reason: "Sub-pixel transform; GPUI cannot render translateY(0.03125rem) \
                         (B §12; BTN-26)."
                    .to_owned(),
                parity_effect: "Active state does not press down half a pixel.".to_owned(),
                evidence_surface: "docs/contracts/components/button.md §12".to_owned(),
                removal_condition: "Revisit if GPUI gains sub-pixel transforms.".to_owned(),
                description: "BTN-26 active translateY delta.".to_owned(),
            },
            Extension {
                id: ident("gpui-box-shadow-omitted"),
                owning_runtime: RuntimeTarget::Gpui,
                reason: "GPUI lacks CSS box-shadow support (B §12; BTN-26).".to_owned(),
                parity_effect: "Variant shadows are not drawn in GPUI.".to_owned(),
                evidence_surface: "docs/contracts/components/button.md §12".to_owned(),
                removal_condition: "Revisit if GPUI adds shadow primitives.".to_owned(),
                description: "BTN-26 box-shadow delta.".to_owned(),
            },
            Extension {
                id: ident("gpui-letter-spacing-omitted"),
                owning_runtime: RuntimeTarget::Gpui,
                reason: "GPUI text rendering has no letter-spacing API (B §12; BTN-26)."
                    .to_owned(),
                parity_effect: "0.01em letter-spacing is not applied in GPUI.".to_owned(),
                evidence_surface: "docs/contracts/components/button.md §12".to_owned(),
                removal_condition: "Revisit if GPUI adds letter-spacing.".to_owned(),
                description: "BTN-26 letter-spacing delta.".to_owned(),
            },
            Extension {
                id: ident("jetstream-no-focus-events"),
                owning_runtime: RuntimeTarget::Jetstream,
                reason: "The runtime raises pointer events, not focus ones; a handler for an \
                         event that is never delivered is worse than a missing one (B §12; \
                         BTN-27)."
                    .to_owned(),
                parity_effect: "onFocus/onBlur are absent in Jetstream; focus arrives with \
                                 focus plumbing."
                    .to_owned(),
                evidence_surface: "docs/contracts/components/button.md §12".to_owned(),
                removal_condition: "Arrives with focus plumbing.".to_owned(),
                description: "BTN-27 Jetstream focus-event delta.".to_owned(),
            },
            Extension {
                id: ident("jetstream-no-pressed-change"),
                owning_runtime: RuntimeTarget::Jetstream,
                reason: "pressed is a spec input there; a host that owns the state derives the \
                         change from on_click (B §12; BTN-27)."
                    .to_owned(),
                parity_effect: "onPressedChange is absent in Jetstream.".to_owned(),
                evidence_surface: "docs/contracts/components/button.md §12".to_owned(),
                removal_condition: "None — the difference is intentional.".to_owned(),
                description: "BTN-27 Jetstream pressed-change delta.".to_owned(),
            },
            Extension {
                id: ident("rust-button-variant-superset"),
                owning_runtime: RuntimeTarget::Gpui,
                reason: "ButtonVariant::Danger is retained in the Rust enum for backward \
                         compatibility and is equivalent to Primary + Danger tone; it is not \
                         part of the authored vocabulary (004-shared-control-types.md; BTN-29)."
                    .to_owned(),
                parity_effect: "A legacy Rust value resolves to a defined variant × tone cell \
                                 instead of a new visual treatment."
                    .to_owned(),
                evidence_surface: "docs/contracts/004-shared-control-types.md".to_owned(),
                removal_condition: "Drop the legacy member once native specs stop constructing \
                                     it (BTN-29)."
                    .to_owned(),
                description: "BTN-29 Rust enum superset.".to_owned(),
            },
        ],
    }
}

/// Builds a semantic [`TokenRef`] (`CROSS-09`).
fn token(path: &str, description: &str) -> TokenRef {
    TokenRef {
        path: path.to_owned(),
        group: TokenGroup::Semantic,
        description: description.to_owned(),
    }
}

/// The per-rung size metrics (B §8, BTN-23): height, min-width, padding
/// (flat `0 var(--poodle-space-control-x)` across the ladder), font-size,
/// the icon-only square width (equals the height), and the per-size
/// icon-side padding delta.
fn size_metrics(
    height: f64,
    min_width: f64,
    font_size: f64,
    icon_only_width: f64,
    icon_padding_delta: f64,
) -> std::collections::BTreeMap<String, MetricValue> {
    let mut metrics = std::collections::BTreeMap::new();
    metrics.insert("height".to_owned(), MetricValue::Rem(height));
    metrics.insert("min-width".to_owned(), MetricValue::Rem(min_width));
    metrics.insert(
        "padding".to_owned(),
        MetricValue::Text("0 var(--poodle-space-control-x)".to_owned()),
    );
    metrics.insert("font-size".to_owned(), MetricValue::Rem(font_size));
    metrics.insert(
        "icon-only-width".to_owned(),
        MetricValue::Rem(icon_only_width),
    );
    metrics.insert(
        "icon-padding-adjustment".to_owned(),
        MetricValue::Rem(icon_padding_delta),
    );
    metrics
}

/// The Button model — the one component, its shared types, and nothing else
/// (R5: the shell scene and synthetic fixture are untouched).
pub fn button_model() -> IrModel {
    IrModel {
        schema_version: poodle_ir::IR_SCHEMA_VERSION,
        shared_types: vec![
            shared_type(
                "button-variant",
                "ButtonVariant",
                &[
                    ("primary", "Accent-filled treatment."),
                    ("secondary", "Surface treatment (default)."),
                    ("ghost", "Borderless, transparent-fill treatment."),
                ],
                "Visual treatment of a button (B §3; the TS ButtonVariant union).",
            ),
            shared_type(
                "button-tone",
                "ButtonTone",
                &[
                    ("default", "Neutral baseline tone."),
                    ("danger", "Destructive intent."),
                    ("success", "Positive intent."),
                    ("warning", "Caution intent."),
                ],
                "Intent modifier (004-shared-control-types.md ButtonTone; every tone composes \
                 with every variant).",
            ),
            shared_type(
                "control-size",
                "ControlSize",
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
                &[
                    ("chrome", "Shell chrome size role."),
                    ("control", "Standard control size role (default)."),
                    ("prominent", "Prominent size role."),
                ],
                "Semantic size offset from inherited presentation (CROSS-07, BTN-04; the TS \
                 SemanticControlSizeRole union).",
            ),
            shared_type(
                "button-fit",
                "ButtonFit",
                &[
                    ("default", "Holds the default minimum width."),
                    ("content", "Shrink-wraps the content."),
                ],
                "Width mode of the button (B §3, BTN-13).",
            ),
            shared_type(
                "button-type",
                "ButtonType",
                &[
                    ("button", "Plain push button (default)."),
                    ("submit", "Submits the owning form."),
                    ("reset", "Resets the owning form."),
                ],
                "HTML button type (B §3; CROSS-03 web-only).",
            ),
            shared_type(
                "button-form-enc-type",
                "ButtonFormEncType",
                &[
                    (
                        "application/x-www-form-urlencoded",
                        "Default URL-encoded encoding.",
                    ),
                    ("multipart/form-data", "Multipart encoding."),
                    ("text/plain", "Plain-text encoding."),
                ],
                "Per-button form encoding override (B §3; CROSS-03 web-only).",
            ),
            shared_type(
                "button-form-method",
                "ButtonFormMethod",
                &[
                    ("get", "GET submission."),
                    ("post", "POST submission."),
                    ("dialog", "Dialog submission."),
                ],
                "Per-button form method override (B §3; CROSS-03 web-only).",
            ),
        ],
        components: vec![button_definition()],
        conformance_vectors: Vec::new(),
        scenes: Vec::new(),
        specimen_registry: None,
    }
}
