//! ModelConnectionPicker — a searchable, grouped radio-card picker for one
//! exact configured model route.
//!
//! Contract: `docs/contracts/components/model-connection-picker.md`
//!
//! Filtering, grouping, selectability, shell-state resolution, result
//! announcements, and posture copy all derive once through
//! `poodle_headless::model_connection`, so this file only composes nodes and
//! attaches callbacks. Nothing here probes, ranks, resolves a provider, or
//! decides what a route is: the host orders `options`, classifies
//! availability, and receives the exact opaque id back.

use std::collections::BTreeMap;
use std::sync::Arc;

use poodle_headless::model_connection::{
    filter_model_connection_options, group_model_connection_options,
    model_connection_availability_label, model_connection_availability_tone,
    model_connection_option_selectable, model_connection_picker_result_announcement,
    model_connection_picker_state_copy, resolve_model_connection_picker_shell_state,
    ModelConnectionAvailability, ModelConnectionOption, ModelConnectionPickerShellState,
    ModelConnectionStatusTone,
};
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeKey, NodeRole, NodeToggled, StylePatch, TextChangeHandler,
};
use poodle_specs::{
    BrowseState, ModelConnectionPickerSpec, PickerShellSpec, SelectionMode, StatusIndicatorSpec,
    StatusTone, TextInputSpec,
};

use crate::context::RenderContext;
use crate::picker_shell::picker_shell;
use crate::presentation::rem_to_px;
use crate::status_indicator::status_indicator;
use crate::text_input::text_input_with_change;

/// Contract §8: the label weight the group heading and provider line share.
const LABEL_WEIGHT: u16 = 500;

/// Host callbacks. Both are requests: the host updates the spec and the next
/// render shows the result.
#[derive(Default)]
pub struct ModelConnectionPickerHandlers {
    /// An enabled, available option was chosen. The payload is the exact
    /// opaque option id — never a provider label.
    pub on_value_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// The search text changed.
    pub on_query_change: Option<TextChangeHandler>,
    /// Stable native instance scope. Semantic ids stay readable, but two
    /// pickers offering the same routes must never share backend focus
    /// handles — GPUI keys them in one global map by element id.
    pub instance_id: Option<String>,
}

/// Host-composed content. Provider marks are whatever the host supplies for an
/// option id; Poodle resolves no provider catalogue and ships no brand asset.
#[derive(Default)]
pub struct ModelConnectionPickerSlots {
    /// Leading mark per option id. A missing id draws the generic mark.
    pub leading: BTreeMap<String, Node>,
    /// Optional workflow guidance or actions below the results.
    pub footer: Option<Node>,
}

/// The semantic element id of one option row. Readable and stable across
/// instances; accessibility relationships use it.
pub fn model_connection_option_id(option_id: &str) -> String {
    format!("model-connection-option:{option_id}")
}

/// The backend-state id of one option row: the instance scope when the host
/// supplied one, else the semantic id. Roving focus and every focus request
/// name this, because it is what the backend keys focus handles by.
pub fn model_connection_option_focus_id(instance_id: Option<&str>, option_id: &str) -> String {
    match instance_id {
        Some(scope) => format!("model-connection-picker:{scope}:option:{option_id}"),
        None => model_connection_option_id(option_id),
    }
}

/// The backend-state id of the search field, scoped the same way: the field
/// holds host-owned editing state the backend caches by element id.
pub fn model_connection_picker_search_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("model-connection-picker:{scope}:search"),
        None => "model-connection-picker:search".to_string(),
    }
}

pub fn model_connection_picker(
    spec: &ModelConnectionPickerSpec,
    ctx: &RenderContext<'_>,
    handlers: ModelConnectionPickerHandlers,
) -> Node {
    model_connection_picker_with_slots(
        spec,
        ctx,
        ModelConnectionPickerSlots::default(),
        handlers,
    )
}

