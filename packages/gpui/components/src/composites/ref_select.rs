//! RefSelect — GPUI version-control ref chooser backed by RefSelectSpec.
//!
//! Contract: `docs/contracts/components/ref-select.md`
//! Reference: `packages/svelte/components/src/RefSelect.svelte`
//!
//! Trigger (kind glyph + label + chevron) over an anchored panel holding the
//! search field, the ref list with its current marker, and the empty/loading
//! footers. GPUI has no ARIA channel and no anchored positioning — the surface
//! renders inline below the trigger, and typing/clicking live in the host event
//! loop. Build-verified only.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, IconSize, IconSpec, RefSelectSpec, RefSelectVariant,
    SemanticControlSizeRole, TextInputSpec,
};

use super::super::primitives::{Icon, TextInput};
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct RefSelect {
    spec: RefSelectSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for RefSelect {
    type Target = RefSelectSpec;
    fn deref(&self) -> &RefSelectSpec {
        &self.spec
    }
}

impl RefSelect {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(RefSelectSpec::new(), theme)
    }

    pub fn from_spec(spec: RefSelectSpec, theme: &GpuiThemeProvider) -> Self {
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

    pub fn with_open(mut self, v: bool) -> Self {
        self.spec.is_open = v;
        self
    }
}

impl IntoElement for RefSelect {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size table (contract §8) ──────────────────────────────────────────
        let trigger_height = px(rem_to_px(match effective_size {
            ControlSize::Xs => 1.5,
            ControlSize::Sm => 1.75,
            ControlSize::Md => 2.25,
            ControlSize::Lg => 2.75,
            ControlSize::Xl => 3.25,
        }));
        let trigger_font = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.6875,
            ControlSize::Sm => 0.8125,
            ControlSize::Md => 0.875,
            ControlSize::Lg => 0.9375,
            ControlSize::Xl => 1.0,
        }));
        let trigger_gap = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        }));

        // ── Colors ────────────────────────────────────────────────────────────
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, spec.secondary_color_token());
        let muted = resolve_color(theme, spec.muted_color_token());
        let label_color = if spec.has_selection() {
            resolve_color(theme, spec.label_color_token())
        } else {
            muted
        };
        // Subdued dims the resting trigger; hover/focus restoration is web-only
        // (contract §12).
        let subdued_opacity = if spec.emphasis.is_subdued() {
            resolve_opacity(theme, spec.subdued_opacity_token())
        } else {
            1.0
        };
        let dim = |color: Hsla| Hsla {
            a: color.a * subdued_opacity,
            ..color
        };
        let border = resolve_color(theme, spec.trigger_border_token());
        let item_border = resolve_color(theme, spec.item_border_token());
        let surface = resolve_color(theme, spec.trigger_fill_token());
        let elevated = resolve_color(theme, spec.surface_fill_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let surface_radius = resolve_radius(theme, spec.surface_radius_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        // ── Trigger ───────────────────────────────────────────────────────────
        let mut trigger = div()
            .flex()
            .items_center()
            .gap(trigger_gap)
            .min_h(trigger_height)
            .px(px(rem_to_px(0.375)))
            .rounded(radius);

        if spec.variant == RefSelectVariant::Outlined {
            trigger = trigger.border_1().border_color(border).bg(surface);
        }

        trigger = trigger
            .child(
                Icon::from_spec(
                    IconSpec::new(spec.trigger_icon()).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(dim(text_secondary)),
            )
            .child(
                div()
                    .text_size(trigger_font)
                    .text_color(label_color)
                    .child(spec.trigger_label()),
            )
            .child(
                div()
                    .text_size(trigger_font)
                    .text_color(dim(text_secondary))
                    .child("▾"),
            );

        let mut root = div()
            .flex()
            .flex_col()
            .gap(px(rem_to_px(0.5)))
            .child(trigger);

        // ── Dialog surface (rendered inline when open) ────────────────────────
        if spec.is_open {
            let mut panel = div().flex().flex_col().gap(px(rem_to_px(0.5)));

            if spec.is_searchable {
                let mut search = TextInputSpec::new();
                if let Some(query) = &spec.search_value {
                    search = search.with_value(query.clone());
                }
                search.placeholder = Some(spec.search_placeholder.clone());
                panel = panel.child(
                    TextInput::from_spec(search, theme)
                        .size(effective_size)
                        .density(spec.density)
                        .disabled(spec.is_disabled),
                );
            }

            let rows = spec.rows();
            let mut list = div().flex().flex_col().gap(px(rem_to_px(0.125)));
            for (index, option) in rows.iter().enumerate() {
                if let Some(heading) = spec.group_heading_for(&rows, index) {
                    list = list.child(
                        div()
                            .flex_none()
                            .pl(px(rem_to_px(0.5)))
                            .pt(px(rem_to_px(if index == 0 { 0.5 } else { 0.875 })))
                            .pb(px(rem_to_px(0.25)))
                            .text_size(px(rem_to_px(0.6875)))
                            .text_color(text_secondary)
                            .child(heading.to_string()),
                    );
                }

                let is_selected = option.value == spec.value;
                let mut row = div()
                    .flex()
                    .flex_none()
                    .items_start()
                    .gap(px(rem_to_px(0.5)))
                    .px(px(rem_to_px(0.5)))
                    .py(px(rem_to_px(0.375)))
                    .rounded(radius)
                    .child(
                        Icon::from_spec(
                            IconSpec::new(option.resolved_icon().to_string())
                                .with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(text_secondary),
                    );

                let mut text = div().flex().flex_col().flex_grow().child(
                    div()
                        .text_size(px(rem_to_px(0.875)))
                        .text_color(text_primary)
                        .child(option.label.clone()),
                );
                if let Some(description) = &option.description {
                    text = text.child(
                        div()
                            .text_size(px(rem_to_px(0.75)))
                            .text_color(text_secondary)
                            .child(description.clone()),
                    );
                }
                row = row.child(text);

                if spec.is_current(option) {
                    row = row.child(
                        div()
                            .flex_none()
                            .text_size(px(rem_to_px(0.75)))
                            .text_color(text_secondary)
                            .child(spec.current_label.clone()),
                    );
                }

                if option.is_disabled {
                    row = row.opacity(disabled_opacity);
                }
                if is_selected {
                    row = row.border_1().border_color(item_border);
                }

                list = list.child(row);
            }
            panel = panel.child(list);

            if spec.show_empty() {
                panel = panel.child(
                    div()
                        .text_size(px(rem_to_px(0.75)))
                        .text_color(text_secondary)
                        .child(spec.empty_label.clone()),
                );
            }

            if spec.is_loading {
                panel = panel.child(
                    div()
                        .text_size(px(rem_to_px(0.75)))
                        .text_color(text_secondary)
                        .child(spec.loading_label.clone()),
                );
            }

            root = root.child(
                div()
                    .min_w(px(rem_to_px(16.0)))
                    .max_w(px(rem_to_px(24.0)))
                    .p(px(rem_to_px(0.5)))
                    .rounded(surface_radius)
                    .border_1()
                    .border_color(item_border)
                    .bg(elevated)
                    .child(panel),
            );
        }

        if spec.is_disabled {
            root = root.opacity(disabled_opacity);
        }

        root.into_any_element()
    }
}
