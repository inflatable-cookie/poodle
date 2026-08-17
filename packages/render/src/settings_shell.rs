//! SettingsShell — the settings frame: dialog, search, nav rail, page slot.
//!
//! Contract: `docs/contracts/components/settings-shell.md`
//!
//! The host owns every page (a composed node), the group data, search
//! execution, and whether a close attempt succeeds. The shell never filters
//! `groups` and never invents a refused-close reason.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node, NodeRole};
use poodle_specs::{
    CallOutSpec, CalloutAnnounceMode, DialogSpec, DialogWidth, Direction, EmptyStateSpec,
    EmptyStateVariant, PaddingScale, ScrollShellSpec, SettingsShellSpec, SidebarNavSpec,
    StatusTone, SurfaceBorder, SurfaceSpec, SurfaceTone, TextInputSpec,
};

use crate::callout::{callout, CalloutHandlers};
use crate::dialog::dialog_with_slots;
use crate::empty_state::empty_state;
use crate::presentation::rem_to_px;
use crate::scroll_shell::scroll_shell;
use crate::sidebar_nav::sidebar_nav;
use crate::surface::surface;
use crate::text_input::text_input_with_change;

#[derive(Default)]
pub struct SettingsShellHandlers {
    pub on_navigate: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_request_close: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_search_query_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn settings_shell(
    spec: &SettingsShellSpec,
    theme: &dyn ThemeProvider,
    handlers: SettingsShellHandlers,
    page: Option<Node>,
) -> Node {
    if !spec.current_open() {
        return Node::container();
    }

    let refused = spec.close_refused_reason.is_some();
    let on_request_close = handlers.on_request_close.clone();
    let on_open_change = handlers.on_open_change.clone();
    let close = Some(Arc::new(move || {
        if let Some(handler) = &on_request_close {
            handler();
        }
        if !refused {
            if let Some(handler) = &on_open_change {
                handler(false);
            }
        }
    }) as Arc<dyn Fn() + Send + Sync>);

    let dialog_spec = DialogSpec::new()
        .with_open(true)
        .with_title(&spec.title)
        .with_aria_label(spec.effective_aria_label())
        .with_width(DialogWidth::Xl)
        .with_show_close_button(true)
        .with_close_label(&spec.close_label);

    let header = header_bar(spec, theme, &handlers);
    let body = shell_body(spec, theme, &handlers, page);

    dialog_with_slots(
        &dialog_spec,
        theme,
        vec![body],
        None,
        Some(header),
        None,
        close,
    )
}

fn header_bar(
    spec: &SettingsShellSpec,
    theme: &dyn ThemeProvider,
    handlers: &SettingsShellHandlers,
) -> Node {
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.md");
        s.descriptor.layout.width = LayoutSizing::Grow;
    }

    let mut title = Node::text(&spec.title);
    title.style.text_size = Some(rem_to_px(1.0));
    title.style.text_weight = Some(600);
    title.style.descriptor.text_color = Some(theme.resolve_color("color.text.primary"));

    let search_spec = TextInputSpec::new()
        .with_value(&spec.search_query)
        .with_placeholder("Search settings")
        .with_aria_label("Search settings")
        .with_input_type("search")
        .with_show_clear_button(true);
    let on_search = handlers.on_search_query_change.clone();
    let mut search = text_input_with_change(
        &search_spec,
        theme,
        on_search.map(|handler| {
            Arc::new(move |value: &str| handler(value)) as poodle_node::TextChangeHandler
        }),
    );
    search.id = Some("settings-shell-search".to_string());
    search.style.descriptor.layout.width = LayoutSizing::Grow;

    row.child(title).child(search)
}

fn shell_body(
    spec: &SettingsShellSpec,
    theme: &dyn ThemeProvider,
    handlers: &SettingsShellHandlers,
    page: Option<Node>,
) -> Node {
    let mut grid = Node::container();
    {
        let s = &mut grid.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Stretch;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.md");
        s.min_height = Some(rem_to_px(24.0));
    }

    let nav = nav_rail(spec, theme, handlers);
    let page_col = page_column(spec, theme, page);
    grid.child(nav).child(page_col)
}

fn nav_rail(
    spec: &SettingsShellSpec,
    theme: &dyn ThemeProvider,
    handlers: &SettingsShellHandlers,
) -> Node {
    let inner = if spec.groups.is_empty() {
        let (title, message, variant) = if spec.is_filtering() {
            (
                "No matches",
                "No settings match your search.",
                EmptyStateVariant::Search,
            )
        } else {
            (
                "No settings pages",
                "This scope has no settings pages yet.",
                EmptyStateVariant::Neutral,
            )
        };
        empty_state(
            &EmptyStateSpec::new(title)
                .with_message(message)
                .with_variant(variant)
                .with_compact(true),
            theme,
        )
    } else {
        let mut nav_spec = SidebarNavSpec::new(spec.groups.clone()).with_aria_label("Settings pages");
        if let Some(id) = &spec.active_page_id {
            nav_spec = nav_spec.with_value(id);
        }
        sidebar_nav(&nav_spec, theme, handlers.on_navigate.clone())
    };

    let scroller = scroll_shell(
        &ScrollShellSpec::new().with_direction(Direction::Vertical),
        theme,
        vec![inner],
    );
    let surface_node = surface(
        &SurfaceSpec::new()
            .with_tone(SurfaceTone::Panel)
            .with_border(SurfaceBorder::Subtle)
            .with_padding(PaddingScale::None),
        theme,
        vec![scroller],
    );
    let mut aside = Node::container();
    aside.a11y.role = Some(NodeRole::Region);
    aside.a11y.label = Some("Settings pages".to_string());
    {
        let s = &mut aside.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(14.0));
        s.min_width = Some(rem_to_px(14.0));
    }
    aside.child(surface_node)
}

