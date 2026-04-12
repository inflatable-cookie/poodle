//! Poodle GPUI Preview — native macOS preview application for the Poodle design system.
//!
//! Matches the Svelte preview app layout: top bar with pill tabs,
//! display controls bar, and section-specific content areas.

mod app_state;
mod component_registry;
mod contract_usage_docs;
mod demo_view;
mod specimens;
#[allow(dead_code)]
mod style_bridge;
mod token_view;

use std::borrow::Cow;
use std::path::PathBuf;

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_specs::{
    CodeSpec, ColumnAlign, ControlDensity as SpecControlDensity, ControlSize as SpecControlSize,
    SemanticControlSizeRole, SidebarNavGroup, SidebarNavItem, SidebarNavSpec, TabDefinition,
    TabVariant, TableColumn, TableRow, TableSpec, TabsSpec, TextInputSpec,
};

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
    TokenPanel,
};
use component_registry::{find_component, grouped_components, package_name};
use contract_usage_docs::{load_contract_usage_docs, ContractUsageDocs};
use poodle_gpui_components::{Code, SidebarNav, Table, Tabs, TextInput};
use style_bridge::color_to_hsla;

// Global keyboard actions
actions!(poodle_preview, [Quit, CloseWindow]);

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
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let active_value = self.state.section.label();
        let nav_sections = [Section::Components, Section::Tokens, Section::Treatments];
        let tab_defs: Vec<TabDefinition> = nav_sections
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
                    "Components" => this.state.section = Section::Components,
                    "Tokens" => this.state.section = Section::Tokens,
                    "Treatments" => this.state.section = Section::Treatments,
                    _ => {}
                }
                cx.notify();
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

    /// Display controls bar — theme, density, size, treatment toggle groups + catalogue search.
    /// Matches Svelte: 80px height, panel bg, 12px 16px padding, 20px/32px gap.
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
                self.render_toggle_group(
                    "Theme",
                    text_secondary,
                    &opts,
                    accent,
                    border,
                    "theme",
                    cx,
                )
            })
            // Density group
            .child({
                let opts: Vec<(&str, bool)> = Density::ALL
                    .iter()
                    .map(|d| (d.label(), self.state.density == *d))
                    .collect();
                self.render_toggle_group(
                    "Density",
                    text_secondary,
                    &opts,
                    accent,
                    border,
                    "density",
                    cx,
                )
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
                self.render_toggle_group(
                    "Treatment",
                    text_secondary,
                    &opts,
                    accent,
                    border,
                    "treatment",
                    cx,
                )
            })
            .child(
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
                            .child("SEARCH"),
                    )
                    .child(
                        TextInput::from_spec(
                            TextInputSpec::new()
                                .with_id("component-search")
                                .with_input_type("search")
                                .with_placeholder("Find component...")
                                .with_value(&self.state.component_search)
                                .with_aria_label("Search components"),
                            &self.state.theme,
                        )
                        .with_id("component-search")
                        .on_change(cx.listener(
                            |this, val: &str, _w, cx| {
                                this.state.component_search = val.to_string();
                                this.state.section = Section::Components;
                                cx.notify();
                            },
                        )),
                    ),
            )
    }

    /// A labelled toggle group (uppercase eyebrow + row of individual toggle buttons).
    /// Matches Svelte: each button is a separate pill with its own border.
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
                    "treatment" => {
                        this.state.appearance_treatment = AppearanceTreatment::ALL[i];
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
    fn render_section_content(&self, available_h: Pixels, cx: &mut Context<Self>) -> Div {
        let show_native_state = self.state.has_native_review_state();
        let native_state_h = if show_native_state { px(52.0) } else { px(0.0) };
        let section_h = available_h - native_state_h;

        let section_content = match self.state.section {
            Section::Components => self.render_components_section(section_h, cx),
            Section::Demo => self.render_demo_section(section_h, cx),
            Section::Tokens => self.render_tokens_section(section_h, cx),
            Section::Treatments => self.render_treatments_section(section_h),
        };

        div()
            .w_full()
            .h(available_h)
            .flex()
            .flex_col()
            .when(show_native_state, |el| {
                el.child(self.render_native_state_strip(native_state_h))
            })
            .child(section_content)
    }

    fn render_native_state_strip(&self, h: Pixels) -> Div {
        let theme = &self.state.theme;
        let border_subtle = theme.resolve_color("color.border.subtle");
        let panel_bg = theme.resolve_color("color.background.panel");
        let text_secondary = theme.resolve_color("color.text.secondary");
        let text_primary = theme.resolve_color("color.text.primary");
        let command = self.state.native_launch_command();

        div()
            .w_full()
            .h(h)
            .flex()
            .items_center()
            .justify_between()
            .gap(px(12.0))
            .px(px(16.0))
            .py(px(8.0))
            .bg(color_to_hsla(panel_bg))
            .border_b_1()
            .border_color(color_to_hsla(border_subtle))
            .child(
                div()
                    .flex_shrink_0()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color_to_hsla(text_secondary))
                    .child("Launch with current state"),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .overflow_x_hidden()
                    .text_ellipsis()
                    .font_family("SF Mono")
                    .text_size(px(12.0))
                    .text_color(color_to_hsla(text_primary))
                    .child(command),
            )
    }

    fn render_tokens_section(&self, available_h: Pixels, cx: &mut Context<Self>) -> Div {
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
                                        .child("@poodle/svelte-tokens"),
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
                            .on_change(cx.listener(
                                |this, val: &str, _w, cx| {
                                    this.state.active_token_panel = match val {
                                        "token-inspector" => TokenPanel::Inspector,
                                        _ => TokenPanel::Summary,
                                    };
                                    cx.notify();
                                },
                            )),
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
                                            .on_change(cx.listener(|this, val: &str, _w, cx| {
                                                this.state.token_inspector_query = val.to_string();
                                                cx.notify();
                                            })),
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
    fn render_components_section(&self, available_h: Pixels, cx: &mut Context<Self>) -> Div {
        let theme = &self.state.theme;
        let border_subtle = theme.resolve_color("color.border.subtle");
        let text_secondary = theme.resolve_color("color.text.secondary");
        let elevated_bg = theme.resolve_color("color.background.elevated");
        let groups = grouped_components(&self.state.component_search);
        let active_component = self
            .state
            .active_component_slug
            .as_deref()
            .and_then(find_component);
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
            .with_density(sidebar_nav_density(self.state.density))
            .with_size(sidebar_nav_size(self.state.control_size))
            .with_size_role(SemanticControlSizeRole::Chrome);
        if let Some(active_slug) = self.state.active_component_slug.as_deref() {
            sidebar_spec = sidebar_spec.with_value(active_slug);
        }
        let sidebar = div()
            .id("catalogue-sidebar")
            .w(px(224.0))
            .h(available_h)
            .flex_shrink_0()
            .overflow_y_scroll()
            .border_r_1()
            .border_color(color_to_hsla(border_subtle).opacity(0.6))
            .child(
                SidebarNav::from_spec(sidebar_spec, theme).on_select(cx.listener(
                    |this, val: &str, _window, cx| {
                        this.state.active_component_slug = Some(val.to_string());
                        cx.notify();
                    },
                )),
            );

        // Outer layout: horizontal flex row with explicit height.
        let mut layout = div().w_full().h(available_h).flex().child(sidebar);

        if let Some(component) = active_component {
            layout = layout.child(
                div()
                    .id("specimen-content")
                    .flex_1()
                    .h(available_h)
                    .flex()
                    .flex_col()
                    .gap(px(0.0))
                    .p(px(24.0))
                    .overflow_y_scroll()
                    .child(self.render_component_page_header(component))
                    .child(self.render_component_specimen(component.slug, cx))
                    .child(self.render_component_page_support(component)),
            );
        } else {
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
                container.child(self.render_contract_doc_summary(&contract_doc))
            })
    }

    fn render_contract_doc_summary(&self, contract_doc: &ContractUsageDocs) -> Div {
        let theme = &self.state.theme;
        let text_secondary = theme.resolve_color("color.text.secondary");
        let border_subtle = theme.resolve_color("color.border.subtle");

        let mut meta_parts = vec![contract_doc.path.clone()];
        if let Some(status) = contract_doc.status.as_ref() {
            meta_parts.push(status.clone());
        }
        if let Some(updated) = contract_doc.updated.as_ref() {
            meta_parts.push(format!("updated {}", updated));
        }

        let mut docs = div().flex().flex_col().gap(px(24.0)).child(
            div()
                .text_xs()
                .text_color(color_to_hsla(text_secondary))
                .child(meta_parts.join(" • ")),
        );

        if let Some(usage) = contract_doc.usage.as_ref() {
            docs = docs.child(self.render_doc_code_section("Usage", "md", usage));
        }

        if !contract_doc.props.is_empty() {
            docs = docs.child(self.render_props_table(contract_doc));
        }

        if !contract_doc.slots.is_empty() {
            docs = docs.child(self.render_slots_table(contract_doc));
        }

        if !contract_doc.events.is_empty() {
            docs = docs.child(self.render_events_table(contract_doc));
        }

        docs.child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)))
    }

    fn render_doc_code_section(
        &self,
        title: &'static str,
        language: &'static str,
        content: &str,
    ) -> Div {
        let theme = &self.state.theme;
        let text_primary = theme.resolve_color("color.text.primary");

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color_to_hsla(text_primary))
                    .child(title),
            )
            .child(Code::from_spec(
                CodeSpec::new()
                    .with_language(language)
                    .with_content(content)
                    .with_copyable(false),
                theme,
            ))
    }

    fn render_props_table(&self, contract_doc: &ContractUsageDocs) -> Div {
        let theme = &self.state.theme;
        let text_primary = theme.resolve_color("color.text.primary");
        let columns = vec![
            TableColumn::new("prop", "Prop").with_row_header(true),
            TableColumn::new("type", "Type"),
            TableColumn::new("default", "Default"),
            TableColumn::new("description", "Description"),
        ];
        let rows = contract_doc
            .props
            .iter()
            .enumerate()
            .map(|(index, prop)| {
                let name = if prop.required {
                    format!("{}*", prop.name)
                } else {
                    prop.name.clone()
                };
                TableRow::new(
                    format!("prop-{}", index),
                    vec![
                        ("prop".to_string(), name),
                        ("type".to_string(), prop.type_name.clone()),
                        (
                            "default".to_string(),
                            prop.default_value
                                .clone()
                                .unwrap_or_else(|| "—".to_string()),
                        ),
                        ("description".to_string(), prop.description.clone()),
                    ],
                )
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color_to_hsla(text_primary))
                    .child("Props"),
            )
            .child(Table::from_spec(
                TableSpec::new()
                    .with_columns(columns)
                    .with_rows(rows)
                    .with_aria_label("Component props")
                    .with_empty_message("No props documented."),
                theme,
            ))
    }

    fn render_slots_table(&self, contract_doc: &ContractUsageDocs) -> Div {
        let theme = &self.state.theme;
        let text_primary = theme.resolve_color("color.text.primary");
        let columns = vec![
            TableColumn::new("slot", "Slot").with_row_header(true),
            TableColumn::new("description", "Description"),
        ];
        let rows = contract_doc
            .slots
            .iter()
            .enumerate()
            .map(|(index, slot)| {
                TableRow::new(
                    format!("slot-{}", index),
                    vec![
                        ("slot".to_string(), slot.name.clone()),
                        ("description".to_string(), slot.description.clone()),
                    ],
                )
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color_to_hsla(text_primary))
                    .child("Slots"),
            )
            .child(Table::from_spec(
                TableSpec::new()
                    .with_columns(columns)
                    .with_rows(rows)
                    .with_aria_label("Component slots")
                    .with_empty_message("No slots documented."),
                theme,
            ))
    }

    fn render_events_table(&self, contract_doc: &ContractUsageDocs) -> Div {
        let theme = &self.state.theme;
        let text_primary = theme.resolve_color("color.text.primary");
        let columns = vec![
            TableColumn::new("event", "Event").with_row_header(true),
            TableColumn::new("payload", "Payload"),
            TableColumn::new("description", "Description").with_align(ColumnAlign::Start),
        ];
        let rows = contract_doc
            .events
            .iter()
            .enumerate()
            .map(|(index, event)| {
                TableRow::new(
                    format!("event-{}", index),
                    vec![
                        ("event".to_string(), event.name.clone()),
                        ("payload".to_string(), event.payload.clone()),
                        ("description".to_string(), event.description.clone()),
                    ],
                )
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color_to_hsla(text_primary))
                    .child("Events"),
            )
            .child(Table::from_spec(
                TableSpec::new()
                    .with_columns(columns)
                    .with_rows(rows)
                    .with_aria_label("Component events")
                    .with_empty_message("No events documented."),
                theme,
            ))
    }

    /// Landing page grid showing all components as cards.
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

    fn render_treatments_section(&self, available_h: Pixels) -> Div {
        let theme = &self.state.theme;
        let text_primary = theme.resolve_color("color.text.primary");
        let text_secondary = theme.resolve_color("color.text.secondary");
        let border_subtle = theme.resolve_color("color.border.subtle");
        let elevated_bg = theme.resolve_color("color.background.elevated");
        let panel_bg = theme.resolve_color("color.background.panel");
        let accent = theme.resolve_color("color.accent.base");

        let card =
            |eyebrow: &'static str, title: &'static str, body: &'static str, active: bool| {
                div()
                    .flex_1()
                    .min_w(px(220.0))
                    .p(px(16.0))
                    .rounded(px(8.0))
                    .bg(if active {
                        color_to_hsla(accent).opacity(0.07)
                    } else {
                        color_to_hsla(elevated_bg)
                    })
                    .border_1()
                    .border_color(if active {
                        color_to_hsla(accent).opacity(0.4)
                    } else {
                        color_to_hsla(border_subtle)
                    })
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_secondary))
                            .child(eyebrow),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_primary))
                            .child(title),
                    )
                    .child(
                        div()
                            .text_sm()
                            .line_height(relative(1.6))
                            .text_color(color_to_hsla(text_secondary))
                            .child(body),
                    )
            };

        let section_heading = |title: &'static str| {
            div()
                .text_lg()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(color_to_hsla(text_primary))
                .child(title)
        };

        let body = |text: &'static str| {
            div()
                .text_sm()
                .line_height(relative(1.6))
                .text_color(color_to_hsla(text_secondary))
                .child(text)
        };

        let code_panel = |content: &'static str| {
            Code::from_spec(
                CodeSpec::new()
                    .with_language("css")
                    .with_content(content)
                    .with_copyable(false),
                theme,
            )
        };

        let role_card = |name: &'static str,
                         description: &'static str,
                         variables: &'static [(&'static str, &'static str)],
                         components: &'static [&'static str]| {
            let mut role = div()
                .p(px(16.0))
                .rounded(px(8.0))
                .bg(color_to_hsla(elevated_bg))
                .border_1()
                .border_color(color_to_hsla(border_subtle))
                .flex()
                .flex_col()
                .gap(px(10.0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(color_to_hsla(text_primary))
                                .child(name),
                        )
                        .child(body(description)),
                );

            if !components.is_empty() {
                let mut component_row = div().flex().flex_wrap().gap(px(6.0));
                for component in components {
                    component_row = component_row.child(
                        div()
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded(px(999.0))
                            .border_1()
                            .border_color(color_to_hsla(border_subtle))
                            .bg(color_to_hsla(panel_bg))
                            .text_size(px(11.0))
                            .text_color(color_to_hsla(text_secondary))
                            .child((*component).to_string()),
                    );
                }
                role = role.child(component_row);
            }

            for (variable, purpose) in variables {
                role = role.child(
                    div()
                        .flex()
                        .gap(px(12.0))
                        .items_start()
                        .child(
                            div().min_w(px(280.0)).child(Code::from_spec(
                                CodeSpec::new()
                                    .with_content(*variable)
                                    .with_inline(true)
                                    .with_copyable(false),
                                theme,
                            )),
                        )
                        .child(
                            div()
                                .text_xs()
                                .line_height(relative(1.5))
                                .text_color(color_to_hsla(text_secondary))
                                .child((*purpose).to_string()),
                        ),
                );
            }

            role
        };

        let step = |number: &'static str, title: &'static str, text: &'static str| {
            div()
                .flex()
                .gap(px(12.0))
                .items_start()
                .child(
                    div()
                        .w(px(28.0))
                        .h(px(28.0))
                        .rounded(px(999.0))
                        .bg(color_to_hsla(accent).opacity(0.16))
                        .text_color(color_to_hsla(text_primary))
                        .font_weight(FontWeight::BOLD)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(number),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(color_to_hsla(text_primary))
                                .child(title),
                        )
                        .child(body(text)),
                )
        };

        let rule = |title: &'static str, text: &'static str| {
            div()
                .p(px(14.0))
                .rounded(px(8.0))
                .bg(color_to_hsla(elevated_bg))
                .border_1()
                .border_color(color_to_hsla(border_subtle))
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(color_to_hsla(text_primary))
                        .child(title),
                )
                .child(body(text))
        };

        div()
            .w_full()
            .h(available_h)
            .child(
                div()
                    .id("treatments-section")
                    .size_full()
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    .p(px(24.0))
                    .max_w(px(920.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_3xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(color_to_hsla(text_primary))
                                    .child("Treatment system"),
                            )
                            .child(body(
                                "Treatments sit between canonical semantic tokens and app-owned wrappers. They let downstream consumers apply cohesive visual branding across component families without redefining token meaning.",
                            )),
                    )
                    .child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)))
                    .child(section_heading("Three-Layer Architecture"))
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .flex_wrap()
                            .child(card(
                                "LAYER 1",
                                "Canonical Semantic Tokens",
                                "Typed, narrow values: color, spacing, radius. Never broadened to hold gradients or web-only effects.",
                                false,
                            ))
                            .child(card(
                                "LAYER 2",
                                "Appearance Recipes & Treatment Roles",
                                "Grouped visual overrides scoped to component families. They may include gradients, layered shadows, and other web-only effects.",
                                true,
                            ))
                            .child(card(
                                "LAYER 3",
                                "App-Owned Wrappers & Composites",
                                "Structural brand expression built by composing Poodle primitives without changing core token meaning.",
                                false,
                            )),
                    )
                    .child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)))
                    .child(section_heading("How Components Consume Treatments"))
                    .child(body(
                        "Components reference treatment variables using fallback chains. The treatment variable is tried first; if undefined, the semantic token value is used.",
                    ))
                    .child(code_panel(
                        ".text-input {\n  background: var(\n    --poodle-treatment-interactive-subtle-fill,\n    var(--poodle-color-background-surface)\n  );\n}",
                    ))
                    .child(body(
                        "This means components render with standard token values by default, and treatment values take precedence only when explicitly set. Components never need to know which specific treatment is active.",
                    ))
                    .child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)))
                    .child(section_heading("Treatment Roles"))
                    .child(body(
                        "Six family-level roles are defined. Components map these into local aliases rather than inventing per-component treatment vocabularies.",
                    ))
                    .child(role_card(
                        "interactive",
                        "General interactive surfaces such as secondary buttons, toggles, and menu triggers.",
                        &[
                            ("--poodle-treatment-interactive-radius", "Border radius"),
                            ("--poodle-treatment-interactive-fill", "Resting background"),
                            ("--poodle-treatment-interactive-fill-active", "Hover or active background"),
                            ("--poodle-treatment-interactive-border", "Resting border color"),
                            ("--poodle-treatment-interactive-border-active", "Hover or active border color"),
                        ],
                        &[
                            "Button (secondary)",
                            "IconButton (secondary)",
                            "SplitButton",
                            "ToggleGroup",
                            "SegmentedControl",
                        ],
                    ))
                    .child(role_card(
                        "interactive-primary",
                        "Primary action buttons and more prominent call-to-action surfaces.",
                        &[
                            ("--poodle-treatment-interactive-primary-radius", "Border radius"),
                            ("--poodle-treatment-interactive-primary-fill", "Resting background"),
                            ("--poodle-treatment-interactive-primary-fill-hover", "Hover background"),
                            ("--poodle-treatment-interactive-primary-border", "Resting border color"),
                            ("--poodle-treatment-interactive-primary-text", "Text and icon color"),
                        ],
                        &["Button (primary)", "SplitButton (primary)", "IconButton (primary)"],
                    ))
                    .child(role_card(
                        "interactive-subtle",
                        "Text inputs, selects, and search fields that keep their chrome restrained.",
                        &[
                            ("--poodle-treatment-interactive-subtle-radius", "Border radius"),
                            ("--poodle-treatment-interactive-subtle-fill", "Resting background"),
                            ("--poodle-treatment-interactive-subtle-fill-hover", "Hover background"),
                            ("--poodle-treatment-interactive-subtle-fill-focus", "Focus background"),
                            ("--poodle-treatment-interactive-subtle-border-focus", "Focus border"),
                        ],
                        &["TextInput", "Select"],
                    ))
                    .child(role_card(
                        "surface",
                        "Panel backgrounds, card frames, and general container surfaces.",
                        &[
                            ("--poodle-treatment-surface-radius", "Border radius"),
                            ("--poodle-treatment-surface-fill", "Background"),
                            ("--poodle-treatment-surface-border", "Border color"),
                            ("--poodle-treatment-surface-shadow", "Shadow"),
                            ("--poodle-treatment-surface-divider", "Internal divider color"),
                        ],
                        &["Surface", "Card", "MetricTile"],
                    ))
                    .child(role_card(
                        "surface-elevated",
                        "Elevated surfaces such as dialogs, drawers, popovers, and elevated cards.",
                        &[
                            ("--poodle-treatment-surface-elevated-radius", "Border radius"),
                            ("--poodle-treatment-surface-elevated-fill", "Background"),
                            ("--poodle-treatment-surface-elevated-border", "Border color"),
                            ("--poodle-treatment-surface-elevated-shadow", "Shadow"),
                        ],
                        &["Dialog", "Drawer", "Popover", "Menu", "HoverCard", "Tooltip"],
                    ))
                    .child(role_card(
                        "focus-ring",
                        "Focus-state treatment for keyboard indicators. It currently uses accent token posture directly and leaves room for future divergence.",
                        &[],
                        &[],
                    ))
                    .child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)))
                    .child(section_heading("Applying a Treatment"))
                    .child(body(
                        "Set a data appearance-treatment attribute on a container element. All descendants inherit treatment values through the scoped override layer.",
                    ))
                    .child(code_panel(
                        "<div data-appearance-treatment=\"brand-raised\">\n  <!-- All Poodle components inside inherit treatment values -->\n</div>",
                    ))
                    .child(body(
                        "Then define scoped overrides for the treatment variables when that attribute is present.",
                    ))
                    .child(code_panel(
                        "[data-appearance-treatment=\"brand-raised\"] {\n  --poodle-treatment-interactive-fill:\n    linear-gradient(180deg, rgba(255,255,255,0.14), transparent),\n    var(--poodle-color-background-elevated);\n  --poodle-treatment-interactive-primary-fill:\n    linear-gradient(180deg, rgba(255,255,255,0.24), transparent),\n    var(--poodle-color-accent-base);\n  --poodle-treatment-surface-fill:\n    linear-gradient(180deg, rgba(255,255,255,0.14), transparent),\n    var(--poodle-color-background-panel);\n  /* ... all other treatment variables */\n}",
                    ))
                    .child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)))
                    .child(section_heading("Creating a New Treatment"))
                    .child(step(
                        "1",
                        "Define treatment variables",
                        "Create a scoped override block that sets the treatment variables for every role you want to affect. Unset variables fall through to semantic-token defaults.",
                    ))
                    .child(step(
                        "2",
                        "Add theme-specific adjustments",
                        "Treatments may need per-theme tweaks, especially where shadows and contrast behave differently on light and dark backgrounds.",
                    ))
                    .child(step(
                        "3",
                        "Register in the preview app",
                        "Keep the treatment selectable in the preview controls so component review can exercise the same shell under different appearance recipes.",
                    ))
                    .child(div().h(px(1.0)).w_full().bg(color_to_hsla(border_subtle)))
                    .child(section_heading("Rules"))
                    .child(rule(
                        "Token purity",
                        "Semantic tokens must remain typed and narrow. Do not broaden a color token to hold a gradient.",
                    ))
                    .child(rule(
                        "Family-level roles",
                        "Prefer shared treatment roles over per-component treatment variables.",
                    ))
                    .child(rule(
                        "Fallback chain",
                        "Every treatment variable reference must include a semantic token fallback.",
                    ))
                    .child(rule(
                        "Gradient rule",
                        "Gradients are valid appearance treatments, not canonical colors.",
                    ))
                    .child(rule(
                        "Safe override boundary",
                        "Downstream apps may scope recipe overrides to subtrees. They must not redefine semantic token meaning.",
                    )),
            )
    }

    /// Render a single specimen for a specific component by slug.
    fn render_component_specimen(&self, slug: &str, cx: &mut Context<Self>) -> Div {
        specimens::render_single_specimen(slug, &self.state, cx)
    }

    /// Shared demo-contract target.
    ///
    /// This is intentionally separate from the current docs-shell parity surface.
    fn render_demo_section(&self, available_h: Pixels, cx: &mut Context<Self>) -> Div {
        let theme = &self.state.theme;
        let text_secondary = theme.resolve_color("color.text.secondary");
        let text_primary = theme.resolve_color("color.text.primary");
        let accent = theme.resolve_color("color.accent.base");
        let border = theme.resolve_color("color.border.default");
        let border_subtle = theme.resolve_color("color.border.subtle");
        let elevated_bg = theme.resolve_color("color.background.elevated");
        let panel_bg = theme.resolve_color("color.background.panel");
        let active_screen = self.state.active_demo_screen;

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

        let screen_content = demo_view::render_single_screen(theme, active_screen);

        let chip = |label: String| {
            div()
                .px(px(8.0))
                .py(px(3.0))
                .rounded(px(999.0))
                .border_1()
                .border_color(color_to_hsla(border))
                .bg(color_to_hsla(elevated_bg).opacity(0.9))
                .text_size(px(11.0))
                .text_color(color_to_hsla(text_secondary))
                .child(label)
        };

        let mut region_row = div().flex().flex_wrap().gap(px(6.0));
        for region in active_screen.region_ids() {
            region_row = region_row.child(chip((*region).to_string()));
        }

        let mut section_row = div().flex().flex_wrap().gap(px(6.0));
        for section in active_screen.source_sections() {
            section_row = section_row.child(chip((*section).to_string()));
        }

        let mut state_row = div().flex().flex_wrap().gap(px(6.0));
        for state in active_screen.state_matrix() {
            state_row = state_row.child(chip((*state).to_string()));
        }

        let contract_notice = div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .p(px(14.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(color_to_hsla(accent).opacity(0.42))
            .bg(color_to_hsla(accent).opacity(0.08))
            .child(
                div()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color_to_hsla(text_secondary))
                    .child("Internal demo target"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_primary))
                    .child("This surface tracks the contract-owned shared demo app target. It is separate from the current docs shell and should not be read as current cross-runtime docs-shell parity."),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .flex_wrap()
                    .child(chip("docs shell stays separate".to_string()))
                    .child(chip("contract-owned target".to_string()))
                    .child(chip("internal review surface".to_string())),
            );

        let context_toolbar = div()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(12.0))
            .p(px(12.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(color_to_hsla(border_subtle))
            .bg(color_to_hsla(panel_bg))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(color_to_hsla(text_secondary))
                            .child("Contract notes"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(color_to_hsla(text_primary))
                            .child(active_screen.summary()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(chip(format!("mode: {}", active_screen.comparison_mode())))
                    .child(chip(format!(
                        "sections: {}",
                        active_screen.source_sections().len()
                    )))
                    .child(chip(format!(
                        "states: {}",
                        active_screen.state_matrix().len()
                    ))),
            );

        let companion_panel = div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .p(px(16.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(color_to_hsla(border_subtle))
            .bg(color_to_hsla(elevated_bg).opacity(0.88))
            .child(
                div()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color_to_hsla(text_secondary))
                    .child("Contract notes"),
            )
            .child(
                div()
                    .text_lg()
                    .text_color(color_to_hsla(text_primary))
                    .child(active_screen.title()),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Shell regions"),
            )
            .child(region_row)
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child("Source sections from current docs shell"),
            )
            .child(section_row)
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child("State matrix"),
            )
            .child(state_row)
            .child(
                div()
                    .text_sm()
                    .text_color(color_to_hsla(text_secondary))
                    .child(if active_screen.has_modal_layer() {
                        "Modal layer is part of the contract scope for this screen and should stay visible during target-app review."
                    } else {
                        "This screen should read clearly as a contract target without relying on modal-layer posture."
                    }),
            );

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
                        .child(contract_notice)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(4.0))
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(color_to_hsla(text_secondary))
                                        .child("Screens"),
                                )
                                .child(seg),
                        )
                        .child(context_toolbar)
                        .child(
                            div()
                                .flex()
                                .items_start()
                                .gap(px(16.0))
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w(px(0.0))
                                        .flex()
                                        .flex_col()
                                        .gap(px(12.0))
                                        .child(screen_content),
                                )
                                .child(div().w(px(320.0)).flex_shrink_0().child(companion_panel)),
                        ),
                ),
        )
    }
}

