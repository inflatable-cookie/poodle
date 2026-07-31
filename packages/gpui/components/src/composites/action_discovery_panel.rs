//! ActionDiscoveryPanel — real GPUI component backed by ActionDiscoveryPanelSpec.

use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ActionDiscoveryPanelSpec, ActionDiscoverySection, DiscoveryState};
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole, SkeletonSpec};

use crate::composites::EmptyState;
use crate::presentation::{
    control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::primitives::{Eyebrow, Skeleton};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};
use poodle_specs::{EmptyStateSpec, EmptyStateVariant, EyebrowSpec};

/// Per-size chip dimensions (`chip-height`, `chip-x`, `chip-font-size`) for the
/// badge/kbd pills — contract §9 "Chip size table".
fn chip_dims(size: ControlSize) -> (f32, f32, f32) {
    match size {
        ControlSize::Xs => (1.125, 0.375, 0.5625),
        ControlSize::Sm => (1.25, 0.5, 0.625),
        ControlSize::Md => (1.375, 0.5, 0.6875),
        ControlSize::Lg => (1.5, 0.625, 0.75),
        ControlSize::Xl => (1.75, 0.75, 0.8125),
    }
}

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
    fn deref(&self) -> &ActionDiscoveryPanelSpec {
        &self.spec
    }
}

