//! g16.111 — A1 paired accessibility proofs for the three foundation rows.
//!
//! Each proof mounts the row's production node tree through `HeadlessDriver`,
//! replays the shared scenario's actions through GPUI's real dispatch tree,
//! reads the mounted accessibility projection, and compares it with the
//! committed Svelte DOM snapshot produced from the same scenario file. The
//! A1 receipt is emitted only after the diff is empty.

use std::sync::{Arc, Mutex};

use poodle_headless::agent_plan::AgentPlanStatus;
use poodle_headless::agent_question::{AgentQuestionItem, AgentQuestionOption};
use poodle_headless::agent_transcript::{TranscriptItem, TranscriptMessage, TranscriptRole};
use poodle_node::Node;
use poodle_render::{
    AgentChatInputHandlers, AgentPlanHandlers, AgentQuestionHandlers, AgentTranscriptHandlers,
    CommandPaletteHandlers, ConfirmActionHandlers, EditableLabelHandlers, MessageCenterHandlers,
    PopoverHandlers, RenderContext, SelectHandlers, TabsHandlers, ToastStackHandlers,
};
use poodle_specs::{
    AgentChatInputSpec, AgentPlanSpec, AgentQuestionSpec, AgentTranscriptSpec, AppHeaderSpec,
    ButtonSpec, ButtonVariant, ChoiceOption, CommandActionItem, CommandPaletteSpec, ControlDensity,
    ControlSize, DialogSpec, EditableLabelActivation, EditableLabelSpec, IconButtonSpec,
    IconProviderSpec, IconSize, IconSpec, MenuEntry, MenuSpec, MessageCenterItem,
    MessageCenterSpec, ModelOption, ModelPickerSpec, ModelPickerVariant, ModelSelection,
    Orientation, PaddingScale, PopoverInitialFocus, PopoverSpec, SegmentedControlOption,
    SegmentedControlSpec, SelectMode, SelectSpec, SplitOrientation, SplitViewSpec,
    StatusIndicatorSpec, StatusTone, SurfaceBorder, SurfaceRole, SurfaceSpec, SurfaceTone,
    SwitchSpec, TabActivationMode, TabDefinition, TabsSpec, TextElement, TextSize, TextSpec,
    TextTone, TextWeight, Toast, ToastHostPlacement, ToastHostSpec, ToastStackSpec,
};
use serde::Deserialize;
use serde_json::Value;

use super::headless_driver::HeadlessDriver;
use super::nucleus_receipts::{self, A1Action, A1Target, LoadedA1Scenario};
use super::{run_headless, theme};

fn gpui_key(key: &str) -> &str {
    match key {
        "right" | "left" | "up" | "down" | "enter" | "space" | "escape" | "home" | "end"
        | "tab" => key,
        other => panic!("A1 scenario uses an unmapped key `{other}`"),
    }
}

/// The first mounted node, in document order, whose role and record name
/// match the shared target. Resolution reads the mounted tree, never a
/// runtime id from the scenario.
fn resolve_target(driver: &mut HeadlessDriver, target: &A1Target) -> Option<String> {
    let nodes = driver.accessibility_nodes();
    let node = nodes.iter().find(|node| {
        target
            .role
            .as_deref()
            .map_or(true, |role| nucleus_receipts::aria_role(node.role) == role)
            && target.name.as_deref().map_or(true, |name| {
                node.label.as_deref().map(str::trim) == Some(name)
            })
    });
    node.and_then(|node| (!node.element_id.is_empty()).then(|| node.element_id.clone()))
}

fn replay(driver: &mut HeadlessDriver, actions: &[A1Action]) {
    for action in actions {
        match action {
            A1Action::PointerActivate { target } => {
                let Some(id) = resolve_target(driver, target) else {
                    return;
                };
                assert!(
                    poodle_gpui_node_backend::bounds_for(&id).is_some(),
                    "A1 target `{id}` has no painted bounds"
                );
                driver.pointer_activate_id(&id);
            }
            A1Action::Key { target, key } => {
                let Some(id) = resolve_target(driver, target) else {
                    return;
                };
                driver.wait_for_focus_handle(&id);
                driver.keyboard_key(&id, gpui_key(key));
            }
        }
        // Host rebuilds land in the callback; two frames let the backend
        // mint and attach any focus handle the rebuilt tree introduced.
        driver.draw_frame();
        driver.draw_frame();
    }
}

