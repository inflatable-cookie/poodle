//! Project the node accessibility record into GPUI's AccessKit elements.

use gpui::{
    accesskit::{self, Role},
    StatefulInteractiveElement,
};
use poodle_node::{HasPopup, Node, NodeRole, NodeToggled};

use super::record_probe_channel;

/// Map the shared role vocabulary to AccessKit. Presentation nodes are
/// intentionally omitted: GPUI filters its generic-container role.
pub(super) fn accesskit_role(role: NodeRole) -> Option<Role> {
    Some(match role {
        NodeRole::Alert => Role::Alert,
        NodeRole::AlertDialog => Role::AlertDialog,
        NodeRole::Banner => Role::Banner,
        NodeRole::Button => Role::Button,
        NodeRole::Cell => Role::Cell,
        NodeRole::CheckBox => Role::CheckBox,
        NodeRole::ColumnHeader => Role::ColumnHeader,
        NodeRole::ComboBox => Role::ComboBox,
        NodeRole::ContentInfo => Role::ContentInfo,
        NodeRole::Dialog => Role::Dialog,
        NodeRole::Figure => Role::Figure,
        NodeRole::Grid => Role::Grid,
        NodeRole::Group => Role::Group,
        NodeRole::Heading => Role::Heading,
        NodeRole::SearchBox => Role::SearchInput,
        NodeRole::Label => Role::Label,
        NodeRole::Link => Role::Link,
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
        NodeRole::Meter => Role::Meter,
        NodeRole::Navigation => Role::Navigation,
        NodeRole::Presentation => return None,
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
        NodeRole::Table => Role::Table,
        NodeRole::RowHeader => Role::RowHeader,
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

pub(super) fn has_accesskit_role(node: &Node) -> bool {
    node.a11y
        .role
        .is_some_and(|role| accesskit_role(role).is_some())
}

/// Attach the node record to an identified GPUI element. The public fluent
/// API does not expose every field in `NodeA11y`; its subtree callback gives
/// access to the element's AccessKit node for those same existing fields.
pub(super) fn apply<E: StatefulInteractiveElement>(mut element: E, node: &Node) -> E {
    let Some(role) = node.a11y.role.and_then(accesskit_role) else {
        return element;
    };
    record_probe_channel("accessibility.projection.received");
    element = element.role(role);

    let a11y = node.a11y.clone();
    let disabled = node.interaction.disabled;
    element.a11y_synthetic_children(move |builder| {
        let node = builder.parent_node();

        if let Some(label) = a11y.label {
            node.set_label(label);
        }
        if let Some(expanded) = a11y.expanded {
            node.set_expanded(expanded);
        }
        if let Some(selected) = a11y.selected {
            node.set_selected(selected);
        }
        if let Some(multiselectable) = a11y.multiselectable {
            if multiselectable {
                node.set_multiselectable();
            } else {
                node.clear_multiselectable();
            }
        }
        if let Some(toggled) = a11y.toggled {
            node.set_toggled(match toggled {
                NodeToggled::True => accesskit::Toggled::True,
                NodeToggled::False => accesskit::Toggled::False,
                NodeToggled::Mixed => accesskit::Toggled::Mixed,
            });
        }
        if let Some(level) = a11y.level {
            node.set_level(level);
        }
        if let Some(value) = a11y.value {
            node.set_numeric_value(value);
        }
        if let Some(value_min) = a11y.value_min {
            node.set_min_numeric_value(value_min);
        }
        if let Some(value_max) = a11y.value_max {
            node.set_max_numeric_value(value_max);
        }
        if let Some(value_text) = a11y.value_text {
            node.set_value(value_text);
        }
        if let Some(orientation) = a11y.orientation.as_deref() {
            match orientation {
                "horizontal" => node.set_orientation(accesskit::Orientation::Horizontal),
                "vertical" => node.set_orientation(accesskit::Orientation::Vertical),
                _ => {}
            }
        }
        if a11y.has_popup == Some(HasPopup::Menu) {
            node.set_has_popup(accesskit::HasPopup::Menu);
        }
        if disabled {
            node.set_disabled();
        }
        if a11y.hidden == Some(true) {
            node.set_hidden();
        }
        if a11y.invalid == Some(true) {
            node.set_invalid(accesskit::Invalid::True);
        }
        if a11y.busy == Some(true) {
            node.set_busy();
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_roles_map_to_accesskit_and_presentation_stays_omitted() {
        assert_eq!(accesskit_role(NodeRole::Heading), Some(Role::Heading));
        assert_eq!(accesskit_role(NodeRole::SearchBox), Some(Role::SearchInput));
        assert_eq!(
            accesskit_role(NodeRole::ProgressIndicator),
            Some(Role::ProgressIndicator)
        );
        assert_eq!(accesskit_role(NodeRole::Presentation), None);
    }
}
