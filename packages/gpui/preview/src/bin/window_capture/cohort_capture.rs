//! g16.122 — the closed Nucleus cohort fixture kind.
//!
//! A cohort scene is built from the same shared scenario file used by the A1
//! GPUI proof. The node is rendered through `poodle_render` and the GPUI node
//! backend, then the scenario's pointer/key actions are posted through the
//! real window event queue before the non-activating transport captures it.
//! This module owns no Nucleus data: the scenario props are the complete
//! Poodle fixture input.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::{bail, Context as _, Result};
use gpui::{
    div, px, AnyElement, App, AppContext as _, Context, IntoElement, ParentElement, Render, Styled,
    Window,
};
use poodle_adapter::ThemeProvider;
use poodle_headless::agent_plan::AgentPlanStatus;
use poodle_headless::agent_question::{AgentQuestionItem, AgentQuestionOption};
use poodle_headless::agent_transcript::{TranscriptItem, TranscriptMessage, TranscriptRole};
use poodle_node::{Node, NodeRole};
use poodle_render::{
    AgentChatInputHandlers, AgentPlanHandlers, AgentQuestionHandlers, AgentTranscriptHandlers,
    CalloutHandlers, CommandPaletteHandlers, ConfirmActionHandlers, MessageCenterHandlers,
    PopoverHandlers, RadioGroupHandlers, RenderContext, SelectHandlers, TabsHandlers,
    ToastStackHandlers,
};
use poodle_specs::{
    AgentChatInputSpec, AgentPlanSpec, AgentQuestionSpec, AgentTranscriptSpec, AppHeaderSpec,
    ButtonSpec, ButtonVariant, CallOutSpec, CalloutAnnounceMode, ChoiceOption, CommandActionItem,
    CommandPaletteSpec, ControlDensity, ControlSize, DialogSpec, EditableLabelActivation,
    EditableLabelSpec, IconButtonSpec, IconProviderSpec, IconSize, IconSpec, MenuEntry, MenuSpec,
    MessageCenterItem, MessageCenterSpec, ModelOption, ModelPickerSpec, ModelPickerVariant,
    ModelSelection, Orientation, PaddingScale, PopoverInitialFocus, PopoverSpec, RadioGroupSpec,
    SegmentedControlOption, SegmentedControlSpec, SelectMode, SelectSpec, SplitOrientation,
    SplitViewSpec, StatusIndicatorSpec, StatusTone, SurfaceBorder, SurfaceRole, SurfaceSpec,
    SurfaceTone, SwitchSpec, TabActivationMode, TabDefinition, TabsSpec, TextElement,
    TextInputSpec, TextSize, TextSpec, TextTone, TextWeight, Toast, ToastHostPlacement,
    ToastHostSpec, ToastStackSpec,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::presentation_axes::ThemePreset;
use crate::transport::{self, GPUI_SOURCE, GPUI_VERSION, TRANSPORT};
use crate::{normalize_output_path, publish_pair};

const SCENARIO_SCHEMA: &str = "poodle.g16-nucleus-a11y-scenario.v1";
const RECEIPT_SCHEMA: &str = "poodle.cohort-visual-capture.v1";
const SCENE_PADDING: f32 = 16.0;
const INSTANCE: &str = "cohort";
const POODLE_SOURCE_ID: &str = concat!("poodle-gpui-preview@", env!("CARGO_PKG_VERSION"));

/// This is deliberately a closed registry. A path that happens to exist in
/// the scenario directory is not enough to become a capture fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RegistryEntry {
    scenario_id: &'static str,
    row: &'static str,
}

