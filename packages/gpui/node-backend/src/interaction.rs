//! GPUI listener, editing, gesture, and drop routing for node interactions.

use super::*;

struct ContinuousValueSession {
    owner_id: String,
    last_window: (f32, f32),
    last_norm: (f32, f32),
    handler: Arc<dyn Fn(&NodeContinuousValueEvent) + Send + Sync>,
    painted: bool,
}

thread_local! {
    static CONTINUOUS_VALUE: RefCell<Option<ContinuousValueSession>> = const { RefCell::new(None) };
}

pub(crate) fn reset_continuous_value_session() {
    CONTINUOUS_VALUE.with(|slot| *slot.borrow_mut() = None);
}

pub(crate) fn prepare_continuous_value_frame() {
    CONTINUOUS_VALUE.with(|slot| {
        if let Some(session) = slot.borrow_mut().as_mut() {
            session.painted = false;
        }
    });
}

/// If a continuous-value node was not rebuilt this frame, the host is gone:
/// emit cancel exactly once. Nested refreshes that still paint the owner
/// keep the gesture open.
pub(crate) fn sweep_lost_continuous_host() {
    let pending = CONTINUOUS_VALUE.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.as_ref().is_some_and(|session| session.painted) {
            return None;
        }
        slot.take().map(|session| {
            (
                session.handler,
                NodeContinuousValueEvent {
                    phase: ContinuousValuePhase::Cancel,
                    x: session.last_norm.0,
                    y: session.last_norm.1,
                    delta_x: 0.0,
                    delta_y: 0.0,
                    modifiers: NodeModifiers::default(),
                },
            )
        })
    });
    if let Some((handler, event)) = pending {
        handler(&event);
    }
}

fn retain_continuous_handler(
    owner_id: &str,
    handler: Arc<dyn Fn(&NodeContinuousValueEvent) + Send + Sync>,
) {
    CONTINUOUS_VALUE.with(|slot| {
        if let Some(session) = slot.borrow_mut().as_mut() {
            if session.owner_id == owner_id {
                session.handler = handler;
                session.painted = true;
            }
        }
    });
}

fn continuous_is_open() -> bool {
    CONTINUOUS_VALUE.with(|slot| slot.borrow().is_some())
}

fn begin_continuous(
    owner_id: String,
    window: (f32, f32),
    norm: (f32, f32),
    modifiers: NodeModifiers,
    handler: Arc<dyn Fn(&NodeContinuousValueEvent) + Send + Sync>,
) {
    if continuous_is_open() {
        return;
    }
    handler(&NodeContinuousValueEvent {
        phase: ContinuousValuePhase::Press,
        x: norm.0,
        y: norm.1,
        delta_x: 0.0,
        delta_y: 0.0,
        modifiers,
    });
    CONTINUOUS_VALUE.with(|slot| {
        *slot.borrow_mut() = Some(ContinuousValueSession {
            owner_id,
            last_window: window,
            last_norm: norm,
            handler,
            painted: true,
        });
    });
}

fn move_continuous(
    window: (f32, f32),
    bounds: gpui::Bounds<gpui::Pixels>,
    modifiers: NodeModifiers,
) {
    let pending = CONTINUOUS_VALUE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let session = slot.as_mut()?;
        let norm = continuous_local(window, bounds);
        let event = NodeContinuousValueEvent {
            phase: ContinuousValuePhase::Move,
            x: norm.0,
            y: norm.1,
            delta_x: window.0 - session.last_window.0,
            delta_y: session.last_window.1 - window.1,
            modifiers,
        };
        session.last_window = window;
        session.last_norm = norm;
        Some((Arc::clone(&session.handler), event))
    });
    if let Some((handler, event)) = pending {
        handler(&event);
    }
}

fn release_continuous(
    owner_id: &str,
    window: (f32, f32),
    bounds: gpui::Bounds<gpui::Pixels>,
    modifiers: NodeModifiers,
) {
    let pending = CONTINUOUS_VALUE.with(|slot| {
        let mut slot = slot.borrow_mut();
        if !slot
            .as_ref()
            .is_some_and(|session| session.owner_id == owner_id)
        {
            return None;
        }
        slot.take().map(|session| {
            let norm = continuous_local(window, bounds);
            (
                session.handler,
                NodeContinuousValueEvent {
                    phase: ContinuousValuePhase::Release,
                    x: norm.0,
                    y: norm.1,
                    delta_x: window.0 - session.last_window.0,
                    delta_y: session.last_window.1 - window.1,
                    modifiers,
                },
            )
        })
    });
    if let Some((handler, event)) = pending {
        handler(&event);
    }
}

