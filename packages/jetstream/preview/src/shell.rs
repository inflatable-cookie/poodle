//! Navigation shell — pure function building the preview app layout as JsEl.
//!
//! Layout:
//! ```text
//! Root (Column, Grow)
//! ├── TabBar (Row, h=44, surface bg, bottom border)
//! │   ├── "Poodle" title
//! │   └── Tab buttons...
//! ├── ControlsBar (Row, h=56, surface bg, bottom border)
//! │   ├── Theme group
//! │   ├── Density group
//! │   ├── Size group
//! │   └── State probes
//! └── ContentArea (Row, Grow)
//!     ├── Sidebar (List, w=224, overflow scroll)
//!     │   └── Items...
//!     └── SpecimenArea (List, Grow, overflow scroll)
//!         └── Content...
//! ```

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use jetstream_runtime::game_ui::Color;
use poodle_jetstream_components::theme_ext::*;

use crate::app_state::*;
use crate::component_registry;
use crate::specimens;

/// Build the entire shell as a pure JsEl tree.
pub fn build_shell(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
    let bg_panel = resolve_color(theme, "color.background.panel");
    let bg_surface = resolve_color(theme, "color.background.surface");
    let border = resolve_color(theme, "color.border.subtle");

    div().flex_col().grow().bg(bg_panel)
        .child(build_tab_bar(state, theme, bg_surface, border))
        .child(build_controls_bar(state, theme, bg_surface, border))
        .child(
            div().flex_row().grow()
                .child(build_sidebar(state, theme, bg_surface, border))
                .child(build_content_area(state, theme))
        )
}

/// Tab bar with section navigation.
fn build_tab_bar(
    state: &AppState,
    theme: &JetstreamThemeProvider,
    bg_surface: glam::Vec4,
    border: glam::Vec4,
) -> JsEl {
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");

    let mut bar = div().flex_row().w_full().h(44.0).px(8.0).gap(4.0)
        .items_center()
        .bg(bg_surface).border_1().border_color(border)
        .child(
            label("Poodle").text_color(text_primary).text_size(14.0).pr(12.0).pl(4.0)
        );

    for &section in Section::ALL {
        let is_active = section == state.section;
        let tab_bg: Option<Color> = if is_active { Some(tint(accent, 0.18).into()) } else { None };
        let tab_text = if is_active { text_primary } else { text_secondary };
        let tab_border: Option<Color> = if is_active { Some(tint(accent, 0.56).into()) } else { None };

        bar = bar.child(
            button(section.label())
                .id(format!("tab:{}", section.label()))
                .h(28.0).px(12.0)
                .items_center().justify_center()
                .rounded(6.0)
                .bg_opt(tab_bg)
                .border(if is_active { 1.0 } else { 0.0 })
                .border_color_opt(tab_border)
                .text_color(tab_text).text_size(12.0)
                .focusable()
        );
    }

    bar
}

/// Controls bar with theme, density, size toggles and state probes.
fn build_controls_bar(
    state: &AppState,
    theme: &JetstreamThemeProvider,
    bg_surface: glam::Vec4,
    border: glam::Vec4,
) -> JsEl {
    let text_secondary = resolve_color(theme, "color.text.secondary");

    let mut bar = div().flex_row().w_full().h(56.0).px(16.0).gap(24.0)
        .items_center()
        .bg(bg_surface).border_1().border_color(border);

    // Theme group
    let theme_labels: Vec<&str> = ThemePreset::ALL.iter().map(|t| t.label()).collect();
    let theme_active = ThemePreset::ALL.iter().position(|&t| t == state.theme_preset).unwrap_or(0);
    bar = bar.child(build_toggle_group("THEME", "theme", &theme_labels, theme_active, theme));

    // Density group
    let density_labels: Vec<&str> = Density::ALL.iter().map(|d| d.label()).collect();
    let density_active = Density::ALL.iter().position(|&d| d == state.density).unwrap_or(0);
    bar = bar.child(build_toggle_group("DENSITY", "density", &density_labels, density_active, theme));

    // Size group
    let size_labels: Vec<&str> = ControlSize::ALL.iter().map(|s| s.label()).collect();
    let size_active = ControlSize::ALL.iter().position(|&s| s == state.control_size).unwrap_or(0);
    bar = bar.child(build_toggle_group("SIZE", "size", &size_labels, size_active, theme));

    // Contrast group — a real engine slider driving the oklch neutral-contrast
    // axis (mirrors the Svelte preview's CONTRAST control; 0.5 = library default).
    let contrast_group = div().flex_col().gap(4.0)
        .child(label("CONTRAST").text_color(text_secondary).text_size(9.0))
        .child(
            slider(state.contrast, 0.0, 1.0)
                .id("contrast")
                .w(120.0).h(16.0)
                .focusable(),
        );
    bar = bar.child(contrast_group);

    // Separator
    bar = bar.child(div().w(1.0).h(28.0).bg(border));

    // State probes
    let probes = div().flex_col().gap(4.0)
        .child(label("STATE").text_color(text_secondary).text_size(9.0))
        .child(
            div().flex_row().gap(12.0).items_center()
                .child(build_probe_toggle("disabled", state.disabled, theme))
                .child(build_probe_toggle("invalid", state.invalid, theme))
                .child(build_probe_toggle("busy", state.busy, theme))
        );
    bar = bar.child(probes);

    bar
}