/// Read the projection, cross-check gpui's real tab traversal against the
/// declared order, compare with the committed Svelte snapshot, and emit.
fn prove(
    driver: &mut HeadlessDriver,
    loaded: &LoadedA1Scenario,
    actions: &[&'static str],
    assertions: &[&'static str],
) {
    let raw = driver.accessibility_nodes();
    let nodes = nucleus_receipts::normalise_a1_nodes(&raw, &loaded.scenario);

    let candidates: Vec<String> = raw
        .iter()
        .filter(|node| node.focus_tracked)
        .map(|node| node.element_id.clone())
        .collect();
    let declared_tracked_order: Vec<String> = raw
        .iter()
        .filter(|node| node.focus_tracked && nucleus_receipts::is_sequential_tab_stop(node))
        .map(|node| node.element_id.clone())
        .collect();
    let traversal = driver.focus_traversal(&candidates, 16);
    let attributed: Vec<String> = traversal.iter().flatten().cloned().collect();
    assert_eq!(
        attributed, declared_tracked_order,
        "gpui sequential focus traversal {traversal:?} disagrees with the declared tab order"
    );

    // The Svelte snapshot is deserialised first: it must carry this
    // scenario file's hash before any comparison or publication happens.
    let (svelte_path, svelte_sha256, svelte_file) = nucleus_receipts::load_svelte_snapshot(loaded);
    let observation = driver.mounted_observation();
    let gpui_file = nucleus_receipts::gpui_snapshot_file(loaded, observation, nodes.clone());
    nucleus_receipts::check_committed_gpui_snapshot(loaded.row, &gpui_file);
    let svelte_nodes = svelte_file["nodes"].as_array().cloned().unwrap_or_default();
    let diff = nucleus_receipts::diff_a1_nodes(&nodes, &svelte_nodes);
    if !diff.is_empty() {
        nucleus_receipts::publish_a1_divergence_if_configured(loaded, &gpui_file, &diff);
        return;
    }
    nucleus_receipts::emit_a1_if_configured(
        loaded,
        observation,
        &gpui_file,
        &svelte_path,
        &svelte_sha256,
        &diff,
        actions,
        assertions,
    );
}

fn props<T: for<'de> Deserialize<'de>>(loaded: &LoadedA1Scenario) -> T {
    serde_json::from_value(loaded.scenario.props.clone()).unwrap_or_else(|error| {
        panic!("{} props do not map to the Rust spec: {error}", loaded.path)
    })
}

fn prop_string(props: &Value, key: &str) -> String {
    props[key]
        .as_str()
        .unwrap_or_else(|| panic!("missing string prop `{key}`"))
        .to_owned()
}

fn prop_size(props: &Value) -> ControlSize {
    match prop_string(props, "size").as_str() {
        "xs" => ControlSize::Xs,
        "sm" => ControlSize::Sm,
        "md" => ControlSize::Md,
        "lg" => ControlSize::Lg,
        "xl" => ControlSize::Xl,
        other => panic!("unmapped size `{other}`"),
    }
}

fn prop_density(props: &Value) -> ControlDensity {
    match prop_string(props, "density").as_str() {
        "compact" => ControlDensity::Compact,
        "default" => ControlDensity::Default,
        "comfortable" => ControlDensity::Comfortable,
        other => panic!("unmapped density `{other}`"),
    }
}

fn prove_static_row(row: &'static str, node: Node, width: f32, height: f32) {
    let loaded = nucleus_receipts::load_a1_scenario(row);
    run_headless(|cx| {
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), width, height);
        replay(&mut driver, &loaded.scenario.actions);
        // StatusIndicator is intentionally non-interactive, but its mounted
        // root still receives a real pointer dispatch so the receipt records
        // an observed native input path rather than a static render.
        if loaded.scenario.actions.is_empty() && row == "status-indicator" {
            driver.pointer_activate_id("status-indicator:a1");
            driver.draw_frame();
        }
        prove(
            &mut driver,
            &loaded,
            &["mount the production renderer path through HeadlessDriver", "replay every shared scenario action through GPUI dispatch"],
            &["the normalised GPUI snapshot is compared with the committed Svelte DOM snapshot for the same scenario hash"],
        );
    });
}
// ── NP-1 shell ───────────────────────────────────────────────────────────

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct IconProps {
    name: String,
    aria_label: Option<String>,
    size: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct TextProps {
    #[serde(rename = "as")]
    as_: Option<String>,
    tone: Option<String>,
    size: Option<String>,
    weight: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct SurfaceProps {
    tone: Option<String>,
    border: Option<String>,
    padding: Option<String>,
    as_role: Option<String>,
    label: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ButtonProps {
    aria_label: Option<String>,
    variant: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct IconButtonProps {
    icon: Option<String>,
    aria_label: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct AppHeaderProps {
    title: Option<String>,
    subtitle: Option<String>,
    aria_label: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct SplitViewProps {
    orientation: String,
    default_ratio: f32,
    aria_label: Option<String>,
    show_collapse_primary: bool,
    divider: bool,
}

fn fixture_text(loaded: &LoadedA1Scenario) -> String {
    loaded.scenario.fixtures["panel_text"]
        .as_str()
        .expect("panel_text fixture")
        .to_owned()
}

fn named<T>(value: Option<T>, name: &str) -> T {
    value.unwrap_or_else(|| panic!("missing {name}"))
}

fn text_size(value: Option<&str>) -> TextSize {
    match value.unwrap_or("md") {
        "xs" => TextSize::Xs,
        "sm" => TextSize::Sm,
        "md" => TextSize::Md,
        other => panic!("unknown text size {other}"),
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
        other => panic!("unknown text tone {other}"),
    }
}
fn text_weight(value: Option<&str>) -> TextWeight {
    match value.unwrap_or("normal") {
        "normal" => TextWeight::Normal,
        "medium" => TextWeight::Medium,
        "semibold" => TextWeight::Semibold,
        "bold" => TextWeight::Bold,
        other => panic!("unknown text weight {other}"),
    }
}
fn icon_size(value: Option<&str>) -> IconSize {
    match value.unwrap_or("md") {
        "xs" => IconSize::Xs,
        "sm" => IconSize::Sm,
        "md" => IconSize::Md,
        "lg" => IconSize::Lg,
        "xl" => IconSize::Xl,
        other => panic!("unknown icon size {other}"),
    }
}
fn button_variant(value: Option<&str>) -> ButtonVariant {
    match value.unwrap_or("secondary") {
        "primary" => ButtonVariant::Primary,
        "secondary" => ButtonVariant::Secondary,
        "ghost" => ButtonVariant::Ghost,
        "danger" => ButtonVariant::Danger,
        other => panic!("unknown button variant {other}"),
    }
}
fn split_orientation(value: &str) -> SplitOrientation {
    match value {
        "horizontal" => SplitOrientation::Horizontal,
        "vertical" => SplitOrientation::Vertical,
        other => panic!("unknown split orientation {other}"),
    }
}

fn prove_shell_row(
    driver: &mut HeadlessDriver,
    loaded: &LoadedA1Scenario,
    actions: &[&'static str],
    assertions: &[&'static str],
) {
    prove(driver, loaded, actions, assertions);
}

fn record_shell_divergence(
    driver: &mut HeadlessDriver,
    loaded: &LoadedA1Scenario,
    expected: &[(&str, &str)],
) {
    let nodes =
        nucleus_receipts::normalise_a1_nodes(&driver.accessibility_nodes(), &loaded.scenario);
    let (_, _, svelte) = nucleus_receipts::load_svelte_snapshot(loaded);
    let diff = nucleus_receipts::diff_a1_nodes(
        &nodes,
        svelte["nodes"].as_array().expect("snapshot nodes"),
    );
    let fields: Vec<(String, String)> = diff
        .iter()
        .map(|entry| {
            (
                entry["index"].as_i64().unwrap().to_string(),
                entry["field"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    assert_eq!(
        fields,
        expected
            .iter()
            .map(|(index, field)| ((*index).to_owned(), (*field).to_owned()))
            .collect::<Vec<_>>()
    );
    let gpui = nucleus_receipts::gpui_snapshot_file(loaded, driver.mounted_observation(), nodes);
    nucleus_receipts::publish_a1_divergence_if_configured(loaded, &gpui, &diff);
}

#[test]
fn np1_icon_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("icon");
    let input: IconProps = props(&loaded);
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut icon = poodle_render::icon(
            &IconSpec::new(input.name.clone())
                .with_size(icon_size(input.size.as_deref()))
                .with_aria_label(named(input.aria_label.clone(), "ariaLabel")),
            &ctx,
        );
        icon.id = Some("np1-icon".to_owned());
        let root = poodle_render::icon_provider(&IconProviderSpec::new(), &ctx, Some(icon));
        let mounted = Arc::new(Mutex::new(root));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 120.0, 80.0);
        driver.pointer_hover(gpui::Point::new(
            gpui::Pixels::from(20.0),
            gpui::Pixels::from(20.0),
        ));
        prove_shell_row(
            &mut driver,
            &loaded,
            &["mount production Icon through IconProvider and dispatch pointer hover"],
            &["mounted Icon accessibility projection matches Svelte"],
        );
    });
}

#[test]
fn np1_text_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("text");
    let input: TextProps = props(&loaded);
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let text = TextSpec::new(fixture_text(&loaded))
            .with_element(match input.as_.as_deref().unwrap_or("p") {
                "p" => TextElement::P,
                "span" => TextElement::Span,
                "div" => TextElement::Div,
                other => panic!("unknown text element {other}"),
            })
            .with_size(text_size(input.size.as_deref()))
            .with_tone(text_tone(input.tone.as_deref()))
            .with_weight(text_weight(input.weight.as_deref()));
        let mut probe = Node::container();
        probe.a11y.role = Some(poodle_node::NodeRole::Status);
        probe.a11y.label = Some(
            loaded.scenario.fixtures["a11y_probe"]["label"]
                .as_str()
                .expect("probe label")
                .to_owned(),
        );
        let node = probe.child(poodle_render::text(&text, &ctx));
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 160.0, 60.0);
        driver.dispatch_probe_key("tab");
        prove_shell_row(
            &mut driver,
            &loaded,
            &["mount production Text and dispatch a harmless key"],
            &["styled Text remains outside the accessibility projection on both runtimes"],
        );
    });
}

#[test]
fn np1_surface_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("surface");
    let input: SurfaceProps = props(&loaded);
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = poodle_render::surface(
            &SurfaceSpec::new()
                .with_tone(match input.tone.as_deref().unwrap_or("panel") {
                    "panel" => SurfaceTone::Panel,
                    "canvas" => SurfaceTone::Canvas,
                    "elevated" => SurfaceTone::Elevated,
                    other => panic!("unknown surface tone {other}"),
                })
                .with_border(match input.border.as_deref().unwrap_or("subtle") {
                    "none" => SurfaceBorder::None,
                    "subtle" => SurfaceBorder::Subtle,
                    "default" => SurfaceBorder::Default,
                    other => panic!("unknown surface border {other}"),
                })
                .with_padding(match input.padding.as_deref().unwrap_or("md") {
                    "sm" => PaddingScale::Sm,
                    "md" => PaddingScale::Md,
                    "lg" => PaddingScale::Lg,
                    other => panic!("unknown surface padding {other}"),
                })
                .with_role(match input.as_role.as_deref().expect("asRole") {
                    "group" => SurfaceRole::Group,
                    "region" => SurfaceRole::Region,
                    other => panic!("unknown surface role {other}"),
                })
                .with_label(named(input.label.clone(), "label")),
            &ctx,
            vec![],
        );
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 220.0, 100.0);
        driver.dispatch_probe_key("tab");
        prove_shell_row(
            &mut driver,
            &loaded,
            &["mount production Surface and dispatch a harmless key"],
            &["region role and accessible name match the Svelte projection"],
        );
    });
}

