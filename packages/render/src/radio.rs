//! Radio — a single option with a circular indicator.
//!
//! Contract: `docs/contracts/components/radio.md`
//!
//! Single-option semantics, not RadioGroup: activate selects (fires
//! `true`) and never unchecks this control. Group exclusivity is host-owned
//! on native — the web uses the browser's `name`. Disabled is inert;
//! read-only stays focusable but reports no change.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeRole, NodeToggled, StylePatch,
};
use poodle_specs::{ControlDensity, ControlSize, RadioSpec};

use crate::color::hex_color;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};

fn indicator_size_px(size: ControlSize, icon_md_px: f32) -> f32 {
    match size {
        ControlSize::Xs => icon_md_px - rem_to_px(0.125),
        ControlSize::Sm => icon_md_px,
        ControlSize::Md => rem_to_px(1.125),
        ControlSize::Lg => icon_md_px + rem_to_px(0.375),
        ControlSize::Xl => icon_md_px + rem_to_px(0.625),
    }
}

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

/// Build a radio node. `on_checked_change` fires with `true` when the user
/// selects an unchecked, interactive radio. A second activate on an already
/// checked radio is a no-op — native radios do not uncheck themselves.
pub fn radio(
    spec: &RadioSpec,
    theme: &dyn ThemeProvider,
    on_checked_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let icon_md = theme.resolve_space("size.icon.md");
    let indicator_size = indicator_size_px(effective_size, icon_md);
    let dot_size = dot_size_px(effective_size, icon_md);
    let border_width = rem_to_px(0.0625);
    let item_gap = match spec.density {
        ControlDensity::Compact => rem_to_px(0.375),
        ControlDensity::Default => theme.resolve_space("space.inline.sm"),
        ControlDensity::Comfortable => theme.resolve_space("space.inline.md"),
    };

    let accent = spec
        .selected_color
        .as_deref()
        .and_then(hex_color)
        .unwrap_or_else(|| theme.resolve_color("color.accent.base"));
    let border = theme.resolve_color("color.border.default");
    let text_color = theme.resolve_color("color.text.primary");
    let is_checked = spec.is_checked();
    let indicator_color = if is_checked { accent } else { border };
    let indicator_bg = theme.resolve_color("color.background.surface");

    let mut indicator = Node::container();
    circle(&mut indicator, indicator_size);
    {
        let s = &mut indicator.style;
        s.descriptor.background = Some(indicator_bg);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = indicator_color;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }
    if is_checked {
        let mut dot = Node::container();
        circle(&mut dot, dot_size);
        dot.style.descriptor.background = Some(accent);
        indicator = indicator.child(dot);
    }

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = item_gap;
        s.descriptor.cursor = if spec.is_disabled {
            CursorHint::Default
        } else {
            CursorHint::Pointer
        };
    }
    if !spec.is_disabled {
        root.interaction.focusable = true;
        root.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            text_color: None,
            opacity: None,
        });
    }
    root = root.child(indicator);

    if let Some(label) = &spec.label {
        let mut text = Node::text(label);
        text.style.descriptor.text_color = Some(text_color);
        text.style.text_size = Some(font_size);
        text.style.text_weight = Some(500);
        root = root.child(text);
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        root.interaction.disabled = true;
    }

    if spec.is_interactive() && !is_checked {
        if let Some(handler) = on_checked_change {
            root.interaction.on_activate = Some(Arc::new(move || handler(true)));
        }
    }

    if let Some(id) = &spec.id {
        root.id = Some(id.clone());
    }
    if let Some(name) = &spec.name {
        root.roles.insert("name".to_owned(), name.clone());
    }
    if let Some(value) = &spec.value {
        root.roles.insert("value".to_owned(), value.clone());
    }

    root.a11y.role = Some(NodeRole::RadioButton);
    root.a11y.toggled = Some(if is_checked {
        NodeToggled::True
    } else {
        NodeToggled::False
    });
    root.a11y.label = spec
        .aria_label
        .clone()
        .or_else(|| spec.label.clone());
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn texts(node: &Node) -> Vec<String> {
        node.texts()
            .into_iter()
            .map(str::to_string)
            .filter(|t| !t.is_empty())
            .collect()
    }

    #[test]
    fn unchecked_activate_fires_true_and_does_not_uncheck() {
        let hits = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&hits);
        let node = radio(
            &RadioSpec::new()
                .with_name("shipping")
                .with_value("standard")
                .with_label("Standard shipping"),
            &theme(),
            Some(Arc::new(move |checked| sink.lock().unwrap().push(checked))),
        );
        assert_eq!(node.a11y.role, Some(NodeRole::RadioButton));
        assert_eq!(node.a11y.toggled, Some(NodeToggled::False));
        assert_eq!(node.roles.get("name").map(String::as_str), Some("shipping"));
        assert_eq!(
            node.roles.get("value").map(String::as_str),
            Some("standard")
        );
        node.interaction
            .on_activate
            .as_ref()
            .expect("unchecked radio is activatable")();
        assert_eq!(*hits.lock().unwrap(), vec![true]);

        let checked = radio(
            &RadioSpec::new()
                .with_checked(true)
                .with_name("shipping")
                .with_value("standard")
                .with_label("Standard shipping"),
            &theme(),
            Some(Arc::new(|_| panic!("already-checked radio must not fire"))),
        );
        assert_eq!(checked.a11y.toggled, Some(NodeToggled::True));
        assert!(
            checked.interaction.on_activate.is_none(),
            "a second activate must not uncheck — that is RadioGroup's job"
        );
        assert!(texts(&checked).iter().any(|t| t == "Standard shipping"));
        assert_eq!(
            checked.children[0].children.len(),
            1,
            "checked radio paints a dot"
        );
        assert!(
            node.children[0].children.is_empty(),
            "unchecked radio has no dot"
        );
    }

    #[test]
    fn disabled_and_readonly_do_not_report_a_change() {
        let disabled = radio(
            &RadioSpec::new()
                .with_disabled(true)
                .with_label("Standard shipping"),
            &theme(),
            Some(Arc::new(|_| panic!("disabled"))),
        );
        assert!(disabled.interaction.disabled);
        assert!(disabled.interaction.on_activate.is_none());
        assert!(!disabled.interaction.focusable);

        let readonly = radio(
            &RadioSpec::new()
                .with_read_only(true)
                .with_label("Standard shipping"),
            &theme(),
            Some(Arc::new(|_| panic!("readonly"))),
        );
        assert!(readonly.interaction.focusable);
        assert!(readonly.interaction.on_activate.is_none());
        assert!(!readonly.interaction.disabled);
    }

    #[test]
    fn accessible_name_comes_from_the_visible_label_or_aria_label() {
        let labelled = radio(
            &RadioSpec::new().with_label("Standard shipping"),
            &theme(),
            None,
        );
        assert_eq!(labelled.a11y.label.as_deref(), Some("Standard shipping"));

        let unlabelled = radio(
            &RadioSpec::new().with_aria_label("Standard shipping"),
            &theme(),
            None,
        );
        assert_eq!(unlabelled.a11y.label.as_deref(), Some("Standard shipping"));
        assert!(texts(&unlabelled).is_empty());
    }
}
