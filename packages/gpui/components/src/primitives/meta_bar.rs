use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::MetaBarSpec;

use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// Separator-dot color mix: `color-mix(in srgb, text-secondary 72%, transparent)`
/// (contract §7 separator-dot `background`). No token carries the 72% factor —
/// it is a contract literal, sourced here as a named constant rather than inlined.
const SEPARATOR_DOT_MIX: f32 = 0.72;

pub struct MetaBar {
    spec: MetaBarSpec,
    theme: GpuiThemeProvider,
    /// Each child paired with its `data-separator` intent (default `true`).
    /// A child draws a leading dot only when it is not first, separators are on,
    /// and its flag is `true` (contract §4 per-child opt-in).
    children: Vec<(AnyElement, bool)>,
}

impl std::ops::Deref for MetaBar {
    type Target = MetaBarSpec;
    fn deref(&self) -> &MetaBarSpec {
        &self.spec
    }
}

impl MetaBar {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(MetaBarSpec::new(), theme)
    }

    pub fn from_spec(spec: MetaBarSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }

    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }

    pub fn show_separators(mut self, v: bool) -> Self {
        self.spec.show_separators = v;
        self
    }

    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push((child.into_any_element(), true));
        self
    }

    /// Add a child carrying explicit `data-separator` intent. Pass `false` to
    /// suppress this child's leading separator dot (contract §4 per-child opt-out).
    pub fn with_child_sep(mut self, child: impl IntoElement, separator: bool) -> Self {
        self.children.push((child.into_any_element(), separator));
        self
    }

    pub fn with_children(mut self, children: impl IntoIterator<Item = AnyElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| (c, true)));
        self
    }

    pub fn with_children_sep(
        mut self,
        children: impl IntoIterator<Item = (AnyElement, bool)>,
    ) -> Self {
        self.children.extend(children);
        self
    }
}

impl IntoElement for MetaBar {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let separator_color = resolve_color(theme, "color.text.secondary");
        // Dot geometry: 0.25rem square, pill radius — both token/rem-derived.
        let dot_size = px(rem_to_px(0.25));
        let dot_radius = resolve_radius(theme, "radius.pill");
        let dot_color = Hsla {
            a: separator_color.a * SEPARATOR_DOT_MIX,
            ..separator_color
        };

        let mut row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(resolve_px(theme, "space.inline.sm"))
            .min_w(px(0.0));

        for (idx, (child, separator)) in self.children.into_iter().enumerate() {
            // Per-child opt-in: dot only when not first, separators on, and the
            // child carries `data-separator="true"` (contract §4).
            if idx > 0 && self.spec.show_separators && separator {
                row = row.child(
                    div()
                        .w(dot_size)
                        .h(dot_size)
                        .rounded(dot_radius)
                        .bg(dot_color),
                );
            }

            row = row.child(div().min_w(px(0.0)).child(child));
        }

        row.into_any_element()
    }
}
