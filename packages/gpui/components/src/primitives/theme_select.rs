use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize, ThemeOption, ThemeSelectSpec, SemanticControlSizeRole};

use super::Icon;
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{parse_hex_color, resolve_color, resolve_opacity, resolve_radius};
use poodle_specs::{IconSize, IconSpec};

/// ThemeSelect — theme picker backed by `ThemeSelectSpec`.
///
/// Contract: `docs/contracts/components/theme-select.md`. A trigger showing the
/// current theme's swatch opens a popover grid of theme swatch tiles. Swatch
/// colors are literal per-theme hex; chrome resolves from tokens. Selection +
/// applying the theme is host/controller work; render-only build-verified.
pub struct ThemeSelect {
    spec: ThemeSelectSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for ThemeSelect {
    type Target = ThemeSelectSpec;
    fn deref(&self) -> &ThemeSelectSpec {
        &self.spec
    }
}

impl ThemeSelect {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(ThemeSelectSpec::new(), theme)
    }

    pub fn from_spec(spec: ThemeSelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }

    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }

    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
}

/// Mini theme preview: canvas fill + surface card + accent dot + text bar. When
/// `selected`, an accent ring + check overlay is drawn.
fn swatch(option: &ThemeOption, theme: &GpuiThemeProvider, w: f32, h: f32, selected: bool) -> Div {
    let fallback = resolve_color(theme, "color.background.surface");
    let color = |hex: &str| parse_hex_color(hex).unwrap_or(fallback);
    let border = resolve_color(theme, "color.border.subtle");
    let accent = resolve_color(theme, "color.accent.base");

    let mut el = div()
        .relative()
        .w(px(rem_to_px(w)))
        .h(px(rem_to_px(h)))
        .rounded(px(rem_to_px(0.375)))
        .border_1()
        .border_color(if selected { accent } else { border })
        .bg(color(&option.swatch.canvas))
        .overflow_hidden()
        // Surface card.
        .child(
            div()
                .absolute()
                .left(px(rem_to_px(w * 0.14)))
                .bottom(px(0.0))
                .w(px(rem_to_px(w * 0.72)))
                .h(px(rem_to_px(h * 0.52)))
                .bg(color(&option.swatch.surface)),
        )
        // Accent dot.
        .child(
            div()
                .absolute()
                .top(px(rem_to_px(h * 0.18)))
                .left(px(rem_to_px(w * 0.16)))
                .w(px(rem_to_px(h * 0.26)))
                .h(px(rem_to_px(h * 0.26)))
                .rounded(px(999.0))
                .bg(color(&option.swatch.accent)),
        )
        // Text bar.
        .child(
            div()
                .absolute()
                .top(px(rem_to_px(h * 0.24)))
                .right(px(rem_to_px(w * 0.16)))
                .w(px(rem_to_px(w * 0.34)))
                .h(px(rem_to_px(0.125)))
                .rounded(px(999.0))
                .bg(color(&option.swatch.text)),
        );

    if selected {
        el = el.child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .text_color(accent)
                .child(Icon::from_spec(
                    IconSpec::new("check").with_size(IconSize::Sm),
                    theme,
                )),
        );
    }
    el
}

impl IntoElement for ThemeSelect {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let trigger_height = px(rem_to_px(match effective_size {
            ControlSize::Xs => 1.5,
            ControlSize::Sm => 1.75,
            ControlSize::Md => 2.25,
            ControlSize::Lg => 2.75,
            ControlSize::Xl => 3.25,
        }));

        let text_primary = resolve_color(theme, spec.field_text_token());
        let text_secondary = resolve_color(theme, spec.label_color_token());
        let border = resolve_color(theme, spec.field_border_token());
        let surface = resolve_color(theme, spec.field_fill_token());
        let elevated = resolve_color(theme, spec.surface_fill_token());
        let item_border = resolve_color(theme, spec.item_border_token());
        let accent = resolve_color(theme, spec.accent_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let surface_radius = resolve_radius(theme, spec.surface_radius_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        // ── Trigger ───────────────────────────────────────────────────────────
        let mut trigger = div()
            .flex()
            .items_center()
            .gap(px(rem_to_px(0.5)))
            .min_h(trigger_height)
            .px(px(rem_to_px(0.75)))
            .rounded(radius)
            .border_1()
            .border_color(border)
            .bg(surface)
            .text_color(text_primary);

        if let Some(current) = spec.current_option() {
            trigger = trigger.child(swatch(current, theme, 1.25, 1.25, false));
        }
        if spec.show_label {
            trigger = trigger.child(
                div()
                    .text_size(px(rem_to_px(0.8125)))
                    .child(spec.trigger_label()),
            );
        }
        trigger = trigger.child(div().text_color(text_secondary).child("▾"));

        let mut root = div().id("theme-select").relative().flex().child(trigger);

        // ── Popover grid (rendered inline when open) ──────────────────────────
        if spec.is_open {
            let mut grid = div().flex().flex_wrap().gap(px(rem_to_px(0.5))).max_w(px(rem_to_px(22.0)));
            for option in spec.themes.iter() {
                let selected = spec.is_selected(option);
                grid = grid.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(rem_to_px(0.375)))
                        .w(px(rem_to_px(4.5)))
                        .p(px(rem_to_px(0.375)))
                        .rounded(radius)
                        .border_1()
                        .border_color(if selected { accent } else { hsla(0.0, 0.0, 0.0, 0.0) })
                        .child(swatch(option, theme, 2.75, 2.0, selected))
                        .child(
                            div()
                                .text_size(px(rem_to_px(0.71875)))
                                .text_color(text_primary)
                                .child(option.label.clone()),
                        ),
                );
            }

            root = root.child(
                div()
                    .rounded(surface_radius)
                    .border_1()
                    .border_color(item_border)
                    .bg(elevated)
                    .p(px(rem_to_px(0.5)))
                    .child(grid),
            );
        }

        if spec.is_disabled {
            root = root.opacity(disabled_opacity);
        }

        root.into_any_element()
    }
}
