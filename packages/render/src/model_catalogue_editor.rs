//! ModelCatalogueEditor — a controlled surface for ordering shown models and
//! hiding or restoring models from one configured connection.
//!
//! Contract: `docs/contracts/components/model-catalogue-editor.md`
//!
//! Structural only. Every move emits the complete shown-id order and every
//! visibility change emits `{ id, visible }`; nothing here mutates catalogue
//! truth, chooses a default, or decides where a restored model lands. The
//! derivations, the reorder result, the focus-after-hide rule, and the
//! announcements come from `poodle_headless::model_connection`.
//!
//! Three movement routes share one decision function: the explicit up/down
//! buttons, the keyboard grab-and-move on the handle, and the admitted pointer
//! drag. Keyboard grab/drop rides the backend's own activation path (Enter and
//! Space), arrows ride `on_key`, and Escape rides `on_cancel` — the vocabulary
//! has no other Escape channel, and binding Space twice would toggle twice.

use std::collections::BTreeMap;
use std::sync::Arc;

use poodle_headless::model_connection::{
    hidden_model_catalogue_items, model_catalogue_focus_after_hide,
    model_catalogue_grab_announcement, model_catalogue_reorder_announcement,
    model_catalogue_reorder_key_intent, model_catalogue_state_copy,
    model_catalogue_visibility_announcement, request_model_catalogue_order,
    request_model_catalogue_visibility, shown_model_catalogue_items, ModelCatalogueFocusAfterHide,
    ModelCatalogueItem, ModelCatalogueKeyIntent, ModelCatalogueState,
    ModelCatalogueVisibilityChange, ModelConnectionBadgeTone,
    MODEL_CATALOGUE_BOUNDARY_ANNOUNCEMENT, MODEL_CATALOGUE_CANCEL_GRAB_ANNOUNCEMENT,
    MODEL_CATALOGUE_DROP_ANNOUNCEMENT,
};
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
    NodeKey, NodeRole, StylePatch,
};
use poodle_specs::{
    ButtonVariant, CallOutSpec, CalloutAnnounceMode, CollapsibleSpec, ControlSize, EmptyStateSpec,
    EmptyStateVariant, IconButtonSpec, ModelCatalogueEditorSpec, PillAppearance, PillSpec,
    PillTone, SemanticControlSizeRole, StatusTone, MODEL_CATALOGUE_HIDDEN_SECTION_ID,
};

use crate::callout::{callout, CalloutHandlers};
use crate::collapsible::{collapsible_with_handlers, CollapsibleHandlers};
use crate::context::RenderContext;
use crate::empty_state::empty_state;
use crate::icon_button::icon_button;
use crate::pill::pill;
use crate::presentation::rem_to_px;

/// Contract §8: the label weight the header and row titles share.
const LABEL_WEIGHT: u16 = 500;

/// Host callbacks. Every one is a request against host-owned truth.
#[derive(Default)]
pub struct ModelCatalogueEditorHandlers {
    /// A valid move. The payload is the complete shown-id order.
    pub on_order_change: Option<Arc<dyn Fn(&[String]) + Send + Sync>>,
    /// Hide or Restore was activated. The host chooses the restored position.
    pub on_visibility_change: Option<Arc<dyn Fn(&ModelCatalogueVisibilityChange) + Send + Sync>>,
    /// The optional per-row info action. Setting it is what renders it.
    pub on_info: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// The keyboard grab changed: `Some(id)` grabbed, `None` dropped or
    /// cancelled. The host holds the grab and passes it back on the spec.
    pub on_grab_change: Option<Arc<dyn Fn(Option<&str>) + Send + Sync>>,
    /// A pointer drag is over this row, or has ended.
    pub on_drop_target_change: Option<Arc<dyn Fn(Option<&str>) + Send + Sync>>,
    /// The hidden section was disclosed or collapsed.
    pub on_hidden_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    /// Live-region copy for a move or a visibility request. The host stores it
    /// and passes it back as `live_message`.
    pub on_announce: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Focus destination after a move or a hide. The backend performs the move.
    pub on_focus_request: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Stable native instance scope. Semantic ids stay readable, but two
    /// editors over the same catalogue must never share backend focus handles.
    pub instance_id: Option<String>,
}

/// The backend-state id of one row's reorder handle: the instance scope when
/// the host supplied one, else the semantic id. Focus requests name this,
/// because it is what the backend keys focus handles by.
pub fn model_catalogue_handle_focus_id(instance_id: Option<&str>, item_id: &str) -> String {
    match instance_id {
        Some(scope) => format!("model-catalogue-editor:{scope}:{item_id}:handle"),
        None => format!("model-catalogue-editor:{item_id}:handle"),
    }
}

/// The backend-state id of the hidden-section disclosure, scoped the same way.
pub fn model_catalogue_hidden_focus_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("model-catalogue-editor:{scope}:hidden"),
        None => MODEL_CATALOGUE_HIDDEN_SECTION_ID.to_string(),
    }
}

/// The backend-state id of the hidden-section content region.
pub fn model_catalogue_hidden_content_focus_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("model-catalogue-editor:{scope}:hidden-content"),
        None => "model-catalogue-hidden-content".to_string(),
    }
}

fn scoped_row_id(instance_id: Option<&str>, item_id: &str, part: &str) -> Option<String> {
    instance_id.map(|scope| format!("model-catalogue-editor:{scope}:{item_id}:{part}"))
}

/// Host-composed content keyed by opaque model id. Poodle resolves no model
/// mark and reads no capability metadata.
#[derive(Default)]
pub struct ModelCatalogueEditorSlots {
    /// Optional model/provider mark per item id.
    pub leading: BTreeMap<String, Node>,
    /// Optional safe capability metadata per item id.
    pub row_meta: BTreeMap<String, Node>,
    /// A custom model or refresh action in the header.
    pub custom_action: Option<Node>,
}

pub fn model_catalogue_editor(
    spec: &ModelCatalogueEditorSpec,
    ctx: &RenderContext<'_>,
    handlers: ModelCatalogueEditorHandlers,
) -> Node {
    model_catalogue_editor_with_slots(spec, ctx, ModelCatalogueEditorSlots::default(), handlers)
}

