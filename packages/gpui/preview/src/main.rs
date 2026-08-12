//! Poodle GPUI Preview — native macOS preview application for the Poodle design system.
//!
//! Matches the Svelte preview app layout: top bar with pill tabs,
//! display controls bar, and section-specific content areas.

mod app_state;
mod component_registry;
mod contract_usage_docs;
mod node_compat;
mod providers;
mod specimens;
mod style_bridge;
mod token_view;
mod usage_docs_view;

/// The scene's generated Rust artifact (card 036): plain data, no Poodle
/// crate imports — pulled in via the `poodle-tokens` `#[path]` mechanism
/// (g13-b003 R1 names it as the shape g13 follows).
#[path = "generated/preview-shell.rs"]
mod generated_shell;

use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_specs::{
    CodeSpec, ControlDensity as SpecControlDensity, ControlSize as SpecControlSize,
    SemanticControlSizeRole, SidebarNavGroup, SidebarNavItem, SidebarNavSpec, SliderSpec,
    TabDefinition,
    ThemeSelectSpec,
    TabVariant, TabsSpec, TextInputSpec,
};

/// Asset source that loads files from the preview app's directory.
struct PreviewAssets {
    base: PathBuf,
}

impl AssetSource for PreviewAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        let full_path = path.strip_prefix("assets/icons/").map_or_else(
            || self.base.join(path),
            |name| self.base.join("../../render/assets/icons").join(name),
        );
        match std::fs::read(&full_path) {
            Ok(data) => Ok(Some(Cow::Owned(data))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let full_path = if path == "assets/icons" {
            self.base.join("../../render/assets/icons")
        } else {
            self.base.join(path)
        };
        match std::fs::read_dir(&full_path) {
            Ok(entries) => Ok(entries
                .filter_map(|entry| {
                    entry
                        .ok()
                        .and_then(|e| e.file_name().into_string().ok())
                        .map(SharedString::from)
                })
                .collect()),
            Err(_) => Ok(vec![]),
        }
    }
}

use app_state::{
    AppState, ChromeEvent, ControlSize, Density, NodeSpecimenEvent, Section, CONTRAST_MAX,
    CONTRAST_MIN,
    ThemePreset, TokenPanel,
};
use component_registry::{find_component, grouped_components, package_name};
use contract_usage_docs::load_contract_usage_docs;
use crate::node_compat::{Code, SidebarNav, Slider, Tabs, TextInput, ThemeSelect};
use style_bridge::color_to_hsla;

// Global keyboard actions
actions!(poodle_preview, [Quit, CloseWindow]);

/// Root view for the preview application.
struct PreviewRoot {
    state: AppState,
    catalogue_sidebar: Entity<CatalogueSidebar>,
    component_page_list: ListState,
    component_page_key: Option<ComponentPageKey>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ComponentPageKey {
    slug: &'static str,
    theme: ThemePreset,
    density: Density,
    control_size: ControlSize,
    contrast: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CatalogueSidebarKey {
    theme: ThemePreset,
    density: Density,
    control_size: ControlSize,
    contrast: u32,
    component_search: String,
    active_component_slug: Option<String>,
}

struct CatalogueSidebar {
    key: CatalogueSidebarKey,
    theme: poodle_gpui::GpuiThemeProvider,
    node_events: std::sync::Arc<std::sync::Mutex<Vec<NodeSpecimenEvent>>>,
}

impl CatalogueSidebarKey {
    fn from_state(state: &AppState) -> Self {
        Self {
            theme: state.theme_preset,
            density: state.density,
            control_size: state.control_size,
            contrast: state.contrast.to_bits(),
            component_search: state.component_search.clone(),
            active_component_slug: state.active_component_slug.clone(),
        }
    }
}

impl CatalogueSidebar {
    fn new(state: &AppState) -> Self {
        Self {
            key: CatalogueSidebarKey::from_state(state),
            theme: state.theme.clone(),
            node_events: std::sync::Arc::clone(&state.node_events),
        }
    }

    fn sync(&mut self, state: &AppState, cx: &mut Context<Self>) {
        let key = CatalogueSidebarKey::from_state(state);
        if self.key != key {
            self.key = key;
            self.theme = state.theme.clone();
            cx.notify();
        }
    }
}

impl Render for CatalogueSidebar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let border_subtle = self.theme.resolve_color("color.border.subtle");
        let groups = grouped_components(&self.key.component_search);
        let sidebar_groups: Vec<SidebarNavGroup> = groups
            .iter()
            .map(|group| {
                SidebarNavGroup::new(
                    group.tag.label().to_ascii_lowercase(),
                    group
                        .items
                        .iter()
                        .map(|component| {
                            SidebarNavItem::new(component.slug, component.display_name)
                        })
                        .collect(),
                )
                .with_label(group.tag.label())
            })
            .collect();
        let mut sidebar_spec = SidebarNavSpec::new(sidebar_groups)
            .with_aria_label("Component catalogue")
            .with_density(sidebar_nav_density(self.key.density))
            .with_size(sidebar_nav_size(self.key.control_size))
            .with_size_role(SemanticControlSizeRole::Chrome);
        if let Some(active_slug) = self.key.active_component_slug.as_deref() {
            sidebar_spec = sidebar_spec.with_value(active_slug);
        }

        div()
            .id("catalogue-sidebar")
            .size_full()
            .overflow_y_scroll()
            .border_r_1()
            .border_color(color_to_hsla(border_subtle).opacity(0.6))
            .child(SidebarNav::from_spec(sidebar_spec, &self.theme).on_change({
                let queue = std::sync::Arc::clone(&self.node_events);
                std::sync::Arc::new(move |val: &str| {
                    queue.lock().unwrap().push(NodeSpecimenEvent::Chrome(
                        ChromeEvent::ActiveComponent(val.to_string()),
                    ));
                })
            }))
    }
}

impl PreviewRoot {
    fn new(cx: &mut Context<Self>) -> Self {
        let state = AppState::new();
        let catalogue_sidebar = cx.new(|_| CatalogueSidebar::new(&state));
        Self {
            state,
            catalogue_sidebar,
            component_page_list: ListState::new(3, ListAlignment::Top, px(256.0)),
            component_page_key: None,
        }
    }
}

fn sidebar_nav_density(density: Density) -> SpecControlDensity {
    match density {
        Density::Compact => SpecControlDensity::Compact,
        Density::Default => SpecControlDensity::Default,
        Density::Comfortable => SpecControlDensity::Comfortable,
    }
}

fn sidebar_nav_size(size: ControlSize) -> SpecControlSize {
    match size {
        ControlSize::Xs => SpecControlSize::Xs,
        ControlSize::Sm => SpecControlSize::Sm,
        ControlSize::Md => SpecControlSize::Md,
        ControlSize::Lg => SpecControlSize::Lg,
        ControlSize::Xl => SpecControlSize::Xl,
    }
}

impl Render for PreviewRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Apply interactions node-backed specimens reported since the last frame.
        let specimen_changed = self.state.drain_node_events();
        if specimen_changed {
            self.component_page_list.reset(3);
        }
        // Restart the backend's generated-id counter so a node that declares no
        // id keeps the same ElementId between frames. gpui keys a click's
        // pending mouse-down by element id, and a real click spans frames.
        poodle_gpui_node_backend::reset_element_ids();
        let theme = &self.state.theme;

        let canvas_bg = theme.resolve_color("color.background.canvas");
        let elevated_bg = theme.resolve_color("color.background.elevated");
        let panel_bg = theme.resolve_color("color.background.panel");
        let text_primary = theme.resolve_color("color.text.primary");
        let text_secondary = theme.resolve_color("color.text.secondary");
        let accent = theme.resolve_color("color.accent.base");
        let border_subtle = theme.resolve_color("color.border.subtle");
        let border = theme.resolve_color("color.border.default");

        // Compute remaining height for the content area so scroll containers
        // get a definite pixel height (required for gpui content-mask hit testing).
        let window_h = window.viewport_size().height;
        let top_bar_h = px(55.0);
        let controls_h = px(80.0);
        let content_h = window_h - top_bar_h - controls_h;

        div()
            .size_full()
            .flex()
            .flex_col()
            .font_family("Inter")
            .bg(color_to_hsla(canvas_bg))
            .text_color(color_to_hsla(text_primary))
            // ── Top bar ──────────────────────────────────────────────
            .child(
                div()
                    .w_full()
                    .h(top_bar_h)
                    .flex()
                    .items_center()
                    .gap(px(16.0))
                    .px(px(16.0))
                    .py(px(8.0))
                    .flex_shrink_0()
                    .bg(color_to_hsla(elevated_bg))
                    .border_b_1()
                    .border_color(color_to_hsla(border_subtle))
                    // Title — bold, 16px
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .child("Poodle"),
                    )
                    // Nav tabs (pill style)
                    .child(self.render_nav_tabs(text_secondary, accent, cx))
                    // Spacer
                    .child(div().flex_1())
                    // Right pills showing current settings
                    .child(self.render_status_pills(text_secondary, border)),
            )
            // ── Display controls bar ─────────────────────────────────
            .child(self.render_display_controls(
                text_secondary,
                accent,
                border,
                border_subtle,
                panel_bg,
                controls_h,
                cx,
            ))
            // ── Main content area ────────────────────────────────────
            // Section content is a direct child of root — no intermediate wrapper.
            // Each section is given an explicit pixel height so overflow_y_scroll
            // containers get a definite content-mask for hit testing.
            .child(self.render_section_content(content_h, cx))
    }
}