#[test]
fn np1_button_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("button");
    let input: ButtonProps = props(&loaded);
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut node = poodle_render::button(
            &ButtonSpec::new()
                .with_label(fixture_text(&loaded))
                .with_aria_label(named(input.aria_label.clone(), "ariaLabel"))
                .with_variant(button_variant(input.variant.as_deref())),
            &ctx,
            None,
        );
        node.id = Some("np1-button".to_owned());
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 160.0, 60.0);
        replay(&mut driver, &loaded.scenario.actions);
        prove_shell_row(
            &mut driver,
            &loaded,
            &["mount production Button and replay the shared pointer action"],
            &["button role, name, disabled state, and focus match the Svelte projection"],
        );
    });
}

#[test]
fn np1_icon_button_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("icon-button");
    let input: IconButtonProps = props(&loaded);
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut node = poodle_render::icon_button(
            &IconButtonSpec::new()
                .with_icon(named(input.icon.clone(), "icon"))
                .with_aria_label(named(input.aria_label.clone(), "ariaLabel")),
            &ctx,
            None,
        );
        node.id = Some("np1-icon-button".to_owned());
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 100.0, 100.0);
        replay(&mut driver, &loaded.scenario.actions);
        prove_shell_row(
            &mut driver,
            &loaded,
            &["mount production IconButton and replay the shared pointer action"],
            &["button role, name, disabled state, and focus match the Svelte projection"],
        );
    });
}

#[test]
fn np1_split_view_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("split-view");
    let input: SplitViewProps = props(&loaded);
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut spec = SplitViewSpec::new("np1", split_orientation(&input.orientation))
            .with_default_ratio(input.default_ratio)
            .with_aria_label(named(input.aria_label.clone(), "ariaLabel"))
            .with_show_collapse_primary(input.show_collapse_primary);
        spec.divider = input.divider;
        let node = poodle_render::split_view(
            &spec,
            &ctx,
            None,
            None,
            poodle_render::SplitViewHandlers::default(),
        );
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 360.0, 180.0);
        driver.dispatch_probe_key("tab");
        record_shell_divergence(&mut driver, &loaded, &[("0", "value"), ("1", "name")]);
    });
}

#[test]
fn np1_app_header_a1_accessibility_divergence_is_recorded() {
    let loaded = nucleus_receipts::load_a1_scenario("app-header");
    let input: AppHeaderProps = props(&loaded);
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = poodle_render::app_header(
            &AppHeaderSpec::new()
                .with_title(named(input.title.clone(), "title"))
                .with_subtitle(named(input.subtitle.clone(), "subtitle"))
                .with_aria_label(named(input.aria_label.clone(), "ariaLabel")),
            &ctx,
            None,
            None,
            None,
            None,
        );
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 360.0, 80.0);
        driver.dispatch_probe_key("tab");
        record_shell_divergence(&mut driver, &loaded, &[("0", "role")]);
    });
}