impl ActionDiscoveryPanel {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ActionDiscoveryPanelSpec::default(),
            theme: theme.clone(),
            id_prefix: String::new(),
            on_select: None,
        }
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
    pub fn sections(mut self, v: Vec<ActionDiscoverySection>) -> Self {
        self.spec.sections = v;
        self
    }
    pub fn state(mut self, v: DiscoveryState) -> Self {
        self.spec.state = v;
        self
    }
    pub fn empty_message(mut self, v: impl Into<String>) -> Self {
        self.spec.empty_message = Some(v.into());
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
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

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl IntoElement for ActionDiscoveryPanel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        // Rc so every row closure can hold a clone.
        let on_select_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_select.map(|h| std::rc::Rc::from(h));

        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let item_gap = rem_to_px(control_space_x_rem(spec.density));
        // Per-size row x/y padding matches Svelte --poodle-action-discovery-row-x/y
        let (row_x, row_y) = match effective_size {
            ControlSize::Xs => (rem_to_px(0.5), rem_to_px(0.25)),
            ControlSize::Sm => (rem_to_px(0.5), rem_to_px(0.3125)),
            ControlSize::Md => (rem_to_px(0.625), rem_to_px(0.375)),
            ControlSize::Lg => (rem_to_px(0.75), rem_to_px(0.5)),
            ControlSize::Xl => (rem_to_px(0.875), rem_to_px(0.625)),
        };
        let (chip_h, chip_x, chip_font) = chip_dims(effective_size);
        // Density-aware chip gap (--poodle-action-discovery-chip-gap)
        let chip_gap = rem_to_px(match spec.density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        });
        // Skeleton padding (--poodle-action-discovery-skeleton-pad)
        let skeleton_pad = rem_to_px(match spec.density {
            ControlDensity::Compact => 0.625,
            ControlDensity::Default => 0.875,
            ControlDensity::Comfortable => 1.0,
        });

        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");
        let elevated = resolve_color(theme, "color.background.elevated");
        let surface = resolve_color(theme, "color.background.surface");
        let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
        let hover_bg = elevated;
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let body_size = px(font_size);
        let gap = theme.resolve_space(spec.gap_token());
        let group_gap = rem_to_px(0.375);
        let list_gap = rem_to_px(0.25);
        let label_size = resolve_px(theme, "typography.label.size");
        let caption_size = resolve_px(theme, "typography.caption.size");
        let radius_control = resolve_radius(theme, "radius.control");

        // Active-item accent treatment (contract §9):
        //   bg   = accent 18% over background-elevated
        //   ring = inset 0.0625rem accent 22% (over transparent)
        let active_bg = color_mix(accent, elevated, 0.18);
        let active_ring = accent.opacity(0.22);

        let mut panel = div().flex().flex_col().gap(px(gap)).size_full();

        match spec.state {
            DiscoveryState::Loading => {
                // 5 skeleton rows; two skeletons per row (48% / 20% width).
                let mut skeletons = div()
                    .flex()
                    .flex_col()
                    .gap(px(list_gap))
                    .w_full()
                    .p(px(skeleton_pad));
                for _ in 0..5 {
                    skeletons = skeletons.child(
                        div()
                            .flex()
                            .justify_between()
                            .gap(px(chip_gap))
                            .child(div().w(relative(0.48)).child(Skeleton::from_spec(
                                SkeletonSpec::new().with_animated(true),
                                theme,
                            )))
                            .child(div().w(relative(0.20)).child(Skeleton::from_spec(
                                SkeletonSpec::new().with_animated(true),
                                theme,
                            ))),
                    );
                }
                // __state wrapper: min-height 10rem, centered.
                panel = panel.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .min_h(px(rem_to_px(10.0)))
                        .child(skeletons),
                );
                return panel.into_any_element();
            }
            DiscoveryState::Error => {
                panel = panel.child(EmptyState::from_spec(
                    EmptyStateSpec::new("Could not load actions")
                        .with_message("Actions could not be loaded. Try again.")
                        .with_compact(true),
                    theme,
                ));
                return panel.into_any_element();
            }
            DiscoveryState::Empty => {
                let title = spec.empty_message.as_deref().unwrap_or("No actions available");
                panel = panel.child(EmptyState::from_spec(
                    EmptyStateSpec::new(title)
                        .with_message("No actions are available in this context.")
                        .with_compact(true),
                    theme,
                ));
                return panel.into_any_element();
            }
            DiscoveryState::NoResults => {
                panel = panel.child(EmptyState::from_spec(
                    EmptyStateSpec::new("No matching actions")
                        .with_message("No actions match the current search.")
                        .with_variant(EmptyStateVariant::Search)
                        .with_compact(true),
                    theme,
                ));
                return panel.into_any_element();
            }
            DiscoveryState::Ready => {}
        }

        // Render each section (group: Eyebrow heading + list of items)
        for section in &spec.sections {
            let mut section_el = div().flex().flex_col().gap(px(group_gap));

            // Section heading via Eyebrow primitive
            section_el = section_el.child(Eyebrow::from_spec(
                EyebrowSpec::new().with_content(section.title.clone()),
                theme,
            ));

            // Description (not in Svelte anatomy, but spec carries it)
            if let Some(ref desc) = section.description {
                section_el = section_el.child(
                    div()
                        .px(px(row_x))
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(desc.clone()),
                );
            }

            // List of action items
            let mut list = div().flex().flex_col().gap(px(list_gap));

            for action in &section.actions {
                let action_id = SharedString::from(format!("{}-{}", self.id_prefix, action.id));
                let is_active = spec.active_id.as_deref() == Some(action.id.as_str());

                let mut row = div()
                    .id(action_id)
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(item_gap))
                    .px(px(row_x))
                    .py(px(row_y))
                    .rounded(radius_control)
                    .text_size(body_size)
                    .text_color(text_primary);

                row = row.focus(move |s| {
                    s.border_color(focus_ring)
                        .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
                });

                // Active item: accent-tinted bg + inset accent ring (contract §9).
                if is_active {
                    row = row.bg(active_bg).shadow(vec![gpui::BoxShadow {
                        color: active_ring,
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(rem_to_px(0.0625)),
                    }]);
                }

                if action.is_disabled {
                    row = row
                        .opacity(disabled_opacity)
                        .cursor(CursorStyle::OperationNotAllowed);
                } else {
                    if !is_active {
                        row = row.cursor_pointer().hover(|s| s.bg(hover_bg));
                    } else {
                        row = row.cursor_pointer();
                    }

                    if let Some(handler) = &on_select_rc {
                        let handler = handler.clone();
                        let id = action.id.clone();
                        row = row.on_click(move |_event, window, cx| {
                            handler(&id, window, cx);
                        });
                    }
                }

                // Left: title + subtitle (description)
                let mut text_block = div().flex().flex_col().gap(px(rem_to_px(0.125)));
                text_block = text_block.child(action.title.clone());
                if let Some(ref desc) = action.description {
                    text_block = text_block.child(
                        div()
                            .text_size(caption_size)
                            .text_color(text_secondary)
                            .child(desc.clone()),
                    );
                }
                row = row.child(text_block);

                // Right: trailing snippet — badge chip + kbd chip.
                let has_badge = action.badge.is_some();
                let has_shortcut = action.shortcut.is_some();
                if has_badge || has_shortcut {
                    let mut trailing = div().flex().items_center().gap(px(chip_gap));

                    // Shared chip base: uppercase label typography, control radius,
                    // per-size height/padding/font.
                    let chip_base = || {
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .min_h(px(rem_to_px(chip_h)))
                            .px(px(rem_to_px(chip_x)))
                            .rounded(radius_control)
                            .text_size(px(rem_to_px(chip_font)))
                            .font_weight(FontWeight::SEMIBOLD)
                    };

                    if let Some(ref badge) = action.badge {
                        // Accent override: accent 16% bg, accent text.
                        trailing = trailing.child(
                            chip_base()
                                .bg(accent.opacity(0.16))
                                .text_color(accent)
                                .child(badge.to_uppercase()),
                        );
                    }
                    if let Some(ref shortcut) = action.shortcut {
                        // Kbd: surface 76% bg, secondary text, monospace (no uppercase).
                        trailing = trailing.child(
                            chip_base()
                                .bg(surface.opacity(0.76))
                                .text_color(text_secondary)
                                .child(shortcut.clone()),
                        );
                    }

                    row = row.child(trailing);
                }

                list = list.child(row);
            }

            section_el = section_el.child(list);
            panel = panel.child(section_el);
        }

        panel.into_any_element()
    }
}
