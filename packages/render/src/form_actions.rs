//! FormActions — form action bar.
//!
//! Contract: `docs/contracts/components/form-actions.md`
//! Ported from: `packages/jetstream/components/src/form_actions.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::{
    ButtonVariant, FormActionAlign, FormActionsSpec, IconButtonSpec, SemanticControlSizeRole,
};

use crate::icon_button::icon_button;
use crate::presentation::rem_to_px;

/// Render a form-actions row with no inline danger content (the common case).
pub fn form_actions(
    spec: &FormActionsSpec,
    theme: &dyn ThemeProvider,
    children: Vec<Node>,
) -> Node {
    form_actions_full(spec, theme, Vec::new(), children)
}

/// Render a form-actions row with optional inline danger content (contract
/// `danger` snippet, §2/§8 Danger Inline) plus the collapsed overflow danger
/// menu trigger when `spec.danger_items` is non-empty (§8 Responsive Swap).
pub fn form_actions_full(
    spec: &FormActionsSpec,
    theme: &dyn ThemeProvider,
    danger: Vec<Node>,
    children: Vec<Node>,
) -> Node {
    // Density-keyed gap: compact/comfortable use contract-exact rems,
    // default inherits the inline-md token (contract §8).
    let gap = match spec.gap_rem() {
        Some(rem) => rem_to_px(rem),
        None => theme.resolve_space(spec.action_gap_token()),
    };
    // Density-keyed top separation; default inherits the stack-sm token.
    let separation = if spec.shows_top_separation() {
        match spec.top_separation_rem() {
            Some(rem) => rem_to_px(rem),
            None => theme.resolve_space(spec.stack_separation_token()),
        }
    } else {
        0.0
    };
    // Density-keyed divider offset (contract §8 Divider Offset Variants).
    let border_gap = if spec.shows_top_border() {
        rem_to_px(spec.border_gap_rem())
    } else {
        0.0
    };

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.flex_wrap = true;
        s.descriptor.layout.spacing.padding.top = separation;
        s.descriptor.layout.spacing.margin.top = border_gap;

        if spec.shows_top_border() {
            s.border_top_width = Some(1.0);
            s.descriptor.border.color = theme.resolve_color(spec.border_token());
        }

        s.descriptor.layout.alignment.main = match spec.align {
            FormActionAlign::Start => MainAxisAlignment::Start,
            FormActionAlign::End => MainAxisAlignment::End,
            FormActionAlign::Between => MainAxisAlignment::SpaceBetween,
        };
    }

    // Inline danger group (contract §2 danger snippet, §8 Danger Inline:
    // inline-flex, gap == form-actions gap). Rendered before the primary
    // actions so destructive/cancel content stays visually separated.
    if !danger.is_empty() {
        let mut danger_group = Node::container();
        {
            let s = &mut danger_group.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = gap;
        }
        for child in danger {
            danger_group = danger_group.child(child);
        }
        el = el.child(danger_group);
    }

    // Overflow danger menu trigger (contract §2 danger menu / §8 Responsive
    // Swap). Shown when `dangerItems` is present. The container-query
    // collapse below 31.25rem has no channel here, so both render; the
    // trigger is a real ghost ellipsis icon_button, named per contract §6.
    if spec.has_danger_menu() {
        let trigger = icon_button(
            &IconButtonSpec::new()
                .with_icon("ellipsis")
                .with_variant(ButtonVariant::Ghost)
                .with_aria_label("More actions")
                .with_size_role(SemanticControlSizeRole::Chrome),
            theme,
            None,
        );
        let mut slot = Node::container();
        {
            let s = &mut slot.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        }
        el = el.child(slot.child(trigger));
    }

    for child in children {
        el = el.child(child);
    }

    el
}