#[test]
fn agent_plan_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("agent-plan");
    let scenario_props = loaded.scenario.props.clone();
    let theme_provider = theme();
    let ctx = RenderContext::new(&theme_provider);
    let status = match prop_string(&scenario_props, "status").as_str() {
        "pending" => AgentPlanStatus::Pending,
        "accepted" => AgentPlanStatus::Accepted,
        "dismissed" => AgentPlanStatus::Dismissed,
        "revised" => AgentPlanStatus::Revised,
        other => panic!("unmapped plan status `{other}`"),
    };
    let spec = AgentPlanSpec::new(prop_string(&scenario_props, "plan"))
        .with_status(status)
        .with_dismissible(scenario_props["dismissible"].as_bool().unwrap_or(true))
        .with_size(prop_size(&scenario_props))
        .with_density(prop_density(&scenario_props));
    let handlers = AgentPlanHandlers {
        instance_id: Some("a1".to_owned()),
        ..Default::default()
    };
    let node = poodle_render::agent_plan(&spec, &ctx, handlers);
    run_headless(|cx| {
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 180.0);
        driver.wait_for_focus_handle("agent-plan:a1:accept");
        replay(&mut driver, &loaded.scenario.actions);
        prove(
            &mut driver,
            &loaded,
            &[
                "mount AgentPlan through the production renderer and HeadlessDriver",
                "replay the shared Accept action through GPUI dispatch",
            ],
            &["plan heading and decision controls match the Svelte accessibility projection"],
        );
    });
}

#[test]
fn status_indicator_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("status-indicator");
    let scenario_props = loaded.scenario.props;
    let provider = theme();
    let mut node = poodle_render::status_indicator(
        &StatusIndicatorSpec::new()
            .with_status(poodle_specs::StatusTone::Success)
            .with_label(prop_string(&scenario_props, "label"))
            .with_aria_label(prop_string(&scenario_props, "ariaLabel"))
            .with_size(prop_size(&scenario_props))
            .with_density(prop_density(&scenario_props)),
        &RenderContext::new(&provider),
    );
    node.id = Some("status-indicator:a1".into());
    node.runtime_id = Some("status-indicator:a1".into());
    prove_static_row("status-indicator", node, 240.0, 60.0);
}

#[test]
fn agent_transcript_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("agent-transcript");
    let scenario_props = loaded.scenario.props.clone();
    let provider = theme();
    let items = scenario_props["items"]
        .as_array()
        .expect("transcript items")
        .iter()
        .map(|item| match item["kind"].as_str() {
            Some("message") => TranscriptItem::Message(TranscriptMessage {
                id: prop_string(item, "id"),
                role: Some(match prop_string(item, "role").as_str() {
                    "user" => TranscriptRole::User,
                    "assistant" => TranscriptRole::Assistant,
                    other => panic!("unmapped transcript role `{other}`"),
                }),
                markdown: prop_string(item, "markdown"),
                ..Default::default()
            }),
            Some("activity") => {
                TranscriptItem::Activity(poodle_headless::agent_transcript::TranscriptActivity {
                    id: prop_string(item, "id"),
                    label: prop_string(item, "label"),
                    spinning: item["spinning"].as_bool(),
                })
            }
            other => panic!("unmapped transcript item kind {other:?}"),
        })
        .collect();
    let spec = AgentTranscriptSpec::new(items)
        .with_virtualized(scenario_props["virtualized"].as_bool().unwrap_or(false))
        .with_aria_label(prop_string(&scenario_props, "ariaLabel"))
        .with_size(prop_size(&scenario_props))
        .with_density(prop_density(&scenario_props));
    let node = poodle_render::agent_transcript(
        &spec,
        &RenderContext::new(&provider),
        AgentTranscriptHandlers::default(),
    );
    prove_static_row("agent-transcript", node, 520.0, 240.0);
}

#[test]
fn agent_question_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("agent-question");
    let scenario_props = loaded.scenario.props.clone();
    let provider = theme();
    let question_value = &scenario_props["questions"][0];
    let question = AgentQuestionItem {
        id: prop_string(question_value, "id"),
        header: question_value["header"].as_str().map(str::to_owned),
        prompt: prop_string(question_value, "prompt"),
        options: question_value["options"]
            .as_array()
            .expect("question options")
            .iter()
            .map(|option| AgentQuestionOption {
                value: prop_string(option, "value"),
                label: prop_string(option, "label"),
                description: option["description"].as_str().map(str::to_owned),
            })
            .collect(),
        allow_multiple: question_value["allowMultiple"].as_bool().unwrap_or(false),
    };
    let spec = AgentQuestionSpec::new(vec![question])
        .with_active_index(scenario_props["activeIndex"].as_u64().unwrap_or(0) as usize)
        .with_selections(
            scenario_props["selections"]
                .as_array()
                .expect("selections")
                .iter()
                .map(|v| v.as_str().expect("selection string").to_owned())
                .collect(),
        )
        .with_size(prop_size(&scenario_props))
        .with_density(prop_density(&scenario_props));
    let host = Arc::new(Mutex::new(spec));
    let mounted = Arc::new(Mutex::new(Node::container()));
    let callback_host = Arc::clone(&host);
    let callback_mounted = Arc::clone(&mounted);
    let build = move || {
        let current = callback_host.lock().expect("question host").clone();
        let callback_provider = provider.clone();
        poodle_render::agent_question(
            &current,
            &RenderContext::new(&provider),
            AgentQuestionHandlers {
                instance_id: Some("a1".into()),
                on_select: Some(Arc::new({
                    let callback_host = Arc::clone(&callback_host);
                    let callback_mounted = Arc::clone(&callback_mounted);
                    move |value| {
                        let mut next = callback_host.lock().expect("question host").clone();
                        next.selections = vec![value.to_owned()];
                        *callback_host.lock().expect("question host") = next.clone();
                        *callback_mounted.lock().expect("question mount") =
                            poodle_render::agent_question(
                                &next,
                                &RenderContext::new(&callback_provider),
                                AgentQuestionHandlers::default(),
                            );
                    }
                })),
                ..Default::default()
            },
        )
    };
    *mounted.lock().expect("question mount") = build();
    let loaded = nucleus_receipts::load_a1_scenario("agent-question");
    run_headless(|cx| {
        let mounted = Arc::clone(&mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, mounted, 520.0, 240.0);
        replay(&mut driver, &loaded.scenario.actions);
        prove(
            &mut driver,
            &loaded,
            &[
                "mount AgentQuestion through the production renderer",
                "replay the shared option action through GPUI dispatch and rebuild the host",
            ],
            &["the checked radio projection matches Svelte after the action"],
        );
    });
}

