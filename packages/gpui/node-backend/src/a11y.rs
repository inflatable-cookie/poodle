//! Record `poodle-node` accessibility roles on the GPUI 0.2.2 path.
//!
//! crates.io GPUI has no AccessKit paint API. Roles stay on the node record
//! and the painted snapshot (`a11y_role`); this match is total so a new role
//! cannot ship unmapped.

use poodle_node::NodeRole;

use super::record_probe_channel;

/// ARIA role string the backend records. Paint is a no-op.
pub fn record_role(role: NodeRole) -> &'static str {
    let aria = match role {
        NodeRole::Alert => "alert",
        NodeRole::AlertDialog => "alertdialog",
        NodeRole::Banner => "banner",
        NodeRole::Button => "button",
        NodeRole::Cell => "cell",
        NodeRole::CheckBox => "checkbox",
        NodeRole::ComboBox => "combobox",
        NodeRole::Dialog => "dialog",
        NodeRole::Grid => "grid",
        NodeRole::Group => "group",
        NodeRole::Heading => "heading",
        NodeRole::SearchBox => "searchbox",
        NodeRole::Label => "label",
        NodeRole::List => "list",
        NodeRole::ListItem => "listitem",
        NodeRole::ListBox => "listbox",
        NodeRole::ListBoxOption => "option",
        NodeRole::Log => "log",
        NodeRole::Image => "img",
        NodeRole::Menu => "menu",
        NodeRole::MenuBar => "menubar",
        NodeRole::MenuItem => "menuitem",
        NodeRole::MenuItemCheckBox => "menuitemcheckbox",
        NodeRole::MenuItemRadio => "menuitemradio",
        NodeRole::Splitter => "separator",
        NodeRole::Slider => "slider",
        NodeRole::ProgressIndicator => "progressbar",
        NodeRole::RadioGroup => "radiogroup",
        NodeRole::RadioButton => "radio",
        NodeRole::Region => "region",
        NodeRole::Row => "row",
        NodeRole::SpinButton => "spinbutton",
        NodeRole::Status => "status",
        NodeRole::Switch => "switch",
        NodeRole::Tab => "tab",
        NodeRole::TabList => "tablist",
        NodeRole::TabPanel => "tabpanel",
        NodeRole::TextInput => "textbox",
        NodeRole::Toolbar => "toolbar",
        NodeRole::Tooltip => "tooltip",
        NodeRole::Tree => "tree",
        NodeRole::TreeItem => "treeitem",
    };
    record_probe_channel("accessibility.projection.received");
    aria
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_and_banner_map() {
        assert_eq!(record_role(NodeRole::Heading), "heading");
        assert_eq!(record_role(NodeRole::Banner), "banner");
        assert_eq!(record_role(NodeRole::SearchBox), "searchbox");
    }
}
