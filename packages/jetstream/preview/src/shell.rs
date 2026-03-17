//! Navigation shell — builds the UiTree structure for the preview app layout.
//!
//! Layout:
//! ```text
//! Root (Column, Grow×Grow, panel bg)
//! ├── TabBar (Row, Fixed height 44px, surface bg, bottom border)
//! │   ├── Title "Pug" (Label, bold 14px)
//! │   ├── Tab "Primitives" (Button, focusable)
//! │   ├── Tab "Composites" (Button, focusable)
//! │   ├── Tab "Demo" (Button, focusable)
//! │   └── Tab "Tokens" (Button, focusable)
//! ├── ContentArea (Row, Grow×Grow)
//! │   ├── Sidebar (Column, Fixed 224px, overflow Scroll, right border)
//! │   │   └── Items...
//! │   └── SpecimenArea (Column, Grow, padding 16px, overflow Scroll)
//! │       └── Placeholder label
//! ```

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::app_state::{AppState, ControlSize, Density, Section, ThemePreset};
use crate::component_registry;
use crate::specimens;
use crate::theme_bridge;

/// Node IDs for the key shell parts, so we can update them on state change.
pub struct ShellNodes {
    pub root: UiNodeId,
    pub tab_bar: UiNodeId,
    pub sidebar: UiNodeId,
    pub content: UiNodeId,
    /// Tab button nodes, in the same order as Section::ALL.
    pub tabs: Vec<UiNodeId>,
    /// Sidebar list item nodes, in the order of current section's components.
    pub sidebar_items: Vec<UiNodeId>,
    /// Theme toggle button nodes, in order of ThemePreset::ALL.
    pub theme_buttons: Vec<UiNodeId>,
    /// Density toggle button nodes, in order of Density::ALL.
    pub density_buttons: Vec<UiNodeId>,
    /// Size toggle button nodes, in order of ControlSize::ALL.
    pub size_buttons: Vec<UiNodeId>,
    /// State probe button nodes: [disabled, invalid, busy].
    pub probe_buttons: [UiNodeId; 3],
}

/// Build the entire shell tree. Returns the root node ID and tracked sub-nodes.
pub fn build_shell(
    tree: &mut UiTree,
    state: &AppState,
    theme: &dyn ThemeProvider,
) -> ShellNodes {
    let bg_panel = theme_bridge::panel_background(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let border = theme_bridge::border_subtle(theme);
    let accent = theme_bridge::accent_base(theme);

    // ── Root ──
    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        height: Sizing::Grow(1.0),
        background: Some(bg_panel),
        ..UiStyle::default()
    });
    tree.set_root(root);

    // ── Tab bar ──
    let tab_bar = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(44.0),
        padding: Edges::all(8.0),
        gap: 4.0,
        align: Align::Center,
        background: Some(bg_surface),
        border_color: Some(border),
        border_width: 1.0,
        ..UiStyle::default()
    });
    tree.add_child(root, tab_bar);

    // Title
    let title = tree.create(Widget::Label { text: "Pug".to_string() }, UiStyle {
        text_color: Some(text_primary),
        text_size: Some(14.0),
        padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 4.0 },
        ..UiStyle::default()
    });
    tree.add_child(tab_bar, title);

    // Tab buttons
    let mut tabs = Vec::new();
    for &section in Section::ALL {
        let is_active = section == state.section;
        let tab_bg = if is_active {
            Some(theme_bridge::tint(accent, 0.18))
        } else {
            None
        };
        let tab_text = if is_active { text_primary } else { text_secondary };
        let tab_border = if is_active {
            Some(theme_bridge::tint(accent, 0.56))
        } else {
            None
        };

        let tab = tree.create(Widget::Button {
            label: section.label().to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(28.0),
            padding: Edges { top: 0.0, right: 12.0, bottom: 0.0, left: 12.0 },
            corner_radii: [6.0; 4],
            background: tab_bg,
            border_color: tab_border,
            border_width: if is_active { 1.0 } else { 0.0 },
            text_color: Some(tab_text),
            text_size: Some(12.0),
            focusable: true,
            align: Align::Center,
            justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(tab_bar, tab);
        tabs.push(tab);
    }

    // ── Display controls bar ──
    let (theme_buttons, density_buttons, size_buttons, probe_buttons) =
        build_display_controls(tree, root, state, theme);

    // ── Content area ──
    let content_area = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        height: Sizing::Grow(1.0),
        ..UiStyle::default()
    });
    tree.add_child(root, content_area);

    // ── Sidebar ──
    let sidebar = tree.create(Widget::List { scroll_offset: 0.0 }, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(224.0),
        height: Sizing::Grow(1.0),
        padding: Edges { top: 4.0, right: 0.0, bottom: 4.0, left: 0.0 },
        gap: 1.0,
        overflow: Overflow::Scroll,
        background: Some(bg_surface),
        border_color: Some(border),
        border_width: 1.0,
        ..UiStyle::default()
    });
    tree.add_child(content_area, sidebar);

    // Sidebar items
    let sidebar_items = build_sidebar_items(tree, sidebar, state, theme);

    // ── Specimen area ──
    // Must be a List widget (not Panel) so that scroll_at() can find it.
    let content = tree.create(Widget::List { scroll_offset: 0.0 }, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        height: Sizing::Grow(1.0),
        padding: Edges::all(16.0),
        overflow: Overflow::Scroll,
        ..UiStyle::default()
    });
    tree.add_child(content_area, content);

    // Build content based on current state (specimen page, catalogue landing, or placeholder)
    let content_node = specimens::build_content(tree, state, theme);
    tree.add_child(content, content_node);

    ShellNodes {
        root,
        tab_bar,
        sidebar,
        content,
        tabs,
        sidebar_items,
        theme_buttons,
        density_buttons,
        size_buttons,
        probe_buttons,
    }
}

