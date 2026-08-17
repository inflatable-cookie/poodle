//! ModelConnectionSetup — an adaptive shell for choosing one exact model
//! connection and, when required, completing host-owned setup.
//!
//! Contract: `docs/contracts/components/model-connection-setup.md`
//!
//! Every workflow decision runs through
//! `poodle_headless::model_connection::model_connection_setup_transition`, so
//! the guards, the direct-add path, and the emitted effects are the approved
//! ones. This file composes the picker, the selected summary, the host's
//! configuration content, and the workflow actions, and turns the machine's
//! effects into callbacks.
//!
//! Poodle never manufactures a credential form, a stepper, or a provider
//! schema. The configuration body is host content; its values never reach a
//! spec, a callback, or this tree.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_headless::model_connection::{
    model_connection_setup_can_continue, model_connection_setup_can_submit,
    model_connection_setup_selected_option, model_connection_setup_transition,
    ModelConnectionOption, ModelConnectionSetupEffect, ModelConnectionSetupEvent,
    ModelConnectionSetupStage,
};
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeRole,
    StylePatch, TextChangeHandler,
};
use poodle_specs::{
    ButtonSpec, ButtonVariant, CallOutSpec, CalloutAnnounceMode, ModelConnectionPickerSpec,
    ModelConnectionSetupSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant, StatusTone,
};

use crate::button::button;
use crate::callout::callout;
use crate::model_connection_picker::{
    model_connection_option_focus_id, model_connection_picker_with_slots,
    ModelConnectionPickerHandlers, ModelConnectionPickerSlots,
};
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::spinner::spinner;

/// Contract §8: the label weight the workflow heading shares with the family.
const LABEL_WEIGHT: u16 = 500;

/// The semantic id of the heading a configured-flow stage change moves focus
/// to. Readable and stable across instances.
pub const MODEL_CONNECTION_SETUP_TITLE_ID: &str = "model-connection-setup:title";

/// The backend-state id of that heading: the instance scope when the host
/// supplied one, else the semantic id. Focus requests name this, because it is
/// what the backend keys focus handles by.
pub fn model_connection_setup_title_focus_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("model-connection-setup:{scope}:title"),
        None => MODEL_CONNECTION_SETUP_TITLE_ID.to_string(),
    }
}

/// The backend-state id of one workflow action, so a host or a mounted test
/// can name it.
pub fn model_connection_setup_action_id(instance_id: Option<&str>, action: &str) -> String {
    match instance_id {
        Some(scope) => format!("model-connection-setup:{scope}:{action}"),
        None => format!("model-connection-setup:{action}"),
    }
}

/// Host callbacks. Every one is a request; the host updates the spec.
#[derive(Default)]
pub struct ModelConnectionSetupHandlers {
    /// Continue or Back was accepted. A direct route never emits a stage.
    pub on_stage_change: Option<Arc<dyn Fn(ModelConnectionSetupStage) + Send + Sync>>,
    /// The picker selection changed; the payload is the exact option id.
    pub on_value_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_query_change: Option<TextChangeHandler>,
    /// Add was activated. The host validates and persists.
    pub on_submit: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Cancel was activated. The host owns overlay closure.
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
    /// A stage change wants focus moved to this element id. The backend owns
    /// the actual focus operation; the component names the destination.
    pub on_focus_request: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Stable native instance scope, forwarded to the composed picker. Two
    /// setups over the same routes must never share backend focus handles.
    pub instance_id: Option<String>,
}

/// Host-composed content. `configuration` is the only place provider fields,
/// OAuth, or detection may live, and Poodle never reads it.
#[derive(Default)]
pub struct ModelConnectionSetupSlots {
    /// Leading marks forwarded to the picker and the selected summary, keyed
    /// by option id.
    pub picker: ModelConnectionPickerSlots,
    /// The host's configuration body, rendered only in the configure stage.
    pub configuration: Option<Node>,
    /// Optional secondary guidance beside the configuration body.
    pub configure_aside: Option<Node>,
}

