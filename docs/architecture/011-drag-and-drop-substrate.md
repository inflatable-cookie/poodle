# 011 Drag And Drop Substrate

Status: active
Accepted: 2026-08-28
Owner: Poodle core
Depends on: `001-poodle-system-shape.md`,
`006-headless-core-and-machine-model.md`,
`../contracts/001-working-rules.md`

## Decision

Poodle owns one semantic drag-and-drop model with paired TypeScript and Rust
implementations. Runtime adapters translate pointer, keyboard, browser-native,
window-host, and operating-system events into that model. Components and
consumers register sources and targets; they do not write their own drag event
choreography.

The model owns:

- source and target identity;
- payload kind and opaque subject identity;
- preparation, activation, hover, drop, cancellation, and finalization;
- target eligibility and semantic drop intent;
- deterministic nested-target arbitration;
- current operation (`move`, `copy`, or `link`);
- exactly-once terminal cleanup; and
- announcements and focus-return intent.

Adapters own mechanisms that cannot cross renderer boundaries:

- pointer capture and hit testing;
- measured geometry and auto-scroll;
- drag previews and platform cursors;
- DOM, GPUI, browser, webview, IPC, and native-shell event translation; and
- platform accessibility projection.

Consumers own durable mutation and domain policy. A successful Poodle drop
reports semantic intent. It does not reorder, move, copy, persist, authorize,
render, or delete application data unless a separately documented helper is
explicitly selected.

## Runtime Shape

```text
                    component / consumer registrations
                                   |
                    semantic drag session contract
                         /                    \
             TypeScript machine             Rust machine
              poodle-core              poodle-headless/render
                /       \                    /          \
          Svelte       React              GPUI       Jetstream deferred
                \       /                    |
          web sensors and transports   native input adapter
                    \                       /
                       optional host bridge
                  cross-window / OS file drag
```

The two machines use the same lifecycle vocabulary and shared transition
vectors. This is a bounded substrate contract, not a new component IR or a
replacement cross-runtime conformance authority.

## Transport Families

### Internal interaction

Same-document pointer and keyboard movement uses a Poodle-owned transport.
Web uses Pointer Events and explicit pointer capture rather than HTML Drag and
Drop. Its first delivery certifies touch, mouse, pen-shaped pointers, and
keyboard against the same session and target-selection rules.

GPUI uses stock crates.io input rather than a fork or a second OS input
backend. GPUI 0.2.2 certifies mouse and keyboard. Its typed `on_drag_move`
route supplies the observable in-window capture result, while Escape, release,
host rebuild, and explicit host cancellation close the session. The crate does
not expose touch contacts, pen identity, or a device-originated pointer-cancel
event, so the GPUI adapter advertises those capabilities as unsupported. Mouse
synthesis is not pen or touch evidence. This is declared active-runtime debt;
the semantic session and target-selection rules do not fork around it.

This transport covers reorder, nested placement, application-local movement,
previews, auto-scroll, and cancellation. It does not pretend to follow a
pointer outside its process or window.

### Host cross-window bridge

Cross-window transfer is a host capability. Poodle exposes preparation,
activation, target projection, and terminal-result hooks. The host owns global
window geometry, IPC or native observation, session leases, authorization,
and authoritative commit.

Longhorn remains authoritative for its bounded transfer sessions. Poodle may
carry only a host-issued opaque session identifier across a web or native
transport. It does not learn window topology, panel placement, durable layout,
or transfer credentials.

Keyboard cross-window movement uses an accessible target-selection command
such as “Move to…”. Arrow keys do not simulate traversal between independent
windows.

### Native web data transfer

HTML Drag and Drop is a bounded adapter for capabilities supplied by the
browser or webview: inbound files, interoperable external data, and opaque
host-issued session tokens. `DataTransfer` is never the internal source of
truth. Its mutability and payload visibility are phase-dependent, and webviews
do not behave identically.

### Native shell drag-out

Drag-out is a host capability from the first release. Poodle describes an
opaque export subject and its preparation state. The host resolves that
subject into one or more existing or temporary files and begins the native OS
drag.

Electron can implement the adapter with its supported `webContents.startDrag`
API. Tauri requires a bounded application or plugin adapter because its public
JavaScript drag/drop surface is inbound-file oriented. Poodle does not depend
on either shell.

File-backed export is the portable baseline. Promised files and arbitrary
custom MIME data are optional advertised capabilities. Consumer code must not
branch on user agents or shell names.

## Session And Payload Boundaries

A semantic subject is stable and intentionally small:

```text
subject = { kind, id }
```

Presentation metadata, domain records, secrets, filesystem paths, and mutable
application objects do not become the portable payload. A same-process host may
keep richer data behind the identifier. A process or window boundary carries
only an opaque host-issued session token.

A target resolves raw geometry into semantic intent:

```text
target = { targetId, position, operation }
position = before | inside | after | consumer-defined
operation = move | copy | link
```

The target may reject the subject with a reason suitable for presentation or
announcement. Eligibility is checked again at commit. Hover acceptance never
authorizes durable mutation.

