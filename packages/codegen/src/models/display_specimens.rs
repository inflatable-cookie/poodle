//! The display-specimen scenes — tranche one of `g14.003` (batch card
//! `g14-b005`): Callout, EmptyState, Avatar, Pill, Spinner authored once in
//! Rust and emitted to all four previews via the `specimen-ts` and
//! `specimen-rust` targets (spec 065 "Scene Authoring": fixture authority,
//! four runtimes, no evaluator, no application framework).
//!
//! # Fixture scope — the static tier (R1/R2, amended by the tranche)
//!
//! The scene carries instances with typed prop bindings, specimen-section
//! groups (the `group` field added to `ComponentInstance` by this card —
//! spec 063 Scene IR lists "groups" as scene content and the implementation
//! lacked it), and the size/density axes the matrix tabs iterate. Content
//! text is bound to a fixture `content` prop and projected by the emitters
//! (web children, native spec fields). Anything the static tier cannot say
//! is classified, not schema-creeped:
//!
//! - interactive harnesses (Callout's dismissal demo) — the scene carries
//!   the static `dismissible` instance; the interaction stays out;
//! - composition/snippets (Callout's action button, EmptyState's
//!   actions/visual snippets) — excluded from the fixture, recorded in the
//!   batch log.
//!
//! The definitions below are fixture vocabulary, not contract
//! transcriptions: each prop exists to bind a specimen value. The b052 rule
//! applies to their size — the tranche measures whether this stays under
//! the 9× definition cost.

use poodle_ir::{
    A11yRole, Accessibility, AxisValues, ComponentDefinition, ComponentInstance, ContractRef,
    Identifier, IrModel, Layer, NameRule, PermittedSubset, Prop, PropBinding, PropType, Scene,
    SceneAxis, SceneAxisKind, SharedType, SpecimenTabs, Value,
};

fn ids(values: impl IntoIterator<Item = impl Into<String>>) -> Vec<Identifier> {
    values.into_iter().map(Into::into).map(Identifier::new).collect()
}

/// The fixture definitions carry no accessibility model — validation needs
/// the field, not a claim about these components.
fn a11y() -> Accessibility {
    Accessibility {
        role: A11yRole::Group,
        name_rule: NameRule::FromContent,
        name_source: None,
        aria: Vec::new(),
        native: Vec::new(),
        description: "Fixture definition (g14-b005): no accessibility claims authored.".to_owned(),
    }
}

fn subset(shared: &str, members: &[&str]) -> PermittedSubset {
    PermittedSubset::new(
        Identifier::new(shared),
        members.iter().map(|member| Identifier::new(*member)).collect::<Vec<_>>(),
    )
}

fn member_type(shared: &str) -> PropType {
    PropType::Shared(Identifier::new(shared))
}

fn shared_type(id: &str, name: &str, contract_path: &str, members: &[(&str, &str)]) -> SharedType {
    SharedType {
        id: Identifier::new(id),
        name: name.to_owned(),
        description: format!("{name} — the union the contract fragment defines; specimen fixtures bind its members."),
        canonical_ref: ContractRef {
            path: contract_path.to_owned(),
            section: None,
        },
        members: members
            .iter()
            .map(|(id, name)| poodle_ir::SharedEnumMember {
                id: Identifier::new(*id),
                name: (*name).to_owned(),
                description: format!("{name} member."),
            })
            .collect(),
    }
}

fn prop(id: &str, prop_type: PropType, subset: Option<PermittedSubset>, description: &str) -> Prop {
    Prop {
        id: Identifier::new(id),
        name: id.to_owned(),
        prop_type,
        default: None,
        required: false,
        web_only: false,
        description: description.to_owned(),
        permitted_subset: subset,
    }
}

fn shared_prop(id: &str, shared: &str, members: &[&str], description: &str) -> Prop {
    prop(
        id,
        member_type(shared),
        Some(subset(shared, members)),
        description,
    )
}

fn instance(
    component: &str,
    group: &str,
    caption: &str,
    bindings: Vec<(&str, Value)>,
) -> ComponentInstance {
    ComponentInstance {
        component: Identifier::new(component),
        bindings: bindings
            .into_iter()
            .map(|(prop, value)| PropBinding {
                prop: Identifier::new(prop),
                value,
                description: None,
            })
            .collect(),
        caption: Some(caption.to_owned()),
        group: Some(group.to_owned()),
    }
}