pub fn model_connection_setup(
    spec: &ModelConnectionSetupSpec,
    theme: &dyn ThemeProvider,
    handlers: ModelConnectionSetupHandlers,
) -> Node {
    model_connection_setup_with_slots(
        spec,
        theme,
        ModelConnectionSetupSlots::default(),
        handlers,
    )
}

pub fn model_connection_setup_with_slots(
    spec: &ModelConnectionSetupSpec,
    theme: &dyn ThemeProvider,
    slots: ModelConnectionSetupSlots,
    handlers: ModelConnectionSetupHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let context = spec.behaviour_context();
    let selected = model_connection_setup_selected_option(&context).cloned();
    let requires_configuration = selected
        .as_ref()
        .map(|option| option.requires_configuration)
        .unwrap_or(true);
    let can_continue = model_connection_setup_can_continue(&context);
    let can_add = model_connection_setup_can_submit(&context);

    let handlers = Arc::new(handlers);
    let instance = handlers.instance_id.clone();
    let run = {
        let spec = spec.clone();
        let handlers = Arc::clone(&handlers);
        let instance = instance.clone();
        move |event: ModelConnectionSetupEvent| {
            let result = model_connection_setup_transition(spec.behaviour_context(), event);
            for effect in result.effects {
                match effect {
                    ModelConnectionSetupEffect::EmitStageChange { stage } => {
                        if let Some(handler) = &handlers.on_stage_change {
                            handler(stage);
                        }
                        // The web moves focus to the new visible heading on
                        // the way into configure, and back to the selected
                        // option on the way out. The component names the
                        // destination; the backend performs the move.
                        if let Some(handler) = &handlers.on_focus_request {
                            match stage {
                                ModelConnectionSetupStage::Configure => handler(
                                    &model_connection_setup_title_focus_id(instance.as_deref()),
                                ),
                                ModelConnectionSetupStage::Choose => {
                                    if let Some(value) = spec.value.as_deref() {
                                        handler(&model_connection_option_focus_id(
                                            instance.as_deref(),
                                            value,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    ModelConnectionSetupEffect::EmitValueChange { id } => {
                        if let Some(handler) = &handlers.on_value_change {
                            handler(&id);
                        }
                    }
                    ModelConnectionSetupEffect::EmitQueryChange { query } => {
                        if let Some(handler) = &handlers.on_query_change {
                            handler(&query);
                        }
                    }
                    ModelConnectionSetupEffect::EmitSubmit { id } => {
                        if let Some(handler) = &handlers.on_submit {
                            handler(&id);
                        }
                    }
                    ModelConnectionSetupEffect::EmitCancel => {
                        if let Some(handler) = &handlers.on_cancel {
                            handler();
                        }
                    }
                }
            }
        }
    };
    let run = Arc::new(run);

    // ── Header ──
    let mut header = Node::container();
    {
        let s = &mut header.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.fill_width = true;
        s.min_width = Some(0.0);
    }
    let mut title = Node::text(&spec.title);
    title.id = Some(MODEL_CONNECTION_SETUP_TITLE_ID.to_string());
    title.runtime_id = instance
        .as_deref()
        .map(|scope| model_connection_setup_title_focus_id(Some(scope)));
    title.style.text_size = Some(theme.resolve_space("typography.body.size"));
    title.style.text_weight = Some(LABEL_WEIGHT);
    title.style.descriptor.text_color = Some(theme.resolve_color("color.text.primary"));
    // Programmatically focusable, sequentially skipped — the web heading's
    // `tabindex="-1"`, which exists so a stage change has somewhere to land.
    // The focus patch is not decoration: the GPUI backend creates a tracked
    // focus handle only for a focusable node that draws differently when
    // focused, so without it the stage's focus request would be dropped and
    // the heading could never receive focus.
    title.interaction.focusable = true;
    title.a11y.tab_index = Some(-1);
    title.style.focus = Some(StylePatch {
        border_color: Some(theme.resolve_color("color.accent.focusRing")),
        ..StylePatch::default()
    });
    let mut header = header.child(title);
    if let Some(description) = spec.description.as_deref() {
        header = header.child(secondary_text(theme, description));
    }

    // ── Body ──
    let mut body = Node::container();
    {
        let s = &mut body.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.stack.md");
        s.descriptor.layout.height = LayoutSizing::Grow;
        s.fill_width = true;
        s.min_width = Some(0.0);
    }
    let mut body = body;

    if spec.stage == ModelConnectionSetupStage::Choose {
        let picker_spec = ModelConnectionPickerSpec::new()
            .with_options(spec.options.clone())
            .with_value(spec.value.clone())
            .with_query(spec.query.clone())
            .with_state(spec.picker_state)
            .with_disabled(spec.is_pending)
            .with_size(spec.size)
            .with_size_role(spec.size_role)
            .with_density(spec.density);
        let picker = model_connection_picker_with_slots(
            &picker_spec,
            theme,
            slots.picker,
            ModelConnectionPickerHandlers {
                on_value_change: Some({
                    let run = Arc::clone(&run);
                    Arc::new(move |id: &str| {
                        run(ModelConnectionSetupEvent::Select { id: id.to_string() })
                    })
                }),
                on_query_change: Some({
                    let run = Arc::clone(&run);
                    Arc::new(move |query: &str| {
                        run(ModelConnectionSetupEvent::SetQuery {
                            query: query.to_string(),
                        })
                    })
                }),
                instance_id: instance.clone(),
            },
        );
        body = body.child(picker);
    } else if let Some(option) = selected.as_ref() {
        body = body.child(selected_summary(theme, option, &slots.picker));

        if let Some(error) = spec.error.as_deref() {
            body = body.child(callout(
                &CallOutSpec::new()
                    .with_tone(StatusTone::Danger)
                    .with_content(error)
                    .with_announce_mode(CalloutAnnounceMode::Assertive)
                    .with_size(effective_size)
                    .with_density(spec.density),
                theme,
                None,
            ));
        }
        if let Some(success) = spec.success.as_deref() {
            body = body.child(callout(
                &CallOutSpec::new()
                    .with_tone(StatusTone::Success)
                    .with_content(success)
                    .with_announce_mode(CalloutAnnounceMode::Polite)
                    .with_size(effective_size)
                    .with_density(spec.density),
                theme,
                None,
            ));
        }

        if slots.configuration.is_some() || slots.configure_aside.is_some() {
            let mut configuration = Node::container();
            {
                let s = &mut configuration.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = theme.resolve_space("space.stack.sm");
                let pad = &mut s.descriptor.layout.spacing.padding;
                let inset = theme.resolve_space("space.stack.md");
                pad.top = inset;
                pad.bottom = inset;
                pad.left = inset;
                pad.right = inset;
                s.descriptor.border.width = rem_to_px(0.0625);
                s.descriptor.border.color = theme.resolve_color("color.border.subtle");
                s.descriptor.background = Some(theme.resolve_color("color.background.surface"));
                s.fill_width = true;
                s.min_width = Some(0.0);
                surface_radius(&mut configuration.style, theme);
            }
            let mut configuration = configuration;
            if let Some(content) = slots.configuration {
                configuration = configuration.child(content);
            }
            if let Some(aside) = slots.configure_aside {
                let mut wrapper = Node::container();
                {
                    let s = &mut wrapper.style;
                    s.descriptor.layout.direction = LayoutDirection::Column;
                    s.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
                    s.text_size = Some(theme.resolve_space("typography.label.size"));
                    s.fill_width = true;
                }
                configuration = configuration.child(wrapper.child(aside));
            }
            body = body.child(configuration);
        }

        if spec.is_pending {
            let mut pending = Node::container();
            {
                let s = &mut pending.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
            }
            pending.a11y.role = Some(NodeRole::Status);
            pending.a11y.label = Some(spec.pending_label.clone());
            let pending = pending
                .child(spinner(
                    &SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Grid)
                        .with_size(SpinnerSize::Sm)
                        .with_tone(SpinnerTone::Accent),
                    theme,
                ))
                .child(secondary_text(theme, &spec.pending_label));
            body = body.child(pending);
        }
    }

    // ── Workflow actions ──
    let mut actions = Node::container();
    {
        let s = &mut actions.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::End;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        s.descriptor.layout.spacing.padding.top = theme.resolve_space("space.stack.sm");
        s.border_top_width = Some(rem_to_px(0.0625));
        s.border_color_top = Some(theme.resolve_color("color.border.subtle"));
        s.flex_wrap = true;
        s.fill_width = true;
    }
    let cancel = action_button(
        theme,
        spec,
        instance.as_deref(),
        "cancel",
        effective_size,
        &spec.cancel_label,
        ButtonVariant::Ghost,
        spec.is_pending,
        {
            let run = Arc::clone(&run);
            Arc::new(move || run(ModelConnectionSetupEvent::Cancel))
        },
    );
    let actions = if spec.stage == ModelConnectionSetupStage::Choose {
        // A direct route shows Add and submits from choose; nothing else does.
        let (label, disabled, event) = if requires_configuration {
            (
                &spec.continue_label,
                !can_continue,
                ModelConnectionSetupEvent::Continue,
            )
        } else {
            (
                &spec.submit_label,
                !can_add,
                ModelConnectionSetupEvent::Submit,
            )
        };
        let primary = action_button(
            theme,
            spec,
            instance.as_deref(),
            if requires_configuration { "continue" } else { "submit" },
            effective_size,
            label,
            ButtonVariant::Primary,
            disabled,
            {
                let run = Arc::clone(&run);
                Arc::new(move || run(event.clone()))
            },
        );
        actions.child(cancel).child(primary)
    } else {
        let back = action_button(
            theme,
            spec,
            instance.as_deref(),
            "back",
            effective_size,
            &spec.back_label,
            ButtonVariant::Ghost,
            spec.is_pending,
            {
                let run = Arc::clone(&run);
                Arc::new(move || run(ModelConnectionSetupEvent::Back))
            },
        );
        let submit = action_button(
            theme,
            spec,
            instance.as_deref(),
            "submit",
            effective_size,
            &spec.submit_label,
            ButtonVariant::Primary,
            !can_add,
            {
                let run = Arc::clone(&run);
                Arc::new(move || run(ModelConnectionSetupEvent::Submit))
            },
        );
        actions.child(back).child(cancel).child(submit)
    };

    // ── Root ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.stack.md");
        s.fill_width = true;
        s.min_width = Some(0.0);
    }
    // A labelled region, never a dialog: the host owns the overlay.
    root.a11y.role = Some(NodeRole::Region);
    root.a11y.label = Some(spec.effective_aria_label().to_string());
    root.roles.insert(
        "stage".to_string(),
        match spec.stage {
            ModelConnectionSetupStage::Choose => "choose".to_string(),
            ModelConnectionSetupStage::Configure => "configure".to_string(),
        },
    );
    root.roles
        .insert("pending".to_string(), spec.is_pending.to_string());
    root.child(header).child(body).child(actions)
}

fn surface_radius(style: &mut poodle_node::NodeStyle, theme: &dyn ThemeProvider) {
    let radius = theme.resolve_radius("radius.surface");
    let c = &mut style.descriptor.corner_radii;
    c.top_left = radius;
    c.top_right = radius;
    c.bottom_right = radius;
    c.bottom_left = radius;
}

fn secondary_text(theme: &dyn ThemeProvider, content: &str) -> Node {
    let mut node = Node::text(content);
    node.style.text_size = Some(theme.resolve_space("typography.label.size"));
    node.style.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
    node
}

#[allow(clippy::too_many_arguments)]
fn action_button(
    theme: &dyn ThemeProvider,
    spec: &ModelConnectionSetupSpec,
    instance_id: Option<&str>,
    action: &str,
    effective_size: poodle_specs::ControlSize,
    label: &str,
    variant: ButtonVariant,
    disabled: bool,
    on_click: Arc<dyn Fn() + Send + Sync>,
) -> Node {
    let mut node = button(
        &ButtonSpec::new()
            .with_label(label)
            .with_variant(variant)
            .with_disabled(disabled)
            .with_size(effective_size)
            .with_density(spec.density),
        theme,
        (!disabled).then_some(on_click),
    );
    node.id = Some(model_connection_setup_action_id(None, action));
    node.runtime_id = instance_id.map(|scope| model_connection_setup_action_id(Some(scope), action));
    node
}

/// The configure stage's selected-route header: supplied labels, repeated.
fn selected_summary(
    theme: &dyn ThemeProvider,
    option: &ModelConnectionOption,
    picker_slots: &ModelConnectionPickerSlots,
) -> Node {
    let mut mark = picker_slots
        .leading
        .get(&option.id)
        .cloned()
        .unwrap_or_else(|| Node::icon("package", rem_to_px(1.0)));
    if mark.style.descriptor.text_color.is_none() {
        mark.style.descriptor.text_color = Some(theme.resolve_color("color.text.secondary"));
    }

    let mut copy = Node::container();
    {
        let s = &mut copy.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }
    let mut provider = Node::text(&option.provider_label);
    provider.style.text_size = Some(theme.resolve_space("typography.body.size"));
    provider.style.text_weight = Some(LABEL_WEIGHT);
    provider.style.descriptor.text_color = Some(theme.resolve_color("color.text.primary"));
    let mut copy = copy.child(provider);
    if let Some(route) = option.route_label.as_deref() {
        copy = copy.child(secondary_text(theme, route));
    }

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = theme.resolve_space("space.stack.sm");
        pad.bottom = theme.resolve_space("space.stack.sm");
        pad.left = theme.resolve_space("space.inline.md");
        pad.right = theme.resolve_space("space.inline.md");
        s.descriptor.border.width = rem_to_px(0.0625);
        s.descriptor.border.color = theme.resolve_color("color.border.subtle");
        s.descriptor.background = Some(theme.resolve_color("color.background.surface"));
        s.fill_width = true;
        s.min_width = Some(0.0);
        surface_radius(&mut row.style, theme);
    }
    row.child(mark).child(copy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model_connection_picker::model_connection_option_id;
    use poodle_headless::model_connection::{
        model_connection_picker_fixtures, ModelConnectionAvailability,
    };
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> ModelConnectionSetupSpec {
        ModelConnectionSetupSpec::new().with_options(model_connection_picker_fixtures())
    }

    /// `codex-app` made available, so one route submits without configuring.
    fn direct_spec() -> ModelConnectionSetupSpec {
        let options = model_connection_picker_fixtures()
            .into_iter()
            .map(|option| {
                if option.id == "codex-app" {
                    option
                        .with_availability(ModelConnectionAvailability::Available, "Available")
                        .with_disabled(false)
                } else {
                    option
                }
            })
            .collect();
        ModelConnectionSetupSpec::new().with_options(options)
    }

    fn button_labelled<'a>(node: &'a Node, label: &str) -> Option<&'a Node> {
        node.find(&|n| matches!(&n.kind, poodle_node::NodeKind::Button { label: l } if l == label))
    }

    fn press(node: &Node, label: &str) {
        let button = button_labelled(node, label).unwrap_or_else(|| panic!("{label} button"));
        (button
            .interaction
            .on_activate
            .as_ref()
            .unwrap_or_else(|| panic!("{label} is enabled")))();
    }

    #[derive(Default)]
    struct Recorder {
        stages: Arc<Mutex<Vec<ModelConnectionSetupStage>>>,
        submits: Arc<Mutex<Vec<String>>>,
        cancels: Arc<Mutex<usize>>,
        focus: Arc<Mutex<Vec<String>>>,
        values: Arc<Mutex<Vec<String>>>,
        queries: Arc<Mutex<Vec<String>>>,
    }

    impl Recorder {
        fn handlers(&self) -> ModelConnectionSetupHandlers {
            let stages = Arc::clone(&self.stages);
            let submits = Arc::clone(&self.submits);
            let cancels = Arc::clone(&self.cancels);
            let focus = Arc::clone(&self.focus);
            let values = Arc::clone(&self.values);
            let queries = Arc::clone(&self.queries);
            ModelConnectionSetupHandlers {
                on_stage_change: Some(Arc::new(move |stage| stages.lock().unwrap().push(stage))),
                on_value_change: Some(Arc::new(move |id: &str| {
                    values.lock().unwrap().push(id.to_string())
                })),
                on_query_change: Some(Arc::new(move |query: &str| {
                    queries.lock().unwrap().push(query.to_string())
                })),
                on_submit: Some(Arc::new(move |id: &str| {
                    submits.lock().unwrap().push(id.to_string())
                })),
                on_cancel: Some(Arc::new(move || *cancels.lock().unwrap() += 1)),
                on_focus_request: Some(Arc::new(move |id: &str| {
                    focus.lock().unwrap().push(id.to_string())
                })),
                instance_id: None,
            }
        }
    }

    #[test]
    fn choose_shows_continue_for_a_route_that_requires_configuration() {
        let recorder = Recorder::default();
        let node = model_connection_setup(
            &spec().with_value(Some("openai-responses".to_string())),
            &theme(),
            recorder.handlers(),
        );

        assert!(button_labelled(&node, "Add connection").is_none());
        press(&node, "Continue");

        assert_eq!(
            recorder.stages.lock().unwrap().as_slice(),
            [ModelConnectionSetupStage::Configure]
        );
        assert_eq!(
            recorder.focus.lock().unwrap().as_slice(),
            [MODEL_CONNECTION_SETUP_TITLE_ID],
            "entering configure moves focus to the new visible heading"
        );
    }

    #[test]
    fn a_direct_route_submits_from_choose_without_emitting_a_stage() {
        let recorder = Recorder::default();
        let node = model_connection_setup(
            &direct_spec()
                .with_value(Some("codex-app".to_string()))
                .with_can_submit(true),
            &theme(),
            recorder.handlers(),
        );

        assert!(button_labelled(&node, "Continue").is_none());
        press(&node, "Add connection");

        assert_eq!(recorder.submits.lock().unwrap().as_slice(), ["codex-app"]);
        assert!(
            recorder.stages.lock().unwrap().is_empty(),
            "the configure stage is skipped entirely"
        );
    }

    #[test]
    fn continue_stays_disabled_until_a_selectable_route_is_chosen() {
        let node = model_connection_setup(&spec(), &theme(), ModelConnectionSetupHandlers::default());
        let continue_button = button_labelled(&node, "Continue").expect("continue");
        assert!(continue_button.interaction.disabled);
        assert!(continue_button.interaction.on_activate.is_none());

        // `lmstudio-local` requires configuration but is unavailable, so the
        // action stays Continue and stays disabled.
        let node = model_connection_setup(
            &spec().with_value(Some("lmstudio-local".to_string())),
            &theme(),
            ModelConnectionSetupHandlers::default(),
        );
        assert!(
            button_labelled(&node, "Continue")
                .expect("continue")
                .interaction
                .disabled,
            "an unavailable route is not a selection"
        );

        // `codex-app` is a direct route, so the action is Add — and it stays
        // disabled while the route is unavailable and the host has not
        // approved a submit.
        let node = model_connection_setup(
            &spec()
                .with_value(Some("codex-app".to_string()))
                .with_can_submit(true),
            &theme(),
            ModelConnectionSetupHandlers::default(),
        );
        assert!(button_labelled(&node, "Continue").is_none());
        assert!(
            button_labelled(&node, "Add connection")
                .expect("add")
                .interaction
                .disabled,
            "a direct route that is not selectable cannot submit"
        );
    }

    #[test]
    fn configure_renders_host_content_and_submits_only_when_the_host_allows_it() {
        let recorder = Recorder::default();
        let denied = model_connection_setup_with_slots(
            &spec()
                .with_stage(ModelConnectionSetupStage::Configure)
                .with_value(Some("openai-responses".to_string())),
            &theme(),
            ModelConnectionSetupSlots {
                configuration: Some(Node::text("HOST FIELDS")),
                configure_aside: Some(Node::text("HOST ASIDE")),
                ..ModelConnectionSetupSlots::default()
            },
            recorder.handlers(),
        );

        assert!(denied.texts().contains(&"HOST FIELDS"));
        assert!(denied.texts().contains(&"HOST ASIDE"));
        // The selected route's own labels are repeated, and nothing else.
        assert!(denied.texts().contains(&"OpenAI"));
        assert!(denied.texts().contains(&"Responses API"));
        assert!(
            button_labelled(&denied, "Add connection")
                .expect("add")
                .interaction
                .disabled,
            "canSubmit=false disables Add"
        );

        let allowed = model_connection_setup(
            &spec()
                .with_stage(ModelConnectionSetupStage::Configure)
                .with_value(Some("openai-responses".to_string()))
                .with_can_submit(true),
            &theme(),
            recorder.handlers(),
        );
        press(&allowed, "Add connection");
        assert_eq!(
            recorder.submits.lock().unwrap().as_slice(),
            ["openai-responses"]
        );
    }

    #[test]
    fn back_returns_to_choose_and_restores_focus_to_the_selected_option() {
        let recorder = Recorder::default();
        let node = model_connection_setup(
            &spec()
                .with_stage(ModelConnectionSetupStage::Configure)
                .with_value(Some("openai-responses".to_string())),
            &theme(),
            recorder.handlers(),
        );
        press(&node, "Back");

        assert_eq!(
            recorder.stages.lock().unwrap().as_slice(),
            [ModelConnectionSetupStage::Choose]
        );
        assert_eq!(
            recorder.focus.lock().unwrap().as_slice(),
            [model_connection_option_id("openai-responses")]
        );
    }

    #[test]
    fn pending_locks_every_workflow_action_and_announces_itself() {
        let node = model_connection_setup(
            &spec()
                .with_stage(ModelConnectionSetupStage::Configure)
                .with_value(Some("openai-responses".to_string()))
                .with_can_submit(true)
                .with_pending(true),
            &theme(),
            ModelConnectionSetupHandlers {
                on_submit: Some(Arc::new(|_| unreachable!("pending submits"))),
                on_cancel: Some(Arc::new(|| unreachable!("pending cancels"))),
                on_stage_change: Some(Arc::new(|_| unreachable!("pending navigates"))),
                ..ModelConnectionSetupHandlers::default()
            },
        );

        for label in ["Back", "Cancel", "Add connection"] {
            let button = button_labelled(&node, label).unwrap_or_else(|| panic!("{label}"));
            assert!(button.interaction.disabled, "{label} is locked while pending");
            assert!(button.interaction.on_activate.is_none());
        }

        let status = node
            .find(&|n| n.a11y.role == Some(NodeRole::Status))
            .expect("the pending live region");
        assert_eq!(status.a11y.label.as_deref(), Some("Checking connection"));
    }

    #[test]
    fn safe_feedback_renders_without_resetting_the_stage() {
        let node = model_connection_setup(
            &spec()
                .with_stage(ModelConnectionSetupStage::Configure)
                .with_value(Some("openai-responses".to_string()))
                .with_error("That route refused the request.")
                .with_success("Connection added."),
            &theme(),
            ModelConnectionSetupHandlers::default(),
        );
        assert!(node
            .texts().contains(&"That route refused the request."));
        assert!(node.texts().contains(&"Connection added."));
        assert_eq!(node.roles.get("stage").map(String::as_str), Some("configure"));
    }

    #[test]
    fn the_choose_stage_forwards_picker_selection_and_query_requests() {
        let recorder = Recorder::default();
        let node = model_connection_setup(&spec(), &theme(), recorder.handlers());

        let option = node
            .find(&|n| n.id.as_deref() == Some(model_connection_option_id("ollama-local").as_str()))
            .expect("option row");
        (option.interaction.on_activate.as_ref().expect("activation"))();

        let field = node
            .find(&|n| n.interaction.on_text_change.is_some())
            .expect("search field");
        (field.interaction.on_text_change.as_ref().unwrap())("ollama");

        assert_eq!(recorder.values.lock().unwrap().as_slice(), ["ollama-local"]);
        assert_eq!(recorder.queries.lock().unwrap().as_slice(), ["ollama"]);
    }

    #[test]
    fn the_configure_heading_can_actually_take_focus() {
        let node = model_connection_setup(&spec(), &theme(), ModelConnectionSetupHandlers::default());
        let heading = node
            .find(&|n| n.id.as_deref() == Some(MODEL_CONNECTION_SETUP_TITLE_ID))
            .expect("the workflow heading");
        assert!(heading.interaction.focusable);
        assert_eq!(heading.a11y.tab_index, Some(-1));
        assert!(
            heading.style.focus.is_some(),
            "the GPUI backend only tracks a focusable node that draws differently when focused"
        );
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let scoped = |scope: &str| ModelConnectionSetupHandlers {
            instance_id: Some(scope.to_string()),
            ..ModelConnectionSetupHandlers::default()
        };
        let first = model_connection_setup(&spec(), &theme(), scoped("first"));
        let second = model_connection_setup(&spec(), &theme(), scoped("second"));

        for (node, scope) in [(&first, "first"), (&second, "second")] {
            assert!(node
                .find(&|n| n.runtime_id.as_deref()
                    == Some(model_connection_setup_title_focus_id(Some(scope)).as_str()))
                .is_some());
            // The scope reaches the composed picker's options too.
            assert!(node
                .find(&|n| n.runtime_id.as_deref()
                    == Some(
                        model_connection_option_focus_id(Some(scope), "openai-responses").as_str()
                    ))
                .is_some());
        }
        assert!(first
            .find(&|n| n.runtime_id.as_deref()
                == Some(model_connection_setup_title_focus_id(Some("second")).as_str()))
            .is_none());
        assert!(first
            .find(&|n| n.id.as_deref() == Some(MODEL_CONNECTION_SETUP_TITLE_ID))
            .is_some());
    }

    #[test]
    fn a_scoped_setup_requests_scoped_focus_destinations() {
        let recorder = Recorder::default();
        let mut handlers = recorder.handlers();
        handlers.instance_id = Some("second".to_string());
        let node = model_connection_setup(
            &spec().with_value(Some("openai-responses".to_string())),
            &theme(),
            handlers,
        );
        press(&node, "Continue");
        assert_eq!(
            recorder.focus.lock().unwrap().as_slice(),
            [model_connection_setup_title_focus_id(Some("second"))]
        );

        let recorder = Recorder::default();
        let mut handlers = recorder.handlers();
        handlers.instance_id = Some("second".to_string());
        let node = model_connection_setup(
            &spec()
                .with_stage(ModelConnectionSetupStage::Configure)
                .with_value(Some("openai-responses".to_string())),
            &theme(),
            handlers,
        );
        press(&node, "Back");
        assert_eq!(
            recorder.focus.lock().unwrap().as_slice(),
            [model_connection_option_focus_id(Some("second"), "openai-responses")]
        );
    }

    #[test]
    fn cancel_requests_closure_and_the_root_is_a_region_not_a_dialog() {
        let recorder = Recorder::default();
        let node = model_connection_setup(&spec(), &theme(), recorder.handlers());
        press(&node, "Cancel");
        assert_eq!(*recorder.cancels.lock().unwrap(), 1);
        assert_eq!(node.a11y.role, Some(NodeRole::Region));
        assert_eq!(node.a11y.label.as_deref(), Some("Add model connection"));
    }
}