impl PreviewRoot {
    /// Pill-style nav tabs using Tabs with the Pill variant.
    fn render_nav_tabs(
        &self,
        _text_secondary: poodle_tokens::typed::ColorValue,
        _accent: poodle_tokens::typed::ColorValue,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let active_value = self.state.section.label();
        let nav_sections = [Section::Components, Section::Tokens];
        let tab_defs: Vec<TabDefinition> = nav_sections
            .iter()
            .map(|s| TabDefinition::new(s.label(), s.label()))
            .collect();

        let spec = TabsSpec::new(tab_defs)
            .with_variant(TabVariant::Pill)
            .with_value(active_value);

        let queue = std::sync::Arc::clone(&self.state.node_events);
        Tabs::from_spec(spec, &self.state.theme)
            .with_id("nav-tabs")
            .on_change(std::sync::Arc::new(move |val: &str| {
                let section = match val {
                    "Components" => Section::Components,
                    "Tokens" => Section::Tokens,
                    _ => return,
                };
                queue
                    .lock()
                    .unwrap()
                    .push(NodeSpecimenEvent::Chrome(ChromeEvent::Section(section)));
            }))
    }

    /// Right-aligned pills showing current theme, density, and size.
    /// Matches Svelte: 11px text, 999px radius, 1px subtle border, 3px 8px padding.
    fn render_status_pills(
        &self,
        text_secondary: poodle_tokens::typed::ColorValue,
        border: poodle_tokens::typed::ColorValue,
    ) -> Div {
        // Svelte computed: bg srgb(0.094, 0.094, 0.094 / 0.9), border srgb(0.776, 0.776, 0.776 / 0.116)
        let canvas_bg = self.state.theme.resolve_color("color.background.canvas");

        let pill = |text: &str| {
            div()
                .px(px(8.0))
                .py(px(3.0))
                .rounded(px(999.0))
                .border_1()
                .border_color(color_to_hsla(border))
                .bg(color_to_hsla(canvas_bg).opacity(0.9))
                .text_size(px(11.0))
                .text_color(color_to_hsla(text_secondary))
                .child(text.to_string())
        };

        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .child(pill(self.state.theme_preset.label()))
            .child(pill(self.state.density.label()))
            .child(pill(self.state.control_size.label()))
    }

