//! Select — real GPUI component backed by SelectSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ChoiceOption, ControlSize, SelectMode, SelectSpec,
};


/// A real GPUI select/dropdown component backed by `SelectSpec`.
pub struct Select {
    spec: SelectSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_search_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Select {
    type Target = SelectSpec;
    fn deref(&self) -> &SelectSpec {
        &self.spec
    }
}

impl Select {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SelectSpec::default(),
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_change: None,
            on_search_change: None,
        }
    }

    pub fn from_spec(spec: SelectSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_change: None,
            on_search_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = Some(v.into());
        self
    }
    pub fn placeholder(mut self, v: impl Into<String>) -> Self {
        self.spec.placeholder = Some(v.into());
        self
    }
    pub fn options(mut self, v: Vec<ChoiceOption>) -> Self {
        self.spec.options = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn description_id(mut self, v: impl Into<String>) -> Self {
        self.spec.description_id = Some(v.into());
        self
    }
    pub fn open(mut self, v: bool) -> Self {
        self.spec.open = Some(v);
        self
    }
    pub fn default_open(mut self, v: bool) -> Self {
        self.spec.default_open = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
    pub fn mode(mut self, v: SelectMode) -> Self {
        self.spec.mode = v;
        self
    }
    pub fn searchable(mut self, v: bool) -> Self {
        self.spec.searchable = v;
        self
    }
    pub fn freeform(mut self, v: bool) -> Self {
        self.spec.freeform = v;
        self
    }
    pub fn empty_message(mut self, v: impl Into<String>) -> Self {
        self.spec.empty_message = v.into();
        self
    }

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    pub fn search_query(mut self, q: impl Into<String>) -> Self {
        self.spec.search_query = Some(q.into());
        self
    }

    pub fn on_search_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_search_change = Some(Box::new(handler));
        self
    }
}


impl IntoElement for Select {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        self.render()
    }
}

mod render;

/// Parse a CSS length string (e.g. "12rem", "200px") to pixels.
/// Returns 0.0 on parse failure. Uses 16px as the rem base.
pub(super) fn parse_css_length_to_px(value: &str) -> f32 {
    let trimmed = value.trim();
    if let Some(num_str) = trimmed.strip_suffix("rem") {
        num_str.trim().parse::<f32>().unwrap_or(0.0) * 16.0
    } else if let Some(num_str) = trimmed.strip_suffix("px") {
        num_str.trim().parse::<f32>().unwrap_or(0.0)
    } else {
        trimmed.parse::<f32>().unwrap_or(0.0)
    }
}