fn str_value(value: &str) -> Value {
    Value::string(value)
}

fn member_value(value: &str) -> Value {
    Value::member(value)
}

/// The five display components, props only — the smallest surface the
/// specimen bindings need. Everything a specimen binds must exist and be
/// typed, or `ir:check` fails.
fn display_components() -> Vec<ComponentDefinition> {
    let contract = |path: &str| ContractRef {
        path: path.to_owned(),
        section: Some("§3 Public Props".to_owned()),
    };

    vec![
        ComponentDefinition {
            id: Identifier::new("callout"),
            name: "Callout".to_owned(),
            layer: Layer::Foundation,
            contract: contract("docs/contracts/components/callout.md"),
            description: "Semantic status callout; fixture definition for the callout specimen scene.".to_owned(),
            props: vec![
                shared_prop("tone", "status-tone", &["neutral", "info", "success", "warning", "danger", "pending"], "semantic tone and coloring"),
                shared_prop("fill", "tone-fill", &["tint", "solid"], "tone surface treatment"),
                prop("title", PropType::String, None, "bold heading text"),
                prop("message", PropType::String, None, "body text rendered as a paragraph"),
                prop("content", PropType::String, None, "fixture content prop (g14-b005): projected to children on web, the native spec content field on GPUI/Jetstream"),
                prop("dismissible", PropType::Bool, None, "shows the dismiss control"),
                shared_prop("size", "control-size", &["xs", "sm", "md", "lg", "xl"], "explicit control-size override"),
                shared_prop("density", "control-density", &["compact", "default", "comfortable"], "explicit density override"),
            ],
            controlled_state: Vec::new(),
            events: Vec::new(),
            parts: Vec::new(),
            attributes: Vec::new(),
            axes: poodle_ir::Axes::default(),
            tokens: Vec::new(),
            recipe_hooks: Vec::new(),
            accessibility: a11y(),
            capabilities: Vec::new(),
            keyboard: Vec::new(),
            visual_state: Vec::new(),
            conformance: Vec::new(),
            extensions: Vec::new(),
        },
        ComponentDefinition {
            id: Identifier::new("pill"),
            name: "Pill".to_owned(),
            layer: Layer::Foundation,
            contract: contract("docs/contracts/components/pill.md"),
            description: "Status pill; fixture definition for the pill specimen scene.".to_owned(),
            props: vec![
                shared_prop("tone", "pill-tone", &["neutral", "info", "success", "warning", "danger"], "semantic tone controlling fill/border/text color"),
                shared_prop("fill", "tone-fill", &["tint", "solid"], "tone surface treatment"),
                shared_prop("appearance", "pill-appearance", &["solid", "subtle", "badge"], "fill opacity variant"),
                shared_prop("sizeRole", "semantic-size-role", &["chrome", "control", "prominent"], "semantic size offset"),
                shared_prop("font", "pill-font", &["normal", "mono"], "content font variant"),
                shared_prop("typography", "pill-typography", &["label", "inherit"], "label typography or inherited"),
                prop("muted", PropType::Bool, None, "visual de-emphasis"),
                prop("accent", PropType::String, None, "custom accent color"),
                prop("content", PropType::String, None, "fixture content prop (g14-b005): projected to children on web, the native spec label field on GPUI/Jetstream"),
                shared_prop("size", "control-size", &["xs", "sm", "md", "lg", "xl"], "explicit size override"),
                shared_prop("density", "control-density", &["compact", "default", "comfortable"], "explicit density override"),
            ],
            controlled_state: Vec::new(),
            events: Vec::new(),
            parts: Vec::new(),
            attributes: Vec::new(),
            axes: poodle_ir::Axes::default(),
            tokens: Vec::new(),
            recipe_hooks: Vec::new(),
            accessibility: a11y(),
            capabilities: Vec::new(),
            keyboard: Vec::new(),
            visual_state: Vec::new(),
            conformance: Vec::new(),
            extensions: Vec::new(),
        },
        ComponentDefinition {
            id: Identifier::new("spinner"),
            name: "Spinner".to_owned(),
            layer: Layer::Foundation,
            contract: contract("docs/contracts/components/spinner.md"),
            description: "Loading indicator; fixture definition for the spinner specimen scene.".to_owned(),
            props: vec![
                shared_prop("variant", "spinner-variant", &["ring", "grid", "dots"], "loader visual"),
                shared_prop("tone", "spinner-tone", &["current", "accent", "muted"], "color source for the indicator"),
                shared_prop("size", "control-size", &["xs", "sm", "md", "lg", "xl"], "explicit size override"),
                shared_prop("density", "control-density", &["compact", "default", "comfortable"], "explicit density override"),
            ],
            controlled_state: Vec::new(),
            events: Vec::new(),
            parts: Vec::new(),
            attributes: Vec::new(),
            axes: poodle_ir::Axes::default(),
            tokens: Vec::new(),
            recipe_hooks: Vec::new(),
            accessibility: a11y(),
            capabilities: Vec::new(),
            keyboard: Vec::new(),
            visual_state: Vec::new(),
            conformance: Vec::new(),
            extensions: Vec::new(),
        },
        ComponentDefinition {
            id: Identifier::new("avatar"),
            name: "Avatar".to_owned(),
            layer: Layer::Foundation,
            contract: contract("docs/contracts/components/avatar.md"),
            description: "Image or initials avatar; fixture definition for the avatar specimen scene.".to_owned(),
            props: vec![
                prop("initials", PropType::String, None, "initials fallback content"),
                prop("src", PropType::String, None, "image source"),
                prop("alt", PropType::String, None, "image alt text"),
                shared_prop("size", "control-size", &["xs", "sm", "md", "lg", "xl"], "explicit size override"),
                shared_prop("tone", "avatar-tone", &["neutral", "accent"], "semantic tone"),
                shared_prop("shape", "avatar-shape", &["circle", "rounded"], "silhouette shape"),
            ],
            controlled_state: Vec::new(),
            events: Vec::new(),
            parts: Vec::new(),
            attributes: Vec::new(),
            axes: poodle_ir::Axes::default(),
            tokens: Vec::new(),
            recipe_hooks: Vec::new(),
            accessibility: a11y(),
            capabilities: Vec::new(),
            keyboard: Vec::new(),
            visual_state: Vec::new(),
            conformance: Vec::new(),
            extensions: Vec::new(),
        },
        ComponentDefinition {
            id: Identifier::new("empty-state"),
            name: "EmptyState".to_owned(),
            layer: Layer::Foundation,
            contract: contract("docs/contracts/components/empty-state.md"),
            description: "Empty-data placeholder; fixture definition for the empty-state specimen scene.".to_owned(),
            props: vec![
                shared_prop("variant", "empty-state-variant", &["neutral", "search", "firstRun"], "semantic posture"),
                prop("title", PropType::String, None, "primary message"),
                prop("message", PropType::String, None, "supporting explanation"),
                shared_prop("size", "empty-state-size", &["default", "compact"], "visual and copy sizing"),
                shared_prop("density", "control-density", &["compact", "default", "comfortable"], "explicit density override"),
            ],
            controlled_state: Vec::new(),
            events: Vec::new(),
            parts: Vec::new(),
            attributes: Vec::new(),
            axes: poodle_ir::Axes::default(),
            tokens: Vec::new(),
            recipe_hooks: Vec::new(),
            accessibility: a11y(),
            capabilities: Vec::new(),
            keyboard: Vec::new(),
            visual_state: Vec::new(),
            conformance: Vec::new(),
            extensions: Vec::new(),
        },
    ]
}