pub fn model_catalogue_editor_with_slots(
    spec: &ModelCatalogueEditorSpec,
    ctx: &RenderContext<'_>,
    slots: ModelCatalogueEditorSlots,
    handlers: ModelCatalogueEditorHandlers,
) -> Node {
    let mut slots = slots;
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let shown = shown_model_catalogue_items(&spec.items);
    let hidden = hidden_model_catalogue_items(&spec.items);
    let shown_ids: Vec<String> = shown.iter().map(|item| item.id.clone()).collect();
    let handlers = Arc::new(handlers);

    // ── Header ──
    let mut heading = Node::container();
    {
        let s = &mut heading.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.min_width = Some(0.0);
    }
    let mut title = Node::text(&spec.title);
    title.style.text_size = Some(ctx.theme().resolve_space("typography.body.size"));
    title.style.text_weight = Some(LABEL_WEIGHT);
    title.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.primary"));
    let mut heading = heading.child(title);
    if spec.state == ModelCatalogueState::Ready {
        heading = heading.child(secondary_text(ctx, &count_line(shown.len(), hidden.len())));
    }

    let mut header = Node::container();
    {
        let s = &mut header.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.md");
        s.flex_wrap = true;
        s.fill_width = true;
    }
    let mut header = header.child(heading);
    if let Some(action) = slots.custom_action.take() {
        header = header.child(action);
    }

    // ── Root ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.md");
        s.fill_width = true;
        s.min_width = Some(0.0);
    }
    root.a11y.role = Some(NodeRole::Region);
    root.a11y.label = Some(spec.effective_aria_label().to_string());
    root.roles
        .insert("state".to_string(), state_role(spec.state).to_string());
    root.roles
        .insert("pending".to_string(), spec.is_pending.to_string());
    let mut root = root.child(header).child(live_region(ctx, spec));

    if spec.state != ModelCatalogueState::Ready {
        return root.child(state_region(spec, ctx, effective_size));
    }

    // ── Shown list ──
    let mut list = Node::container();
    {
        let s = &mut list.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
        s.fill_width = true;
    }
    list.a11y.role = Some(NodeRole::List);
    list.a11y.label = Some("Shown models".to_string());
    let mut list = list;
    for (index, item) in shown.iter().enumerate() {
        list = list.child(shown_row(
            spec,
            ctx,
            effective_size,
            item,
            index,
            &shown,
            &shown_ids,
            &slots,
            &handlers,
        ));
    }
    root = root.child(list);

    // ── Hidden section ──
    if !hidden.is_empty() {
        let mut hidden_list = Node::container();
        {
            let s = &mut hidden_list.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
            s.fill_width = true;
        }
        hidden_list.a11y.role = Some(NodeRole::List);
        hidden_list.a11y.label = Some(spec.hidden_title.clone());
        let mut hidden_list = hidden_list;
        for item in &hidden {
            hidden_list = hidden_list.child(hidden_row(spec, ctx, effective_size, item, &handlers));
        }

        let on_open = handlers.on_hidden_open_change.clone().map(|handler| {
            Arc::new(move |open: bool| handler(open)) as Arc<dyn Fn(bool) + Send + Sync>
        });
        let mut section = collapsible_with_handlers(
            &CollapsibleSpec::new()
                .with_title(spec.hidden_title.clone())
                .with_open(spec.hidden_open)
                .with_size(effective_size)
                .with_density(density),
            ctx,
            Some(hidden_list),
            CollapsibleHandlers {
                on_open_change: on_open,
                instance_id: handlers.instance_id.clone(),
            },
        );
        // The focus destination when the last shown model is hidden. The
        // outer region Collapsible returns is not focusable — its trigger is —
        // so naming the outer node would hand the backend a destination it can
        // never focus.
        mark_hidden_disclosure(&mut section, handlers.instance_id.as_deref(), ctx);
        let mut wrapper = Node::container();
        {
            let s = &mut wrapper.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
            let pad = &mut s.descriptor.layout.spacing.padding;
            let inset = ctx.theme().resolve_space("space.stack.sm");
            pad.top = inset;
            pad.bottom = inset;
            pad.left = inset;
            pad.right = inset;
            s.descriptor.background = Some(ctx.theme().resolve_color("color.background.panel"));
            s.fill_width = true;
            let radius = ctx.theme().resolve_radius("radius.surface");
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
        }
        root = root.child(wrapper.child(section));
    }

    root
}

/// Stamp the hidden-section disclosure on `Collapsible`'s direct trigger and
/// optional content wrapper only. Nested focusable controls keep their own ids.
fn mark_hidden_disclosure(
    section: &mut Node,
    instance_id: Option<&str>,
    ctx: &RenderContext<'_>,
) {
    let trigger_focus = model_catalogue_hidden_focus_id(instance_id);
    let content_focus = model_catalogue_hidden_content_focus_id(instance_id);

    if let Some(trigger) = section.children.first_mut() {
        if trigger.interaction.focusable {
            trigger.id = Some(MODEL_CATALOGUE_HIDDEN_SECTION_ID.to_string());
            trigger.runtime_id = Some(trigger_focus.clone());
            trigger.a11y.controls = Some(content_focus.clone());
            trigger.style.focus = Some(StylePatch {
                border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
                ..StylePatch::default()
            });
        }
    }

    if let Some(content) = section.children.get_mut(1) {
        if content.a11y.role == Some(NodeRole::Region) {
            content.id = Some(content_focus.clone());
            content.runtime_id = Some(content_focus);
            content.a11y.labelled_by = Some(trigger_focus);
        }
    }
}

/// `icon_button` renders no focus patch, and the GPUI backend only creates a
/// focus handle for a focusable node that carries one — so every utility here
/// would be unreachable by keyboard, and a keyboard reorder could not receive
/// focus at all (PAPERCUTS: icon-button focus patch). Same workaround
/// `poodle-render::history_center` already carries.
fn focusable_chrome(mut node: Node, ctx: &RenderContext<'_>) -> Node {
    node.style.focus = Some(StylePatch {
        border_color: Some(ctx.theme().resolve_color("color.accent.focusRing")),
        ..StylePatch::default()
    });
    node
}

fn count_line(shown: usize, hidden: usize) -> String {
    if hidden > 0 {
        format!("{shown} shown, {hidden} hidden")
    } else {
        format!("{shown} shown")
    }
}

fn state_role(state: ModelCatalogueState) -> &'static str {
    match state {
        ModelCatalogueState::Ready => "ready",
        ModelCatalogueState::Loading => "loading",
        ModelCatalogueState::Unavailable => "unavailable",
        ModelCatalogueState::Empty => "empty",
        ModelCatalogueState::Error => "error",
        ModelCatalogueState::SessionNegotiated => "sessionNegotiated",
    }
}

fn badge_tone(tone: ModelConnectionBadgeTone) -> PillTone {
    match tone {
        ModelConnectionBadgeTone::Neutral => PillTone::Neutral,
        ModelConnectionBadgeTone::Info => PillTone::Info,
        ModelConnectionBadgeTone::Success => PillTone::Success,
        ModelConnectionBadgeTone::Warning => PillTone::Warning,
        ModelConnectionBadgeTone::Danger => PillTone::Danger,
    }
}

fn secondary_text(ctx: &RenderContext<'_>, content: &str) -> Node {
    let mut node = Node::text(content);
    node.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
    node.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.secondary"));
    node
}

/// The polite atomic live region. Its copy is host state: a callback asks for
/// it, the host stores it, and the next render says it.
fn live_region(ctx: &RenderContext<'_>, spec: &ModelCatalogueEditorSpec) -> Node {
    let mut clip = Node::container();
    {
        let s = &mut clip.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(1.0);
        s.descriptor.layout.height = LayoutSizing::Fixed(1.0);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
    }
    clip.a11y.role = Some(NodeRole::Status);
    clip.a11y.label = Some(spec.live_message.clone());
    clip.child(secondary_text(ctx, &spec.live_message))
}