#[test]
fn model_picker_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("model-picker");
    let scenario_props = loaded.scenario.props.clone();
    let provider = theme();
    let models = scenario_props["models"]
        .as_array()
        .expect("models")
        .iter()
        .map(|model| {
            let mut option =
                ModelOption::new(prop_string(model, "value"), prop_string(model, "label"));
            if let Some(description) = model["description"].as_str() {
                option = option.with_description(description);
            }
            if model["disabled"].as_bool().unwrap_or(false) {
                option = option.with_disabled(true);
            }
            option
        })
        .collect();
    let spec = ModelPickerSpec::new()
        .with_models(models)
        .with_value(ModelSelection {
            model: prop_string(&scenario_props["value"], "model"),
            axes: Vec::new(),
        })
        .with_aria_label(prop_string(&scenario_props, "ariaLabel"))
        .with_variant(ModelPickerVariant::Outlined)
        .with_size(prop_size(&scenario_props))
        .with_density(prop_density(&scenario_props));
    let spec = spec.with_open(true);
    let node = poodle_render::model_picker(&spec, &RenderContext::new(&provider), "a1", None);
    prove_static_row("model-picker", node, 520.0, 300.0);
}

#[test]
fn agent_chat_input_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("agent-chat-input");
    let scenario_props = loaded.scenario.props.clone();
    let provider = theme();
    let spec = AgentChatInputSpec::new()
        .with_value(prop_string(&scenario_props, "value"))
        .with_placeholder(prop_string(&scenario_props, "placeholder"))
        .with_aria_label(prop_string(&scenario_props, "ariaLabel"))
        .with_size(prop_size(&scenario_props))
        .with_density(prop_density(&scenario_props));
    let node = poodle_render::agent_chat_input(
        &spec,
        &RenderContext::new(&provider),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        AgentChatInputHandlers::default(),
    );
    prove_static_row("agent-chat-input", node, 640.0, 240.0);
}

// ── Switch ─────────────────────────────────────────────────────────────────

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct SwitchProps {
    label: Option<String>,
    aria_label: Option<String>,
    default_checked: Option<bool>,
    disabled: Option<bool>,
}

fn build_switch(checked: bool, props: &SwitchProps, mounted: &Arc<Mutex<Node>>) -> Node {
    let mut spec = SwitchSpec::new().with_checked(checked);
    if let Some(label) = &props.label {
        spec = spec.with_label(label.clone());
    }
    if let Some(aria_label) = &props.aria_label {
        spec = spec.with_aria_label(aria_label.clone());
    }
    spec.is_disabled = props.disabled.unwrap_or(false);
    let handler = {
        let props = props.clone();
        let mounted = Arc::clone(mounted);
        Arc::new(move |next: bool| {
            let rebuilt = build_switch(next, &props, &mounted);
            *mounted.lock().expect("mount lock") = rebuilt;
        })
    };
    let theme_provider = theme();
    let mut node =
        poodle_render::switch(&spec, &RenderContext::new(&theme_provider), Some(handler));
    node.id = Some("poodle-switch-a1".to_owned());
    node
}

#[test]
fn switch_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("switch");
    let switch_props: SwitchProps = props(&loaded);
    run_headless(|cx| {
        let mounted = Arc::new(Mutex::new(Node::container()));
        let initial = build_switch(
            switch_props.default_checked.unwrap_or(false),
            &switch_props,
            &mounted,
        );
        *mounted.lock().expect("mount lock") = initial;
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 200.0, 60.0);
        driver.wait_for_focus_handle("poodle-switch-a1");
        replay(&mut driver, &loaded.scenario.actions);
        prove(
            &mut driver,
            &loaded,
            &[
                "mount the production Switch node tree through HeadlessDriver with the shared scenario props",
                "replay the shared scenario actions through GPUI test-platform dispatch (pointer activate resolved by role and name)",
                "read the mounted accessibility projection and the backend focus registry after dispatch",
                "execute gpui sequential focus traversal and attribute every stop to a tracked node",
            ],
            &[
                "the normalised GPUI snapshot equals the committed Svelte DOM snapshot for the same scenario hash",
                "gpui tab traversal visits the tracked focusable nodes in the declared order",
                "switch role, accessible name, checked state, and focus match the Svelte ARIA projection",
            ],
        );
    });
}

