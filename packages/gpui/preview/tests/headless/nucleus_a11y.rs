//! g16.111 — A1 paired accessibility proofs for the three foundation rows.
//!
//! Each proof mounts the row's production node tree through `HeadlessDriver`,
//! replays the shared scenario's actions through GPUI's real dispatch tree,
//! reads the mounted accessibility projection, and compares it with the
//! committed Svelte DOM snapshot produced from the same scenario file. The
//! A1 receipt is emitted only after the diff is empty.

use std::sync::{Arc, Mutex};

use poodle_node::Node;
use poodle_render::{RenderContext, SelectHandlers, TabsHandlers};
use poodle_specs::{
    ChoiceOption, Orientation, SelectMode, SelectSpec, SwitchSpec, TabActivationMode,
    TabDefinition, TabsSpec,
};
use serde::Deserialize;
use serde_json::Value;

use super::headless_driver::HeadlessDriver;
use super::nucleus_receipts::{self, A1Action, A1Target, LoadedA1Scenario};
use super::{run_headless, theme};

fn gpui_key(key: &str) -> &str {
    match key {
        "right" | "left" | "up" | "down" | "enter" | "space" | "escape" | "home" | "end" | "tab" => key,
        other => panic!("A1 scenario uses an unmapped key `{other}`"),
    }
}

/// The first mounted node, in document order, whose role and record name
/// match the shared target. Resolution reads the mounted tree, never a
/// runtime id from the scenario.
fn resolve_target(driver: &mut HeadlessDriver, target: &A1Target) -> String {
    let nodes = driver.accessibility_nodes();
    let node = nodes
        .iter()
        .find(|node| {
            target
                .role
                .as_deref()
                .map_or(true, |role| nucleus_receipts::aria_role(node.role) == role)
                && target
                    .name
                    .as_deref()
                    .map_or(true, |name| node.label.as_deref().map(str::trim) == Some(name))
        })
        .unwrap_or_else(|| panic!("no mounted node matches A1 target {target:?}"));
    assert!(
        !node.element_id.is_empty(),
        "A1 target {target:?} resolved to a node without a backend identity"
    );
    node.element_id.clone()
}

fn replay(driver: &mut HeadlessDriver, actions: &[A1Action]) {
    for action in actions {
        match action {
            A1Action::PointerActivate { target } => {
                let id = resolve_target(driver, target);
                assert!(
                    poodle_gpui_node_backend::bounds_for(&id).is_some(),
                    "A1 target `{id}` has no painted bounds"
                );
                driver.pointer_activate_id(&id);
            }
            A1Action::Key { target, key } => {
                let id = resolve_target(driver, target);
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
        panic!(
            "A1 divergence for {} ({} entries):\n{}",
            loaded.scenario.component,
            diff.len(),
            serde_json::to_string_pretty(&diff).expect("diff serialises")
        );
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
    serde_json::from_value(loaded.scenario.props.clone())
        .unwrap_or_else(|error| panic!("{} props do not map to the Rust spec: {error}", loaded.path))
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
    let mut node = poodle_render::switch(&spec, &RenderContext::new(&theme_provider), Some(handler));
    node.id = Some("poodle-switch-a1".to_owned());
    node
}

#[test]
fn switch_a1_accessibility_projection_matches_svelte() {
    let loaded = nucleus_receipts::load_a1_scenario("switch");
    let switch_props: SwitchProps = props(&loaded);
    run_headless(|cx| {
        let mounted = Arc::new(Mutex::new(Node::container()));
        let initial = build_switch(switch_props.default_checked.unwrap_or(false), &switch_props, &mounted);
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

fn build_select(spec: &SelectSpec, host: &Arc<Mutex<SelectSpec>>, mounted: &Arc<Mutex<Node>>) -> Node {
    let handler = {
        let host = Arc::clone(host);
        let mounted = Arc::clone(mounted);
        Arc::new(move |result: poodle_render::SelectTransitionResult| {
            let next = host.lock().expect("host lock").clone().applying_context(&result.context);
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
/// Run with `--ignored` to reproduce the diff.
#[test]
#[ignore = "g16.111: Select A1 diverges from the Svelte reference; repair candidate for g16.113 (see the g16.111 execution log)"]
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
    if let Some(value) = select_props.value.clone().or_else(|| select_props.default_value.clone()) {
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
