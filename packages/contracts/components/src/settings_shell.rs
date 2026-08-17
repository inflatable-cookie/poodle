use crate::sidebar_nav::SidebarNavGroup;

/// SettingsShell — the settings frame. The host owns every page, the group
/// data, search execution, and whether a close attempt succeeds.
///
/// Contract: `docs/contracts/components/settings-shell.md`
///
/// `page` is a host-composed node, not a spec field. `search_query` is
/// host-owned; the shell never filters `groups` itself.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingsShellSpec {
    pub groups: Vec<SidebarNavGroup>,
    pub active_page_id: Option<String>,
    pub page_title: Option<String>,
    pub search_query: String,
    pub aria_label: Option<String>,
    pub open: Option<bool>,
    pub default_open: bool,
    pub title: String,
    pub close_label: String,
    pub close_refused_reason: Option<String>,
}

impl Default for SettingsShellSpec {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            active_page_id: None,
            page_title: None,
            search_query: String::new(),
            aria_label: None,
            open: None,
            default_open: false,
            title: "Settings".to_string(),
            close_label: "Close settings".to_string(),
            close_refused_reason: None,
        }
    }
}

impl SettingsShellSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_groups(mut self, groups: Vec<SidebarNavGroup>) -> Self {
        self.groups = groups;
        self
    }

    pub fn with_active_page_id(mut self, id: impl Into<String>) -> Self {
        self.active_page_id = Some(id.into());
        self
    }

    pub fn with_page_title(mut self, title: impl Into<String>) -> Self {
        self.page_title = Some(title.into());
        self
    }

    pub fn with_search_query(mut self, query: impl Into<String>) -> Self {
        self.search_query = query.into();
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_close_label(mut self, label: impl Into<String>) -> Self {
        self.close_label = label.into();
        self
    }

    pub fn with_close_refused_reason(mut self, reason: impl Into<String>) -> Self {
        self.close_refused_reason = Some(reason.into());
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn is_filtering(&self) -> bool {
        !self.search_query.trim().is_empty()
    }

    pub fn effective_aria_label(&self) -> &str {
        self.aria_label.as_deref().unwrap_or(&self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sidebar_nav::{SidebarNavGroup, SidebarNavItem};

    #[test]
    fn empty_groups_with_a_live_query_is_filtering_not_an_empty_scope() {
        let spec = SettingsShellSpec::new().with_search_query("xyzzy");
        assert!(spec.is_filtering());
        assert!(spec.groups.is_empty());
        assert!(!SettingsShellSpec::new().is_filtering());
    }

    #[test]
    fn open_is_host_owned_and_the_dialog_name_falls_back_to_title() {
        let spec = SettingsShellSpec::new()
            .with_open(true)
            .with_groups(vec![SidebarNavGroup::new(
                "general",
                vec![SidebarNavItem::new("general", "General")],
            )
            .with_label("General")]);
        assert!(spec.current_open());
        assert_eq!(spec.effective_aria_label(), "Settings");
    }
}