pub fn model_connection_picker_with_slots(
    spec: &ModelConnectionPickerSpec,
    ctx: &RenderContext<'_>,
    slots: ModelConnectionPickerSlots,
    handlers: ModelConnectionPickerHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    let filtered = filter_model_connection_options(&spec.options, &spec.query);
    let groups = group_model_connection_options(&filtered);
    let selectable: Vec<&ModelConnectionOption> = filtered
        .iter()
        .filter(|option| model_connection_option_selectable(option))
        .collect();
    let has_visible_selection = selectable
        .iter()
        .any(|option| Some(option.id.as_str()) == spec.value.as_deref());
    let roving_ids: Vec<String> = selectable
        .iter()
        .map(|option| option.id.clone())
        .collect();

    let shell_state = resolve_model_connection_picker_shell_state(
        spec.state,
        spec.options.len(),
        filtered.len(),
        &spec.query,
    );
    let state_copy = model_connection_picker_state_copy(shell_state, &spec.query);
    let status_text = matches!(
        shell_state,
        ModelConnectionPickerShellState::Ready | ModelConnectionPickerShellState::NoResults
    )
    .then(|| model_connection_picker_result_announcement(filtered.len(), &spec.query));

    // ── Toolbar: the search field ──
    let mut search_spec = TextInputSpec::new()
        .with_value(spec.query.clone())
        .with_type("search")
        .with_placeholder(spec.search_placeholder.clone())
        .with_aria_label(spec.search_placeholder.clone())
        .with_disabled(spec.is_disabled)
        .with_size(effective_size)
        .with_density(density);
    search_spec.id = Some(model_connection_picker_search_id(
        handlers.instance_id.as_deref(),
    ));
    let search = text_input_with_change(&search_spec, ctx, handlers.on_query_change.clone());

    // ── Body: the grouped radio cards ──
    let mut body = Node::container();
    {
        let s = &mut body.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.md");
        s.fill_width = true;
    }
    let mut body = body;
    for group in &groups {
        body = body.child(group_node(
            spec,
            ctx,
            group.group.as_str(),
            &group.options,
            &roving_ids,
            has_visible_selection,
            &slots,
            &handlers,
            effective_size,
        ));
    }

    let mut shell_spec = PickerShellSpec::new(spec.title.clone())
        .with_variant(spec.variant)
        .with_selection_mode(SelectionMode::Single)
        .with_state(browse_state(shell_state))
        .with_query(spec.query.clone())
        .with_selected_count(usize::from(spec.value.is_some()))
        .with_selection_count(usize::from(spec.value.is_some()))
        .with_state_title(state_copy.title.clone())
        .with_aria_label(spec.effective_aria_label().to_string());
    if shell_state == ModelConnectionPickerShellState::Ready {
        shell_spec = shell_spec.with_result_count(filtered.len());
    }
    if !state_copy.message.is_empty() {
        shell_spec = shell_spec.with_state_message(state_copy.message.clone());
    }
    if let Some(description) = spec.description.as_deref() {
        shell_spec = shell_spec.with_description(description.to_string());
    }
    if let Some(status_text) = status_text {
        shell_spec = shell_spec.with_status_text(status_text);
    }

    let mut root = picker_shell(
        &shell_spec,
        ctx,
        Some(search),
        None,
        Some(body),
        None,
        slots.footer,
    );
    root.roles
        .insert("disabled".to_string(), spec.is_disabled.to_string());
    root
}

fn browse_state(state: ModelConnectionPickerShellState) -> BrowseState {
    match state {
        ModelConnectionPickerShellState::Ready => BrowseState::Ready,
        ModelConnectionPickerShellState::Loading => BrowseState::Loading,
        ModelConnectionPickerShellState::Error => BrowseState::Error,
        ModelConnectionPickerShellState::Empty => BrowseState::Empty,
        ModelConnectionPickerShellState::NoResults => BrowseState::NoResults,
    }
}

