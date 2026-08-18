//! SpecimenLayout — the Examples / Sizes / Densities specimen container.
//!
//! Mirrors the Svelte `SpecimenLayout` component
//! (packages/svelte/preview/src/components/SpecimenLayout.svelte): a set of
//! curated Examples plus whichever axis panes the component actually takes.
//!
//! Axis admission is explicit. A page passes a [`SpecimenAxes`] value that
//! names the panes it supplies, and the helper renders exactly those tabs:
//!
//! ```ignore
//! specimen_layout(
//!     state, cx, "button",
//!     examples_body(theme),
//!     SpecimenAxes::examples_only()
//!         .with_sizes(|size, theme| Button::...with_size(size).into_any_element())
//!         .with_densities(|density, theme| Button::...with_density(density).into_any_element()),
//! )
//! ```
//!
//! A page the census admits for one axis calls only that builder method, and a
//! page it admits for neither passes `SpecimenAxes::examples_only()` alone.
//! There is no default: a page that supplies no renderer gets no axis tab, so
//! no pane can render empty.
//!
//! The current active tab is stored in `state.specimens.text` under the key
//! `specimen-layout-tab-<name>` so each specimen tracks its own tab
//! independently. A retained tab the page no longer admits normalises back to
//! Examples — see [`AxisAdmission::resolve_tab`].

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlDensity, ControlSize};

use crate::app_state::AppState;
use crate::specimens::specimen_axes::{
    AxisAdmission, ALL_DENSITIES, ALL_SIZES, DENSITIES_TAB, SIZES_TAB,
};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

type SizeRenderer<'a> = Box<dyn Fn(ControlSize, &GpuiThemeProvider) -> Option<AnyElement> + 'a>;
type DensityRenderer<'a> =
    Box<dyn Fn(ControlDensity, &GpuiThemeProvider) -> Option<AnyElement> + 'a>;
type NamedSizeRenderer<'a> = Box<dyn Fn(&str, &GpuiThemeProvider) -> AnyElement + 'a>;
type NamedDensityRenderer<'a> = Box<dyn Fn(&str, &GpuiThemeProvider) -> AnyElement + 'a>;

/// The axis panes a specimen page admits, together with their renderers.
///
/// Construct one per page. Every constructor is explicit about its axes, so a
/// page can neither inherit a pane it does not support nor advertise a tab it
/// cannot fill.
pub struct SpecimenAxes<'a> {
    sizes: Option<SizeRenderer<'a>>,
    named_sizes: Option<(Vec<&'static str>, NamedSizeRenderer<'a>)>,
    densities: Option<DensityRenderer<'a>>,
    named_densities: Option<(Vec<&'static str>, NamedDensityRenderer<'a>)>,
}

impl<'a> SpecimenAxes<'a> {
    /// Start from a page with no axis panes at all.
    ///
    /// This is the only constructor. A page admits `Sizes` or `Densities` by
    /// saying so, which is why neither axis can arrive by default and no
    /// admitted pane can be empty.
    pub fn examples_only() -> Self {
        Self {
            sizes: None,
            named_sizes: None,
            densities: None,
            named_densities: None,
        }
    }

    /// Admit the `Sizes` pane, one representative per control step.
    pub fn with_sizes(
        self,
        row: impl Fn(ControlSize, &GpuiThemeProvider) -> AnyElement + 'a,
    ) -> Self {
        self.with_sizes_where(move |size, theme| Some(row(size, theme)))
    }

    /// Admit the `Sizes` pane for a component whose own size enum stops short
    /// of the five control steps. The renderer returns `None` for a step the
    /// component does not take and that row is dropped rather than faked —
    /// matching what the Svelte specimen renders.
    pub fn with_sizes_where(
        mut self,
        row: impl Fn(ControlSize, &GpuiThemeProvider) -> Option<AnyElement> + 'a,
    ) -> Self {
        self.sizes = Some(Box::new(row));
        self
    }

    /// Admit the `Sizes` pane for a component whose public size domain is not
    /// the five control steps. The provided labels are rendered in order.
    pub fn with_named_sizes(
        mut self,
        values: &'static [&'static str],
        row: impl Fn(&str, &GpuiThemeProvider) -> AnyElement + 'a,
    ) -> Self {
        self.named_sizes = Some((values.to_vec(), Box::new(row)));
        self
    }

    /// Admit the `Densities` pane, one representative per density step.
    pub fn with_densities(
        self,
        row: impl Fn(ControlDensity, &GpuiThemeProvider) -> AnyElement + 'a,
    ) -> Self {
        self.with_densities_where(move |density, theme| Some(row(density, theme)))
    }

    /// Admit the `Densities` pane for a component that does not take every
    /// density step. See [`SpecimenAxes::with_sizes_where`].
    pub fn with_densities_where(
        mut self,
        row: impl Fn(ControlDensity, &GpuiThemeProvider) -> Option<AnyElement> + 'a,
    ) -> Self {
        self.densities = Some(Box::new(row));
        self
    }

    /// Admit the `Densities` pane for an explicit ordered density domain.
    pub fn with_named_densities(
        mut self,
        values: &'static [&'static str],
        row: impl Fn(&str, &GpuiThemeProvider) -> AnyElement + 'a,
    ) -> Self {
        self.named_densities = Some((values.to_vec(), Box::new(row)));
        self
    }

    /// Which tabs this page publishes.
    pub fn admission(&self) -> AxisAdmission {
        AxisAdmission {
            sizes: self.sizes.is_some() || self.named_sizes.is_some(),
            densities: self.densities.is_some() || self.named_densities.is_some(),
        }
    }
}

