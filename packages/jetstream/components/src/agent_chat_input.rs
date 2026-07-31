//! AgentChatInput — Jetstream agent composer backed by AgentChatInputSpec.
//!
//! Contract: `docs/contracts/components/agent-chat-input.md`
//! Reference: `packages/svelte/components/src/AgentChatInput.svelte`
//!
//! Anatomy: root → field (attachment chips → editor → toolbar with host-supplied
//! leading children, context ring and the submit/stop action) → optional footer
//! bar.
//!
//! Text editing and key handling live in the host event loop; the editor part
//! renders the current value (or the placeholder) as text, and the auto-grow
//! height comes from `visible_rows()` rather than text measurement (contract §12).

use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{AgentChatInputSpec, ControlSize, MeterShape, MeterSpec};

use crate::meter::js_meter;
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius, tint};

/// AgentChatInput — the composer.
///
/// One action control serves submit and stop, so one press event serves both:
/// `on_action` fires when the control is pressed, and the host reads the spec
/// it already holds to know which intent that was — the same derivation the
/// control itself uses to pick its icon.
///
/// `on_remove_attachment` carries the attachment's id. `on_value_change` is
/// typed; no route.
pub struct AgentChatInput {
    spec: AgentChatInputSpec,
    theme: JetstreamThemeProvider,
    toolbar_children: Vec<JsEl>,
    footer_children: Vec<JsEl>,
    on_action: Option<crate::element::ActionHandler>,
    on_remove_attachment: Option<crate::element::Handler>,
}

impl AgentChatInput {
    pub fn from_spec(spec: AgentChatInputSpec, theme: &JetstreamThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            toolbar_children: Vec::new(),
            footer_children: Vec::new(),
            on_action: None,
            on_remove_attachment: None,
        }
    }

    pub fn toolbar_child(mut self, child: impl crate::element::IntoJsEl) -> Self {
        self.toolbar_children.push(child.into_js_el());
        self
    }

    pub fn footer_child(mut self, child: impl crate::element::IntoJsEl) -> Self {
        self.footer_children.push(child.into_js_el());
        self
    }

    /// Fires when the action control is pressed — submit or stop, per the spec.
    /// Never fires when the control is inert (`can_submit` false and nothing
    /// streaming to stop).
    pub fn on_action(mut self, handler: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_action = Some(std::sync::Arc::new(handler));
        self
    }

    /// Fires with the removed attachment's id.
    pub fn on_remove_attachment(mut self, handler: impl Fn(&str) + Send + Sync + 'static) -> Self {
        self.on_remove_attachment = Some(std::sync::Arc::new(handler));
        self
    }
}

impl crate::element::IntoJsEl for AgentChatInput {
    fn into_js_el(self) -> JsEl {
        build(
            &self.spec,
            &self.theme,
            self.toolbar_children,
            self.footer_children,
            self.on_action,
            self.on_remove_attachment,
        )
    }
}

pub fn js_agent_chat_input(
    spec: &AgentChatInputSpec,
    theme: &JetstreamThemeProvider,
    toolbar_children: Vec<JsEl>,
    footer_children: Vec<JsEl>,
) -> JsEl {
    build(spec, theme, toolbar_children, footer_children, None, None)
}

