mod action_discovery_panel;
mod app_header;
mod command_palette;
mod command_palette_shell;
mod dock_region;
mod panel_header;
mod panel_surface;
mod panel_tabs;
mod persistence;
mod project_header;
mod shell_status_bar;
mod split_view;
mod surface_tabs;
mod types;
mod workspace_shell;

pub use action_discovery_panel::ActionDiscoveryPanelSpec;
pub use app_header::AppHeaderSpec;
pub use command_palette::CommandPaletteSpec;
pub use command_palette_shell::CommandPaletteShellSpec;
pub use dock_region::{DockRegionSpec, DockTabsPlacement};
pub use panel_header::PanelHeaderSpec;
pub use panel_surface::{PanelBodyPadding, PanelScrollMode, PanelSurfaceSpec};
pub use panel_tabs::PanelTabsSpec;
pub use persistence::{parse_workspace_layout_snapshot, serialize_workspace_layout_snapshot};
pub use project_header::ProjectHeaderSpec;
pub use shell_status_bar::ShellStatusBarSpec;
pub use split_view::SplitViewSpec;
pub use surface_tabs::SurfaceTabsSpec;
pub use types::{
    ActionDiscoverySection, CommandActionItem, DiscoveryState, DockEdge, DockRegionSnapshot,
    PanelTabItem, SplitOrientation, SurfaceTabItem, WorkspaceLayoutSnapshot, WorkspaceShellState,
};
pub use workspace_shell::WorkspaceShellSpec;

pub const CURRENT_GENERATION: &str = "g04.009";
pub const WORKSTATION_EXPORTS: &[&str] = &[
    "ActionDiscoveryPanelSpec",
    "AppHeaderSpec",
    "CommandPaletteShellSpec",
    "CommandPaletteSpec",
    "DockRegionSpec",
    "PanelHeaderSpec",
    "PanelSurfaceSpec",
    "PanelTabsSpec",
    "ProjectHeaderSpec",
    "ShellStatusBarSpec",
    "SplitViewSpec",
    "SurfaceTabsSpec",
    "WorkspaceShellSpec",
];

#[cfg(test)]
mod tests {
    use pug_gpui_primitives::OverlayPlacement;
    use pug_gpui_tokens::semantic;

    use super::{
        parse_workspace_layout_snapshot, serialize_workspace_layout_snapshot,
        ActionDiscoveryPanelSpec, ActionDiscoverySection, AppHeaderSpec, CommandActionItem,
        CommandPaletteShellSpec, CommandPaletteSpec, DiscoveryState, DockEdge, DockRegionSnapshot,
        DockRegionSpec, DockTabsPlacement, PanelBodyPadding, PanelHeaderSpec, PanelScrollMode,
        PanelSurfaceSpec, PanelTabItem, PanelTabsSpec, ProjectHeaderSpec, ShellStatusBarSpec,
        SplitOrientation, SplitViewSpec, SurfaceTabItem, SurfaceTabsSpec, WorkspaceLayoutSnapshot,
        WorkspaceShellSpec, WorkspaceShellState,
    };

    #[test]
    fn app_and_project_headers_preserve_shell_identity_state() {
        let app = AppHeaderSpec::new()
            .with_title("Pug")
            .with_drag_region(true)
            .with_primary_action_count(2)
            .with_utility_item_count(1);
        let project = ProjectHeaderSpec::new("Aura mix review")
            .with_dirty(true)
            .with_subtitle("loophole-studio")
            .with_action_count(3);

        assert!(app.is_utility_heavy());
        assert_eq!(app.background_token(), semantic::COLOR_BACKGROUND_PANEL);
        assert!(project.is_descriptive());
        assert_eq!(project.dirty_color_token(), semantic::COLOR_STATUS_WARNING);
    }

    #[test]
    fn panel_surface_header_and_tabs_preserve_dense_shell_posture() {
        let surface = PanelSurfaceSpec::new()
            .with_title("Review")
            .with_active(true)
            .with_elevated(true)
            .with_body_padding(PanelBodyPadding::Sm)
            .with_scroll_mode(PanelScrollMode::Content);
        let header = PanelHeaderSpec::new()
            .with_title("Review")
            .with_active(true)
            .with_collapsible(true)
            .with_tabs(true)
            .with_utility_action_count(2);
        let tabs = PanelTabsSpec::new(vec![
            PanelTabItem::new("review", "Review"),
            PanelTabItem::new("history", "History").with_closable(true),
        ])
        .with_default_value("history")
        .with_reorderable(true);

        assert_eq!(
            surface.resolved_padding_scale().panel_inset().horizontal,
            Some(semantic::SPACE_INLINE_SM)
        );
        assert_eq!(surface.elevation_token(), semantic::ELEVATION_OVERLAY);
        assert_eq!(
            header.background_token(),
            semantic::COLOR_BACKGROUND_SURFACE
        );
        assert_eq!(tabs.current_value(), Some("history"));
        assert_eq!(tabs.closable_item_count(), 1);
    }

