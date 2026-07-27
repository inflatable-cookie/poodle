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

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use jetstream_ui::Color;
use poodle_jetstream_components::theme_ext::*;

use poodle_jetstream_components::eyebrow::js_eyebrow;
use poodle_jetstream_components::pill::js_pill;
use poodle_jetstream_components::tabs::js_tabs;
use poodle_jetstream_components::toggle_group::js_toggle_group;
use poodle_specs::{
    EyebrowSpec, PillSpec, TabDefinition, TabVariant, TabsSpec, ToggleGroupOption,
    ToggleGroupSpec,
};

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

    let _ = (text_secondary, accent);
    // Mirror the Svelte top bar: title, Poodle pill-variant Tabs for section
    // navigation, and summary Pills (theme/density/size) on the right —
    // dogfooding the real components instead of hand-rolled chrome.
    let nav = js_tabs(
        &TabsSpec::new(vec![
            TabDefinition::new("components", "Components"),
            TabDefinition::new("demo", "Demo"),
            TabDefinition::new("tokens", "Tokens"),
        ])
        .with_variant(TabVariant::Pill)
        .with_size(poodle_specs::ControlSize::Sm)
        .with_value(match state.section {
            Section::Components => "components",
            Section::Demo => "demo",
            Section::Tokens => "tokens",
        })
        // A tab list is announced as a group of tabs, so without a name it is
        // "tab group" and nothing else — and this preview has more than one.
        .with_aria_label("Preview sections"),
        theme,
    );

    let summary = div().flex_row().gap(8.0).items_center()
        .child(js_pill(&PillSpec::new().with_label(state.theme_preset.label()), theme))
        .child(js_pill(&PillSpec::new().with_label(state.density.label()), theme))
        .child(js_pill(&PillSpec::new().with_label(state.control_size.label()), theme));

    div().flex_row().w_full().px(12.0).py(8.0).gap(16.0)
        .items_center()
        .bg(bg_surface).border_1().border_color(border)
        .child(label("Poodle").text_color(text_primary).text_size(15.0).text_weight(700).pr(8.0).pl(4.0))
        .child(nav)
        .child(div().grow())
        .child(summary)
}

/// Controls bar with theme, density, size toggles and state probes.
fn build_controls_bar(
    state: &AppState,
    theme: &JetstreamThemeProvider,
    bg_surface: glam::Vec4,
    border: glam::Vec4,
) -> JsEl {
    let text_secondary = resolve_color(theme, "color.text.secondary");

    let mut bar = div().flex_row().w_full().px(16.0).py(10.0).gap(20.0)
        .items_center()
        .bg(bg_surface).border_1().border_color(border);

    // Theme / density / size — real Eyebrow + ToggleGroup components (the
    // Svelte DisplayControls uses exactly these). Option activation routes via
    // the components' `toggle:{value}` ids.
    let group = |eyebrow: &str, options: &[&str], active: &str| -> JsEl {
        let opts = options
            .iter()
            .map(|&o| ToggleGroupOption::new(o, o))
            .collect();
        div().flex_col().gap(4.0)
            .flex_shrink_0()
            .child(js_eyebrow(&EyebrowSpec::default().with_content(eyebrow), theme))
            .child(js_toggle_group(
                &ToggleGroupSpec::new(opts)
                    .with_value(vec![active.to_string()])
                    .with_size(poodle_specs::ControlSize::Xs),
                theme,
            ))
    };

    let theme_labels: Vec<&str> = ThemePreset::ALL.iter().map(|t| t.label()).collect();
    bar = bar.child(group("Theme", &theme_labels, state.theme_preset.label()));

    let density_labels: Vec<&str> = Density::ALL.iter().map(|d| d.label()).collect();
    bar = bar.child(group("Density", &density_labels, state.density.label()));

    let size_labels: Vec<&str> = ControlSize::ALL.iter().map(|s| s.label()).collect();
    bar = bar.child(group("Size", &size_labels, state.control_size.label()));

    // Contrast group — a real engine slider driving the oklch neutral-contrast
    // axis (mirrors the Svelte preview's CONTRAST control; 0.5 = library default).
    let contrast_group = div().flex_col().gap(4.0)
        .flex_shrink_0()
        .child(label("CONTRAST").text_color(text_secondary).text_size(9.0))
        .child(
            slider(state.contrast, 0.0, 1.0)
                .id("contrast")
                .w(140.0).h(14.0)
                .rounded(999.0)
                // The "CONTRAST" caption beside it is a sibling, not a label,
                // so a screen reader reaches the slider with nothing to say.
                .aria_label("Contrast")
                .focusable(),
        );
    bar = bar.child(contrast_group);

    // Search group — filters the sidebar + catalogue by name (Svelte parity).
    let bg_canvas = resolve_color(theme, "color.background.canvas");
    let search_group = div().flex_col().gap(4.0)
        .flex_shrink_0()
        .child(label("SEARCH").text_color(text_secondary).text_size(9.0))
        .child(
            div().flex_row().items_center().gap(6.0)
                .h(28.0).pl(8.0).pr(8.0)
                .bg(bg_canvas)
                .border_1().border_color(border)
                .rounded(8.0)
                // Decorative: the field beside it is already named "Search",
                // so announcing this too would just repeat the word.
                .child(
                    icon("search")
                        .w(13.0)
                        .h(13.0)
                        .text_color(text_secondary)
                        .aria_hidden(true),
                )
                .child(
                    text_input(state.search.clone(), "Find component...")
                        .id("search")
                        .aria_label("Search components")
                        .w(160.0)
                        .h(24.0)
                        .bg(Color::TRANSPARENT)
                        .text_size(11.5),
                ),
        );
    bar = bar.child(search_group);

    // Separator
    bar = bar.child(div().w(1.0).h(28.0).bg(border));

    bar
}

/// State-probe toggles (disabled/invalid/busy) — shown on the specimen page,
/// not in the global bar (the Svelte preview has no global STATE group).
pub(crate) fn build_state_probes(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
    div().flex_row().gap(12.0).items_center()
        .child(build_probe_toggle("disabled", state.disabled, theme))
        .child(build_probe_toggle("invalid", state.invalid, theme))
        .child(build_probe_toggle("busy", state.busy, theme))
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
                if !state.matches_search(entry.display_name) {
                    continue;
                }
                if current_tag != Some(entry.tag) {
                    current_tag = Some(entry.tag);
                    sidebar = sidebar.child(
                        label(entry.tag.label().to_uppercase())
                            .px(14.0)
                            .pt(if i == 0 { 12.0 } else { 20.0 })
                            .pb(6.0)
                            .text_color(accent)
                            .text_size(10.0)
                            .text_weight(700)
                            .letter_spacing_em(0.1),
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
        .flex_row().h(34.0).self_stretch().px(14.0)
        .items_center().justify_start()
        .rounded(6.0)
        .bg(item_bg.map(Color::from).unwrap_or(Color::TRANSPARENT))
        .text_color(item_text).text_size(13.0)
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