pub(super) fn apply_listeners(mut el: Stateful<Div>, node: &Node, id: &str) -> Stateful<Div> {
    if node.interaction.request_focus {
        super::layers::request_focus(id);
    }
    // A pointer press that lands inside a dismissable layer must still run
    // the shared outside-dismissal check when the layer's own subtree does
    // not contain the position. gpui 0.2.2 does not route pointer events
    // over deferred overlay content to window-level (root) listeners, so the
    // window host alone cannot dismiss a nested layer from a press inside
    // the enclosing overlay. Every layer member therefore observes presses
    // in the capture phase, where activation and click-synthesis handlers
    // cannot suppress the dismissal. Dismissed records are consumed by
    // `dismiss_layers_at`, so the extra listeners never dismiss twice.
    if node.interaction.dismiss_layer.is_some() {
        el = el.capture_any_mouse_down(|event, _window, cx| {
            if event.button == MouseButton::Left {
                super::layers::dismiss_layers_at(event.position, cx);
            }
        });
    }
    // A disabled control is not focusable: gpui would otherwise take focus on
    // pointer-down, and a browser never focuses a disabled control. The focus
    // tracking canvas below still attaches (the patch may exist), so blur and
    // focus remain observable through the registry.
    if !node.interaction.disabled {
        if let Some(tab_index) = node.a11y.tab_index {
            // GPUI uses a non-negative index for ordering and a separate flag
            // for whether the element participates in sequential traversal.
            // Preserve DOM-style roving tabindex: -1 stays programmatically
            // focusable.
            el = el
                .tab_index(tab_index.max(0) as isize)
                .tab_stop(tab_index >= 0);
        } else if node.interaction.focusable {
            // No declared index means the DOM default: a focusable control is
            // a tab stop at index 0. `focusable()` alone leaves gpui's
            // auto-created handle with `tab_stop` off, which took every field,
            // slot row and segment out of sequential traversal — the
            // vocabulary says `focusable` *participates* in it.
            el = el.focusable().tab_index(0);
        }
    }

    // Real focus, observed both ways. gpui auto-creates a focus handle and
    // keeps it in element state it never hands back, so we own one instead:
    // created lazily in the paint pass (the first place with an `App`) and
    // attached from the next build onward. That makes *blur* observable, which
    // is what a latched-on-click flag could never do.
    if tracks_focus(node) {
        let id = id.to_owned();
        if let Some(handle) = focus_handle_for(&id) {
            el = el.track_focus(&handle);
        }
        let on_focus_change = node.interaction.on_focus_change.clone();
        let tab_index = node.a11y.tab_index;
        let node_focusable = node.interaction.focusable;
        let painted_id = input_text::painted_key(node, &id);
        let focus_tooltip = node.tooltip.clone();
        let focus_disabled = node.interaction.disabled;
        el = el.child(
            gpui::canvas(
                move |_bounds, window, cx| {
                    let mut created = false;
                    let handle = FOCUS_HANDLES.with(|handles| {
                        let mut handles = handles.borrow_mut();
                        let entry = handles.entry(id.clone()).or_insert_with(|| {
                            created = true;
                            cx.focus_handle()
                        });
                        // Re-apply the declared sequential-focus flags on
                        // every frame, not only at creation: a roving
                        // component changes `a11y.tab_index` over time, and a
                        // flag frozen at first paint would make the initially
                        // selected item the permanent tab stop. gpui
                        // default-creates handles with tab_stop off, and once
                        // `track_focus` attaches, the handle's flags — not
                        // the element refinement's — decide traversal. So the
                        // undeclared-index case has to repeat the DOM default
                        // here as well: focusable means tab stop.
                        let updated = entry
                            .clone()
                            .tab_index(tab_index.unwrap_or(0).max(0) as isize)
                            .tab_stop(tab_index.map_or(node_focusable, |index| index >= 0));
                        *entry = updated.clone();
                        updated
                    });
                    if created {
                        // The element that wants this handle was already built
                        // without it — nothing else would repaint, so the
                        // handle would sit unattached and never see a focus.
                        cx.refresh_windows();
                    }
                    // A queued focus request (a machine focus effect from the
                    // overlay host) is applied here, in the paint pass, once
                    // the target element exists and has a handle.
                    if super::layers::take_focus_request(&id) {
                        if !handle.is_focused(window) {
                            handle.focus(window);
                            cx.refresh_windows();
                        }
                    }
                    let now = handle.is_focused(window);
                    let changed = FOCUS_STATES.with(|states| {
                        let mut states = states.borrow_mut();
                        // The first observation of a node that is *not*
                        // focused is not a change: it never held focus, so it
                        // did not lose it. Reporting it as a blur made the
                        // first painted frame indistinguishable from focus
                        // leaving — which a field that commits on blur cannot
                        // survive.
                        match states.insert(id.clone(), now) {
                            Some(previous) => previous != now,
                            None => now,
                        }
                    });
                    if changed {
                        // The value node draws the caret but the *root* holds
                        // focus, and the platform input handler must be
                        // registered against the handle that is actually
                        // focused. Recording it here is what lets the two meet.
                        FOCUSED_FIELD.with(|f| {
                            let mut f = f.borrow_mut();
                            if now {
                                *f = Some(id.clone());
                            } else if f.as_deref() == Some(id.as_str()) {
                                *f = None;
                            }
                        });
                        if !now {
                            // A stale measured line must not answer clicks on
                            // whatever takes this id next. The transient state
                            // hangs off the node that painted the value, which
                            // is the root itself for a childless input and a
                            // derived child for a composite field; clear both,
                            // so neither shape can leave the other's entries
                            // behind.
                            input_text::forget(&id);
                            if painted_id != id {
                                input_text::forget(&painted_id);
                            }
                        }
                        if let Some(handler) = &on_focus_change {
                            handler(now);
                        }
                        if let Some(tooltip_text) =
                            focus_tooltip.as_deref().filter(|text| !text.is_empty())
                        {
                            if !focus_disabled {
                                if now {
                                    crate::tooltip::on_focus_enter(window, cx, &id, tooltip_text);
                                } else {
                                    crate::tooltip::on_focus_departure(window, cx, &id);
                                }
                            }
                        }
                        cx.refresh_windows();
                    }
                },
                |_, _, _, _| {},
            )
            .absolute()
            .size_full(),
        );
    }
    // The declared focus ring, painted while — and only while — the node's
    // real focus handle holds focus. A canvas child, not a style refinement:
    // the ring is out-of-flow (layout never sees it), it must preserve the
    // resting border rather than replace it, and hover/active patches — gpui
    // refines hover after focus — must not overwrite it. The canvas is
    // anchored at the element's top-left inset, so its bounds ARE the padding
    // box (an unanchored absolute child would sit at the justify-static
    // position instead); the border box is one border-width outward per side,
    // and the ring's outer edge sits `offset + width` beyond that, exactly
    // CSS `outline` + `outline-offset` (a negative offset insets the ring).
    if let Some(ring) = node.style.focus_ring {
        let ring_id = id.to_owned();
        let ring_within = node.style.focus_ring_within;
        let border = &node.style.descriptor.border;
        let border_left = node.style.border_left_width.unwrap_or(border.width);
        let border_right = node.style.border_right_width.unwrap_or(border.width);
        let border_top = node.style.border_top_width.unwrap_or(border.width);
        let border_bottom = node.style.border_bottom_width.unwrap_or(border.width);
        let radii = node.style.descriptor.corner_radii;
        el = el.child(
            gpui::canvas(
                move |_, _, _| {},
                move |bounds, (), window, cx| {
                    let focused = super::focus_handle_for(&ring_id).is_some_and(|handle| {
                        if ring_within {
                            handle.contains_focused(window, cx)
                        } else {
                            handle.is_focused(window)
                        }
                    });
                    if !focused {
                        super::clear_painted_ring(&ring_id);
                        return;
                    }
                    let expand = ring.offset + ring.width;
                    let x = f32::from(bounds.origin.x) - border_left - expand;
                    let y = f32::from(bounds.origin.y) - border_top - expand;
                    let width =
                        f32::from(bounds.size.width) + border_left + border_right + 2.0 * expand;
                    let height =
                        f32::from(bounds.size.height) + border_top + border_bottom + 2.0 * expand;
                    if width <= 0.0 || height <= 0.0 {
                        super::clear_painted_ring(&ring_id);
                        return;
                    }
                    // The ring is concentric with the element: each corner
                    // radius grows by the same expansion, so the inner edge
                    // parallels the border box instead of rounding harder.
                    let corner = |r: f32| px((r + expand).max(0.0));
                    window.paint_quad(
                        gpui::outline(
                            gpui::Bounds {
                                origin: point(px(x), px(y)),
                                size: size(px(width), px(height)),
                            },
                            super::color(ring.color),
                            gpui::BorderStyle::default(),
                        )
                        .corner_radii(gpui::Corners {
                            top_left: corner(radii.top_left),
                            top_right: corner(radii.top_right),
                            bottom_right: corner(radii.bottom_right),
                            bottom_left: corner(radii.bottom_left),
                        })
                        .border_widths(px(ring.width)),
                    );
                    super::record_painted_ring(
                        &ring_id,
                        super::PaintedRing {
                            ring,
                            bounds: [x, y, width, height],
                        },
                    );
                },
            )
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .size_full(),
        );
    }
    if !node.interaction.disabled {
        if let Some(patch) = &node.style.active {
            let patch = *patch;
            el = el.active(move |s| apply_patch(s, patch));
            record_probe_channel("surface.state-patches.active");
        }
        // gpui's focus styling needs the element focusable, which the
        // `focusable` flag above has already arranged.
        if node.interaction.focusable {
            if let Some(patch) = &node.style.focus {
                let patch = *patch;
                el = el.focus(move |s| apply_patch(s, patch));
                record_probe_channel("surface.state-patches.focus");
            }
        }
    }
    // Overlay-layer membership: record this element's rendered bounds into
    // the layer registry so outside-interaction containment and relative
    // logical-bounds observation read real geometry.
    //
    // Bounds key off the resolved element identity (`runtime_id` or `id`),
    // not only `Node.id`. Deferred overlay rows often carry a stable
    // runtime_id without a separate `id`; skipping them made pointer
    // hit-testing fall back to a mount-box guess.
    let layer_id = node
        .interaction
        .dismiss_layer
        .clone()
        .or_else(crate::current_dismiss_layer);
    let in_dismiss_layer = layer_id.is_some();
    if let Some(layer) = layer_id {
        let element_id = id.to_owned();
        // Overlay surfaces pin all four insets instead of `size_full`.
        // Percentage height on an auto-sized deferred box collapses Taffy's
        // used height to padding; inset-0 fills the laid-out padding box
        // without contributing to that height.
        let canvas = gpui::canvas(
            move |bounds, _window, _cx| {
                super::layers::record_bounds(&element_id, &layer, bounds);
            },
            |_, _, _, _| {},
        )
        .absolute()
        .top(px(0.0))
        .left(px(0.0));
        el = el.child(if node.style.overlay {
            canvas.right(px(0.0)).bottom(px(0.0))
        } else {
            canvas.size_full()
        });
    } else if !id.is_empty() {
        let element_id = id.to_owned();
        el = el.child(
            gpui::canvas(
                move |bounds, _window, _cx| {
                    super::layers::record_element_bounds(&element_id, bounds);
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .size_full(),
        );
    }
    if let Some(text) = node
        .tooltip
        .as_deref()
        .filter(|text| !text.is_empty())
        .map(str::to_string)
    {
        record_probe_channel("tooltip.projection.received");
        let target_id = id.to_owned();
        let is_disabled = node.interaction.disabled;
        let tooltip_text = text.clone();
        el = el.child(
            gpui::canvas(
                move |bounds, window, _cx| {
                    crate::tooltip::record_tooltip_target_paint(
                        window,
                        &target_id,
                        &tooltip_text,
                        bounds,
                        is_disabled,
                    );
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .size_full(),
        );

        if !is_disabled {
            let enter_id = id.to_owned();
            let leave_id = id.to_owned();
            let hover_text = text.clone();
            el = el.on_hover(move |hovered: &bool, window: &mut Window, cx: &mut App| {
                if *hovered {
                    crate::tooltip::on_pointer_enter(window, cx, &enter_id, &hover_text);
                } else {
                    crate::tooltip::on_pointer_leave(window, cx, &leave_id);
                }
            });
        }
    }
    // Non-focusable overlay members (option rows, disabled rows) must stop
    // the window host from taking focus on press. That blur otherwise runs
    // the open-state Close handler and unmounts the layer before click.
    if !node.interaction.focusable && in_dismiss_layer {
        el = el.on_mouse_down(
            MouseButton::Left,
            move |_event: &MouseDownEvent, window, _cx| {
                window.prevent_default();
            },
        );
    }
    if node.interaction.disabled {
        record_probe_channel("semantic.disabled.blocked");
        return el;
    }
    if let Some(handler) = &node.interaction.on_activate {
        let click = handler.clone();
        el = el.on_click(
            move |_event: &ClickEvent, _window: &mut Window, cx: &mut App| {
                click();
                // Node handlers carry no context, so they cannot notify the
                // entity that owns the state they mutated; a repaint is what
                // lets the host observe the mutation on the next frame.
                cx.stop_propagation();
                cx.refresh_windows();
            },
        );
        record_probe_channel("activate.listener");
        // Enter/Space activation is gpui's own: a focused div with a click
        // listener synthesizes KeyUp → click (div.rs). Binding `on_key_down`
        // here as well double-fires — the conformance corpus caught it (two
        // press events from one Enter). The click binding above is the one
        // activation path, for pointer and keyboard alike.
    }
    // Pointer selection is keyed on the channel, not on the node's kind: a
    // field's value is a *text* node carrying a caret, so that the field root
    // stays the only input in the accessibility tree.
    // Click to place the caret, drag to select. Both are the same
    // question — "which character is under this x?" — answered from the
    // last painted line, because only a painted line has been measured.
    if let Some(select) = node.interaction.on_select_range.clone() {
        let id = id.to_owned();
        let down_id = id.clone();
        let down_select = select.clone();
        el = el.on_mouse_down(
            MouseButton::Left,
            move |event: &MouseDownEvent, _window, cx| {
                let Some(index) = input_text::char_index_for_position(&down_id, event.position)
                else {
                    return;
                };
                // Click count is the backend's to know; what a "word" is
                // is not, so the granularity is named rather than resolved.
                let granularity = match event.click_count {
                    0 | 1 => SelectGranularity::Character,
                    2 => SelectGranularity::Word,
                    _ => SelectGranularity::Line,
                };
                if granularity != SelectGranularity::Character {
                    down_select(index, index, granularity);
                    cx.refresh_windows();
                    return;
                }
                if event.modifiers.shift {
                    // Shift-click extends from wherever the drag anchor is.
                    if let Some(anchor) = input_text::drag_anchor(&down_id) {
                        down_select(anchor, index, granularity);
                        cx.refresh_windows();
                        return;
                    }
                }
                input_text::begin_select(&down_id, index);
                down_select(index, index, granularity);
                cx.refresh_windows();
            },
        );

        let move_id = id.clone();
        el = el.on_mouse_move(move |event: &MouseMoveEvent, _window, cx| {
            let Some(anchor) = input_text::drag_anchor(&move_id) else {
                return;
            };
            if !event.dragging() {
                return;
            }
            let Some(index) = input_text::char_index_for_position(&move_id, event.position) else {
                return;
            };
            if index != anchor {
                select(anchor, index, SelectGranularity::Character);
                cx.refresh_windows();
            }
        });

        // `_out` as well: a drag that ends past the field's edge must still
        // end, or the next unrelated move keeps extending the selection.
        el = el.on_mouse_up(
            MouseButton::Left,
            move |_event: &MouseUpEvent, _window, _cx| {
                input_text::end_select();
            },
        );
        el = el.on_mouse_up_out(
            MouseButton::Left,
            move |_event: &MouseUpEvent, _window, _cx| {
                input_text::end_select();
            },
        );
    }

    // Keyed on the channels, not on `NodeKind::Input`. CodeInput puts its key
    // handler on the slot *row* and DurationInput on each segment — both plain
    // containers — so gating this on the input kind meant neither ever received
    // a keystroke, while their component-level tests (which call the handler
    // directly) passed.
    if node.interaction.on_edit_key.is_some()
        || node.interaction.on_edit_insert.is_some()
        || node.interaction.on_submit.is_some()
        || node.interaction.on_cancel.is_some()
    {
        let edit_key = node.interaction.on_edit_key.clone();

        let submit = node.interaction.on_submit.clone();
        let cancel = node.interaction.on_cancel.clone();
        let overlay_owns_escape = in_dismiss_layer;
        // Clipboard is the backend's: the text comes from outside the tree, and
        // `App` reaches it directly. (IME still needs an `EntityInputHandler`,
        // which a `&Node -> AnyElement` backend has no entity to hang on.)
        let insert = node.interaction.on_edit_insert.clone();
        // Undo needs the id of the node that actually paints the value,
        // because that is the only one that sees an edit's result; the keys
        // arrive at the focusable root, which may be that same node or an
        // ancestor of it.
        let value_id = input_text::painted_key(node, id);
        let text_change = node.interaction.on_text_change.clone();
        let select_range = node.interaction.on_select_range.clone();
        let selection_text = node.caret.map(|c| c.selection).and_then(|(a, b)| {
            let NodeKind::Input { value, .. } = &node.kind else {
                return None;
            };
            let (start, end) = if a <= b { (a, b) } else { (b, a) };
            let text: String = value.chars().skip(start).take(end - start).collect();
            (!text.is_empty()).then_some(text)
        });
        if edit_key.is_some() || submit.is_some() || cancel.is_some() || insert.is_some() {
            el = el.on_key_down(move |event: &KeyDownEvent, _window, cx| {
                let key = event.keystroke.key.as_str();
                let accel = event.keystroke.modifiers.platform;
                if accel && key == "z" {
                    // Undo restores a whole snapshot rather than replaying an
                    // edit, so it reports the value and the caret together.
                    let restored = if event.keystroke.modifiers.shift {
                        input_text::redo(&value_id)
                    } else {
                        input_text::undo(&value_id)
                    };
                    if let Some(snapshot) = restored {
                        if let Some(change) = &text_change {
                            change(&snapshot.value);
                        }
                        if let Some(select) = &select_range {
                            select(
                                snapshot.state.anchor,
                                snapshot.state.head,
                                SelectGranularity::Character,
                            );
                        }
                        cx.refresh_windows();
                    }
                    return;
                }
                if accel && matches!(key, "c" | "x" | "v") {
                    match key {
                        "c" | "x" => {
                            // Copying an empty selection must not clear what is
                            // already on the clipboard.
                            if let Some(text) = &selection_text {
                                cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                                    text.clone(),
                                ));
                                if key == "x" {
                                    if let Some(insert) = &insert {
                                        insert("");
                                    }
                                }
                                cx.refresh_windows();
                            }
                        }
                        _ => {
                            if let Some(text) =
                                cx.read_from_clipboard().and_then(|item| item.text())
                            {
                                if let Some(insert) = &insert {
                                    // A single-line field takes a multi-line
                                    // paste as one line, like `<input>` does.
                                    insert(&text.replace('\n', " "));
                                    cx.refresh_windows();
                                }
                            }
                        }
                    }
                    return;
                }
                if key == "tab" {
                    // Traversal, never submission and never an edit. The
                    // window host turns it into gpui's own `focus_next`/
                    // `focus_prev`; a field that wants to commit when Tab
                    // leaves it hears the blur that follows. Returning here
                    // only ends this listener — the keystroke still bubbles
                    // to the host.
                    return;
                }
                if key == "enter" {
                    if let Some(handler) = &submit {
                        handler();
                        cx.refresh_windows();
                        return;
                    }
                } else if key == "escape" {
                    // Overlay members already register on the dismiss stack.
                    // Invoking on_cancel here would Close the focused instance
                    // and then let the window host dismiss_innermost — two
                    // different layers on one keystroke.
                    if overlay_owns_escape {
                        return;
                    }
                    if let Some(handler) = &cancel {
                        handler();
                        cx.refresh_windows();
                        return;
                    }
                }

                // Editing itself belongs to the component: it owns the caret
                // through its spec, so it decides what the key means. The
                // backend only reports which key arrived.
                if let Some(edit) = &edit_key {
                    edit(key, node_modifiers(&event.keystroke.modifiers));
                    cx.refresh_windows();
                }
            });
        }
    }
    if let Some(handler) = &node.interaction.on_drag {
        // `on_drag_move`, NOT `on_mouse_move`: the latter only fires while the
        // pointer is over this element's hitbox, so a drag detached the moment
        // it left the control — a slider stayed with the mouse for a few pixels
        // and then stopped. `on_drag_move` keeps receiving moves anywhere in the
        // window as long as the gesture started here, which is what a drag is.
        //
        // gpui 0.2.2 has no mouse-up listener on this surface, so
        // NodeDragPhase::End is still never emitted. Deltas remain per-frame
        // from the last reported position — the vocabulary's contract.
        // Registering `on_drag` makes gpui swallow this element's mouse-down, so
        // Start cannot come from a down listener: the first move of a gesture
        // emits it, then reports deltas.
        let last: Rc<RefCell<Option<(f32, f32)>>> = Rc::new(RefCell::new(None));
        let last_move = last.clone();
        let mv = handler.clone();
        let gesture_id = next_gesture_id();
        el = el
            .on_drag(NodeGestureDrag(gesture_id.clone()), |_, _, _window, cx| {
                cx.new(|_| EmptyDragPreview)
            })
            .on_drag_move::<NodeGestureDrag>(move |event, _window, cx| {
                if event.drag(cx).0 != gesture_id {
                    return;
                }
                let pos: (f32, f32) =
                    (event.event.position.x.into(), event.event.position.y.into());
                let mut last = last_move.borrow_mut();
                match *last {
                    None => mv(&NodeDragEvent {
                        phase: NodeDragPhase::Start,
                        delta_x: 0.0,
                        delta_y: 0.0,
                    }),
                    Some(prev) => mv(&NodeDragEvent {
                        phase: NodeDragPhase::Move,
                        delta_x: pos.0 - prev.0,
                        delta_y: pos.1 - prev.1,
                    }),
                }
                cx.refresh_windows();
                *last = Some(pos);
            });
    }

    if let Some(handler) = &node.interaction.on_scrub {
        // Scrub reports where the pointer sits along this element's declared
        // axis as a fraction of that axis. Mouse down/up carry the explicit gesture
        // boundaries. `on_drag_move` keeps real pointer capture after the
        // pointer leaves the element; the mouse-move listener is the fallback
        // for synthetic window events that do not arm gpui's drag payload.
        //
        // Do not latch "scrubbing" in a Cell across this render: Press calls
        // refresh_windows, which rebuilds listeners and would clear the latch
        // before Release. Gate Drag with MouseMoveEvent::dragging() instead.
        //
        // A mouse-down event carries no bounds, so a zero-cost canvas child
        // records the track rectangle at paint time.
        let track: Rc<RefCell<Option<gpui::Bounds<gpui::Pixels>>>> = Rc::new(RefCell::new(None));
        let track_paint = track.clone();
        let track_down = track.clone();
        let track_move = track.clone();
        let track_up = track.clone();
        let press = handler.clone();
        let mv = handler.clone();
        let captured_mv = handler.clone();
        let release = handler.clone();
        let axis = node.interaction.scrub_axis;
        let gesture_id = next_gesture_id();
        el = el
            .child(
                gpui::canvas(
                    move |bounds, _window, _cx| {
                        *track_paint.borrow_mut() = Some(bounds);
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .on_mouse_down(
                MouseButton::Left,
                move |event: &MouseDownEvent, _window, cx| {
                    if let Some(bounds) = *track_down.borrow() {
                        press(
                            scrub_fraction(event.position, bounds, axis),
                            ScrubPhase::Press,
                        );
                        cx.refresh_windows();
                    }
                },
            )
            .on_mouse_move(move |event: &MouseMoveEvent, _window, cx| {
                if !event.dragging() || cx.has_active_drag() {
                    return;
                }
                if let Some(bounds) = *track_move.borrow() {
                    mv(
                        scrub_fraction(event.position, bounds, axis),
                        ScrubPhase::Drag,
                    );
                    cx.refresh_windows();
                }
            })
            .on_drag(NodeGestureDrag(gesture_id.clone()), |_, _, _window, cx| {
                cx.new(|_| EmptyDragPreview)
            })
            .on_drag_move::<NodeGestureDrag>(move |event, _window, cx| {
                if event.drag(cx).0 != gesture_id {
                    return;
                }
                captured_mv(
                    scrub_fraction(event.event.position, event.bounds, axis),
                    ScrubPhase::Drag,
                );
                cx.refresh_windows();
            })
            .on_mouse_up(
                MouseButton::Left,
                move |event: &MouseUpEvent, _window, cx| {
                    if let Some(bounds) = *track_up.borrow() {
                        release(
                            scrub_fraction(event.position, bounds, axis),
                            ScrubPhase::Release,
                        );
                        cx.refresh_windows();
                    }
                },
            );
    }
    el = apply_continuous_listeners(el, node, id);
    el = apply_selection_listeners(el, node);
    crate::drag::apply_drag_listeners(el, node, id)
}

/// Captured continuous-value, wheel, and double-activation routes.
///
/// GPUI 0.2.2 supplies captured moves through `on_drag_move`, release outside
/// the node through `on_mouse_up_out`, and wheel through `on_scroll_wheel`.
/// Unique lifetime lives in a thread-local session so a rebuild cannot admit
/// a second press or drop a terminal.
fn apply_continuous_listeners(mut el: Stateful<Div>, node: &Node, id: &str) -> Stateful<Div> {
    let continuous = node.interaction.on_continuous_value.clone();
    let wheel = node.interaction.on_wheel.clone();
    let double_activate = node.interaction.on_double_activate.clone();
    if continuous.is_none() && wheel.is_none() && double_activate.is_none() {
        return el;
    }

    let disabled = node.interaction.disabled;
    if let Some(handler) = continuous.clone() {
        retain_continuous_handler(id, handler);
    }

    let track: Rc<RefCell<Option<gpui::Bounds<gpui::Pixels>>>> = Rc::new(RefCell::new(None));
    if continuous.is_some() {
        let track_paint = track.clone();
        el = el.child(
            gpui::canvas(
                move |bounds, _window, _cx| {
                    *track_paint.borrow_mut() = Some(bounds);
                },
                |_, _, _, _| {},
            )
            .absolute()
            .size_full(),
        );
    }

    if continuous.is_some() || double_activate.is_some() {
        let press = continuous.clone();
        let double = double_activate;
        let track_down = track.clone();
        let owner = id.to_owned();
        el = el.on_mouse_down(
            MouseButton::Left,
            move |event: &MouseDownEvent, _window, cx| {
                if event.click_count >= 2 {
                    if let Some(double) = &double {
                        if !disabled {
                            double(node_modifiers(&event.modifiers));
                            cx.refresh_windows();
                        }
                    }
                    return;
                }
                let Some(handler) = &press else {
                    return;
                };
                if disabled {
                    return;
                }
                let Some(bounds) = *track_down.borrow() else {
                    return;
                };
                let window = window_point(event.position);
                begin_continuous(
                    owner.clone(),
                    window,
                    continuous_local(window, bounds),
                    node_modifiers(&event.modifiers),
                    Arc::clone(handler),
                );
                cx.refresh_windows();
            },
        );
    }

    if continuous.is_some() {
        let track_move = track.clone();
        let track_captured = track.clone();
        let track_up = track.clone();
        let track_up_out = track.clone();
        let owner_up = id.to_owned();
        let owner_out = id.to_owned();
        let gesture_id = next_gesture_id();
        el = el
            .on_mouse_move(move |event: &MouseMoveEvent, _window, cx| {
                if !event.dragging() || cx.has_active_drag() {
                    return;
                }
                if !continuous_is_open() {
                    return;
                }
                let Some(bounds) = *track_move.borrow() else {
                    return;
                };
                move_continuous(
                    window_point(event.position),
                    bounds,
                    node_modifiers(&event.modifiers),
                );
                cx.refresh_windows();
            })
            .on_drag(NodeGestureDrag(gesture_id.clone()), |_, _, _window, cx| {
                cx.new(|_| EmptyDragPreview)
            })
            .on_drag_move::<NodeGestureDrag>(move |event, _window, cx| {
                if event.drag(cx).0 != gesture_id {
                    return;
                }
                if !continuous_is_open() {
                    return;
                }
                let bounds = (*track_captured.borrow()).unwrap_or(event.bounds);
                move_continuous(
                    window_point(event.event.position),
                    bounds,
                    node_modifiers(&event.event.modifiers),
                );
                cx.refresh_windows();
            })
            .on_mouse_up(
                MouseButton::Left,
                move |event: &MouseUpEvent, _window, cx| {
                    let Some(bounds) = *track_up.borrow() else {
                        return;
                    };
                    release_continuous(
                        &owner_up,
                        window_point(event.position),
                        bounds,
                        node_modifiers(&event.modifiers),
                    );
                    cx.refresh_windows();
                },
            )
            .on_mouse_up_out(
                MouseButton::Left,
                move |event: &MouseUpEvent, _window, cx| {
                    let bounds = match *track_up_out.borrow() {
                        Some(bounds) => bounds,
                        None => return,
                    };
                    release_continuous(
                        &owner_out,
                        window_point(event.position),
                        bounds,
                        node_modifiers(&event.modifiers),
                    );
                    cx.refresh_windows();
                },
            );
    }

    if let Some(handler) = wheel {
        el = el.on_scroll_wheel(move |event: &ScrollWheelEvent, window, cx| {
            if disabled {
                return;
            }
            let Some((dx, dy)) = wheel_direction(event.delta) else {
                return;
            };
            handler(&NodeWheelEvent {
                dx,
                dy,
                modifiers: node_modifiers(&event.modifiers),
            });
            window.prevent_default();
            cx.stop_propagation();
            cx.refresh_windows();
        });
    }

    el
}

/// Collapse gpui's platform modifier pair onto the vocabulary's single
/// `accel` flag, so components never branch on the host OS.
fn node_modifiers(m: &gpui::Modifiers) -> NodeModifiers {
    NodeModifiers {
        shift: m.shift,
        accel: m.platform || m.control,
        alt: m.alt,
    }
}

fn node_key(key: &str) -> Option<NodeKey> {
    Some(match key {
        "up" => NodeKey::ArrowUp,
        "down" => NodeKey::ArrowDown,
        "left" => NodeKey::ArrowLeft,
        "right" => NodeKey::ArrowRight,
        "home" => NodeKey::Home,
        "end" => NodeKey::End,
        "pageup" => NodeKey::PageUp,
        "pagedown" => NodeKey::PageDown,
        "space" => NodeKey::Space,
        "f2" => NodeKey::F2,
        "delete" => NodeKey::Delete,
        _ => return None,
    })
}

/// Modifier-aware activation, secondary activation, and navigation keys.
fn apply_selection_listeners(mut el: Stateful<Div>, node: &Node) -> Stateful<Div> {
    if let Some(handler) = &node.interaction.on_activate_modified {
        let click = handler.clone();
        el = el.on_click(move |event: &ClickEvent, _window, cx| {
            click(node_modifiers(&event.modifiers()));
            cx.stop_propagation();
            cx.refresh_windows();
        });
    }
    if let Some(handler) = &node.interaction.on_context {
        let ctx = handler.clone();
        el = el.on_mouse_down(MouseButton::Right, move |event, _window, cx| {
            ctx(NodePoint {
                x: event.position.x.into(),
                y: event.position.y.into(),
            });
            cx.refresh_windows();
        });
    }
    if let Some(handler) = &node.interaction.on_key {
        let keys = handler.clone();
        el = el.on_key_down(move |event: &KeyDownEvent, window, cx| {
            if let Some(key) = node_key(event.keystroke.key.as_str()) {
                if let Some(target) = keys(key, node_modifiers(&event.keystroke.modifiers)) {
                    if let Some(handle) = focus_handle_for(&target) {
                        handle.focus(window);
                    }
                }
                cx.refresh_windows();
            }
        });
    }
    el
}

fn window_point(position: gpui::Point<gpui::Pixels>) -> (f32, f32) {
    (position.x.into(), position.y.into())
}

/// Normalized local position: x right, y up, both clamped to 0..=1.
fn continuous_local(window: (f32, f32), bounds: gpui::Bounds<gpui::Pixels>) -> (f32, f32) {
    let left: f32 = bounds.origin.x.into();
    let top: f32 = bounds.origin.y.into();
    let width: f32 = bounds.size.width.into();
    let height: f32 = bounds.size.height.into();
    let x = if width <= 0.0 {
        0.0
    } else {
        ((window.0 - left) / width).clamp(0.0, 1.0)
    };
    let y = if height <= 0.0 {
        0.0
    } else {
        (1.0 - (window.1 - top) / height).clamp(0.0, 1.0)
    };
    (x, y)
}

fn wheel_direction(delta: ScrollDelta) -> Option<(f32, f32)> {
    let (x, y): (f32, f32) = match delta {
        ScrollDelta::Pixels(point) => (point.x.into(), point.y.into()),
        ScrollDelta::Lines(point) => (point.x, point.y),
    };
    if x == 0.0 && y == 0.0 {
        return None;
    }
    Some((
        if x == 0.0 { 0.0 } else { x.signum() },
        if y == 0.0 { 0.0 } else { -y.signum() },
    ))
}

/// Where the pointer sits along `bounds` on the declared axis, clamped to
/// 0.0..=1.0. Horizontal is left → right; vertical is bottom → top.
fn scrub_fraction(
    position: gpui::Point<gpui::Pixels>,
    bounds: gpui::Bounds<gpui::Pixels>,
    axis: ScrubAxis,
) -> f32 {
    match axis {
        ScrubAxis::Horizontal => {
            let left: f32 = bounds.origin.x.into();
            let width: f32 = bounds.size.width.into();
            if width <= 0.0 {
                return 0.0;
            }
            let x: f32 = position.x.into();
            ((x - left) / width).clamp(0.0, 1.0)
        }
        ScrubAxis::Vertical => {
            let top: f32 = bounds.origin.y.into();
            let height: f32 = bounds.size.height.into();
            if height <= 0.0 {
                return 0.0;
            }
            let y: f32 = position.y.into();
            (1.0 - (y - top) / height).clamp(0.0, 1.0)
        }
    }
}

/// Payload for gesture drags (scrub, resize) — distinct from the drag
/// controller's `NativeDragPayload`, so a scrub never joins a drag session.
///
/// It carries the originating element's id because `on_drag_move` is dispatched
/// by drag *type*: every gesture-draggable node in the window hears every
/// gesture drag. Two range sliders on one page both moved from a single drag
/// until each listener started checking that the gesture began on itself.
#[derive(Clone, Debug, PartialEq, Eq)]
struct NodeGestureDrag(String);

/// gpui requires a preview entity for every drag; components draw their own
/// indicator, so this renders nothing.
struct EmptyDragPreview;

impl gpui::Render for EmptyDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div()
    }
}

#[cfg(test)]
mod scrub_axis_tests {
    use super::*;

    fn track() -> gpui::Bounds<gpui::Pixels> {
        gpui::Bounds {
            origin: point(px(10.0), px(20.0)),
            size: size(px(100.0), px(200.0)),
        }
    }

    #[test]
    fn horizontal_scrub_is_left_to_right() {
        let bounds = track();
        assert!(
            (scrub_fraction(point(px(10.0), px(20.0)), bounds, ScrubAxis::Horizontal) - 0.0).abs()
                < 1e-6
        );
        assert!(
            (scrub_fraction(point(px(60.0), px(20.0)), bounds, ScrubAxis::Horizontal) - 0.5).abs()
                < 1e-6
        );
        assert!(
            (scrub_fraction(point(px(110.0), px(20.0)), bounds, ScrubAxis::Horizontal) - 1.0).abs()
                < 1e-6
        );
        assert_eq!(
            scrub_fraction(point(px(0.0), px(20.0)), bounds, ScrubAxis::Horizontal),
            0.0
        );
        assert_eq!(
            scrub_fraction(point(px(200.0), px(20.0)), bounds, ScrubAxis::Horizontal),
            1.0
        );
    }

    #[test]
    fn vertical_scrub_is_bottom_to_top() {
        let bounds = track();
        assert!(
            (scrub_fraction(point(px(10.0), px(220.0)), bounds, ScrubAxis::Vertical) - 0.0).abs()
                < 1e-6
        );
        assert!(
            (scrub_fraction(point(px(10.0), px(120.0)), bounds, ScrubAxis::Vertical) - 0.5).abs()
                < 1e-6
        );
        assert!(
            (scrub_fraction(point(px(10.0), px(20.0)), bounds, ScrubAxis::Vertical) - 1.0).abs()
                < 1e-6
        );
        assert_eq!(
            scrub_fraction(point(px(10.0), px(400.0)), bounds, ScrubAxis::Vertical),
            0.0
        );
        assert_eq!(
            scrub_fraction(point(px(10.0), px(0.0)), bounds, ScrubAxis::Vertical),
            1.0
        );
    }

    #[test]
    fn continuous_local_is_right_and_up() {
        let bounds = track();
        let bottom_left = continuous_local((10.0, 220.0), bounds);
        let top_right = continuous_local((110.0, 20.0), bounds);
        let center = continuous_local((60.0, 120.0), bounds);
        assert!((bottom_left.0 - 0.0).abs() < 1e-6);
        assert!((bottom_left.1 - 0.0).abs() < 1e-6);
        assert!((top_right.0 - 1.0).abs() < 1e-6);
        assert!((top_right.1 - 1.0).abs() < 1e-6);
        assert!((center.0 - 0.5).abs() < 1e-6);
        assert!((center.1 - 0.5).abs() < 1e-6);
    }

    #[test]
    fn wheel_up_is_positive_dy() {
        assert_eq!(
            wheel_direction(ScrollDelta::Lines(point(0.0, -1.0))),
            Some((0.0, 1.0))
        );
        assert_eq!(
            wheel_direction(ScrollDelta::Lines(point(0.0, 1.0))),
            Some((0.0, -1.0))
        );
        assert_eq!(wheel_direction(ScrollDelta::Lines(point(0.0, 0.0))), None);
    }

    #[test]
    fn one_continuous_gesture_then_exactly_one_terminal() {
        reset_continuous_value_session();
        let phases = std::sync::Mutex::new(Vec::new());
        let phases = std::sync::Arc::new(phases);
        let log = std::sync::Arc::clone(&phases);
        let handler: Arc<dyn Fn(&NodeContinuousValueEvent) + Send + Sync> =
            Arc::new(move |event: &NodeContinuousValueEvent| {
                log.lock().expect("phase log").push(event.phase);
            });
        begin_continuous(
            "knob".into(),
            (10.0, 20.0),
            (0.0, 1.0),
            NodeModifiers::default(),
            Arc::clone(&handler),
        );
        begin_continuous(
            "knob".into(),
            (20.0, 20.0),
            (0.1, 1.0),
            NodeModifiers::default(),
            Arc::clone(&handler),
        );
        release_continuous("other", (20.0, 20.0), track(), NodeModifiers::default());
        release_continuous("knob", (60.0, 120.0), track(), NodeModifiers::default());
        release_continuous("knob", (60.0, 120.0), track(), NodeModifiers::default());
        assert_eq!(
            *phases.lock().expect("phase log"),
            [ContinuousValuePhase::Press, ContinuousValuePhase::Release]
        );
        reset_continuous_value_session();
    }

    #[test]
    fn a_lost_host_cancels_once() {
        reset_continuous_value_session();
        let phases = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let log = std::sync::Arc::clone(&phases);
        let handler: Arc<dyn Fn(&NodeContinuousValueEvent) + Send + Sync> =
            Arc::new(move |event: &NodeContinuousValueEvent| {
                log.lock().expect("phase log").push(event.phase);
            });
        begin_continuous(
            "fader".into(),
            (10.0, 20.0),
            (0.0, 1.0),
            NodeModifiers::default(),
            handler,
        );
        prepare_continuous_value_frame();
        sweep_lost_continuous_host();
        sweep_lost_continuous_host();
        assert_eq!(
            *phases.lock().expect("phase log"),
            [ContinuousValuePhase::Press, ContinuousValuePhase::Cancel]
        );
        reset_continuous_value_session();
    }
}
