//! TokenInput — committed token pills + a live draft input in a wrap row.
//!
//! Contract: `docs/contracts/components/token-input.md`
//! Ported from: `packages/jetstream/components/src/token_input.rs`.
//!
//! `on_remove` fires with the removed token's value (contract §8a: a host
//! removing by index would delete the wrong token whenever two removals arrive
//! between renders). `on_values_change` reports the whole committed list after
//! a draft commit, separator split, or empty-draft Backspace, so the host owns
//! the controlled value exactly as the Svelte component does. Entry rides the
//! shared token machinery in `poodle_headless::token`; the draft itself is a
//! borderless text control inside the field chrome, not a nested field.

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use poodle_headless::token::{merge_tokens, split_token_input, token_backspace_removes};
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, Node, NodeRole, TextChangeHandler,
};
use poodle_specs::{PillAppearance, PillSpec, PillTone, TextInputSpec, TokenInputSpec};

use crate::color::{mix_srgb, with_alpha, TRANSPARENT};
use crate::context::RenderContext;
use crate::pill::pill_with_handlers;
use crate::presentation::{
    control_space_x_rem, rem_to_px, token_input_gap_rem, token_input_pad_x_offset_rem,
    token_input_pad_y_offset_rem,
};
use crate::text_input::{text_input_with_handlers, TextInputHandlers};

/// Host callbacks for an editable TokenInput.
#[derive(Clone, Default)]
pub struct TokenInputHandlers {
    /// A token's remove affordance was activated; carries the token text.
    pub on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// The committed token list changed (entry commit, separator, Backspace).
    pub on_values_change: Option<Arc<dyn Fn(Vec<String>) + Send + Sync>>,
    /// A draft was rejected. The Rust spec has no `resolveToken` hook yet, so
    /// this only fires if a future reject path is added.
    pub on_token_reject: Option<Arc<dyn Fn(String) + Send + Sync>>,
}

/// Host-owned draft/caret state for one TokenInput instance. Committed values
/// stay in the spec (controlled), exactly as the web adapter's bindable
/// `values` do.
pub struct TokenInputLive {
    pub draft: String,
    pub selection: (usize, usize),
}

