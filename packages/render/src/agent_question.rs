//! AgentQuestion — a question the agent needs answered before it continues.
//!
//! Contract: `docs/contracts/components/agent-question.md`
//! Ported from: `packages/jetstream/components/src/agent_question.rs`.

use std::sync::Arc;

use poodle_headless::agent_question::QuestionProgressState;
use poodle_node::{CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole};
use poodle_specs::AgentQuestionSpec;

use crate::color::{mix_srgb, TRANSPARENT};
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct AgentQuestionHandlers {
    /// Fires with the option's `value` when an option is clicked.
    ///
    /// The value, not the label: the label is what the reader sees and the
    /// value is what the agent asked about, and hosts localise the first.
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the question's id when the dismiss control is used.
    pub on_dismiss: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn agent_question(
    spec: &AgentQuestionSpec,
    ctx: &RenderContext<'_>,
    handlers: AgentQuestionHandlers,
) -> Node {
    let Some(question) = spec.active_question() else {
        let mut empty = Node::container();
        // Explicit Row (see switch.rs) — the old tier returns a bare div.
        empty.style.descriptor.layout.direction = LayoutDirection::Row;
        return empty;
    };

    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let prompt_color = ctx.theme().resolve_color(spec.prompt_token());
    let label_color = ctx.theme().resolve_color(spec.option_label_token());
    let description_color = ctx.theme().resolve_color(spec.option_description_token());
    let option_fill = ctx.theme().resolve_color(spec.option_fill_token());
    let accent = ctx.theme().resolve_color(spec.accent_token());
    // Contract §10: a selected option carries the accent at 10% over its own
    // fill, not the border alone.
    let selected_fill = mix_srgb(accent, option_fill, 0.1);
    let border = ctx.theme().resolve_color(spec.border_token());
    let shortcut_color = ctx.theme().resolve_color(spec.shortcut_token());
    let progress_color = ctx.theme().resolve_color(spec.progress_token());
    let dismiss_color = ctx.theme().resolve_color(spec.dismiss_token());
    let radius = ctx.theme().resolve_radius(spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem(base_size));
    let prompt_size = rem_to_px(spec.prompt_size_rem(base_size));
    let gap = rem_to_px(spec.gap_rem(density));
    let option_gap = rem_to_px(spec.option_gap_rem(density));
    let pad_block = rem_to_px(spec.option_padding_block_rem(base_size));
    let pad_inline = rem_to_px(spec.option_padding_inline_rem(density));
    let hairline = rem_to_px(0.0625);

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };

    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.fill_width = true;
    root.style.descriptor.layout.spacing.gap = gap;

    if spec.shows_progress() {
        let progress = spec.progress();
        let mut dots = Node::container();
        dots.style.descriptor.layout.direction = LayoutDirection::Row;
        dots.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        dots.style.descriptor.layout.spacing.gap = rem_to_px(0.25);

        for state in &progress.states {
            let answered = !matches!(state, QuestionProgressState::Pending);
            let current = matches!(state, QuestionProgressState::Current);
            let mut dot = Node::container();
            {
                let s = &mut dot.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                // The live one reads as the focus of the row without a second
                // colour.
                s.descriptor.layout.width = LayoutSizing::Fixed(if current {
                    rem_to_px(0.875)
                } else {
                    rem_to_px(0.375)
                });
                s.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(0.375));
                s.descriptor.background = Some(if answered { accent } else { border });
            }
            all_radius(&mut dot, 999.0);
            dots = dots.child(dot);
        }

        // The dots are a picture of the label; the label carries the fact for
        // anyone who cannot see them.
        let mut label = Node::text(spec.resolved_progress_label());
        label.style.text_size = Some(font_size);
        label.style.descriptor.text_color = Some(progress_color);
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        row.style.descriptor.layout.spacing.gap = rem_to_px(0.5);
        root = root.child(row.child(dots).child(label));
    }

    if let Some(header) = &question.header {
        let mut h = Node::text(header.clone());
        h.style.text_size = Some(font_size * 0.85);
        h.style.descriptor.text_color = Some(progress_color);
        root = root.child(h);
    }

    let mut prompt = Node::text(question.prompt.clone());
    prompt.style.text_size = Some(prompt_size);
    prompt.style.descriptor.text_color = Some(prompt_color);
    let mut root = root.child(prompt);

    // The question and its answers are separate units, so the step between them
    // is larger than the gap stacking progress, header and prompt.
    let mut options = Node::container();
    options.style.descriptor.layout.direction = LayoutDirection::Column;
    options.style.fill_width = true;
    options.style.descriptor.layout.spacing.gap = option_gap;
    options.style.descriptor.layout.spacing.padding.top =
        rem_to_px(spec.prompt_gap_rem(density)) - gap;
    options.a11y.role = Some(if spec.is_multi_select() {
        NodeRole::Group
    } else {
        NodeRole::RadioGroup
    });
    options.a11y.label = Some(question.prompt.clone());

    for (index, option) in question.options.iter().enumerate() {
        let selected = spec.is_selected(&option.value);

        let mut label = Node::text(option.label.clone());
        label.style.text_size = Some(font_size);
        label.style.text_weight = Some(600);
        label.style.descriptor.text_color = Some(label_color);

        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.min_width = Some(0.0);
            s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        }
        let mut body = body.child(label);

        if let Some(description) = &option.description {
            let mut d = Node::text(description.clone());
            d.style.text_size = Some(font_size);
            d.style.descriptor.text_color = Some(description_color);
            body = body.child(d);
        }

        let mut row = Node::button("");
        row.a11y.label = Some(option.label.clone());
        row.a11y.role = Some(if spec.is_multi_select() {
            NodeRole::CheckBox
        } else {
            NodeRole::RadioButton
        });
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.fill_width = true;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
            s.descriptor.layout.spacing.gap = rem_to_px(0.5);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = pad_block;
            pad.bottom = pad_block;
            pad.left = pad_inline;
            pad.right = pad_inline;
            s.descriptor.border.width = hairline;
            s.descriptor.border.color = if selected { accent } else { TRANSPARENT };
            s.descriptor.background = Some(if selected { selected_fill } else { option_fill });
        }
        all_radius(&mut row, radius);
        row.interaction.focusable = true;

        // Only multi-select shows a check, so the mode is visible before the
        // first click rather than inferred after it.
        if spec.is_multi_select() {
            let mut check = Node::icon(if selected { "check" } else { "square" }, font_size);
            check.style.descriptor.text_color =
                Some(if selected { accent } else { description_color });
            row = row.child(check);
        }

        row = row.child(body);

        if let Some(shortcut) = spec.shortcut_for(index) {
            let mut key = Node::text(format!("{shortcut}"));
            key.style.text_size = Some(font_size * 0.9);
            key.style.descriptor.text_color = Some(shortcut_color);
            row = row.child(key);
        }

        if let Some(handler) = &handlers.on_select {
            let handler = Arc::clone(handler);
            let value = option.value.clone();
            row.style.descriptor.cursor = CursorHint::Pointer;
            row.interaction.on_activate = Some(Arc::new(move || handler(&value)));
        }

        options = options.child(row);
    }

    root = root.child(options);

    if spec.is_dismissible {
        let mut dismiss = Node::button("");
        dismiss.a11y.label = Some(spec.dismiss_label.clone());
        dismiss.a11y.role = Some(NodeRole::Button);
        dismiss.style.descriptor.background = Some(TRANSPARENT);
        dismiss.interaction.focusable = true;

        let mut label = Node::text(spec.dismiss_label.clone());
        label.style.text_size = Some(font_size);
        label.style.descriptor.text_color = Some(dismiss_color);
        let mut dismiss = dismiss.child(label);

        if let Some(handler) = handlers.on_dismiss {
            let id = question.id.clone();
            dismiss.style.descriptor.cursor = CursorHint::Pointer;
            dismiss.interaction.on_activate = Some(Arc::new(move || handler(&id)));
        }

        root = root.child(dismiss);
    }

    root
}
