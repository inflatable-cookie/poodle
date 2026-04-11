//! ActionDiscoveryPanel — real GPUI component backed by ActionDiscoveryPanelSpec.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ActionDiscoveryPanelSpec, ActionDiscoverySection, DiscoveryState};
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole, SkeletonSpec};

use crate::presentation::{resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem, control_space_x_rem, rem_to_px};
use crate::primitives::Skeleton;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI action discovery panel backed by `ActionDiscoveryPanelSpec`.
///
/// Renders categorized action sections, each with a heading and a list of
/// action items showing title, shortcut, and optional badge.
pub struct ActionDiscoveryPanel {
    spec: ActionDiscoveryPanelSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ActionDiscoveryPanel {
    type Target = ActionDiscoveryPanelSpec;
    fn deref(&self) -> &ActionDiscoveryPanelSpec { &self.spec }
}

impl ActionDiscoveryPanel {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ActionDiscoveryPanelSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_select: None }
    }

    pub fn from_spec(spec: ActionDiscoveryPanelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-action-discovery".to_string(),
            on_select: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn sections(mut self, v: Vec<ActionDiscoverySection>) -> Self { self.spec.sections = v; self }
    pub fn state(mut self, v: DiscoveryState) -> Self { self.spec.state = v; self }
    pub fn empty_message(mut self, v: impl Into<String>) -> Self { self.spec.empty_message = Some(v.into()); self }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }


    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl IntoElement for ActionDiscoveryPanel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let panel_px = rem_to_px(panel_space_x_rem(spec.density));
        let panel_py = rem_to_px(panel_space_y_rem(spec.density));
        let item_gap = rem_to_px(control_space_x_rem(spec.density));

        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let text_muted = resolve_color(theme, "color.text.muted");
        let border = resolve_color(theme, "color.border.default");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let hover_bg = resolve_color(theme, "color.background.elevated");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let body_size = px(font_size);
        let gap = theme.resolve_space(spec.gap_token());
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");
        let label_size = resolve_px(theme, "typography.label.size");
        let radius_control = resolve_radius(theme, "radius.control");

        let mut panel = div()
            .flex()
            .flex_col()
            .gap(px(gap))
            .size_full();

        match spec.state {
            DiscoveryState::Loading => {
                let mut skeletons = div().flex().flex_col().gap(gap_sm);
                for _ in 0..5 {
                    skeletons = skeletons.child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(gap_md)
                            .p(px(14.0))
                            .rounded(radius_control)
                            .bg(resolve_color(theme, "color.background.surface").opacity(0.72))
                            .child(
                                div()
                                    .flex_grow()
                                    .child(
                                        Skeleton::from_spec(
                                            SkeletonSpec::new().with_width("160").with_animated(true),
                                            theme,
                                        ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_none()
                                    .child(
                                        Skeleton::from_spec(
                                            SkeletonSpec::new().with_width("56").with_animated(true),
                                            theme,
                                        ),
                                    ),
                            ),
                    );
                }
                panel = panel.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(gap_sm)
                        .child(skeletons),
                );
                return panel.into_any_element();
            }
            DiscoveryState::Error => {
                panel = panel.child(
                    div()
                        .px(px(panel_px)).py(px(panel_py))
                        .text_size(body_size)
                        .text_color(resolve_color(theme, "color.status.danger"))
                        .child("Failed to load actions"),
                );
                return panel.into_any_element();
            }
            DiscoveryState::Empty | DiscoveryState::NoResults => {
                let msg = spec
                    .empty_message
                    .as_deref()
                    .unwrap_or("No actions found");
                panel = panel.child(
                    div()
                        .px(px(panel_px)).py(px(panel_py))
                        .text_size(body_size)
                        .text_color(text_secondary)
                        .child(msg.to_string()),
                );
                return panel.into_any_element();
            }
            DiscoveryState::Ready => {}
        }

        // Render each section
        for section in &spec.sections {
            let mut section_el = div().flex().flex_col().gap(px(item_gap));

            // Section heading
            section_el = section_el.child(
                div()
                    .px(px(panel_px * 0.5))
                    .py(px(panel_py * 0.5))
                    .text_size(px(font_size * 0.85))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_muted)
                    .child(section.title.clone()),
            );

            // Description
            if let Some(ref desc) = section.description {
                section_el = section_el.child(
                    div()
                        .px(px(panel_px * 0.5))
                        .text_size(px(font_size * 0.85))
                        .text_color(text_secondary)
                        .child(desc.clone()),
                );
            }

            // Action items
            for action in &section.actions {
                let action_id =
                    SharedString::from(format!("{}-{}", self.id_prefix, action.id));

                let mut row = div()
                    .id(action_id)
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(panel_px * 0.5))
                    .py(px(panel_py * 0.5))
                    .rounded(radius_control)
                    .text_size(body_size)
                    .text_color(text_primary);

                row = row.focus(move |s| s.border_color(focus_ring).shadow(crate::theme_ext::focus_ring_shadow(focus_ring)));

                if action.is_disabled {
                    row = row
                        .opacity(disabled_opacity)
                        .cursor(CursorStyle::OperationNotAllowed);
                } else {
                    row = row
                        .cursor_pointer()
                        .hover(|s| s.bg(hover_bg));
                }

                // Left: title + badge
                let mut left = div().flex().items_center().gap(gap_sm);
                left = left.child(action.title.clone());

                if let Some(ref badge) = action.badge {
                    left = left.child(
                        div()
                            .text_size(label_size)
                            .px(gap_sm)
                            .py(px(1.0))
                            .rounded(radius_control)
                            .bg(resolve_color(theme, "color.accent.base").opacity(0.12))
                            .text_color(resolve_color(theme, "color.accent.base"))
                            .child(badge.clone()),
                    );
                }

                row = row.child(left);

                // Right: shortcut
                if let Some(ref shortcut) = action.shortcut {
                    row = row.child(
                        div()
                            .text_size(label_size)
                            .text_color(text_muted)
                            .child(shortcut.clone()),
                    );
                }

                section_el = section_el.child(row);
            }

            // Section separator
            section_el = section_el.child(
                div()
                    .h(px(1.0))
                    .w_full()
                    .bg(border),
            );

            panel = panel.child(section_el);
        }

        panel.into_any_element()
    }
}