fn status_tone(tone: ModelConnectionStatusTone) -> StatusTone {
    match tone {
        ModelConnectionStatusTone::Neutral => StatusTone::Neutral,
        ModelConnectionStatusTone::Info => StatusTone::Info,
        ModelConnectionStatusTone::Success => StatusTone::Success,
        ModelConnectionStatusTone::Warning => StatusTone::Warning,
        ModelConnectionStatusTone::Danger => StatusTone::Danger,
    }
}

fn availability_role(availability: ModelConnectionAvailability) -> &'static str {
    match availability {
        ModelConnectionAvailability::Available => "available",
        ModelConnectionAvailability::Checking => "checking",
        ModelConnectionAvailability::Unavailable => "unavailable",
        ModelConnectionAvailability::Unsupported => "unsupported",
    }
}

#[allow(clippy::too_many_arguments)]
fn group_node(
    spec: &ModelConnectionPickerSpec,
    ctx: &RenderContext<'_>,
    group_label: &str,
    options: &[ModelConnectionOption],
    roving_ids: &[String],
    has_visible_selection: bool,
    slots: &ModelConnectionPickerSlots,
    handlers: &ModelConnectionPickerHandlers,
    effective_size: poodle_specs::ControlSize,
) -> Node {
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");

    let mut title = Node::text(group_label);
    title.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
    title.style.text_weight = Some(LABEL_WEIGHT);
    title.style.descriptor.text_color = Some(text_secondary);

    // The radiogroup carries the group's name by object: natives label by
    // object rather than by DOM id.
    let mut radiogroup = Node::container();
    {
        let s = &mut radiogroup.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
        s.fill_width = true;
    }
    radiogroup.a11y.role = Some(NodeRole::RadioGroup);
    radiogroup.a11y.label = Some(group_label.to_string());
    let mut radiogroup = radiogroup;
    for option in options {
        radiogroup = radiogroup.child(option_node(
            spec,
            ctx,
            option,
            roving_ids,
            has_visible_selection,
            slots,
            handlers,
            effective_size,
        ));
    }

    let mut group = Node::container();
    {
        let s = &mut group.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
        s.fill_width = true;
    }
    group.a11y.role = Some(NodeRole::Group);
    group.a11y.label = Some(group_label.to_string());
    group.child(title).child(radiogroup)
}

/// The option's accessible name. The web tree composes it from the provider
/// line, the route line, the visually-hidden description, and the status
/// label; natives label by object, so the same content is stated once here.
fn option_accessible_name(option: &ModelConnectionOption) -> String {
    let mut parts = vec![option.provider_label.clone()];
    if let Some(route) = option.route_label.as_deref() {
        parts.push(route.to_string());
    }
    if let Some(description) = option.description.as_deref() {
        parts.push(description.to_string());
    }
    parts.push(option.availability_label.clone());
    parts.join(", ")
}