fn build(
    spec: &AgentChatInputSpec,
    theme: &JetstreamThemeProvider,
    toolbar_children: Vec<JsEl>,
    footer_children: Vec<JsEl>,
    on_action: Option<crate::element::ActionHandler>,
    on_remove_attachment: Option<crate::element::Handler>,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Size table (contract §8) ──────────────────────────────────────────────
    let (pad_y_rem, pad_x_rem) = spec.field_padding_rem(effective_size);
    let pad_y = rem_to_px(pad_y_rem);
    let pad_x = rem_to_px(pad_x_rem);
    let editor_font = rem_to_px(spec.editor_font_rem(effective_size));
    let action_box = rem_to_px(spec.action_size_rem(effective_size));
    let gap = rem_to_px(spec.toolbar_gap_rem(effective_size)) * spec.density_gap_scale();
    let line_height = editor_font * 1.5;
    let divider_height = rem_to_px(spec.toolbar_divider_height_rem(effective_size));

    // ── Colors ────────────────────────────────────────────────────────────────
    let text_primary = resolve_color(theme, spec.text_token());
    let text_secondary = resolve_color(theme, spec.secondary_token());
    // Secondary text held below the muted opacity — a standing hint, not a
    // value (contract §8).
    let placeholder = tint(
        resolve_color(theme, spec.placeholder_token()),
        resolve_opacity(theme, spec.placeholder_opacity_token()) * spec.placeholder_opacity_ratio(),
    );
    let border = resolve_color(theme, spec.field_border_token());
    let divider = resolve_color(theme, spec.divider_token());
    let surface = resolve_color(theme, spec.field_fill_token());
    let elevated = resolve_color(theme, spec.attachment_fill_token());
    let action_fill = resolve_color(theme, spec.action_fill_token());
    let action_text = resolve_color(theme, spec.action_text_token());
    let field_radius = resolve_radius(theme, spec.field_radius_token()) * 1.5;

    let chip_radius = resolve_radius(theme, spec.attachment_radius_token());

    // ── Field ─────────────────────────────────────────────────────────────────
    let mut field = ui_element::div()
        .flex_col()
        .gap(gap)
        .w_full()
        .min_w(0.0)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .rounded(field_radius)
        .border_1()
        .border_color(border)
        .bg(surface);

    // Attachment chips.
    if !spec.attachments.is_empty() {
        let mut chips = ui_element::div()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(rem_to_px(0.375));
        let thumb = rem_to_px(spec.attachment_thumb_rem(effective_size));
        for attachment in spec.attachments.iter() {
            // Image attachments render as tiles: the picture says more than the
            // filename does (contract §2).
            if let Some(url) = &attachment.thumbnail_url {
                let mut tile = ui_element::image(url)
                    .w(thumb)
                    .h(thumb)
                    .flex_none()
                    .rounded(chip_radius)
                    .object_fit_cover();
                if attachment.is_disabled {
                    tile = tile.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
                }
                chips = chips.child(tile);
                continue;
            }

            let mut chip = ui_element::div()
                .flex_row()
                .items_center()
                .gap(rem_to_px(0.25))
                .pl(rem_to_px(0.5))
                .pr(rem_to_px(0.375))
                .pt(rem_to_px(0.125))
                .pb(rem_to_px(0.125))
                .rounded(chip_radius)
                .border_1()
                .border_color(divider)
                .bg(elevated);

            if let Some(icon) = &attachment.icon {
                chip = chip.child(
                    ui_element::icon(icon)
                        .w(rem_to_px(0.75))
                        .h(rem_to_px(0.75))
                        .text_color(text_secondary),
                );
            }

            chip = chip
                .child(
                    ui_element::label(&attachment.label)
                        .text_color(text_primary)
                        .text_size(rem_to_px(0.75))
                        .text_ellipsis()
                        .whitespace_nowrap(),
                )
                // Compact remove glyph (not a full IconButton), matching the
                // FilterBuilder pill treatment.
                .child(
                    ui_element::icon("x")
                        .w(rem_to_px(0.75))
                        .h(rem_to_px(0.75))
                        .text_color(text_secondary),
                );

            // A disabled attachment cannot be removed, so it reads dimmed —
            // matching the web, where the remove IconButton carries the state.
            if attachment.is_disabled {
                chip = chip.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
            } else if let Some(handler) = &on_remove_attachment {
                let handler = std::sync::Arc::clone(handler);
                let id = attachment.id.clone();
                chip = chip.cursor_pointer().on_click(move |_event| handler(&id));
            }
            chips = chips.child(chip);
        }
        field = field.child(chips);
    }

    // Editor: the value, or the placeholder when empty. Height comes from the
    // clamped row count.
    let is_empty = spec.value.is_empty();
    field = field.child(
        ui_element::label(if is_empty { &spec.placeholder } else { &spec.value })
            .w_full()
            .min_h(line_height * spec.visible_rows() as f32)
            .text_color(if is_empty { placeholder } else { text_primary })
            .text_size(editor_font),
    );

    // ── Toolbar ───────────────────────────────────────────────────────────────
    let mut leading = ui_element::div()
        .flex_row()
        .items_center()
        .gap(gap)
        .grow()
        .min_w(0.0);
    for (index, child) in toolbar_children.into_iter().enumerate() {
        // Hairline dividers between leading children (contract §8).
        if index > 0 && spec.toolbar_dividers {
            leading = leading.child(
                ui_element::div()
                    .w(rem_to_px(0.0625))
                    .h(divider_height)
                    .flex_none()
                    .bg(divider),
            );
        }
        leading = leading.child(child);
    }

    let mut trailing = ui_element::div()
        .flex_row()
        .items_center()
        .justify_end()
        .gap(gap)
        .flex_none();

    if spec.show_context() {
        let mut meter = MeterSpec::new()
            .with_shape(MeterShape::Ring)
            .with_value(spec.context_used.unwrap_or(0.0))
            .with_max(spec.context_limit.unwrap_or(100.0))
            .with_size(effective_size)
            .with_aria_label(spec.context_aria_label());
        if let Some(high) = spec.context_high() {
            meter = meter.with_high(high);
        }
        trailing = trailing.child(js_meter(&meter, theme));
    }

    let mut action = ui_element::div()
        .w(action_box)
        .h(action_box)
        .flex_none()
        .items_center()
        .justify_center()
        .rounded(action_box / 2.0)
        .bg(action_fill)
        .child(
            ui_element::icon(spec.action_icon())
                .w(action_box * 0.5)
                .h(action_box * 0.5)
                .text_color(action_text),
        );
    if !spec.can_submit() {
        action = action.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    } else if let Some(handler) = &on_action {
        let handler = std::sync::Arc::clone(handler);
        action = action.cursor_pointer().on_click(move |_event| handler());
    }
    trailing = trailing.child(action);

    field = field.child(
        ui_element::div()
            .flex_row()
            .items_center()
            .gap(gap)
            .w_full()
            .min_w(0.0)
            .child(leading)
            .child(trailing),
    );

    let mut root = ui_element::div().flex_col().w_full().min_w(0.0).child(field);

    // ── Footer bar ────────────────────────────────────────────────────────────
    if !footer_children.is_empty() {
        let mut footer = ui_element::div()
            .flex_row()
            .items_center()
            .gap(gap)
            .ml(rem_to_px(1.5))
            .mr(rem_to_px(1.5))
            .pl(pad_x)
            .pr(pad_x)
            .pt(pad_y)
            .pb(pad_y)
            .rounded(chip_radius)
            .border_1()
            .border_color(divider)
            .bg(elevated);
        for child in footer_children {
            footer = footer.child(child);
        }
        root = root.child(footer);
    }

    if spec.is_disabled {
        root = root.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    crate::aria::with_aria_label(root, Some(spec.aria_label.as_str()))
}

/// Convenience for the common case: no toolbar or footer children.
pub fn js_agent_chat_input_bare(
    spec: &AgentChatInputSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    js_agent_chat_input(spec, theme, Vec::new(), Vec::new())
}

/// Editor font size for a resolved control size — exposed so preview specimens
/// can align sibling text without re-deriving the ladder.
pub fn editor_font_px(spec: &AgentChatInputSpec, size: ControlSize) -> f32 {
    rem_to_px(spec.editor_font_rem(size))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::{AgentChatAttachment, AgentChatStatus};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn empty_composer_shows_the_placeholder() {
        let tree = crate::render_probe::probe(
            &js_agent_chat_input_bare(&AgentChatInputSpec::new(), &theme()),
            480.0,
            200.0,
        );
        assert!(
            tree.has_text("Send a message"),
            "placeholder missing: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn value_replaces_the_placeholder() {
        let spec = AgentChatInputSpec::new().with_value("Ship it");
        let tree = crate::render_probe::probe(
            &js_agent_chat_input_bare(&spec, &theme()),
            480.0,
            200.0,
        );
        assert!(tree.has_text("Ship it"), "value missing: {:?}", tree.texts());
        assert!(
            !tree.has_text("Send a message"),
            "placeholder must not show with a value: {:?}",
            tree.texts()
        );
    }

    #[test]
    fn attachments_render_as_chips() {
        let spec = AgentChatInputSpec::new().with_attachments(vec![
            AgentChatAttachment::new("a1", "diagram.png").with_icon("image"),
        ]);
        let tree = crate::render_probe::probe(
            &js_agent_chat_input_bare(&spec, &theme()),
            480.0,
            240.0,
        );
        assert!(tree.has_text("diagram.png"), "chip label missing: {:?}", tree.texts());
    }

    #[test]
    fn busy_action_uses_the_danger_fill() {
        let idle = AgentChatInputSpec::new().with_value("hi");
        let busy = idle.clone().with_status(AgentChatStatus::Busy);
        // The spec drives the fill; assert the two states resolve differently.
        let th = theme();
        assert_ne!(
            resolve_color(&th, idle.action_fill_token()),
            resolve_color(&th, busy.action_fill_token())
        );
        // Both render without panicking at probe size.
        let tree = crate::render_probe::probe(&js_agent_chat_input_bare(&busy, &th), 480.0, 200.0);
        assert!(!tree.nodes.is_empty());
    }

    #[test]
    fn image_attachments_render_as_tiles_instead_of_labelled_chips() {
        let th = theme();
        let file = AgentChatInputSpec::new()
            .with_attachments(vec![AgentChatAttachment::new("a1", "notes.md").with_icon("file")]);
        let image = AgentChatInputSpec::new().with_attachments(vec![
            AgentChatAttachment::new("a1", "diagram.png").with_thumbnail("assets/diagram.png"),
        ]);

        let chip = crate::render_probe::probe(&js_agent_chat_input_bare(&file, &th), 480.0, 240.0);
        assert!(chip.has_text("notes.md"), "chip should label the file: {:?}", chip.texts());

        // A tile shows the picture, not the filename.
        let tile = crate::render_probe::probe(&js_agent_chat_input_bare(&image, &th), 480.0, 240.0);
        assert!(
            !tile.has_text("diagram.png"),
            "tile must not render the filename as text: {:?}",
            tile.texts()
        );
    }

    #[test]
    fn context_ring_only_renders_with_a_limit() {
        let th = theme();
        let without = js_agent_chat_input_bare(&AgentChatInputSpec::new(), &th);
        let with = js_agent_chat_input_bare(
            &AgentChatInputSpec::new().with_context(160_000.0, 200_000.0),
            &th,
        );
        let without_nodes = crate::render_probe::probe(&without, 480.0, 200.0).nodes.len();
        let with_nodes = crate::render_probe::probe(&with, 480.0, 200.0).nodes.len();
        assert!(
            with_nodes > without_nodes,
            "the context ring should add nodes: {without_nodes} vs {with_nodes}"
        );
    }

    #[test]
    fn toolbar_children_get_dividers_between_them() {
        let th = theme();
        let spec = AgentChatInputSpec::new();
        let two = js_agent_chat_input(
            &spec,
            &th,
            vec![ui_element::label("A"), ui_element::label("B")],
            Vec::new(),
        );
        let one = js_agent_chat_input(&spec, &th, vec![ui_element::label("A")], Vec::new());
        let two_nodes = crate::render_probe::probe(&two, 480.0, 200.0).nodes.len();
        let one_nodes = crate::render_probe::probe(&one, 480.0, 200.0).nodes.len();
        // Second child + one divider.
        assert_eq!(two_nodes, one_nodes + 2);
    }

    /// One control serves submit and stop, so one event serves both; the host
    /// derives the intent from the spec, exactly as the control picks its icon.
    #[test]
    fn the_action_control_reports_a_press() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = AgentChatInput::from_spec(
            AgentChatInputSpec::new().with_value("hi"),
            &theme(),
        )
        .on_action(move || { counter.fetch_add(1, Ordering::SeqCst); })
        .into_js_el();

        let spec = AgentChatInputSpec::new().with_value("hi");
        crate::element::click_probe::click_text(&el, 640.0, 300.0, spec.action_icon());

        assert_eq!(hits.load(Ordering::SeqCst), 1, "on_action fired exactly once");
    }

    /// Empty composer, nothing streaming: the control is inert and must not
    /// report a press it cannot honour.
    #[test]
    fn an_inert_action_control_ignores_clicks() {
        use crate::element::IntoJsEl;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);

        let el = AgentChatInput::from_spec(AgentChatInputSpec::new(), &theme())
            .on_action(move || { counter.fetch_add(1, Ordering::SeqCst); })
            .into_js_el();

        let spec = AgentChatInputSpec::new();
        crate::element::click_probe::click_text(&el, 640.0, 300.0, spec.action_icon());

        assert_eq!(hits.load(Ordering::SeqCst), 0, "an inert action control fired");
    }

    #[test]
    fn removing_an_attachment_reports_its_id() {
        use crate::element::IntoJsEl;
        use std::sync::{Arc, Mutex};

        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let ids = Arc::clone(&seen);

        let spec = AgentChatInputSpec::new().with_attachments(vec![
            AgentChatAttachment::new("a1", "design.fig"),
            AgentChatAttachment::new("a2", "notes.md"),
        ]);

        let el = AgentChatInput::from_spec(spec, &theme())
            .on_remove_attachment(move |id| ids.lock().unwrap().push(id.to_string()))
            .into_js_el();

        crate::element::click_probe::click_text(&el, 640.0, 300.0, "notes.md");

        assert_eq!(seen.lock().unwrap().as_slice(), ["a2"]);
    }

}
