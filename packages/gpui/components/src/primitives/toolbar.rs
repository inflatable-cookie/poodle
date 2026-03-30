//! Toolbar — real GPUI component backed by ToolbarSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{Alignment, ControlDensity, ControlSize, SemanticControlSizeRole, ToolbarSpec};

use crate::presentation::{rem_to_px, control_space_x_rem, panel_space_y_rem};
use crate::theme_ext::{color_mix, resolve_color};

/// A real GPUI horizontal toolbar component backed by `ToolbarSpec`.
pub struct Toolbar {
    spec: ToolbarSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for Toolbar {
    type Target = ToolbarSpec;
    fn deref(&self) -> &ToolbarSpec { &self.spec }
}

impl Toolbar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ToolbarSpec::new(), theme: theme.clone(), children: Vec::new() }
    }

    pub fn from_spec(spec: ToolbarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn alignment(mut self, v: Alignment) -> Self { self.spec.alignment = v; self }
    pub fn has_separator(mut self, v: bool) -> Self { self.spec.has_separator = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for Toolbar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let bg = resolve_color(theme, "semantic.color.background.surface");
        let border_raw = resolve_color(theme, "semantic.color.border.default");
        let panel = resolve_color(theme, "semantic.color.background.panel");
        let gap = px(rem_to_px(control_space_x_rem(spec.density)));
        let padding = px(rem_to_px(panel_space_y_rem(spec.density) * 0.5));

        // Contract: border color-mix 78% border-default over panel
        let border = color_mix(border_raw, panel, 0.78);

        let mut el = div()
            .flex()
            .items_center()
            .gap(gap)
            // Contract: padding 0.25rem (4px)
            .p(padding)
            // Contract: background surface
            .bg(bg)
            // Contract: border color-mix 78% of border-default applied always
            .border_1()
            .border_color(border);

        match spec.alignment {
            Alignment::Start => {}
            Alignment::Center => {
                el = el.justify_center();
            }
            Alignment::End => {
                el = el.justify_end();
            }
            Alignment::Stretch => {}
        }

        if spec.has_separator {
            // Additional bottom separator emphasis (border color already set above)
            el = el.border_b_2();
        }

        for child in self.children {
            el = el.child(child);
        }

        el.into_any_element()
    }
}