/// Build sidebar list items for the current section.
fn build_sidebar_items(
    tree: &mut UiTree,
    sidebar: UiNodeId,
    state: &AppState,
    theme: &dyn ThemeProvider,
) -> Vec<UiNodeId> {
    match state.section {
        Section::Demo => build_demo_sidebar_items(tree, sidebar, state, theme),
        _ => build_component_sidebar_items(tree, sidebar, state, theme),
    }
}

/// Build sidebar items for Primitives/Composites sections (component list).
fn build_component_sidebar_items(
    tree: &mut UiTree,
    sidebar: UiNodeId,
    state: &AppState,
    theme: &dyn ThemeProvider,
) -> Vec<UiNodeId> {
    let components = component_registry::components_for_section(state.section);
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let active_idx = state.active_component();

    let mut items = Vec::with_capacity(components.len());
    for (i, entry) in components.iter().enumerate() {
        let is_active = active_idx == Some(i);
        let item = build_sidebar_button(tree, sidebar, entry.display_name, is_active,
            text_primary, text_secondary, accent);
        items.push(item);
    }

    items
}

/// Build sidebar items for the Demo section (demo screen list).
fn build_demo_sidebar_items(
    tree: &mut UiTree,
    sidebar: UiNodeId,
    state: &AppState,
    theme: &dyn ThemeProvider,
) -> Vec<UiNodeId> {
    use crate::app_state::DemoScreen;

    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);

    let mut items = Vec::with_capacity(DemoScreen::ALL.len());
    for (_i, &screen) in DemoScreen::ALL.iter().enumerate() {
        let is_active = state.active_demo_screen == screen;
        let item = build_sidebar_button(tree, sidebar, screen.label(), is_active,
            text_primary, text_secondary, accent);
        items.push(item);
    }

    items
}

/// Create a single sidebar button with active/inactive styling.
fn build_sidebar_button(
    tree: &mut UiTree,
    sidebar: UiNodeId,
    label: &str,
    is_active: bool,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    accent: glam::Vec4,
) -> UiNodeId {
    let item_bg = if is_active {
        Some(theme_bridge::tint(accent, 0.14))
    } else {
        None
    };
    let item_text = if is_active { text_primary } else { text_secondary };

    let item = tree.create(Widget::Button {
        label: label.to_string(),
        pressed: false,
        hovered: false,
    }, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(32.0),
        padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 12.0 },
        align: Align::Center,
        background: item_bg,
        text_color: Some(item_text),
        text_size: Some(12.0),
        focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(sidebar, item);
    item
}

