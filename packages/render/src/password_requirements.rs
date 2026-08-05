//! PasswordRequirements — policy checklist with pass/fail indicators.
//!
//! Contract: `docs/contracts/components/password-requirements.md`
//! Ported from: `packages/jetstream/components/src/password_requirements.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{ColorValue, CrossAxisAlignment, LayoutDirection, Node, NodeRole};
use poodle_specs::PasswordRequirementsSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};

/// Build a single requirement row with pass/fail indicator.
fn requirement_item(
    label_text: &str,
    is_met: bool,
    met_color: ColorValue,
    unmet_color: ColorValue,
    font_size: f32,
    icon_size: f32,
    gap: f32,
) -> Node {
    let indicator_icon = if is_met { "check" } else { "x" };
    let indicator_color = if is_met { met_color } else { unmet_color };

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    }
    let mut icon = Node::icon(indicator_icon, icon_size);
    icon.style.descriptor.text_color = Some(indicator_color);
    let mut label = Node::text(label_text);
    label.style.text_size = Some(font_size);
    // The label tracks the indicator colour: a met requirement reads green.
    label.style.descriptor.text_color = Some(indicator_color);
    row.child(icon).child(label)
}

pub fn password_requirements(
    spec: &PasswordRequirementsSpec,
    theme: &dyn ThemeProvider,
) -> Node {
    // ── Token resolution ──
    let fill = theme.resolve_color(spec.fill_token());
    let border_color = theme.resolve_color(spec.border_token());
    let title_color = theme.resolve_color(spec.title_color_token());
    let text_color = theme.resolve_color(spec.text_color_token());
    let met_color = theme.resolve_color(spec.met_color_token());
    let error_color = theme.resolve_color(spec.error_color_token());

    // ── Sizing (contract §7 ladder, resolved via effective size) ──
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let title_size = rem_to_px(PasswordRequirementsSpec::title_size_rem(effective_size));
    let body_size = rem_to_px(PasswordRequirementsSpec::body_size_rem(effective_size));
    let icon_size = body_size; // indicator tracks body type size
    let gap = rem_to_px(PasswordRequirementsSpec::hint_gap_rem(effective_size));
    let item_gap = rem_to_px(0.375); // indicator↔label gap
    let padding = rem_to_px(PasswordRequirementsSpec::padding_rem(effective_size));
    let border_width = theme.resolve_space(spec.border_width_token());
    let radius = theme.resolve_radius(spec.radius_token());

    // ── Root container ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = padding;
        pad.right = padding;
        pad.top = padding;
        pad.bottom = padding;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border_color;
    }

    let text = |content: &str, size: f32, color: ColorValue| {
        let mut t = Node::text(content);
        t.style.text_size = Some(size);
        t.style.descriptor.text_color = Some(color);
        t
    };

    // ── Loading state (no title — matches the reference) ──
    if spec.is_loading {
        return root.child(text(&spec.loading_label, body_size, text_color));
    }

    // ── Requirements checklist (title lives only in this branch) ──
    if let Some(ref policy) = spec.requirements {
        let mut title = text(&format!("{}:", spec.title), title_size, title_color);
        title.style.text_weight = Some(600);
        root = root.child(title);

        root = root.child(requirement_item(
            &format!("At least {} characters", policy.min_length),
            spec.length_met(),
            met_color,
            text_color,
            body_size,
            icon_size,
            item_gap,
        ));

        if policy.require_mixed_case {
            root = root.child(requirement_item(
                "Mix of uppercase and lowercase letters",
                spec.mixed_case_met(),
                met_color,
                text_color,
                body_size,
                icon_size,
                item_gap,
            ));
        }

        if policy.require_digit {
            root = root.child(requirement_item(
                "At least one number",
                spec.digit_met(),
                met_color,
                text_color,
                body_size,
                icon_size,
                item_gap,
            ));
        }

        if policy.require_special {
            root = root.child(requirement_item(
                "At least one special character",
                spec.special_met(),
                met_color,
                text_color,
                body_size,
                icon_size,
                item_gap,
            ));
        }

        if let Some(ref description) = policy.description {
            root = root.child(text(description.as_str(), body_size, text_color));
        }

        // Hint — only inside the requirements branch (matches the reference).
        if let Some(ref hint) = spec.hint {
            root = root.child(text(hint.as_str(), body_size, text_color));
        }
    } else if let Some(ref error) = spec.error {
        // ── Error state (no requirements, no title) ──
        root = root.child(text(error.as_str(), body_size, error_color));
    }

    root.a11y.role = Some(NodeRole::Alert);
    root
}