/// Parsed CLI arguments.
struct CliArgs {
    section: Option<Section>,
    component: Option<String>,
    component_search: Option<String>,
    demo_screen: Option<DemoScreen>,
    token_panel: Option<TokenPanel>,
    token_query: Option<String>,
    theme: Option<ThemePreset>,
    density: Option<Density>,
    control_size: Option<ControlSize>,
    treatment: Option<AppearanceTreatment>,
    screenshot: Option<String>,
}

fn parse_cli_args() -> CliArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut section = None;
    let mut component = None;
    let mut component_search = None;
    let mut demo_screen = None;
    let mut token_panel = None;
    let mut token_query = None;
    let mut theme = None;
    let mut density = None;
    let mut control_size = None;
    let mut treatment = None;
    let mut screenshot = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--section" => {
                if let Some(val) = args.get(i + 1) {
                    section = match val.as_str() {
                        "components" | "primitives" | "composites" | "shells" => {
                            Some(Section::Components)
                        }
                        "demo" => Some(Section::Demo),
                        "tokens" => Some(Section::Tokens),
                        "treatments" => Some(Section::Treatments),
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
            "--demo-screen" => {
                if let Some(val) = args.get(i + 1) {
                    demo_screen = match val.as_str() {
                        "overview" => Some(DemoScreen::OverviewShell),
                        "form" => Some(DemoScreen::FormAndValidation),
                        "browse" => Some(DemoScreen::BrowseAndTable),
                        "detail" => Some(DemoScreen::DetailAndRelatedData),
                        "picker" => Some(DemoScreen::PickerAndMedia),
                        "workspace" => Some(DemoScreen::CommandAndWorkspace),
                        _ => None,
                    };
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
            "--screenshot" => {
                if let Some(val) = args.get(i + 1) {
                    screenshot = Some(val.clone());
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
        demo_screen,
        token_panel,
        token_query,
        theme,
        density,
        control_size,
        treatment,
        screenshot,
    }
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

        // Use a taller window in screenshot mode so all specimen sections fit.
        let window_height = if cli.screenshot.is_some() { 1600.0 } else { 800.0 };
        let bounds = Bounds::centered(None, size(px(1280.0), px(window_height)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |_window, cx| {
                cx.new(move |_| {
                    let mut root = PreviewRoot::new();

                    // Apply CLI overrides — display controls
                    // Set all values first, then rebuild once so density +
                    // control-size + treatment are all layered correctly.
                    if let Some(preset) = cli.theme {
                        root.state.theme_preset = preset;
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
                    // Rebuild theme with all overrides applied together
                    root.state.rebuild_theme();

                    // Apply CLI overrides — navigation
                    if let Some(section) = cli.section {
                        root.state.section = section;
                    }

                    if let Some(ref search) = cli.component_search {
                        root.state.component_search = search.clone();
                    }

                    if let Some(screen) = cli.demo_screen {
                        root.state.active_demo_screen = screen;
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

        // Screenshot mode: spawn a background thread that waits for render,
        // captures the window by PID (without stealing focus), saves, and exits.
        if let Some(ref output_path) = cli.screenshot {
            let path = output_path.clone();
            std::thread::spawn(move || {
                // Wait for initial render
                std::thread::sleep(std::time::Duration::from_millis(1500));

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