/// Determine which tab was clicked, given a node ID.
pub fn tab_index_for_node(shell: &ShellNodes, node: UiNodeId) -> Option<usize> {
    shell.tabs.iter().position(|&t| t == node)
}

/// Determine which sidebar item was clicked, given a node ID.
pub fn sidebar_index_for_node(shell: &ShellNodes, node: UiNodeId) -> Option<usize> {
    shell.sidebar_items.iter().position(|&t| t == node)
}

/// Determine which theme button was clicked.
pub fn theme_index_for_node(shell: &ShellNodes, node: UiNodeId) -> Option<usize> {
    shell.theme_buttons.iter().position(|&t| t == node)
}

/// Determine which density button was clicked.
pub fn density_index_for_node(shell: &ShellNodes, node: UiNodeId) -> Option<usize> {
    shell.density_buttons.iter().position(|&t| t == node)
}

/// Determine which size button was clicked.
pub fn size_index_for_node(shell: &ShellNodes, node: UiNodeId) -> Option<usize> {
    shell.size_buttons.iter().position(|&t| t == node)
}

/// Which state probe was clicked? Returns 0=disabled, 1=invalid, 2=busy.
pub fn probe_index_for_node(shell: &ShellNodes, node: UiNodeId) -> Option<usize> {
    shell.probe_buttons.iter().position(|&t| t == node)
}

// ── Display controls bar ──

/// Build the display controls bar with theme, density, size toggles and state probes.
/// Returns (theme_buttons, density_buttons, size_buttons, probe_buttons).
fn build_display_controls(
    tree: &mut UiTree,
    parent: UiNodeId,
    state: &AppState,
    theme: &dyn ThemeProvider,
) -> (Vec<UiNodeId>, Vec<UiNodeId>, Vec<UiNodeId>, [UiNodeId; 3]) {
    let text_secondary = theme_bridge::text_secondary(theme);
    let bg_surface = theme_bridge::surface_background(theme);
    let border = theme_bridge::border_subtle(theme);

    // Controls bar container
    let bar = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        height: Sizing::Fixed(56.0),
        padding: Edges { top: 8.0, right: 16.0, bottom: 8.0, left: 16.0 },
        gap: 24.0,
        align: Align::Center,
        background: Some(bg_surface),
        border_color: Some(border),
        border_width: 1.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, bar);

    // Theme group
    let theme_labels: Vec<&str> = ThemePreset::ALL.iter().map(|t| t.label()).collect();
    let theme_active = ThemePreset::ALL.iter().position(|&t| t == state.theme_preset).unwrap_or(0);
    let theme_buttons = build_toggle_group(tree, bar, "THEME", &theme_labels, theme_active, state, theme);

    // Density group
    let density_labels: Vec<&str> = Density::ALL.iter().map(|d| d.label()).collect();
    let density_active = Density::ALL.iter().position(|&d| d == state.density).unwrap_or(0);
    let density_buttons = build_toggle_group(tree, bar, "DENSITY", &density_labels, density_active, state, theme);

    // Size group
    let size_labels: Vec<&str> = ControlSize::ALL.iter().map(|s| s.label()).collect();
    let size_active = ControlSize::ALL.iter().position(|&s| s == state.control_size).unwrap_or(0);
    let size_buttons = build_toggle_group(tree, bar, "SIZE", &size_labels, size_active, state, theme);

    // Separator
    let sep = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(1.0),
        height: Sizing::Fixed(28.0),
        background: Some(border),
        ..UiStyle::default()
    });
    tree.add_child(bar, sep);

    // State probes
    let probes_group = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 4.0,
        ..UiStyle::default()
    });
    tree.add_child(bar, probes_group);

    let probes_label = tree.create(Widget::Label { text: "STATE".to_string() }, UiStyle {
        text_color: Some(text_secondary), text_size: Some(9.0),
        ..UiStyle::default()
    });
    tree.add_child(probes_group, probes_label);

    let probes_row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 12.0, align: Align::Center,
        ..UiStyle::default()
    });
    tree.add_child(probes_group, probes_row);

    let probe_disabled = build_probe_toggle(tree, probes_row, "disabled", state.disabled, theme);
    let probe_invalid = build_probe_toggle(tree, probes_row, "invalid", state.invalid, theme);
    let probe_busy = build_probe_toggle(tree, probes_row, "busy", state.busy, theme);

    (theme_buttons, density_buttons, size_buttons, [probe_disabled, probe_invalid, probe_busy])
}