// ── Tabs ───────────────────────────────────────────────────────────────────

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct TabItemProps {
    value: String,
    label: String,
    disabled: Option<bool>,
    closable: Option<bool>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct TabsProps {
    items: Vec<TabItemProps>,
    default_value: Option<String>,
    value: Option<String>,
    aria_label: Option<String>,
    orientation: Option<String>,
    activation_mode: Option<String>,
}

#[derive(Clone)]
struct TabsState {
    value: String,
    focused: Option<String>,
}

fn build_tabs(
    state: &TabsState,
    props: &TabsProps,
    panel_text: &str,
    mounted: &Arc<Mutex<Node>>,
) -> Node {
    const INSTANCE: &str = "a1";
    let items = props
        .items
        .iter()
        .map(|item| {
            TabDefinition::new(item.value.clone(), item.label.clone())
                .with_disabled(item.disabled.unwrap_or(false))
                .with_closable(item.closable.unwrap_or(false))
        })
        .collect();
    let mut spec = TabsSpec::new(items).with_value(state.value.clone());
    if let Some(label) = &props.aria_label {
        spec = spec.with_aria_label(label.clone());
    }
    spec = spec.with_orientation(match props.orientation.as_deref() {
        None | Some("horizontal") => Orientation::Horizontal,
        Some("vertical") => Orientation::Vertical,
        Some(other) => panic!("unmapped tabs orientation `{other}`"),
    });
    spec = spec.with_activation_mode(match props.activation_mode.as_deref() {
        None | Some("automatic") => TabActivationMode::Automatic,
        Some("manual") => TabActivationMode::Manual,
        Some(other) => panic!("unmapped tabs activation mode `{other}`"),
    });

    let rebuild = {
        let props = props.clone();
        let panel_text = panel_text.to_owned();
        let mounted = Arc::clone(mounted);
        move |next: TabsState| {
            let rebuilt = build_tabs(&next, &props, &panel_text, &mounted);
            *mounted.lock().expect("mount lock") = rebuilt;
        }
    };
    let handlers = TabsHandlers {
        on_change: Some({
            let rebuild = rebuild.clone();
            let state = state.clone();
            Arc::new(move |value: &str| {
                let mut next = state.clone();
                next.value = value.to_owned();
                next.focused = Some(value.to_owned());
                rebuild(next);
            })
        }),
        on_focus: Some({
            let rebuild = rebuild.clone();
            let state = state.clone();
            Arc::new(move |value: &str| {
                poodle_gpui_node_backend::request_focus(&format!("tabs:{INSTANCE}:tab:{value}"));
                let mut next = state.clone();
                next.focused = Some(value.to_owned());
                rebuild(next);
            })
        }),
        focused_value: state.focused.clone(),
        instance_id: Some(INSTANCE.to_owned()),
        ..TabsHandlers::default()
    };
    let panel = Node::text(panel_text.replace("{value}", &state.value));
    let theme_provider = theme();
    poodle_render::tabs_with_panel(&spec, &RenderContext::new(&theme_provider), handlers, panel)
}

#[test]
fn tabs_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("tabs");
    let tabs_props: TabsProps = props(&loaded);
    let panel_text = loaded
        .scenario
        .fixtures
        .get("panel_text")
        .and_then(Value::as_str)
        .expect("tabs scenario declares fixtures.panel_text")
        .to_owned();
    let initial_value = tabs_props
        .value
        .clone()
        .or_else(|| tabs_props.default_value.clone())
        .expect("tabs scenario declares a value");
    run_headless(|cx| {
        let mounted = Arc::new(Mutex::new(Node::container()));
        let state = TabsState {
            value: initial_value.clone(),
            focused: Some(initial_value.clone()),
        };
        let initial = build_tabs(&state, &tabs_props, &panel_text, &mounted);
        *mounted.lock().expect("mount lock") = initial;
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 160.0);
        driver.wait_for_focus_handle(&format!("tabs:a1:tab:{initial_value}"));
        replay(&mut driver, &loaded.scenario.actions);
        prove(
            &mut driver,
            &loaded,
            &[
                "mount the production Tabs node tree with a panel through HeadlessDriver with the shared scenario props",
                "replay the shared scenario actions through GPUI test-platform dispatch (pointer activate a tab, then ArrowRight on it)",
                "read the mounted accessibility projection and the backend focus registry after the host rebuilds",
                "execute gpui sequential focus traversal and attribute every stop to a tracked node",
            ],
            &[
                "the normalised GPUI snapshot equals the committed Svelte DOM snapshot for the same scenario hash",
                "tablist, tab, and tabpanel roles, names, selected state, controls and labelled-by indices, orientation, and roving focus order match the Svelte ARIA projection",
                "gpui tab traversal visits the tracked focusable nodes in the declared order",
            ],
        );
    });
}

// ── Select ─────────────────────────────────────────────────────────────────

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct SelectOptionProps {
    value: String,
    label: String,
    disabled: Option<bool>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct SelectProps {
    options: Vec<SelectOptionProps>,
    default_value: Option<String>,
    value: Option<String>,
    aria_label: Option<String>,
    placeholder: Option<String>,
    native: Option<bool>,
    disabled: Option<bool>,
}

fn build_select(
    spec: &SelectSpec,
    host: &Arc<Mutex<SelectSpec>>,
    mounted: &Arc<Mutex<Node>>,
) -> Node {
    let handler = {
        let host = Arc::clone(host);
        let mounted = Arc::clone(mounted);
        Arc::new(move |result: poodle_render::SelectTransitionResult| {
            let next = host
                .lock()
                .expect("host lock")
                .clone()
                .applying_context(&result.context);
            *host.lock().expect("host lock") = next.clone();
            let rebuilt = build_select(&next, &host, &mounted);
            *mounted.lock().expect("mount lock") = rebuilt;
        })
    };
    let handlers = SelectHandlers::new("a1").on_transition(handler);
    let theme_provider = theme();
    let mut node = poodle_render::select(spec, &RenderContext::new(&theme_provider), &handlers);
    node.id = Some("poodle-select-a1".to_owned());
    node
}

/// Executed on 2026-09-05 and diverged on real semantics (trigger role
/// `combobox` vs Svelte `button`, unnamed GPUI listbox, focusable Svelte
/// option buttons, Svelte indicator button). The Svelte reference wins; the
/// repair belongs to the NP-2 tranche (`g16.113`), not this foundation card.
/// Log: `docs/logs/2026-09/20260905-g16-111-nucleus-a1-accessibility-receipt-foundation.md`.
/// g16.117 aligned Select, so this row runs with the rest of the cohort.
#[test]
fn select_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("select");
    let select_props: SelectProps = props(&loaded);
    let options = select_props
        .options
        .iter()
        .map(|option| {
            let mut choice = ChoiceOption::new(option.value.clone(), option.label.clone());
            choice.is_disabled = option.disabled.unwrap_or(false);
            choice
        })
        .collect();
    let mut spec = SelectSpec::new(options);
    if let Some(value) = select_props
        .value
        .clone()
        .or_else(|| select_props.default_value.clone())
    {
        spec = spec.with_value(value);
    }
    if let Some(label) = &select_props.aria_label {
        spec = spec.with_aria_label(label.clone());
    }
    if let Some(placeholder) = &select_props.placeholder {
        spec.placeholder = Some(placeholder.clone());
    }
    spec = spec.with_mode(match select_props.native {
        Some(false) => SelectMode::Custom,
        Some(true) => SelectMode::Native,
        None => SelectMode::Auto,
    });
    spec.is_disabled = select_props.disabled.unwrap_or(false);
    run_headless(|cx| {
        let host = Arc::new(Mutex::new(spec.clone()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        let initial = build_select(&spec, &host, &mounted);
        *mounted.lock().expect("mount lock") = initial;
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 360.0, 280.0);
        driver.wait_for_focus_handle(&poodle_render::select_trigger_focus_id("a1"));
        replay(&mut driver, &loaded.scenario.actions);
        prove(
            &mut driver,
            &loaded,
            &[
                "mount the production Select node tree through HeadlessDriver with the shared scenario props",
                "replay the shared scenario actions through GPUI test-platform dispatch (pointer activate the trigger to open the listbox)",
                "read the mounted accessibility projection and the backend focus registry after the host rebuild",
                "execute gpui sequential focus traversal and attribute every stop to a tracked node",
            ],
            &[
                "the normalised GPUI snapshot equals the committed Svelte DOM snapshot for the same scenario hash",
                "trigger, listbox, and option roles, names, expanded and selected states, controls indices, and value text match the Svelte ARIA projection",
                "gpui tab traversal visits the tracked focusable nodes in the declared order",
            ],
        );
    });
}

// ── g16.118 overlay structure rows ─────────────────────────────────────────

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct DialogProps {
    default_open: Option<bool>,
    title: Option<String>,
    description: Option<String>,
    show_close_button: Option<bool>,
    close_label: Option<String>,
    aria_label: Option<String>,
    dismiss_on_escape: Option<bool>,
    dismiss_on_backdrop: Option<bool>,
}

#[test]
fn dialog_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("dialog");
    let props: DialogProps = props(&loaded);
    let mut spec = DialogSpec::new().with_default_open(props.default_open.unwrap_or(false));
    if let Some(title) = &props.title {
        spec = spec.with_title(title);
    }
    if let Some(description) = &props.description {
        spec = spec.with_description(description);
    }
    if let Some(label) = &props.aria_label {
        spec = spec.with_aria_label(label);
    }
    if let Some(label) = &props.close_label {
        spec = spec.with_close_label(label);
    }
    spec = spec.with_show_close_button(props.show_close_button.unwrap_or(false));
    spec = spec
        .with_dismiss_on_escape(props.dismiss_on_escape.unwrap_or(true))
        .with_dismiss_on_backdrop(props.dismiss_on_backdrop.unwrap_or(true));
    run_headless(|cx| {
        let theme_provider = theme();
        let body = Node::text("Confirm deletion");
        let mounted = Arc::new(Mutex::new(poodle_render::dialog(
            &spec,
            &RenderContext::new(&theme_provider),
            vec![body],
            None,
            None,
        )));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 360.0);
        driver.dispatch_probe_key("f13");
        prove(
            &mut driver,
            &loaded,
            &["mount the production Dialog modal with title, description, close affordance, and body"],
            &["backdrop, dialog, heading, and close-button semantics match the Svelte projection"],
        );
    });
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct PopoverProps {
    default_open: Option<bool>,
    aria_label: Option<String>,
    initial_focus: Option<String>,
    surface_width: Option<String>,
}

