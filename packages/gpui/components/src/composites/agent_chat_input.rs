//! AgentChatInput — GPUI agent composer backed by AgentChatInputSpec.
//!
//! Contract: `docs/contracts/components/agent-chat-input.md`
//! Reference: `packages/svelte/components/src/AgentChatInput.svelte`
//!
//! Anatomy: root → field (attachment chips → editor → toolbar with host-supplied
//! leading children, context ring and the submit/stop action) → optional footer
//! bar. Text editing and key handling live in the host event loop; the editor
//! part renders the current value (or the placeholder) as text, and the
//! auto-grow height comes from `visible_rows()` (contract §12).
//! Build-verified only.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    AgentChatInputSpec, ControlDensity, ControlSize, IconSize, IconSpec, MeterShape, MeterSpec,
    SemanticControlSizeRole,
};

use super::super::primitives::{Icon, Meter};
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct AgentChatInput {
    spec: AgentChatInputSpec,
    theme: GpuiThemeProvider,
    toolbar_children: Vec<AnyElement>,
    footer_children: Vec<AnyElement>,
}

impl std::ops::Deref for AgentChatInput {
    type Target = AgentChatInputSpec;
    fn deref(&self) -> &AgentChatInputSpec {
        &self.spec
    }
}

impl AgentChatInput {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(AgentChatInputSpec::new(), theme)
    }

    pub fn from_spec(spec: AgentChatInputSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            toolbar_children: Vec::new(),
            footer_children: Vec::new(),
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

    /// Append one host-composed toolbar control (canonically a `ModelPicker`).
    pub fn toolbar_child(mut self, child: impl IntoElement) -> Self {
        self.toolbar_children.push(child.into_any_element());
        self
    }

    pub fn with_toolbar_children(
        mut self,
        children: impl IntoIterator<Item = AnyElement>,
    ) -> Self {
        self.toolbar_children.extend(children);
        self
    }

    /// Append one child to the secondary footer bar.
    pub fn footer_child(mut self, child: impl IntoElement) -> Self {
        self.footer_children.push(child.into_any_element());
        self
    }

    pub fn with_footer_children(mut self, children: impl IntoIterator<Item = AnyElement>) -> Self {
        self.footer_children.extend(children);
        self
    }
}

impl IntoElement for AgentChatInput {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        // ── Size table (contract §8) ──────────────────────────────────────────
        let (pad_y_rem, pad_x_rem) = spec.field_padding_rem(effective_size);
        let pad_y = px(rem_to_px(pad_y_rem));
        let pad_x = px(rem_to_px(pad_x_rem));
        let editor_font = px(rem_to_px(spec.editor_font_rem(effective_size)));
        let action_box = px(rem_to_px(spec.action_size_rem(effective_size)));
        let gap = px(rem_to_px(spec.toolbar_gap_rem(effective_size)) * spec.density_gap_scale());
        let line_height = editor_font * 1.5;
        let divider_height = px(rem_to_px(spec.toolbar_divider_height_rem(effective_size)));

        // ── Colors ────────────────────────────────────────────────────────────
        let text_primary = resolve_color(theme, spec.text_token());
        let text_secondary = resolve_color(theme, spec.secondary_token());
        // Secondary text held below the muted opacity — a standing hint, not a
        // value (contract §8).
        let placeholder = {
            let base = resolve_color(theme, spec.placeholder_token());
            let alpha = resolve_opacity(theme, spec.placeholder_opacity_token())
                * spec.placeholder_opacity_ratio();
            Hsla {
                a: base.a * alpha,
                ..base
            }
        };
        let border = resolve_color(theme, spec.field_border_token());
        let divider = resolve_color(theme, spec.divider_token());
        let surface = resolve_color(theme, spec.field_fill_token());
        let elevated = resolve_color(theme, spec.attachment_fill_token());
        let action_fill = resolve_color(theme, spec.action_fill_token());
        let action_text = resolve_color(theme, spec.action_text_token());
        let field_radius = resolve_radius(theme, spec.field_radius_token()) * 1.5;