    /// Display controls bar — theme, density, size, contrast, and catalogue
    /// search. Matches Svelte: 80px height, panel bg, 12px 16px padding,
    /// 20px/32px gap.
    ///
    /// The control surface is the scene's (card 036 R4): which controls
    /// exist and their label text come from the generated artifact
    /// (`generated/preview-shell.rs` — plain data, no Poodle crate imports,
    /// per card 036 R1); widget mechanics, value sets, and runtime state
    /// stay host-owned. Kinds are compared as plain strings so deleting an
    /// axis or search from the scene removes the control cleanly instead of
    /// becoming a compile error (the card-035 R3 removal property, repeated
    /// for the natives). Casing is presentation (R3): this shell renders
    /// eyebrows uppercase as its house look.
    #[expect(clippy::too_many_arguments, reason = "the preview control bar keeps resolved theme values explicit")]
    fn render_display_controls(
        &self,
        text_secondary: poodle_tokens::typed::ColorValue,
        accent: poodle_tokens::typed::ColorValue,
        border: poodle_tokens::typed::ColorValue,
        border_subtle: poodle_tokens::typed::ColorValue,
        panel_bg: poodle_tokens::typed::ColorValue,
        h: Pixels,
        cx: &mut Context<Self>,
    ) -> Div {
        let scene = &generated_shell::PREVIEW_SHELL;
        let find_control = |kind: &str| scene.controls.iter().find(|c| c.kind == kind);

        let mut bar = div()
            .w_full()
            .h(h)
            .flex()
            .flex_wrap()
            .items_start()
            .gap_x(px(32.0))
            .gap_y(px(20.0))
            .px(px(16.0))
            .py(px(12.0))
            .bg(color_to_hsla(panel_bg))
            .border_b_1()
            .border_color(color_to_hsla(border_subtle))
            .flex_shrink_0();
        // No `overflow_hidden` here: the ThemeSelect panel paints outside
        // this bar when open, and clipping it left the swatch grid cut in
        // half. See the placement note in the g12.019 card — the shared
        // recipe lays the panel out beside the trigger rather than
        // portalling it, which the contract asks for.

        // Theme group — the real ThemeSelect, as the Svelte preview uses,
        // rather than a row of one button per theme. Label from the scene.
        if let Some(control) = find_control("theme") {
            bar = bar.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_secondary))
                            .child(control.label.to_uppercase()),
                    )
                    .child(deferred(
                        ThemeSelect::from_spec(
                            ThemeSelectSpec::new()
                                .with_themes(self.state.theme_options.clone())
                                .with_value(self.state.theme_preset.label())
                                .with_open(self.state.theme_select_open),
                            &self.state.theme,
                        )
                        .on_open_change({
                            let queue = std::sync::Arc::clone(&self.state.node_events);
                            std::sync::Arc::new(move |open: bool| {
                                queue.lock().unwrap().push(NodeSpecimenEvent::Chrome(
                                    ChromeEvent::ThemeSelectOpen(open),
                                ));
                            })
                        })
                        .on_change({
                            let queue = std::sync::Arc::clone(&self.state.node_events);
                            std::sync::Arc::new(move |value: &str| {
                                let Some(preset) = ThemePreset::ALL
                                    .iter()
                                    .copied()
                                    .find(|p| p.label() == value)
                                else {
                                    return;
                                };
                                queue
                                    .lock()
                                    .unwrap()
                                    .push(NodeSpecimenEvent::Chrome(ChromeEvent::Theme(preset)));
                            })
                        }),
                    )),
            );
        }

        // Density group
        if let Some(control) = find_control("density") {
            let opts: Vec<(&str, bool)> = Density::ALL
                .iter()
                .map(|d| (d.label(), self.state.density == *d))
                .collect();
            bar = bar.child(self.render_toggle_group(
                control.label,
                text_secondary,
                &opts,
                accent,
                border,
                "density",
                cx,
            ));
        }

        // Size group
        if let Some(control) = find_control("size") {
            let opts: Vec<(&str, bool)> = ControlSize::ALL
                .iter()
                .map(|s| (s.label(), self.state.control_size == *s))
                .collect();
            bar = bar.child(self.render_toggle_group(
                control.label,
                text_secondary,
                &opts,
                accent,
                border,
                "size",
                cx,
            ));
        }

        // Contrast — a real Slider over the continuous neutral-ramp axis,
        // matching the web preview's range input and the Jetstream shell.
        // Four preset buttons could not express the values between them.
        // Header text is the scene's label, uppercased as house style; the
        // accessibility label is the scene's word, not a second copy (R3).
        if let Some(control) = find_control("contrast") {
            bar = bar.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_secondary))
                            .child(format!(
                                "{}  {:.2}",
                                control.label.to_uppercase(),
                                self.state.contrast
                            )),
                    )
                    .child(
                        // The slider fills its host's width, so the header has
                        // to give it one — unsized it collapsed to a stub. The
                        // box is the same 32px the toggle buttons beside it
                        // use, with the 6px track centred in it, so the row
                        // lines up on one baseline instead of the slider
                        // floating above it.
                        div()
                            .w(px(160.0))
                            .h(px(32.0))
                            .flex()
                            .items_center()
                            .child(Slider::from_spec(
                            SliderSpec {
                                // The default step is 1.0, which over a 0..1
                                // range can only ever snap to the endpoints —
                                // the axis is continuous, so it needs a fine
                                // one.
                                step: 0.01,
                                ..SliderSpec::new(self.state.contrast as f64)
                                    .with_bounds(CONTRAST_MIN as f64, CONTRAST_MAX as f64)
                            },
                            &self.state.theme,
                        )
                        .with_id("contrast")
                        .aria_label(control.label)
                        .on_change({
                            let queue = std::sync::Arc::clone(&self.state.node_events);
                            std::sync::Arc::new(move |value: f64| {
                                queue.lock().unwrap().push(NodeSpecimenEvent::Chrome(
                                    ChromeEvent::Contrast(value as f32),
                                ));
                            })
                        })),
                    ),
            );
        }

        // Search group — filters the component catalogue by name.
        if let Some(control) = find_control("search") {
            bar = bar.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .min_w(px(240.0))
                    .flex_1()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_secondary))
                            .child(control.label.to_uppercase()),
                    )
                    .child(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("component-search")
                                .with_input_type("search")
                                .with_placeholder(control.placeholder.unwrap_or("Find component..."))
                                .with_value(&self.state.component_search)
                                .with_selection(
                                    self.state.search_selection.0,
                                    self.state.search_selection.1,
                                )
                                .with_focused(self.state.search_focused)
                                .with_aria_label("Search components"),
                            &self.state.theme,
                        )
                        .with_id("component-search")
                        .on_focus_change({
                            let queue = std::sync::Arc::clone(&self.state.node_events);
                            std::sync::Arc::new(move |focused: bool| {
                                queue.lock().unwrap().push(NodeSpecimenEvent::Chrome(
                                    ChromeEvent::SearchFocused(focused),
                                ));
                            })
                        })
                        .on_selection_change({
                            let queue = std::sync::Arc::clone(&self.state.node_events);
                            std::sync::Arc::new(move |start: usize, end: usize| {
                                queue.lock().unwrap().push(NodeSpecimenEvent::Chrome(
                                    ChromeEvent::SearchSelection(start, end),
                                ));
                            })
                        })
                        .on_change({
                            let queue = std::sync::Arc::clone(&self.state.node_events);
                            move |val: &str| {
                                let mut events = queue.lock().unwrap();
                                events.push(NodeSpecimenEvent::Chrome(
                                    ChromeEvent::ComponentSearch(val.to_string()),
                                ));
                                events.push(NodeSpecimenEvent::Chrome(ChromeEvent::Section(
                                    Section::Components,
                                )));
                            }
                        }),
                    ),
            );
        }

        bar
    }

    /// A labelled toggle group (uppercase eyebrow + row of individual toggle buttons).
    /// Matches Svelte: each button is a separate pill with its own border.
    #[expect(clippy::too_many_arguments, reason = "toggle groups keep preview state and resolved theme values explicit")]
    fn render_toggle_group(
        &self,
        label: &'static str,
        text_secondary: poodle_tokens::typed::ColorValue,
        options: &[(&str, bool)],
        accent: poodle_tokens::typed::ColorValue,
        _border: poodle_tokens::typed::ColorValue,
        group_id: &'static str,
        cx: &mut Context<Self>,
    ) -> Div {
        let text_primary = self.state.theme.resolve_color("color.text.primary");
        let border_default = self.state.theme.resolve_color("color.border.default");
        let canvas_bg = self.state.theme.resolve_color("color.background.canvas");

        let mut toggle_row = div().flex().gap(px(4.0));

        for (i, &(opt_label, is_active)) in options.iter().enumerate() {
            let mut btn = div()
                .id(SharedString::from(format!("{}-{}", group_id, opt_label)))
                .h(px(32.0))
                .px(px(12.0))
                .flex()
                .items_center()
                .rounded(px(6.0))
                .border_1()
                .text_size(px(12.0))
                .font_weight(FontWeight::SEMIBOLD)
                .cursor_pointer()
                .child(opt_label.to_string());

            btn = if is_active {
                btn.bg(color_to_hsla(accent).opacity(0.22))
                    .border_color(color_to_hsla(accent).opacity(0.56))
                    .text_color(color_to_hsla(text_primary))
            } else {
                btn.bg(color_to_hsla(canvas_bg).opacity(0.88))
                    .border_color(color_to_hsla(border_default))
                    .text_color(color_to_hsla(text_primary))
            };

            btn = btn.on_click(cx.listener(move |this, _event: &ClickEvent, _window, cx| {
                match group_id {
                    "theme" => {
                        this.state.set_theme(ThemePreset::ALL[i]);
                    }
                    "density" => {
                        this.state.density = Density::ALL[i];
                        this.state.rebuild_theme();
                    }
                    "size" => {
                        this.state.control_size = ControlSize::ALL[i];
                        this.state.rebuild_theme();
                    }
                    _ => {}
                }
                cx.notify();
            }));

            toggle_row = toggle_row.child(btn);
        }

        // Eyebrow label: uppercase, 11px, semibold, 1.32px letter-spacing
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color_to_hsla(text_secondary))
                    .child(label.to_uppercase()),
            )
            .child(toggle_row)
    }

    /// Section content router.
    /// `available_h` is the pixel height remaining after top bar + controls.
    fn render_section_content(&mut self, available_h: Pixels, cx: &mut Context<Self>) -> Div {
        let section_content = match self.state.section {
            Section::Components => self.render_components_section(available_h, cx),
            Section::Tokens => self.render_tokens_section(available_h, cx),
        };

        div()
            .w_full()
            .h(available_h)
            .flex()
            .flex_col()
            .child(section_content)
    }

    fn render_tokens_section(&self, available_h: Pixels, _cx: &mut Context<Self>) -> Div {
        let theme = &self.state.theme;
        let text_primary = theme.resolve_color("color.text.primary");
        let text_secondary = theme.resolve_color("color.text.secondary");
        let border_subtle = theme.resolve_color("color.border.subtle");
        let panel_bg = theme.resolve_color("color.background.panel");

        let tab_defs = vec![
            TabDefinition::new(TokenPanel::Summary.value(), TokenPanel::Summary.label()),
            TabDefinition::new(TokenPanel::Inspector.value(), TokenPanel::Inspector.label()),
        ];
        let active_tab = self.state.active_token_panel.value();
        let matching_count = token_view::matching_token_count(&self.state.token_inspector_query);

        div().w_full().h(available_h).child(
            div()
                .id("tokens-section")
                .size_full()
                .overflow_y_scroll()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .p(px(24.0))
                        .max_w(px(1024.0))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("TOKEN TOOLS"),
                                )
                                .child(
                                    div()
                                        .text_3xl()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(color_to_hsla(text_primary))
                                        .child("Runtime values and emitted-token inspection"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("@inflatable-cookie/poodle-core/tokens"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("packages/tokens/artifacts/css/"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("packages/tokens/artifacts/ts/"),
                                ),
                        )
                        .child(
                            Tabs::from_spec(
                                TabsSpec::new(tab_defs)
                                    .with_variant(TabVariant::Pill)
                                    .with_value(active_tab),
                                theme,
                            )
                            .with_id("token-panel-tabs")
                            .on_change({
                                let queue = std::sync::Arc::clone(&self.state.node_events);
                                std::sync::Arc::new(move |val: &str| {
                                    let panel = match val {
                                        "token-inspector" => TokenPanel::Inspector,
                                        _ => TokenPanel::Summary,
                                    };
                                    queue.lock().unwrap().push(NodeSpecimenEvent::Chrome(
                                        ChromeEvent::TokenPanel(panel),
                                    ));
                                })
                            }),
                        )
                        .child(
                            div()
                                .p(px(16.0))
                                .rounded(px(8.0))
                                .bg(color_to_hsla(panel_bg))
                                .border_1()
                                .border_color(color_to_hsla(border_subtle))
                                .flex()
                                .flex_col()
                                .gap(px(16.0))
                                .when(self.state.active_token_panel == TokenPanel::Summary, |el| {
                                    el.child(token_view::render_runtime_token_summary(theme))
                                })
                                .when(
                                    self.state.active_token_panel == TokenPanel::Inspector,
                                    |el| {
                                        el.child(
                                            TextInput::from_spec(
                                                TextInputSpec::new()
                                                    .with_id("token-inspector-query")
                                                    .with_input_type("search")
                                                    .with_placeholder("Filter tokens by path")
                                                    .with_value(&self.state.token_inspector_query)
                                                    .with_aria_label("Filter semantic tokens"),
                                                theme,
                                            )
                                            .with_id("token-inspector-query")
                                            .on_change({
                                                let queue =
                                                    std::sync::Arc::clone(&self.state.node_events);
                                                move |val: &str| {
                                                    queue.lock().unwrap().push(
                                                        NodeSpecimenEvent::Chrome(
                                                            ChromeEvent::TokenInspectorQuery(
                                                                val.to_string(),
                                                            ),
                                                        ),
                                                    );
                                                }
                                            }),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(color_to_hsla(text_secondary))
                                                .child(format!(
                                                    "{} semantic tokens shown",
                                                    matching_count
                                                )),
                                        )
                                        .child(
                                            token_view::render_token_inspector(
                                                theme,
                                                &self.state.token_inspector_query,
                                            ),
                                        )
                                    },
                                ),
                        ),
                ),
        )
    }

    /// Unified component catalogue mirroring the Svelte preview information architecture.
    fn render_components_section(&mut self, available_h: Pixels, cx: &mut Context<Self>) -> Div {
        let theme = &self.state.theme;
        let border_subtle = theme.resolve_color("color.border.subtle");
        let text_secondary = theme.resolve_color("color.text.secondary");
        let elevated_bg = theme.resolve_color("color.background.elevated");
        let active_component = self
            .state
            .active_component_slug
            .as_deref()
            .and_then(find_component);
        self.catalogue_sidebar
            .update(cx, |sidebar, cx| sidebar.sync(&self.state, cx));
        let sidebar = AnyView::from(self.catalogue_sidebar.clone()).cached(
            StyleRefinement::default()
                .w(px(224.0))
                .h(available_h)
                .flex_shrink_0(),
        );

        // Outer layout: horizontal flex row with explicit height.
        let mut layout = div().w_full().h(available_h).flex().child(sidebar);

        if let Some(component) = active_component {
            let mut hasher = DefaultHasher::new();
            component.slug.hash(&mut hasher);
            let content_id = hasher.finish();
            let page_key = ComponentPageKey {
                slug: component.slug,
                theme: self.state.theme_preset,
                density: self.state.density,
                control_size: self.state.control_size,
                contrast: self.state.contrast.to_bits(),
            };
            if self.component_page_key != Some(page_key) {
                self.component_page_key = Some(page_key);
                self.component_page_list.reset(3);
            }
            let slug = component.slug;
            layout = layout.child(
                div()
                    .id(("specimen-content", content_id))
                    .flex_1()
                    .h(available_h)
                    .child(
                        list(
                            self.component_page_list.clone(),
                            cx.processor(move |this, index, _window, cx| {
                                let component = find_component(slug).expect(
                                    "active component disappeared from the static registry",
                                );
                                match index {
                                    0 => this
                                        .render_component_page_header(component)
                                        .into_any_element(),
                                    1 => this
                                        .render_component_specimen(component.slug, cx)
                                        .into_any_element(),
                                    2 => this
                                        .render_component_page_support(component)
                                        .into_any_element(),
                                    _ => Empty.into_any_element(),
                                }
                            }),
                        )
                        .size_full()
                        .p(px(24.0)),
                    ),
            );
        } else {
            let groups = grouped_components(&self.state.component_search);
            layout = layout.child(self.render_catalogue_landing(
                &groups,
                theme,
                available_h,
                text_secondary,
                border_subtle,
                elevated_bg,
                cx,
            ));
        }

        layout
    }

    fn render_component_page_header(&self, component: &component_registry::ComponentEntry) -> Div {
        let theme = &self.state.theme;
        let text_primary = theme.resolve_color("color.text.primary");
        let text_secondary = theme.resolve_color("color.text.secondary");
        let border_subtle = theme.resolve_color("color.border.subtle");
        let elevated_bg = theme.resolve_color("color.background.elevated");
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .pb(px(24.0))
            .child(
                div().flex().items_center().gap(px(8.0)).child(
                    div()
                        .px(px(8.0))
                        .py(px(3.0))
                        .rounded(px(999.0))
                        .border_1()
                        .border_color(color_to_hsla(border_subtle))
                        .bg(color_to_hsla(elevated_bg).opacity(0.92))
                        .text_size(px(11.0))
                        .text_color(color_to_hsla(text_secondary))
                        .child(package_name().to_string()),
                ),
            )
            .child(
                div()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(color_to_hsla(text_primary))
                    .child(component.display_name),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .max_w(px(720.0))
                    .child(component.description),
            )
    }

    fn render_component_page_support(&self, component: &component_registry::ComponentEntry) -> Div {
        let theme = &self.state.theme;
        let text_primary = theme.resolve_color("color.text.primary");
        let border_subtle = theme.resolve_color("color.border.subtle");
        let contract_doc = load_contract_usage_docs(component.slug);
        let import_snippet = format!(
            "use {}::{};",
            package_name().replace('-', "_"),
            component.display_name
        );

        div()
            .flex()
            .flex_col()
            .gap(px(24.0))
            .pt(px(24.0))
            .child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(10.0))
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_primary))
                            .child("Import"),
                    )
                    .child(Code::from_spec(
                        CodeSpec::new()
                            .with_language("rust")
                            .with_content(import_snippet)
                            .with_copyable(false),
                        theme,
                    )),
            )
            .when(contract_doc.exists, |container| {
                container
                    .child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)))
                    .child(usage_docs_view::render_usage_docs(theme, &contract_doc))
            })
    }

    /// Landing page grid showing all components as cards.
    #[expect(clippy::too_many_arguments, reason = "the catalogue layout keeps viewport and resolved theme values explicit")]
    fn render_catalogue_landing(
        &self,
        groups: &[component_registry::ComponentGroup],
        _theme: &poodle_gpui::GpuiThemeProvider,
        available_h: Pixels,
        text_secondary: poodle_tokens::typed::ColorValue,
        border: poodle_tokens::typed::ColorValue,
        elevated_bg: poodle_tokens::typed::ColorValue,
        cx: &mut Context<Self>,
    ) -> Div {
        let mut landing = div().flex().flex_col().gap(px(20.0));
        let filtered_count: usize = groups.iter().map(|group| group.items.len()).sum();

        for group in groups {
            let mut grid = div().flex().flex_wrap().gap(px(12.0));

            for component in &group.items {
                let slug = component.slug;
                grid = grid.child(
                    div()
                        .id(SharedString::from(format!("landing-{}", component.slug)))
                        .w(px(220.0))
                        .p(px(12.0))
                        .rounded(px(8.0))
                        .bg(color_to_hsla(elevated_bg))
                        .border_1()
                        .border_color(color_to_hsla(border))
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .cursor_pointer()
                        .child(div().text_sm().child(component.display_name))
                        .child(
                            div()
                                .text_xs()
                                .text_color(color_to_hsla(text_secondary))
                                .child(component.description),
                        )
                        .on_click(cx.listener(move |this, _event: &ClickEvent, _window, cx| {
                            this.state.active_component_slug = Some(slug.to_string());
                            cx.notify();
                        })),
                );
            }

            landing = landing.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_secondary))
                            .child(group.tag.label()),
                    )
                    .child(grid),
            );
        }

        div().flex_1().h(available_h).child(
            div()
                .id("catalogue-landing")
                .size_full()
                .overflow_y_scroll()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .p(px(24.0))
                        .child(
                            div()
                                .text_xl()
                                .child("Component catalogue"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child("Browse the full Poodle component library. Each component handles accessibility, keyboard support, and theming."),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("{} components", filtered_count)),
                        )
                        .child(landing),
                ),
        )
    }


    /// Render a single specimen for a specific component by slug.
    fn render_component_specimen(&self, slug: &str, cx: &mut Context<Self>) -> Div {
        specimens::render_single_specimen(slug, &self.state, cx)
    }
}

