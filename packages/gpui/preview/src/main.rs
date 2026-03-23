//! Pug GPUI Preview — native macOS preview application for the Pug design system.
//!
//! Matches the Svelte preview app layout: top bar with pill tabs,
//! display controls bar, and section-specific content areas.

mod app_state;
mod component_registry;
mod demo_view;
mod specimens;
#[allow(dead_code)]
mod style_bridge;
mod token_view;

use std::borrow::Cow;
use std::path::PathBuf;

use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::{TabDefinition, TabsSpec, TabVariant};

/// Asset source that loads files from the preview app's directory.
struct PreviewAssets {
    base: PathBuf,
}

impl AssetSource for PreviewAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        let full_path = self.base.join(path);
        match std::fs::read(&full_path) {
            Ok(data) => Ok(Some(Cow::Owned(data))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let full_path = self.base.join(path);
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
    AppState, AppearanceTreatment, ControlSize, DemoScreen, Density, Section, ThemePreset,
};
use component_registry::{COMPOSITES, PRIMITIVES, SHELLS};
use pug_gpui_components::Tabs;
use style_bridge::color_to_hsla;

// Global keyboard actions
actions!(pug_preview, [Quit, CloseWindow]);

/// Root view for the preview application.
struct PreviewRoot {
    state: AppState,
}

impl PreviewRoot {
    fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }
}

