//! AgentQuestion — a question the agent needs answered before it continues.
//!
//! Contract: `docs/contracts/components/agent-question.md`
//! Ported from: `packages/jetstream/components/src/agent_question.rs`.

use std::sync::Arc;

use poodle_headless::agent_question::QuestionProgressState;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node, NodeRole, NodeToggled};
use poodle_specs::{
    AgentQuestionSpec, ButtonSpec, ButtonVariant, TextSize, TextSpec, TextTone, TextWeight,
};

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
    /// Stable native instance scope for the root and its controls. Duplicate
    /// questions can share option values, so question ids alone are not enough
    /// to keep backend focus and input state separate.
    pub instance_id: Option<String>,
}

/// The backend-state id of one question option.
pub fn agent_question_option_focus_id(instance_id: Option<&str>, value: &str) -> String {
    match instance_id {
        Some(scope) => format!("agent-question:{scope}:option:{value}"),
        None => format!("agent-question-option-{value}"),
    }
}

/// The backend-state id of the optional dismiss control.
pub fn agent_question_dismiss_focus_id(instance_id: Option<&str>) -> String {
    match instance_id {
        Some(scope) => format!("agent-question:{scope}:dismiss"),
        None => "agent-question-dismiss".to_owned(),
    }
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
    root.runtime_id = handlers
        .instance_id
        .as_deref()
        .map(|scope| format!("agent-question:{scope}"));
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.fill_width = true;
    root.style.descriptor.layout.spacing.gap = gap;
    root.roles.insert(
        "size".to_owned(),
        format!("{base_size:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "multi-select".to_owned(),
        spec.is_multi_select().to_string(),
    );

    let text = |content: &str,
                tone: TextTone,
                size: TextSize,
                weight: TextWeight,
                resolved_size: f32,
                color| {
        let mut node = crate::text::text(
            &TextSpec::new(content)
                .with_tone(tone)
                .with_size(size)
                .with_weight(weight),
            ctx,
        );
        node.style.text_size = Some(resolved_size);
        node.style.descriptor.text_color = Some(color);
        node
    };

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
            dot.roles
                .insert("state".to_owned(), state.as_str().to_owned());
            dots = dots.child(dot);
        }

        // The dots are a picture of the label; the label carries the fact for
        // anyone who cannot see them.
        let label = text(
            &spec.resolved_progress_label(),
            TextTone::Secondary,
            TextSize::Xs,
            TextWeight::Normal,
            font_size,
            progress_color,
        );
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        row.style.descriptor.layout.spacing.gap = rem_to_px(0.5);
        root = root.child(row.child(dots).child(label));
    }

    if let Some(header) = &question.header {
        let h = text(
            header,
            TextTone::Secondary,
            TextSize::Xs,
            TextWeight::Semibold,
            font_size * 0.85,
            progress_color,
        );
        root = root.child(h);
    }

    let prompt = text(
        &question.prompt,
        TextTone::Default,
        TextSize::Md,
        TextWeight::Normal,
        prompt_size,
        prompt_color,
    );
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

        let label = text(
            &option.label,
            TextTone::Default,
            TextSize::Sm,
            TextWeight::Semibold,
            font_size,
            label_color,
        );

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
            let d = text(
                description,
                TextTone::Secondary,
                TextSize::Sm,
                TextWeight::Normal,
                font_size,
                description_color,
            );
            body = body.child(d);
        }

        let on_click = handlers.on_select.as_ref().map(|handler| {
            let handler = Arc::clone(handler);
            let value = option.value.clone();
            Arc::new(move || handler(&value)) as Arc<dyn Fn() + Send + Sync>
        });
        let button_spec = ButtonSpec::new()
            .with_label("")
            .with_aria_label(option.label.clone())
            .with_variant(ButtonVariant::Secondary)
            .with_size(base_size)
            .with_density(density)
            .with_pressed(selected);
        let mut row = crate::button::button(&button_spec, ctx, on_click);
        row.id = Some(format!("agent-question-option-{}", option.value));
        row.runtime_id = Some(agent_question_option_focus_id(
            handlers.instance_id.as_deref(),
            &option.value,
        ));
        row.a11y.label = Some(option.label.clone());
        row.a11y.role = Some(if spec.is_multi_select() {
            NodeRole::CheckBox
        } else {
            NodeRole::RadioButton
        });
        row.a11y.selected = Some(selected);
        row.a11y.toggled = Some(if selected {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        row.roles
            .insert("selected".to_owned(), selected.to_string());
        {
            let s = &mut row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.fill_width = true;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.descriptor.layout.height = LayoutSizing::Fit;
            s.min_width = None;
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
            s.descriptor.text_color = Some(label_color);
        }
        all_radius(&mut row, radius);

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
            let key = text(
                &shortcut.to_string(),
                TextTone::Secondary,
                TextSize::Xs,
                TextWeight::Normal,
                font_size * 0.9,
                shortcut_color,
            );
            row = row.child(key);
        }

        options = options.child(row);
    }

    root = root.child(options);

    if spec.is_dismissible {
        let on_click = handlers.on_dismiss.map(|handler| {
            let id = question.id.clone();
            Arc::new(move || handler(&id)) as Arc<dyn Fn() + Send + Sync>
        });
        let button_spec = ButtonSpec::new()
            .with_label(spec.dismiss_label.clone())
            .with_aria_label(spec.dismiss_label.clone())
            .with_variant(ButtonVariant::Ghost)
            .with_size(base_size)
            .with_density(density);
        let mut dismiss = crate::button::button(&button_spec, ctx, on_click);
        dismiss.id = Some("agent-question-dismiss".to_owned());
        dismiss.runtime_id = Some(agent_question_dismiss_focus_id(
            handlers.instance_id.as_deref(),
        ));
        dismiss.style.descriptor.background = Some(TRANSPARENT);
        dismiss.style.descriptor.text_color = Some(dismiss_color);
        dismiss.style.text_size = Some(font_size);

        root = root.child(dismiss);
    }

    root
}
