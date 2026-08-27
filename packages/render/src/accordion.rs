//! Accordion — a stack of disclosure items.
//!
//! Contract: `docs/contracts/components/accordion.md`
//! Selection always flows through `toggle_group_transition`. The callback
//! receives the owned resulting `AccordionSelectionValue`, not the activated item.

use std::sync::Arc;

use poodle_headless::single_select::SelectOption;
use poodle_headless::toggle_group::{
    toggle_group_transition, SelectionMode, ToggleGroupContext, ToggleGroupEffect,
    ToggleGroupEvent, ToggleGroupValue,
};
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, Node, NodeRole,
    ShadowLayer,
};
use poodle_specs::{
    AccordionItemSpec, AccordionSelectionMode, AccordionSelectionValue, AccordionSpec,
    ControlDensity, ControlSize,
};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::presentation::{rem_to_px, size_font_rem};

/// Host-owned native interaction for one Accordion instance.
///
/// `instance_id` is the lifetime-stable scope. It is construction data, not a
/// semantic item value, and the renderer never invents one from render order.
#[derive(Clone)]
pub struct AccordionHandlers {
    pub instance_id: String,
    pub on_value_change: Option<Arc<dyn Fn(AccordionSelectionValue) + Send + Sync>>,
}

impl AccordionHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "AccordionHandlers requires a non-empty lifetime-stable instance_id"
        );
        Self {
            instance_id,
            on_value_change: None,
        }
    }

    pub fn on_value_change(
        mut self,
        handler: Arc<dyn Fn(AccordionSelectionValue) + Send + Sync>,
    ) -> Self {
        self.on_value_change = Some(handler);
        self
    }
}

fn scoped(instance_id: &str, part: &str, item_value: &str) -> String {
    format!("accordion:{instance_id}:{part}:{item_value}")
}

/// The backend-state id of one accordion trigger for a scoped instance.
pub fn accordion_trigger_focus_id(instance_id: &str, item_value: &str) -> String {
    scoped(instance_id, "trigger", item_value)
}

/// The backend-state id of one open accordion panel for a scoped instance.
pub fn accordion_panel_focus_id(instance_id: &str, item_value: &str) -> String {
    scoped(instance_id, "panel", item_value)
}

fn to_toggle_value(value: &AccordionSelectionValue) -> ToggleGroupValue {
    match value {
        AccordionSelectionValue::Single(value) => ToggleGroupValue::Single(value.clone()),
        AccordionSelectionValue::Multiple(values) => ToggleGroupValue::Multiple(values.clone()),
    }
}

fn to_accordion_value(value: ToggleGroupValue) -> AccordionSelectionValue {
    match value {
        ToggleGroupValue::Single(value) => AccordionSelectionValue::Single(value),
        ToggleGroupValue::Multiple(values) => AccordionSelectionValue::Multiple(values),
    }
}

fn headless_context(spec: &AccordionSpec) -> ToggleGroupContext {
    let selection_mode = match spec.selection_mode {
        AccordionSelectionMode::Single => SelectionMode::Single,
        AccordionSelectionMode::Multiple => SelectionMode::Multiple,
    };
    let value = match spec.current_value() {
        Some(value) => to_toggle_value(value),
        None => match selection_mode {
            SelectionMode::Single => ToggleGroupValue::Single(None),
            SelectionMode::Multiple => ToggleGroupValue::Multiple(vec![]),
        },
    };
    ToggleGroupContext {
        value,
        options: spec
            .items
            .iter()
            .map(|item| SelectOption {
                value: item.value.clone(),
                disabled: item.is_disabled,
            })
            .collect(),
        selection_mode,
        allow_deactivation: spec.is_collapsible,
        disabled: false,
    }
}

fn item_open(context: &ToggleGroupContext, item_value: &str) -> bool {
    match &context.value {
        ToggleGroupValue::Multiple(values) => values.iter().any(|value| value == item_value),
        ToggleGroupValue::Single(Some(value)) => value == item_value,
        ToggleGroupValue::Single(None) => false,
    }
}

fn emit_toggle(
    context: &ToggleGroupContext,
    item_value: &str,
    on_value_change: &Option<Arc<dyn Fn(AccordionSelectionValue) + Send + Sync>>,
) {
    let Some(handler) = on_value_change else {
        return;
    };
    let (_, effects) = toggle_group_transition(
        context.clone(),
        ToggleGroupEvent::Toggle {
            value: item_value.to_string(),
        },
    );
    for effect in effects {
        let ToggleGroupEffect::EmitValueChange { value } = effect;
        handler(to_accordion_value(value));
    }
}

