//! GPUI listener, editing, gesture, and drop routing for node interactions.

use super::*;

pub(super) fn apply_listeners(mut el: Stateful<Div>, node: &Node) -> Stateful<Div> {
    if node.interaction.focusable {
        el = el.focusable();
    }

    // Real focus, observed both ways. gpui auto-creates a focus handle and
    // keeps it in element state it never hands back, so we own one instead:
    // created lazily in the paint pass (the first place with an `App`) and
    // attached from the next build onward. That makes *blur* observable, which
    // is what a latched-on-click flag could never do.
    if tracks_focus(node) {
        let id = element_id_string(node);
        if let Some(handle) = focus_handle_for(&id) {
            el = el.track_focus(&handle);
        }
        let on_focus_change = node.interaction.on_focus_change.clone();
        el = el.child(
            gpui::canvas(
                move |_bounds, window, cx| {
                    let mut created = false;
                    let handle = FOCUS_HANDLES.with(|handles| {
                        handles
                            .borrow_mut()
                            .entry(id.clone())
                            .or_insert_with(|| {
                                created = true;
                                cx.focus_handle()
                            })
                            .clone()
                    });
                    if created {
                        // The element that wants this handle was already built
                        // without it — nothing else would repaint, so the
                        // handle would sit unattached and never see a focus.
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
    if !node.interaction.disabled {
        if let Some(patch) = &node.style.active {
            let patch = *patch;
            el = el.active(move |s| apply_patch(s, patch));
        }
        // gpui's focus styling needs the element focusable, which the
        // `focusable` flag above has already arranged.
        if node.interaction.focusable {
            if let Some(patch) = &node.style.focus {
                let patch = *patch;
                el = el.focus(move |s| apply_patch(s, patch));
            }
        }
    }
    if node.interaction.disabled {
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
        let id = element_id_string(node);
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
        let value_id = input_text::history_key(&element_id_string(node));
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
        // A scrub reports where the pointer sits ALONG this element, as a
        // fraction of its own width. Pressing counts — clicking a slider track
        // jumps the value there — and the gesture then follows the pointer
        // anywhere via `on_drag_move`.
        //
        // A mouse-down event carries no bounds, so a zero-cost `canvas` child
        // records the track's rectangle at paint time for the press to read.
        // `on_drag_move` supplies its own.
        let track: Rc<RefCell<Option<gpui::Bounds<gpui::Pixels>>>> = Rc::new(RefCell::new(None));
        let track_paint = track.clone();
        let track_press = track.clone();
        let press = handler.clone();
        let mv = handler.clone();
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
            // `on_click`, NOT `on_mouse_down`: registering `on_drag` on an
            // element makes gpui's drag machinery swallow that element's
            // mouse-down, so a down listener here never runs. Click survives,
            // and a press-without-drag is exactly the track-click case.
            .on_click(move |event: &ClickEvent, _window, cx| {
                if let Some(bounds) = *track_press.borrow() {
                    press(
                        scrub_fraction(event.position().x.into(), bounds),
                        ScrubPhase::Press,
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
                mv(
                    scrub_fraction(event.event.position.x.into(), event.bounds),
                    ScrubPhase::Drag,
                );
                cx.refresh_windows();
            });
    }
    el = apply_selection_listeners(el, node);
    apply_drop_listeners(el, node)
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
        el = el.on_key_down(move |event: &KeyDownEvent, _window, cx| {
            if let Some(key) = node_key(event.keystroke.key.as_str()) {
                keys(key, node_modifiers(&event.keystroke.modifiers));
                cx.refresh_windows();
            }
        });
    }
    el
}

/// Drag sources and drop zones.
///
/// The edge is derived here, from the zone's own bounds, and only the
/// resulting `DropEdge` reaches the component — the vocabulary's rule that a
/// component never sees layout stays intact.
fn apply_drop_listeners(mut el: Stateful<Div>, node: &Node) -> Stateful<Div> {
    if let Some(payload) = &node.interaction.drag_payload {
        let payload = NodeDragPayload {
            id: payload.clone(),
        };
        el = el.on_drag(payload, |_payload, _offset, _window, cx| {
            // gpui requires a preview entity; the drop indicator is drawn by
            // the component from its own `on_drop_hover` state, so this is
            // deliberately empty.
            cx.new(|_| EmptyDragPreview)
        });
    }
    if !node.interaction.drop_zone {
        return el;
    }
    // A branch zone accepts an "inside" drop; a leaf only takes before/after.
    let accepts_inside = node.a11y.role == Some(NodeRole::TreeItem) || node.children.is_empty();
    if let Some(handler) = &node.interaction.on_drop_hover {
        let hover = handler.clone();
        el = el.on_drag_move::<NodeDragPayload>(move |event, _window, cx| {
            let height = f32::from(event.bounds.size.height).max(1.0);
            let rel = f32::from(event.event.position.y - event.bounds.origin.y) / height;
            hover(&NodeDropEvent {
                payload: event.drag(cx).id.clone(),
                edge: edge_for(rel, accepts_inside),
            });
            cx.refresh_windows();
        });
    }
    if let Some(handler) = &node.interaction.on_drop {
        let drop = handler.clone();
        el = el.on_drop::<NodeDragPayload>(move |payload, _window, cx| {
            drop(&NodeDropEvent {
                payload: payload.id.clone(),
                // The last hover already told the component where the
                // indicator sits; a drop reuses it rather than recomputing
                // from a position gpui does not hand to `on_drop`.
                edge: DropEdge::default(),
            });
            cx.refresh_windows();
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

/// Where `x` sits across `bounds`, clamped to 0.0..=1.0.
fn scrub_fraction(x: f32, bounds: gpui::Bounds<gpui::Pixels>) -> f32 {
    let left: f32 = bounds.origin.x.into();
    let width: f32 = bounds.size.width.into();
    if width <= 0.0 {
        return 0.0;
    }
    ((x - left) / width).clamp(0.0, 1.0)
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