/// The non-ready postures. Each one is distinct: loading and the neutral
/// postures use EmptyState, error uses a danger Callout.
fn state_region(
    spec: &ModelCatalogueEditorSpec,
    ctx: &RenderContext<'_>,
    effective_size: ControlSize,
) -> Node {
    let defaults = model_catalogue_state_copy(spec.state);
    let density = ctx.resolve_density(spec.density);
    let title = spec.state_title.clone().unwrap_or(defaults.title);
    let message = spec.state_message.clone().unwrap_or(defaults.message);

    if spec.state == ModelCatalogueState::Error {
        return callout(
            &CallOutSpec::new()
                .with_tone(StatusTone::Danger)
                .with_title(title)
                .with_content(message)
                .with_announce_mode(CalloutAnnounceMode::Assertive)
                .with_size(effective_size)
                .with_density(density),
            ctx,
            CalloutHandlers::default(),
        );
    }

    let mut state_spec = EmptyStateSpec::new(title)
        .with_variant(if spec.state == ModelCatalogueState::Empty {
            EmptyStateVariant::FirstRun
        } else {
            EmptyStateVariant::Neutral
        })
        .with_density(density);
    if !message.is_empty() {
        state_spec = state_spec.with_message(message);
    }
    empty_state(&state_spec, ctx)
}

