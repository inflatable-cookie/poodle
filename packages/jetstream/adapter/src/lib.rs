//! Theme and accessibility role mappings for the Jetstream node route.
//!
//! Shared components produce Poodle node trees in `poodle-render`.
//! The Jetstream repository's `jetstream-poodle` crate converts them to `JsEl`.

mod theme;

pub use theme::JetstreamThemeProvider;

use poodle_node::NodeRole;

/// AccessKit / ARIA role used by Jetstream for a node role.
/// Exhaustive so a new `NodeRole` cannot ship unmapped.
pub fn accesskit_role(role: NodeRole) -> &'static str {
    match role {
        NodeRole::Alert => "alert",
        NodeRole::AlertDialog => "alertdialog",
        NodeRole::Banner => "banner",
        NodeRole::Button => "button",
        NodeRole::Cell => "cell",
        NodeRole::CheckBox => "checkbox",
        NodeRole::ColumnHeader => "columnheader",
        NodeRole::ComboBox => "combobox",
        NodeRole::ContentInfo => "contentinfo",
        NodeRole::Dialog => "dialog",
        NodeRole::Figure => "figure",
        NodeRole::Grid => "grid",
        NodeRole::Group => "group",
        NodeRole::Heading => "heading",
        NodeRole::SearchBox => "searchbox",
        NodeRole::Label => "label",
        NodeRole::Link => "link",
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
        NodeRole::Meter => "meter",
        NodeRole::Navigation => "navigation",
        NodeRole::Presentation => "presentation",
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
        NodeRole::Table => "table",
        NodeRole::RowHeader => "rowheader",
        NodeRole::Tab => "tab",
        NodeRole::TabList => "tablist",
        NodeRole::TabPanel => "tabpanel",
        NodeRole::TextInput => "textbox",
        NodeRole::Toolbar => "toolbar",
        NodeRole::Tooltip => "tooltip",
        NodeRole::Tree => "tree",
        NodeRole::TreeItem => "treeitem",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_and_banner_roles_map() {
        assert_eq!(accesskit_role(NodeRole::Heading), "heading");
        assert_eq!(accesskit_role(NodeRole::Banner), "banner");
    }
}
