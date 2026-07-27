//! Carrying spec accessibility onto the elements components return.
//!
//! Until Jetstream grew an AccessKit surface, `aria_label` was a field that
//! every spec had and nothing read: components forwarded it to each other and
//! it terminated in a struct. It now reaches assistive technology, and the job
//! of this module is to make attaching it a single call so that the 100-odd
//! components do it the same way.
//!
//! The rule is that **a spec's `aria_label` names the component as a whole**,
//! so it belongs on the component's root element and nowhere else. A component
//! that composes another one forwards the spec instead, and the inner component
//! attaches it — adding it in both places would announce the name twice.

use jetstream_ui::ui_element::JsEl;

/// Attach an accessible name to a component root.
///
/// A `None` or empty label leaves the element alone rather than setting an
/// empty name, because an empty accessible name is worse than none: it
/// overrides the text the element would otherwise have been announced by.
pub fn with_aria_label(el: JsEl, label: Option<&str>) -> JsEl {
    match label {
        Some(label) if !label.is_empty() => el.aria_label(label),
        _ => el,
    }
}

/// Map a contract tri-state onto AccessKit's checked state.
///
/// Poodle specs spell the third state as `None` — `checked: Option<bool>` on
/// checkbox, switch and their kin — which reads as "unset" and means
/// "indeterminate". AccessKit spells it `Toggled::Mixed`, and the contracts
/// spell it `aria-checked="mixed"`. Naming the mapping once keeps the three
/// spellings from drifting, and stops `None` being quietly rendered as
/// unchecked, which is a different and wrong claim about the control.
pub fn toggled(checked: Option<bool>) -> jetstream_ui::accesskit::Toggled {
    use jetstream_ui::accesskit::Toggled;
    match checked {
        Some(true) => Toggled::True,
        Some(false) => Toggled::False,
        None => Toggled::Mixed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jetstream_ui::ui_element::div;

    /// The whole chain, in one test: a spec's `aria_label` → the component's
    /// root element → the materialized `UiTree` → the AccessKit tree a screen
    /// reader reads. Every link used to exist except the last two, which is
    /// why the field was inert on this target.
    #[test]
    fn a_spec_label_reaches_the_accesskit_tree() {
        use jetstream_ui::GameUi;
        use poodle_specs::ButtonSpec;

        let spec = ButtonSpec::new()
            .with_label("")
            .with_aria_label("Delete project");
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);

        let mut ui = GameUi::new(400.0, 200.0);
        ui.render_immediate(&crate::button::js_button(&spec, &theme));

        let update = ui.accessibility_update().expect("a rendered tree projects");
        let labels: Vec<_> = update
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .collect();

        assert!(
            labels.iter().any(|l| &**l == "Delete project"),
            "aria_label never reached the accessibility tree; found {labels:?}"
        );
    }

    /// `None` is the contract's indeterminate state, not "unchecked". Rendering
    /// it as `False` would tell a screen reader something specific and untrue.
    #[test]
    fn an_unset_tri_state_is_mixed_not_false() {
        use jetstream_ui::accesskit::Toggled;
        assert_eq!(toggled(Some(true)), Toggled::True);
        assert_eq!(toggled(Some(false)), Toggled::False);
        assert_eq!(toggled(None), Toggled::Mixed);
    }

    /// A checkbox that reports `GenericContainer` is announced as nothing in
    /// particular. Its contract requires the role *and* `aria-checked="mixed"`
    /// for the indeterminate state, so both are asserted through the real tree.
    #[test]
    fn a_checkbox_reports_its_role_and_mixed_state() {
        use jetstream_ui::accesskit::{Role, Toggled};
        use jetstream_ui::GameUi;
        use poodle_specs::CheckboxSpec;

        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let mut ui = GameUi::new(400.0, 200.0);
        // `checked: None` is the contract's indeterminate state.
        ui.render_immediate(&crate::checkbox::js_checkbox(&CheckboxSpec::new(), &theme));

        let update = ui.accessibility_update().expect("a rendered tree projects");
        let checkbox = update
            .nodes
            .iter()
            .find(|(_, node)| node.role() == Role::CheckBox)
            .expect("a node reports Role::CheckBox");
        assert_eq!(checkbox.1.toggled(), Some(Toggled::Mixed));
    }

    #[test]
    fn a_label_is_attached() {
        let el = with_aria_label(div(), Some("Close"));
        assert_eq!(el.style.accessibility.label.as_deref(), Some("Close"));
    }

    /// An empty string must not shadow the element's own text.
    #[test]
    fn an_empty_label_is_not_attached() {
        assert!(with_aria_label(div(), Some("")).style.accessibility.label.is_none());
        assert!(with_aria_label(div(), None).style.accessibility.label.is_none());
    }
}