fn size_axis() -> SceneAxis {
    SceneAxis {
        kind: SceneAxisKind::Size,
        values: AxisValues::Named(ids(["xs", "sm", "md", "lg", "xl"])),
        description: "Control-size axis the Sizes matrix iterates (SHELL-02; CROSS-07).".to_owned(),
    }
}

fn empty_state_size_axis() -> SceneAxis {
    SceneAxis {
        kind: SceneAxisKind::Size,
        values: AxisValues::Named(ids(["default", "compact"])),
        description: "EmptyState size axis the Sizes matrix iterates.".to_owned(),
    }
}

fn density_axis() -> SceneAxis {
    SceneAxis {
        kind: SceneAxisKind::Density,
        values: AxisValues::Named(ids(["compact", "default", "comfortable"])),
        description: "Density axis the Densities matrix iterates (SHELL-03; CROSS-08).".to_owned(),
    }
}

fn size_density_axes() -> Vec<SceneAxis> {
    vec![size_axis(), density_axis()]
}

fn specimen_tabs() -> SpecimenTabs {
    SpecimenTabs {
        tabs: ids(["examples", "sizes", "densities"]),
    }
}

fn scene(id: &str, name: &str, description: &str, instances: Vec<ComponentInstance>) -> Scene {
    Scene {
        id: Identifier::new(id),
        name: name.to_owned(),
        description: description.to_owned(),
        instances,
        axes: size_density_axes(),
        layout: None,
        tabs: Some(specimen_tabs()),
        search: None,
        preview_state: None,
        parity: None,
        captures: Vec::new(),
    }
}

