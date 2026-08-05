//! MarkdownEditor — toolbar + edit/split/preview panes.
//!
//! Contract: `docs/contracts/components/markdown-editor.md`
//! Ported from: `packages/jetstream/components/src/markdown_editor.rs`.
//!
//! Real text editing + toolbar markdown insertion live in the host event loop
//! (Tier-3), so the edit pane is an Input node (placeholder + current value)
//! and the preview shows source text.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node,
};
use poodle_specs::{ButtonVariant, IconButtonSpec, MarkdownEditorSpec};

use crate::color::with_alpha;
use crate::icon_button::icon_button;
use crate::presentation::rem_to_px;

/// Toolbar formatting tools in contract §2 anatomy order, with their icons.
const TOOLS: [(&str, &str); 7] = [
    ("bold", "Bold"),
    ("italic", "Italic"),
    ("heading", "Heading"),
    ("link", "Link"),
    ("code", "Code"),
    ("quote", "Quote"),
    ("list", "List"),
];

pub fn markdown_editor(spec: &MarkdownEditorSpec, theme: &dyn ThemeProvider) -> Node {
    // ── Size / density geometry (contract §8 tables, token-resolved rem) ──
    let tool_size = rem_to_px(spec.tool_size_rem());
    let toolbar_y = rem_to_px(spec.toolbar_y_rem());
    let toolbar_x = rem_to_px(spec.toolbar_x_rem());
    let tool_gap = rem_to_px(spec.tool_gap_rem());
    let pane_pad = rem_to_px(spec.pane_pad_rem());
    let mode_x = rem_to_px(spec.mode_x_rem());
    let mode_y = rem_to_px(spec.mode_y_rem());
    let toolbar_gap = rem_to_px(0.5); // contract toolbar `gap: 0.5rem`
    let tool_font = rem_to_px(0.75); // contract tool font-size
    let textarea_font = rem_to_px(0.8125); // contract textarea font-size
    let preview_font = rem_to_px(0.875); // contract preview font-size
    let min_h = rem_to_px(spec.min_height_rem());

    // ── Token resolution ──────────────────────────────────────
    let fill = theme.resolve_color(spec.fill_token());
    let border = theme.resolve_color(spec.border_token());
    let toolbar_border = theme.resolve_color(spec.toolbar_border_token());
    let split_divider = theme.resolve_color(spec.split_divider_color_token());
    let text_primary = theme.resolve_color(spec.textarea_color_token());
    let tool_color = theme.resolve_color(spec.tool_color_token());
    let placeholder_color = theme.resolve_color(spec.placeholder_color_token());
    let preview_empty_color = theme.resolve_color(spec.preview_empty_color_token());
    let radius = theme.resolve_radius("radius.surface");
    let ctrl_radius = theme.resolve_radius("radius.control");
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");

    // Toolbar bg: `color-mix(elevated 72%, transparent)` (alpha reduction).
    let elevated = theme.resolve_color("color.background.elevated");
    let toolbar_bg = with_alpha(elevated, elevated.3 * 0.72);

    let is_edit = spec.shows_editor();
    let is_preview = spec.shows_preview();
    let tools_disabled = spec.tools_disabled();

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };

    // ── Root ──────────────────────────────────────────────────
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.min_height = Some(min_h);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        if spec.is_disabled {
            s.descriptor.opacity = disabled_opacity;
        }
    }
    all_radius(&mut el, radius);

    // ── Toolbar (bottom border only, space-between) ───────────
    let mut toolbar = Node::container();
    {
        let s = &mut toolbar.style;
        s.descriptor.background = Some(toolbar_bg);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = toolbar_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = toolbar_x;
        pad.right = toolbar_x;
        pad.top = toolbar_y;
        pad.bottom = toolbar_y;
        s.border_bottom_width = Some(1.0);
        s.descriptor.border.color = toolbar_border;
    }

    // Tools container — real Icon glyphs in contract order.
    let mut tools_row = Node::container();
    tools_row.style.descriptor.layout.direction = LayoutDirection::Row;
    tools_row.style.descriptor.layout.spacing.gap = tool_gap;
    for (icon_name, label) in &TOOLS {
        let mut btn = Node::button("");
        // TOOLS already pairs each glyph with its name; rendering the tool
        // icon-only discarded it and left a row of unnamed buttons.
        btn.a11y.label = Some((*label).to_string());
        {
            let s = &mut btn.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(tool_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(tool_size);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }
        all_radius(&mut btn, ctrl_radius);
        if tools_disabled {
            btn.style.descriptor.opacity = 0.4; // contract tool `:disabled` opacity
        } else {
            btn.interaction.focusable = true;
            btn.style.descriptor.cursor = CursorHint::Pointer;
        }
        let mut glyph = Node::icon(*icon_name, tool_font);
        glyph.style.descriptor.text_color = Some(tool_color);
        tools_row = tools_row.child(btn.child(glyph));
    }
    let mut toolbar = toolbar.child(tools_row);

    // Mode switcher — delegates chrome to IconButton.
    let mode_button = |icon: &str, aria: &str, mode_val: &str| -> Node {
        icon_button(
            &IconButtonSpec::new()
                .with_icon(icon)
                .with_aria_label(aria)
                .with_variant(if spec.mode == mode_val {
                    ButtonVariant::Secondary
                } else {
                    ButtonVariant::Ghost
                })
                .with_size(spec.size)
                .with_size_role(spec.size_role)
                .with_density(spec.density),
            theme,
            None,
        )
    };
    let mut modes = Node::container();
    {
        let s = &mut modes.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.spacing.gap = tool_gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = mode_x;
        pad.right = mode_x;
        pad.top = mode_y;
        pad.bottom = mode_y;
    }
    toolbar = toolbar.child(
        modes
            .child(mode_button("pencil", "Edit", "edit"))
            .child(mode_button("columns-2", "Split", "split"))
            .child(mode_button("eye", "Preview", "preview")),
    );
    let mut el = el.child(toolbar);

    // ── Body ──────────────────────────────────────────────────
    let mut body = Node::container();
    body.style.descriptor.layout.direction = LayoutDirection::Row;
    body.style.descriptor.layout.width = LayoutSizing::Grow;

    if is_edit {
        let placeholder = spec.placeholder.as_deref().unwrap_or("Write markdown...");
        // Real text surface — an Input node carrying the value + placeholder.
        // Editing/insertion is owned by the host loop (Tier-3).
        let mut input = Node::input(spec.value.clone(), placeholder);
        // Svelte defaults `ariaLabel` to this rather than leaving the editing
        // surface nameless; a placeholder is not a name, so without it the
        // textarea announces as untitled.
        input.a11y.label = Some(if spec.aria_label.is_empty() {
            "Markdown editor".to_string()
        } else {
            spec.aria_label.clone()
        });
        {
            let s = &mut input.style;
            s.fill_width = true;
            // Textarea is monospace (contract §8 `.md-editor__textarea`).
            s.text_size = Some(textarea_font);
            s.font_family = Some(FontFamily::Mono);
            s.descriptor.text_color = Some(if spec.value.is_empty() {
                placeholder_color
            } else {
                text_primary
            });
        }

        let mut textarea = Node::container();
        {
            let s = &mut textarea.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Grow;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pane_pad;
            pad.right = pane_pad;
            pad.top = pane_pad;
            pad.bottom = pane_pad;
            // Split mode: textarea gets a right border (contract border-right).
            if is_preview {
                s.border_right_width = Some(1.0);
                s.descriptor.border.color = split_divider;
            }
        }
        body = body.child(textarea.child(input));
    }

    if is_preview {
        let mut preview = Node::container();
        {
            let s = &mut preview.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Grow;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pane_pad;
            pad.right = pane_pad;
            pad.top = pane_pad;
            pad.bottom = pane_pad;
        }

        let mut copy = if spec.value.is_empty() {
            let mut t = Node::text("Nothing to preview");
            t.style.descriptor.text_color = Some(preview_empty_color);
            t
        } else {
            // Rendered HTML is Tier-3; show source text until a renderer is
            // plugged.
            let mut t = Node::text(&spec.value);
            t.style.descriptor.text_color = Some(text_primary);
            t
        };
        copy.style.text_size = Some(preview_font);
        body = body.child(preview.child(copy));
    }

    let mut el = el.child(body);
    if !spec.aria_label.is_empty() {
        el.a11y.label = Some(spec.aria_label.clone());
    }
    el
}