const REGISTRY: &[RegistryEntry] = &[
    RegistryEntry {
        scenario_id: "nucleus.agent.agent-chat-input",
        row: "agent-chat-input",
    },
    RegistryEntry {
        scenario_id: "nucleus.agent.agent-plan",
        row: "agent-plan",
    },
    RegistryEntry {
        scenario_id: "nucleus.agent.agent-question",
        row: "agent-question",
    },
    RegistryEntry {
        scenario_id: "nucleus.agent.agent-transcript",
        row: "agent-transcript",
    },
    RegistryEntry {
        scenario_id: "nucleus.shell.app-header",
        row: "app-header",
    },
    RegistryEntry {
        scenario_id: "nucleus.shell.button",
        row: "button",
    },
    RegistryEntry {
        scenario_id: "nucleus.settings.callout",
        row: "callout",
    },
    RegistryEntry {
        scenario_id: "nucleus.attention.command-palette",
        row: "command-palette",
    },
    RegistryEntry {
        scenario_id: "nucleus.settings.confirm-action",
        row: "confirm-action",
    },
    RegistryEntry {
        scenario_id: "nucleus.settings.detail-item",
        row: "detail-item",
    },
    RegistryEntry {
        scenario_id: "nucleus.navigation.dialog",
        row: "dialog",
    },
    RegistryEntry {
        scenario_id: "nucleus.navigation.editable-label",
        row: "editable-label",
    },
    RegistryEntry {
        scenario_id: "nucleus.shell.icon",
        row: "icon",
    },
    RegistryEntry {
        scenario_id: "nucleus.shell.icon-button",
        row: "icon-button",
    },
    RegistryEntry {
        scenario_id: "nucleus.navigation.menu",
        row: "menu",
    },
    RegistryEntry {
        scenario_id: "nucleus.attention.message-center",
        row: "message-center",
    },
    RegistryEntry {
        scenario_id: "nucleus.agent.model-picker",
        row: "model-picker",
    },
    RegistryEntry {
        scenario_id: "nucleus.navigation.popover",
        row: "popover",
    },
    RegistryEntry {
        scenario_id: "nucleus.settings.radio-group",
        row: "radio-group",
    },
    RegistryEntry {
        scenario_id: "nucleus.navigation.segmented-control",
        row: "segmented-control",
    },
    RegistryEntry {
        scenario_id: "nucleus.navigation.select",
        row: "select",
    },
    RegistryEntry {
        scenario_id: "nucleus.shell.split-view",
        row: "split-view",
    },
    RegistryEntry {
        scenario_id: "nucleus.agent.status-indicator",
        row: "status-indicator",
    },
    RegistryEntry {
        scenario_id: "nucleus.shell.surface",
        row: "surface",
    },
    RegistryEntry {
        scenario_id: "nucleus.settings.switch",
        row: "switch",
    },
    RegistryEntry {
        scenario_id: "nucleus.navigation.tabs",
        row: "tabs",
    },
    RegistryEntry {
        scenario_id: "nucleus.shell.text",
        row: "text",
    },
    RegistryEntry {
        scenario_id: "nucleus.settings.text-input",
        row: "text-input",
    },
    RegistryEntry {
        scenario_id: "nucleus.attention.toast-host",
        row: "toast-host",
    },
];

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CaptureViewport {
    width: u32,
    height: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Scenario {
    schema: String,
    component: String,
    scenario_id: String,
    props: Value,
    #[serde(default)]
    fixtures: Map<String, Value>,
    actions: Vec<Action>,
    #[allow(dead_code)]
    declared_states: Vec<String>,
    #[serde(default)]
    #[allow(dead_code)]
    web_only_exclusions: Vec<Exclusion>,
    capture: CaptureViewport,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Exclusion {
    attribute: String,
    reason: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum Action {
    PointerActivate { target: Target },
    Key { target: Target, key: String },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Target {
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CohortState {
    Initial,
    AfterActions,
}

#[derive(Debug)]
pub struct CohortArgs {
    pub scenario_id: String,
    pub state: CohortState,
    pub out_png: PathBuf,
    pub out_receipt: PathBuf,
}

const USAGE: &str = "usage: poodle-window-capture --cohort <scenario-id> --state <initial|after-actions> --out <png> --receipt <json>";

pub fn parse_args(argv: &[String]) -> Result<CohortArgs> {
    if argv.first().map(String::as_str) != Some("--cohort") {
        bail!("cohort mode must begin with --cohort\n{USAGE}");
    }
    let mut scenario_id = None;
    let mut state = None;
    let mut out_png = None;
    let mut out_receipt = None;
    let mut i = 0;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = argv
            .get(i + 1)
            .with_context(|| format!("missing value for {flag}\n{USAGE}"))?;
        i += 2;
        match flag {
            "--cohort" => {
                if scenario_id.replace(value.clone()).is_some() {
                    bail!("--cohort may be specified only once\n{USAGE}");
                }
                if registry_entry(value).is_none() {
                    bail!("unknown cohort scenario id '{value}': expected one of the 29 registered scenario ids");
                }
            }
            "--state" => {
                if state.is_some() {
                    bail!("--state may be specified only once\n{USAGE}");
                }
                state = Some(match value.as_str() {
                    "initial" => CohortState::Initial,
                    "after-actions" => CohortState::AfterActions,
                    other => {
                        bail!("unknown cohort state '{other}': expected initial or after-actions")
                    }
                });
            }
            "--out" => out_png = Some(PathBuf::from(value)),
            "--receipt" => out_receipt = Some(PathBuf::from(value)),
            other => bail!("argument '{other}' is not accepted in cohort mode\n{USAGE}"),
        }
    }
    let out_png = normalize_output_path(
        &out_png.with_context(|| format!("--out is required\n{USAGE}"))?,
        "--out",
    )?;
    let out_receipt = normalize_output_path(
        &out_receipt.with_context(|| format!("--receipt is required\n{USAGE}"))?,
        "--receipt",
    )?;
    if out_png == out_receipt {
        bail!("--out and --receipt must name different files");
    }
    Ok(CohortArgs {
        scenario_id: scenario_id.with_context(|| format!("--cohort is required\n{USAGE}"))?,
        state: state.with_context(|| format!("--state is required\n{USAGE}"))?,
        out_png,
        out_receipt,
    })
}

fn registry_entry(scenario_id: &str) -> Option<RegistryEntry> {
    REGISTRY
        .iter()
        .copied()
        .find(|entry| entry.scenario_id == scenario_id)
}

fn scenario_path(row: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../test/nucleus-a11y/scenarios")
        .join(format!("{row}.json"))
}

fn load_scenario(entry: RegistryEntry) -> Result<(Scenario, String)> {
    let path = scenario_path(entry.row);
    let bytes =
        std::fs::read(&path).with_context(|| format!("read cohort scenario {}", path.display()))?;
    let scenario: Scenario = serde_json::from_slice(&bytes)
        .with_context(|| format!("parse cohort scenario {}", path.display()))?;
    if scenario.schema != SCENARIO_SCHEMA {
        bail!("{} schema is not {SCENARIO_SCHEMA}", path.display());
    }
    if scenario.scenario_id != entry.scenario_id {
        bail!(
            "{} scenario_id {} does not match the closed registry id {}",
            path.display(),
            scenario.scenario_id,
            entry.scenario_id
        );
    }
    if scenario.capture.width == 0 || scenario.capture.height == 0 {
        bail!("{} capture dimensions must be positive", path.display());
    }
    let hash = format!("{:x}", Sha256::digest(&bytes));
    Ok((scenario, hash))
}

fn string(value: &Value, key: &str) -> String {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("missing string prop `{key}`"))
        .to_owned()
}

fn optional_string(value: &Value, key: &str) -> Option<String> {
    value[key].as_str().map(str::to_owned)
}

fn bool_or(value: &Value, key: &str, default: bool) -> bool {
    value[key].as_bool().unwrap_or(default)
}

fn size(value: &Value) -> ControlSize {
    match value["size"].as_str().unwrap_or("md") {
        "xs" => ControlSize::Xs,
        "sm" => ControlSize::Sm,
        "md" => ControlSize::Md,
        "lg" => ControlSize::Lg,
        "xl" => ControlSize::Xl,
        other => panic!("unknown control size `{other}`"),
    }
}

fn density(value: &Value) -> ControlDensity {
    match value["density"].as_str().unwrap_or("default") {
        "compact" => ControlDensity::Compact,
        "default" => ControlDensity::Default,
        "comfortable" => ControlDensity::Comfortable,
        other => panic!("unknown control density `{other}`"),
    }
}

fn icon_size(value: Option<&str>) -> IconSize {
    match value.unwrap_or("md") {
        "xs" => IconSize::Xs,
        "sm" => IconSize::Sm,
        "md" => IconSize::Md,
        "lg" => IconSize::Lg,
        "xl" => IconSize::Xl,
        other => panic!("unknown icon size `{other}`"),
    }
}

fn button_variant(value: Option<&str>) -> ButtonVariant {
    match value.unwrap_or("secondary") {
        "primary" => ButtonVariant::Primary,
        "secondary" => ButtonVariant::Secondary,
        "ghost" => ButtonVariant::Ghost,
        "danger" => ButtonVariant::Danger,
        other => panic!("unknown button variant `{other}`"),
    }
}

fn status_tone(value: &str) -> StatusTone {
    match value {
        "neutral" => StatusTone::Neutral,
        "info" => StatusTone::Info,
        "success" => StatusTone::Success,
        "warning" => StatusTone::Warning,
        "danger" => StatusTone::Danger,
        other => panic!("unknown status tone `{other}`"),
    }
}

fn text_size(value: Option<&str>) -> TextSize {
    match value.unwrap_or("md") {
        "xs" => TextSize::Xs,
        "sm" => TextSize::Sm,
        "md" => TextSize::Md,
        other => panic!("unknown text size `{other}`"),
    }
}

fn text_tone(value: Option<&str>) -> TextTone {
    match value.unwrap_or("default") {
        "default" => TextTone::Default,
        "secondary" => TextTone::Secondary,
        "muted" => TextTone::Muted,
        "success" => TextTone::Success,
        "danger" => TextTone::Danger,
        "warning" => TextTone::Warning,
        other => panic!("unknown text tone `{other}`"),
    }
}

fn text_weight(value: Option<&str>) -> TextWeight {
    match value.unwrap_or("normal") {
        "normal" => TextWeight::Normal,
        "medium" => TextWeight::Medium,
        "semibold" => TextWeight::Semibold,
        "bold" => TextWeight::Bold,
        other => panic!("unknown text weight `{other}`"),
    }
}

fn fixture_text(scenario: &Scenario) -> String {
    scenario.fixtures["panel_text"]
        .as_str()
        .unwrap_or_else(|| panic!("{} declares no panel_text fixture", scenario.scenario_id))
        .to_owned()
}

fn initial_state(scenario: &Scenario) -> HostState {
    HostState {
        plan_status: scenario.props["status"]
            .as_str()
            .unwrap_or("pending")
            .to_owned(),
        question_selections: scenario.props["selections"]
            .as_array()
            .map(|values| {
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default(),
        switch_checked: bool_or(&scenario.props, "defaultChecked", false),
        tabs_value: scenario.props["value"]
            .as_str()
            .or_else(|| scenario.props["defaultValue"].as_str())
            .unwrap_or("")
            .to_owned(),
        tabs_focused: scenario.props["value"]
            .as_str()
            .or_else(|| scenario.props["defaultValue"].as_str())
            .map(str::to_owned),
        model_open: false,
        select_open: false,
        select_value: scenario.props["value"]
            .as_str()
            .or_else(|| scenario.props["defaultValue"].as_str())
            .unwrap_or("")
            .to_owned(),
        confirm_open: false,
        detail_open: false,
        callout_dismissed: false,
        radio_value: scenario.props["value"]
            .as_str()
            .or_else(|| scenario.props["defaultValue"].as_str())
            .unwrap_or("")
            .to_owned(),
    }
}

#[derive(Clone, Debug)]
struct HostState {
    plan_status: String,
    question_selections: Vec<String>,
    switch_checked: bool,
    tabs_value: String,
    tabs_focused: Option<String>,
    model_open: bool,
    select_open: bool,
    select_value: String,
    confirm_open: bool,
    detail_open: bool,
    callout_dismissed: bool,
    radio_value: String,
}

struct CohortHost {
    scenario: Scenario,
    theme: poodle_gpui::GpuiThemeProvider,
    state: Mutex<HostState>,
    mounted: Mutex<Node>,
}

fn remount(host: &Arc<CohortHost>) {
    let mut node = build_node(host);
    stamp_semantic_ids(&mut node, &mut 0);
    *host.mounted.lock().expect("cohort mount lock") = node;
}

fn render_context<'a>(theme: &'a poodle_gpui::GpuiThemeProvider) -> RenderContext<'a> {
    RenderContext::new(theme)
}

fn build_node(host: &Arc<CohortHost>) -> Node {
    let scenario = &host.scenario;
    let props = &scenario.props;
    let state = host.state.lock().expect("cohort state lock").clone();
    let ctx = render_context(&host.theme);
    match scenario.component.as_str() {
        "Icon" => {
            let mut icon = poodle_render::icon(
                &IconSpec::new(string(props, "name"))
                    .with_size(icon_size(optional_string(props, "size").as_deref()))
                    .with_aria_label(string(props, "ariaLabel")),
                &ctx,
            );
            icon.id = Some("cohort-icon".to_owned());
            poodle_render::icon_provider(&IconProviderSpec::new(), &ctx, Some(icon))
        }
        "Text" => {
            let spec = TextSpec::new(fixture_text(scenario))
                .with_element(
                    match optional_string(props, "as").as_deref().unwrap_or("p") {
                        "p" => TextElement::P,
                        "span" => TextElement::Span,
                        "div" => TextElement::Div,
                        other => panic!("unknown text element `{other}`"),
                    },
                )
                .with_size(text_size(optional_string(props, "size").as_deref()))
                .with_tone(text_tone(optional_string(props, "tone").as_deref()))
                .with_weight(text_weight(optional_string(props, "weight").as_deref()));
            let mut probe = Node::container();
            probe.a11y.role = Some(NodeRole::Status);
            probe.a11y.label = Some(
                scenario.fixtures["a11y_probe"]["label"]
                    .as_str()
                    .expect("text probe label")
                    .to_owned(),
            );
            probe.child(poodle_render::text(&spec, &ctx))
        }
        "Surface" => poodle_render::surface(
            &SurfaceSpec::new()
                .with_tone(
                    match optional_string(props, "tone").as_deref().unwrap_or("panel") {
                        "panel" => SurfaceTone::Panel,
                        "canvas" => SurfaceTone::Canvas,
                        "elevated" => SurfaceTone::Elevated,
                        other => panic!("unknown surface tone `{other}`"),
                    },
                )
                .with_border(
                    match optional_string(props, "border")
                        .as_deref()
                        .unwrap_or("subtle")
                    {
                        "none" => SurfaceBorder::None,
                        "subtle" => SurfaceBorder::Subtle,
                        "default" => SurfaceBorder::Default,
                        other => panic!("unknown surface border `{other}`"),
                    },
                )
                .with_padding(
                    match optional_string(props, "padding").as_deref().unwrap_or("md") {
                        "sm" => PaddingScale::Sm,
                        "md" => PaddingScale::Md,
                        "lg" => PaddingScale::Lg,
                        other => panic!("unknown surface padding `{other}`"),
                    },
                )
                .with_role(match string(props, "asRole").as_str() {
                    "group" => SurfaceRole::Group,
                    "region" => SurfaceRole::Region,
                    other => panic!("unknown surface role `{other}`"),
                })
                .with_label(string(props, "label")),
            &ctx,
            vec![Node::text(fixture_text(scenario))],
        ),
        "Button" => poodle_render::button(
            &ButtonSpec::new()
                .with_label(fixture_text(scenario))
                .with_aria_label(string(props, "ariaLabel"))
                .with_variant(button_variant(optional_string(props, "variant").as_deref())),
            &ctx,
            None,
        ),
        "IconButton" => poodle_render::icon_button(
            &IconButtonSpec::new()
                .with_icon(string(props, "icon"))
                .with_aria_label(string(props, "ariaLabel")),
            &ctx,
            None,
        ),
        "SplitView" => {
            let mut spec = SplitViewSpec::new(
                "cohort",
                match string(props, "orientation").as_str() {
                    "horizontal" => SplitOrientation::Horizontal,
                    "vertical" => SplitOrientation::Vertical,
                    other => panic!("unknown split orientation `{other}`"),
                },
            )
            .with_default_ratio(props["defaultRatio"].as_f64().unwrap_or(0.5) as f32)
            .with_aria_label(string(props, "ariaLabel"))
            .with_show_collapse_primary(bool_or(props, "showCollapsePrimary", false));
            spec.divider = bool_or(props, "divider", true);
            poodle_render::split_view(&spec, &ctx, None, None, Default::default())
        }
        "AppHeader" => poodle_render::app_header(
            &AppHeaderSpec::new()
                .with_title(string(props, "title"))
                .with_subtitle(string(props, "subtitle"))
                .with_aria_label(string(props, "ariaLabel")),
            &ctx,
            None,
            None,
            None,
            None,
        ),
        "AgentPlan" => build_agent_plan(host, &ctx, &state),
        "StatusIndicator" => {
            let mut node = poodle_render::status_indicator(
                &StatusIndicatorSpec::new()
                    .with_status(status_tone(string(props, "status").as_str()))
                    .with_label(string(props, "label"))
                    .with_aria_label(string(props, "ariaLabel"))
                    .with_size(size(props))
                    .with_density(density(props)),
                &ctx,
            );
            node.id = Some("cohort-status-indicator".into());
            node.runtime_id = Some("cohort-status-indicator".into());
            node
        }
        "AgentTranscript" => build_agent_transcript(scenario, &ctx),
        "AgentQuestion" => build_agent_question(host, &ctx, &state),
        "ModelPicker" => build_model_picker(host, &ctx, &state),
        "AgentChatInput" => poodle_render::agent_chat_input(
            &AgentChatInputSpec::new()
                .with_value(string(props, "value"))
                .with_placeholder(string(props, "placeholder"))
                .with_aria_label(string(props, "ariaLabel"))
                .with_size(size(props))
                .with_density(density(props)),
            &ctx,
            vec![],
            vec![],
            vec![],
            vec![],
            AgentChatInputHandlers::default(),
        ),
        "Switch" => build_switch(host, &ctx, &state),
        "Tabs" => build_tabs(host, &ctx, &state),
        "Select" => build_select(host, &ctx, &state),
        "Dialog" => build_dialog(scenario, &ctx),
        "Popover" => build_popover(scenario, &ctx),
        "ConfirmAction" => build_confirm_action(host, &ctx, &state),
        "DetailItem" => build_detail_item(host, &ctx, &state),
        "CommandPalette" => build_command_palette(scenario, &ctx),
        "MessageCenter" => build_message_center(scenario, &ctx),
        "ToastHost" => build_toast_host(scenario, &ctx),
        "SegmentedControl" => build_segmented_control(scenario, &ctx),
        "Menu" => build_menu(scenario, &ctx),
        "RadioGroup" => build_radio_group(host, &ctx, &state),
        "Callout" => build_callout(host, &ctx, &state),
        "EditableLabel" => {
            let mut spec = EditableLabelSpec::new();
            spec.value = string(props, "value");
            spec.activation_mode = match optional_string(props, "activationMode").as_deref() {
                None | Some("enterOrSpace") => EditableLabelActivation::EnterOrSpace,
                Some("doubleClick") => EditableLabelActivation::DoubleClick,
                Some(other) => panic!("unknown editable-label activation `{other}`"),
            };
            spec.show_edit_icon = bool_or(props, "showEditIcon", false);
            poodle_render::editable_label(&spec, &ctx, None)
        }
        "TextInput" => {
            let mut spec = TextInputSpec::default();
            spec.id = optional_string(props, "id");
            spec.default_value = optional_string(props, "defaultValue").unwrap_or_default();
            spec.aria_label = optional_string(props, "ariaLabel");
            spec.placeholder = optional_string(props, "placeholder");
            poodle_render::text_input(&spec, &ctx, None)
        }
        other => panic!("unmapped cohort component `{other}`"),
    }
}

fn build_agent_plan(host: &Arc<CohortHost>, ctx: &RenderContext<'_>, state: &HostState) -> Node {
    let status = match state.plan_status.as_str() {
        "pending" => AgentPlanStatus::Pending,
        "accepted" => AgentPlanStatus::Accepted,
        "dismissed" => AgentPlanStatus::Dismissed,
        "revised" => AgentPlanStatus::Revised,
        other => panic!("unknown AgentPlan status `{other}`"),
    };
    let spec = AgentPlanSpec::new(string(&host.scenario.props, "plan"))
        .with_status(status)
        .with_dismissible(bool_or(&host.scenario.props, "dismissible", true))
        .with_size(size(&host.scenario.props))
        .with_density(density(&host.scenario.props));
    let accept_host = Arc::clone(host);
    let dismiss_host = Arc::clone(host);
    poodle_render::agent_plan(
        &spec,
        ctx,
        AgentPlanHandlers {
            on_accept: Some(Arc::new(move || {
                accept_host.state.lock().expect("plan state").plan_status = "accepted".into();
                remount(&accept_host);
            })),
            on_dismiss: Some(Arc::new(move || {
                dismiss_host.state.lock().expect("plan state").plan_status = "dismissed".into();
                remount(&dismiss_host);
            })),
            instance_id: Some(INSTANCE.into()),
            ..Default::default()
        },
    )
}

fn build_agent_transcript(scenario: &Scenario, ctx: &RenderContext<'_>) -> Node {
    let items = scenario.props["items"]
        .as_array()
        .expect("transcript items")
        .iter()
        .map(|item| match item["kind"].as_str() {
            Some("message") => TranscriptItem::Message(TranscriptMessage {
                id: string(item, "id"),
                role: Some(match string(item, "role").as_str() {
                    "user" => TranscriptRole::User,
                    "assistant" => TranscriptRole::Assistant,
                    other => panic!("unknown transcript role `{other}`"),
                }),
                markdown: string(item, "markdown"),
                ..Default::default()
            }),
            Some("activity") => {
                TranscriptItem::Activity(poodle_headless::agent_transcript::TranscriptActivity {
                    id: string(item, "id"),
                    label: string(item, "label"),
                    spinning: item["spinning"].as_bool(),
                })
            }
            other => panic!("unknown transcript item kind {other:?}"),
        })
        .collect();
    let spec = AgentTranscriptSpec::new(items)
        .with_virtualized(bool_or(&scenario.props, "virtualized", false))
        .with_aria_label(string(&scenario.props, "ariaLabel"))
        .with_size(size(&scenario.props))
        .with_density(density(&scenario.props));
    poodle_render::agent_transcript(&spec, ctx, AgentTranscriptHandlers::default())
}

fn build_agent_question(
    host: &Arc<CohortHost>,
    ctx: &RenderContext<'_>,
    state: &HostState,
) -> Node {
    let question_value = &host.scenario.props["questions"][0];
    let question = AgentQuestionItem {
        id: string(question_value, "id"),
        header: question_value["header"].as_str().map(str::to_owned),
        prompt: string(question_value, "prompt"),
        options: question_value["options"]
            .as_array()
            .expect("question options")
            .iter()
            .map(|option| AgentQuestionOption {
                value: string(option, "value"),
                label: string(option, "label"),
                description: optional_string(option, "description"),
            })
            .collect(),
        allow_multiple: bool_or(question_value, "allowMultiple", false),
    };
    let spec = AgentQuestionSpec::new(vec![question])
        .with_active_index(host.scenario.props["activeIndex"].as_u64().unwrap_or(0) as usize)
        .with_selections(state.question_selections.clone())
        .with_size(size(&host.scenario.props))
        .with_density(density(&host.scenario.props));
    let callback_host = Arc::clone(host);
    poodle_render::agent_question(
        &spec,
        ctx,
        AgentQuestionHandlers {
            instance_id: Some(INSTANCE.into()),
            on_select: Some(Arc::new(move |value| {
                callback_host
                    .state
                    .lock()
                    .expect("question state")
                    .question_selections = vec![value.into()];
                remount(&callback_host);
            })),
            ..Default::default()
        },
    )
}

fn build_model_picker(host: &Arc<CohortHost>, ctx: &RenderContext<'_>, state: &HostState) -> Node {
    let models = host.scenario.props["models"]
        .as_array()
        .expect("model picker models")
        .iter()
        .map(|model| {
            let mut option = ModelOption::new(string(model, "value"), string(model, "label"));
            if let Some(description) = optional_string(model, "description") {
                option = option.with_description(description);
            }
            if bool_or(model, "disabled", false) {
                option = option.with_disabled(true);
            }
            option
        })
        .collect();
    let spec = ModelPickerSpec::new()
        .with_models(models)
        .with_value(ModelSelection {
            model: string(&host.scenario.props["value"], "model"),
            axes: Vec::new(),
        })
        .with_aria_label(string(&host.scenario.props, "ariaLabel"))
        .with_variant(ModelPickerVariant::Outlined)
        .with_size(size(&host.scenario.props))
        .with_density(density(&host.scenario.props))
        .with_open(state.model_open);
    let mut node = poodle_render::model_picker(&spec, ctx, INSTANCE, None);
    if let Some(trigger) = node.children.first_mut() {
        let toggle_host = Arc::clone(host);
        trigger.interaction.on_activate = Some(Arc::new(move || {
            let mut state = toggle_host.state.lock().expect("model picker state");
            state.model_open = !state.model_open;
            drop(state);
            remount(&toggle_host);
        }));
    }
    node
}

fn build_switch(host: &Arc<CohortHost>, ctx: &RenderContext<'_>, state: &HostState) -> Node {
    let mut spec = SwitchSpec::new().with_checked(state.switch_checked);
    if let Some(label) = optional_string(&host.scenario.props, "label") {
        spec = spec.with_label(label);
    }
    if let Some(label) = optional_string(&host.scenario.props, "ariaLabel") {
        spec = spec.with_aria_label(label);
    }
    spec.is_disabled = bool_or(&host.scenario.props, "disabled", false);
    let callback_host = Arc::clone(host);
    let mut node = poodle_render::switch(
        &spec,
        ctx,
        Some(Arc::new(move |checked| {
            callback_host
                .state
                .lock()
                .expect("switch state")
                .switch_checked = checked;
            remount(&callback_host);
        })),
    );
    node.id = Some("cohort-switch".into());
    node
}

fn build_tabs(host: &Arc<CohortHost>, ctx: &RenderContext<'_>, state: &HostState) -> Node {
    let items = host.scenario.props["items"]
        .as_array()
        .expect("tabs items")
        .iter()
        .map(|item| {
            TabDefinition::new(string(item, "value"), string(item, "label"))
                .with_disabled(bool_or(item, "disabled", false))
                .with_closable(bool_or(item, "closable", false))
        })
        .collect();
    let mut spec = TabsSpec::new(items).with_value(state.tabs_value.clone());
    if let Some(label) = optional_string(&host.scenario.props, "ariaLabel") {
        spec = spec.with_aria_label(label);
    }
    spec = spec
        .with_orientation(
            match optional_string(&host.scenario.props, "orientation").as_deref() {
                None | Some("horizontal") => Orientation::Horizontal,
                Some("vertical") => Orientation::Vertical,
                Some(other) => panic!("unknown tabs orientation `{other}`"),
            },
        )
        .with_activation_mode(
            match optional_string(&host.scenario.props, "activationMode").as_deref() {
                None | Some("automatic") => TabActivationMode::Automatic,
                Some("manual") => TabActivationMode::Manual,
                Some(other) => panic!("unknown tabs activation mode `{other}`"),
            },
        );
    let change_host = Arc::clone(host);
    let focus_host = Arc::clone(host);
    let instance = INSTANCE.to_owned();
    let panel_text = fixture_text(&host.scenario);
    poodle_render::tabs_with_panel(
        &spec,
        ctx,
        TabsHandlers {
            on_change: Some(Arc::new(move |value| {
                let mut state = change_host.state.lock().expect("tabs state");
                state.tabs_value = value.into();
                state.tabs_focused = Some(value.into());
                drop(state);
                remount(&change_host);
            })),
            on_focus: Some(Arc::new(move |value| {
                poodle_gpui_node_backend::request_focus(&format!("tabs:{instance}:tab:{value}"));
                let mut state = focus_host.state.lock().expect("tabs state");
                state.tabs_focused = Some(value.into());
                drop(state);
                remount(&focus_host);
            })),
            focused_value: state.tabs_focused.clone(),
            instance_id: Some(INSTANCE.into()),
            ..Default::default()
        },
        Node::text(panel_text.replace("{value}", &state.tabs_value)),
    )
}

fn build_select(host: &Arc<CohortHost>, ctx: &RenderContext<'_>, state: &HostState) -> Node {
    let options = host.scenario.props["options"]
        .as_array()
        .expect("select options")
        .iter()
        .map(|option| {
            let mut choice = ChoiceOption::new(string(option, "value"), string(option, "label"));
            choice.is_disabled = bool_or(option, "disabled", false);
            choice
        })
        .collect();
    let mut spec = SelectSpec::new(options).with_open(state.select_open);
    if !state.select_value.is_empty() {
        spec = spec.with_value(state.select_value.clone());
    }
    if let Some(label) = optional_string(&host.scenario.props, "ariaLabel") {
        spec = spec.with_aria_label(label);
    }
    if let Some(placeholder) = optional_string(&host.scenario.props, "placeholder") {
        spec.placeholder = Some(placeholder);
    }
    spec = spec.with_mode(match host.scenario.props["native"].as_bool() {
        Some(false) => SelectMode::Custom,
        Some(true) => SelectMode::Native,
        None => SelectMode::Auto,
    });
    spec.is_disabled = bool_or(&host.scenario.props, "disabled", false);
    let callback_host = Arc::clone(host);
    let handlers =
        SelectHandlers::new(format!("{INSTANCE}:select")).on_transition(Arc::new(move |result| {
            let mut state = callback_host.state.lock().expect("select state");
            state.select_value = result.context.value;
            state.select_open = result.context.open;
            drop(state);
            remount(&callback_host);
        }));
    let mut node = poodle_render::select(&spec, ctx, &handlers);
    node.id = Some("cohort-select".into());
    node
}

fn build_dialog(scenario: &Scenario, ctx: &RenderContext<'_>) -> Node {
    let mut spec =
        DialogSpec::new().with_default_open(bool_or(&scenario.props, "defaultOpen", false));
    if let Some(value) = optional_string(&scenario.props, "title") {
        spec = spec.with_title(value);
    }
    if let Some(value) = optional_string(&scenario.props, "description") {
        spec = spec.with_description(value);
    }
    if let Some(value) = optional_string(&scenario.props, "ariaLabel") {
        spec = spec.with_aria_label(value);
    }
    if let Some(value) = optional_string(&scenario.props, "closeLabel") {
        spec = spec.with_close_label(value);
    }
    spec = spec
        .with_show_close_button(bool_or(&scenario.props, "showCloseButton", false))
        .with_dismiss_on_escape(bool_or(&scenario.props, "dismissOnEscape", true))
        .with_dismiss_on_backdrop(bool_or(&scenario.props, "dismissOnBackdrop", true));
    poodle_render::dialog(
        &spec,
        ctx,
        vec![Node::text(fixture_text(scenario))],
        None,
        None,
    )
}

fn build_popover(scenario: &Scenario, ctx: &RenderContext<'_>) -> Node {
    let mut spec =
        PopoverSpec::new().with_default_open(bool_or(&scenario.props, "defaultOpen", false));
    if let Some(value) = optional_string(&scenario.props, "ariaLabel") {
        spec = spec.with_aria_label(value);
    }
    if optional_string(&scenario.props, "initialFocus").as_deref() == Some("content") {
        spec = spec.with_initial_focus(PopoverInitialFocus::Content);
    }
    let handlers = PopoverHandlers {
        instance_id: Some(INSTANCE.into()),
        ..Default::default()
    };
    let trigger = poodle_render::button(&ButtonSpec::new().with_label("Settings"), ctx, None);
    poodle_render::popover(
        &spec,
        ctx,
        &handlers,
        Some(trigger),
        Some(Node::text(fixture_text(scenario))),
    )
}

fn build_confirm_action(
    host: &Arc<CohortHost>,
    ctx: &RenderContext<'_>,
    state: &HostState,
) -> Node {
    let mut spec = poodle_specs::ConfirmActionSpec::new(
        &string(&host.scenario.props, "title"),
        &string(&host.scenario.props, "description"),
        &string(&host.scenario.props, "confirmLabel"),
        &string(&host.scenario.props, "cancelLabel"),
    )
    .with_trigger_label(string(&host.scenario.props, "triggerLabel"))
    .with_open(state.confirm_open);
    spec.tone = status_tone(string(&host.scenario.props, "tone").as_str());
    let trigger_host = Arc::clone(host);
    let trigger = poodle_render::button(
        &ButtonSpec::new().with_label(string(&host.scenario.props, "triggerLabel")),
        ctx,
        Some(Arc::new(move || {
            trigger_host
                .state
                .lock()
                .expect("confirm state")
                .confirm_open = true;
            remount(&trigger_host);
        })),
    );
    poodle_render::confirm_action::confirm_action_with_slots_state(
        &spec,
        ctx,
        Some(trigger),
        None,
        false,
        "Working…",
        ConfirmActionHandlers::default(),
    )
}

fn build_detail_item(host: &Arc<CohortHost>, ctx: &RenderContext<'_>, state: &HostState) -> Node {
    let spec = poodle_specs::DetailItemSpec::new(string(&host.scenario.props, "label"))
        .with_value(string(&host.scenario.props, "value"))
        .with_description(string(&host.scenario.props, "description"));
    let callback_host = Arc::clone(host);
    poodle_render::detail_item_with_slots_state(
        &spec,
        ctx,
        None,
        None,
        state.detail_open,
        Some(Arc::new(move || {
            callback_host
                .state
                .lock()
                .expect("detail state")
                .detail_open = true;
            remount(&callback_host);
        })),
    )
}

fn build_command_palette(scenario: &Scenario, ctx: &RenderContext<'_>) -> Node {
    let actions = scenario.props["items"]
        .as_array()
        .expect("command palette items")
        .iter()
        .map(|item| {
            let mut action = CommandActionItem::new(string(item, "id"), string(item, "title"));
            if let Some(group) = optional_string(item, "group") {
                action = action.with_group(group);
            }
            if let Some(shortcut) = optional_string(item, "shortcut") {
                action = action.with_shortcut(shortcut);
            }
            action
        })
        .collect();
    let mut spec =
        CommandPaletteSpec::new(actions).with_open(bool_or(&scenario.props, "open", false));
    if let Some(value) = optional_string(&scenario.props, "title") {
        spec = spec.with_title(value);
    }
    if let Some(value) = optional_string(&scenario.props, "description") {
        spec = spec.with_description(value);
    }
    if let Some(value) = optional_string(&scenario.props, "invocationHint") {
        spec = spec.with_invocation_hint(value);
    }
    poodle_render::command_palette_with_handlers(
        &spec,
        ctx,
        CommandPaletteHandlers {
            instance_id: Some(INSTANCE.into()),
            ..Default::default()
        },
    )
}

fn build_message_center(scenario: &Scenario, ctx: &RenderContext<'_>) -> Node {
    let items = scenario.props["items"]
        .as_array()
        .expect("message center items")
        .iter()
        .map(|item| {
            let mut out = MessageCenterItem::new(string(item, "id"), string(item, "title"))
                .with_read(bool_or(item, "read", false));
            if let Some(value) = optional_string(item, "message") {
                out = out.with_message(value);
            }
            if let Some(value) = optional_string(item, "meta") {
                out = out.with_meta(value);
            }
            if let Some("success") = item["tone"].as_str() {
                out = out.with_tone(StatusTone::Success);
            }
            out
        })
        .collect();
    let open = bool_or(&scenario.props, "defaultOpen", false);
    let spec = MessageCenterSpec::new(items)
        .with_default_open(open)
        .with_open(open)
        .with_title(string(&scenario.props, "title"));
    poodle_render::message_center(
        &spec,
        ctx,
        MessageCenterHandlers {
            on_item_select: Some(Arc::new(|_| {})),
            instance_id: Some(INSTANCE.into()),
            ..Default::default()
        },
    )
}

fn build_toast_host(scenario: &Scenario, ctx: &RenderContext<'_>) -> Node {
    let toasts = scenario.props["toasts"]
        .as_array()
        .expect("toast host toasts")
        .iter()
        .map(|item| {
            let mut out = Toast::new(string(item, "id"), string(item, "title"));
            if let Some(value) = optional_string(item, "message") {
                out = out.with_message(value);
            }
            if item["tone"].as_str() == Some("success") {
                out = out.with_tone(poodle_specs::ToastTone::Success);
            }
            if item["tone"].as_str() == Some("danger") {
                out = out.with_tone(poodle_specs::ToastTone::Danger);
            }
            if let Some(value) = optional_string(item, "actionLabel") {
                out = out.with_action_label(value);
            }
            out
        })
        .collect();
    let placement = match string(&scenario.props, "placement").as_str() {
        "top-start" => ToastHostPlacement::TopStart,
        "top-end" => ToastHostPlacement::TopEnd,
        "bottom-start" => ToastHostPlacement::BottomStart,
        "bottom-end" => ToastHostPlacement::BottomEnd,
        other => panic!("unknown toast placement `{other}`"),
    };
    let host = ToastHostSpec::new()
        .with_auto_dismiss_ms(scenario.props["autoDismissMs"].as_u64().unwrap_or(0) as u32)
        .with_placement(placement)
        .with_aria_label(string(&scenario.props, "ariaLabel"));
    poodle_render::toast_host(
        &host,
        ctx,
        &ToastStackSpec::new().with_toasts(toasts),
        ToastStackHandlers {
            instance_id: Some(INSTANCE.into()),
            ..Default::default()
        },
    )
}

fn build_segmented_control(scenario: &Scenario, ctx: &RenderContext<'_>) -> Node {
    let options = scenario.props["options"]
        .as_array()
        .expect("segmented options")
        .iter()
        .map(|item| {
            SegmentedControlOption::new(string(item, "value"), string(item, "label"))
                .with_disabled(bool_or(item, "disabled", false))
        })
        .collect();
    let mut spec = SegmentedControlSpec::new("cohort", options);
    spec.default_value = scenario.props["defaultValue"].as_str().map(str::to_owned);
    spec.value = scenario.props["value"].as_str().map(str::to_owned);
    spec.aria_label = scenario.props["ariaLabel"].as_str().map(str::to_owned);
    spec.equal_width = bool_or(&scenario.props, "equalWidth", true);
    poodle_render::segmented_control(&spec, ctx, None)
}

fn menu_trigger(label: &str, text: &str) -> Node {
    let mut inner = Node::container();
    inner.a11y.role = Some(NodeRole::Button);
    inner.a11y.label = Some(text.to_owned());
    inner.interaction.focusable = true;
    inner.a11y.tab_index = Some(0);
    let mut trigger = Node::container().child(inner);
    trigger.a11y.role = Some(NodeRole::Button);
    trigger.a11y.label = Some(label.to_owned());
    trigger.interaction.focusable = true;
    trigger.a11y.tab_index = Some(0);
    trigger
}

fn build_menu(scenario: &Scenario, ctx: &RenderContext<'_>) -> Node {
    let items = scenario.props["items"]
        .as_array()
        .expect("menu items")
        .iter()
        .map(|item| {
            MenuEntry::new(string(item, "value"), string(item, "label"))
                .with_disabled(bool_or(item, "disabled", false))
                .with_destructive(item["tone"].as_str() == Some("danger"))
        })
        .collect();
    let mut spec =
        MenuSpec::new(items).with_default_open(bool_or(&scenario.props, "defaultOpen", false));
    if let Some(value) = optional_string(&scenario.props, "ariaLabel") {
        spec = spec.with_aria_label(value);
    }
    Node::container()
        .child(menu_trigger(
            &optional_string(&scenario.props, "triggerAriaLabel").unwrap_or_else(|| "Open".into()),
            scenario.fixtures["trigger_text"]
                .as_str()
                .expect("trigger_text fixture"),
        ))
        .child(poodle_render::menu(&spec, ctx, None))
}

fn build_radio_group(host: &Arc<CohortHost>, ctx: &RenderContext<'_>, state: &HostState) -> Node {
    let options = host.scenario.props["options"]
        .as_array()
        .expect("radio options")
        .iter()
        .map(|item| {
            let mut choice = ChoiceOption::new(string(item, "value"), string(item, "label"));
            choice.is_disabled = bool_or(item, "disabled", false);
            choice
        })
        .collect();
    let mut spec = RadioGroupSpec::new(options).with_value(state.radio_value.clone());
    spec.aria_label = optional_string(&host.scenario.props, "ariaLabel");
    spec.orientation = match optional_string(&host.scenario.props, "orientation").as_deref() {
        Some("horizontal") => Orientation::Horizontal,
        _ => Orientation::Vertical,
    };
    let callback_host = Arc::clone(host);
    poodle_render::radio_group(
        &spec,
        ctx,
        RadioGroupHandlers::new(INSTANCE).on_change(Arc::new(move |value| {
            callback_host.state.lock().expect("radio state").radio_value = value.into();
            remount(&callback_host);
        })),
    )
}

fn build_callout(host: &Arc<CohortHost>, ctx: &RenderContext<'_>, state: &HostState) -> Node {
    if state.callout_dismissed {
        return Node::container();
    }
    let mut spec = CallOutSpec::new()
        .with_tone(status_tone(string(&host.scenario.props, "tone").as_str()))
        .dismissible(bool_or(&host.scenario.props, "dismissible", false));
    spec.title = optional_string(&host.scenario.props, "title");
    spec.content = optional_string(&host.scenario.props, "message");
    spec.announce_mode = match optional_string(&host.scenario.props, "announceMode").as_deref() {
        None | Some("none") => CalloutAnnounceMode::None,
        Some("polite") => CalloutAnnounceMode::Polite,
        Some("assertive") => CalloutAnnounceMode::Assertive,
        Some(other) => panic!("unknown callout announce mode `{other}`"),
    };
    if let Some(value) = optional_string(&host.scenario.props, "dismissLabel") {
        spec.dismiss_label = value;
    }
    let callback_host = Arc::clone(host);
    poodle_render::callout(
        &spec,
        ctx,
        CalloutHandlers {
            on_dismiss: Some(Arc::new(move || {
                callback_host
                    .state
                    .lock()
                    .expect("callout state")
                    .callout_dismissed = true;
                remount(&callback_host);
            })),
            instance_id: Some(INSTANCE.into()),
            ..Default::default()
        },
    )
}

fn role_name(role: NodeRole) -> Option<&'static str> {
    Some(match role {
        NodeRole::Button => "button",
        NodeRole::RadioButton => "radio",
        NodeRole::Log => "log",
        NodeRole::Switch => "switch",
        NodeRole::Tab => "tab",
        NodeRole::TextInput => "textbox",
        NodeRole::ComboBox => "combobox",
        NodeRole::MenuItem => "menuitem",
        NodeRole::Dialog => "dialog",
        NodeRole::AlertDialog => "alertdialog",
        NodeRole::Status => "status",
        _ => return None,
    })
}

fn stamp_semantic_ids(node: &mut Node, next: &mut usize) {
    if (node.a11y.role.is_some() || node.a11y.label.is_some())
        && node.runtime_id.is_none()
        && node.id.is_none()
    {
        node.id = Some(format!("cohort:semantic:{next}"));
        *next += 1;
    }
    for child in &mut node.children {
        stamp_semantic_ids(child, next);
    }
}

fn find_target(node: &Node, target: &Target) -> Option<(String, gpui::Bounds<gpui::Pixels>)> {
    let role_matches = target.role.as_deref().map_or(true, |role| {
        node.a11y.role.and_then(role_name) == Some(role)
    });
    let name_matches = target.name.as_deref().map_or(true, |name| {
        node.a11y.label.as_deref().map(str::trim) == Some(name)
    });
    if role_matches && name_matches {
        if let Some(id) = node.runtime_id.as_deref().or(node.id.as_deref()) {
            if let Some(bounds) = poodle_gpui_node_backend::bounds_for(id) {
                return Some((id.to_owned(), bounds));
            }
        }
    }
    node.children
        .iter()
        .find_map(|child| find_target(child, target))
}

#[derive(Clone)]
enum ReplayPhase {
    Idle,
    PointerRelease { position: gpui::Point<gpui::Pixels> },
    KeySend { key: String },
    Wait { frames: u32 },
}

struct ReplayController {
    actions: Vec<Action>,
    next: usize,
    phase: ReplayPhase,
}

impl ReplayController {
    fn new(actions: Vec<Action>) -> Self {
        Self {
            actions,
            next: 0,
            phase: ReplayPhase::Idle,
        }
    }

    fn frame(
        &mut self,
        host: &Arc<CohortHost>,
        window: &mut Window,
        frame: u32,
    ) -> Result<transport::Settled> {
        if frame < transport::FRAMES_BEFORE_CAPTURE {
            return Ok(transport::Settled::Wait);
        }
        match &mut self.phase {
            ReplayPhase::Idle => {
                let Some(action) = self.actions.get(self.next) else {
                    return Ok(transport::Settled::Ready);
                };
                let target = match action {
                    Action::PointerActivate { target } | Action::Key { target, .. } => target,
                };
                let (id, bounds) =
                    find_target(&host.mounted.lock().expect("cohort mount lock"), target)
                        .with_context(|| {
                            format!("resolve cohort action target {:?}", target.name)
                        })?;
                let position = bounds.center();
                match action {
                    Action::PointerActivate { .. } => {
                        post_mouse_event(window, MouseEvent::Moved, position);
                        post_mouse_event(window, MouseEvent::Down, position);
                        self.phase = ReplayPhase::PointerRelease { position };
                    }
                    Action::Key { key, .. } => {
                        poodle_gpui_node_backend::request_focus(&id);
                        self.phase = ReplayPhase::KeySend { key: key.clone() };
                    }
                }
                Ok(transport::Settled::Wait)
            }
            ReplayPhase::PointerRelease { position } => {
                post_mouse_event(window, MouseEvent::Up, *position);
                self.phase = ReplayPhase::Wait { frames: 2 };
                Ok(transport::Settled::Wait)
            }
            ReplayPhase::KeySend { key } => {
                dispatch_key(key);
                self.phase = ReplayPhase::Wait { frames: 2 };
                Ok(transport::Settled::Wait)
            }
            ReplayPhase::Wait { frames } if *frames > 0 => {
                *frames -= 1;
                if *frames == 0 {
                    self.next += 1;
                    self.phase = ReplayPhase::Idle;
                }
                Ok(transport::Settled::Wait)
            }
            ReplayPhase::Wait { .. } => unreachable!(),
        }
    }
}

enum MouseEvent {
    Moved,
    Down,
    Up,
}

#[cfg(target_os = "macos")]
fn post_mouse_event(window: &mut Window, event: MouseEvent, position: gpui::Point<gpui::Pixels>) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSEvent, NSEventModifierFlags, NSEventType};
    use objc2_foundation::NSPoint;
    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let Some(ns_window) = app.windows().iter().next() else {
        return;
    };
    let event_type = match event {
        MouseEvent::Moved => NSEventType::MouseMoved,
        MouseEvent::Down => NSEventType::LeftMouseDown,
        MouseEvent::Up => NSEventType::LeftMouseUp,
    };
    let location = NSPoint {
        x: f64::from(f32::from(position.x)),
        y: f64::from(f32::from(window.viewport_size().height - position.y)),
    };
    let pressure = if matches!(event, MouseEvent::Down) {
        1.0
    } else {
        0.0
    };
    if let Some(event) = NSEvent::mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure(
        event_type,
        location,
        NSEventModifierFlags::empty(),
        0.0,
        ns_window.windowNumber(),
        None,
        0,
        1,
        pressure,
    ) {
        app.postEvent_atStart(&event, false);
    }
}