/// Parsed CLI arguments.
struct CliArgs {
    section: Option<Section>,
    component: Option<String>,
    component_search: Option<String>,
    token_panel: Option<TokenPanel>,
    token_query: Option<String>,
    theme: Option<ThemePreset>,
    density: Option<Density>,
    control_size: Option<ControlSize>,
    screenshot: Option<String>,
    /// Points to click, in window coordinates, before capturing.
    clicks: Vec<DriverAction>,
    /// Print every specimen-state entry whose key starts with this, after the
    /// clicks land. Turns "is this component interactive" into an assertion.
    print_state: Option<String>,
    /// How long a synthetic click holds the button down, in milliseconds.
    ///
    /// Non-zero by default: a real click always outlives at least one
    /// repaint, and a driver that never does cannot see a bug that only
    /// appears when a press spans frames. `--hold 0` restores the old
    /// single-frame behaviour for runs that only need speed.
    hold_ms: u64,
}

fn parse_cli_args() -> CliArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut section = None;
    let mut component = None;
    let mut component_search = None;
    let mut token_panel = None;
    let mut token_query = None;
    let mut theme = None;
    let mut density = None;
    let mut control_size = None;
    let mut screenshot = None;
    let mut clicks: Vec<DriverAction> = Vec::new();
    let mut print_state = None;
    let mut hold_ms: u64 = 120;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--section" => {
                if let Some(val) = args.get(i + 1) {
                    section = match val.as_str() {
                        "components" | "primitives" | "composites" | "shells" => {
                            Some(Section::Components)
                        }
                        "tokens" => Some(Section::Tokens),
                        _ => None,
                    };
                    i += 1;
                }
            }
            "--component" => {
                if let Some(val) = args.get(i + 1) {
                    component = Some(val.clone());
                    i += 1;
                }
            }
            "--search" => {
                if let Some(val) = args.get(i + 1) {
                    component_search = Some(val.clone());
                    i += 1;
                }
            }
            "--token-panel" => {
                if let Some(val) = args.get(i + 1) {
                    token_panel = match val.as_str() {
                        "token-summary-section" | "summary" => Some(TokenPanel::Summary),
                        "token-inspector" | "inspector" => Some(TokenPanel::Inspector),
                        _ => None,
                    };
                    i += 1;
                }
            }
            "--token-query" => {
                if let Some(val) = args.get(i + 1) {
                    token_query = Some(val.clone());
                    i += 1;
                }
            }
            "--theme" => {
                if let Some(val) = args.get(i + 1) {
                    theme = match val.as_str() {
                        "graphite" => Some(ThemePreset::Graphite),
                        "eclipse" => Some(ThemePreset::Eclipse),
                        "iceberg" => Some(ThemePreset::Iceberg),
                        "midnight" => Some(ThemePreset::Midnight),
                        "nord" => Some(ThemePreset::Nord),
                        "rose" => Some(ThemePreset::Rose),
                        "forest" => Some(ThemePreset::Forest),
                        "solarized" => Some(ThemePreset::Solarized),
                        "hornet" => Some(ThemePreset::Hornet),
                        "cobalt" => Some(ThemePreset::Cobalt),
                        "clay" => Some(ThemePreset::Clay),
                        "meadow" => Some(ThemePreset::Meadow),
                        "default" => Some(ThemePreset::Default),
                        _ => None,
                    };
                    i += 1;
                }
            }
            "--density" => {
                if let Some(val) = args.get(i + 1) {
                    density = match val.as_str() {
                        "compact" => Some(Density::Compact),
                        "default" => Some(Density::Default),
                        "comfortable" => Some(Density::Comfortable),
                        _ => None,
                    };
                    i += 1;
                }
            }
            "--size" => {
                if let Some(val) = args.get(i + 1) {
                    control_size = match val.as_str() {
                        "xs" => Some(ControlSize::Xs),
                        "sm" => Some(ControlSize::Sm),
                        "md" => Some(ControlSize::Md),
                        "lg" => Some(ControlSize::Lg),
                        "xl" => Some(ControlSize::Xl),
                        _ => None,
                    };
                    i += 1;
                }
            }
            "--screenshot" => {
                if let Some(val) = args.get(i + 1) {
                    screenshot = Some(val.clone());
                    i += 1;
                }
            }
            // Repeatable: `--click 120,340 --click 120,400` clicks in order.
            // `--click X,Y` or `--click X,Y,N` for an N-times click (2 is a
            // double click, which is how word-select is driven).
            "--click" => {
                if let Some(val) = args.get(i + 1) {
                    let parts: Vec<&str> = val.split(',').collect();
                    match parts.as_slice() {
                        [x, y] | [x, y, _] => {
                            let count = match parts.get(2) {
                                Some(n) => n.trim().parse::<isize>().unwrap_or(1),
                                None => 1,
                            };
                            match (x.trim().parse::<f32>(), y.trim().parse::<f32>()) {
                                (Ok(x), Ok(y)) => {
                                    clicks.push(DriverAction::Click(point(px(x), px(y)), count))
                                }
                                _ => eprintln!("--click expects X,Y[,N] in pixels, got {val:?}"),
                            }
                        }
                        _ => eprintln!("--click expects X,Y[,N] in pixels, got {val:?}"),
                    }
                    i += 1;
                }
            }
            // Repeatable: `--drag 100,50,200,50` presses at the first point,
            // walks to the second in steps, and releases.
            "--drag" => {
                if let Some(val) = args.get(i + 1) {
                    let parts: Vec<f32> = val
                        .split(',')
                        .filter_map(|part| part.trim().parse::<f32>().ok())
                        .collect();
                    if let [x1, y1, x2, y2] = parts[..] {
                        clicks.push(DriverAction::Drag(
                            point(px(x1), px(y1)),
                            point(px(x2), px(y2)),
                        ));
                    } else {
                        eprintln!("--drag expects X1,Y1,X2,Y2 in pixels, got {val:?}");
                    }
                    i += 1;
                }
            }
            // `--type hello` sends the text as key events to the focused
            // element — click the input first to focus it.
            "--type" => {
                if let Some(val) = args.get(i + 1) {
                    clicks.push(DriverAction::Type(val.clone()));
                    i += 1;
                }
            }
            // `--key cmd-v` sends one chord to the focused element.
            "--key" => {
                if let Some(val) = args.get(i + 1) {
                    clicks.push(DriverAction::Key(val.clone()));
                    i += 1;
                }
            }
            // `--hold 0` collapses a click back into one frame.
            "--hold" => {
                if let Some(val) = args.get(i + 1) {
                    match val.trim().parse::<u64>() {
                        Ok(ms) => hold_ms = ms,
                        Err(_) => eprintln!("--hold expects milliseconds, got {val:?}"),
                    }
                    i += 1;
                }
            }
            "--print-state" => {
                if let Some(val) = args.get(i + 1) {
                    print_state = Some(val.clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    CliArgs {
        section,
        component,
        component_search,
        token_panel,
        token_query,
        theme,
        density,
        control_size,
        screenshot,
        clicks,
        print_state,
        hold_ms,
    }
}

/// Set once the window has drawn `FRAMES_BEFORE_CAPTURE` frames.
///
/// Screenshot mode used to sleep a fixed 1.5s and capture whatever was on
/// screen. Usually enough; sometimes not, and a half-painted frame is
/// indistinguishable from a real rendering change once it is written into a
/// baseline. `Window::on_next_frame` is the renderer actually reporting, so
/// the capture waits on that instead of on a guess.
static FRAMES_DRAWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// How many drawn frames to wait for.
///
/// One is not enough: the first frame can land before layout has settled, so
/// this waits for the render to repeat itself a few times.
const FRAMES_BEFORE_CAPTURE: u32 = 3;

/// Floor on how soon a capture may be taken, regardless of frames drawn.
///
/// Frames-drawn alone is not readiness. Three frames land in about 50ms, and a
/// capture that early comes back at 1348x1478 instead of 2696x2396 — the window
/// has painted but has not yet been placed on the Retina backing store. The old
/// fixed 1.5s sleep was masking window *setup*, not just paint, which is why
/// replacing it wholesale halved the image.
///
/// So both conditions must hold: the renderer has drawn, and the window has had
/// time to settle.
const MIN_SETTLE: std::time::Duration = std::time::Duration::from_millis(900);

/// Post one synthetic mouse event into our own app's event queue.
///
/// `Window::dispatch_event` returns a crate-private type, so gpui's event
/// entry point cannot be called from outside the crate — Rust rejects even a
/// discarded call. This goes through AppKit instead: build the NSEvent a real
/// click produces and `postEvent:atStart:` it to ourselves. The run loop
/// dequeues it and routes by window number exactly like a real click —
/// hit testing, dispatch, handler — without the window ever being key, which
/// posted CGEvents needed and could not get: macOS focus-stealing prevention
/// keeps a script-launched app inactive, and its posted clicks are dropped
/// before they reach the app. Direct responder calls
/// (`[view mouseDown:]`) do not work either; only the queued route delivers.
///
/// `position` is in the same coordinate space the events are observed in —
/// see `calibrate` for how callers translate window-content coordinates into
/// it.
fn post_mouse_event(
    window: &mut Window,
    event_type: objc2_app_kit::NSEventType,
    position: Point<Pixels>,
) {
    post_mouse_event_counted(window, event_type, position, 1)
}

/// `click_count` is what makes a double click a double click: gpui reads it
/// straight off the NSEvent, so a synthetic click that always says 1 can never
/// drive word-select however fast it repeats.
fn post_mouse_event_counted(
    window: &mut Window,
    event_type: objc2_app_kit::NSEventType,
    position: Point<Pixels>,
    click_count: isize,
) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSEvent, NSEventModifierFlags, NSEventType};
    use objc2_foundation::NSPoint;

    let Some(mtm) = MainThreadMarker::new() else {
        eprintln!("click driver: not on the main thread");
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let ns_windows = app.windows();
    let Some(ns_window) = ns_windows.iter().next() else {
        eprintln!("click driver: no NSWindow to send to");
        return;
    };

    let location = NSPoint {
        x: f64::from(f32::from(position.x)),
        y: f64::from(f32::from(window.viewport_size().height - position.y)),
    };
    let pressure = if event_type == NSEventType::LeftMouseDown {
        1.0
    } else {
        0.0
    };
    let event =
        NSEvent::mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure(
            event_type,
            location,
            NSEventModifierFlags::empty(),
            0.0,
            ns_window.windowNumber(),
            None,
            0,
            click_count,
            pressure,
        );
    let Some(event) = event else {
        eprintln!(
            "click driver: NSEvent construction failed for {:?}",
            event_type
        );
        return;
    };
    app.postEvent_atStart(&event, false);
}

/// One input gesture from the CLI, in the order given.
#[derive(Clone)]
enum DriverAction {
    Click(Point<Pixels>, isize),
    Drag(Point<Pixels>, Point<Pixels>),
    Type(String),
    /// One keystroke with modifiers, e.g. `cmd-c`, `shift-left`.
    Key(String),
}

/// Dispatch a synthetic click: move, down, up.
///
/// The move comes first: hit testing keys off the last known pointer
/// position, so a down with no preceding move lands on a window that thinks
/// the pointer is somewhere else.
/// Press: move the pointer there, then hold the button down.
///
/// Split from the release so a rebuild can happen *while the button is down*,
/// which is what every real click does. Posting both in one frame made the
/// driver blind to state keyed on the press surviving a repaint — it passed
/// the node-backend's id-stability bug in both the broken and fixed states.
fn dispatch_press(window: &mut Window, position: Point<Pixels>, click_count: isize) {
    use objc2_app_kit::NSEventType;
    post_mouse_event(window, NSEventType::MouseMoved, position);
    post_mouse_event_counted(window, NSEventType::LeftMouseDown, position, click_count);
}

fn dispatch_release(window: &mut Window, position: Point<Pixels>, click_count: isize) {
    use objc2_app_kit::NSEventType;
    post_mouse_event_counted(window, NSEventType::LeftMouseUp, position, click_count);
}

/// Dispatch synthetic typing: one keyDown/keyUp pair per character, called
/// directly on gpui's NSView responder methods. Key events carry no
/// position, so unlike clicks there is nothing to route or calibrate — but
/// they do need AppKit's key-window status if posted to the queue, which a
/// script-launched app never gets. The direct responder call skips that:
/// gpui parses the event's charactersIgnoringModifiers and dispatches along
/// its own focus path, so the target element must be focused first (click
/// it).
/// Post one keystroke, with modifiers: `cmd-c`, `shift-left`, `backspace`.
///
/// `--type` covers plain characters; chords need this, and without it the
/// clipboard and shift-selection paths cannot be driven at all.
fn dispatch_key(chord: &str) {
    use objc2_app_kit::NSEventModifierFlags;

    let mut modifiers = NSEventModifierFlags::empty();
    let mut parts: Vec<&str> = chord.split('-').collect();
    let key = parts.pop().unwrap_or("");
    for part in parts {
        match part {
            "cmd" | "super" | "platform" => modifiers |= NSEventModifierFlags::Command,
            "shift" => modifiers |= NSEventModifierFlags::Shift,
            "alt" | "option" => modifiers |= NSEventModifierFlags::Option,
            "ctrl" | "control" => modifiers |= NSEventModifierFlags::Control,
            other => eprintln!("key driver: unknown modifier {other:?}"),
        }
    }

    // Named keys carry no character; gpui reads them off the keycode.
    let named = match key {
        "left" => Some(123),
        "right" => Some(124),
        "down" => Some(125),
        "up" => Some(126),
        "home" => Some(115),
        "end" => Some(119),
        "backspace" => Some(51),
        "delete" => Some(117),
        "escape" => Some(53),
        "enter" | "return" => Some(36),
        "tab" => Some(48),
        _ => None,
    };
    match named {
        Some(code) => post_key(code, "", modifiers),
        None => match key.chars().next().and_then(ansi_key_code) {
            Some(code) => post_key(code, key, modifiers),
            None => eprintln!("key driver: no keycode for {key:?}"),
        },
    }
}

fn post_key(key_code: u16, characters: &str, modifiers: objc2_app_kit::NSEventModifierFlags) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSEvent, NSEventType};
    use objc2_foundation::{NSPoint, NSString};

    let Some(mtm) = MainThreadMarker::new() else {
        eprintln!("key driver: not on the main thread");
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let ns_windows = app.windows();
    let Some(ns_window) = ns_windows.iter().next() else {
        eprintln!("key driver: no NSWindow to send to");
        return;
    };
    let window_number = ns_window.windowNumber();
    let chars = NSString::from_str(characters);
    for event_type in [NSEventType::KeyDown, NSEventType::KeyUp] {
        let event =
            NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
                event_type,
                NSPoint { x: 0.0, y: 0.0 },
                modifiers,
                0.0,
                window_number,
                None,
                &chars,
                &chars,
                false,
                key_code,
            );
        match event {
            Some(event) => app.postEvent_atStart(&event, false),
            None => eprintln!("key driver: NSEvent construction failed for keycode {key_code}"),
        }
    }
}