/// Build a labeled toggle group (eyebrow label + row of buttons).
fn build_toggle_group(
    tree: &mut UiTree,
    parent: UiNodeId,
    label: &str,
    options: &[&str],
    active: usize,
    _state: &AppState,
    theme: &dyn ThemeProvider,
) -> Vec<UiNodeId> {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_subtle(theme);
    let bg_canvas = theme_bridge::canvas_background(theme);

    let group = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column, gap: 4.0,
        ..UiStyle::default()
    });
    tree.add_child(parent, group);

    // Eyebrow label
    let lbl = tree.create(Widget::Label { text: label.to_string() }, UiStyle {
        text_color: Some(text_secondary), text_size: Some(9.0),
        ..UiStyle::default()
    });
    tree.add_child(group, lbl);

    // Button row
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row, gap: 2.0,
        ..UiStyle::default()
    });
    tree.add_child(group, row);

    let mut buttons = Vec::with_capacity(options.len());
    for (i, &option) in options.iter().enumerate() {
        let is_active = i == active;
        let btn_bg = if is_active {
            Some(theme_bridge::tint(accent, 0.22))
        } else {
            Some(theme_bridge::tint(bg_canvas, 0.88))
        };
        let btn_border = if is_active {
            Some(theme_bridge::tint(accent, 0.56))
        } else {
            Some(border)
        };
        let btn_text = if is_active { text_primary } else { text_secondary };

        let btn = tree.create(Widget::Button {
            label: option.to_string(),
            pressed: false,
            hovered: false,
        }, UiStyle {
            height: Sizing::Fixed(26.0),
            padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
            corner_radii: [4.0; 4],
            background: btn_bg,
            border_color: btn_border,
            border_width: 1.0,
            text_color: Some(btn_text),
            text_size: Some(10.0),
            align: Align::Center,
            justify: Justify::Center,
            focusable: true,
            ..UiStyle::default()
        });
        tree.add_child(row, btn);
        buttons.push(btn);
    }

    buttons
}

/// Build a single state probe toggle (checkbox-style button).
fn build_probe_toggle(
    tree: &mut UiTree,
    parent: UiNodeId,
    label: &str,
    checked: bool,
    theme: &dyn ThemeProvider,
) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let accent = theme_bridge::accent_base(theme);
    let border = theme_bridge::border_subtle(theme);

    let indicator = if checked { "✓ " } else { "○ " };
    let display = format!("{}{}", indicator, label);
    let text_color = if checked { text_primary } else { text_secondary };
    let btn_bg = if checked {
        Some(theme_bridge::tint(accent, 0.14))
    } else {
        None
    };
    let btn_border = if checked { Some(accent) } else { Some(border) };

    let btn = tree.create(Widget::Button {
        label: display,
        pressed: false,
        hovered: false,
    }, UiStyle {
        height: Sizing::Fixed(24.0),
        padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 6.0 },
        corner_radii: [4.0; 4],
        background: btn_bg,
        border_color: btn_border,
        border_width: 1.0,
        text_color: Some(text_color),
        text_size: Some(10.0),
        align: Align::Center,
        justify: Justify::Center,
        focusable: true,
        ..UiStyle::default()
    });
    tree.add_child(parent, btn);
    btn
}