    #[test]
    fn surface_tabs_and_dock_region_preserve_shell_selection_and_collapse() {
        let surface_tabs = SurfaceTabsSpec::new(vec![
            SurfaceTabItem::new("browse", "Browse"),
            SurfaceTabItem::new("review", "Review").with_closable(true),
        ])
        .with_default_value("browse")
        .with_add_enabled(true);
        let dock = DockRegionSpec::new(
            DockEdge::Left,
            vec![
                PanelTabItem::new("inspector", "Inspector"),
                PanelTabItem::new("history", "History"),
            ],
        )
        .with_tabs_placement(DockTabsPlacement::Top)
        .with_collapsed(true)
        .with_value("history");

        assert_eq!(surface_tabs.current_value(), Some("browse"));
        assert_eq!(surface_tabs.closable_item_count(), 1);
        assert_eq!(dock.current_value(), Some("history"));
        assert!(!dock.is_empty());
        assert_eq!(dock.strip_fill_token(), semantic::COLOR_BACKGROUND_SURFACE);
    }

    #[test]
    fn split_view_and_workspace_shell_hold_layout_state() {
        let split = SplitViewSpec::new(SplitOrientation::Horizontal)
            .with_default_ratio(0.6)
            .with_primary_collapsed(false)
            .with_secondary_collapsed(false);
        let shell = WorkspaceShellSpec::new()
            .with_state(WorkspaceShellState::Offline)
            .with_app_header(true)
            .with_project_header(true)
            .with_surface_tabs(true)
            .with_utility_overlays(true)
            .with_active_surface_label("Review");

        assert_eq!(split.current_ratio(), 0.6);
        assert!(split.keyboard_resize_supported());
        assert!(shell.is_multi_surface());
        assert!(shell.is_utility_open());
        assert_eq!(shell.background_token(), semantic::COLOR_BACKGROUND_CANVAS);
    }

    #[test]
    fn command_palette_and_discovery_panel_preserve_grouped_action_posture() {
        let actions = vec![
            CommandActionItem::new("open-palette", "Open palette")
                .with_group("Navigation")
                .with_shortcut("Cmd+K"),
            CommandActionItem::new("rerun-validation", "Rerun validation")
                .with_group("Workspace")
                .with_badge("recent"),
        ];
        let shell = CommandPaletteShellSpec::new().with_default_open(true);
        let palette = CommandPaletteSpec::new(actions.clone())
            .with_query("open")
            .with_state(DiscoveryState::Ready)
            .with_active_action_id("open-palette");
        let discovery = ActionDiscoveryPanelSpec::new(vec![ActionDiscoverySection::new(
            "navigation",
            "Navigation",
            actions,
        )])
        .with_state(DiscoveryState::Ready);

        assert!(shell.current_open());
        assert_eq!(
            shell.backdrop_fill_token(),
            semantic::COLOR_BACKGROUND_OVERLAY
        );
        assert_eq!(
            palette.active_action().map(|action| action.id.as_str()),
            Some("open-palette")
        );
        assert_eq!(palette.grouped_action_count(), 2);
        assert_eq!(palette.placement, OverlayPlacement::BottomStart);
        assert_eq!(discovery.section_count(), 1);
        assert_eq!(discovery.action_count(), 2);
    }

    #[test]
    fn shell_status_and_layout_snapshot_round_trip() {
        let status_bar = ShellStatusBarSpec::new()
            .with_summary("4 validations pending")
            .with_leading_item_count(2)
            .with_trailing_item_count(2);
        let snapshot = WorkspaceLayoutSnapshot {
            version: 1,
            active_surface: String::from("review"),
            surface_order: vec![String::from("browse"), String::from("review")],
            primary_split_ratio: 0.62,
            secondary_split_ratio: 0.38,
            left_dock: DockRegionSnapshot {
                edge: DockEdge::Left,
                is_collapsed: false,
                active_panel: Some(String::from("inspector")),
                order: vec![String::from("inspector"), String::from("history")],
            },
            right_dock: DockRegionSnapshot {
                edge: DockEdge::Right,
                is_collapsed: true,
                active_panel: Some(String::from("metadata")),
                order: vec![String::from("metadata")],
            },
        };

        let serialized = serialize_workspace_layout_snapshot(&snapshot).expect("serialize layout");
        let parsed = parse_workspace_layout_snapshot(&serialized).expect("parse layout");

        assert!(status_bar.is_dense());
        assert_eq!(
            status_bar.background_token(),
            semantic::COLOR_BACKGROUND_SURFACE
        );
        assert_eq!(parsed, snapshot);
    }
}