/// A labeled toggle group (eyebrow label + row of buttons).
fn build_toggle_group(
    eyebrow: &str,
    id_prefix: &str,
    options: &[&str],
    active: usize,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let border = resolve_color(theme, "color.border.subtle");
    let bg_canvas = resolve_color(theme, "color.background.canvas");

    let mut row = div().flex_row().gap(2.0);
    for (i, &option) in options.iter().enumerate() {
        let is_active = i == active;
        let btn_bg = if is_active { tint(accent, 0.22) } else { tint(bg_canvas, 0.88) };
        let btn_border = if is_active { tint(accent, 0.56) } else { border };
        let btn_text = if is_active { text_primary } else { text_secondary };

        row = row.child(
            button(option)
                .id(format!("{id_prefix}:{option}"))
                .h(26.0).px(8.0)
                .items_center().justify_center()
                .rounded(4.0)
                .bg(btn_bg).border_1().border_color(btn_border)
                .text_color(btn_text).text_size(10.0)
                .focusable()
        );
    }

    div().flex_col().gap(4.0)
        .child(label(eyebrow).text_color(text_secondary).text_size(9.0))
        .child(row)
}

/// A single state probe toggle (checkbox-style button).
fn build_probe_toggle(name: &str, checked: bool, theme: &JetstreamThemeProvider) -> JsEl {
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    let border = resolve_color(theme, "color.border.subtle");

    let indicator = if checked { "✓ " } else { "○ " };
    let display = format!("{indicator}{name}");
    let text_color = if checked { text_primary } else { text_secondary };
    let btn_bg: Option<Color> = if checked { Some(tint(accent, 0.14).into()) } else { None };
    let btn_border = if checked { accent } else { border };

    button(&display)
        .id(format!("probe:{name}"))
        .h(24.0).px(6.0)
        .items_center().justify_center()
        .rounded(4.0)
        .bg_opt(btn_bg).border_1().border_color(btn_border)
        .text_color(text_color).text_size(10.0)
        .focusable()
}

/// Sidebar with component/demo list.
fn build_sidebar(
    state: &AppState,
    theme: &JetstreamThemeProvider,
    bg_surface: glam::Vec4,
    border: glam::Vec4,
) -> JsEl {
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");

    let mut sidebar = div().flex_col().w(224.0).flex_shrink_0().self_stretch()
        .py(4.0).gap(1.0)
        .overflow_scroll()
        .id("sidebar")
        .bg(bg_surface).border_1().border_color(border);

    match state.section {
        Section::Demo => {
            for (i, &screen) in DemoScreen::ALL.iter().enumerate() {
                let is_active = state.active_demo_screen == screen;
                sidebar = sidebar.child(build_sidebar_item(
                    screen.label(), &format!("sidebar:{i}"), is_active,
                    text_primary, text_secondary, accent,
                ));
            }
        }
        _ => {
            // Tag-grouped list mirroring the Svelte preview sidebar: a group
            // heading (CONTROLS, INPUTS, …) whenever the tag changes — entries
            // are pre-ordered by (tag order, name) in the registry.
            let components = component_registry::ALL_COMPONENTS;
            let active_idx = state.active_component();
            let mut current_tag = None;
            for (i, entry) in components.iter().enumerate() {
                if current_tag != Some(entry.tag) {
                    current_tag = Some(entry.tag);
                    sidebar = sidebar.child(
                        label(entry.tag.label())
                            .px(12.0)
                            .pt(if i == 0 { 8.0 } else { 16.0 })
                            .pb(4.0)
                            .text_color(accent)
                            .text_size(10.0)
                            .text_weight(700)
                            .letter_spacing_em(0.08),
                    );
                }
                let is_active = active_idx == Some(i);
                sidebar = sidebar.child(build_sidebar_item(
                    entry.display_name, &format!("sidebar:{i}"), is_active,
                    text_primary, text_secondary, accent,
                ));
            }
        }
    }

    sidebar
}