#[allow(clippy::too_many_arguments)]
fn option_node(
    spec: &ModelConnectionPickerSpec,
    ctx: &RenderContext<'_>,
    option: &ModelConnectionOption,
    roving_ids: &[String],
    has_visible_selection: bool,
    slots: &ModelConnectionPickerSlots,
    handlers: &ModelConnectionPickerHandlers,
    effective_size: poodle_specs::ControlSize,
) -> Node {
    let selectable = model_connection_option_selectable(option);
    let density = ctx.resolve_density(spec.density);
    let is_selected = spec.value.as_deref() == Some(option.id.as_str());
    let is_option_disabled = spec.is_disabled || !selectable;

    let accent = ctx.theme().resolve_color("color.accent.base");
    let focus_ring = ctx.theme().resolve_color("color.accent.focusRing");
    let border = ctx.theme().resolve_color("color.border.subtle");
    let surface = ctx.theme().resolve_color("color.background.surface");
    let panel = ctx.theme().resolve_color("color.background.panel");
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let text_inverse = ctx.theme().resolve_color("color.text.inverse");
    let label_size = ctx.theme().resolve_space("typography.label.size");
    let body_size = ctx.theme().resolve_space("typography.body.size");

    // ── Leading: the selected check replaces the mark; it never adds a
    // second trailing indicator. ──
    let leading = if is_selected {
        selected_mark(ctx, accent, text_inverse)
    } else {
        let mark = slots
            .leading
            .get(&option.id)
            .cloned()
            .unwrap_or_else(|| generic_mark(ctx, text_secondary));
        leading_lane(ctx, panel, mark)
    };

    // ── Copy: provider, route; the description stays in the accessible name ──
    let mut copy = Node::container();
    {
        let s = &mut copy.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }
    let mut provider = Node::text(&option.provider_label);
    provider.style.text_size = Some(body_size);
    provider.style.text_weight = Some(LABEL_WEIGHT);
    provider.style.descriptor.text_color = Some(text_primary);
    provider.style.text_ellipsis = true;
    let mut copy = copy.child(provider);
    if let Some(route) = option.route_label.as_deref() {
        let mut route_node = Node::text(route);
        route_node.style.text_size = Some(label_size);
        route_node.style.descriptor.text_color = Some(text_secondary);
        route_node.style.text_ellipsis = true;
        copy = copy.child(route_node);
    }

    // ── Availability: compact visible text, supplied reason in the name ──
    let availability = status_indicator(
        &StatusIndicatorSpec::new()
            .with_status(status_tone(model_connection_availability_tone(
                option.availability,
            )))
            .with_label(model_connection_availability_label(option.availability))
            .with_aria_label(option.availability_label.clone())
            .with_size(effective_size)
            .with_density(density),
        ctx,
    );

    let mut node = Node::container();
    {
        let s = &mut node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = ctx.theme().resolve_space("space.stack.sm");
        pad.bottom = ctx.theme().resolve_space("space.stack.sm");
        pad.left = ctx.theme().resolve_space("space.inline.md");
        pad.right = ctx.theme().resolve_space("space.inline.md");
        s.descriptor.border.width = rem_to_px(0.0625);
        s.descriptor.border.color = if is_selected { accent } else { border };
        s.descriptor.background = Some(surface);
        s.fill_width = true;
        s.min_width = Some(0.0);
        let c = &mut s.descriptor.corner_radii;
        let radius = ctx.theme().resolve_radius("radius.surface");
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.focus = Some(StylePatch {
            border_color: Some(focus_ring),
            ..StylePatch::default()
        });
        if is_option_disabled {
            s.descriptor.opacity = 0.72; // contract §8 disabled option opacity
        } else {
            s.descriptor.cursor = CursorHint::Pointer;
        }
    }
    node.id = Some(model_connection_option_id(&option.id));
    // The instance scope lives on `runtime_id`, which is what the backend keys
    // focus, editing and gesture state by; `id` stays the readable semantic
    // one so accessibility relationships do not carry a scope.
    node.runtime_id = handlers
        .instance_id
        .as_deref()
        .map(|scope| model_connection_option_focus_id(Some(scope), &option.id));
    node.a11y.role = Some(NodeRole::RadioButton);
    node.a11y.label = Some(option_accessible_name(option));
    node.a11y.toggled = Some(if is_selected {
        NodeToggled::True
    } else {
        NodeToggled::False
    });
    node.a11y.tab_index = Some(tab_index_for(
        option,
        spec,
        selectable,
        roving_ids,
        has_visible_selection,
    ));
    node.roles.insert(
        "availability".to_string(),
        availability_role(option.availability).to_string(),
    );
    node.roles
        .insert("selected".to_string(), is_selected.to_string());

    node.interaction.disabled = is_option_disabled;
    if !is_option_disabled {
        node.interaction.focusable = true;
        if let Some(handler) = &handlers.on_value_change {
            let handler = Arc::clone(handler);
            let id = option.id.clone();
            // Enter and Space reach this through the backend's own activation
            // path; `on_key` below only moves the roving stop.
            node.interaction.on_activate = Some(Arc::new(move || handler(&id)));
        }
        node.interaction.on_key = roving_key_handler(
            option,
            roving_ids,
            handlers.instance_id.clone(),
            handlers.on_value_change.clone(),
        );
    }

    node.child(leading).child(copy).child(availability)
}