impl Default for TokenInputLive {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenInputLive {
    pub fn new() -> Self {
        Self {
            draft: String::new(),
            selection: (0, 0),
        }
    }
}

/// Stable element id for the live draft control of a TokenInput instance.
/// Distinct from the root id: the backend keys focus and editing state by
/// element id, so the field and its inner control must never share one.
pub fn token_input_draft_id(spec: &TokenInputSpec) -> String {
    if spec.id.is_empty() {
        "poodle-token-input-control".to_string()
    } else {
        format!("poodle-token-input-{}-control", spec.id)
    }
}

/// Token pills track the field size so they remain visually secondary.
fn pill_size(size: poodle_specs::ControlSize) -> poodle_specs::PillSize {
    use poodle_specs::{ControlSize, PillSize};
    match size {
        ControlSize::Xs => PillSize::Xs,
        ControlSize::Sm => PillSize::Sm,
        ControlSize::Md => PillSize::Md,
        ControlSize::Lg => PillSize::Lg,
        ControlSize::Xl => PillSize::Xl,
    }
}

fn normalize_token(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Normalize and merge a batch of raw committed parts. Returns `None` when
/// every part is empty after trimming.
fn add_tokens(raw: &[String], current: &[String], dedupe: bool) -> Option<Vec<String>> {
    let next: Vec<String> = raw
        .iter()
        .filter_map(|token| normalize_token(token))
        .collect();
    if next.is_empty() {
        return None;
    }
    Some(merge_tokens(current, &next, dedupe))
}

fn apply_values(handler: &Option<Arc<dyn Fn(Vec<String>) + Send + Sync>>, values: Vec<String>) {
    if let Some(handler) = handler {
        handler(values);
    }
}

fn report_reject(handler: &Option<Arc<dyn Fn(String) + Send + Sync>>, value: &str) {
    if let Some(handler) = handler {
        handler(value.to_string());
    }
}

/// The shared renderer. `live` present means the draft is wired for editing;
/// absent keeps the historical static draft (the shared Jetstream specimen).
#[expect(
    clippy::too_many_arguments,
    reason = "the builder keeps every host channel explicit, matching the other handler-backed controls"
)]
fn build(
    spec: &TokenInputSpec,
    ctx: &RenderContext<'_>,
    handlers: TokenInputHandlers,
    live: Option<&Arc<Mutex<TokenInputLive>>>,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);

    let surface = ctx.theme().resolve_color("color.background.surface");
    let border = ctx.theme().resolve_color("color.border.subtle");
    // Contract §8 field family: a surface-mix fill behind a 76%-mixed border.
    let fill = mix_srgb(surface, TRANSPARENT, 0.96);
    let border = with_alpha(border, border.3 * 0.76);
    let radius = ctx.theme().resolve_radius("radius.control");

    // Size-driven font + density-driven wrap gap (contract §8).
    let gap = rem_to_px(token_input_gap_rem(density));

    // Padding-block from control.y + per-size offset; padding-inline from
    // control.x (density) + per-size offset.
    let pad_y = (ctx.theme().resolve_space("space.control.y")
        + rem_to_px(token_input_pad_y_offset_rem(effective_size)))
    .max(0.0);
    let pad_x = (rem_to_px(control_space_x_rem(density))
        + rem_to_px(token_input_pad_x_offset_rem(effective_size)))
    .max(0.0);

    let can_edit = spec.can_edit();

    // Wrapping token row: committed pills (+ remove ×) then the draft.
    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
        s.descriptor.layout.spacing.gap = gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
    }

    for (index, token) in spec.values.iter().enumerate() {
        let remove = if can_edit {
            handlers.on_remove.as_ref().map(|handler| {
                let handler = Arc::clone(handler);
                let value = token.clone();
                Arc::new(move || handler(&value)) as Arc<dyn Fn() + Send + Sync>
            })
        } else {
            None
        };
        // Contract §8: the pill must wrap long values instead of overflowing.
        let mut token_pill = PillSpec::new()
            .with_label(token.clone())
            .with_tone(PillTone::Neutral)
            .with_appearance(PillAppearance::Subtle)
            .with_size(pill_size(effective_size))
            .with_removable(can_edit);
        token_pill.has_adaptive_width = true;
        // Instance-scoped so two tokens never share a remove control id, and
        // so the remove affordance is a real keyboard target (contract §7).
        let scope = if spec.id.is_empty() {
            format!("{index}:{token}")
        } else {
            format!("{}:{index}:{token}", spec.id)
        };
        row = row.child(pill_with_handlers(&token_pill, ctx, remove, None, &scope));
    }

    // Live draft control — borderless inside the field chrome, exactly as the
    // Svelte `.poodle-token-input__control`.
    let (draft_text, selection) = match live {
        Some(live) => {
            let state = live.lock().expect("token input live");
            (state.draft.clone(), state.selection)
        }
        None => (String::new(), (0, 0)),
    };
    let mut draft = TextInputSpec::new()
        .with_size(effective_size)
        .with_size_role(spec.size_role)
        .with_density(density)
        .with_disabled(spec.disabled)
        .with_read_only(spec.read_only);
    draft.value = Some(draft_text.clone());
    draft.selection_start = selection.0;
    draft.selection_end = selection.1;
    draft.is_focused = live.is_some();
    if !spec.id.is_empty() {
        draft.id = Some(spec.id.clone());
    }
    // Placeholder only shows when there are no committed tokens.
    if spec.values.is_empty() {
        if let Some(placeholder) = &spec.placeholder {
            draft = draft.with_placeholder(placeholder.clone());
        }
    }
    if let Some(max) = spec.max_length {
        draft = draft.with_max_length(max);
    }
    draft.name = spec.name.clone();
    draft.description_id = spec.described_by.clone();
    // The draft field needs a name whether or not the host supplied one: it is
    // the only thing here you can type into, and the committed tokens beside it
    // are not its label. Without this it is announced as an unnamed text input.
    draft = draft.with_aria_label(
        spec.aria_label
            .clone()
            .unwrap_or_else(|| "Add token".to_string()),
    );

    let mut text_handlers = TextInputHandlers::default();
    if let Some(live) = live {
        let values = spec.values.clone();
        let separators: BTreeSet<char> = poodle_headless::token::separator_chars(&spec.separators);
        let dedupe = spec.dedupe;
        let commit_on_blur = spec.commit_on_blur;
        let on_values = handlers.on_values_change.clone();
        let on_reject = handlers.on_token_reject.clone();

        // Entry/separator path: every accepted edit is re-read as a raw draft
        // string and split on the separator set.
        let change_live = Arc::clone(live);
        let change_values = values.clone();
        let change_on_values = on_values.clone();
        let change_on_reject = on_reject.clone();
        text_handlers.on_change = Some(Arc::new(move |next: &str| {
            if let Some(split) = split_token_input(next, &separators) {
                if let Some(next_values) = add_tokens(&split.committed, &change_values, dedupe) {
                    apply_values(&change_on_values, next_values);
                } else {
                    for raw in &split.committed {
                        if let Some(trimmed) = normalize_token(raw) {
                            report_reject(&change_on_reject, &trimmed);
                        }
                    }
                }
                let mut state = change_live.lock().expect("token input live");
                state.draft = split.remainder;
                let len = state.draft.chars().count();
                state.selection = (len, len);
            } else {
                let mut state = change_live.lock().expect("token input live");
                state.draft = next.to_string();
            }
        }) as TextChangeHandler);

        let select_live = Arc::clone(live);
        text_handlers.on_selection_change = Some(Arc::new(move |start, end| {
            select_live.lock().expect("token input live").selection = (start, end);
        }));

        // Commit the live draft. Shared by Enter, blur, and Tab (Tab arrives
        // as a focus change because the backend owns traversal).
        let commit_live = Arc::clone(live);
        let commit_values = values.clone();
        let commit_on_values = on_values.clone();
        let commit_on_reject = on_reject.clone();
        let commit: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
            let draft = {
                let state = commit_live.lock().expect("token input live");
                state.draft.clone()
            };
            let mut state = commit_live.lock().expect("token input live");
            match normalize_token(&draft) {
                Some(trimmed) => {
                    if let Some(next) =
                        add_tokens(std::slice::from_ref(&trimmed), &commit_values, dedupe)
                    {
                        apply_values(&commit_on_values, next);
                    } else {
                        report_reject(&commit_on_reject, &trimmed);
                    }
                }
                None => {}
            }
            state.draft.clear();
            state.selection = (0, 0);
        });
        text_handlers.on_submit = Some(Arc::clone(&commit));
        if commit_on_blur {
            let blur_live = Arc::clone(live);
            let blur_values = values.clone();
            let blur_on_values = on_values.clone();
            let blur_on_reject = on_reject.clone();
            text_handlers.on_focus_change = Some(Arc::new(move |focused| {
                if !focused {
                    // Re-run the same commit on blur.
                    let draft = {
                        let state = blur_live.lock().expect("token input live");
                        state.draft.clone()
                    };
                    let mut state = blur_live.lock().expect("token input live");
                    if let Some(trimmed) = normalize_token(&draft) {
                        if let Some(next) =
                            add_tokens(std::slice::from_ref(&trimmed), &blur_values, dedupe)
                        {
                            apply_values(&blur_on_values, next);
                        } else {
                            report_reject(&blur_on_reject, &trimmed);
                        }
                    }
                    state.draft.clear();
                    state.selection = (0, 0);
                }
            }));
        }
    }

    let mut draft_node = text_input_with_handlers(&draft, ctx, text_handlers);
    // Strip the nested field chrome: the token root owns the border, fill, and
    // padding (contract §8). The draft is only the entry line.
    {
        let s = &mut draft_node.style;
        s.descriptor.background = None;
        s.descriptor.border.width = 0.0;
        s.descriptor.layout.height = LayoutSizing::Fit;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.flex_basis = Some(rem_to_px(8.0));
        s.min_width = Some(rem_to_px(8.0));
        s.descriptor.corner_radii.top_left = 0.0;
        s.descriptor.corner_radii.top_right = 0.0;
        s.descriptor.corner_radii.bottom_right = 0.0;
        s.descriptor.corner_radii.bottom_left = 0.0;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = 0.0;
        pad.right = 0.0;
        pad.top = 0.0;
        pad.bottom = 0.0;
        s.hover = None;
        s.focus = None;
    }
    draft_node.id = Some(token_input_draft_id(spec));
    draft_node.a11y.role = Some(NodeRole::TextInput);

    // Backspace on an empty draft removes the last committed token, before the
    // text-edit model can swallow the key (contract §4 removal semantics).
    if let Some(live) = live {
        if let Some(original) = draft_node.interaction.on_edit_key.take() {
            let backspace_live = Arc::clone(live);
            let backspace_values = spec.values.clone();
            let backspace_on_values = handlers.on_values_change.clone();
            draft_node.interaction.on_edit_key = Some(Arc::new(move |key, mods| {
                if key == "backspace" {
                    let draft = {
                        let state = backspace_live.lock().expect("token input live");
                        state.draft.clone()
                    };
                    if token_backspace_removes(&draft, backspace_values.len()) {
                        let next = backspace_values[..backspace_values.len() - 1].to_vec();
                        apply_values(&backspace_on_values, next);
                        return;
                    }
                }
                original(key, mods);
            }));
        }
    }

    // The draft grows to fill trailing space.
    let mut draft_slot = Node::container();
    {
        let s = &mut draft_slot.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.flex_basis = Some(rem_to_px(8.0));
        s.min_width = Some(rem_to_px(6.0));
    }
    row = row.child(draft_slot.child(draft_node));

    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.fill_width = true;
        s.min_width = Some(0.0);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = border;
        s.descriptor.background = Some(fill);
    }
    let mut el = el.child(row);

    if spec.disabled {
        el.style.descriptor.opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
    }
    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el.id = Some(if spec.id.is_empty() {
        "poodle-token-input".to_string()
    } else {
        format!("poodle-token-input-{}", spec.id)
    });
    el
}

pub fn token_input(
    spec: &TokenInputSpec,
    ctx: &RenderContext<'_>,
    on_remove: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    build(
        spec,
        ctx,
        TokenInputHandlers {
            on_remove,
            ..TokenInputHandlers::default()
        },
        None,
    )
}

/// Render a TokenInput whose draft is actually editable. Committed values stay
/// controlled through `on_values_change`; the host holds draft and caret in
/// `live` between rebuilds.
pub fn token_input_with_handlers(
    spec: &TokenInputSpec,
    ctx: &RenderContext<'_>,
    handlers: TokenInputHandlers,
    live: &Arc<Mutex<TokenInputLive>>,
) -> Node {
    build(spec, ctx, handlers, Some(live))
}