fn dispatch_type(text: &str) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSEvent, NSEventModifierFlags, NSEventType};
    use objc2_foundation::{NSPoint, NSString};

    let Some(mtm) = MainThreadMarker::new() else {
        eprintln!("type driver: not on the main thread");
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let ns_windows = app.windows();
    let Some(ns_window) = ns_windows.iter().next() else {
        eprintln!("type driver: no NSWindow to send to");
        return;
    };
    let Some(view) = ns_window
        .contentView()
        .and_then(|content| content.subviews().iter().next())
    else {
        eprintln!("type driver: no gpui view under the content view");
        return;
    };
    let window_number = ns_window.windowNumber();

    for ch in text.chars() {
        // gpui derives the typed character from the event's keyCode via the
        // active layout, ignoring the characters string — so the keycode has
        // to be right (ANSI-US). Unmapped characters are skipped loudly.
        let Some(key_code) = ansi_key_code(ch) else {
            eprintln!("type driver: no ANSI keycode for {ch:?}; skipped");
            continue;
        };
        let chars = NSString::from_str(&ch.to_string());
        for event_type in [NSEventType::KeyDown, NSEventType::KeyUp] {
            let event =
                NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
                    event_type,
                    NSPoint { x: 0.0, y: 0.0 },
                    NSEventModifierFlags::empty(),
                    0.0,
                    window_number,
                    None,
                    &chars,
                    &chars,
                    false,
                    key_code,
                );
            let Some(event) = event else {
                eprintln!("type driver: NSEvent construction failed for {ch:?}");
                continue;
            };
            app.postEvent_atStart(&event, false);
        }
    }
    let _ = &view;
}

