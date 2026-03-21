//! LogList — timestamped log entry list backed by LogListSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_composites::LogListSpec;
use crate::theme_ext::{resolve_color, resolve_px};

pub struct LogList {
    spec: LogListSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for LogList {
    type Target = LogListSpec;
    fn deref(&self) -> &LogListSpec { &self.spec }
}

impl LogList {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: LogListSpec::new(), theme: theme.clone(), children: Vec::new() }
    }
    pub fn from_spec(spec: LogListSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), children: Vec::new() }
    }
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}

impl IntoElement for LogList {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let gap = resolve_px(&self.theme, self.spec.entry_gap_token());
        let mut el = div().bg(fill).flex().flex_col().gap(gap).overflow_y_hidden();
        for child in self.children { el = el.child(child); }
        el.into_any_element()
    }
}