/// Roving traversal: the selected option is the tab stop, else the first
/// enabled one. Everything else stays programmatically focusable.
fn tab_index_for(
    option: &ModelConnectionOption,
    spec: &ModelConnectionPickerSpec,
    selectable: bool,
    roving_ids: &[String],
    has_visible_selection: bool,
) -> i32 {
    if spec.is_disabled || !selectable {
        return -1;
    }
    if spec.value.as_deref() == Some(option.id.as_str()) {
        return 0;
    }
    if !has_visible_selection && roving_ids.first().map(String::as_str) == Some(option.id.as_str()) {
        return 0;
    }
    -1
}

/// Arrow/Home/End move through the enabled options of the complete picker,
/// select the option moved to, and name it as the focus destination. Space is
/// deliberately not handled here: the backend's activation path already owns
/// Enter and Space, and handling both would select twice.
fn roving_key_handler(
    option: &ModelConnectionOption,
    roving_ids: &[String],
    instance_id: Option<String>,
    on_value_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Option<Arc<dyn Fn(NodeKey, poodle_node::NodeModifiers) -> Option<String> + Send + Sync>> {
    let index = roving_ids.iter().position(|id| id == &option.id)?;
    let ids = roving_ids.to_vec();
    Some(Arc::new(move |key, _modifiers| {
        if ids.is_empty() {
            return None;
        }
        let last = ids.len() - 1;
        let next = match key {
            NodeKey::ArrowDown | NodeKey::ArrowRight => {
                Some(if index == last { 0 } else { index + 1 })
            }
            NodeKey::ArrowUp | NodeKey::ArrowLeft => Some(if index == 0 { last } else { index - 1 }),
            NodeKey::Home => Some(0),
            NodeKey::End => Some(last),
            _ => None,
        }?;
        let target = ids[next].clone();
        if let Some(handler) = &on_value_change {
            handler(&target);
        }
        Some(model_connection_option_focus_id(
            instance_id.as_deref(),
            &target,
        ))
    }))
}

fn leading_lane(ctx: &RenderContext<'_>, panel: ColorValue, mark: Node) -> Node {
    let size = rem_to_px(1.75); // contract §8 `size.icon.lg` lane
    let mut lane = Node::container();
    {
        let s = &mut lane.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(size);
        s.descriptor.layout.height = LayoutSizing::Fixed(size);
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.background = Some(panel);
        s.flex_none = true;
        let c = &mut s.descriptor.corner_radii;
        let radius = ctx.theme().resolve_radius("radius.control");
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
    }
    lane.child(mark)
}

/// The specimen fallback mark. It is not a provider catalogue: a host that
/// wants a brand mark supplies one for that option id.
fn generic_mark(_ctx: &RenderContext<'_>, tint: ColorValue) -> Node {
    let mut icon = Node::icon("package", rem_to_px(1.0));
    icon.style.descriptor.text_color = Some(tint);
    icon
}

fn selected_mark(ctx: &RenderContext<'_>, accent: ColorValue, tint: ColorValue) -> Node {
    let size = rem_to_px(1.25); // contract §8 `size.icon.md`
    let mut lane = Node::container();
    {
        let s = &mut lane.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(size);
        s.descriptor.layout.height = LayoutSizing::Fixed(size);
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.background = Some(accent);
        s.flex_none = true;
        let c = &mut s.descriptor.corner_radii;
        let radius = ctx.theme().resolve_radius("radius.pill");
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
    }
    let mut check = Node::icon("check", rem_to_px(0.875));
    check.style.descriptor.text_color = Some(tint);
    lane.child(check)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::model_connection::model_connection_picker_fixtures;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn spec() -> ModelConnectionPickerSpec {
        ModelConnectionPickerSpec::new().with_options(model_connection_picker_fixtures())
    }

    fn options_in_order(node: &Node) -> Vec<String> {
        let mut found = Vec::new();
        collect_options(node, &mut found);
        found
    }

    fn collect_options(node: &Node, out: &mut Vec<String>) {
        if node.a11y.role == Some(NodeRole::RadioButton) {
            if let Some(id) = &node.id {
                out.push(id.clone());
            }
        }
        for child in &node.children {
            collect_options(child, out);
        }
    }

    fn group_labels(node: &Node) -> Vec<String> {
        let mut out = Vec::new();
        fn walk(node: &Node, out: &mut Vec<String>) {
            if node.a11y.role == Some(NodeRole::RadioGroup) {
                if let Some(label) = &node.a11y.label {
                    out.push(label.clone());
                }
            }
            for child in &node.children {
                walk(child, out);
            }
        }
        walk(node, &mut out);
        out
    }

    fn option_node<'a>(node: &'a Node, option_id: &str) -> &'a Node {
        node.find(&|n| n.id.as_deref() == Some(model_connection_option_id(option_id).as_str()))
            .expect("option row")
    }

    #[test]
    fn filtering_preserves_source_order_and_group_order() {
        let node = model_connection_picker(
            &spec().with_query("local"),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers::default(),
        );
        assert_eq!(
            options_in_order(&node),
            [
                model_connection_option_id("codex-app"),
                model_connection_option_id("ollama-local"),
                model_connection_option_id("lmstudio-local"),
            ]
        );
        assert_eq!(group_labels(&node), ["Installed", "Local runtime"]);
    }

    #[test]
    fn options_carry_radio_semantics_and_the_supplied_reason() {
        let node = model_connection_picker(
            &spec().with_value(Some("openai-responses".to_string())),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers::default(),
        );

        let selected = option_node(&node, "openai-responses");
        assert_eq!(selected.a11y.role, Some(NodeRole::RadioButton));
        assert_eq!(selected.a11y.toggled, Some(NodeToggled::True));
        assert_eq!(selected.a11y.tab_index, Some(0));

        let other = option_node(&node, "anthropic-messages");
        assert_eq!(other.a11y.toggled, Some(NodeToggled::False));
        assert_eq!(other.a11y.tab_index, Some(-1));

        // The full supplied reason reaches the accessible name; the visible
        // status text stays compact.
        let unsupported = option_node(&node, "vendor-legacy");
        assert!(unsupported
            .a11y
            .label
            .as_deref()
            .expect("name")
            .contains("Unsupported on this platform"));
        assert!(unsupported
            .texts().contains(&"Unsupported"));
        assert!(!unsupported
            .texts().contains(&"Unsupported on this platform"));
    }

    #[test]
    fn selection_replaces_the_leading_mark_without_a_second_indicator() {
        let unselected = model_connection_picker(
            &spec(),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers::default(),
        );
        let row = option_node(&unselected, "openai-responses");
        assert!(row
            .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "package"))
            .is_some());
        assert!(row
            .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "check"))
            .is_none());

        let selected = model_connection_picker(
            &spec().with_value(Some("openai-responses".to_string())),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers::default(),
        );
        let row = option_node(&selected, "openai-responses");
        let checks = {
            let mut count = 0usize;
            fn walk(node: &Node, count: &mut usize) {
                if matches!(&node.kind, poodle_node::NodeKind::Icon { name, .. } if name == "check")
                {
                    *count += 1;
                }
                for child in &node.children {
                    walk(child, count);
                }
            }
            walk(row, &mut count);
            count
        };
        assert_eq!(checks, 1, "exactly one selected indicator, in the mark lane");
        assert!(row
            .find(&|n| matches!(&n.kind, poodle_node::NodeKind::Icon { name, .. } if name == "package"))
            .is_none());
    }

    #[test]
    fn disabled_and_unavailable_options_cannot_be_selected() {
        let chosen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&chosen);
        let node = model_connection_picker(
            &spec(),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers {
                on_value_change: Some(Arc::new(move |id: &str| {
                    sink.lock().unwrap().push(id.to_string())
                })),
                ..ModelConnectionPickerHandlers::default()
            },
        );

        for id in ["codex-app", "lmstudio-local", "vendor-legacy"] {
            let row = option_node(&node, id);
            assert!(row.interaction.disabled, "{id} is inert");
            assert!(row.interaction.on_activate.is_none(), "{id} has no activation");
            assert!(!row.interaction.focusable, "{id} is out of focus traversal");
            assert_eq!(row.a11y.tab_index, Some(-1));
        }

        let row = option_node(&node, "ollama-local");
        (row.interaction.on_activate.as_ref().expect("activation"))();
        assert_eq!(chosen.lock().unwrap().as_slice(), ["ollama-local"]);
    }

    #[test]
    fn roving_keys_move_through_enabled_options_and_wrap() {
        let chosen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&chosen);
        let node = model_connection_picker(
            &spec(),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers {
                on_value_change: Some(Arc::new(move |id: &str| {
                    sink.lock().unwrap().push(id.to_string())
                })),
                ..ModelConnectionPickerHandlers::default()
            },
        );

        let first = option_node(&node, "openai-responses");
        let keys = first.interaction.on_key.as_ref().expect("roving handler");
        let modifiers = poodle_node::NodeModifiers::default();

        assert_eq!(
            keys(NodeKey::ArrowDown, modifiers),
            Some(model_connection_option_id("openai-completions"))
        );
        assert_eq!(
            keys(NodeKey::ArrowUp, modifiers),
            Some(model_connection_option_id("ollama-local")),
            "wrapping skips the disabled and unavailable routes"
        );
        assert_eq!(
            keys(NodeKey::End, modifiers),
            Some(model_connection_option_id("ollama-local"))
        );
        assert_eq!(
            keys(NodeKey::Home, modifiers),
            Some(model_connection_option_id("openai-responses"))
        );
        // Space is the backend's activation path, not a roving move.
        assert_eq!(keys(NodeKey::Space, modifiers), None);

        assert_eq!(
            chosen.lock().unwrap().as_slice(),
            [
                "openai-completions",
                "ollama-local",
                "ollama-local",
                "openai-responses"
            ],
            "moving selects the option moved to"
        );
    }

    #[test]
    fn query_changes_reach_the_host_and_announce_the_result_count() {
        let typed = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&typed);
        let node = model_connection_picker(
            &spec().with_query("local"),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers {
                on_query_change: Some(Arc::new(move |next: &str| {
                    sink.lock().unwrap().push(next.to_string())
                })),
                ..ModelConnectionPickerHandlers::default()
            },
        );

        let field = node
            .find(&|n| n.interaction.on_text_change.is_some())
            .expect("the search field");
        (field.interaction.on_text_change.as_ref().unwrap())("ollama");
        assert_eq!(typed.lock().unwrap().as_slice(), ["ollama"]);

        assert!(node
            .texts()
            .iter()
            .any(|text| text.contains("3 connections match")));
    }

    #[test]
    fn every_posture_renders_its_own_copy_and_no_choices() {
        use poodle_headless::model_connection::ModelConnectionPickerState;

        let loading = model_connection_picker(
            &spec().with_state(ModelConnectionPickerState::Loading),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers::default(),
        );
        assert!(loading
            .texts().contains(&"Loading connections"));
        assert!(options_in_order(&loading).is_empty());

        let error = model_connection_picker(
            &spec().with_state(ModelConnectionPickerState::Error),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers::default(),
        );
        assert!(error
            .texts().contains(&"Could not load connections"));
        assert!(options_in_order(&error).is_empty());

        let empty = model_connection_picker(
            &ModelConnectionPickerSpec::new(),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers::default(),
        );
        assert!(empty
            .texts().contains(&"No connections available"));

        let no_results = model_connection_picker(
            &spec().with_query("zzz"),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers::default(),
        );
        assert!(no_results
            .texts().contains(&"No matching connections"));
        assert!(options_in_order(&no_results).is_empty());
    }

    #[test]
    fn host_leading_content_is_keyed_by_option_id() {
        let mut leading = BTreeMap::new();
        leading.insert("ollama-local".to_string(), Node::text("OLLAMA MARK"));
        let node = model_connection_picker_with_slots(
            &spec(),
            &RenderContext::new(&theme()),
            ModelConnectionPickerSlots {
                leading,
                footer: Some(Node::text("Host footer")),
            },
            ModelConnectionPickerHandlers::default(),
        );

        let row = option_node(&node, "ollama-local");
        assert!(row.texts().contains(&"OLLAMA MARK"));
        let other = option_node(&node, "openai-responses");
        assert!(!other.texts().contains(&"OLLAMA MARK"));
        assert!(node.texts().contains(&"Host footer"));
    }

    #[test]
    fn an_instance_scope_isolates_backend_state_ids() {
        let scoped = |scope: &str| ModelConnectionPickerHandlers {
            instance_id: Some(scope.to_string()),
            ..ModelConnectionPickerHandlers::default()
        };
        let first = model_connection_picker(&spec(), &RenderContext::new(&theme()), scoped("first"));
        let second = model_connection_picker(&spec(), &RenderContext::new(&theme()), scoped("second"));

        for (node, scope) in [(&first, "first"), (&second, "second")] {
            assert!(node
                .find(&|n| n.runtime_id.as_deref()
                    == Some(
                        model_connection_option_focus_id(Some(scope), "openai-responses").as_str()
                    ))
                .is_some());
            // `text_input` prefixes the spec id when it derives the field's
            // own backend-state id, which is what keeps two search fields from
            // sharing one caret.
            let field = format!(
                "poodle-input-{}",
                model_connection_picker_search_id(Some(scope))
            );
            assert!(node.find(&|n| n.id.as_deref() == Some(field.as_str())).is_some());
        }
        assert!(first
            .find(&|n| n.runtime_id.as_deref()
                == Some(
                    model_connection_option_focus_id(Some("second"), "openai-responses").as_str()
                ))
            .is_none());
        // The semantic id stays readable and unscoped in both.
        assert!(first
            .find(&|n| n.id.as_deref() == Some(model_connection_option_id("openai-responses").as_str()))
            .is_some());
    }

    #[test]
    fn a_scoped_picker_roves_to_scoped_destinations() {
        let node = model_connection_picker(
            &spec(),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers {
                instance_id: Some("second".to_string()),
                ..ModelConnectionPickerHandlers::default()
            },
        );
        let first = option_node(&node, "openai-responses");
        let keys = first.interaction.on_key.as_ref().expect("roving handler");
        assert_eq!(
            keys(NodeKey::ArrowDown, poodle_node::NodeModifiers::default()),
            Some(model_connection_option_focus_id(
                Some("second"),
                "openai-completions"
            ))
        );
    }

    #[test]
    fn a_disabled_picker_keeps_its_selection_and_takes_no_input() {
        let node = model_connection_picker(
            &spec()
                .with_value(Some("openai-responses".to_string()))
                .with_disabled(true),
            &RenderContext::new(&theme()),
            ModelConnectionPickerHandlers {
                on_value_change: Some(Arc::new(|_| unreachable!("a disabled picker selects"))),
                ..ModelConnectionPickerHandlers::default()
            },
        );
        let row = option_node(&node, "openai-responses");
        assert_eq!(row.a11y.toggled, Some(NodeToggled::True));
        assert!(row.interaction.on_activate.is_none());
        assert_eq!(row.a11y.tab_index, Some(-1));
    }
}