/// ANSI-US virtual keycode for an unshifted character. Lowercase letters,
/// digits, space and the common unshifted punctuation only — enough to drive
/// the text-editing proofs.
fn ansi_key_code(ch: char) -> Option<u16> {
    Some(match ch {
        'a' => 0,
        's' => 1,
        'd' => 2,
        'f' => 3,
        'h' => 4,
        'g' => 5,
        'z' => 6,
        'x' => 7,
        'c' => 8,
        'v' => 9,
        'b' => 11,
        'q' => 12,
        'w' => 13,
        'e' => 14,
        'r' => 15,
        'y' => 16,
        't' => 17,
        '1' => 18,
        '2' => 19,
        '3' => 20,
        '4' => 21,
        '6' => 22,
        '5' => 23,
        '=' => 24,
        '9' => 25,
        '7' => 26,
        '-' => 27,
        '8' => 28,
        '0' => 29,
        ']' => 30,
        'o' => 31,
        'u' => 32,
        '[' => 33,
        'i' => 34,
        'p' => 35,
        'l' => 37,
        'j' => 38,
        '\'' => 39,
        'k' => 40,
        ';' => 41,
        '\\' => 42,
        ',' => 43,
        '/' => 44,
        'n' => 45,
        'm' => 46,
        '.' => 47,
        ' ' => 49,
        _ => return None,
    })
}