DOM geometry is pointer evidence, not the complete keyboard catalogue. A
paged or windowed component may register element-free logical keyboard targets
with stable order and direction-aware position resolution. Pointer and touch
hit-testing ignore them. Keyboard intent reuses their normal eligibility,
announcement, commit, removal, and terminal paths, so an unmounted destination
does not require fake DOM or a component-owned drag lifecycle. The active
source remains mounted until the session ends; layout paging follows commit.

An established component shortcut may express a complete semantic move in one
keystroke rather than entering pickup mode. The web controller accepts that as
an explicit keyboard drop command over a live registered source and DOM or
logical target. It still creates the normal semantic session, revalidates the
target, runs the normal commit/terminal callbacks, announces the result, and
returns focus. The command is not a direct callback shortcut or a second
component-owned lifecycle.

## Touch Boundary

Touch is part of the first contract, not a later compatibility layer. The
sensor supports activation distance, optional hold delay, movement tolerance,
scroll arbitration, pointer capture, edge auto-scroll, and explicit cancel.
Ordinary vertical or horizontal scrolling wins until activation. `touch-action`
is narrowed to the registered handle or surface; Poodle does not disable page
scrolling globally.

Cross-window touch requires a host capable of observing a pointer outside the
origin window. Where that capability is absent, internal touch remains
available and the adapter reports cross-window touch as unsupported.

## External Artifact Ownership

Poodle never receives an eager filesystem path through a component prop. A
drag-out source asks its host adapter to prepare an opaque subject. The adapter
returns an armed export receipt or declines.

The host owns:

- rendering or otherwise materializing an artifact;
- temporary directory choice and permissions;
- file naming and collision policy;
- native drag start;
- retention after drag end; and
- eventual cleanup.

Native drag completion does not prove that a destination consumed the file.
Poodle therefore emits terminal lifecycle information but never deletes a
temporary artifact itself.

## Component And Consumer Boundary

The substrate must work without a Poodle composite. Consumers can build custom
sources, targets, previews, and drop surfaces from the same registrations used
by Poodle components.

Poodle components remain responsible for their public semantic callbacks. A
Tabs reorder reports the resulting order; a Tree move reports a target and
placement; DockRegion projects the host bridge. These component contracts do
not become generic application policy.

An externally authoritative consumer may need to narrow a composite's
resolved candidate before Poodle paints it as accepted. That remains a
component adapter over this substrate, not another drag machine: the adapter
projects one immutable semantic subject, runs synchronous eligibility during
hover and revalidation, may rewrite only the component's own destination
vocabulary, and returns the substrate's real commit result. The ordinary
self-contained callback stays available when no external authority is
installed. Tree is the first bounded use of this pattern; its authority seam
does not expose pointer coordinates, DOM ancestry, application records, or a
second lifecycle.

Continuous gestures such as sliders, faders, scrubbing, resize handles, and
knob movement remain separate. They may reuse low-level pointer-capture helpers
but do not create payload sessions or drop targets.

## Evidence Boundary

Certification is layered:

- pure transition vectors run against the TypeScript and Rust machines;
- mounted Svelte and React tests dispatch real pointer, touch-like pointer,
  and keyboard events;
- Chromium and WebKit prove web geometry, capture, scrolling, and cleanup;
  Chromium additionally proves compositor-native touch hold-versus-scroll.
  Desktop Playwright WebKit has no native touch-move injection, so its headless
  leg proves touch-shaped Pointer Event hold/tolerance behavior and must label
  that limitation rather than claiming native touch scrolling;
- mounted GPUI tests prove native mouse and keyboard dispatch plus the stock
  `on_drag_move` capture-equivalent path. They do not claim pen, touch, or
  device-cancel support from mouse synthesis;
- a host-controlled multi-window fixture proves preparation, opaque-token
  transfer, target revalidation, commit, and cancellation without taking
  operator focus; and
- native drag-out adapters receive platform integration tests plus a bounded
  manual OS interoperability smoke where automation cannot prove destination
  consumption.

Specimen pages demonstrate useful scenarios. They are not exhaustive
conformance matrices.

## Rejected Alternatives

### Standardize on HTML Drag and Drop

Rejected. It is mouse-centric, phase-restricts `DataTransfer`, varies across
webviews, and does not supply the touch, keyboard, deterministic nested target,
or GPUI model required here.

### Keep component-local drag state

Rejected. Tabs, Tree, EditableList, BlockEditor, OrderBy,
ModelCatalogueEditor, and DockRegion already prove that repeated local
lifecycle code drifts and misses cleanup.

### Put window transfer authority in Poodle

Rejected. Poodle cannot authorize or durably mutate Longhorn layouts and must
not learn application window topology.

### Put files in the semantic payload

Rejected. Paths are privileged host data, temporary artifacts have a longer
lifecycle than the pointer gesture, and lazy rendering belongs to the
consumer.

### Treat drag-out as custom MIME first

Rejected. Files are the interoperable boundary for desktops and DAWs. Custom
formats remain negotiated extensions rather than implied portability.