#[allow(clippy::too_many_arguments)]
fn shown_row(
    spec: &ModelCatalogueEditorSpec,
    ctx: &RenderContext<'_>,
    effective_size: ControlSize,
    item: &ModelCatalogueItem,
    index: usize,
    shown: &[ModelCatalogueItem],
    shown_ids: &[String],
    slots: &ModelCatalogueEditorSlots,
    handlers: &Arc<ModelCatalogueEditorHandlers>,
) -> Node {
    let density = ctx.resolve_density(spec.density);
    let locked = spec.is_locked();
    let row_locked = locked || item.is_disabled;
    let accent = ctx.theme().resolve_color("color.accent.base");
    let border = ctx.theme().resolve_color("color.border.subtle");
    let is_grabbed = spec.grabbed_id.as_deref() == Some(item.id.as_str());
    let is_drop_target = spec.drop_target_id.as_deref() == Some(item.id.as_str());

    let move_to = move_emitter(
        spec,
        shown.to_vec(),
        shown_ids.to_vec(),
        Arc::clone(handlers),
    );

    // ── Reorder handle: grab, keyboard move, and the pointer drag source ──
    let handle = icon_button(
        &IconButtonSpec::new()
            .with_icon("grip-vertical")
            .with_variant(ButtonVariant::Ghost)
            .with_size_role(SemanticControlSizeRole::Chrome)
            .with_size(effective_size)
            .with_density(density)
            .with_aria_label(format!(
                "{}, position {} of {}",
                item.label,
                index + 1,
                shown.len()
            ))
            .with_pressed(is_grabbed)
            .with_disabled(row_locked),
        ctx,
        (!row_locked).then(|| {
            let handlers = Arc::clone(handlers);
            let id = item.id.clone();
            let label = item.label.clone();
            let grabbed_index = grabbed_index(spec, shown_ids);
            let item_count = shown.len();
            Arc::new(move || {
                // Enter, Space, and a pointer click all arrive here; the shared
                // intent function decides whether that is a grab or a drop.
                match model_catalogue_reorder_key_intent(" ", index, grabbed_index, item_count) {
                    Some(ModelCatalogueKeyIntent::Drop) => {
                        announce(&handlers, MODEL_CATALOGUE_DROP_ANNOUNCEMENT);
                        grab(&handlers, None);
                    }
                    _ => {
                        announce(&handlers, &model_catalogue_grab_announcement(&label));
                        grab(&handlers, Some(&id));
                    }
                }
            }) as Arc<dyn Fn() + Send + Sync>
        }),
    );
    let mut handle = focusable_chrome(handle, ctx);
    handle.id = Some(spec.row_handle_id(&item.id));
    handle.runtime_id = scoped_row_id(handlers.instance_id.as_deref(), &item.id, "handle");
    handle.a11y.toggled = Some(if is_grabbed {
        poodle_node::NodeToggled::True
    } else {
        poodle_node::NodeToggled::False
    });
    if !row_locked {
        let arrows = {
            let handlers = Arc::clone(handlers);
            let move_to = move_to.clone();
            let shown_ids = shown_ids.to_vec();
            let grabbed_index = grabbed_index(spec, shown_ids.as_slice());
            move |key: NodeKey, _modifiers: poodle_node::NodeModifiers| -> Option<String> {
                let key = match key {
                    NodeKey::ArrowUp => "ArrowUp",
                    NodeKey::ArrowDown => "ArrowDown",
                    // Space is the backend's activation path; handling it here
                    // as well would grab and drop in one keystroke.
                    _ => return None,
                };
                match model_catalogue_reorder_key_intent(
                    key,
                    index,
                    grabbed_index,
                    shown_ids.len(),
                )? {
                    ModelCatalogueKeyIntent::Move { from, to } => move_to(from, to),
                    ModelCatalogueKeyIntent::Boundary => {
                        announce(&handlers, MODEL_CATALOGUE_BOUNDARY_ANNOUNCEMENT);
                        None
                    }
                    _ => None,
                }
            }
        };
        handle.interaction.on_key = Some(Arc::new(arrows));

        // Escape cancels a live grab. The vocabulary routes Escape through
        // `on_cancel`; no other channel carries it to a plain control.
        if is_grabbed {
            let handlers = Arc::clone(handlers);
            handle.interaction.on_cancel = Some(Arc::new(move || {
                announce(&handlers, MODEL_CATALOGUE_CANCEL_GRAB_ANNOUNCEMENT);
                grab(&handlers, None);
            }));
        }

        if spec.is_drag_enabled {
            handle.interaction.drag_payload = Some(item.id.clone());
        }
    }

    // ── Identity ──
    let mut label_row = Node::container();
    {
        let s = &mut label_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.xs");
        s.flex_wrap = true;
        s.min_width = Some(0.0);
    }
    let mut label_row = label_row;
    if let Some(mark) = slots.leading.get(&item.id) {
        let mut mark = mark.clone();
        mark.style.flex_none = true;
        label_row = label_row.child(mark);
    }
    let mut label = Node::text(&item.label);
    label.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
    label.style.text_weight = Some(LABEL_WEIGHT);
    label.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.primary"));
    label.style.text_ellipsis = true;
    let mut label_row = label_row.child(label);
    if let Some(provider) = item.provider_label.as_deref() {
        label_row = label_row.child(secondary_text(ctx, provider));
    }

    let mut identity = Node::container();
    {
        let s = &mut identity.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }
    let mut identity = identity.child(label_row);
    if let Some(description) = item.description.as_deref() {
        let mut node = secondary_text(ctx, description);
        node.style.text_ellipsis = true;
        identity = identity.child(node);
    }
    if let Some(meta) = slots.row_meta.get(&item.id) {
        identity = identity.child(meta.clone());
    }

    // ── Utilities ──
    let mut utilities = Node::container();
    {
        let s = &mut utilities.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::End;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.flex_wrap = true;
        s.flex_none = true;
    }
    let mut utilities = utilities;
    for badge in &item.badges {
        utilities = utilities.child(pill(
            &PillSpec::new()
                .with_label(badge.label.clone())
                .with_tone(badge_tone(badge.tone))
                .with_appearance(PillAppearance::Subtle)
                .with_density(density),
            ctx,
        ));
    }
    if let Some(handler) = &handlers.on_info {
        let handler = Arc::clone(handler);
        let id = item.id.clone();
        let info = icon_button(
            &IconButtonSpec::new()
                .with_icon("info")
                .with_variant(ButtonVariant::Ghost)
                .with_size_role(SemanticControlSizeRole::Chrome)
                .with_size(effective_size)
                .with_density(density)
                .with_aria_label(format!("About {}", item.label))
                .with_disabled(locked),
            ctx,
            (!locked).then(|| Arc::new(move || handler(&id)) as Arc<dyn Fn() + Send + Sync>),
        );
        let mut info = focusable_chrome(info, ctx);
        info.id = Some(format!("model-catalogue-editor:{}:info", item.id));
        info.runtime_id = scoped_row_id(handlers.instance_id.as_deref(), &item.id, "info");
        utilities = utilities.child(info);
    }
    if spec.show_move_actions {
        let up_disabled = row_locked || index == 0;
        let up = icon_button(
            &IconButtonSpec::new()
                .with_icon("arrow-up")
                .with_variant(ButtonVariant::Ghost)
                .with_size_role(SemanticControlSizeRole::Chrome)
                .with_size(effective_size)
                .with_density(density)
                .with_aria_label(format!("Move {} up", item.label))
                .with_disabled(up_disabled),
            ctx,
            (!up_disabled).then(|| {
                let move_to = move_to.clone();
                Arc::new(move || {
                    move_to(index, index - 1);
                }) as Arc<dyn Fn() + Send + Sync>
            }),
        );
        let mut up = focusable_chrome(up, ctx);
        up.id = Some(format!("model-catalogue-editor:{}:up", item.id));
        up.runtime_id = scoped_row_id(handlers.instance_id.as_deref(), &item.id, "up");

        let down_disabled = row_locked || index + 1 == shown.len();
        let down = icon_button(
            &IconButtonSpec::new()
                .with_icon("arrow-down")
                .with_variant(ButtonVariant::Ghost)
                .with_size_role(SemanticControlSizeRole::Chrome)
                .with_size(effective_size)
                .with_density(density)
                .with_aria_label(format!("Move {} down", item.label))
                .with_disabled(down_disabled),
            ctx,
            (!down_disabled).then(|| {
                let move_to = move_to.clone();
                Arc::new(move || {
                    move_to(index, index + 1);
                }) as Arc<dyn Fn() + Send + Sync>
            }),
        );
        let mut down = focusable_chrome(down, ctx);
        down.id = Some(format!("model-catalogue-editor:{}:down", item.id));
        down.runtime_id = scoped_row_id(handlers.instance_id.as_deref(), &item.id, "down");
        utilities = utilities.child(up).child(down);
    }
    let hide = icon_button(
        &IconButtonSpec::new()
            .with_icon("eye")
            .with_variant(ButtonVariant::Ghost)
            .with_size_role(SemanticControlSizeRole::Chrome)
            .with_size(effective_size)
            .with_density(density)
            .with_aria_label(format!("Hide {}", item.label))
            .with_disabled(row_locked),
        ctx,
        (!row_locked).then(|| {
            let handlers = Arc::clone(handlers);
            let shown_ids = shown_ids.to_vec();
            let item = item.clone();
            let hidden_section = model_catalogue_hidden_focus_id(handlers.instance_id.as_deref());
            let instance = handlers.instance_id.clone();
            Arc::new(move || {
                let focus = model_catalogue_focus_after_hide(&shown_ids, &item.id);
                if let Some(handler) = &handlers.on_visibility_change {
                    handler(&request_model_catalogue_visibility(&item.id, false));
                }
                announce(
                    &handlers,
                    &model_catalogue_visibility_announcement(&item.label, false),
                );
                if let Some(handler) = &handlers.on_focus_request {
                    match &focus {
                        ModelCatalogueFocusAfterHide::Shown { id } => {
                            handler(&model_catalogue_handle_focus_id(instance.as_deref(), id))
                        }
                        ModelCatalogueFocusAfterHide::HiddenSection => {
                            // Hiding the last shown model discloses the hidden
                            // section, exactly as the web does, so the focus
                            // destination exists to receive it.
                            if let Some(open) = &handlers.on_hidden_open_change {
                                open(true);
                            }
                            handler(&hidden_section)
                        }
                    }
                }
            }) as Arc<dyn Fn() + Send + Sync>
        }),
    );
    let mut hide = focusable_chrome(hide, ctx);
    hide.id = Some(format!("model-catalogue-editor:{}:hide", item.id));
    hide.runtime_id = scoped_row_id(handlers.instance_id.as_deref(), &item.id, "hide");
    let utilities = utilities.child(hide);

    // ── Row ──
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = ctx.theme().resolve_space("space.stack.sm");
        pad.bottom = ctx.theme().resolve_space("space.stack.sm");
        pad.left = ctx.theme().resolve_space("space.inline.md");
        pad.right = ctx.theme().resolve_space("space.inline.md");
        s.descriptor.border.width = rem_to_px(0.0625);
        s.descriptor.border.color = if is_grabbed || is_drop_target {
            accent
        } else {
            border
        };
        s.descriptor.background = Some(ctx.theme().resolve_color("color.background.surface"));
        s.flex_wrap = true;
        s.fill_width = true;
        s.min_width = Some(0.0);
        let radius = ctx.theme().resolve_radius("radius.control");
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
    }
    row.a11y.role = Some(NodeRole::ListItem);
    row.a11y.level = Some(index + 1);
    row.roles
        .insert("grabbed".to_string(), is_grabbed.to_string());
    row.roles
        .insert("dropTarget".to_string(), is_drop_target.to_string());

    // Every row is a drop zone, so a dragged handle can land on any of them.
    if spec.is_drag_enabled && !locked {
        row.interaction.drop_zone = true;
        if let Some(handler) = &handlers.on_drop_target_change {
            let handler = Arc::clone(handler);
            let id = item.id.clone();
            let leave = Arc::clone(&handler);
            row.interaction.on_drop_hover = Some(Arc::new(move |_event| handler(Some(&id))));
            row.interaction.on_drop_leave = Some(Arc::new(move || leave(None)));
        }
        let handlers = Arc::clone(handlers);
        let shown_ids = shown_ids.to_vec();
        let move_to = move_to.clone();
        row.interaction.on_drop = Some(Arc::new(move |event: &poodle_node::NodeDropEvent| {
            if let Some(handler) = &handlers.on_drop_target_change {
                handler(None);
            }
            grab(&handlers, None);
            let Some(from) = shown_ids.iter().position(|id| id == &event.payload) else {
                return;
            };
            move_to(from, index);
        }));
    }

    row.child(handle).child(identity).child(utilities)
}

