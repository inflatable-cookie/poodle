//! AgentChatInput — the composer: field, attachments, toolbar, action.
//!
//! Contract: `docs/contracts/components/agent-chat-input.md`
//! Ported from: `packages/jetstream/components/src/agent_chat_input.rs`.
//!
//! The editor is render-only here (value or placeholder as text); editing,
//! caret and focus are host concerns on every native target.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeKind,
};
use poodle_specs::{AgentChatInputSpec, AgentChatStatus, MeterShape, MeterSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::meter::meter;
use crate::presentation::rem_to_px;

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct AgentChatInputHandlers {
    /// Fires when the action control is pressed — submit or stop, per the
    /// spec. Never fires when the control is inert (`can_submit` false and
    /// nothing streaming to stop).
    pub on_action: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Fires with the removed attachment's id.
    pub on_remove_attachment: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn agent_chat_input(
    spec: &AgentChatInputSpec,
    ctx: &RenderContext<'_>,
    question_children: Vec<Node>,
    plan_children: Vec<Node>,
    toolbar_children: Vec<Node>,
    footer_children: Vec<Node>,
    handlers: AgentChatInputHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    // ── Size table (contract §8) ──────────────────────────────────────────────
    let (pad_y_rem, pad_x_rem) = spec.field_padding_rem(effective_size);
    let pad_y = rem_to_px(pad_y_rem);
    let pad_x = rem_to_px(pad_x_rem);
    let editor_font = rem_to_px(spec.editor_font_rem(effective_size));
    let action_box = rem_to_px(spec.action_size_rem(effective_size));
    let gap = rem_to_px(spec.toolbar_gap_rem(effective_size)) * spec.density_gap_scale(density);
    let line_height = editor_font * 1.5;
    let divider_height = rem_to_px(spec.toolbar_divider_height_rem(effective_size));

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = ctx.theme().resolve_color(spec.text_token());
    let text_secondary = ctx.theme().resolve_color(spec.secondary_token());
    // Secondary text held below the muted opacity — a standing hint, not a
    // value (contract §8).
    let placeholder_base = ctx.theme().resolve_color(spec.placeholder_token());
    let placeholder = with_alpha(
        placeholder_base,
        placeholder_base.3
            * ctx.theme().resolve_opacity(spec.placeholder_opacity_token())
            * spec.placeholder_opacity_ratio(),
    );
    let border = ctx.theme().resolve_color(spec.field_border_token());
    let divider = ctx.theme().resolve_color(spec.divider_token());
    let surface = ctx.theme().resolve_color(spec.field_fill_token());
    let elevated = ctx.theme().resolve_color(spec.attachment_fill_token());
    let action_fill = ctx.theme().resolve_color(spec.action_fill_token());
    let action_text = ctx.theme().resolve_color(spec.action_text_token());
    let field_radius = ctx.theme().resolve_radius(spec.field_radius_token()) * 1.5;
    let chip_radius = ctx.theme().resolve_radius(spec.attachment_radius_token());
    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };

    // ── Field ─────────────────────────────────────────────────────────────────
    let mut field = Node::container();
    {
        let s = &mut field.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = gap;
        s.fill_width = true;
        s.min_width = Some(0.0);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.background = Some(surface);
    }
    all_radius(&mut field, field_radius);

    let attention_region = |children: Vec<Node>| {
        let mut region = Node::container();
        region.style.descriptor.layout.direction = LayoutDirection::Column;
        region.style.fill_width = true;
        region.children(children)
    };

    if spec.status == AgentChatStatus::Questioning && !question_children.is_empty() {
        field = field.child(attention_region(question_children));
    }
    if spec.status == AgentChatStatus::ReviewingPlan && !plan_children.is_empty() {
        field = field.child(attention_region(plan_children));
    }

    // Attachment chips.
    if !spec.attachments.is_empty() {
        let mut chips = Node::container();
        {
            let s = &mut chips.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.flex_wrap = true;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = rem_to_px(0.375);
        }
        let thumb = rem_to_px(spec.attachment_thumb_rem(effective_size));
        for attachment in spec.attachments.iter() {
            // Image attachments render as tiles: the picture says more than the
            // filename does (contract §2).
            if let Some(url) = &attachment.thumbnail_url {
                let mut tile = Node::container();
                tile.kind = NodeKind::Image {
                    source: url.clone(),
                };
                {
                    let s = &mut tile.style;
                    // Explicit Row (see switch.rs).
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.width = LayoutSizing::Fixed(thumb);
                    s.descriptor.layout.height = LayoutSizing::Fixed(thumb);
                    s.flex_none = true;
                    if attachment.is_disabled {
                        s.descriptor.opacity = disabled_opacity;
                    }
                }
                all_radius(&mut tile, chip_radius);
                chips = chips.child(tile);
                continue;
            }

            let mut chip = Node::container();
            {
                let s = &mut chip.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = rem_to_px(0.25);
                let pad = &mut s.descriptor.layout.spacing.padding;
                pad.left = rem_to_px(0.5);
                pad.right = rem_to_px(0.375);
                pad.top = rem_to_px(0.125);
                pad.bottom = rem_to_px(0.125);
                s.descriptor.border.width = 1.0;
                s.descriptor.border.color = divider;
                s.descriptor.background = Some(elevated);
            }
            all_radius(&mut chip, chip_radius);

            if let Some(icon) = &attachment.icon {
                let mut glyph = Node::icon(icon, rem_to_px(0.75));
                glyph.style.descriptor.text_color = Some(text_secondary);
                chip = chip.child(glyph);
            }

            let mut label = Node::text(&attachment.label);
            {
                let s = &mut label.style;
                s.descriptor.text_color = Some(text_primary);
                s.text_size = Some(rem_to_px(0.75));
                s.text_ellipsis = true;
                s.no_wrap = true;
            }
            // Compact remove glyph (not a full IconButton), matching the
            // FilterBuilder pill treatment.
            let mut remove = Node::icon("x", rem_to_px(0.75));
            remove.style.descriptor.text_color = Some(text_secondary);
            let mut chip = chip.child(label).child(remove);

            // A disabled attachment cannot be removed, so it reads dimmed —
            // matching the web, where the remove IconButton carries the state.
            if attachment.is_disabled {
                chip.style.descriptor.opacity = disabled_opacity;
            } else if let Some(handler) = &handlers.on_remove_attachment {
                let handler = Arc::clone(handler);
                let id = attachment.id.clone();
                chip.style.descriptor.cursor = CursorHint::Pointer;
                chip.interaction.on_activate = Some(Arc::new(move || handler(&id)));
            }
            chips = chips.child(chip);
        }
        field = field.child(chips);
    }

    // Editor: the value, or the placeholder when empty. Height comes from the
    // clamped row count.
    let is_empty = spec.value.is_empty();
    let mut editor = Node::text(if is_empty {
        spec.effective_placeholder().to_string()
    } else {
        spec.value.clone()
    });
    {
        let s = &mut editor.style;
        s.fill_width = true;
        s.min_height = Some(line_height * spec.visible_rows() as f32);
        s.descriptor.text_color = Some(if is_empty { placeholder } else { text_primary });
        s.text_size = Some(editor_font);
    }
    field = field.child(editor);

    // ── Toolbar ───────────────────────────────────────────────────────────────
    let mut leading = Node::container();
    {
        let s = &mut leading.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }
    for (index, child) in toolbar_children.into_iter().enumerate() {
        // Hairline dividers between leading children (contract §8).
        if index > 0 && spec.toolbar_dividers {
            let mut rule = Node::container();
            {
                let s = &mut rule.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(0.0625));
                s.descriptor.layout.height = LayoutSizing::Fixed(divider_height);
                s.flex_none = true;
                s.descriptor.background = Some(divider);
            }
            leading = leading.child(rule);
        }
        leading = leading.child(child);
    }

    let mut trailing = Node::container();
    {
        let s = &mut trailing.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::End;
        s.descriptor.layout.spacing.gap = gap;
        s.flex_none = true;
    }

    if spec.show_context() {
        let mut context = MeterSpec::new()
            .with_shape(MeterShape::Ring)
            .with_value(spec.context_used.unwrap_or(0.0))
            .with_max(spec.context_limit.unwrap_or(100.0))
            .with_size(effective_size)
            .with_aria_label(spec.context_aria_label());
        if let Some(high) = spec.context_high() {
            context = context.with_high(high);
        }
        trailing = trailing.child(meter(&context, ctx));
    }

    let mut action = Node::container();
    {
        let s = &mut action.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.width = LayoutSizing::Fixed(action_box);
        s.descriptor.layout.height = LayoutSizing::Fixed(action_box);
        s.flex_none = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.background = Some(action_fill);
    }
    all_radius(&mut action, action_box / 2.0);
    let mut glyph = Node::icon(spec.action_icon(), action_box * 0.5);
    glyph.style.descriptor.text_color = Some(action_text);
    let mut action = action.child(glyph);
    if !spec.can_submit() {
        action.style.descriptor.opacity = disabled_opacity;
    } else if let Some(handler) = &handlers.on_action {
        let handler = Arc::clone(handler);
        action.style.descriptor.cursor = CursorHint::Pointer;
        action.interaction.on_activate = Some(Arc::new(move || handler()));
    }
    trailing = trailing.child(action);

    let mut toolbar = Node::container();
    {
        let s = &mut toolbar.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.fill_width = true;
        s.min_width = Some(0.0);
    }
    field = field.child(toolbar.child(leading).child(trailing));

    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.fill_width = true;
    root.style.min_width = Some(0.0);
    let mut root = root.child(field);

    // ── Footer bar ────────────────────────────────────────────────────────────
    if !footer_children.is_empty() {
        let mut footer = Node::container();
        {
            let s = &mut footer.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = gap;
            let margin = &mut s.descriptor.layout.spacing.margin;
            margin.left = rem_to_px(1.5);
            margin.right = rem_to_px(1.5);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_x;
            pad.right = pad_x;
            pad.top = pad_y;
            pad.bottom = pad_y;
            s.descriptor.border.width = 1.0;
            s.descriptor.border.color = divider;
            s.descriptor.background = Some(elevated);
        }
        all_radius(&mut footer, chip_radius);
        for child in footer_children {
            footer = footer.child(child);
        }
        root = root.child(footer);
    }

    if spec.is_disabled {
        root.style.descriptor.opacity = disabled_opacity;
    }

    if !spec.aria_label.is_empty() {
        root.a11y.label = Some(spec.aria_label.clone());
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_node::NodeKind;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn has_text(node: &Node, expected: &str) -> bool {
        node.find(&|child| matches!(&child.kind, NodeKind::Text { content } if content == expected))
            .is_some()
    }

    fn render_with_status(status: AgentChatStatus) -> Node {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        agent_chat_input(
            &AgentChatInputSpec::new().with_status(status),
            &ctx,
            vec![Node::text("question region")],
            vec![Node::text("plan region")],
            Vec::new(),
            Vec::new(),
            AgentChatInputHandlers::default(),
        )
    }

    #[test]
    fn attention_regions_follow_status_and_stay_inside_the_field() {
        let question = render_with_status(AgentChatStatus::Questioning);
        assert!(has_text(&question.children[0], "question region"));
        assert!(!has_text(&question, "plan region"));

        let plan = render_with_status(AgentChatStatus::ReviewingPlan);
        assert!(has_text(&plan.children[0], "plan region"));
        assert!(!has_text(&plan, "question region"));

        let idle = render_with_status(AgentChatStatus::Idle);
        assert!(!has_text(&idle, "question region"));
        assert!(!has_text(&idle, "plan region"));
    }
}