impl Render for PreviewRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.state.theme;

        let canvas_bg = theme.resolve_color("semantic.color.background.canvas");
        let elevated_bg = theme.resolve_color("semantic.color.background.elevated");
        let panel_bg = theme.resolve_color("semantic.color.background.panel");
        let text_primary = theme.resolve_color("semantic.color.text.primary");
        let text_secondary = theme.resolve_color("semantic.color.text.secondary");
        let accent = theme.resolve_color("semantic.color.accent.base");
        let border_subtle = theme.resolve_color("semantic.color.border.subtle");
        let border = theme.resolve_color("semantic.color.border.default");

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
                    .child(div().text_size(px(16.0)).font_weight(FontWeight::BOLD).child("Pug"))
                    // Nav tabs (pill style)
                    .child(self.render_nav_tabs(text_secondary, accent, cx))
                    // Spacer
                    .child(div().flex_1())
                    // Right pills showing current settings
                    .child(self.render_status_pills(text_secondary, border)),
            )
            // ── Display controls bar ─────────────────────────────────
            .child(
                self.render_display_controls(
                    text_secondary,
                    accent,
                    border,
                    border_subtle,
                    panel_bg,
                    controls_h,
                    cx,
                ),
            )
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
        _text_secondary: pug_tokens::typed::ColorValue,
        _accent: pug_tokens::typed::ColorValue,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let active_value = self.state.section.label();
        let tab_defs: Vec<TabDefinition> = Section::ALL
            .iter()
            .map(|s| TabDefinition::new(s.label(), s.label()))
            .collect();

        let spec = TabsSpec::new(tab_defs)
            .with_variant(TabVariant::Pill)
            .with_value(active_value);

        Tabs::from_spec(spec, &self.state.theme)
            .with_id("nav-tabs")
            .on_change(cx.listener(|this, val: &str, _w, cx| {
                match val {
                    "Primitives" => this.state.section = Section::Primitives,
                    "Composites" => this.state.section = Section::Composites,
                    "Shells" => this.state.section = Section::Shells,
                    "Demo" => this.state.section = Section::Demo,
                    "Tokens" => this.state.section = Section::Tokens,
                    _ => {}
                }
                cx.notify();
            }))
    }

    /// Right-aligned pills showing current theme, density, and size.
    /// Matches Svelte: 11px text, 999px radius, 1px subtle border, 3px 8px padding.
    fn render_status_pills(
        &self,
        text_secondary: pug_tokens::typed::ColorValue,
        border: pug_tokens::typed::ColorValue,
    ) -> Div {
        // Svelte computed: bg srgb(0.094, 0.094, 0.094 / 0.9), border srgb(0.776, 0.776, 0.776 / 0.116)
        let canvas_bg = self.state.theme.resolve_color("semantic.color.background.canvas");

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

    /// Display controls bar — theme, density, size, treatment toggle groups + state probes.
    /// Matches Svelte: 80px height, panel bg, 12px 16px padding, 20px/32px gap.
    fn render_display_controls(
        &self,
        text_secondary: pug_tokens::typed::ColorValue,
        accent: pug_tokens::typed::ColorValue,
        border: pug_tokens::typed::ColorValue,
        border_subtle: pug_tokens::typed::ColorValue,
        panel_bg: pug_tokens::typed::ColorValue,
        h: Pixels,
        cx: &mut Context<Self>,
    ) -> Div {
        div()
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
            .flex_shrink_0()
            .overflow_hidden()
            // Theme group
            .child({
                let opts: Vec<(&str, bool)> = ThemePreset::ALL
                    .iter()
                    .map(|p| (p.label(), self.state.theme_preset == *p))
                    .collect();
                self.render_toggle_group("Theme", text_secondary, &opts, accent, border, "theme", cx)
            })
            // Density group
            .child({
                let opts: Vec<(&str, bool)> = Density::ALL
                    .iter()
                    .map(|d| (d.label(), self.state.density == *d))
                    .collect();
                self.render_toggle_group("Density", text_secondary, &opts, accent, border, "density", cx)
            })
            // Size group
            .child({
                let opts: Vec<(&str, bool)> = ControlSize::ALL
                    .iter()
                    .map(|s| (s.label(), self.state.control_size == *s))
                    .collect();
                self.render_toggle_group("Size", text_secondary, &opts, accent, border, "size", cx)
            })
            // Treatment group
            .child({
                let opts: Vec<(&str, bool)> = AppearanceTreatment::ALL
                    .iter()
                    .map(|t| (t.label(), self.state.appearance_treatment == *t))
                    .collect();
                self.render_toggle_group("Treatment", text_secondary, &opts, accent, border, "treatment", cx)
            })
            // State probes
            .child(self.render_state_probes(text_secondary, accent, border, cx))
    }

    /// A labelled toggle group (uppercase eyebrow + row of individual toggle buttons).
    /// Matches Svelte: each button is a separate pill with its own border.
    fn render_toggle_group(
        &self,
        label: &'static str,
        text_secondary: pug_tokens::typed::ColorValue,
        options: &[(&str, bool)],
        accent: pug_tokens::typed::ColorValue,
        _border: pug_tokens::typed::ColorValue,
        group_id: &'static str,
        cx: &mut Context<Self>,
    ) -> Div {
        let text_primary = self.state.theme.resolve_color("semantic.color.text.primary");
        let border_default = self.state.theme.resolve_color("semantic.color.border.default");
        let canvas_bg = self.state.theme.resolve_color("semantic.color.background.canvas");

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
                    }
                    "size" => {
                        this.state.control_size = ControlSize::ALL[i];
                    }
                    "treatment" => {
                        this.state.appearance_treatment = AppearanceTreatment::ALL[i];
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

    /// State probe checkboxes (disabled, invalid, busy).
    fn render_state_probes(
        &self,
        text_secondary: pug_tokens::typed::ColorValue,
        accent: pug_tokens::typed::ColorValue,
        border: pug_tokens::typed::ColorValue,
        cx: &mut Context<Self>,
    ) -> Div {
        let check = |label: &'static str, checked: bool, on_toggle: fn(&mut AppState)| {
            let box_el = div()
                .w(px(14.0))
                .h(px(14.0))
                .rounded(px(3.0))
                .border_1()
                .flex()
                .items_center()
                .justify_center();

            let box_el = if checked {
                box_el
                    .bg(color_to_hsla(accent))
                    .border_color(color_to_hsla(accent))
                    .text_color(gpui::white())
                    .text_xs()
                    .child("✓")
            } else {
                box_el.border_color(color_to_hsla(border))
            };

            div()
                .id(SharedString::from(format!("probe-{}", label)))
                .flex()
                .items_center()
                .gap(px(4.0))
                .cursor_pointer()
                .child(box_el)
                .child(
                    div()
                        .text_xs()
                        .text_color(color_to_hsla(text_secondary))
                        .child(label),
                )
                .on_click(cx.listener(move |this, _event: &ClickEvent, _window, cx| {
                    on_toggle(&mut this.state);
                    cx.notify();
                }))
        };

        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color_to_hsla(text_secondary))
                    .child("STATE PROBES"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(16.0))
                    .child(check("Disabled", self.state.disabled, |s| {
                        s.disabled = !s.disabled
                    }))
                    .child(check("Invalid", self.state.invalid, |s| {
                        s.invalid = !s.invalid
                    }))
                    .child(check("Busy", self.state.busy, |s| s.busy = !s.busy)),
            )
    }

    /// Section content router.
    /// `available_h` is the pixel height remaining after top bar + controls.
    fn render_section_content(&self, available_h: Pixels, cx: &mut Context<Self>) -> Div {
        let theme = &self.state.theme;
        match self.state.section {
            Section::Primitives => self.render_catalogue_section(PRIMITIVES, Section::Primitives, available_h, cx),
            Section::Composites => self.render_catalogue_section(COMPOSITES, Section::Composites, available_h, cx),
            Section::Shells => self.render_catalogue_section(SHELLS, Section::Shells, available_h, cx),
            Section::Demo => self.render_demo_section(available_h, cx),
            Section::Tokens => {
                div().w_full().h(available_h).child(
                    div()
                        .id("tokens-section")
                        .size_full()
                        .overflow_y_scroll()
                        .child(token_view::render_token_inspector(theme)),
                )
            }
        }
    }

    /// Two-column layout: sidebar listing components + content area.
    /// Both sidebar and content get explicit pixel heights so their
    /// overflow_y_scroll content-masks have definite bounds for hit testing.
    fn render_catalogue_section(
        &self,
        components: &'static [component_registry::ComponentEntry],
        which: Section,
        available_h: Pixels,
        cx: &mut Context<Self>,
    ) -> Div {
        let theme = &self.state.theme;
        let border_subtle = theme.resolve_color("semantic.color.border.subtle");
        let text_primary = theme.resolve_color("semantic.color.text.primary");
        let text_secondary = theme.resolve_color("semantic.color.text.secondary");
        let accent = theme.resolve_color("semantic.color.accent.base");
        let elevated_bg = theme.resolve_color("semantic.color.background.elevated");

        let active_idx = match which {
            Section::Primitives => self.state.active_primitive,
            Section::Composites => self.state.active_composite,
            Section::Shells => self.state.active_shell,
            _ => None,
        };

        // Sidebar — explicit height so overflow_y_scroll has definite bounds.
        let mut sidebar = div()
            .id("catalogue-sidebar")
            .w(px(224.0))
            .h(available_h)
            .flex_shrink_0()
            .flex()
            .flex_col()
            .py(px(12.0))
            .overflow_y_scroll()
            .border_r_1()
            .border_color(color_to_hsla(border_subtle).opacity(0.6));

        for (i, comp) in components.iter().enumerate() {
            let is_active = active_idx == Some(i);

            let mut link = div()
                .id(SharedString::from(format!("comp-{}", comp.slug)))
                .px(px(16.0))
                .py(px(6.0))
                .text_sm()
                .cursor_pointer()
                .border_l_2();

            link = if is_active {
                link.text_color(color_to_hsla(text_primary))
                    .border_color(color_to_hsla(accent))
                    .bg(color_to_hsla(accent).opacity(0.08))
            } else {
                link.text_color(color_to_hsla(text_secondary))
                    .border_color(hsla(0.0, 0.0, 0.0, 0.0))
            };

            link = link.child(comp.display_name);

            link = link.on_click(
                cx.listener(move |this, _event: &ClickEvent, _window, cx| {
                    match which {
                        Section::Primitives => this.state.active_primitive = Some(i),
                        Section::Composites => this.state.active_composite = Some(i),
                        Section::Shells => this.state.active_shell = Some(i),
                        _ => {}
                    }
                    cx.notify();
                }),
            );

            sidebar = sidebar.child(link);
        }

        // Outer layout: horizontal flex row with explicit height.
        let mut layout = div()
            .w_full()
            .h(available_h)
            .flex()
            .child(sidebar);

        if let Some(idx) = active_idx {
            let comp = &components[idx];
            layout = layout.child(
                div()
                    .id("specimen-content")
                    .flex_1()
                    .h(available_h)
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .p(px(24.0))
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(div().text_xl().child(comp.display_name))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(comp.description),
                            ),
                    )
                    .child(
                        div()
                            .h(px(1.0))
                            .w_full()
                            .bg(color_to_hsla(border_subtle)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(color_to_hsla(text_secondary))
                            .child("Specimen preview — rendered through pug-gpui adapter"),
                    )
                    .child(self.render_component_specimen(comp.slug, cx)),
            );
        } else {
            layout = layout.child(
                self.render_catalogue_landing(components, theme, available_h, text_secondary, border_subtle, elevated_bg),
            );
        }

        layout
    }

    /// Landing page grid showing all components as cards.
    fn render_catalogue_landing(
        &self,
        components: &[component_registry::ComponentEntry],
        _theme: &pug_gpui::GpuiThemeProvider,
        available_h: Pixels,
        text_secondary: pug_tokens::typed::ColorValue,
        border: pug_tokens::typed::ColorValue,
        elevated_bg: pug_tokens::typed::ColorValue,
    ) -> Div {
        let mut grid = div().flex().flex_wrap().gap(px(12.0));

        for comp in components {
            grid = grid.child(
                div()
                    .w(px(200.0))
                    .p(px(12.0))
                    .rounded(px(8.0))
                    .bg(color_to_hsla(elevated_bg))
                    .border_1()
                    .border_color(color_to_hsla(border))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(div().text_sm().child(comp.display_name))
                    .child(
                        div()
                            .text_xs()
                            .text_color(color_to_hsla(text_secondary))
                            .child(comp.description),
                    ),
            );
        }

        div()
            .flex_1()
            .h(available_h)
            .child(
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
                                    .text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("{} components", components.len())),
                            )
                            .child(grid),
                    ),
            )
    }

    /// Render a single specimen for a specific component by slug.
    fn render_component_specimen(
        &self,
        slug: &str,
        cx: &mut Context<Self>,
    ) -> Div {
        specimens::render_single_specimen(slug, &self.state, cx)
    }

    /// Demo section with segmented control for screen switching.
    fn render_demo_section(&self, available_h: Pixels, cx: &mut Context<Self>) -> Div {
        let theme = &self.state.theme;
        let text_secondary = theme.resolve_color("semantic.color.text.secondary");
        let accent = theme.resolve_color("semantic.color.accent.base");
        let border = theme.resolve_color("semantic.color.border.default");

        // Segmented control for demo screens
        let mut seg = div()
            .flex()
            .border_1()
            .border_color(color_to_hsla(border))
            .rounded(px(8.0))
            .overflow_hidden();

        for &screen in DemoScreen::ALL {
            let is_active = self.state.active_demo_screen == screen;
            let label = screen.label();

            let mut btn = div()
                .id(SharedString::from(format!("demo-{}", label)))
                .px(px(14.0))
                .py(px(6.0))
                .text_sm()
                .cursor_pointer()
                .child(label);

            btn = if is_active {
                btn.bg(color_to_hsla(accent).opacity(0.15))
                    .text_color(color_to_hsla(accent))
            } else {
                btn.text_color(color_to_hsla(text_secondary))
            };

            btn = btn.on_click(cx.listener(move |this, _event: &ClickEvent, _window, cx| {
                this.state.active_demo_screen = screen;
                cx.notify();
            }));

            seg = seg.child(btn);
        }

        let screen_content = demo_view::render_single_screen(theme, self.state.active_demo_screen);

        div().w_full().h(available_h).child(
            div()
                .id("demo-section")
                .size_full()
                .overflow_y_scroll()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.0))
                        .p(px(24.0))
                        .child(seg)
                        .child(screen_content),
                ),
        )
    }
}

