//! AgentQuestionRecord — the read-only record an answered question leaves.
//!
//! Contract: `docs/contracts/components/agent-question-record.md`
//! Ported from: `packages/jetstream/components/src/agent_question_record.rs`.
//!
//! No handlers, and that is the component: an answer the agent already has
//! cannot be changed from the transcript, so there is nothing to click.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::AgentQuestionRecordSpec;

use crate::presentation::rem_to_px;

pub fn agent_question_record(
    spec: &AgentQuestionRecordSpec,
    theme: &dyn ThemeProvider,
) -> Node {
    let surface = theme.resolve_color(spec.surface_token());
    let border = theme.resolve_color(spec.border_token());
    let prompt_color = theme.resolve_color(spec.prompt_token());
    let chosen_color = theme.resolve_color(spec.chosen_token());
    let unchosen_color = theme.resolve_color(spec.unchosen_token());
    let mark_color = theme.resolve_color(spec.mark_token());
    let radius = theme.resolve_radius(spec.radius_token());

    let font_size = rem_to_px(spec.font_size_rem());
    let gap = rem_to_px(spec.gap_rem());
    let inset = rem_to_px(spec.padding_inset_rem());
    let hairline = rem_to_px(0.0625);

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.fill_width = true;
        s.descriptor.layout.spacing.gap = gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = inset;
        pad.right = inset;
        pad.top = inset;
        pad.bottom = inset;
        s.descriptor.border.width = hairline;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.background = Some(surface);
    }

    if let Some(header) = &spec.question.header {
        let mut h = Node::text(header.clone());
        h.style.text_size = Some(font_size * 0.85);
        h.style.descriptor.text_color = Some(unchosen_color);
        root = root.child(h);
    }

    let mut prompt = Node::text(spec.question.prompt.clone());
    prompt.style.text_size = Some(font_size);
    prompt.style.descriptor.text_color = Some(prompt_color);
    let mut root = root.child(prompt);

    if spec.shows_options() {
        // Every option survives: why the agent did something is usually
        // answered by what it did not do.
        for option in &spec.question.options {
            let chosen = spec.is_chosen(&option.value);

            let mut row = Node::container();
            {
                let s = &mut row.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.fill_width = true;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.spacing.gap = rem_to_px(0.375);
            }
            // The tick alone is not the signal.
            row.a11y.label = Some(spec.option_accessible_name(&option.value, &option.label));

            let lead = if chosen {
                let mut tick = Node::icon("check", font_size);
                tick.style.descriptor.text_color = Some(mark_color);
                tick
            } else {
                let mut spacer = Node::container();
                let s = &mut spacer.style;
                // Explicit Row (see switch.rs).
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.width = LayoutSizing::Fixed(font_size);
                s.descriptor.layout.height = LayoutSizing::Fixed(font_size);
                spacer
            };

            let mut label = Node::text(option.label.clone());
            label.style.text_size = Some(font_size);
            label.style.descriptor.text_color =
                Some(if chosen { chosen_color } else { unchosen_color });

            root = root.child(row.child(lead).child(label));
        }
    } else {
        let mut summary = Node::text(spec.summary());
        summary.style.text_size = Some(font_size);
        summary.style.descriptor.text_color = Some(chosen_color);
        root = root.child(summary);
    }

    root
}