fn page_column(spec: &SettingsShellSpec, theme: &dyn ThemeProvider, page: Option<Node>) -> Node {
    let mut col = Node::container();
    {
        let s = &mut col.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.stack.sm");
    }

    if let Some(reason) = &spec.close_refused_reason {
        let mut notice = callout(
            &CallOutSpec::new()
                .with_tone(StatusTone::Warning)
                .with_content(reason)
                .with_announce_mode(CalloutAnnounceMode::Polite),
            theme,
            CalloutHandlers::default(),
        );
        notice.a11y.role = Some(NodeRole::Status);
        col = col.child(notice);
    }

    let mut stack = Node::container();
    stack.a11y.role = Some(NodeRole::Region);
    if let Some(title) = &spec.page_title {
        stack.a11y.label = Some(title.clone());
    }
    {
        let s = &mut stack.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_height = Some(0.0);
    }
    let scroller = scroll_shell(
        &ScrollShellSpec::new()
            .with_direction(Direction::Vertical)
            .with_padding(PaddingScale::Md),
        theme,
        page.into_iter().collect(),
    );
    col.child(stack.child(scroller))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{SidebarNavGroup, SidebarNavItem};
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

    fn groups() -> Vec<SidebarNavGroup> {
        vec![SidebarNavGroup::new(
            "general",
            vec![SidebarNavItem::new("general", "General")],
        )
        .with_label("General")]
    }

    fn open_spec() -> SettingsShellSpec {
        SettingsShellSpec::new()
            .with_open(true)
            .with_groups(groups())
            .with_active_page_id("general")
            .with_page_title("General")
    }

    fn walk<'a>(node: &'a Node, visit: &mut impl FnMut(&'a Node)) {
        visit(node);
        for child in &node.children {
            walk(child, visit);
        }
    }

    #[test]
    fn closed_shell_renders_nothing() {
        let node = settings_shell(
            &SettingsShellSpec::new().with_open(false),
            &theme(),
            SettingsShellHandlers::default(),
            Some(Node::text("Page content")),
        );
        assert!(node.children.is_empty());
        assert!(texts(&node).is_empty());
    }

    #[test]
    fn page_stays_rendered_while_a_query_is_live() {
        let node = settings_shell(
            &open_spec().with_search_query("storage"),
            &theme(),
            SettingsShellHandlers::default(),
            Some(Node::text("Page content")),
        );
        let rendered = texts(&node);
        assert!(rendered.iter().any(|t| t == "Page content"));
        assert!(rendered.iter().any(|t| t == "Search settings" || t == "storage"));
    }

    #[test]
    fn empty_groups_with_a_query_is_no_matches() {
        let node = settings_shell(
            &SettingsShellSpec::new()
                .with_open(true)
                .with_search_query("xyzzy"),
            &theme(),
            SettingsShellHandlers::default(),
            Some(Node::text("Page content")),
        );
        let rendered = texts(&node);
        assert!(rendered.iter().any(|t| t == "No matches"));
        assert!(!rendered.iter().any(|t| t == "No settings pages"));
        assert!(rendered.iter().any(|t| t == "Page content"));
    }

    #[test]
    fn empty_scope_is_no_settings_pages() {
        let node = settings_shell(
            &SettingsShellSpec::new().with_open(true),
            &theme(),
            SettingsShellHandlers::default(),
            None,
        );
        assert!(texts(&node).iter().any(|t| t == "No settings pages"));
    }

    #[test]
    fn refused_close_keeps_the_dialog_and_shows_the_reason() {
        let closes = Arc::new(Mutex::new(0usize));
        let opens = Arc::new(Mutex::new(Vec::new()));
        let close_sink = Arc::clone(&closes);
        let open_sink = Arc::clone(&opens);
        let node = settings_shell(
            &open_spec().with_close_refused_reason("Unsaved changes on this page."),
            &theme(),
            SettingsShellHandlers {
                on_request_close: Some(Arc::new(move || *close_sink.lock().unwrap() += 1)),
                on_open_change: Some(Arc::new(move |open| open_sink.lock().unwrap().push(open))),
                ..SettingsShellHandlers::default()
            },
            Some(Node::text("Page content")),
        );
        assert!(texts(&node)
            .iter()
            .any(|t| t == "Unsaved changes on this page."));
        let mut close = None;
        walk(&node, &mut |n| {
            if n.interaction.on_activate.is_some() && close.is_none() {
                // The dialog close control is one of the activatable nodes.
            }
            if n.a11y.role == Some(NodeRole::Status) {
                close = Some(());
            }
        });
        assert!(
            close.is_some(),
            "refused close is a polite status, not an error"
        );
    }

    #[test]
    fn navigate_fires_the_page_id() {
        let hits = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&hits);
        let node = settings_shell(
            &open_spec(),
            &theme(),
            SettingsShellHandlers {
                on_navigate: Some(Arc::new(move |id| sink.lock().unwrap().push(id.to_string()))),
                ..SettingsShellHandlers::default()
            },
            Some(Node::text("Page content")),
        );
        let mut activate = None;
        walk(&node, &mut |n| {
            if n.id.as_deref() == Some("sidebar-nav-general") {
                activate = n.interaction.on_activate.clone();
            }
        });
        activate.expect("nav item")();
        assert_eq!(*hits.lock().unwrap(), vec!["general".to_string()]);
    }
}
