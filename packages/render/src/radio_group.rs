//! RadioGroup — one choice from a list.
//!
//! Contract: `docs/contracts/components/radio-group.md`
//! Ported from: `packages/jetstream/components/src/radio_group.rs`.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeKey, NodeRole, NodeToggled,
};
use poodle_specs::{ControlDensity, ControlSize, Orientation, RadioGroupSpec};

use crate::color::hex_color;
use crate::context::RenderContext;
use crate::presentation::{rem_to_px, size_font_rem};

/// Host-owned native interaction for one RadioGroup instance.
///
/// `instance_id` is the lifetime-stable scope. It is not the web form `name`,
/// and the renderer never invents one from render order or option values.
#[derive(Clone)]
pub struct RadioGroupHandlers {
    pub instance_id: String,
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl RadioGroupHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            on_change: None,
        }
    }

    pub fn on_change(mut self, handler: Arc<dyn Fn(&str) + Send + Sync>) -> Self {
        self.on_change = Some(handler);
        self
    }
}

/// Indicator (outer circle) size — contract §8 table over the icon-md token.
fn indicator_size_px(size: ControlSize, icon_md_px: f32) -> f32 {
    match size {
        ControlSize::Xs => icon_md_px - rem_to_px(0.125),
        ControlSize::Sm => icon_md_px,
        ControlSize::Md => rem_to_px(1.125),
        ControlSize::Lg => icon_md_px + rem_to_px(0.375),
        ControlSize::Xl => icon_md_px + rem_to_px(0.625),
    }
}

/// Dot (inner circle) size — contract §8 ratios over icon-md; md explicit.
fn dot_size_px(size: ControlSize, icon_md_px: f32) -> f32 {
    match size {
        ControlSize::Xs => icon_md_px * 0.40,
        ControlSize::Sm => icon_md_px * 0.45,
        ControlSize::Md => rem_to_px(0.5),
        ControlSize::Lg => icon_md_px * 0.55,
        ControlSize::Xl => icon_md_px * 0.60,
    }
}

fn circle(node: &mut Node, diameter: f32) {
    let s = &mut node.style;
    s.descriptor.layout.width = LayoutSizing::Fixed(diameter);
    s.descriptor.layout.height = LayoutSizing::Fixed(diameter);
    let r = diameter * 0.5;
    s.descriptor.corner_radii.top_left = r;
    s.descriptor.corner_radii.top_right = r;
    s.descriptor.corner_radii.bottom_right = r;
    s.descriptor.corner_radii.bottom_left = r;
}

fn option_id(value: &str) -> String {
    format!("radio:{value}")
}

fn option_focus_id(instance_scope: &str, value: &str) -> String {
    format!("radio:{instance_scope}:option:{value}")
}

fn roving_values(spec: &RadioGroupSpec) -> Vec<String> {
    spec.options
        .iter()
        .filter(|option| !spec.is_disabled && !option.is_disabled)
        .map(|option| option.value.clone())
        .collect()
}

fn tab_stop_value<'a>(spec: &'a RadioGroupSpec, roving: &'a [String]) -> Option<&'a str> {
    let selected = spec.current_value();
    if selected.is_some_and(|value| roving.iter().any(|candidate| candidate == value)) {
        selected
    } else {
        roving.first().map(String::as_str)
    }
}

fn axis_step(orientation: Orientation, key: NodeKey, index: usize, last: usize) -> Option<usize> {
    match (orientation, key) {
        (Orientation::Vertical, NodeKey::ArrowDown)
        | (Orientation::Horizontal, NodeKey::ArrowRight) => {
            Some(if index == last { 0 } else { index + 1 })
        }
        (Orientation::Vertical, NodeKey::ArrowUp)
        | (Orientation::Horizontal, NodeKey::ArrowLeft) => {
            Some(if index == 0 { last } else { index - 1 })
        }
        // Contract §6: Home and End jump to the first/last enabled option on
        // either axis; extremes stay inert because the roving handler never
        // fires a change for the current value.
        (_, NodeKey::Home) => Some(0),
        (_, NodeKey::End) => Some(last),
        _ => None,
    }
}