pub fn accordion(
    spec: &AccordionSpec,
    ctx: &RenderContext<'_>,
    handlers: AccordionHandlers,
) -> Node {
    accordion_with_content(spec, ctx, &[], handlers)
}

/// Render with per-item content keyed by accordion item value.
pub fn accordion_with_content(
    spec: &AccordionSpec,
    ctx: &RenderContext<'_>,
    content: &[(String, Node)],
    handlers: AccordionHandlers,
) -> Node {
    assert!(
        !handlers.instance_id.is_empty(),
        "accordion requires a non-empty AccordionHandlers instance_id"
    );
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let root_gap = ctx.theme().resolve_space(spec.root_gap_token());
    let context = headless_context(spec);
    let is_multiple = context.selection_mode == SelectionMode::Multiple;
    let instance_scope = handlers.instance_id.as_str();
    let focus_ring = FocusRing {
        color: ctx.theme().resolve_color(spec.focus_ring_color_token()),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset: rem_to_px(0.125),
    };

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
        s.min_width = Some(0.0);
        s.self_stretch = true;
    }

    for item in &spec.items {
        let is_expanded = item_open(&context, &item.value);
        root = root.child(render_item(
            spec,
            item,
            is_expanded,
            effective_size,
            density,
            ctx,
            content
                .iter()
                .find(|(value, _)| value == &item.value)
                .map(|(_, node)| node),
            instance_scope,
            &context,
            &handlers.on_value_change,
            focus_ring,
        ));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            root.a11y.label = Some(label.to_string());
        }
    }
    if is_multiple {
        root.a11y.role = Some(NodeRole::Group);
    }
    root
}

