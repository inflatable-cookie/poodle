//! Toolbar — real GPUI component backed by ToolbarSpec.

use gpui::{prelude::FluentBuilder, *};
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    Alignment, ControlDensity, ControlSize, Orientation, SemanticControlSizeRole, ToolbarSpec,
};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_radius};

/// A real GPUI horizontal toolbar component backed by `ToolbarSpec`.
pub struct Toolbar {
    spec: ToolbarSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for Toolbar {
    type Target = ToolbarSpec;
    fn deref(&self) -> &ToolbarSpec {
        &self.spec
    }
}

impl Toolbar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ToolbarSpec::new(),
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn from_spec(spec: ToolbarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn alignment(mut self, v: Alignment) -> Self {
        self.spec.alignment = v;
        self
    }
    pub fn orientation(mut self, v: Orientation) -> Self {
        self.spec.orientation = v;
        self
    }
    pub fn has_separator(mut self, v: bool) -> Self {
        self.spec.has_separator = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
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

        let panel = resolve_color(theme, "color.background.panel");
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let radius = resolve_radius(theme, "radius.surface");

        // Svelte: background = color-mix(panel 94%, transparent)
        let bg = Hsla { a: panel.a * 0.94, ..panel };
        // Svelte: border = color-mix(border-subtle 78%, transparent)
        let border = Hsla { a: border_subtle.a * 0.78, ..border_subtle };

        // Svelte: padding and gap vary by size, density overrides inline padding + gap
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        use poodle_specs::ControlSize;
        let (pad_v, pad_h_base, gap_base) = match effective_size {
            ControlSize::Xs => (0.125_f32, 0.25_f32, 0.25_f32),
            ControlSize::Sm => (0.1875, 0.3125, 0.3125),
            ControlSize::Md => (0.25, 0.375, 0.375),
            ControlSize::Lg => (0.3125, 0.5, 0.5),
            ControlSize::Xl => (0.375, 0.625, 0.625),
        };
        let (pad_h, gap_val) = match spec.density {
            ControlDensity::Compact => (0.25_f32, 0.25_f32),
            ControlDensity::Default => (pad_h_base, gap_base),
            ControlDensity::Comfortable => (0.5, 0.5),
        };

        let is_vertical = spec.orientation == Orientation::Vertical;

        let mut el = div()
            .flex()
            .when(is_vertical, |el| el.flex_col())
            .when(!is_vertical, |el| el.items_center())
            .gap(px(rem_to_px(gap_val)))
            .py(px(rem_to_px(pad_v)))
            .px(px(rem_to_px(pad_h)))
            .bg(bg)
            .rounded(radius)
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