fn roving_key_handler(
    value: &str,
    roving: &[String],
    instance_scope: String,
    orientation: Orientation,
    on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Option<Arc<dyn Fn(NodeKey, poodle_node::NodeModifiers) -> Option<String> + Send + Sync>> {
    let index = roving.iter().position(|candidate| candidate == value)?;
    let ids = roving.to_vec();
    let current = value.to_string();
    Some(Arc::new(move |key, _modifiers| {
        if ids.is_empty() {
            return None;
        }
        let last = ids.len() - 1;
        let next = axis_step(orientation, key, index, last)?;
        let target = ids[next].clone();
        if target == current {
            return None;
        }
        if let Some(handler) = &on_change {
            handler(&target);
        }
        Some(option_focus_id(&instance_scope, &target))
    }))
}

pub fn radio_group(
    spec: &RadioGroupSpec,
    ctx: &RenderContext<'_>,
    handlers: RadioGroupHandlers,
) -> Node {
    let instance_scope = handlers.instance_id.as_str();
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let icon_md = ctx.theme().resolve_space("size.icon.md");
    let indicator_size = indicator_size_px(effective_size, icon_md);
    let dot_size = dot_size_px(effective_size, icon_md);
    let border_width = rem_to_px(0.0625);

    // Density override wins over the orientation gap.
    let group_gap = match density {
        ControlDensity::Compact => ctx.theme().resolve_space("space.stack.sm"),
        ControlDensity::Comfortable => ctx.theme().resolve_space("space.stack.lg"),
        ControlDensity::Default => ctx.theme().resolve_space(spec.option_gap_token()),
    };
    let item_gap = ctx.theme().resolve_space("space.inline.sm");

    // Custom hex wins over accent. Colour-space note as in checkbox: the hex
    // lands in sRGB and converts at the backend edge — the old tier passed it
    // raw; divergence pinned in the parity suite.
    let accent = spec
        .selected_color
        .as_deref()
        .and_then(hex_color)
        .unwrap_or_else(|| ctx.theme().resolve_color("color.accent.base"));
    let border = ctx.theme().resolve_color("color.border.default");
    let text_color = ctx.theme().resolve_color("color.text.primary");
    let selected_value = spec.current_value();
    let disabled_opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
    let focus_ring = FocusRing {
        color: ctx.theme().resolve_color("color.accent.focusRing"),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset: rem_to_px(0.125),
    };
    let roving = roving_values(spec);
    let tab_stop = tab_stop_value(spec, &roving);

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = match spec.orientation {
            Orientation::Horizontal => LayoutDirection::Row,
            Orientation::Vertical => LayoutDirection::Column,
        };
        s.descriptor.layout.spacing.gap = group_gap;
    }

    for option in &spec.options {
        let is_selected = selected_value == Some(option.value.as_str());
        let option_disabled = spec.is_disabled || option.is_disabled;
        let indicator_color = if is_selected { accent } else { border };
        let indicator_bg = ctx.theme().resolve_color("color.background.surface");

        let mut indicator = Node::container();
        circle(&mut indicator, indicator_size);
        {
            let s = &mut indicator.style;
            s.descriptor.background = Some(indicator_bg);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = indicator_color;
            // Explicit Row (see switch.rs): the old tier got taffy's Row default.
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }

        if is_selected {
            let mut dot = Node::container();
            circle(&mut dot, dot_size);
            dot.style.descriptor.background = Some(accent);
            indicator = indicator.child(dot);
        }

        let mut row = Node::container();
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = item_gap;
            s.descriptor.cursor = if option_disabled {
                CursorHint::Default
            } else {
                CursorHint::Pointer
            };
        }
        row = row.child(indicator);

        let mut label = Node::text(&option.label);
        label.style.descriptor.text_color = Some(text_color);
        label.style.text_size = Some(font_size);
        row = row.child(label);

        // Per-option disabled dims that row only.
        if option.is_disabled {
            row.style.descriptor.opacity = disabled_opacity;
        }

        row.id = Some(option_id(&option.value));
        row.runtime_id = Some(option_focus_id(instance_scope, &option.value));
        row.a11y.role = Some(NodeRole::RadioButton);
        row.a11y.selected = Some(is_selected);
        row.a11y.toggled = Some(if is_selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        row.a11y.label = Some(
            option
                .aria_label
                .clone()
                .unwrap_or_else(|| option.label.clone()),
        );

        if option_disabled {
            row.interaction.disabled = true;
            row.interaction.focusable = false;
            row.a11y.tab_index = Some(-1);
        } else {
            row.interaction.focusable = true;
            row.a11y.tab_index = Some(if tab_stop == Some(option.value.as_str()) {
                0
            } else {
                -1
            });
            // Contract §8: `border-width-focus` of `accent-focusRing` at a
            // 0.125rem offset. The dedicated ring is also what GPUI tracks.
            row.style.focus_ring = Some(focus_ring);
            // Same-value selection is inert: native radios do not re-fire.
            if !is_selected {
                if let Some(handler) = &handlers.on_change {
                    let handler = Arc::clone(handler);
                    let value = option.value.clone();
                    row.interaction.on_activate = Some(Arc::new(move || handler(&value)));
                }
            }
            row.interaction.on_key = roving_key_handler(
                &option.value,
                &roving,
                instance_scope.to_string(),
                spec.orientation,
                handlers.on_change.clone(),
            );
        }

        el = el.child(row);
    }

    if spec.is_disabled {
        el.style.descriptor.opacity = disabled_opacity;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        el.a11y.label = Some(label.to_string());
    }
    el.a11y.role = Some(NodeRole::RadioGroup);
    el.a11y.orientation = Some(format!("{:?}", spec.orientation).to_ascii_lowercase());
    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_node::NodeModifiers;
    use poodle_specs::ChoiceOption;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn plan_options() -> Vec<ChoiceOption> {
        vec![
            ChoiceOption::new("free", "Free"),
            ChoiceOption::new("pro", "Pro"),
            ChoiceOption::new("enterprise", "Enterprise"),
        ]
    }

    fn find_option<'a>(node: &'a Node, value: &str) -> &'a Node {
        let id = option_id(value);
        node.find(&|n| n.id.as_deref() == Some(id.as_str()))
            .unwrap_or_else(|| panic!("option {value:?} exists"))
    }

    fn render(spec: &RadioGroupSpec, handlers: RadioGroupHandlers) -> Node {
        radio_group(spec, &RenderContext::new(&theme()), handlers)
    }

    #[test]
    fn selected_option_is_the_roving_tab_stop() {
        let spec = RadioGroupSpec::new(plan_options()).with_value("pro");
        let node = render(&spec, RadioGroupHandlers::new("plan"));
        assert_eq!(find_option(&node, "free").a11y.tab_index, Some(-1));
        let pro = find_option(&node, "pro");
        assert_eq!(pro.a11y.tab_index, Some(0));
        assert_eq!(pro.a11y.selected, Some(true));
        assert_eq!(pro.a11y.toggled, Some(NodeToggled::True));
        assert_eq!(find_option(&node, "enterprise").a11y.tab_index, Some(-1));
    }

    #[test]
    fn unknown_or_disabled_selection_falls_back_to_the_first_enabled_option() {
        let unknown = RadioGroupSpec::new(plan_options()).with_value("missing");
        let node = render(&unknown, RadioGroupHandlers::new("plan"));
        assert_eq!(find_option(&node, "free").a11y.tab_index, Some(0));

        let selected_disabled = RadioGroupSpec::new(vec![
            ChoiceOption::new("free", "Free").with_disabled(true),
            ChoiceOption::new("pro", "Pro"),
            ChoiceOption::new("enterprise", "Enterprise"),
        ])
        .with_value("free");
        let node = render(&selected_disabled, RadioGroupHandlers::new("plan"));
        assert_eq!(find_option(&node, "free").a11y.tab_index, Some(-1));
        assert!(!find_option(&node, "free").interaction.focusable);
        assert_eq!(find_option(&node, "pro").a11y.tab_index, Some(0));
    }

    #[test]
    fn disabled_group_has_no_tab_stop() {
        let spec = RadioGroupSpec {
            is_disabled: true,
            ..RadioGroupSpec::new(plan_options()).with_value("pro")
        };
        let node = render(&spec, RadioGroupHandlers::new("plan"));
        for value in ["free", "pro", "enterprise"] {
            let option = find_option(&node, value);
            assert!(!option.interaction.focusable, "{value}");
            assert!(option.interaction.disabled, "{value}");
            assert_eq!(option.a11y.tab_index, Some(-1), "{value}");
            assert!(option.style.focus_ring.is_none(), "{value}");
            assert!(option.interaction.on_activate.is_none(), "{value}");
            assert!(option.interaction.on_key.is_none(), "{value}");
        }
    }

    #[test]
    fn choosing_an_option_reports_its_value_and_same_value_stays_inert() {
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = RadioGroupSpec::new(plan_options()).with_value("pro");
        let node = render(
            &spec,
            RadioGroupHandlers::new("plan")
                .on_change(Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()))),
        );
        (find_option(&node, "enterprise")
            .interaction
            .on_activate
            .as_ref()
            .unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), ["enterprise"]);
        assert!(
            find_option(&node, "pro").interaction.on_activate.is_none(),
            "re-picking the selected option is inert"
        );
    }

    #[test]
    fn vertical_arrows_move_and_horizontal_arrows_are_inert() {
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = RadioGroupSpec::new(vec![
            ChoiceOption::new("free", "Free"),
            ChoiceOption::new("pro", "Pro").with_disabled(true),
            ChoiceOption::new("enterprise", "Enterprise"),
        ])
        .with_value("free");
        let node = render(
            &spec,
            RadioGroupHandlers::new("plan")
                .on_change(Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()))),
        );
        let keys = find_option(&node, "free")
            .interaction
            .on_key
            .as_ref()
            .expect("roving handler");
        let modifiers = NodeModifiers::default();
        assert_eq!(
            keys(NodeKey::ArrowDown, modifiers),
            Some(option_focus_id("plan", "enterprise"))
        );
        assert_eq!(seen.lock().unwrap().as_slice(), ["enterprise"]);
        assert!(keys(NodeKey::ArrowRight, modifiers).is_none());
        assert!(keys(NodeKey::ArrowLeft, modifiers).is_none());
        assert!(find_option(&node, "pro").interaction.on_key.is_none());
    }

    #[test]
    fn horizontal_arrows_wrap_and_vertical_arrows_are_inert() {
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = RadioGroupSpec::new(plan_options())
            .with_value("enterprise")
            .with_orientation(Orientation::Horizontal);
        let node = render(
            &spec,
            RadioGroupHandlers::new("size")
                .on_change(Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()))),
        );
        let keys = find_option(&node, "enterprise")
            .interaction
            .on_key
            .as_ref()
            .expect("roving handler");
        let modifiers = NodeModifiers::default();
        assert_eq!(
            keys(NodeKey::ArrowRight, modifiers),
            Some(option_focus_id("size", "free"))
        );
        assert_eq!(seen.lock().unwrap().as_slice(), ["free"]);
        assert!(keys(NodeKey::ArrowDown, modifiers).is_none());
        assert!(keys(NodeKey::ArrowUp, modifiers).is_none());
    }

    #[test]
    fn home_and_end_jump_across_enabled_options_and_stay_inert_at_extremes() {
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = RadioGroupSpec::new(vec![
            ChoiceOption::new("free", "Free"),
            ChoiceOption::new("pro", "Pro").with_disabled(true),
            ChoiceOption::new("enterprise", "Enterprise"),
        ])
        .with_value("free");
        let node = render(
            &spec,
            RadioGroupHandlers::new("plan")
                .on_change(Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()))),
        );
        let modifiers = NodeModifiers::default();
        let free_keys = find_option(&node, "free")
            .interaction
            .on_key
            .as_ref()
            .expect("roving handler");
        let enterprise_keys = find_option(&node, "enterprise")
            .interaction
            .on_key
            .as_ref()
            .expect("roving handler");
        assert_eq!(
            free_keys(NodeKey::End, modifiers),
            Some(option_focus_id("plan", "enterprise")),
            "End jumps over the disabled option to the last enabled one"
        );
        assert_eq!(seen.lock().unwrap().as_slice(), ["enterprise"]);
        assert!(
            enterprise_keys(NodeKey::End, modifiers).is_none(),
            "End on the last enabled option is inert"
        );
        assert_eq!(
            enterprise_keys(NodeKey::Home, modifiers),
            Some(option_focus_id("plan", "free")),
            "Home jumps to the first enabled option"
        );
        assert_eq!(seen.lock().unwrap().as_slice(), ["enterprise", "free"]);
        assert!(
            free_keys(NodeKey::Home, modifiers).is_none(),
            "Home on the first enabled option is inert"
        );
    }

    #[test]
    fn enabled_options_declare_the_contracted_focus_ring() {
        let theme = theme();
        let ring_color = theme.resolve_color("color.accent.focusRing");
        let spec = RadioGroupSpec::new(vec![
            ChoiceOption::new("free", "Free"),
            ChoiceOption::new("pro", "Pro"),
            ChoiceOption::new("enterprise", "Enterprise").with_disabled(true),
        ])
        .with_value("pro");
        let node = render(&spec, RadioGroupHandlers::new("plan"));
        let ring = find_option(&node, "pro")
            .style
            .focus_ring
            .expect("an enabled option declares a ring");
        assert_eq!(ring.color, ring_color);
        assert_eq!(ring.width, 2.0);
        assert_eq!(ring.offset, rem_to_px(0.125));
        assert!(
            find_option(&node, "free").style.focus_ring.is_some(),
            "unselected enabled options also declare the ring",
        );
        assert!(
            find_option(&node, "enterprise").style.focus_ring.is_none(),
            "a disabled option is unfocusable and declares no ring",
        );
    }

    #[test]
    fn instance_scope_keeps_roving_focus_inside_the_originating_control() {
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_change: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |v: &str| sink.lock().unwrap().push(v.into()));
        let spec = RadioGroupSpec::new(plan_options()).with_value("free");
        let a = render(
            &spec,
            RadioGroupHandlers::new("left").on_change(Arc::clone(&on_change)),
        );
        let b = render(&spec, RadioGroupHandlers::new("right").on_change(on_change));
        let a_free = find_option(&a, "free");
        let b_free = find_option(&b, "free");
        assert_eq!(a_free.id.as_deref(), Some("radio:free"));
        assert_eq!(b_free.id.as_deref(), Some("radio:free"));
        assert_eq!(a_free.runtime_id.as_deref(), Some("radio:left:option:free"));
        assert_eq!(
            b_free.runtime_id.as_deref(),
            Some("radio:right:option:free")
        );
        let modifiers = NodeModifiers::default();
        assert_eq!(
            (a_free.interaction.on_key.as_ref().unwrap())(NodeKey::ArrowDown, modifiers),
            Some("radio:left:option:pro".to_string())
        );
        assert_eq!(
            (b_free.interaction.on_key.as_ref().unwrap())(NodeKey::ArrowDown, modifiers),
            Some("radio:right:option:pro".to_string())
        );
        assert_eq!(seen.lock().unwrap().as_slice(), ["pro", "pro"]);
    }

    #[test]
    fn radiogroup_role_and_orientation_ride_the_root() {
        let mut spec =
            RadioGroupSpec::new(plan_options()).with_orientation(Orientation::Horizontal);
        spec.aria_label = Some("Plan".to_string());
        let node = render(&spec, RadioGroupHandlers::new("plan"));
        assert_eq!(node.a11y.role, Some(NodeRole::RadioGroup));
        assert_eq!(node.a11y.label.as_deref(), Some("Plan"));
        assert_eq!(node.a11y.orientation.as_deref(), Some("horizontal"));
        let free = find_option(&node, "free");
        assert_eq!(free.a11y.role, Some(NodeRole::RadioButton));
        assert_eq!(free.a11y.selected, Some(false));
        assert_eq!(free.a11y.toggled, Some(NodeToggled::False));
        assert_eq!(free.a11y.tab_index, Some(0));
    }

    #[test]
    fn native_scope_is_not_the_web_form_name() {
        let spec = RadioGroupSpec::new(plan_options()).with_name("form-plan");
        let node = render(&spec, RadioGroupHandlers::new("host-plan"));
        assert_eq!(
            find_option(&node, "free").runtime_id.as_deref(),
            Some("radio:host-plan:option:free")
        );
        assert!(find_option(&node, "free")
            .runtime_id
            .as_deref()
            .is_none_or(|id| !id.contains("form-plan")));
    }
}