#[cfg(not(target_os = "macos"))]
fn post_mouse_event(
    _window: &mut Window,
    _event: MouseEvent,
    _position: gpui::Point<gpui::Pixels>,
) {
}

#[cfg(target_os = "macos")]
fn dispatch_key(key: &str) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSEvent, NSEventModifierFlags, NSEventType};
    use objc2_foundation::{NSPoint, NSString};
    let named = match key {
        "left" => Some(123),
        "right" => Some(124),
        "down" => Some(125),
        "up" => Some(126),
        "home" => Some(115),
        "end" => Some(119),
        "escape" => Some(53),
        "enter" => Some(36),
        "space" => Some(49),
        "tab" => Some(48),
        _ => None,
    };
    let Some(key_code) = named else { return };
    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let Some(ns_window) = app.windows().iter().next() else {
        return;
    };
    let chars = NSString::from_str("");
    for event_type in [NSEventType::KeyDown, NSEventType::KeyUp] {
        if let Some(event) = NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            event_type,
            NSPoint { x: 0.0, y: 0.0 },
            NSEventModifierFlags::empty(),
            0.0,
            ns_window.windowNumber(),
            None,
            &chars,
            &chars,
            false,
            key_code,
        ) {
            app.postEvent_atStart(&event, false);
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn dispatch_key(_key: &str) {}

struct CohortRoot {
    host: Arc<CohortHost>,
    canvas: gpui::Hsla,
}

impl Render for CohortRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let handle = window.window_handle();
        poodle_gpui_node_backend::overlay_frame_begin_for(handle, cx);
        cx.defer(move |_cx| poodle_gpui_node_backend::overlay_frame_end_for(handle));
        poodle_gpui_node_backend::reset_element_ids();
        let node = self.host.mounted.lock().expect("cohort mount lock").clone();
        let element: AnyElement = poodle_gpui_node_backend::to_gpui(&node);
        poodle_gpui_node_backend::attach_overlay_host(
            div()
                .size_full()
                .p(px(SCENE_PADDING))
                .flex()
                .items_start()
                .bg(self.canvas)
                .font_family("Inter")
                .child(element),
            handle,
        )
    }
}