#[expect(
    clippy::too_many_arguments,
    reason = "item rendering keeps shared visual metrics explicit"
)]
fn render_item(
    spec: &AccordionSpec,
    item: &AccordionItemSpec,
    is_expanded: bool,
    effective_size: ControlSize,
    density: ControlDensity,
    ctx: &RenderContext<'_>,
    content: Option<&Node>,
    instance_scope: &str,
    context: &ToggleGroupContext,
    on_value_change: &Option<Arc<dyn Fn(AccordionSelectionValue) + Send + Sync>>,
    focus_ring: FocusRing,
) -> Node {
    let border_subtle = ctx.theme().resolve_color("color.border.subtle");
    let elevated = ctx.theme().resolve_color(spec.item_bg_elevated_token());
    let panel = ctx.theme().resolve_color(spec.item_bg_panel_token());
    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let radius = ctx.theme().resolve_radius("radius.surface");

    let item_border = with_alpha(border_subtle, border_subtle.3 * spec.border_subtle_alpha());
    let item_bg = mix_srgb(elevated, panel, spec.item_bg_elevated_ratio());

    let pad_x = rem_to_px(spec.inline_padding_rem(density));
    let pad_y = rem_to_px(spec.block_padding_rem());
    let item_gap = rem_to_px(spec.item_internal_gap_rem());
    let summary_gap = ctx.theme().resolve_space(spec.summary_gap_token());
    let trigger_gap = ctx.theme().resolve_space(spec.trigger_grid_gap_token());

    let title_font_size = rem_to_px(match effective_size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.0625,
        ControlSize::Xl => 1.125,
    });
    let description_font_size = rem_to_px(size_font_rem(effective_size));
    let indicator_size = rem_to_px(0.75);

    let trigger_id = accordion_trigger_focus_id(instance_scope, &item.value);
    let panel_id = accordion_panel_focus_id(instance_scope, &item.value);

    // Summary: title over optional description, always visible.
    let mut summary = Node::container();
    {
        let s = &mut summary.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = summary_gap;
        s.min_width = Some(0.0);
        s.flex_grow = Some(1.0);
        s.flex_basis = Some(0.0);
    }
    let mut title = Node::text(&item.label);
    title.style.descriptor.text_color = Some(text_primary);
    title.style.text_size = Some(title_font_size);
    title.style.text_weight = Some(700);
    title.style.line_height = Some(1.2);
    summary = summary.child(title);
    if let Some(ref desc) = item.description {
        let mut d = Node::text(desc);
        d.style.descriptor.text_color = Some(text_secondary);
        d.style.text_size = Some(description_font_size);
        d.style.line_height = Some(1.45);
        summary = summary.child(d);
    }

    let chevron_icon = if is_expanded {
        "chevron-up"
    } else {
        "chevron-down"
    };
    let mut indicator = Node::icon(chevron_icon, indicator_size);
    indicator.style.flex_shrink_zero = true;
    indicator.style.descriptor.text_color = Some(text_secondary);

    let mut trigger = Node::button("");
    trigger.id = Some(trigger_id.clone());
    trigger.runtime_id = Some(trigger_id.clone());
    trigger.a11y.role = Some(NodeRole::Button);
    trigger.a11y.label = Some(item.label.clone());
    trigger.a11y.expanded = Some(is_expanded);
    trigger.a11y.controls = Some(panel_id.clone());
    {
        let s = &mut trigger.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.self_stretch = true;
        s.descriptor.layout.spacing.gap = trigger_gap;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.background = Some(crate::color::TRANSPARENT);
        s.descriptor.border.width = 0.0;
        s.fill_width = true;
    }
    trigger = trigger.child(summary).child(indicator);

    if item.is_disabled {
        trigger.style.descriptor.cursor = CursorHint::NotAllowed;
        trigger.interaction.disabled = true;
        trigger.interaction.focusable = false;
        trigger.a11y.tab_index = Some(-1);
        trigger.interaction.on_activate = None;
        trigger.style.focus_ring = None;
    } else {
        trigger.style.descriptor.cursor = CursorHint::Pointer;
        trigger.interaction.focusable = true;
        trigger.a11y.tab_index = Some(0);
        trigger.style.focus_ring = Some(focus_ring);
        if on_value_change.is_some() {
            let context = context.clone();
            let on_value_change = on_value_change.clone();
            let value = item.value.clone();
            trigger.interaction.on_activate = Some(Arc::new(move || {
                emit_toggle(&context, &value, &on_value_change);
            }));
        }
    }

    let text_inverse = ctx.theme().resolve_color("color.text.inverse");
    let highlight = ColorValue(text_inverse.0, text_inverse.1, text_inverse.2, 0.08);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.self_stretch = true;
        s.descriptor.layout.spacing.gap = item_gap;
        s.descriptor.layout.spacing.padding.left = pad_x;
        s.descriptor.layout.spacing.padding.right = pad_x;
        s.descriptor.layout.spacing.padding.top = pad_y;
        s.descriptor.layout.spacing.padding.bottom = pad_y;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.background = Some(item_bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = item_border;
        s.shadow_layers = vec![ShadowLayer {
            offset_x: 0.0,
            offset_y: rem_to_px(0.0625),
            blur: 0.0,
            spread: 0.0,
            color: highlight,
            inset: true,
        }];
    }
    el = el.child(trigger);

    if is_expanded {
        let mut panel_node = Node::container();
        {
            let s = &mut panel_node.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.min_width = Some(0.0);
            s.self_stretch = true;
        }
        panel_node.id = Some(panel_id.clone());
        panel_node.runtime_id = Some(panel_id);
        panel_node.a11y.role = Some(NodeRole::Region);
        panel_node.a11y.labelled_by = Some(trigger_id);
        if let Some(content) = content {
            panel_node = panel_node.child(content.clone());
        }
        el = el.child(panel_node);
    }

    if item.is_disabled {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::AccordionItemSpec;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn items() -> Vec<AccordionItemSpec> {
        vec![
            AccordionItemSpec::new("first", "First"),
            AccordionItemSpec::new("second", "Second"),
            AccordionItemSpec::new("locked", "Locked").with_disabled(true),
        ]
    }

    fn render_with(spec: &AccordionSpec, handlers: AccordionHandlers) -> Node {
        accordion_with_content(spec, &RenderContext::new(&theme()), &[], handlers)
    }

    fn trigger<'a>(root: &'a Node, scope: &str, value: &str) -> &'a Node {
        root.find(&|node| {
            node.runtime_id.as_deref() == Some(accordion_trigger_focus_id(scope, value).as_str())
        })
        .unwrap_or_else(|| panic!("trigger {value}"))
    }

    #[test]
    #[should_panic(expected = "AccordionHandlers requires a non-empty lifetime-stable instance_id")]
    fn empty_instance_scope_is_rejected() {
        let _ = AccordionHandlers::new("");
    }

    #[test]
    fn single_mode_omits_root_group_and_reports_resulting_selection() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<AccordionSelectionValue>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = AccordionSpec::new(items())
            .with_value(AccordionSelectionValue::Single(Some("first".into())));
        let node = render_with(
            &spec,
            AccordionHandlers::new("faq").on_value_change(Arc::new(move |value| {
                sink.lock().unwrap().push(value);
            })),
        );

        assert!(node.a11y.role.is_none());
        let first = trigger(&node, "faq", "first");
        assert_eq!(first.a11y.role, Some(NodeRole::Button));
        assert_eq!(first.a11y.expanded, Some(true));
        assert_eq!(first.a11y.tab_index, Some(0));
        assert!(first.style.focus_ring.is_some());
        (first.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [AccordionSelectionValue::Single(None)]
        );
    }

    #[test]
    fn multiple_mode_exposes_group_role_and_complete_set_results() {
        use std::sync::Mutex;
        let seen: Arc<Mutex<Vec<AccordionSelectionValue>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = AccordionSpec::new(items())
            .with_selection_mode(AccordionSelectionMode::Multiple)
            .with_value(AccordionSelectionValue::Multiple(vec!["first".into()]));
        let node = render_with(
            &spec,
            AccordionHandlers::new("docs").on_value_change(Arc::new(move |value| {
                sink.lock().unwrap().push(value);
            })),
        );

        assert_eq!(node.a11y.role, Some(NodeRole::Group));
        (trigger(&node, "docs", "second")
            .interaction
            .on_activate
            .as_ref()
            .unwrap())();
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [AccordionSelectionValue::Multiple(vec![
                "first".into(),
                "second".into()
            ])]
        );
    }

    #[test]
    fn disabled_items_do_not_activate_or_focus() {
        let spec = AccordionSpec::new(items())
            .with_value(AccordionSelectionValue::Single(Some("first".into())));
        let node = render_with(
            &spec,
            AccordionHandlers::new("faq")
                .on_value_change(Arc::new(|_| panic!("disabled items never fire"))),
        );
        let locked = trigger(&node, "faq", "locked");
        assert!(locked.interaction.disabled);
        assert!(!locked.interaction.focusable);
        assert_eq!(locked.a11y.tab_index, Some(-1));
        assert!(locked.interaction.on_activate.is_none());
        assert!(locked.style.focus_ring.is_none());
    }

    #[test]
    fn instance_scope_keeps_trigger_and_panel_identity_independent() {
        let spec = AccordionSpec::new(vec![
            AccordionItemSpec::new("shared", "Shared"),
            AccordionItemSpec::new("other", "Other"),
        ])
        .with_value(AccordionSelectionValue::Single(Some("shared".into())));
        let left = render_with(&spec, AccordionHandlers::new("left"));
        let right = render_with(&spec, AccordionHandlers::new("right"));
        let left_trigger = trigger(&left, "left", "shared");
        let right_trigger = trigger(&right, "right", "shared");
        assert_eq!(
            left_trigger.runtime_id.as_deref(),
            Some("accordion:left:trigger:shared")
        );
        assert_eq!(
            right_trigger.runtime_id.as_deref(),
            Some("accordion:right:trigger:shared")
        );
        assert_ne!(left_trigger.runtime_id, right_trigger.runtime_id);

        let left_panel = left
            .find(&|node| node.a11y.role == Some(NodeRole::Region))
            .expect("open panel");
        assert_eq!(
            left_panel.a11y.labelled_by.as_deref(),
            Some("accordion:left:trigger:shared")
        );
        assert_eq!(
            left_trigger.a11y.controls.as_deref(),
            Some("accordion:left:panel:shared")
        );
    }

    #[test]
    fn keyed_content_reaches_the_expanded_item() {
        let spec = AccordionSpec::new(vec![
            AccordionItemSpec::new("first", "First"),
            AccordionItemSpec::new("second", "Second"),
        ])
        .with_value(AccordionSelectionValue::Single(Some("first".into())));
        let content = vec![
            ("first".to_string(), Node::text("Expanded body")),
            ("second".to_string(), Node::text("Hidden body")),
        ];
        let node = accordion_with_content(
            &spec,
            &RenderContext::new(&theme()),
            &content,
            AccordionHandlers::new("content"),
        );

        assert!(node.has_text("Expanded body"));
        assert!(!node.has_text("Hidden body"));
    }
}