/// Tranche-one scenes. Contract reference: each specimen's §13 specimen
/// set (`B/R/T §13`), superseding the four hand-written copies.
pub fn display_specimens_scenes() -> Vec<Scene> {
    let scenes = vec![
        scene(
            "callout-specimen",
            "Callout",
            "Callout specimen (contract §13): the full tone set, message prop, dismissible, \
             and title-less variants.",
            vec![
                instance("callout", "Tones", "Neutral", vec![
                    ("tone", member_value("neutral")),
                    ("title", str_value("Neutral callout")),
                    ("content", str_value("A general informational message with no specific severity.")),
                ]),
                instance("callout", "Tones", "Info", vec![
                    ("tone", member_value("info")),
                    ("title", str_value("Info")),
                    ("content", str_value("Your changes have been saved and will take effect on next deploy.")),
                ]),
                instance("callout", "Tones", "Success", vec![
                    ("tone", member_value("success")),
                    ("title", str_value("Success")),
                    ("content", str_value("All tests passed. The build is ready for production.")),
                ]),
                instance("callout", "Tones", "Warning", vec![
                    ("tone", member_value("warning")),
                    ("title", str_value("Warning")),
                    ("content", str_value("This API key expires in 7 days. Rotate it to avoid service interruption.")),
                ]),
                instance("callout", "Tones", "Danger", vec![
                    ("tone", member_value("danger")),
                    ("title", str_value("Error")),
                    ("content", str_value("Unable to connect to the database. Check your credentials and try again.")),
                ]),
                instance("callout", "Tones", "Pending", vec![
                    ("tone", member_value("pending")),
                    ("title", str_value("Pending")),
                    ("content", str_value("Provisioning resources. This may take a moment.")),
                ]),
                instance("callout", "Message prop", "Information", vec![
                    ("tone", member_value("info")),
                    ("title", str_value("Information")),
                    ("message", str_value("This is an informational callout using the message prop instead of slot content.")),
                ]),
                instance("callout", "Without title", "Info", vec![
                    ("tone", member_value("info")),
                    ("content", str_value("A simple inline callout without a title for brief contextual notes.")),
                ]),
                instance("callout", "Dismissible", "Dismissible", vec![
                    ("tone", member_value("info")),
                    ("title", str_value("Dismissible callout")),
                    ("content", str_value("This callout can be dismissed by the user.")),
                    ("dismissible", Value::boolean(true)),
                ]),
                instance("callout", "Solid fills", "Solid neutral", vec![
                    ("tone", member_value("neutral")),
                    ("fill", member_value("solid")),
                    ("title", str_value("Solid neutral")),
                    ("content", str_value("Neutral solid surfaces use primary text as the background.")),
                ]),
                instance("callout", "Solid fills", "Solid warning", vec![
                    ("tone", member_value("warning")),
                    ("fill", member_value("solid")),
                    ("title", str_value("Solid warning")),
                    ("content", str_value("Tone base and inverse foreground stay readable across themes.")),
                ]),
                instance("callout", "Solid fills", "Solid pending", vec![
                    ("tone", member_value("pending")),
                    ("fill", member_value("solid")),
                    ("title", str_value("Solid pending")),
                    ("content", str_value("The pending spinner inherits the solid inverse foreground.")),
                ]),
            ],
        ),
        scene(
            "pill-specimen",
            "Pill",
            "Pill specimen (contract §13): tones, code font, muted, badge, inherited typography, \
             and custom accents.",
            vec![
                instance("pill", "Tones", "Neutral", vec![
                    ("tone", member_value("neutral")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Neutral")),
                ]),
                instance("pill", "Tones", "Info", vec![
                    ("tone", member_value("info")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Info")),
                ]),
                instance("pill", "Tones", "Success", vec![
                    ("tone", member_value("success")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Success")),
                ]),
                instance("pill", "Tones", "Warning", vec![
                    ("tone", member_value("warning")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Warning")),
                ]),
                instance("pill", "Tones", "Danger", vec![
                    ("tone", member_value("danger")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Danger")),
                ]),
                instance("pill", "Code font", "Mono", vec![
                    ("font", member_value("mono")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("v2.4.1")),
                ]),
                instance("pill", "Code font", "Mono success", vec![
                    ("font", member_value("mono")),
                    ("tone", member_value("success")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("stable")),
                ]),
                instance("pill", "Code font", "Mono warning", vec![
                    ("font", member_value("mono")),
                    ("tone", member_value("warning")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("beta")),
                ]),
                instance("pill", "Muted", "Muted neutral", vec![
                    ("muted", Value::boolean(true)),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Muted neutral")),
                ]),
                instance("pill", "Muted", "Muted success", vec![
                    ("muted", Value::boolean(true)),
                    ("tone", member_value("success")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Muted success")),
                ]),
                instance("pill", "Muted", "Muted danger", vec![
                    ("muted", Value::boolean(true)),
                    ("tone", member_value("danger")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Muted danger")),
                ]),
                instance("pill", "Badge", "Badge 3", vec![
                    ("appearance", member_value("badge")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("3")),
                ]),
                instance("pill", "Badge", "Badge 12", vec![
                    ("appearance", member_value("badge")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("12")),
                ]),
                instance("pill", "Badge", "Badge 99+", vec![
                    ("appearance", member_value("badge")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("99+")),
                ]),
                instance("pill", "Badge", "Badge New", vec![
                    ("appearance", member_value("badge")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("New")),
                ]),
                instance("pill", "Badge", "Badge Draft", vec![
                    ("appearance", member_value("badge")),
                    ("tone", member_value("neutral")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Draft")),
                ]),
                instance("pill", "Inherited typography", "Active", vec![
                    ("appearance", member_value("badge")),
                    ("tone", member_value("success")),
                    ("typography", member_value("inherit")),
                    ("content", str_value("Active")),
                ]),
                instance("pill", "Custom accent", "Info-ish", vec![
                    ("accent", str_value("#3b82f6")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Info-ish")),
                ]),
                instance("pill", "Custom accent", "Positive-ish", vec![
                    ("accent", str_value("#22c55e")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Positive-ish")),
                ]),
                instance("pill", "Custom accent", "Caution-ish", vec![
                    ("accent", str_value("#f59e0b")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Caution-ish")),
                ]),
                instance("pill", "Custom accent", "Danger-ish", vec![
                    ("accent", str_value("#ef4444")),
                    ("sizeRole", member_value("control")),
                    ("content", str_value("Danger-ish")),
                ]),
                instance("pill", "Solid fills", "Solid neutral", vec![
                    ("tone", member_value("neutral")),
                    ("fill", member_value("solid")),
                    ("content", str_value("Solid neutral")),
                ]),
                instance("pill", "Solid fills", "Solid success subtle", vec![
                    ("tone", member_value("success")),
                    ("fill", member_value("solid")),
                    ("appearance", member_value("subtle")),
                    ("content", str_value("Solid success")),
                ]),
                instance("pill", "Solid fills", "Solid custom badge", vec![
                    ("fill", member_value("solid")),
                    ("appearance", member_value("badge")),
                    ("accent", str_value("#3b82f6")),
                    ("content", str_value("Solid custom")),
                ]),
            ],
        ),
        scene(
            "spinner-specimen",
            "Spinner",
            "Spinner specimen (contract §13): ring and CLI-grid variants across sizes, and \
             context tones.",
            vec![
                instance("spinner", "Ring", "Ring xs", vec![
                    ("variant", member_value("ring")),
                    ("size", member_value("xs")),
                ]),
                instance("spinner", "Ring", "Ring sm", vec![
                    ("variant", member_value("ring")),
                    ("size", member_value("sm")),
                ]),
                instance("spinner", "Ring", "Ring md", vec![
                    ("variant", member_value("ring")),
                    ("size", member_value("md")),
                ]),
                instance("spinner", "Ring", "Ring lg", vec![
                    ("variant", member_value("ring")),
                    ("size", member_value("lg")),
                ]),
                instance("spinner", "Ring", "Ring xl", vec![
                    ("variant", member_value("ring")),
                    ("size", member_value("xl")),
                ]),
                instance("spinner", "CLI grid", "Grid xs", vec![
                    ("variant", member_value("grid")),
                    ("size", member_value("xs")),
                    ("tone", member_value("muted")),
                ]),
                instance("spinner", "CLI grid", "Grid sm", vec![
                    ("variant", member_value("grid")),
                    ("size", member_value("sm")),
                    ("tone", member_value("current")),
                ]),
                instance("spinner", "CLI grid", "Grid md", vec![
                    ("variant", member_value("grid")),
                    ("size", member_value("md")),
                    ("tone", member_value("accent")),
                ]),
                instance("spinner", "CLI grid", "Grid lg", vec![
                    ("variant", member_value("grid")),
                    ("size", member_value("lg")),
                    ("tone", member_value("current")),
                ]),
                instance("spinner", "CLI grid", "Grid xl", vec![
                    ("variant", member_value("grid")),
                    ("size", member_value("xl")),
                    ("tone", member_value("current")),
                ]),
                instance("spinner", "Context tones", "Ring current", vec![
                    ("variant", member_value("ring")),
                    ("tone", member_value("current")),
                ]),
                instance("spinner", "Context tones", "Ring accent", vec![
                    ("variant", member_value("ring")),
                    ("tone", member_value("accent")),
                ]),
                instance("spinner", "Context tones", "Grid muted", vec![
                    ("variant", member_value("grid")),
                    ("tone", member_value("muted")),
                ]),
            ],
        ),
        {
            // Avatar takes no `density` prop (contract §3): the scene must
            // not declare the density axis or tab, or the Densities tab
            // would be advertised for a prop the component does not take.
            let mut avatar_scene = scene(
                "avatar-specimen",
                "Avatar",
                "Avatar specimen (contract §13): initials sizes, tone and shape, and an image \
                 avatar.",
                vec![
                    instance("avatar", "Initials", "TA xs", vec![
                        ("initials", str_value("TA")),
                        ("size", member_value("xs")),
                    ]),
                    instance("avatar", "Initials", "TA sm", vec![
                        ("initials", str_value("TA")),
                        ("size", member_value("sm")),
                    ]),
                    instance("avatar", "Initials", "TA md", vec![
                        ("initials", str_value("TA")),
                        ("size", member_value("md")),
                    ]),
                    instance("avatar", "Initials", "TA lg", vec![
                        ("initials", str_value("TA")),
                        ("size", member_value("lg")),
                    ]),
                    instance("avatar", "Initials", "TA xl", vec![
                        ("initials", str_value("TA")),
                        ("size", member_value("xl")),
                    ]),
                    instance("avatar", "Tone and shape", "AC neutral", vec![
                        ("initials", str_value("AC")),
                        ("tone", member_value("neutral")),
                    ]),
                    instance("avatar", "Tone and shape", "AC accent", vec![
                        ("initials", str_value("AC")),
                        ("tone", member_value("accent")),
                    ]),
                    instance("avatar", "Tone and shape", "AC rounded", vec![
                        ("initials", str_value("AC")),
                        ("shape", member_value("rounded")),
                        ("tone", member_value("accent")),
                    ]),
                    instance("avatar", "Image", "Example user", vec![
                        ("src", str_value("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 80 80'%3E%3Crect width='80' height='80' fill='%232563eb'/%3E%3Ccircle cx='40' cy='30' r='16' fill='%23fff'/%3E%3Cpath d='M14 74c5-18 17-28 26-28s21 10 26 28' fill='%23fff'/%3E%3C/svg%3E")),
                        ("alt", str_value("Example user")),
                        ("size", member_value("lg")),
                    ]),
                ],
            );
            avatar_scene.axes = vec![size_axis()];
            avatar_scene.tabs = Some(SpecimenTabs { tabs: ids(["examples", "sizes"]) });
            avatar_scene
        },
        {
            let mut empty_state_scene = scene(
                "empty-state-specimen",
                "EmptyState",
                "EmptyState specimen (contract §13): neutral, search, first-run, and compact \
                 postures.",
                vec![
                    instance("empty-state", "Neutral", "No projects yet", vec![
                        ("title", str_value("No projects yet")),
                        ("message", str_value("Create your first project to get started.")),
                    ]),
                    instance("empty-state", "Search", "No results found", vec![
                        ("variant", member_value("search")),
                        ("title", str_value("No results found")),
                        ("message", str_value("Try adjusting your search terms or clearing filters.")),
                    ]),
                    instance("empty-state", "First run", "Welcome", vec![
                        ("variant", member_value("firstRun")),
                        ("title", str_value("Welcome to your workspace")),
                        ("message", str_value("This is where your team's components will appear once you start building.")),
                    ]),
                    instance("empty-state", "Compact custom visual", "No captured emails", vec![
                        ("size", member_value("compact")),
                        ("title", str_value("No captured emails found")),
                        ("message", str_value("Emails will appear here when sent in development mode.")),
                    ]),
                ],
            );
            empty_state_scene.axes = vec![empty_state_size_axis(), density_axis()];
            empty_state_scene
        },
    ];

    scenes
}

/// The display-specimen model: shared types, fixture components, and the
/// five scenes. No conformance vectors, no shell vocabulary — the specimen
/// lane, separated from the shell scene (`preview_shell.rs`).
pub fn display_specimens_model() -> IrModel {
    IrModel {
        schema_version: poodle_ir::IR_SCHEMA_VERSION,
        shared_types: vec![
            shared_type("status-tone", "StatusTone", "docs/contracts/components/callout.md", &[
                ("neutral", "Neutral"), ("info", "Info"), ("success", "Success"),
                ("warning", "Warning"), ("danger", "Danger"), ("pending", "Pending"),
            ]),
            shared_type("tone-fill", "ToneFill", "docs/contracts/004-shared-control-types.md", &[
                ("tint", "Tint"), ("solid", "Solid"),
            ]),
            shared_type("pill-tone", "PillTone", "docs/contracts/components/pill.md", &[
                ("neutral", "Neutral"), ("info", "Info"), ("success", "Success"),
                ("warning", "Warning"), ("danger", "Danger"),
            ]),
            shared_type("control-size", "ControlSize", "docs/contracts/components/slider.md", &[
                ("xs", "Xs"), ("sm", "Sm"), ("md", "Md"), ("lg", "Lg"), ("xl", "Xl"),
            ]),
            shared_type("control-density", "ControlDensity", "docs/contracts/components/slider.md", &[
                ("compact", "Compact"), ("default", "Default"), ("comfortable", "Comfortable"),
            ]),
            shared_type("semantic-size-role", "SemanticControlSizeRole", "docs/contracts/components/pill.md", &[
                ("chrome", "Chrome"), ("control", "Control"), ("prominent", "Prominent"),
            ]),
            shared_type("pill-appearance", "PillAppearance", "docs/contracts/components/pill.md", &[
                ("solid", "Solid"), ("subtle", "Subtle"), ("badge", "Badge"),
            ]),
            shared_type("pill-font", "PillFont", "docs/contracts/components/pill.md", &[
                ("normal", "Normal"), ("mono", "Mono"),
            ]),
            shared_type("pill-typography", "PillTypography", "docs/contracts/components/pill.md", &[
                ("label", "Label"), ("inherit", "Inherit"),
            ]),
            shared_type("avatar-tone", "AvatarTone", "docs/contracts/components/avatar.md", &[
                ("neutral", "Neutral"), ("accent", "Accent"),
            ]),
            shared_type("avatar-shape", "AvatarShape", "docs/contracts/components/avatar.md", &[
                ("circle", "Circle"), ("rounded", "Rounded"),
            ]),
            shared_type("spinner-variant", "SpinnerVariant", "docs/contracts/components/spinner.md", &[
                ("ring", "Ring"), ("grid", "Grid"), ("dots", "Dots"),
            ]),
            shared_type("spinner-tone", "SpinnerTone", "docs/contracts/components/spinner.md", &[
                ("current", "Current"), ("accent", "Accent"), ("muted", "Muted"),
            ]),
            shared_type("empty-state-variant", "EmptyStateVariant", "docs/contracts/components/empty-state.md", &[
                ("neutral", "Neutral"), ("search", "Search"), ("firstRun", "FirstRun"),
            ]),
            shared_type("empty-state-size", "EmptyStateSize", "docs/contracts/components/empty-state.md", &[
                ("default", "Default"), ("compact", "Compact"),
            ]),
        ],
        components: display_components(),
        conformance_vectors: Vec::new(),
        scenes: display_specimens_scenes(),
        specimen_registry: None,
    }
}
