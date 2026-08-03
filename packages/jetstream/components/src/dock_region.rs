//! DockRegion — Jetstream dock region backed by DockRegionSpec.
use jetstream_ui::ui_element::{self, BorderStyle, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{DockCollapsedPosture, DockEdge, DockEmphasis, DockRegionSpec, DockSizing};

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_px, tint};

/// DockRegion — a dockable panel with tabs.
///
/// Mirrors the GPUI target's names: `on_tab_change` and `on_collapse_toggle`.
///
/// The drag events (`onDragStart`, reorder, panel drop) are drag-with-payload
/// gestures the runtime does not carry; recorded as a delta.
pub struct DockRegion {
    spec: DockRegionSpec,
    theme: JetstreamThemeProvider,
    content: Option<JsEl>,
    on_tab_change: Option<crate::element::Handler>,
    on_collapse_toggle: Option<crate::element::ToggleHandler>,
}

impl DockRegion {
    pub fn from_spec(spec: DockRegionSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            content: None,
            on_tab_change: None,
            on_collapse_toggle: None,
        }
    }

    pub fn content(mut self, content: impl crate::element::IntoJsEl) -> Self {
        self.content = Some(content.into_js_el());
        self
    }

    /// Fires with the tab's value when one is pressed.
    pub fn on_tab_change(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_tab_change = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires with the collapsed state the region is moving **to**.
    pub fn on_collapse_toggle(mut self, handler: impl Fn(bool) + Send + Sync + 'static) -> Self {
        self.on_collapse_toggle = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for DockRegion {
    fn into_js_el(self) -> JsEl {
        build(
            &self.spec,
            &self.theme,
            self.content,
            self.on_tab_change,
            self.on_collapse_toggle,
        )
    }
}

pub fn js_dock_region(
    spec: &DockRegionSpec,
    theme: &JetstreamThemeProvider,
    content: Option<JsEl>,
) -> JsEl {
    build(spec, theme, content, None, None)
}

fn build(
    spec: &DockRegionSpec,
    theme: &JetstreamThemeProvider,
    content: Option<JsEl>,
    on_tab_change: Option<crate::element::Handler>,
    on_collapse_toggle: Option<crate::element::ToggleHandler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let tab_font = rem_to_px(size_font_rem(effective_size));
    // Density → spacing (contract: density controls horizontal padding / gaps).
    let space_x = rem_to_px(control_space_x_rem(spec.density));
    let space_y = rem_to_px(panel_space_y_rem(spec.density));
    let tab_gap = space_x * 0.5;
    let border_w = resolve_px(theme, "border.width.default");

    let fill = resolve_color(theme, spec.strip_fill_token());
    let panel_fill = resolve_color(theme, "color.background.panel");
    let border_subtle = resolve_color(theme, "color.border.subtle");
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_muted = resolve_color(theme, "color.text.secondary");
    let accent = resolve_color(theme, "color.accent.base");
    // Active-tab fill: accent mixed into the strip fill (matches GPUI's
    // `accent.opacity(0.10)` and the Svelte active-tab tint). TOKEN GAP: there
    // is no semantic opacity token for the selected-tab tint strength, so the
    // ratio is a single named contract-exact constant rather than a raw literal
    // scattered inline.
    const ACTIVE_TAB_ACCENT_RATIO: f32 = 0.10;
    let active_bg = color_mix(accent, fill, ACTIVE_TAB_ACCENT_RATIO);

    let is_side_edge = matches!(spec.edge, DockEdge::Left | DockEdge::Right);
    let active = spec.current_value().map(|s| s.to_string());

    // ── Root emphasis treatment ────────────────────────────────
    // Standard: panel fill + subtle border. Quiet: transparent. Strong: accent
    // mixed 32% into the subtle border.
    let (root_bg, root_border) = match spec.emphasis {
        DockEmphasis::Standard => (panel_fill, border_subtle),
        DockEmphasis::Quiet => (tint(panel_fill, 0.0), tint(border_subtle, 0.0)),
        DockEmphasis::Strong => (panel_fill, color_mix(accent, border_subtle, 0.32)),
    };

    // Build a single tab. `compact` → icon-only (label suppressed); `vertical`
    // → full-width stacked entry.
    let build_tab =
        |value: &str, label: &str, icon: Option<&str>, compact: bool, vertical: bool| -> JsEl {
            let is_active = active.as_deref() == Some(value);
            let text_color = if is_active { text_primary } else { text_muted };

            // Icon-only compact / icon-strip tabs render the icon glyph; otherwise
            // the label text. When an icon exists it is shown alongside the label.
            let display = if compact {
                icon.unwrap_or(label).to_string()
            } else if let Some(ic) = icon {
                format!("{ic} {label}")
            } else {
                label.to_string()
            };

            let mut tab_btn = ui_element::button(&display)
                .id(format!("dock-tab-{value}"))
                .text_color(text_color)
                .text_size(tab_font)
                .text_weight(if is_active { 600 } else { 400 })
                .focusable()
                .cursor_pointer();

            if vertical {
                tab_btn = tab_btn
                    .w_full()
                    .pt(space_y * 0.5)
                    .pb(space_y * 0.5)
                    .pl(space_x)
                    .pr(space_x);
            } else if compact {
                tab_btn = tab_btn
                    .pl(space_x * 0.5)
                    .pr(space_x * 0.5)
                    .pt(space_y * 0.5)
                    .pb(space_y * 0.5);
            } else {
                tab_btn = tab_btn
                    .pl(space_x)
                    .pr(space_x)
                    .pt(space_y * 0.5)
                    .pb(space_y * 0.5);
            }

            if is_active {
                tab_btn = tab_btn
                    .bg(active_bg)
                    .border_b_2()
                    .border_color_bottom(accent);
            }
            if let Some(handler) = &on_tab_change {
                let handler = std::sync::Arc::clone(handler);
                let value = value.to_string();
                tab_btn = tab_btn.on_click(move |_event| handler(&value));
            }

            tab_btn
        };

    // Collapse toggle (only when collapsible).
    let build_toggle = |vertical: bool| -> JsEl {
        let glyph = match spec.edge {
            DockEdge::Left | DockEdge::Top => "‹",
            DockEdge::Right | DockEdge::Bottom => "›",
        };
        let mut t = ui_element::button(glyph)
            .id("dock-collapse")
            .text_color(text_muted)
            .text_size(tab_font)
            .focusable()
            .cursor_pointer();
        if vertical {
            t = t.w_full().pt(space_y * 0.5).pb(space_y * 0.5);
        } else {
            t = t
                .pl(space_x * 0.5)
                .pr(space_x * 0.5)
                .pt(space_y * 0.5)
                .pb(space_y * 0.5);
        }
        if let Some(handler) = &on_collapse_toggle {
            let handler = std::sync::Arc::clone(handler);
            let next = !spec.is_collapsed;
            t = t.on_click(move |_event| handler(next));
        }

        t
    };

    // Drop-zone overlay affordance — dashed accent border + tinted fill
    // (contract .dock-region__drop-zone: 0.125rem dashed accent).
    let drop_zone = || -> JsEl {
        ui_element::div()
            .grow()
            .border(border_w * 2.0)
            .border_style(BorderStyle::Dashed)
            .border_color(accent)
            .bg(tint(accent, 0.08))
    };

    // ── Static mode: stacked panels, no tabs / collapse ─────────
    if spec.sizing == DockSizing::Static {
        let mut stack = if is_side_edge {
            ui_element::div().flex_row()
        } else {
            ui_element::div().flex_col()
        };
        stack = stack
            .bg(root_bg)
            .border(border_w)
            .border_color(root_border)
            .grow();

        for item in &spec.items {
            let mut cell = ui_element::div()
                .id(format!("dock-stack-{}", item.value))
                .flex_1()
                .min_w_0()
                .child(ui_element::label(&item.label));
            if spec.can_accept_panel {
                // Drop-target ring affordance per stack item.
                cell = cell.border(border_w).border_color(accent);
            }
            stack = stack.child(cell);
        }
        if let Some(c) = content {
            stack = stack.child(c);
        }
        if spec.can_accept_panel {
            stack = stack.child(drop_zone());
        }
        return stack;
    }

    // ── Collapsed: hidden posture (toggle only) ─────────────────
    if spec.is_collapsed && spec.collapsed_posture == DockCollapsedPosture::Hidden {
        let mut region = ui_element::div().flex_col().items_center().justify_center();
        if spec.is_collapsible {
            region = region.child(build_toggle(false).pt(space_y).pb(space_y));
        }
        return region;
    }

    // ── Collapsed: icon-strip posture ───────────────────────────
    if spec.is_collapsed && spec.collapsed_posture == DockCollapsedPosture::IconStrip {
        if is_side_edge {
            // Vertical icon strip: toggle on top, icon-only tabs stacked.
            let mut strip = ui_element::div()
                .flex_col()
                .gap(space_y * 0.5)
                .pt(space_y)
                .pb(space_y)
                .bg(fill)
                .border(border_w)
                .border_color(root_border);
            if spec.is_collapsible {
                strip = strip.child(build_toggle(true));
            }
            for item in &spec.items {
                strip = strip.child(build_tab(
                    &item.value,
                    &item.label,
                    item.icon.as_deref(),
                    true,
                    true,
                ));
            }
            return strip;
        } else {
            // Horizontal compact icon strip: icon-only tabs + toggle, no body.
            let mut strip = ui_element::div()
                .flex_row()
                .items_center()
                .gap(tab_gap)
                .pl(space_x)
                .pr(space_x)
                .pt(space_y * 0.5)
                .pb(space_y * 0.5)
                .bg(fill)
                .border(border_w)
                .border_color(root_border);
            for item in &spec.items {
                strip = strip.child(build_tab(
                    &item.value,
                    &item.label,
                    item.icon.as_deref(),
                    true,
                    false,
                ));
            }
            if spec.is_collapsible {
                strip = strip.child(build_toggle(false));
            }
            return strip;
        }
    }

    // ── Expanded flexible mode: strip (tabs + toggle) + body ────
    let mut el = ui_element::div()
        .bg(root_bg)
        .border(border_w)
        .border_color(root_border)
        .flex_col()
        .grow();

    let mut tab_bar = ui_element::div()
        .flex_row()
        .items_center()
        .gap(tab_gap)
        .pl(space_x)
        .pr(space_x)
        .pt(space_y * 0.5)
        .border_b_1()
        .border_color(border_subtle);

    // Tab list grows; toggle pinned at the end.
    let mut tab_list = ui_element::div()
        .flex_row()
        .items_center()
        .gap(tab_gap)
        .flex_1()
        .min_w_0();
    for item in &spec.items {
        tab_list = tab_list.child(build_tab(
            &item.value,
            &item.label,
            item.icon.as_deref(),
            false,
            false,
        ));
    }
    tab_bar = tab_bar.child(tab_list);
    if spec.is_collapsible {
        tab_bar = tab_bar.child(build_toggle(false));
    }
    el = el.child(tab_bar);

    // Body region — active panel content.
    if let Some(c) = content {
        el = el.child(ui_element::div().grow().min_h(0.0).child(c));
    }

    // Cross-region drop-zone affordance.
    if spec.can_accept_panel {
        el = el.child(drop_zone());
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
        // Contract: dock tabs are a `role="tablist"`.
        .aria_role(jetstream_ui::accesskit::Role::TabList)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::{probe, ProbeTree};
    use poodle_specs::{DockEdge, PanelTabItem};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn items() -> Vec<PanelTabItem> {
        vec![
            PanelTabItem::new("explorer", "Explorer").with_icon("folder"),
            PanelTabItem::new("search", "Search").with_icon("search"),
            PanelTabItem::new("scm", "Source Control").with_icon("git"),
        ]
    }

    fn spec() -> DockRegionSpec {
        DockRegionSpec::new(DockEdge::Left, items()).with_value("explorer")
    }

    fn render(spec: &DockRegionSpec) -> ProbeTree {
        let el = js_dock_region(spec, &theme(), Some(ui_element::label("Panel body")));
        probe(&el, 480.0, 320.0)
    }

    #[test]
    fn expanded_renders_tabs_and_body() {
        let tree = render(&spec());
        assert!(!tree.is_empty(), "no nodes rendered");
        // All three tab buttons rendered (label text present).
        assert!(
            tree.texts().iter().any(|t| t.contains("Explorer")),
            "Explorer tab missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.texts().iter().any(|t| t.contains("Search")),
            "Search tab missing"
        );
        assert!(
            tree.texts().iter().any(|t| t.contains("Source Control")),
            "SCM tab missing"
        );
        // Body content present.
        assert!(
            tree.has_text("Panel body"),
            "body missing: {:?}",
            tree.texts()
        );
        // Tab button anatomy present.
        assert!(
            tree.find_token("dock-tab-explorer").is_some(),
            "tab id missing"
        );
    }

    #[test]
    fn compact_tabs_render_icon_only() {
        // Collapsed icon-strip on a top edge = horizontal compact icon-only tabs.
        let s = DockRegionSpec::new(DockEdge::Top, items())
            .with_value("explorer")
            .with_collapsed(true)
            .with_collapsed_posture(DockCollapsedPosture::IconStrip);
        let el = js_dock_region(&s, &theme(), None);
        let tree = probe(&el, 480.0, 80.0);
        // Compact mode shows icon glyphs, not labels.
        assert!(
            tree.texts().iter().any(|t| t.contains("folder")),
            "compact icon missing: {:?}",
            tree.texts()
        );
        assert!(
            !tree.texts().iter().any(|t| t.contains("Explorer")),
            "compact tabs must suppress labels: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn collapsed_posture_differs_from_expanded() {
        let expanded = render(&spec());
        // Hidden posture: only the collapse toggle, no tab labels, no body.
        let hidden_spec = spec()
            .with_collapsed(true)
            .with_collapsible(true)
            .with_collapsed_posture(DockCollapsedPosture::Hidden);
        let hidden = render(&hidden_spec);

        assert!(
            hidden.len() < expanded.len(),
            "hidden posture ({}) should render fewer nodes than expanded ({})",
            hidden.len(),
            expanded.len()
        );
        assert!(
            !hidden.texts().iter().any(|t| t.contains("Explorer")),
            "hidden posture must not render tab labels: {:?}",
            hidden.texts()
        );
        assert!(
            !hidden.has_text("Panel body"),
            "hidden posture must not render body"
        );

        // Icon-strip (side edge) renders icon-only tabs, no body.
        let strip_spec = spec()
            .with_collapsed(true)
            .with_collapsed_posture(DockCollapsedPosture::IconStrip);
        let strip = render(&strip_spec);
        assert!(
            !strip.has_text("Panel body"),
            "icon-strip must hide the body"
        );
        assert!(
            strip.texts().iter().any(|t| t.contains("folder")),
            "icon-strip must render icon glyphs: {:?}",
            strip.texts()
        );
    }

    #[test]
    fn emphasis_strong_renders() {
        // Strong emphasis tints the border; the region still lays out with content.
        let s = spec().with_emphasis(DockEmphasis::Strong);
        let strong = render(&s);
        assert!(!strong.is_empty(), "strong emphasis produced no nodes");
        assert!(strong.has_text("Panel body"), "strong emphasis lost body");

        // Quiet emphasis renders transparent chrome but keeps anatomy.
        let quiet = render(&spec().with_emphasis(DockEmphasis::Quiet));
        assert!(quiet.has_text("Panel body"), "quiet emphasis lost body");
        // Root background between quiet and standard differs (transparent vs panel).
        let standard_root_a = render(&spec()).nodes[0].bg.map(|b| b.a);
        let quiet_root_a = quiet.nodes[0].bg.map(|b| b.a);
        assert_ne!(
            standard_root_a, quiet_root_a,
            "quiet root background alpha should differ from standard"
        );
    }

    #[test]
    fn can_accept_panel_renders_drop_affordance() {
        let baseline = render(&spec());
        let accepting = render(&spec().with_can_accept_panel(true));
        // Drop-zone overlay adds an extra node vs the baseline.
        assert!(
            accepting.len() > baseline.len(),
            "can_accept_panel should add a drop-zone node ({} vs {})",
            accepting.len(),
            baseline.len()
        );
        // The drop zone is tinted with the accent color.
        let accent = resolve_color(&theme(), "color.accent.base");
        let dz = crate::render_probe::ProbeColor {
            r: accent.x,
            g: accent.y,
            b: accent.z,
            a: 0.08,
        };
        assert!(
            accepting.has_background(dz, 0.05),
            "drop-zone accent tint not found in render"
        );
    }

    #[test]
    fn static_mode_stacks_panels() {
        let s = DockRegionSpec::new(DockEdge::Top, items()).with_sizing(DockSizing::Static);
        let el = js_dock_region(&s, &theme(), None);
        let tree = probe(&el, 480.0, 200.0);
        // Static mode renders each panel label, no collapse toggle.
        assert!(
            tree.has_text("Explorer"),
            "static panel missing: {:?}",
            tree.texts()
        );
        assert!(
            tree.find_token("dock-stack-explorer").is_some(),
            "stack item id missing"
        );
        assert!(
            tree.find_token("dock-collapse").is_none(),
            "static mode must not render collapse toggle"
        );
    }

    #[test]
    fn a_tab_press_reports_its_value() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let tabs = Arc::clone(&seen);

        let el = DockRegion::from_spec(spec(), &theme())
            .content(ui_element::label("Panel body"))
            .on_tab_change(move |value| tabs.lock().unwrap().push(value.to_string()))
            .into_js_el();

        // The tab's button text is "<icon> <label>" when it carries an icon.
        let second = spec().items[1].clone();
        let button_text = match &second.icon {
            Some(icon) => format!("{icon} {}", second.label),
            None => second.label.clone(),
        };
        crate::element::click_probe::click_text(&el, 480.0, 320.0, &button_text);

        assert_eq!(seen.lock().unwrap().as_slice(), [second.value]);
    }

    #[test]
    fn the_collapse_toggle_reports_the_next_state() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(Vec::new()));
        let values = Arc::clone(&seen);

        let el = DockRegion::from_spec(spec().with_collapsible(true), &theme())
            .content(ui_element::label("Panel body"))
            .on_collapse_toggle(move |next| values.lock().unwrap().push(next))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 480.0, 320.0, "‹");

        assert_eq!(seen.lock().unwrap().as_slice(), [true]);
    }
}