/// Render a specimen page: a tab strip over the admitted panes, then the body.
///
/// `name` scopes the tab-state key so each specimen remembers its own tab.
///
/// `examples` is a pre-built element containing the Examples pane body.
/// Passing a concrete element rather than a closure lets the caller freely
/// capture `cx` when wiring per-example click handlers.
pub fn specimen_layout(
    state: &AppState,
    cx: &mut Context<PreviewRoot>,
    name: &str,
    examples: AnyElement,
    axes: SpecimenAxes<'_>,
) -> Div {
    let theme = &state.theme;
    let admission = axes.admission();

    let key = format!("specimen-layout-tab-{name}");
    let active_tab = admission.resolve_tab(state.specimens.text.get(&key).map(String::as_str));

    let border_subtle = theme.resolve_color("color.border.subtle");
    let panel_bg = theme.resolve_color("color.background.panel");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");

    // ── Tab bar ──
    let mut tab_bar = div()
        .flex()
        .items_end()
        .gap(px(4.0))
        .border_b_1()
        .border_color(color_to_hsla(border_subtle));

    for (value, label) in admission.tabs() {
        let is_active = active_tab == value;
        let tab_color = if is_active {
            color_to_hsla(text_primary)
        } else {
            color_to_hsla(text_secondary)
        };

        let mut tab = div()
            .id(SharedString::from(format!("specimen-tab-{name}-{value}")))
            .px(px(12.0))
            .py(px(8.0))
            .text_size(px(13.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(tab_color)
            .cursor_pointer()
            .child(label);

        if is_active {
            tab = tab.border_b_2().border_color(color_to_hsla(accent));
        } else {
            tab = tab.border_b_2().border_color(hsla(0.0, 0.0, 0.0, 0.0));
        }

        let key_owned = key.clone();
        let value_owned = value.to_string();
        tab = tab.on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
            this.state
                .specimens
                .text
                .insert(key_owned.clone(), value_owned.clone());
            cx.notify();
        }));

        tab_bar = tab_bar.child(tab);
    }

    // ── Pane body ──
    let body: Div = match active_tab {
        SIZES_TAB => {
            let mut col = variant_pane(border_subtle, panel_bg);
            if let Some((values, row)) = axes.named_sizes.as_ref() {
                for value in values {
                    col = col.child(variant_row(value, row(value, theme), text_secondary));
                }
            } else {
                let row = axes.sizes.as_ref().expect("sizes tab is admitted");
                for (size, label) in ALL_SIZES {
                    if let Some(body) = row(*size, theme) {
                        col = col.child(variant_row(label, body, text_secondary));
                    }
                }
            }
            col
        }
        DENSITIES_TAB => {
            let mut col = variant_pane(border_subtle, panel_bg);
            if let Some((values, row)) = axes.named_densities.as_ref() {
                for value in values {
                    col = col.child(variant_row(value, row(value, theme), text_secondary));
                }
            } else {
                let row = axes.densities.as_ref().expect("densities tab is admitted");
                for (density, label) in ALL_DENSITIES {
                    if let Some(body) = row(*density, theme) {
                        col = col.child(variant_row(label, body, text_secondary));
                    }
                }
            }
            col
        }
        _ => div().flex().flex_col().gap(px(16.0)).child(examples),
    };

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .child(tab_bar)
        .child(body)
}

/// The bordered panel both axis panes stack their variant rows inside.
fn variant_pane(
    border_subtle: poodle_tokens::typed::ColorValue,
    panel_bg: poodle_tokens::typed::ColorValue,
) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .p(px(16.0))
        .border_1()
        .border_color(color_to_hsla(border_subtle))
        .rounded(px(6.0))
        .bg(color_to_hsla(panel_bg).alpha(0.4))
}

/// Single variant row: small label column on the left, the specimen body
/// on the right. Used inside the Sizes and Densities panes.
fn variant_row(
    label: &'static str,
    body: AnyElement,
    label_color: poodle_tokens::typed::ColorValue,
) -> Div {
    div()
        .flex()
        .items_start()
        .gap(px(16.0))
        .child(
            div()
                .w(px(80.0))
                .pt(px(4.0))
                .text_size(px(11.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(color_to_hsla(label_color))
                .child(label),
        )
        .child(div().flex_1().child(body))
}

/// Hsla extension: alpha helper for panel tinting.
trait HslaAlpha {
    fn alpha(self, a: f32) -> Self;
}

impl HslaAlpha for Hsla {
    fn alpha(mut self, a: f32) -> Self {
        self.a *= a;
        self
    }
}
