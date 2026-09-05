//! Project `poodle-node` accessibility records onto GPUI 1.19 AccessKit
//! attributes. Roles, labels, states, numeric values, and one Click action
//! round-trip. Relationship fields (`controls`, `labelled_by`, `described_by`)
//! and `invalid`/`busy` have no fluent GPUI counterpart at this version.

use super::*;
use gpui::{AccessibleAction, Orientation, Role, Toggled};
use poodle_node::NodeToggled;

pub(super) fn requires_state(node: &Node) -> bool {
    node.a11y.role.is_some()
        || node.a11y.label.is_some()
        || node.a11y.expanded.is_some()
        || node.a11y.selected.is_some()
        || node.a11y.toggled.is_some()
        || node.a11y.value.is_some()
        || node.a11y.value_min.is_some()
        || node.a11y.value_max.is_some()
        || node.a11y.value_text.is_some()
        || node.a11y.orientation.is_some()
        || node.a11y.level.is_some()
}

pub(super) fn apply(mut el: Stateful<Div>, node: &Node) -> Stateful<Div> {
    let Some(role) = node.a11y.role.and_then(map_role) else {
        return el;
    };
    record_probe_channel("accessibility.projection.applied");
    el = el.role(role);
    if let Some(label) = node.a11y.label.as_deref() {
        el = el.aria_label(label.to_owned());
    }
    if let Some(selected) = node.a11y.selected {
        el = el.aria_selected(selected);
    }
    if let Some(expanded) = node.a11y.expanded {
        el = el.aria_expanded(expanded);
    }
    if let Some(toggled) = node.a11y.toggled {
        el = el.aria_toggled(map_toggled(toggled));
    }
    if let Some(value) = node.a11y.value {
        el = el.aria_numeric_value(value);
    }
    if let Some(min) = node.a11y.value_min {
        el = el.aria_min_numeric_value(min);
    }
    if let Some(max) = node.a11y.value_max {
        el = el.aria_max_numeric_value(max);
    }
    if let Some(text) = node.a11y.value_text.as_deref() {
        el = el.aria_value(text.to_owned());
    }
    if let Some(level) = node.a11y.level {
        el = el.aria_level(level);
    }
    if let Some(orientation) = node.a11y.orientation.as_deref().and_then(map_orientation) {
        el = el.aria_orientation(orientation);
    }
    if let Some(activate) = node.interaction.on_activate.clone() {
        el = el.on_a11y_action(AccessibleAction::Click, move |_data, _window, cx| {
            activate();
            cx.refresh_windows();
        });
    }
    el
}

pub(super) fn map_role(role: NodeRole) -> Option<Role> {
    Some(match role {
        NodeRole::Alert => Role::Alert,
        NodeRole::AlertDialog => Role::AlertDialog,
        NodeRole::Button => Role::Button,
        NodeRole::Cell => Role::Cell,
        NodeRole::CheckBox => Role::CheckBox,
        NodeRole::ComboBox => Role::ComboBox,
        NodeRole::Dialog => Role::Dialog,
        NodeRole::Grid => Role::Grid,
        NodeRole::Group => Role::Group,
        NodeRole::Label => Role::Label,
        NodeRole::List => Role::List,
        NodeRole::ListItem => Role::ListItem,
        NodeRole::ListBox => Role::ListBox,
        NodeRole::ListBoxOption => Role::ListBoxOption,
        NodeRole::Log => Role::Log,
        NodeRole::Image => Role::Image,
        NodeRole::Menu => Role::Menu,
        NodeRole::MenuBar => Role::MenuBar,
        NodeRole::MenuItem => Role::MenuItem,
        NodeRole::MenuItemCheckBox => Role::MenuItemCheckBox,
        NodeRole::MenuItemRadio => Role::MenuItemRadio,
        NodeRole::Splitter => Role::Splitter,
        NodeRole::Slider => Role::Slider,
        NodeRole::ProgressIndicator => Role::ProgressIndicator,
        NodeRole::RadioGroup => Role::RadioGroup,
        NodeRole::RadioButton => Role::RadioButton,
        NodeRole::Region => Role::Region,
        NodeRole::Row => Role::Row,
        NodeRole::SpinButton => Role::SpinButton,
        NodeRole::Status => Role::Status,
        NodeRole::Switch => Role::Switch,
        NodeRole::Tab => Role::Tab,
        NodeRole::TabList => Role::TabList,
        NodeRole::TabPanel => Role::TabPanel,
        NodeRole::TextInput => Role::TextInput,
        NodeRole::Toolbar => Role::Toolbar,
        NodeRole::Tooltip => Role::Tooltip,
        NodeRole::Tree => Role::Tree,
        NodeRole::TreeItem => Role::TreeItem,
    })
}

fn map_toggled(value: NodeToggled) -> Toggled {
    match value {
        NodeToggled::True => Toggled::True,
        NodeToggled::False => Toggled::False,
        NodeToggled::Mixed => Toggled::Mixed,
    }
}

fn map_orientation(value: &str) -> Option<Orientation> {
    match value {
        "horizontal" => Some(Orientation::Horizontal),
        "vertical" => Some(Orientation::Vertical),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbox_slider_and_tabs_roles_map() {
        assert_eq!(map_role(NodeRole::CheckBox), Some(Role::CheckBox));
        assert_eq!(map_role(NodeRole::Slider), Some(Role::Slider));
        assert_eq!(map_role(NodeRole::Tab), Some(Role::Tab));
        assert_eq!(map_role(NodeRole::TabList), Some(Role::TabList));
        assert_eq!(map_role(NodeRole::TabPanel), Some(Role::TabPanel));
    }

    #[test]
    fn a11y_role_forces_element_state() {
        let mut node = Node::button("ok");
        assert!(!requires_state(&node));
        node.a11y.role = Some(NodeRole::CheckBox);
        node.a11y.label = Some("Accept".into());
        node.a11y.toggled = Some(NodeToggled::False);
        assert!(requires_state(&node));
    }
}