fn hidden_row(
    spec: &ModelCatalogueEditorSpec,
    ctx: &RenderContext<'_>,
    effective_size: ControlSize,
    item: &ModelCatalogueItem,
    handlers: &Arc<ModelCatalogueEditorHandlers>,
) -> Node {
    let density = ctx.resolve_density(spec.density);
    let row_locked = spec.is_locked() || item.is_disabled;

    let mut label_row = Node::container();
    {
        let s = &mut label_row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.xs");
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }
    let mut label = Node::text(&item.label);
    label.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
    label.style.text_weight = Some(LABEL_WEIGHT);
    label.style.text_ellipsis = true;
    let mut label_row = label_row.child(label);
    if let Some(provider) = item.provider_label.as_deref() {
        label_row = label_row.child(secondary_text(ctx, provider));
    }

    let restore = icon_button(
        &IconButtonSpec::new()
            .with_icon("undo")
            .with_variant(ButtonVariant::Ghost)
            .with_size_role(SemanticControlSizeRole::Chrome)
            .with_size(effective_size)
            .with_density(density)
            .with_aria_label(format!("Restore {}", item.label))
            .with_disabled(row_locked),
        ctx,
        (!row_locked).then(|| {
            let handlers = Arc::clone(handlers);
            let item = item.clone();
            Arc::new(move || {
                if let Some(handler) = &handlers.on_visibility_change {
                    handler(&request_model_catalogue_visibility(&item.id, true));
                }
                announce(
                    &handlers,
                    &model_catalogue_visibility_announcement(&item.label, true),
                );
            }) as Arc<dyn Fn() + Send + Sync>
        }),
    );
    let mut restore = focusable_chrome(restore, ctx);
    restore.id = Some(format!("model-catalogue-editor:{}:restore", item.id));
    restore.runtime_id = scoped_row_id(handlers.instance_id.as_deref(), &item.id, "restore");

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
        s.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.tertiary"));
        s.fill_width = true;
        s.min_width = Some(0.0);
    }
    row.a11y.role = Some(NodeRole::ListItem);
    row.child(label_row).child(restore)
}

/// The one move path: it derives the complete shown-id order, emits it,
/// announces it, and names the moved row as the focus destination.
fn move_emitter(
    spec: &ModelCatalogueEditorSpec,
    shown: Vec<ModelCatalogueItem>,
    shown_ids: Vec<String>,
    handlers: Arc<ModelCatalogueEditorHandlers>,
) -> Arc<dyn Fn(usize, usize) -> Option<String> + Send + Sync> {
    let locked = spec.is_locked();
    let instance = handlers.instance_id.clone();
    Arc::new(move |from: usize, to: usize| {
        if locked {
            return None;
        }
        let next = request_model_catalogue_order(&shown_ids, from, to)?;
        if let Some(handler) = &handlers.on_order_change {
            handler(&next);
        }
        let label = shown
            .get(from)
            .map(|item| item.label.clone())
            .unwrap_or_else(|| "model".to_string());
        announce(
            &handlers,
            &model_catalogue_reorder_announcement(&label, to + 1, next.len()),
        );
        let focus = next
            .get(to)
            .map(|id| model_catalogue_handle_focus_id(instance.as_deref(), id));
        if let (Some(handler), Some(target)) = (&handlers.on_focus_request, focus.as_deref()) {
            handler(target);
        }
        focus
    })
}

fn grabbed_index(spec: &ModelCatalogueEditorSpec, shown_ids: &[String]) -> Option<usize> {
    let grabbed = spec.grabbed_id.as_deref()?;
    shown_ids.iter().position(|id| id == grabbed)
}

fn announce(handlers: &Arc<ModelCatalogueEditorHandlers>, message: &str) {
    if let Some(handler) = &handlers.on_announce {
        handler(message);
    }
}

