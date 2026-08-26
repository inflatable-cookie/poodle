//! GPUI listener, editing, gesture, and drop routing for node interactions.

use super::*;

pub(super) fn apply_listeners(mut el: Stateful<Div>, node: &Node, id: &str) -> Stateful<Div> {
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
            el = el.focusable();
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
                        // the element refinement's — decide traversal.
                        let updated = entry
                            .clone()
                            .tab_index(tab_index.unwrap_or(0).max(0) as isize)
                            .tab_stop(tab_index.is_some_and(|index| index >= 0));
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
                        handle.focus(window);
                        cx.refresh_windows();
                    }
                    let now = handle.is_focused(window);
                    let changed = FOCUS_STATES.with(|states| {
                        let mut states = states.borrow_mut();
                        states.insert(id.clone(), now) != Some(now)
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
                            // whatever takes this id next.
                            input_text::forget(&id);
                        }
                        if let Some(handler) = &on_focus_change {
                            handler(now);
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
        let border = &node.style.descriptor.border;
        let border_left = node.style.border_left_width.unwrap_or(border.width);
        let border_right = node.style.border_right_width.unwrap_or(border.width);
        let border_top = node.style.border_top_width.unwrap_or(border.width);
        let border_bottom = node.style.border_bottom_width.unwrap_or(border.width);
        let radii = node.style.descriptor.corner_radii;
        el = el.child(
            gpui::canvas(
                move |_, _, _| {},
                move |bounds, (), window, _cx| {
                    let focused = super::focus_handle_for(&ring_id)
                        .is_some_and(|handle| handle.is_focused(window));
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
    if let Some(layer_id) = &node.interaction.dismiss_layer {
        let element_id = id.to_owned();
        let layer = layer_id.clone();
        el = el.child(
            gpui::canvas(
                move |bounds, _window, _cx| {
                    super::layers::record_bounds(&element_id, &layer, bounds);
                },
                |_, _, _, _| {},
            )
            .absolute()
            .size_full(),
        );
    } else if node.id.is_some() {
        let element_id = id.to_owned();
        el = el.child(
            gpui::canvas(
                move |bounds, _window, _cx| {
                    super::layers::record_element_bounds(&element_id, bounds);
                },
                |_, _, _, _| {},
            )
            .absolute()
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
        el = el.tooltip(move |_window, cx| {
            AnyView::from(cx.new(|_| NodeTooltip {
                text: SharedString::from(text.clone()),
            }))
        });
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
        // Clipboard is the backend's: the text comes from outside the tree, and
        // `App` reaches it directly. (IME still needs an `EntityInputHandler`,
        // which a `&Node -> AnyElement` backend has no entity to hang on.)
        let insert = node.interaction.on_edit_insert.clone();
        // Undo needs the *value* node's id, because that is what paint records
        // history under; the keys arrive at the focusable root above it.
        let value_id = input_text::history_key(id);
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
                if matches!(key, "enter" | "tab") {
                    if let Some(handler) = &submit {
                        handler();
                        cx.refresh_windows();
                        return;
                    }
                } else if key == "escape" {
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
    el = apply_selection_listeners(el, node);
    apply_drop_listeners(el, node, id)
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

/// One backend-owned payload-drag session. Shared by the production overlay
/// host and the headless mount host so start/leave/drop/end stay ordered and
/// end fires exactly once on every path — including release outside a zone
/// and Escape — without a component-specific global or a GPUI fork.
struct PayloadSession {
    payload: String,
    end: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    current_zone: Option<String>,
    current_leave: Option<Arc<dyn Fn() + Send + Sync>>,
    last_edge: DropEdge,
    ended: bool,
    dropped: bool,
}

thread_local! {
    static PAYLOAD_SESSION: RefCell<Option<PayloadSession>> = const { RefCell::new(None) };
}

fn finish_payload_session(session: &mut PayloadSession, cx: &mut App) {
    if session.ended {
        return;
    }
    session.ended = true;
    if let Some(leave) = session.current_leave.take() {
        leave();
    }
    session.current_zone = None;
    if let Some(end) = session.end.take() {
        end(&session.payload);
    }
    cx.refresh_windows();
}

fn begin_payload_session(
    payload: String,
    start: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    end: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    cx: &mut App,
) {
    PAYLOAD_SESSION.with(|session| {
        let mut slot = session.borrow_mut();
        if let Some(existing) = slot.as_mut() {
            finish_payload_session(existing, cx);
        }
        *slot = Some(PayloadSession {
            payload: payload.clone(),
            end,
            current_zone: None,
            current_leave: None,
            last_edge: DropEdge::default(),
            ended: false,
            dropped: false,
        });
        if let Some(start) = start {
            start(&payload);
        }
    });
    cx.refresh_windows();
}

fn payload_session_move(
    zone_id: &str,
    hit: bool,
    edge: DropEdge,
    hover: Option<&Arc<dyn Fn(&NodeDropEvent) + Send + Sync>>,
    leave: Option<&Arc<dyn Fn() + Send + Sync>>,
    cx: &mut App,
) {
    PAYLOAD_SESSION.with(|session| {
        let mut slot = session.borrow_mut();
        let Some(session) = slot.as_mut() else {
            return;
        };
        if session.ended {
            return;
        }
        if hit {
            if session.current_zone.as_deref() != Some(zone_id) {
                if let Some(previous) = session.current_leave.take() {
                    previous();
                }
                session.current_zone = Some(zone_id.to_owned());
                session.current_leave = leave.cloned();
            }
            session.last_edge = edge;
            if let Some(hover) = hover {
                hover(&NodeDropEvent {
                    payload: session.payload.clone(),
                    edge,
                });
            }
            cx.refresh_windows();
        } else if session.current_zone.as_deref() == Some(zone_id) {
            if let Some(previous) = session.current_leave.take() {
                previous();
            }
            session.current_zone = None;
            cx.refresh_windows();
        }
    });
}

fn payload_session_drop(drop: &Arc<dyn Fn(&NodeDropEvent) + Send + Sync>, cx: &mut App) {
    PAYLOAD_SESSION.with(|session| {
        let mut slot = session.borrow_mut();
        let Some(session) = slot.as_mut() else {
            return;
        };
        if session.ended {
            return;
        }
        session.dropped = true;
        drop(&NodeDropEvent {
            payload: session.payload.clone(),
            edge: session.last_edge,
        });
        finish_payload_session(session, cx);
    });
}

/// End an unfinished payload session as cancellation. Called from the shared
/// root host on mouse-up after the zone `on_drop` bubble (so a successful
/// drop has already ended the session) and when no zone was hit.
pub(crate) fn release_payload_session(cx: &mut App) {
    PAYLOAD_SESSION.with(|session| {
        let mut slot = session.borrow_mut();
        let Some(active) = slot.as_mut() else {
            return;
        };
        if active.ended || active.dropped {
            return;
        }
        finish_payload_session(active, cx);
    });
}

/// Escape cancellation: end the session once, then stop stock GPUI's drag so
/// the preview does not linger. Returns whether a live payload session was
/// cancelled.
pub(crate) fn cancel_payload_session(window: &mut Window, cx: &mut App) -> bool {
    let cancelled = PAYLOAD_SESSION.with(|session| {
        let mut slot = session.borrow_mut();
        let Some(active) = slot.as_mut() else {
            return false;
        };
        if active.ended {
            return false;
        }
        finish_payload_session(active, cx);
        true
    });
    if cancelled {
        cx.stop_active_drag(window);
    }
    cancelled
}

/// Drag sources and drop zones.
///
/// The edge is derived here, from the zone's own bounds, and only the
/// resulting `DropEdge` reaches the component — the vocabulary's rule that a
/// component never sees layout stays intact. Hover is hit-tested against those
/// same bounds because GPUI's `on_drag_move` is type-wide: every zone hears
/// every payload move.
fn apply_drop_listeners(mut el: Stateful<Div>, node: &Node, id: &str) -> Stateful<Div> {
    if let Some(payload) = &node.interaction.drag_payload {
        let payload = NodeDragPayload {
            id: payload.clone(),
        };
        let start = node.interaction.on_drag_start.clone();
        let end = node.interaction.on_drag_end.clone();
        let payload_id = payload.id.clone();
        el = el.on_drag(payload, move |_payload, _offset, _window, cx| {
            // gpui requires a preview entity; the drop indicator is drawn by
            // the component from its own `on_drop_hover` state, so this is
            // deliberately empty. The constructor is stock GPUI's drag-start
            // boundary (after the 2px threshold).
            begin_payload_session(payload_id.clone(), start.clone(), end.clone(), cx);
            cx.new(|_| EmptyDragPreview)
        });
    }
    if !node.interaction.drop_zone {
        return el;
    }
    // A branch zone accepts an "inside" drop; a leaf only takes before/after.
    let accepts_inside = node.a11y.role == Some(NodeRole::TreeItem) || node.children.is_empty();
    let zone_id = id.to_owned();
    let hover = node.interaction.on_drop_hover.clone();
    let leave = node.interaction.on_drop_leave.clone();
    el = el.on_drag_move::<NodeDragPayload>(move |event, _window, cx| {
        let hit = event.bounds.contains(&event.event.position);
        let height = f32::from(event.bounds.size.height).max(1.0);
        let rel = f32::from(event.event.position.y - event.bounds.origin.y) / height;
        payload_session_move(
            &zone_id,
            hit,
            edge_for(rel, accepts_inside),
            hover.as_ref(),
            leave.as_ref(),
            cx,
        );
    });
    if let Some(handler) = &node.interaction.on_drop {
        let drop = handler.clone();
        el = el.on_drop::<NodeDragPayload>(move |_payload, _window, cx| {
            payload_session_drop(&drop, cx);
        });
    }
    el
}

/// Split a zone's height into before / inside / after bands. A zone that
/// cannot take an inside drop splits in half instead of thirds.
fn edge_for(rel: f32, accepts_inside: bool) -> DropEdge {
    if accepts_inside {
        if rel < 0.25 {
            DropEdge::Before
        } else if rel > 0.75 {
            DropEdge::After
        } else {
            DropEdge::Inside
        }
    } else if rel < 0.5 {
        DropEdge::Before
    } else {
        DropEdge::After
    }
}

#[cfg(test)]
mod payload_edge_tests {
    use super::{edge_for, DropEdge};

    #[test]
    fn a_zone_that_accepts_inside_keeps_before_and_after_bands() {
        assert_eq!(edge_for(0.1, true), DropEdge::Before);
        assert_eq!(edge_for(0.5, true), DropEdge::Inside);
        assert_eq!(edge_for(0.9, true), DropEdge::After);
    }

    #[test]
    fn a_leaf_zone_never_collapses_to_the_default_inside_edge() {
        assert_eq!(edge_for(0.25, false), DropEdge::Before);
        assert_eq!(edge_for(0.5, false), DropEdge::After);
        assert_ne!(edge_for(0.5, false), DropEdge::default());
    }
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

/// Payload for gesture drags (scrub, resize) — distinct from
/// `NodeDragPayload`, so a scrub never lands in a drop zone.
///
/// It carries the originating element's id because `on_drag_move` is dispatched
/// by drag *type*: every gesture-draggable node in the window hears every
/// gesture drag. Two range sliders on one page both moved from a single drag
/// until each listener started checking that the gesture began on itself.
#[derive(Clone, Debug, PartialEq, Eq)]
struct NodeGestureDrag(String);

/// The dragged node's opaque id, carried through gpui's drag channel.
#[derive(Clone, Debug)]
struct NodeDragPayload {
    id: String,
}

/// gpui requires a preview entity for every drag; components draw their own
/// indicator, so this renders nothing.
struct EmptyDragPreview;

impl gpui::Render for EmptyDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div()
    }
}

/// Native hover tooltip for `Node.tooltip`. GPUI's `.tooltip()` requires an
/// `AnyView`; this is the smallest text view, not Poodle's Tooltip overlay.
struct NodeTooltip {
    text: SharedString,
}

impl gpui::Render for NodeTooltip {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div()
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(6.0))
            .bg(gpui::hsla(0.0, 0.0, 0.12, 0.96))
            .text_color(gpui::hsla(0.0, 0.0, 0.96, 1.0))
            .text_sm()
            .child(self.text.clone())
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
}