#[derive(Serialize)]
struct CohortReceipt {
    schema: &'static str,
    scenario_id: String,
    state: CohortState,
    scenario_sha256: String,
    poodle_source_id: &'static str,
    gpui_source: &'static str,
    gpui_version: &'static str,
    transport: &'static str,
    focus: bool,
    foreground: transport::ForegroundEvidence,
    permission: &'static str,
    logical_viewport: [u32; 2],
    scale: f32,
    device_dimensions: [u32; 2],
    png_sha256: String,
}

pub fn run(args: &CohortArgs) -> ! {
    let prepared = prepare(args);
    let fonts = prepared
        .as_ref()
        .ok()
        .and_then(|_| crate::fixture_capture::inter_fonts().ok());
    match (prepared, fonts) {
        (Ok(shot), Some(fonts)) => transport::capture(
            crate::fixture_capture::FixtureAssets {
                base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            },
            fonts,
            shot,
        ),
        (Err(error), _) => {
            eprintln!("poodle-window-capture: {error:#}");
            std::process::exit(1)
        }
        (Ok(_), None) => {
            eprintln!("poodle-window-capture: the fixture Inter fonts could not be loaded");
            std::process::exit(1)
        }
    }
}

fn prepare(args: &CohortArgs) -> Result<transport::Shot<CohortRoot>> {
    let entry = registry_entry(&args.scenario_id)
        .with_context(|| format!("unknown cohort scenario id '{}'", args.scenario_id))?;
    let (scenario, scenario_sha256) = load_scenario(entry)?;
    let viewport = scenario.capture;
    let theme = ThemePreset::Eclipse.build_theme();
    let canvas = theme.resolve_color("color.background.canvas");
    let host = Arc::new(CohortHost {
        state: Mutex::new(initial_state(&scenario)),
        scenario,
        theme,
        mounted: Mutex::new(Node::container()),
    });
    remount(&host);
    let replay = Arc::new(Mutex::new(ReplayController::new(
        host.scenario.actions.clone(),
    )));
    let receipt_host = Arc::clone(&host);
    let receipt_state = args.state;
    let receipt_scenario_id = args.scenario_id.clone();
    let receipt_hash = scenario_sha256;
    let out_png = args.out_png.clone();
    let out_receipt = args.out_receipt.clone();
    let replay_frame = Arc::clone(&replay);
    Ok(transport::Shot {
        label: format!("cohort/{}:{:?}", args.scenario_id, args.state),
        logical_width: viewport.width as f32,
        logical_height: viewport.height as f32,
        build: Box::new(move |_window, cx: &mut App| {
            poodle_gpui_node_backend::reset_focus_registry();
            cx.new(|_| CohortRoot {
                host,
                canvas: poodle_gpui_node_backend::color(canvas),
            })
        }),
        on_frame: Box::new(move |window, _cx, frame| {
            if receipt_state == CohortState::Initial {
                return if frame >= transport::FRAMES_BEFORE_CAPTURE {
                    Ok(transport::Settled::Ready)
                } else {
                    Ok(transport::Settled::Wait)
                };
            }
            replay_frame
                .lock()
                .expect("cohort replay lock")
                .frame(&receipt_host, window, frame)
        }),
        finish: Box::new(move |facts| {
            transport::verify_device_size(
                viewport.width as f32,
                viewport.height as f32,
                facts.device_width,
                facts.device_height,
            )?;
            let receipt = CohortReceipt {
                schema: RECEIPT_SCHEMA,
                scenario_id: receipt_scenario_id,
                state: receipt_state,
                scenario_sha256: receipt_hash,
                poodle_source_id: POODLE_SOURCE_ID,
                gpui_source: GPUI_SOURCE,
                gpui_version: GPUI_VERSION,
                transport: TRANSPORT,
                focus: false,
                foreground: facts.foreground.clone(),
                permission: "screen-recording-required",
                logical_viewport: [viewport.width, viewport.height],
                scale: facts.scale,
                device_dimensions: [facts.device_width, facts.device_height],
                png_sha256: format!("{:x}", Sha256::digest(&facts.png)),
            };
            publish_pair(
                &out_png,
                &facts.png,
                &out_receipt,
                &serde_json::to_vec_pretty(&receipt)?,
            )
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(args: &[&str]) -> Vec<String> {
        args.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn registry_is_the_29_row_manifest() {
        assert_eq!(REGISTRY.len(), 29);
        let mut ids = REGISTRY
            .iter()
            .map(|entry| entry.scenario_id)
            .collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 29);
    }

    #[test]
    fn unknown_scenario_id_is_rejected_before_outputs_are_resolved() {
        let error = parse_args(&argv(&[
            "--cohort",
            "nucleus.unknown.row",
            "--state",
            "initial",
            "--out",
            "cohort.png",
            "--receipt",
            "cohort.json",
        ]))
        .expect_err("unknown cohort ids must fail");
        assert!(error.to_string().contains("unknown cohort scenario id"));
    }

    #[test]
    fn unknown_state_is_rejected() {
        assert!(parse_args(&argv(&[
            "--cohort",
            "nucleus.shell.button",
            "--state",
            "after",
            "--out",
            "cohort.png",
            "--receipt",
            "cohort.json",
        ]))
        .is_err());
    }

    #[test]
    fn both_cohort_states_parse() {
        for state in ["initial", "after-actions"] {
            let args = parse_args(&argv(&[
                "--cohort",
                "nucleus.shell.button",
                "--state",
                state,
                "--out",
                "cohort.png",
                "--receipt",
                "cohort.json",
            ]))
            .expect("canonical cohort invocation parses");
            assert_eq!(args.scenario_id, "nucleus.shell.button");
            assert_eq!(
                args.state,
                if state == "initial" {
                    CohortState::Initial
                } else {
                    CohortState::AfterActions
                }
            );
        }
    }

    #[test]
    fn every_scenario_file_loads_from_the_closed_registry() {
        for entry in REGISTRY {
            let (scenario, hash) = load_scenario(*entry).expect("registered scenario loads");
            assert_eq!(scenario.capture.width > 0, true);
            assert_eq!(scenario.capture.height > 0, true);
            assert_eq!(hash.len(), 64);
        }
    }

    #[test]
    fn every_registered_scenario_builds_through_the_production_renderer() {
        for entry in REGISTRY {
            let (scenario, _) = load_scenario(*entry).expect("registered scenario loads");
            let host = Arc::new(CohortHost {
                state: Mutex::new(initial_state(&scenario)),
                scenario,
                theme: ThemePreset::Eclipse.build_theme(),
                mounted: Mutex::new(Node::container()),
            });
            let mut node = build_node(&host);
            stamp_semantic_ids(&mut node, &mut 0);
            assert!(
                !node.children.is_empty() || node.a11y.role.is_some() || node.a11y.label.is_some(),
                "{} rendered an empty cohort node",
                entry.scenario_id
            );
        }
    }
}