#[test]
fn popover_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("popover");
    let props: PopoverProps = props(&loaded);
    let mut spec = PopoverSpec::new().with_default_open(props.default_open.unwrap_or(false));
    if let Some(label) = &props.aria_label {
        spec = spec.with_aria_label(label);
    }
    if props.initial_focus.as_deref() == Some("content") {
        spec = spec.with_initial_focus(PopoverInitialFocus::Content);
    }
    run_headless(|cx| {
        let theme_provider = theme();
        let ctx = RenderContext::new(&theme_provider);
        let handlers = PopoverHandlers {
            instance_id: Some("a1".to_owned()),
            ..PopoverHandlers::default()
        };
        let trigger = poodle_render::button(
            &ButtonSpec::new().with_label("Settings"),
            &ctx,
            None,
        );
        let content = Node::text("Quick settings panel");
        let mounted = Arc::new(Mutex::new(poodle_render::popover(
            &spec,
            &ctx,
            &handlers,
            Some(trigger),
            Some(content),
        )));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 360.0, 220.0);
        driver.dispatch_probe_key("f13");
        prove(
            &mut driver,
            &loaded,
            &["mount the production Popover trigger and open surface through HeadlessDriver"],
            &["trigger disclosure, dialog surface name, and controls relationship match the Svelte projection"],
        );
    });
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ConfirmActionA1Props {
    title: String,
    description: String,
    trigger_label: String,
    confirm_label: String,
    cancel_label: String,
    tone: String,
}

fn build_confirm_action_a1(
    props: &ConfirmActionA1Props,
    open: bool,
    mounted: &Arc<Mutex<Node>>,
) -> Node {
    let mut spec = poodle_specs::ConfirmActionSpec::new(
        &props.title,
        &props.description,
        &props.confirm_label,
        &props.cancel_label,
    )
    .with_trigger_label(&props.trigger_label)
    .with_open(open);
    spec.tone = match props.tone.as_str() {
        "danger" => StatusTone::Danger,
        "warning" => StatusTone::Warning,
        other => panic!("unmapped ConfirmAction tone `{other}`"),
    };
    let trigger_props = props.clone();
    let trigger_mounted = Arc::clone(mounted);
    let trigger = poodle_render::button(
        &ButtonSpec::new().with_label(&props.trigger_label),
        &RenderContext::new(&theme()),
        Some(Arc::new(move || {
            let rebuilt = build_confirm_action_a1(&trigger_props, true, &trigger_mounted);
            *trigger_mounted.lock().expect("ConfirmAction mount lock") = rebuilt;
        })),
    );
    let mut node = poodle_render::confirm_action::confirm_action_with_slots_state(
        &spec,
        &RenderContext::new(&theme()),
        Some(trigger),
        None,
        false,
        "Working…",
        ConfirmActionHandlers::default(),
    );
    node.id = Some("confirm-action-a1".to_owned());
    node
}

#[test]
fn confirm_action_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("confirm-action");
    let input: ConfirmActionA1Props = props(&loaded);
    run_headless(|cx| {
        let mounted = Arc::new(Mutex::new(Node::container()));
        let initial = build_confirm_action_a1(&input, false, &mounted);
        *mounted.lock().expect("ConfirmAction mount lock") = initial;
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 260.0);
        replay(&mut driver, &loaded.scenario.actions);
        prove(
            &mut driver,
            &loaded,
            &[
                "mount the production ConfirmAction node tree through HeadlessDriver with the shared scenario props",
                "replay the shared scenario actions through GPUI test-platform dispatch",
            ],
            &["trigger, backdrop, alert-dialog, close, cancel, and confirm semantics match the Svelte projection"],
        );
    });
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct DetailItemA1Props {
    label: String,
    value: String,
    description: String,
}