/// Parsed CLI arguments.
struct CliArgs {
    section: Option<Section>,
    component: Option<String>,
    theme: Option<ThemePreset>,
    density: Option<Density>,
    control_size: Option<ControlSize>,
    treatment: Option<AppearanceTreatment>,
}

fn parse_cli_args() -> CliArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut section = None;
    let mut component = None;
    let mut theme = None;
    let mut density = None;
    let mut control_size = None;
    let mut treatment = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--section" => {
                if let Some(val) = args.get(i + 1) {
                    section = match val.as_str() {
                        "primitives" => Some(Section::Primitives),
                        "composites" => Some(Section::Composites),
                        "shells" => Some(Section::Shells),
                        "demo" => Some(Section::Demo),
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
            "--theme" => {
                if let Some(val) = args.get(i + 1) {
                    theme = match val.as_str() {
                        "loophole-studio" => Some(ThemePreset::LoopholeStudio),
                        "dark" => Some(ThemePreset::Dark),
                        "light" => Some(ThemePreset::Light),
                        "default" => Some(ThemePreset::Default),
                        _ => None,
                    };
                    i += 1;
                }
            }
            "--density" => {
                if let Some(val) = args.get(i + 1) {
                    density = match val.as_str() {
                        "comfortable" => Some(Density::Comfortable),
                        "compact" => Some(Density::Compact),
                        _ => None,
                    };
                    i += 1;
                }
            }
            "--size" => {
                if let Some(val) = args.get(i + 1) {
                    control_size = match val.as_str() {
                        "sm" => Some(ControlSize::Sm),
                        "md" => Some(ControlSize::Md),
                        "lg" => Some(ControlSize::Lg),
                        _ => None,
                    };
                    i += 1;
                }
            }
            "--treatment" => {
                if let Some(val) = args.get(i + 1) {
                    treatment = match val.as_str() {
                        "system" => Some(AppearanceTreatment::System),
                        "brand-raised" => Some(AppearanceTreatment::BrandRaised),
                        _ => None,
                    };
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    CliArgs { section, component, theme, density, control_size, treatment }
}

fn main() {
    let cli = parse_cli_args();

    let assets = PreviewAssets {
        base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    };

    Application::new().with_assets(assets).run(move |cx: &mut App| {
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
                .map(|data| std::borrow::Cow::Owned(data))
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

        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |_window, cx| {
                cx.new(move |_| {
                    let mut root = PreviewRoot::new();

                    // Apply CLI overrides — display controls
                    if let Some(preset) = cli.theme {
                        root.state.set_theme(preset);
                    }
                    if let Some(d) = cli.density {
                        root.state.density = d;
                    }
                    if let Some(s) = cli.control_size {
                        root.state.control_size = s;
                    }
                    if let Some(t) = cli.treatment {
                        root.state.appearance_treatment = t;
                    }

                    // Apply CLI overrides — navigation
                    if let Some(section) = cli.section {
                        root.state.section = section;
                    }

                    if let Some(ref slug) = cli.component {
                        // Auto-detect section and select the component
                        if let Some(idx) = PRIMITIVES.iter().position(|c| c.slug == slug.as_str()) {
                            root.state.section = Section::Primitives;
                            root.state.active_primitive = Some(idx);
                        } else if let Some(idx) = COMPOSITES.iter().position(|c| c.slug == slug.as_str()) {
                            root.state.section = Section::Composites;
                            root.state.active_composite = Some(idx);
                        } else if let Some(idx) = SHELLS.iter().position(|c| c.slug == slug.as_str()) {
                            root.state.section = Section::Shells;
                            root.state.active_shell = Some(idx);
                        }
                    }

                    root
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
