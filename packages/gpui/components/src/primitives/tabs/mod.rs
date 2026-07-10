//! Tabs — real GPUI component backed by TabsSpec.
//!
//! Supports four variants matching the Svelte Tabs component:
//! - Underline (default): bottom border with accent indicator
//! - Card: bordered tabs
//! - Pill: rounded pill container with tinted active state
//! - Block: full-width tabs with separators and accent-tinted selected fill

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlSize, IconSize, IconSpec, Orientation, TabActivationMode, TabDefinition, TabVariant,
    TabsSpec,
};

use super::icon::Icon;
use crate::presentation::rem_to_px;
use crate::theme_ext::{resolve_color, resolve_px};

/// Build the inner label composite for a tab: optional leading icon,
/// the label text, and an optional trailing count badge. Shared by all
/// four Tab variants (Underline / Card / Pill / Block) so they render
/// consistently.
pub(crate) fn build_tab_label(
    tab_def: &TabDefinition,
    theme: &GpuiThemeProvider,
    text_color: Hsla,
    icon_only: bool,
) -> AnyElement {
    let caption_size = resolve_px(theme, "typography.caption.size");

    // Vertical/icon-only mode: contract §8 hides `.poodle-tabs__label`. Show the
    // icon alone; fall back to the label when the tab has no icon.
    if icon_only {
        if let Some(ref icon_name) = tab_def.icon {
            return Icon::from_spec(
                IconSpec::new(icon_name.clone()).with_size(IconSize::Sm),
                theme,
            )
            .with_color(text_color)
            .into_any_element();
        }
        return div().child(tab_def.label.clone()).into_any_element();
    }

    // Fast path: no decoration — just the label string.
    if tab_def.icon.is_none() && tab_def.count.is_none() {
        return div().child(tab_def.label.clone()).into_any_element();
    }

    // Svelte Tabs: gap between icon and label = space.inline.sm
    let mut inner = div().flex().items_center().gap(resolve_px(theme, "space.inline.sm"));

    if let Some(ref icon_name) = tab_def.icon {
        inner = inner.child(
            Icon::from_spec(
                IconSpec::new(icon_name.clone()).with_size(IconSize::Sm),
                theme,
            )
            .with_color(text_color),
        );
    }

    inner = inner.child(div().child(tab_def.label.clone()));

    if let Some(count) = tab_def.count {
        // Mix text_color with the surface for the badge background so
        // the chip tone follows the tab's active/inactive state.
        let surface = resolve_color(theme, "color.background.surface");
        let badge_bg = crate::theme_ext::color_mix(text_color, surface, 0.14);
        inner = inner.child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .min_w(px(rem_to_px(1.125)))
                .px(px(rem_to_px(0.3125)))
                .rounded(px(rem_to_px(0.5625)))
                .bg(badge_bg)
                .text_size(caption_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_color)
                .child(format!("{count}")),
        );
    }

    inner.into_any_element()
}

/// A real GPUI tabs component backed by `TabsSpec`.
pub struct Tabs {
    spec: TabsSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    /// Content elements keyed by tab value.
    content: Vec<(String, AnyElement)>,
}

impl std::ops::Deref for Tabs {
    type Target = TabsSpec;
    fn deref(&self) -> &TabsSpec {
        &self.spec
    }
}

impl Tabs {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: TabsSpec::default(),
            theme: theme.clone(),
            id_prefix: String::new(),
            on_change: None,
            content: Vec::new(),
        }
    }

    pub fn from_spec(spec: TabsSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-tabs".to_string(),
            on_change: None,
            content: Vec::new(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn tabs(mut self, v: Vec<TabDefinition>) -> Self {
        self.spec.tabs = v;
        self
    }
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.spec.value = Some(v.into());
        self
    }
    pub fn default_value(mut self, v: impl Into<String>) -> Self {
        self.spec.default_value = Some(v.into());
        self
    }
    pub fn variant(mut self, v: TabVariant) -> Self {
        self.spec.variant = v;
        self
    }
    pub fn orientation(mut self, v: Orientation) -> Self {
        self.spec.orientation = v;
        self
    }
    pub fn activation_mode(mut self, v: TabActivationMode) -> Self {
        self.spec.activation_mode = v;
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
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
    /// Set the transient drag-source tab value (the tab being dragged).
    pub fn drag_value(mut self, v: Option<String>) -> Self {
        self.spec.drag_value = v;
        self
    }
    /// Set the transient drop-target tab value (the tab under the drag).
    pub fn drop_target_value(mut self, v: Option<String>) -> Self {
        self.spec.drop_target_value = v;
        self
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }

    /// Add content for a specific tab value.
    pub fn with_content(mut self, value: impl Into<String>, content: impl IntoElement) -> Self {
        self.content
            .push((value.into(), content.into_any_element()));
        self
    }

}

mod block_variant;
mod card_variant;
mod pill_variant;
mod text_variant;

impl IntoElement for Tabs {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let tab_row = match self.spec.variant {
            TabVariant::Pill => self.render_pill(),
            TabVariant::Card => self.render_card(),
            TabVariant::Block => self.render_block(),
            TabVariant::Underline => self.render_underline(),
        };

        let current_value = self.spec.current_value().map(|s| s.to_string());
        let panel_padding = resolve_px(&self.theme, "space.panel.y");

        // Content pane
        let mut wrapper = div().flex().flex_col().child(tab_row);

        // Show content for active tab
        for (value, content) in self.content {
            if current_value.as_deref() == Some(&value) {
                wrapper = wrapper.child(div().p(panel_padding).child(content));
                break;
            }
        }

        wrapper.into_any_element()
    }
}
