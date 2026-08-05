//! PickerShell — header/toolbar/selection/body/footer chrome for pickers.
//!
//! Contract: `docs/contracts/components/picker-shell.md`
//! Ported from: `packages/jetstream/components/src/picker_shell.rs`.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
};
use poodle_specs::{BrowseState, PickerShellSpec, SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::color::{mix_srgb, TRANSPARENT};
use crate::presentation::rem_to_px;
use crate::spinner::spinner;

pub fn picker_shell(
    spec: &PickerShellSpec,
    theme: &dyn ThemeProvider,
    toolbar: Option<Node>,
    selection: Option<Node>,
    body: Option<Node>,
    state_content: Option<Node>,
    footer: Option<Node>,
) -> Node {
    let panel_x = theme.resolve_space("space.panel.x");
    let panel_y = theme.resolve_space("space.panel.y");
    let gap_sm = theme.resolve_space("space.inline.sm");
    let gap_md = theme.resolve_space("space.inline.md");
    let stack_sm = theme.resolve_space("space.stack.sm");
    let stack_md = theme.resolve_space("space.stack.md");
    let label_size = theme.resolve_space("typography.label.size");
    let body_size = theme.resolve_space("typography.body.size");
    let panel = theme.resolve_color("color.background.panel");
    let elevated = theme.resolve_color("color.background.elevated");
    let surface = theme.resolve_color("color.background.surface");
    let border = theme.resolve_color("color.border.subtle");
    let radius = theme.resolve_radius("radius.surface");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let title_size = rem_to_px(1.25); // contract §8 title 1.25rem (no token resolves to this)
    let border_width = rem_to_px(0.0625); // contract border 0.0625rem

    // Contract §8: modal uses elevated background (96%); popover + inline keep
    // the panel background (94%). `color-mix(in srgb, <c> N%, transparent)`.
    let fill = if spec.is_modal() {
        mix_srgb(elevated, TRANSPARENT, 0.96)
    } else {
        mix_srgb(panel, TRANSPARENT, 0.94)
    };

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };
    let text = |content: String, color, size, weight: Option<u16>| -> Node {
        let mut t = Node::text(content);
        t.style.descriptor.text_color = Some(color);
        t.style.text_size = Some(size);
        t.style.text_weight = weight;
        t
    };

    let mut shell = Node::container();
    {
        let s = &mut shell.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_width;
        s.descriptor.border.color = border;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = stack_md;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = panel_x;
        pad.right = panel_x;
        pad.top = panel_y;
        pad.bottom = panel_y;
    }
    all_radius(&mut shell, radius);

    // Variant elevation + width. Token-accurate per contract §8 (popover →
    // `elevation-overlay`, modal → `elevation-dialog`).
    if spec.is_popover() {
        shell.style.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
        shell.style.fill_width = true;
        shell.style.max_width = Some(rem_to_px(spec.popover_max_width_rem()));
    } else if spec.is_modal() {
        shell.style.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_DIALOG);
    }

    let mut title_block = Node::container();
    title_block.style.descriptor.layout.direction = LayoutDirection::Column;
    title_block.style.descriptor.layout.spacing.gap = stack_sm;
    let mut title_block = title_block.child(text(
        spec.title.clone(),
        text_primary,
        title_size,
        Some(600),
    ));

    if let Some(description) = spec.description.as_ref() {
        title_block = title_block.child(text(description.clone(), text_secondary, label_size, None));
    }

    let mut meta = Node::container();
    meta.style.descriptor.layout.direction = LayoutDirection::Row;
    meta.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    meta.style.descriptor.layout.spacing.gap = gap_sm;
    let mut meta = meta.child(text(
        spec.selected_count_text(),
        text_secondary,
        label_size,
        None,
    ));

    if let Some(result_text) = spec.result_count_text() {
        meta = meta.child(text(result_text, text_secondary, label_size, None));
    }

    let mut header = Node::container();
    {
        let s = &mut header.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.spacing.gap = gap_md;
    }
    let mut shell = shell.child(header.child(title_block).child(meta));

    if let Some(toolbar) = toolbar {
        shell = shell.child(toolbar);
    }

    if let Some(selection) = selection {
        shell = shell.child(selection);
    }

    if let Some(status_text) = spec.status_text.as_ref() {
        // sr-only: keep in tree but clip to 1×1 so it claims no layout space
        // (mirrors the contract §8 sr-only clip). Visual collapse only.
        let mut clip = Node::container();
        {
            let s = &mut clip.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(1.0);
            s.descriptor.layout.height = LayoutSizing::Fixed(1.0);
            s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        }
        shell = shell.child(clip.child(text(
            status_text.clone(),
            text_secondary,
            label_size,
            None,
        )));
    }

    if spec.state == BrowseState::Ready {
        if let Some(body) = body {
            shell = shell.child(body);
        }
    } else if let Some(state_content) = state_content {
        shell = shell.child(state_content);
    } else {
        let mut state = Node::container();
        {
            let s = &mut state.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.spacing.gap = stack_sm;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = panel_x;
            pad.right = panel_x;
            pad.top = panel_y * 1.5;
            pad.bottom = panel_y * 1.5;
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = border;
            s.descriptor.background = Some(surface);
        }
        all_radius(&mut state, radius);

        if spec.state == BrowseState::Loading {
            state = state.child(spinner(
                &SpinnerSpec::new()
                    .with_variant(SpinnerVariant::Grid)
                    .with_size(SpinnerSize::Md)
                    .with_tone(SpinnerTone::Accent),
                theme,
            ));
        }

        state = state.child(text(
            spec.effective_state_title().to_string(),
            text_primary,
            body_size,
            Some(600),
        ));

        if let Some(message) = spec.effective_state_message() {
            state = state.child(text(message.to_string(), text_secondary, label_size, None));
        }

        shell = shell.child(state);
    }

    if let Some(footer) = footer {
        let mut bar = Node::container();
        bar.style.descriptor.layout.direction = LayoutDirection::Row;
        bar.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        bar.style.descriptor.layout.spacing.gap = theme.resolve_space(spec.footer_gap_token());
        shell = shell.child(bar.child(footer));
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            shell.a11y.label = Some(label.to_string());
        }
    }
    shell
}