#[test]
fn detail_item_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("detail-item");
    let input: DetailItemA1Props = props(&loaded);
    run_headless(|cx| {
        let spec = poodle_specs::DetailItemSpec::new(&input.label)
            .with_value(&input.value)
            .with_description(&input.description);
        let host = Arc::new(Mutex::new(false));
        let mounted = Arc::new(Mutex::new(Node::container()));
        let toggle_host = Arc::clone(&host);
        let toggle_mounted = Arc::clone(&mounted);
        let toggle_spec = spec.clone();
        *mounted.lock().expect("detail mount") = poodle_render::detail_item_with_slots_state(
            &spec,
            &RenderContext::new(&theme()),
            None,
            None,
            false,
            Some(Arc::new(move || {
                *toggle_host.lock().expect("detail info") = true;
                *toggle_mounted.lock().expect("detail mount") =
                    poodle_render::detail_item_with_slots_state(
                        &toggle_spec,
                        &RenderContext::new(&theme()),
                        None,
                        None,
                        true,
                        None,
                    );
            })),
        );
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 160.0);
        replay(&mut driver, &loaded.scenario.actions);
        prove(
            &mut driver,
            &loaded,
            &[
                "mount the production DetailItem node tree through HeadlessDriver with the shared scenario props",
                "replay the shared scenario actions through GPUI test-platform dispatch",
            ],
            &["information action and dialog semantics match the Svelte projection"],
        );
    });
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct CommandPaletteProps {
    open: bool,
    title: Option<String>,
    description: Option<String>,
    invocation_hint: Option<String>,
    items: Vec<CommandActionProps>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct CommandActionProps {
    id: String,
    title: String,
    group: Option<String>,
    shortcut: Option<String>,
}

#[test]
fn command_palette_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("command-palette");
    let input: CommandPaletteProps = props(&loaded);
    let actions = input
        .items
        .into_iter()
        .map(|item| {
            let mut action = CommandActionItem::new(item.id, item.title);
            if let Some(group) = item.group {
                action = action.with_group(group);
            }
            if let Some(shortcut) = item.shortcut {
                action = action.with_shortcut(shortcut);
            }
            action
        })
        .collect();
    let mut spec = CommandPaletteSpec::new(actions).with_open(input.open);
    if let Some(title) = input.title {
        spec = spec.with_title(title);
    }
    if let Some(description) = input.description {
        spec = spec.with_description(description);
    }
    if let Some(hint) = input.invocation_hint {
        spec = spec.with_invocation_hint(hint);
    }
    run_headless(|cx| {
        let mounted = Arc::new(Mutex::new(poodle_render::command_palette_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            CommandPaletteHandlers {
                instance_id: Some("a1".into()),
                ..Default::default()
            },
        )));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 720.0, 520.0);
        driver.draw_frame();
        driver.draw_frame();
        driver.dispatch_probe_key("escape");
        prove(
            &mut driver,
            &loaded,
            &["mount the production CommandPalette node tree through HeadlessDriver with the shared scenario props"],
            &["dialog, heading, search, status, results, and action roles match the Svelte ARIA projection"],
        );
    });
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct MessageCenterProps {
    default_open: bool,
    title: String,
    items: Vec<MessageItemProps>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct MessageItemProps {
    id: String,
    title: String,
    message: Option<String>,
    read: bool,
    tone: Option<String>,
    meta: Option<String>,
}

#[test]
fn message_center_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("message-center");
    let input: MessageCenterProps = props(&loaded);
    let items = input
        .items
        .into_iter()
        .map(|item| {
            let mut out = MessageCenterItem::new(item.id, item.title).with_read(item.read);
            if let Some(message) = item.message {
                out = out.with_message(message);
            }
            if let Some(meta) = item.meta {
                out = out.with_meta(meta);
            }
            if matches!(item.tone.as_deref(), Some("success")) {
                out = out.with_tone(StatusTone::Success);
            }
            out
        })
        .collect();
    let spec = MessageCenterSpec::new(items)
        .with_default_open(input.default_open)
        .with_open(input.default_open)
        .with_title(input.title);
    run_headless(|cx| {
        let mounted = Arc::new(Mutex::new(poodle_render::message_center(
            &spec,
            &RenderContext::new(&theme()),
            MessageCenterHandlers {
                on_item_select: Some(Arc::new(|_| {})),
                instance_id: Some("a1".into()),
                ..Default::default()
            },
        )));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 720.0, 520.0);
        driver.draw_frame();
        driver.draw_frame();
        driver.dispatch_probe_key("escape");
        prove(
            &mut driver,
            &loaded,
            &["mount the production MessageCenter node tree through HeadlessDriver with the shared scenario props"],
            &["trigger, dialog, banner, heading, list, and item buttons match the Svelte ARIA projection"],
        );
    });
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ToastHostProps {
    placement: String,
    aria_label: String,
    auto_dismiss_ms: u32,
    toasts: Vec<ToastProps>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ToastProps {
    id: String,
    title: String,
    message: Option<String>,
    tone: Option<String>,
    action_label: Option<String>,
}

#[test]
fn toast_host_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("toast-host");
    let input: ToastHostProps = props(&loaded);
    let toasts = input
        .toasts
        .into_iter()
        .map(|item| {
            let mut out = Toast::new(item.id, item.title);
            if let Some(message) = item.message {
                out = out.with_message(message);
            }
            if matches!(item.tone.as_deref(), Some("success")) {
                out = out.with_tone(poodle_specs::ToastTone::Success);
            }
            if matches!(item.tone.as_deref(), Some("danger")) {
                out = out.with_tone(poodle_specs::ToastTone::Danger);
            }
            if let Some(action) = item.action_label {
                out = out.with_action_label(action);
            }
            out
        })
        .collect();
    let placement = match input.placement.as_str() {
        "top-start" => ToastHostPlacement::TopStart,
        "top-end" => ToastHostPlacement::TopEnd,
        "bottom-start" => ToastHostPlacement::BottomStart,
        _ => ToastHostPlacement::BottomEnd,
    };
    let host = ToastHostSpec::new()
        .with_auto_dismiss_ms(input.auto_dismiss_ms)
        .with_placement(placement)
        .with_aria_label(input.aria_label);
    let stack = ToastStackSpec::new().with_toasts(toasts);
    run_headless(|cx| {
        let mounted = Arc::new(Mutex::new(poodle_render::toast_host(
            &host,
            &RenderContext::new(&theme()),
            &stack,
            ToastStackHandlers {
                instance_id: Some("a1".into()),
                ..Default::default()
            },
        )));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 720.0, 520.0);
        driver.draw_frame();
        driver.draw_frame();
        driver.dispatch_probe_key("escape");
        prove(
            &mut driver,
            &loaded,
            &["mount the production ToastHost node tree through HeadlessDriver with the shared scenario props"],
            &["stack label, listitem toasts, dismiss and retry names match the Svelte ARIA projection"],
        );
    });
}