/// Single sidebar item button.
fn build_sidebar_item(
    name: &str,
    id: &str,
    is_active: bool,
    text_primary: glam::Vec4,
    text_secondary: glam::Vec4,
    accent: glam::Vec4,
) -> JsEl {
    let item_bg: Option<Color> = if is_active { Some(tint(accent, 0.14).into()) } else { None };
    let item_text = if is_active { text_primary } else { text_secondary };

    button(name)
        .id(id)
        .flex_row().h(32.0).self_stretch().px(12.0)
        .items_center().justify_start()
        .bg_opt(item_bg)
        .text_color(item_text).text_size(12.0)
        .focusable()
}

/// Content area with specimen or landing page.
fn build_content_area(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
    div().flex_col().grow().p(16.0)
        .overflow_scroll()
        .id("content")
        .child(specimens::build_content(state, theme))
}

// ── Interaction helpers ──

/// Parse a clicked node's token_key (set from JsEl.id) to determine what
/// action to take. Returns a `ShellAction` describing the intent.
pub enum ShellAction {
    SelectTab(usize),
    SelectSidebarItem(usize),
    SelectTheme(usize),
    SelectDensity(usize),
    SelectSize(usize),
    ToggleProbe(usize), // 0=disabled, 1=invalid, 2=busy
    TreeSelect(String),
    TreeToggle(String),
    TreeCheck(String),
    TreeMenu(String),
    None,
}

/// Parse a node's token_key into a ShellAction.
pub fn parse_action(token_key: Option<&str>) -> ShellAction {
    let key = match token_key {
        Some(k) => k,
        None => return ShellAction::None,
    };

    if let Some(tab_name) = key.strip_prefix("tab:") {
        if let Some(idx) = Section::ALL.iter().position(|s| s.label() == tab_name) {
            return ShellAction::SelectTab(idx);
        }
    }
    if let Some(idx_str) = key.strip_prefix("sidebar:") {
        if let Ok(idx) = idx_str.parse::<usize>() {
            return ShellAction::SelectSidebarItem(idx);
        }
    }
    if let Some(name) = key.strip_prefix("theme:") {
        if let Some(idx) = ThemePreset::ALL.iter().position(|t| t.label() == name) {
            return ShellAction::SelectTheme(idx);
        }
    }
    if let Some(name) = key.strip_prefix("density:") {
        if let Some(idx) = Density::ALL.iter().position(|d| d.label() == name) {
            return ShellAction::SelectDensity(idx);
        }
    }
    if let Some(name) = key.strip_prefix("size:") {
        if let Some(idx) = ControlSize::ALL.iter().position(|s| s.label() == name) {
            return ShellAction::SelectSize(idx);
        }
    }
    if let Some(action) = key.strip_prefix("tree-menu:") {
        return ShellAction::TreeMenu(action.to_string());
    }
    if let Some(value) = key.strip_prefix("tree-check:") {
        return ShellAction::TreeCheck(value.to_string());
    }
    if let Some(value) = key.strip_prefix("tree-twisty:") {
        return ShellAction::TreeToggle(value.to_string());
    }
    if let Some(value) = key.strip_prefix("tree:") {
        return ShellAction::TreeSelect(value.to_string());
    }
    if let Some(probe_name) = key.strip_prefix("probe:") {
        let idx = match probe_name {
            "disabled" => 0,
            "invalid" => 1,
            "busy" => 2,
            _ => return ShellAction::None,
        };
        return ShellAction::ToggleProbe(idx);
    }

    ShellAction::None
}