/// Force the next queued event to see a fresh scene.
///
/// With the window occluded (screen locked, covered), gpui's display link is
/// stopped and `refresh()` alone never produces a frame — but
/// `dispatch_key_event` draws first when the window is dirty. A KeyUp is the
/// least intrusive key event: nothing in the tree listens for bare key-ups,
/// so this redraws without side effects, and the *next* click hit-tests the
/// scene the previous action produced instead of a stale one.
fn post_frame_flush(_window: &mut Window) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSEvent, NSEventModifierFlags, NSEventType};
    use objc2_foundation::{NSPoint, NSString};

    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let ns_windows = app.windows();
    let Some(ns_window) = ns_windows.iter().next() else {
        return;
    };
    let chars = NSString::from_str("a");
    let event =
        NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            NSEventType::KeyUp,
            NSPoint { x: 0.0, y: 0.0 },
            NSEventModifierFlags::empty(),
            0.0,
            ns_window.windowNumber(),
            None,
            &chars,
            &chars,
            false,
            0,
        );
    if let Some(event) = event {
        app.postEvent_atStart(&event, false);
    }
}

/// Dispatch a synthetic drag: press at `from`, walk to `to` in dragged-move
/// steps, release. gpui reads `LeftMouseDragged` as a mouse move with the
/// left button pressed, which is what its drag machinery keys off.
fn dispatch_drag(window: &mut Window, from: Point<Pixels>, to: Point<Pixels>) {
    use objc2_app_kit::NSEventType;
    post_mouse_event(window, NSEventType::MouseMoved, from);
    post_mouse_event(window, NSEventType::LeftMouseDown, from);
    const STEPS: i32 = 8;
    for step in 1..=STEPS {
        let t = step as f32 / STEPS as f32;
        let position = point(
            px(f32::from(from.x) + (f32::from(to.x) - f32::from(from.x)) * t),
            px(f32::from(from.y) + (f32::from(to.y) - f32::from(from.y)) * t),
        );
        post_mouse_event(window, NSEventType::LeftMouseDragged, position);
    }
    post_mouse_event(window, NSEventType::LeftMouseUp, to);
}

/// How a posted coordinate maps to the position gpui observes.
///
/// On displays running a scaled resolution, the position that arrives at the
/// view differs from the one posted by a per-axis affine transform (a 2×
/// backing store presented at a non-integer UI scale). Rather than model
/// AppKit's conversion, measure it: post two probe moves through the exact
/// same path the clicks take, read back `Window::mouse_position()`, and
/// solve. `apply` then pre-distorts click targets so they arrive at the
/// window-content coordinates the caller asked for — the space screenshots
/// are read in.
#[derive(Clone, Copy)]
struct ClickCalibration {
    scale_x: f32,
    scale_y: f32,
    offset_x: f32,
    offset_y: f32,
}

impl ClickCalibration {
    fn identity() -> Self {
        Self {
            scale_x: 1.0,
            scale_y: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    /// Solve `observed = posted * scale + offset` from two probe pairs.
    fn solve(
        p1_posted: Point<Pixels>,
        p1_seen: Point<Pixels>,
        p2_posted: Point<Pixels>,
        p2_seen: Point<Pixels>,
    ) -> Option<Self> {
        let dx_posted = f32::from(p2_posted.x - p1_posted.x);
        let dy_posted = f32::from(p2_posted.y - p1_posted.y);
        let dx_seen = f32::from(p2_seen.x - p1_seen.x);
        let dy_seen = f32::from(p2_seen.y - p1_seen.y);
        if dx_posted == 0.0 || dy_posted == 0.0 || dx_seen == 0.0 || dy_seen == 0.0 {
            return None;
        }
        let scale_x = dx_seen / dx_posted;
        let scale_y = dy_seen / dy_posted;
        Some(Self {
            scale_x,
            scale_y,
            offset_x: f32::from(p1_seen.x) - f32::from(p1_posted.x) * scale_x,
            offset_y: f32::from(p1_seen.y) - f32::from(p1_posted.y) * scale_y,
        })
    }

    /// The posted coordinate that will be observed at `target`.
    fn apply(&self, target: Point<Pixels>) -> Point<Pixels> {
        point(
            px((f32::from(target.x) - self.offset_x) / self.scale_x),
            px((f32::from(target.y) - self.offset_y) / self.scale_y),
        )
    }
}

/// Print the specimen state a caller asked about, so a test can assert on it
/// instead of diffing pixels.
fn print_specimen_state(window: &mut Window, cx: &mut App, prefix: &str) {
    let Some(Some(root)) = window.root::<PreviewRoot>() else {
        eprintln!("state: no root view");
        return;
    };

    // Node-backed specimens report interactions through a queue that renders
    // drain; a print that ran before the next frame would miss them, so
    // drain here too.
    root.update(cx, |root, _cx| root.state.drain_node_events());
    let state = root.read(cx);
    let mut lines: Vec<String> = Vec::new();

    for (key, value) in &state.state.specimens.toggles {
        if key.starts_with(prefix) {
            lines.push(format!("{key}={value}"));
        }
    }
    for (key, value) in &state.state.specimens.text {
        if key.starts_with(prefix) {
            lines.push(format!("{key}={value:?}"));
        }
    }
    for (key, value) in &state.state.specimens.selections {
        if key.starts_with(prefix) {
            lines.push(format!("{key}={value}"));
        }
    }
    for (key, value) in &state.state.specimens.counters {
        if key.starts_with(prefix) {
            lines.push(format!("{key}={value}"));
        }
    }

    // Sorted so the output is stable enough to assert on.
    lines.sort();
    let focus = if window.focused(cx).is_some() {
        "yes"
    } else {
        "no"
    };
    println!("STATE focused={focus} {}", lines.join(" "));
}

/// Drive the interaction: warm up, click, report, then flag the capture
/// thread.
///
/// This runs on timers, not frame callbacks. gpui stops a window's display
/// link the moment macOS reports it occluded — behind other windows, or on a
/// locked screen — and a script-launched preview usually is. A frame-chained
/// driver deadlocks there: the next callback waits on a frame that will never
/// be drawn. Clicks do not need frames — `sendEvent` dispatches against the
/// scene from the last draw — so the sequence only needs the initial paints,
/// which land before macOS decides the window is occluded.
///
/// Two consequences worth knowing:
/// - Between clicks the scene may be stale when no redraw can happen, so a
///   click that expands content shifts what a *later* click at a lower point
///   would hit. Order multi-click runs bottom-up, or from the scene each
///   click actually sees.
/// - A `--screenshot` taken with the display link stopped shows the last
///   drawn frame, not the post-click one. The `--print-state` line is the
///   assertion; the screenshot is best-effort. (Each click still calls
///   `refresh()`, so a visible window captures correctly.)
fn schedule_interaction(
    window: &mut Window,
    cx: &mut App,
    clicks: Vec<DriverAction>,
    print_state: Option<String>,
    hold_ms: u64,
) {
    window
        .spawn(cx, async move |cx| {
            // Let the initial paints land so there is a scene to hit.
            cx.background_executor()
                .timer(std::time::Duration::from_millis(400))
                .await;

            // Calibrate: two probe moves through the click path, read back
            // where gpui observed them, and solve the affine transform.
            let mut calibration = ClickCalibration::identity();
            if !clicks.is_empty() {
                use objc2_app_kit::NSEventType;
                let p1_posted = point(px(100.0), px(100.0));
                let p2_posted = point(px(500.0), px(400.0));
                let mut seen = [point(px(0.0), px(0.0)); 2];
                for (i, probe) in [p1_posted, p2_posted].into_iter().enumerate() {
                    cx.update(|window, _cx| {
                        post_mouse_event(window, NSEventType::MouseMoved, probe);
                    })
                    .ok();
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(80))
                        .await;
                    if let Ok(observed) = cx.update(|window, _cx| window.mouse_position()) {
                        seen[i] = observed;
                    }
                }
                match ClickCalibration::solve(p1_posted, seen[0], p2_posted, seen[1]) {
                    Some(solved) => calibration = solved,
                    None => eprintln!(
                        "click driver: calibration probes were not observed; posting uncorrected coordinates"
                    ),
                }
            }

            for action in clicks {
                // A click is two events with a repaint between them. The hold
                // is awaited outside `cx.update`, so the window rebuilds while
                // the button is down.
                if let DriverAction::Click(position, count) = &action {
                    let position = calibration.apply(*position);
                    let count = *count;
                    cx.update(|window, _cx| {
                        dispatch_press(window, position, count);
                        window.refresh();
                        post_frame_flush(window);
                    })
                    .ok();
                    if hold_ms > 0 {
                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(hold_ms))
                            .await;
                    }
                    cx.update(|window, _cx| {
                        dispatch_release(window, position, count);
                        window.refresh();
                        post_frame_flush(window);
                    })
                    .ok();
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(150))
                        .await;
                    continue;
                }
                cx.update(|window, _cx| {
                    match &action {
                        DriverAction::Click(..) => unreachable!("handled above"),
                        DriverAction::Drag(from, to) => {
                            dispatch_drag(
                                window,
                                calibration.apply(*from),
                                calibration.apply(*to),
                            );
                        }
                        DriverAction::Type(text) => dispatch_type(text),
                        DriverAction::Key(chord) => dispatch_key(chord),
                    }
                    window.refresh();
                    post_frame_flush(window);
                })
                .ok();
                // A beat between clicks: handlers may defer work, and a
                // visible window gets a redraw in for the next hit test.
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(150))
                    .await;
            }
            cx.update(|window, cx| {
                if let Some(prefix) = print_state {
                    // After the clicks, before the capture: the state the
                    // clicks produced is the thing worth asserting.
                    print_specimen_state(window, cx, &prefix);
                }
                window.refresh();
            })
            .ok();
            // One more beat so an alive display link can draw the result
            // before the capture thread fires.
            cx.background_executor()
                .timer(std::time::Duration::from_millis(250))
                .await;
            FRAMES_DRAWN.store(true, std::sync::atomic::Ordering::Release);
        })
        .detach();
}