fn grab(handlers: &Arc<ModelCatalogueEditorHandlers>, id: Option<&str>) {
    if let Some(handler) = &handlers.on_grab_change {
        handler(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::model_connection::model_catalogue_fixtures;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> ModelCatalogueEditorSpec {
        ModelCatalogueEditorSpec::new().with_items(model_catalogue_fixtures())
    }

    #[derive(Default)]
    struct Recorder {
        orders: Arc<Mutex<Vec<Vec<String>>>>,
        visibility: Arc<Mutex<Vec<(String, bool)>>>,
        info: Arc<Mutex<Vec<String>>>,
        grabs: Arc<Mutex<Vec<Option<String>>>>,
        drop_targets: Arc<Mutex<Vec<Option<String>>>>,
        hidden_open: Arc<Mutex<Vec<bool>>>,
        announcements: Arc<Mutex<Vec<String>>>,
        focus: Arc<Mutex<Vec<String>>>,
    }

    impl Recorder {
        fn handlers(&self) -> ModelCatalogueEditorHandlers {
            let orders = Arc::clone(&self.orders);
            let visibility = Arc::clone(&self.visibility);
            let info = Arc::clone(&self.info);
            let grabs = Arc::clone(&self.grabs);
            let drop_targets = Arc::clone(&self.drop_targets);
            let hidden_open = Arc::clone(&self.hidden_open);
            let announcements = Arc::clone(&self.announcements);
            let focus = Arc::clone(&self.focus);
            ModelCatalogueEditorHandlers {
                on_order_change: Some(Arc::new(move |order: &[String]| {
                    orders.lock().unwrap().push(order.to_vec())
                })),
                on_visibility_change: Some(Arc::new(
                    move |change: &ModelCatalogueVisibilityChange| {
                        visibility
                            .lock()
                            .unwrap()
                            .push((change.id.clone(), change.visible))
                    },
                )),
                on_info: Some(Arc::new(move |id: &str| {
                    info.lock().unwrap().push(id.to_string())
                })),
                on_grab_change: Some(Arc::new(move |id: Option<&str>| {
                    grabs.lock().unwrap().push(id.map(str::to_string))
                })),
                on_drop_target_change: Some(Arc::new(move |id: Option<&str>| {
                    drop_targets.lock().unwrap().push(id.map(str::to_string))
                })),
                on_hidden_open_change: Some(Arc::new(move |open| {
                    hidden_open.lock().unwrap().push(open)
                })),
                on_announce: Some(Arc::new(move |message: &str| {
                    announcements.lock().unwrap().push(message.to_string())
                })),
                on_focus_request: Some(Arc::new(move |id: &str| {
                    focus.lock().unwrap().push(id.to_string())
                })),
                instance_id: None,
            }
        }
    }

    fn named<'a>(node: &'a Node, aria: &str) -> &'a Node {
        node.find(&|n| n.a11y.label.as_deref() == Some(aria))
            .unwrap_or_else(|| panic!("no control named {aria}"))
    }

    fn press(node: &Node, aria: &str) {
        (named(node, aria)
            .interaction
            .on_activate
            .as_ref()
            .unwrap_or_else(|| panic!("{aria} is enabled")))();
    }

    fn shown_ids() -> Vec<String> {
        shown_model_catalogue_items(&model_catalogue_fixtures())
            .iter()
            .map(|item| item.id.clone())
            .collect()
    }

    #[test]
    fn explicit_moves_emit_the_complete_shown_order_and_follow_focus() {
        let recorder = Recorder::default();
        let node =
            model_catalogue_editor(&spec(), &RenderContext::new(&theme()), recorder.handlers());

        press(&node, "Move Frontier Alpha down");
        assert_eq!(
            recorder.orders.lock().unwrap().as_slice(),
            [vec![
                "model-beta".to_string(),
                "model-alpha".to_string(),
                "model-gamma".to_string(),
                "model-dup-a".to_string(),
            ]]
        );
        assert_eq!(
            recorder.announcements.lock().unwrap().as_slice(),
            ["Moved Frontier Alpha to position 2 of 4."]
        );
        assert_eq!(
            recorder.focus.lock().unwrap().as_slice(),
            ["model-catalogue-editor:model-alpha:handle"]
        );
    }

    #[test]
    fn move_actions_are_disabled_at_the_boundaries() {
        let node = model_catalogue_editor(
            &spec(),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers::default(),
        );
        assert!(named(&node, "Move Frontier Alpha up").interaction.disabled);
        assert!(
            !named(&node, "Move Frontier Alpha down")
                .interaction
                .disabled
        );
        assert!(named(&node, "Move Shared Label down").interaction.disabled);
        assert!(!named(&node, "Move Shared Label up").interaction.disabled);
    }

    #[test]
    fn the_handle_grabs_drops_and_moves_by_keyboard() {
        let recorder = Recorder::default();
        let node =
            model_catalogue_editor(&spec(), &RenderContext::new(&theme()), recorder.handlers());

        // Enter/Space and a pointer click all reach the same activation.
        press(&node, "Frontier Beta, position 2 of 4");
        assert_eq!(
            recorder.grabs.lock().unwrap().as_slice(),
            [Some("model-beta".to_string())]
        );
        assert_eq!(
            recorder.announcements.lock().unwrap().as_slice(),
            ["Grabbed Frontier Beta. Use arrow keys to move, Escape to cancel."]
        );

        // With the grab held, arrows move the grabbed row.
        let grabbed = model_catalogue_editor(
            &spec().with_grabbed(Some("model-beta".to_string())),
            &RenderContext::new(&theme()),
            recorder.handlers(),
        );
        let handle = named(&grabbed, "Frontier Beta, position 2 of 4");
        let keys = handle.interaction.on_key.as_ref().expect("arrow handler");
        assert_eq!(
            keys(NodeKey::ArrowDown, poodle_node::NodeModifiers::default()),
            Some("model-catalogue-editor:model-beta:handle".to_string())
        );
        assert_eq!(
            recorder.orders.lock().unwrap().last().unwrap().as_slice(),
            [
                "model-alpha".to_string(),
                "model-gamma".to_string(),
                "model-beta".to_string(),
                "model-dup-a".to_string(),
            ]
        );

        // A second activation on the grabbed row drops it.
        press(&grabbed, "Frontier Beta, position 2 of 4");
        assert_eq!(recorder.grabs.lock().unwrap().last().unwrap(), &None);
        assert!(recorder
            .announcements
            .lock()
            .unwrap()
            .contains(&"Dropped item.".to_string()));

        // Escape cancels the grab.
        (handle
            .interaction
            .on_cancel
            .as_ref()
            .expect("escape cancels a live grab"))();
        assert!(recorder
            .announcements
            .lock()
            .unwrap()
            .contains(&"Cancelled keyboard move.".to_string()));
    }

    #[test]
    fn arrow_keys_report_the_list_boundary_without_emitting_an_order() {
        let recorder = Recorder::default();
        let node =
            model_catalogue_editor(&spec(), &RenderContext::new(&theme()), recorder.handlers());
        let handle = named(&node, "Frontier Alpha, position 1 of 4");
        let keys = handle.interaction.on_key.as_ref().expect("arrow handler");
        assert_eq!(
            keys(NodeKey::ArrowUp, poodle_node::NodeModifiers::default()),
            None
        );
        assert_eq!(
            recorder.announcements.lock().unwrap().as_slice(),
            ["Reached list boundary."]
        );
        assert!(recorder.orders.lock().unwrap().is_empty());
    }

    #[test]
    fn an_admitted_pointer_drag_moves_through_the_same_order_payload() {
        let recorder = Recorder::default();
        let node =
            model_catalogue_editor(&spec(), &RenderContext::new(&theme()), recorder.handlers());

        let handle = named(&node, "Frontier Alpha, position 1 of 4");
        assert_eq!(
            handle.interaction.drag_payload.as_deref(),
            Some("model-alpha")
        );

        let row = node
            .find(&|n| {
                n.a11y.role == Some(NodeRole::ListItem)
                    && n.find(&|c| {
                        c.a11y.label.as_deref() == Some("Gateway Gamma, position 3 of 4")
                    })
                    .is_some()
            })
            .expect("the third row");
        assert!(row.interaction.drop_zone);
        (row.interaction.on_drop.as_ref().expect("drop"))(&poodle_node::NodeDropEvent {
            payload: "model-alpha".to_string(),
            edge: poodle_node::DropEdge::Inside,
        });

        assert_eq!(
            recorder.orders.lock().unwrap().as_slice(),
            [vec![
                "model-beta".to_string(),
                "model-gamma".to_string(),
                "model-alpha".to_string(),
                "model-dup-a".to_string(),
            ]]
        );
    }

    #[test]
    fn drag_can_be_turned_off_while_keyboard_and_buttons_remain() {
        let node = model_catalogue_editor(
            &spec().with_drag_enabled(false),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers::default(),
        );
        let handle = named(&node, "Frontier Alpha, position 1 of 4");
        assert!(handle.interaction.drag_payload.is_none());
        assert!(handle.interaction.on_key.is_some());
        assert!(handle.interaction.on_activate.is_some());
        assert!(
            !named(&node, "Move Frontier Alpha down")
                .interaction
                .disabled
        );
    }

    #[test]
    fn hiding_emits_only_visibility_and_moves_focus_to_the_next_shown_model() {
        let recorder = Recorder::default();
        let node =
            model_catalogue_editor(&spec(), &RenderContext::new(&theme()), recorder.handlers());
        press(&node, "Hide Frontier Beta");

        assert_eq!(
            recorder.visibility.lock().unwrap().as_slice(),
            [("model-beta".to_string(), false)]
        );
        assert!(
            recorder.orders.lock().unwrap().is_empty(),
            "hiding never reorders"
        );
        assert_eq!(
            recorder.announcements.lock().unwrap().as_slice(),
            ["Hid Frontier Beta."]
        );
        assert_eq!(
            recorder.focus.lock().unwrap().as_slice(),
            ["model-catalogue-editor:model-gamma:handle"]
        );
    }

    #[test]
    fn hiding_the_last_shown_model_discloses_and_focuses_the_hidden_section() {
        let recorder = Recorder::default();
        let only = ModelCatalogueEditorSpec::new().with_items(vec![
            ModelCatalogueItem::new("model-solo", "Solo"),
            ModelCatalogueItem::new("model-gone", "Gone").with_visible(false),
        ]);
        let node =
            model_catalogue_editor(&only, &RenderContext::new(&theme()), recorder.handlers());
        press(&node, "Hide Solo");

        assert_eq!(recorder.hidden_open.lock().unwrap().as_slice(), [true]);
        assert_eq!(
            recorder.focus.lock().unwrap().as_slice(),
            [MODEL_CATALOGUE_HIDDEN_SECTION_ID]
        );
    }

    #[test]
    fn restoring_emits_visibility_and_never_chooses_a_position() {
        let recorder = Recorder::default();
        let node = model_catalogue_editor(
            &spec().with_hidden_open(true),
            &RenderContext::new(&theme()),
            recorder.handlers(),
        );
        press(&node, "Restore Archive Delta");
        assert_eq!(
            recorder.visibility.lock().unwrap().as_slice(),
            [("model-hidden".to_string(), true)]
        );
        assert!(recorder.orders.lock().unwrap().is_empty());
        assert_eq!(
            recorder.announcements.lock().unwrap().as_slice(),
            ["Restored Archive Delta."]
        );
    }

    #[test]
    fn the_info_action_exists_only_when_the_host_wants_it() {
        let without = model_catalogue_editor(
            &spec(),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers::default(),
        );
        assert!(without
            .find(&|n| n.a11y.label.as_deref() == Some("About Frontier Alpha"))
            .is_none());

        let recorder = Recorder::default();
        let with =
            model_catalogue_editor(&spec(), &RenderContext::new(&theme()), recorder.handlers());
        press(&with, "About Frontier Alpha");
        assert_eq!(recorder.info.lock().unwrap().as_slice(), ["model-alpha"]);
    }

    #[test]
    fn locked_surfaces_stay_readable_and_take_no_edits() {
        for spec in [spec().with_disabled(true), spec().with_pending(true)] {
            let node = model_catalogue_editor(
                &spec,
                &RenderContext::new(&theme()),
                ModelCatalogueEditorHandlers {
                    on_order_change: Some(Arc::new(|_| unreachable!("a locked editor reorders"))),
                    on_visibility_change: Some(Arc::new(|_| unreachable!("a locked editor hides"))),
                    ..ModelCatalogueEditorHandlers::default()
                },
            );
            for aria in [
                "Frontier Alpha, position 1 of 4",
                "Move Frontier Alpha down",
                "Hide Frontier Alpha",
            ] {
                let control = named(&node, aria);
                assert!(control.interaction.disabled, "{aria} is locked");
                assert!(control.interaction.on_activate.is_none());
            }
            assert!(node.texts().contains(&"Frontier Alpha"));
        }
    }

    #[test]
    fn a_disabled_row_locks_only_itself() {
        let items = vec![
            ModelCatalogueItem::new("a", "Alpha").with_disabled(true),
            ModelCatalogueItem::new("b", "Beta"),
        ];
        let node = model_catalogue_editor(
            &ModelCatalogueEditorSpec::new().with_items(items),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers::default(),
        );
        assert!(named(&node, "Hide Alpha").interaction.disabled);
        assert!(!named(&node, "Hide Beta").interaction.disabled);
    }

    #[test]
    fn every_posture_renders_its_own_copy_and_no_rows() {
        let ready = model_catalogue_editor(
            &spec(),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers::default(),
        );
        assert!(ready.texts().contains(&"4 shown, 2 hidden"));

        for (state, title) in [
            (ModelCatalogueState::Loading, "Loading models"),
            (ModelCatalogueState::Unavailable, "Models unavailable"),
            (ModelCatalogueState::Empty, "No models"),
            (ModelCatalogueState::Error, "Could not load models"),
            (
                ModelCatalogueState::SessionNegotiated,
                "Models after session",
            ),
        ] {
            let node = model_catalogue_editor(
                &spec().with_state(state),
                &RenderContext::new(&theme()),
                ModelCatalogueEditorHandlers::default(),
            );
            assert!(
                node.texts().contains(&title),
                "{title} posture states itself"
            );
            assert!(
                node.find(&|n| n.a11y.role == Some(NodeRole::ListItem))
                    .is_none(),
                "{title} shows no stale rows"
            );
            assert!(
                !node.texts().iter().any(|text| text.contains("shown")),
                "{title} claims no counts"
            );
        }
    }

    #[test]
    fn the_host_may_override_a_posture_heading_and_message() {
        let node = model_catalogue_editor(
            &spec()
                .with_state(ModelCatalogueState::Unavailable)
                .with_state_title("No catalogue route")
                .with_state_message("This connection negotiates models per session."),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers::default(),
        );
        assert!(node.texts().contains(&"No catalogue route"));
        assert!(!node.texts().contains(&"Models unavailable"));
    }

    #[test]
    fn the_live_region_says_what_the_host_last_stored() {
        let node = model_catalogue_editor(
            &spec().with_live_message("Hid Frontier Beta."),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers::default(),
        );
        let live = node
            .find(&|n| n.a11y.role == Some(NodeRole::Status))
            .expect("live region");
        assert_eq!(live.a11y.label.as_deref(), Some("Hid Frontier Beta."));
    }

    #[test]
    fn rows_carry_position_badges_and_host_content_keyed_by_id() {
        let mut leading = BTreeMap::new();
        leading.insert("model-gamma".to_string(), Node::text("GAMMA MARK"));
        let mut row_meta = BTreeMap::new();
        row_meta.insert("model-gamma".to_string(), Node::text("128k context"));
        let node = model_catalogue_editor_with_slots(
            &spec(),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorSlots {
                leading,
                row_meta,
                custom_action: Some(Node::text("Refresh")),
            },
            ModelCatalogueEditorHandlers::default(),
        );

        assert!(node.texts().contains(&"GAMMA MARK"));
        assert!(node.texts().contains(&"128k context"));
        assert!(node.texts().contains(&"Refresh"));
        assert!(node.texts().contains(&"Default"));
        assert!(node
            .find(&|n| n.a11y.label.as_deref() == Some("Gateway Gamma, position 3 of 4"))
            .is_some());
    }

    #[test]
    fn a_shown_only_catalogue_renders_no_hidden_section() {
        let items = vec![
            ModelCatalogueItem::new("a", "Alpha"),
            ModelCatalogueItem::new("b", "Beta"),
        ];
        let node = model_catalogue_editor(
            &ModelCatalogueEditorSpec::new().with_items(items),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers::default(),
        );
        assert!(node.texts().contains(&"2 shown"));
        assert!(!node.texts().contains(&"Hidden models"));
        assert!(node
            .find(&|n| n.id.as_deref() == Some(MODEL_CATALOGUE_HIDDEN_SECTION_ID))
            .is_none());
    }

    #[test]
    fn the_hidden_section_id_sits_on_a_focusable_disclosure() {
        let recorder = Recorder::default();
        let node = model_catalogue_editor(
            &spec().with_hidden_open(true),
            &RenderContext::new(&theme()),
            recorder.handlers(),
        );
        let disclosure = node
            .find(&|n| n.id.as_deref() == Some(MODEL_CATALOGUE_HIDDEN_SECTION_ID))
            .expect("the hidden-section disclosure");
        assert!(
            disclosure.interaction.focusable,
            "the focus destination must be the focusable trigger, not the outer region"
        );
        assert!(
            disclosure.style.focus.is_some(),
            "the GPUI backend only tracks a focusable node that draws differently when focused"
        );
        assert!(disclosure.interaction.on_activate.is_some());
    }

    #[test]
    fn the_hidden_section_controls_and_labelled_by_agree_after_stamping() {
        let node = model_catalogue_editor(
            &spec().with_hidden_open(true),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers {
                instance_id: Some("editor".to_string()),
                ..ModelCatalogueEditorHandlers::default()
            },
        );
        let trigger_focus = model_catalogue_hidden_focus_id(Some("editor"));
        let content_focus = model_catalogue_hidden_content_focus_id(Some("editor"));
        let trigger = node
            .find(&|n| n.id.as_deref() == Some(MODEL_CATALOGUE_HIDDEN_SECTION_ID))
            .expect("hidden-section trigger");
        let content = node
            .find(&|n| {
                n.a11y.role == Some(NodeRole::Region)
                    && n.a11y.labelled_by.as_deref() == Some(trigger_focus.as_str())
            })
            .expect("hidden-section content");
        assert_eq!(trigger.runtime_id.as_deref(), Some(trigger_focus.as_str()));
        assert_eq!(trigger.a11y.controls.as_deref(), Some(content_focus.as_str()));
        assert_eq!(content.runtime_id.as_deref(), Some(content_focus.as_str()));
        assert_eq!(
            content.a11y.labelled_by.as_deref(),
            Some(trigger_focus.as_str())
        );
    }

    #[test]
    fn hidden_restore_controls_keep_their_scoped_identity() {
        let node = model_catalogue_editor(
            &spec().with_hidden_open(true),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers {
                instance_id: Some("editor".to_string()),
                ..ModelCatalogueEditorHandlers::default()
            },
        );
        let content_focus = model_catalogue_hidden_content_focus_id(Some("editor"));
        let trigger = node
            .find(&|n| n.id.as_deref() == Some(MODEL_CATALOGUE_HIDDEN_SECTION_ID))
            .expect("hidden-section trigger");
        assert_eq!(trigger.a11y.controls.as_deref(), Some(content_focus.as_str()));

        let restore = node
            .find(&|n| n.a11y.label.as_deref() == Some("Restore Archive Delta"))
            .expect("hidden restore control");
        assert_eq!(
            restore.id.as_deref(),
            Some("model-catalogue-editor:model-hidden:restore")
        );
        assert_eq!(
            restore.runtime_id.as_deref(),
            Some("model-catalogue-editor:editor:model-hidden:restore")
        );
        assert_ne!(restore.runtime_id, trigger.runtime_id);
        assert_ne!(restore.id.as_deref(), trigger.id.as_deref());
    }

    #[test]
    fn hiding_the_last_shown_model_names_a_destination_that_can_take_focus() {
        let recorder = Recorder::default();
        let only = ModelCatalogueEditorSpec::new().with_items(vec![
            ModelCatalogueItem::new("model-solo", "Solo"),
            ModelCatalogueItem::new("model-gone", "Gone").with_visible(false),
        ]);
        let node =
            model_catalogue_editor(&only, &RenderContext::new(&theme()), recorder.handlers());
        press(&node, "Hide Solo");

        let requested = recorder
            .focus
            .lock()
            .unwrap()
            .last()
            .cloned()
            .expect("a request");
        // The host discloses the section, so re-render and check the named id
        // resolves to something the backend can focus.
        let disclosed = model_catalogue_editor(
            &only.clone().with_hidden_open(true),
            &RenderContext::new(&theme()),
            recorder.handlers(),
        );
        let target = disclosed
            .find(&|n| n.id.as_deref() == Some(requested.as_str()))
            .expect("the requested id exists in the next render");
        assert!(target.interaction.focusable && target.style.focus.is_some());
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let first = model_catalogue_editor(
            &spec().with_hidden_open(true),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers {
                instance_id: Some("first".to_string()),
                ..ModelCatalogueEditorHandlers::default()
            },
        );
        let second = model_catalogue_editor(
            &spec().with_hidden_open(true),
            &RenderContext::new(&theme()),
            ModelCatalogueEditorHandlers {
                instance_id: Some("second".to_string()),
                ..ModelCatalogueEditorHandlers::default()
            },
        );

        for (node, scope) in [(&first, "first"), (&second, "second")] {
            assert!(node
                .find(&|n| n.runtime_id.as_deref()
                    == Some(model_catalogue_handle_focus_id(Some(scope), "model-alpha").as_str()))
                .is_some());
            assert!(node
                .find(&|n| n.runtime_id.as_deref()
                    == Some(model_catalogue_hidden_focus_id(Some(scope)).as_str()))
                .is_some());
        }
        assert!(first
            .find(&|n| n.runtime_id.as_deref()
                == Some(model_catalogue_handle_focus_id(Some("second"), "model-alpha").as_str()))
            .is_none());
        // The semantic id stays readable and unscoped in both.
        assert!(first
            .find(&|n| n.id.as_deref() == Some("model-catalogue-editor:model-alpha:handle"))
            .is_some());
    }

    #[test]
    fn a_scoped_editor_requests_scoped_focus_destinations() {
        let recorder = Recorder::default();
        let mut handlers = recorder.handlers();
        handlers.instance_id = Some("second".to_string());
        let node = model_catalogue_editor(&spec(), &RenderContext::new(&theme()), handlers);
        press(&node, "Move Frontier Alpha down");
        assert_eq!(
            recorder.focus.lock().unwrap().as_slice(),
            [model_catalogue_handle_focus_id(
                Some("second"),
                "model-alpha"
            )]
        );
    }

    #[test]
    fn duplicate_labels_stay_distinct_rows_keyed_by_id() {
        let recorder = Recorder::default();
        let node = model_catalogue_editor(
            &spec().with_hidden_open(true),
            &RenderContext::new(&theme()),
            recorder.handlers(),
        );
        // Two models share the label "Shared Label"; one is shown, one hidden.
        press(&node, "Hide Shared Label");
        press(&node, "Restore Shared Label");
        assert_eq!(
            recorder.visibility.lock().unwrap().as_slice(),
            [
                ("model-dup-a".to_string(), false),
                ("model-dup-b".to_string(), true),
            ]
        );
        assert_eq!(shown_ids().len(), 4);
    }
}
