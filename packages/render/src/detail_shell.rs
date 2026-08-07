//! DetailShell — detail page shell.
//!
//! Contract: `docs/contracts/components/detail-shell.md`
//! Ported from: `packages/jetstream/components/src/detail_shell.rs`.
//!
//! Anatomy (contract §3): a root section stacking an optional header region
//! (header slot OR title) over either the body (children, when
//! `state="ready"`) or a state region (custom `state_content` slot OR the
//! default state-title + optional state-message). The loading state prepends
//! the shared grid spinner. Root + region stacking gap = `space.stack.lg`.
//! Scroll ownership is expressed with overflow-scroll on the chosen region.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
};
use poodle_specs::{DetailShellSpec, DetailState, ScrollOwner};
use poodle_specs::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::color::mix_srgb;
use crate::spinner::spinner;

/// Build the DetailShell.
///
/// `header` / `content` are the host's header and body slots; `state_content`
/// is an optional custom state-region slot (overrides the default state copy).
pub fn detail_shell(
    spec: &DetailShellSpec,
    theme: &dyn ThemeProvider,
    header: Option<Node>,
    content: Option<Node>,
    state_content: Option<Node>,
) -> Node {
    let bg = theme.resolve_color(spec.body_fill_token());
    // Contract §9: root + region stacking gap = space.stack.lg.
    let stack_gap = theme.resolve_space(spec.stack_gap_token());
    let panel_x = theme.resolve_space("space.panel.x");
    let panel_y = theme.resolve_space("space.panel.y");
    let body_size = theme.resolve_space("typography.body.size");
    let heading_size = theme.resolve_space("typography.heading.size");

    let text_primary = theme.resolve_color(spec.state_title_color_token());
    let header_text = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color(spec.state_message_color_token());
    let border = theme.resolve_color("color.border.subtle");

    // Scroll ownership (contract §3 scrollMode): shell scrolls vs body scrolls.
    let scroll_shell = matches!(spec.scroll_owner, ScrollOwner::Shell);

    let mut shell = Node::container();
    {
        let s = &mut shell.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.fill_width = true;
        s.flex_grow = Some(1.0);
        s.descriptor.layout.spacing.gap = stack_gap;
        s.descriptor.background = Some(bg);
        if scroll_shell {
            s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
        }
    }

    // ── Header region (title OR header slot) ─────────────────────
    if spec.title.is_some() || header.is_some() {
        let mut header_region = Node::container();
        {
            let s = &mut header_region.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.fill_width = true;
            s.border_bottom_width = Some(1.0);
            s.border_color_bottom = Some(border);
            s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = panel_x;
            pad.right = panel_x;
            pad.top = panel_y;
            pad.bottom = panel_y;
        }

        if let Some(ref title) = spec.title {
            let mut t = Node::text(title);
            t.style.descriptor.text_color = Some(header_text);
            t.style.text_size = Some(heading_size);
            t.style.text_weight = Some(600);
            header_region = header_region.child(t);
        }
        if let Some(h) = header {
            header_region = header_region.child(h);
        }
        shell = shell.child(header_region);
    }

    // ── Body / state region ──────────────────────────────────────
    if spec.state == DetailState::Ready {
        let mut body = Node::container();
        {
            let s = &mut body.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.flex_grow = Some(1.0);
            s.descriptor.layout.spacing.gap = stack_gap;
            s.fill_width = true;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = panel_x;
            pad.right = panel_x;
            pad.top = panel_y;
            pad.bottom = panel_y;
            if !scroll_shell {
                s.descriptor.layout.overflow_x = LayoutOverflow::Scroll;
                s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
            }
        }
        let mut body = body;
        if let Some(c) = content {
            body = body.child(c);
        }
        shell = shell.child(body);
    } else {
        // Contract §9-10: subtle surface, doubled panel-y / 1.5× panel-x
        // padding, radius-surface corners.
        let state_fill = theme.resolve_color(spec.state_fill_token());
        let state_mix = theme.resolve_color(spec.state_fill_mix_token());
        let state_bg = mix_srgb(state_fill, state_mix, 0.96);
        let state_radius = theme.resolve_radius(spec.state_radius_token());
        let state_pad_y = theme.resolve_space(spec.state_pad_y_token()) * 2.0;
        let state_pad_x = theme.resolve_space(spec.state_pad_x_token()) * 1.5;

        let mut region = Node::container();
        {
            let s = &mut region.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.flex_grow = Some(1.0);
            s.descriptor.layout.spacing.gap = stack_gap;
            s.fill_width = true;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = state_pad_x;
            pad.right = state_pad_x;
            pad.top = state_pad_y;
            pad.bottom = state_pad_y;
            s.descriptor.background = Some(state_bg);
            s.descriptor.corner_radii.top_left = state_radius;
            s.descriptor.corner_radii.top_right = state_radius;
            s.descriptor.corner_radii.bottom_right = state_radius;
            s.descriptor.corner_radii.bottom_left = state_radius;
        }

        if let Some(custom) = state_content {
            region = region.child(custom);
        } else {
            // Loading prepends the shared grid spinner before the copy.
            if spec.state == DetailState::Loading {
                let mut loading = Node::container();
                {
                    let s = &mut loading.style;
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.alignment.main = MainAxisAlignment::Start;
                    s.descriptor.layout.spacing.gap = theme.resolve_space("space.stack.md");
                }
                loading = loading.child(spinner(
                    &SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Grid)
                        .with_size(SpinnerSize::Md)
                        .with_tone(SpinnerTone::Accent),
                    theme,
                ));
                region = region.child(loading);
            }
            // Default state title + optional message.
            let mut title = Node::text(spec.effective_state_title());
            title.style.descriptor.text_color = Some(text_primary);
            title.style.text_size = Some(body_size);
            title.style.text_weight = Some(600);
            region = region.child(title);
            if let Some(ref message) = spec.state_message {
                let mut msg = Node::text(message);
                msg.style.descriptor.text_color = Some(text_secondary);
                msg.style.text_size = Some(body_size);
                region = region.child(msg);
            }
        }

        shell = shell.child(region);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            shell.a11y.label = Some(label.to_string());
        }
    }
    shell
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_shell_keeps_the_header_rule_and_grid_spinner_row() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let spec = DetailShellSpec::new()
            .with_title("Loading")
            .with_state(DetailState::Loading);
        let node = detail_shell(&spec, &theme, None, None, None);

        let header = &node.children[0];
        assert_eq!(header.style.border_bottom_width, Some(1.0));
        assert_eq!(
            header.style.border_color_bottom,
            Some(theme.resolve_color("color.border.subtle"))
        );

        let region = &node.children[1];
        let loading_row = &region.children[0];
        assert_eq!(
            loading_row.style.descriptor.layout.direction,
            LayoutDirection::Row
        );
        assert!(matches!(
            loading_row.children[0].style.descriptor.layout.height,
            LayoutSizing::Fixed(15.0)
        ));
    }
}