/// Chain `on_next_frame` `remaining` times, then flag the capture thread.
/// Used by pure screenshot runs, whose frames are the initial paints.
fn schedule_frames_drawn(window: &mut Window, remaining: u32) {
    if remaining == 0 {
        FRAMES_DRAWN.store(true, std::sync::atomic::Ordering::Release);
        return;
    }
    window.on_next_frame(move |window, _cx| {
        schedule_frames_drawn(window, remaining - 1);
    });
}

fn main() {
    let cli = parse_cli_args();
    let screenshot_mode = cli.screenshot.is_some();

    let assets = PreviewAssets {
        base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    };

    Application::new().with_assets(assets).run(move |cx: &mut App| {
        // Taken before the window closure consumes `cli`.
        let driver_screenshot = cli.screenshot.clone();
        let has_driver_actions = !cli.clicks.is_empty();
        // Load Inter font family — static weights for reliable rendering
        // (GPUI doesn't support variable font weight axes)
        let font_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/fonts");
        let fonts_to_load: Vec<std::borrow::Cow<'static, [u8]>> = [
            "Inter-Regular.ttf",    // 400
            "Inter-Medium.ttf",     // 500
            "Inter-SemiBold.ttf",   // 600
            "Inter-Bold.ttf",       // 700
        ]
        .iter()
        .filter_map(|name| {
            std::fs::read(font_dir.join(name))
                .ok()
                .map(std::borrow::Cow::Owned)
        })
        .collect();

        if !fonts_to_load.is_empty() {
            cx.text_system().add_fonts(fonts_to_load).ok();
        }

        // Register keyboard shortcuts
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("cmd-w", CloseWindow, None),
        ]);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.on_action(|_: &CloseWindow, cx| cx.quit());

        // Use a taller window in screenshot mode so all specimen sections fit.
        let window_height = if cli.screenshot.is_some() { 1600.0 } else { 800.0 };
        let bounds = Bounds::centered(None, size(px(1280.0), px(window_height)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |window, cx| {
                // Screenshot mode waits on this rather than on a fixed delay:
                // it flips once the window reports several frames actually
                // drawn. A guessed 1.5s sometimes captured a half-painted
                // frame, which is what put a `segmented-control` baseline into
                // the repo with its selected segment barely visible.
                if !cli.clicks.is_empty() || cli.print_state.is_some() {
                    schedule_interaction(
                        window,
                        cx,
                        cli.clicks.clone(),
                        cli.print_state.clone(),
                        cli.hold_ms,
                    );
                } else if screenshot_mode {
                    // A centered screenshot window can open under the physical
                    // pointer and freeze an arbitrary hover state into the
                    // baseline. Move GPUI's pointer position outside the
                    // content before counting settled frames.
                    post_mouse_event(
                        window,
                        objc2_app_kit::NSEventType::MouseMoved,
                        point(px(-100.0), px(-100.0)),
                    );
                    schedule_frames_drawn(window, FRAMES_BEFORE_CAPTURE);
                }

                cx.new(move |cx| {
                    let mut root = PreviewRoot::new(cx);

                    // Apply CLI overrides — display controls
                    // Set all values first, then rebuild once so density and
                    // control size are layered correctly.
                    if let Some(preset) = cli.theme {
                        root.state.theme_preset = preset;
                    }
                    if let Some(d) = cli.density {
                        root.state.density = d;
                    }
                    if let Some(s) = cli.control_size {
                        root.state.control_size = s;
                    }
                    // Rebuild theme with all overrides applied together
                    root.state.rebuild_theme();

                    // Apply CLI overrides — navigation
                    if let Some(section) = cli.section {
                        root.state.section = section;
                    }

                    if let Some(ref search) = cli.component_search {
                        root.state.component_search = search.clone();
                    }

                    if let Some(panel) = cli.token_panel {
                        root.state.active_token_panel = panel;
                    }

                    if let Some(ref query) = cli.token_query {
                        root.state.token_inspector_query = query.clone();
                    }

                    if let Some(ref slug) = cli.component {
                        // Auto-detect component presence from the unified catalogue.
                        if find_component(slug).is_some() {
                            root.state.section = Section::Components;
                            root.state.active_component_slug = Some(slug.clone());
                        }
                    }

                    root
                })
            },
        )
        .unwrap();

        // A click run needs frames to keep flowing: macOS pauses drawing for
        // occluded windows, and a script-launched window usually opens behind
        // whatever the user is doing. Order it front without taking focus —
        // `orderFrontRegardless` works from an inactive app, where activate
        // is blocked by focus-stealing prevention.
        if has_driver_actions {
            cx.defer(move |_cx| {
                use objc2::MainThreadMarker;
                use objc2_app_kit::NSApplication;
                if let Some(mtm) = MainThreadMarker::new() {
                    let app = NSApplication::sharedApplication(mtm);
                    if let Some(ns_window) = app.windows().iter().next() {
                        ns_window.orderFrontRegardless();
                    }
                }
            });
        }

        // Screenshot mode: spawn a background thread that waits for render,
        // captures the window by PID (without stealing focus), saves, and exits.
        if driver_screenshot.is_some() || has_driver_actions {
            let path = driver_screenshot.clone();
            std::thread::spawn(move || {
                // Wait for the renderer to say it has drawn, rather than
                // guessing how long that takes. Falls back to a deadline so a
                // window that never draws cannot hang the run forever.
                let started = std::time::Instant::now();
                let deadline = started + std::time::Duration::from_secs(20);
                loop {
                    let drawn = FRAMES_DRAWN.load(std::sync::atomic::Ordering::Acquire);
                    if drawn && started.elapsed() >= MIN_SETTLE {
                        break;
                    }
                    if std::time::Instant::now() > deadline {
                        eprintln!("timed out waiting for a settled frame");
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }

                // Clicks land after the window has settled — a window that
                // has not laid out yet has nothing to hit.
                let Some(path) = path else {
                    // Clicks with no screenshot: the state print is the output.
                    std::process::exit(0);
                };

                // Find our own window by PID
                let pid = std::process::id();
                let find_wid = std::process::Command::new("swift")
                    .arg("-e")
                    .arg(format!(
                        concat!(
                            "import CoreGraphics\n",
                            "let wl = CGWindowListCopyWindowInfo(.optionAll, kCGNullWindowID) as! [[String: Any]]\n",
                            "var best = 0; var bestArea = 0\n",
                            "for w in wl {{\n",
                            "  let p = w[\"kCGWindowOwnerPID\"] as? Int ?? 0\n",
                            "  if p == {} {{\n",
                            "    let b = w[\"kCGWindowBounds\"] as? [String: Any] ?? [:]\n",
                            "    let h = b[\"Height\"] as? Int ?? 0\n",
                            "    let w2 = b[\"Width\"] as? Int ?? 0\n",
                            "    if h * w2 > bestArea {{ bestArea = h * w2; best = w[\"kCGWindowNumber\"] as? Int ?? 0 }}\n",
                            "  }}\n",
                            "}}\n",
                            "print(best)",
                        ),
                        pid
                    ))
                    .output();

                if let Ok(output) = find_wid {
                    let wid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !wid_str.is_empty() && wid_str != "0" {
                        let result = std::process::Command::new("screencapture")
                            .args(["-x", "-l", &wid_str, &path])
                            .status();
                        match result {
                            Ok(s) if s.success() => eprintln!("Screenshot saved: {}", path),
                            Ok(s) => eprintln!("screencapture failed with status: {}", s),
                            Err(e) => eprintln!("screencapture error: {}", e),
                        }
                    } else {
                        eprintln!("Could not find window (PID {})", pid);
                    }
                }

                // Exit the process
                std::process::exit(0);
            });
        } else {
            cx.activate(true);
        }
    });
}
