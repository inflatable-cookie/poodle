//! MarkdownEditor — toolbar + edit/split/preview panes.
//!
//! Contract: `docs/contracts/components/markdown-editor.md`
//! Ported from: `packages/jetstream/components/src/markdown_editor.rs`.
//!
//! The edit pane is a backend Input node. Keystrokes go through the shared
//! text-input edit model and report via `on_change`. Toolbar tools apply the
//! same markdown syntax as Svelte (wrap or line-prefix the current value;
//! selection is Tier-3) and report the next source through `on_change`. Preview
//! still shows source text until a renderer is plugged (Tier-3).

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeRole, StylePatch, TextChangeHandler,
};
use poodle_specs::{ButtonVariant, IconButtonSpec, MarkdownEditorSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
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

/// Host-owned interactions for the markdown editor.
#[derive(Default, Clone)]
pub struct MarkdownEditorHandlers {
    pub on_change: Option<TextChangeHandler>,
    pub on_mode_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

/// Svelte `insertMarkdown` / `insertLine` against the whole current value.
/// Native has no textarea selection, so wrap/prefix the source the host holds
/// (contract §11 Tier-3 insertion mechanics).
fn apply_toolbar_tool(value: &str, label: &str) -> String {
    let selected = if value.is_empty() { "text" } else { value };
    match label {
        "Bold" => format!("**{selected}**"),
        "Italic" => format!("*{selected}*"),
        "Heading" => format!("## {value}"),
        "Link" => format!("[{selected}](url)"),
        "Code" => format!("`{selected}`"),
        "Quote" => format!("> {value}"),
        "List" => format!("- {value}"),
        _ => value.to_string(),
    }
}

pub fn markdown_editor(spec: &MarkdownEditorSpec, ctx: &RenderContext<'_>) -> Node {
    markdown_editor_with_handlers(spec, ctx, MarkdownEditorHandlers::default())
}

pub fn markdown_editor_with_handlers(
    spec: &MarkdownEditorSpec,
    ctx: &RenderContext<'_>,
    handlers: MarkdownEditorHandlers,
) -> Node {
    // ── Size / density geometry (contract §8 tables, token-resolved rem) ──
    // `tool_size_rem`/`mode_x_rem` apply the size role internally, so they
    // take the base (pre-role) size; the density tables take the resolved
    // density.
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let tool_size = rem_to_px(spec.tool_size_rem(base_size));
    let toolbar_y = rem_to_px(spec.toolbar_y_rem(density));
    let toolbar_x = rem_to_px(spec.toolbar_x_rem(density));
    let tool_gap = rem_to_px(spec.tool_gap_rem(density));
    let pane_pad = rem_to_px(spec.pane_pad_rem(density));
    let mode_x = rem_to_px(spec.mode_x_rem(base_size));
    let mode_y = rem_to_px(spec.mode_y_rem(density));
    let toolbar_gap = rem_to_px(0.5); // contract toolbar `gap: 0.5rem`
    let tool_font = rem_to_px(0.75); // contract tool font-size
    let textarea_font = rem_to_px(0.8125); // contract textarea font-size
    let preview_font = rem_to_px(0.875); // contract preview font-size
    let min_h = rem_to_px(spec.min_height_rem());

    // ── Token resolution ──────────────────────────────────────
    let fill = ctx.theme().resolve_color(spec.fill_token());
    let border = ctx.theme().resolve_color(spec.border_token());
    let toolbar_border = ctx.theme().resolve_color(spec.toolbar_border_token());
    let split_divider = ctx.theme().resolve_color(spec.split_divider_color_token());
    let text_primary = ctx.theme().resolve_color(spec.textarea_color_token());
    let tool_color = ctx.theme().resolve_color(spec.tool_color_token());
    let placeholder_color = ctx.theme().resolve_color(spec.placeholder_color_token());
    let preview_empty_color = ctx.theme().resolve_color(spec.preview_empty_color_token());
    let radius = ctx.theme().resolve_radius("radius.surface");
    let ctrl_radius = ctx.theme().resolve_radius("radius.control");
    let disabled_opacity = ctx.theme().resolve_opacity("state.opacity.disabled");

    // Toolbar bg: `color-mix(elevated 72%, transparent)` (alpha reduction).
    let elevated = ctx.theme().resolve_color("color.background.elevated");
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
    // Column + min_height 0 + overflow hidden matches shared CSS: a definite
    // host can shrink the editor without a public height prop; short content
    // stays natural when the host does not constrain (no fill_height / height
    // 100%). Textarea minHeight stays on the editing pane, not the root.
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.fill_width = true;
        s.min_height = Some(0.0);
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
        s.flex_shrink_zero = true;
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
        btn.a11y.role = Some(NodeRole::Button);
        btn.tooltip = Some((*label).to_string());
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
            btn.interaction.disabled = true;
        } else {
            btn.interaction.focusable = true;
            btn.style.descriptor.cursor = CursorHint::Pointer;
            if let Some(on_change) = handlers.on_change.clone() {
                let current = spec.value.clone();
                let label = (*label).to_string();
                btn.interaction.on_activate = Some(Arc::new(move || {
                    on_change(&apply_toolbar_tool(&current, &label));
                }));
            }
        }
        let mut glyph = Node::icon(*icon_name, tool_font);
        glyph.style.descriptor.text_color = Some(tool_color);
        tools_row = tools_row.child(btn.child(glyph));
    }
    let mut toolbar = toolbar.child(tools_row);

    // Mode switcher — delegates chrome to IconButton.
    let mode_button = |icon: &str, aria: &str, mode_val: &str| -> Node {
        let on_activate = if spec.mode == mode_val {
            None
        } else {
            handlers.on_mode_change.as_ref().map(|handler| {
                let handler = Arc::clone(handler);
                let mode = mode_val.to_string();
                Arc::new(move || handler(&mode)) as Arc<dyn Fn() + Send + Sync>
            })
        };
        icon_button(
            &IconButtonSpec::new()
                .with_icon(icon)
                .with_aria_label(aria)
                .with_variant(if spec.mode == mode_val {
                    ButtonVariant::Secondary
                } else {
                    ButtonVariant::Ghost
                })
                .with_size(base_size)
                .with_size_role(spec.size_role)
                .with_density(density)
                .with_disabled(spec.is_disabled),
            ctx,
            on_activate,
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
    let el = el.child(toolbar);

    // ── Body ──────────────────────────────────────────────────
    // Same shrink chain as shared CSS: body fills remaining column height and
    // may shrink; the preview pane owns vertical scroll (contract §7).
    let mut body = Node::container();
    {
        let s = &mut body.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.fill_width = true;
        s.fill_height = true;
        s.min_height = Some(0.0);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
    }

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
        input.a11y.role = Some(NodeRole::TextInput);
        if !spec.value.is_empty() {
            input.a11y.value_text = Some(spec.value.clone());
        }
        input.interaction.focusable = !spec.is_disabled;
        input.interaction.disabled = spec.is_disabled;
        if !spec.is_disabled {
            input.a11y.tab_index = Some(0);
            // Contract §6 suppresses the textarea outline; an empty focus
            // patch still opts the backend into a tracked handle so keys
            // reach on_edit_key (bare `focusable` stays untracked).
            input.style.focus = Some(StylePatch {
                background: None,
                border_color: None,
                text_color: None,
                opacity: None,
            });
            // Keys go through the shared editing model. Native has no stored
            // caret on this spec, so the caret sits at the end of the current
            // value (Tier-3). Printable keys append; Backspace deletes the
            // last character.
            let value = spec.value.clone();
            let on_change = handlers.on_change.clone();
            input.interaction.on_edit_key = Some(Arc::new(move |key, mods| {
                let len = value.chars().count();
                let state = poodle_headless::text_input::EditState {
                    anchor: len,
                    head: len,
                };
                let Some(outcome) = poodle_headless::text_input::edit_transition(
                    &value, state, key, mods.shift, mods.accel, None,
                ) else {
                    return;
                };
                if let Some(next) = outcome.value {
                    if next != value {
                        if let Some(on_change) = &on_change {
                            on_change(&next);
                        }
                    }
                }
            }));
            {
                let value = spec.value.clone();
                let on_change = handlers.on_change.clone();
                input.interaction.on_edit_insert = Some(Arc::new(move |text: &str| {
                    let len = value.chars().count();
                    let outcome = poodle_headless::text_input::insert_transition(
                        &value,
                        poodle_headless::text_input::EditState {
                            anchor: len,
                            head: len,
                        },
                        text,
                        None,
                    );
                    if let Some(next) = outcome.value {
                        if next != value {
                            if let Some(on_change) = &on_change {
                                on_change(&next);
                            }
                        }
                    }
                }));
            }
            input.interaction.on_text_change = handlers.on_change.clone();
        }
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
            s.fill_height = true;
            // Contract §3 / §7: minHeight is the editing-pane minimum.
            s.min_height = Some(min_h);
            s.min_width = Some(0.0);
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
            // Column so preview content stacks and can overflow vertically;
            // Row would stretch siblings to the viewport and hide scroll extent.
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.fill_height = true;
            s.min_height = Some(0.0);
            s.min_width = Some(0.0);
            // Preview is the vertical scroll owner (contract §7 / §8).
            s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pane_pad;
            pad.right = pane_pad;
            pad.top = pane_pad;
            pad.bottom = pane_pad;
        }
        preview.a11y.label = Some("Preview".to_string());

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

#[cfg(test)]
mod tests {
    use poodle_node::{LayoutDirection, LayoutOverflow};
    use poodle_specs::MarkdownEditorSpec;

    use super::*;
    use crate::context::RenderContext;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn preview_pane(node: &Node) -> &Node {
        let body = node.children.get(1).expect("toolbar then body");
        body.children
            .iter()
            .find(|child| child.a11y.label.as_deref() == Some("Preview"))
            .expect("preview pane carries aria-label Preview")
    }

    fn tool<'a>(node: &'a Node, label: &str) -> &'a Node {
        node.find(&|child| {
            child.a11y.role == Some(poodle_node::NodeRole::Button)
                && child.a11y.label.as_deref() == Some(label)
        })
        .unwrap_or_else(|| panic!("missing {label} tool"))
    }

    fn textarea(node: &Node) -> &Node {
        node.find(&|child| child.a11y.role == Some(poodle_node::NodeRole::TextInput))
            .expect("edit pane is a labelled text input")
    }

    #[test]
    fn apply_toolbar_tool_matches_svelte_syntax() {
        assert_eq!(apply_toolbar_tool("hello", "Bold"), "**hello**");
        assert_eq!(apply_toolbar_tool("", "Bold"), "**text**");
        assert_eq!(apply_toolbar_tool("hello", "Heading"), "## hello");
        assert_eq!(apply_toolbar_tool("", "Heading"), "## ");
        assert_eq!(apply_toolbar_tool("hello", "Link"), "[hello](url)");
        assert_eq!(apply_toolbar_tool("hello", "List"), "- hello");
    }

    #[test]
    fn toolbar_bold_wraps_the_current_value_through_on_change() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let received = std::sync::Arc::new(std::sync::Mutex::new(None));
        let sink = std::sync::Arc::clone(&received);
        let node = markdown_editor_with_handlers(
            &MarkdownEditorSpec::new().with_value("hello"),
            &ctx,
            MarkdownEditorHandlers {
                on_change: Some(std::sync::Arc::new(move |value: &str| {
                    *sink.lock().expect("change") = Some(value.to_string());
                })),
                ..MarkdownEditorHandlers::default()
            },
        );
        let bold = tool(&node, "Bold");
        assert!(bold.interaction.focusable);
        assert!(!bold.interaction.disabled);
        (bold
            .interaction
            .on_activate
            .as_ref()
            .expect("bold activate"))();
        assert_eq!(
            received.lock().expect("change").as_deref(),
            Some("**hello**")
        );
    }

    #[test]
    fn preview_mode_disables_tool_buttons() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = markdown_editor(
            &MarkdownEditorSpec::new()
                .with_mode("preview")
                .with_value("# Hello"),
            &ctx,
        );
        let bold = tool(&node, "Bold");
        assert!(bold.interaction.disabled);
        assert!(!bold.interaction.focusable);
        assert!(bold.interaction.on_activate.is_none());
    }

    #[test]
    fn edit_pane_is_a_labelled_text_input() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = markdown_editor_with_handlers(
            &MarkdownEditorSpec::new()
                .with_value("hello")
                .with_aria_label("Notes"),
            &ctx,
            MarkdownEditorHandlers {
                on_change: Some(std::sync::Arc::new(|_: &str| {})),
                ..MarkdownEditorHandlers::default()
            },
        );
        let input = textarea(&node);
        assert_eq!(input.a11y.label.as_deref(), Some("Notes"));
        assert_eq!(input.a11y.value_text.as_deref(), Some("hello"));
        assert!(input.interaction.on_edit_key.is_some());
        assert!(input.interaction.on_text_change.is_some());
        assert!(input.style.focus.is_some());
    }

    #[test]
    fn preview_pane_declares_vertical_scroll_overflow() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = MarkdownEditorSpec::new()
            .with_mode("preview")
            .with_value("# Long\n\n".repeat(40));
        let node = markdown_editor(&spec, &ctx);
        let preview = preview_pane(&node);
        assert_eq!(
            preview.style.descriptor.layout.overflow_y,
            LayoutOverflow::Scroll
        );
        assert_eq!(
            preview.style.descriptor.layout.direction,
            LayoutDirection::Column
        );
        assert_eq!(preview.style.min_height, Some(0.0));
        assert!(preview.style.fill_height);
    }

    #[test]
    fn split_body_shrinks_and_preview_owns_scroll() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = MarkdownEditorSpec::new()
            .with_mode("split")
            .with_value(format!(
                "short source\n\n{}",
                "# Heading\n\nparagraph\n\n".repeat(30)
            ));
        let node = markdown_editor(&spec, &ctx);
        let body = node.children.get(1).expect("toolbar then body");
        assert_eq!(body.style.min_height, Some(0.0));
        assert!(body.style.fill_height);
        assert_eq!(
            body.style.descriptor.layout.overflow_y,
            LayoutOverflow::Hidden
        );
        let preview = preview_pane(&node);
        assert_eq!(
            preview.style.descriptor.layout.overflow_y,
            LayoutOverflow::Scroll
        );
    }
}