        let chip_radius = resolve_radius(theme, spec.attachment_radius_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        // ── Field ─────────────────────────────────────────────────────────────
        let mut field = div()
            .flex()
            .flex_col()
            .gap(gap)
            .w_full()
            .px(pad_x)
            .py(pad_y)
            .rounded(field_radius)
            .border_1()
            .border_color(border)
            .bg(surface);

        if !spec.attachments.is_empty() {
            let mut chips = div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(rem_to_px(0.375)));
            let thumb = px(rem_to_px(spec.attachment_thumb_rem(effective_size)));
            for attachment in spec.attachments.iter() {
                // Image attachments render as tiles: the picture says more than
                // the filename does (contract §2).
                if let Some(url) = &attachment.thumbnail_url {
                    let mut tile = img(url.clone())
                        .w(thumb)
                        .h(thumb)
                        .flex_none()
                        .rounded(chip_radius);
                    if attachment.is_disabled {
                        tile = tile.opacity(disabled_opacity);
                    }
                    chips = chips.child(tile);
                    continue;
                }

                let mut chip = div()
                    .flex()
                    .items_center()
                    .gap(px(rem_to_px(0.25)))
                    .pl(px(rem_to_px(0.5)))
                    .pr(px(rem_to_px(0.375)))
                    .py(px(rem_to_px(0.125)))
                    .rounded(chip_radius)
                    .border_1()
                    .border_color(divider)
                    .bg(elevated);

                if let Some(icon) = &attachment.icon {
                    chip = chip.child(
                        Icon::from_spec(
                            IconSpec::new(icon.clone()).with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(text_secondary),
                    );
                }

                chip = chip
                    .child(
                        div()
                            .text_size(px(rem_to_px(0.75)))
                            .text_color(text_primary)
                            .child(attachment.label.clone()),
                    )
                    // Compact remove glyph (not a full IconButton), matching the
                    // FilterBuilder pill treatment.
                    .child(
                        Icon::from_spec(IconSpec::new("x").with_size(IconSize::Sm), theme)
                            .with_color(text_secondary),
                    );

                // A disabled attachment cannot be removed, so it reads dimmed —
                // matching the web, where the remove IconButton carries the state.
                if attachment.is_disabled {
                    chip = chip.opacity(disabled_opacity);
                }
                chips = chips.child(chip);
            }
            field = field.child(chips);
        }

        // Editor: the value, or the placeholder when empty.
        let is_empty = spec.value.is_empty();
        field = field.child(
            div()
                .w_full()
                .min_h(line_height * spec.visible_rows() as f32)
                .text_size(editor_font)
                .text_color(if is_empty { placeholder } else { text_primary })
                .child(if is_empty {
                    spec.placeholder.clone()
                } else {
                    spec.value.clone()
                }),
        );

        // ── Toolbar ───────────────────────────────────────────────────────────
        let mut leading = div().flex().items_center().gap(gap).flex_grow();
        for (index, child) in self.toolbar_children.into_iter().enumerate() {
            // Hairline dividers between leading children (contract §8).
            if index > 0 && spec.toolbar_dividers {
                leading = leading.child(
                    div()
                        .w(px(rem_to_px(0.0625)))
                        .h(divider_height)
                        .flex_none()
                        .bg(divider),
                );
            }
            leading = leading.child(child);
        }

        let mut trailing = div()
            .flex()
            .items_center()
            .justify_end()
            .gap(gap)
            .flex_none();

        if spec.show_context() {
            let mut meter = MeterSpec::new()
                .with_shape(MeterShape::Ring)
                .with_value(spec.context_used.unwrap_or(0.0))
                .with_max(spec.context_limit.unwrap_or(100.0))
                .with_size(effective_size)
                .with_aria_label(spec.context_aria_label());
            if let Some(high) = spec.context_high() {
                meter = meter.with_high(high);
            }
            trailing = trailing.child(Meter::from_spec(meter, theme));
        }

        let mut action = div()
            .w(action_box)
            .h(action_box)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(action_fill)
            .child(
                Icon::from_spec(
                    IconSpec::new(spec.action_icon()).with_size(IconSize::Sm),
                    theme,
                )
                .with_color(action_text),
            );
        if !spec.can_submit() {
            action = action.opacity(disabled_opacity);
        }
        trailing = trailing.child(action);

        field = field.child(
            div()
                .flex()
                .items_center()
                .gap(gap)
                .w_full()
                .child(leading)
                .child(trailing),
        );

        let mut root = div().flex().flex_col().w_full().child(field);

        // ── Footer bar ────────────────────────────────────────────────────────
        if !self.footer_children.is_empty() {
            let mut footer = div()
                .flex()
                .items_center()
                .gap(gap)
                .mx(px(rem_to_px(1.5)))
                .px(pad_x)
                .py(pad_y)
                .rounded(chip_radius)
                .border_1()
                .border_color(divider)
                .bg(elevated)
                .text_color(text_secondary);
            for child in self.footer_children {
                footer = footer.child(child);
            }
            root = root.child(footer);
        }

        if spec.is_disabled {
            root = root.opacity(disabled_opacity);
        }

        root.into_any_element()
    }
}
