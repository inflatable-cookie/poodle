# ModelCatalogueEditor

Status: approved
Updated: 2026-08-14
Governing spec: `../../specs/067-model-connection-management.md`

## 1. Purpose

- Component name: `ModelCatalogueEditor`
- Layer: `composites`
- Summary: a controlled surface for ordering shown models and hiding or
  restoring models from one configured connection
- In scope: shown-model reorder, hidden-model recovery, rich model rows,
  keyboard and pointer movement, catalogue postures, optional custom action
- Out of scope: model discovery, route readiness, defaults, favourites,
  invocation, per-model option defaults, persistence, or virtualization

## 2. Anatomy

```text
[Root] <section>
  ├── [Header] title, count, custom action
  ├── [State region] loading/error/unavailable/empty/session-negotiated
  └── ready
      ├── [Shown list] <ol>
      │   └── [Model row] <li> *
      │       ├── [Reorder handle/button]
      │       ├── [Model + provider metadata; optional description]
      │       ├── [Badges + info action] optional
      │       ├── [Move up/down actions]
      │       └── [Hide action]
      └── [Hidden Collapsible]
          └── [Hidden list] <ul>
              └── [Hidden row + Restore action] *
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | labelled curation region | stack gap |
| Header | yes | title, counts, optional custom action | heading, inline gap |
| Shown list | ready | meaningful consumer order | list gap |
| Model row | per shown item | rich identity and utility controls | surface, border, radius |
| Reorder controls | when interactive | pointer and keyboard order change | focus, chrome size |
| Hidden section | when hidden items exist | recoverable but secondary models | muted surface |
| State region | non-ready | honest catalogue posture | status tokens |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `items` | `ModelCatalogueItem[]` | `[]` | no | shown items use source order; hidden order is not meaningful |
| `state` | `ModelCatalogueState` | `"ready"` | no | catalogue posture |
| `title` | `string` | `"Models"` | no | visible heading |
| `hiddenTitle` | `string` | `"Hidden models"` | no | collapsed section heading |
| `ariaLabel` | `string \| null` | `null` | no | falls back to title |
| `isDisabled` | `boolean` | `false` | no | disables editing, preserves reading |
| `isPending` | `boolean` | `false` | no | temporary mutation lock |
| `isDragEnabled` | `boolean` | `true` | no | pointer drag; keyboard/actions remain |
| `showMoveActions` | `boolean` | `true` | no | explicit up/down IconButtons |
| `stateTitle` | `string \| null` | `null` | no | host override for non-ready heading |
| `stateMessage` | `string \| null` | `null` | no | host-safe posture explanation |
| `onOrderChange` | `((orderedIds: string[]) => void) \| null` | `null` | no | complete shown-id order |
| `onVisibilityChange` | `((change: { id: string; visible: boolean }) => void) \| null` | `null` | no | hide/restore request |
| `onInfo` | `((id: string) => void) \| null` | `null` | no | enables per-row info action |

```ts
type ModelCatalogueState =
  | "ready" | "loading" | "unavailable"
  | "empty" | "error" | "sessionNegotiated";

type ModelCatalogueItem = {
  id: string;
  label: string;
  providerLabel: string | null;
  description: string | null;
  badges: { label: string; tone?: PillTone }[];
  visible: boolean;
  isDisabled: boolean;
};
```

### Snippets / Render Props

| Name | Input | Purpose |
|------|-------|---------|
| `leading` | `{ item }` | optional model/provider mark |
| `customAction` | none | custom model or refresh action |
| `rowMeta` | `{ item }` | optional safe capability metadata |

### Controlled And Uncontrolled

The component is controlled. It emits requested order/visibility changes and
renders the next `items` value from the host. Hiding emits only visibility;
restoration position is chosen by the host and reflected in the next array.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | catalogue available | shown order plus optional hidden section |
| loading | `state="loading"` | pending state, no stale controls |
| unavailable | no catalogue route/result | neutral explanation |
| empty | successful catalogue has no entries | distinct empty state |
| error | catalogue request failed | danger explanation |
| session negotiated | models known only after session | informational explanation |
| grabbed | keyboard reorder active | moved row emphasized; live guidance |
| dragging | pointer reorder active | source/drop-target treatment |
| pending/disabled | mutation lock | controls inert, list readable |

### Behavior Machine

Behavior classification: machine-backed.

Context: source items, grabbed id, drag source id, pending/disabled flags.
Events: `GRAB`, `MOVE`, `DROP`, `CANCEL_GRAB`, pointer drag events, `HIDE`,
`RESTORE`, `INFO`, `SET_ITEMS`. Moves apply only to visible enabled items and
emit the complete visible-id order. Visibility events never mutate backend
catalogue truth locally. Effects emit callbacks and live announcements.
Machinery dependencies: `applyReorder`, `listReorderKeyIntent`, focus-by-id,
and disclosure transition for the hidden section.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onOrderChange` | valid pointer/button/keyboard move | complete shown-id array | exact opaque ids |
| `onVisibilityChange` | Hide or Restore activated | `{ id, visible }` | host chooses restored position |
| `onInfo` | optional info action activated | model id | no row-selection meaning |

## 6. Accessibility

- Root is a labelled section; shown models use an ordered list, not listbox
  semantics, because rows contain independent actions.
- Reorder handles are buttons named with label and current position.
- `Space`/`Enter` grabs or drops; arrows move; Escape cancels.
- Explicit up/down buttons provide the same operation without grab mode.
- Each hide/restore action includes the model label.
- A polite atomic live region announces moves and visibility requests.
- Focus follows a moved model. Hiding moves focus to the next shown model, the
  previous model, or the hidden-section disclosure when none remain.
- GPUI later exposes list position, button names, and announcements natively.

## 7. Layout

- Rows are compact grid/flex surfaces: reorder lane, flexible copy, utilities.
- Model leads the compact title line; provider follows as quieter inline
  metadata. The optional description is the only second copy line; both
  truncate visually while retaining accessible names.
- Badges sit at the utility edge immediately before the optional info action.
- Utility controls do not shrink. At narrow widths, description moves below
  title and actions may occupy a second row.
- Hidden section is visually quieter and does not reserve reorder lanes.
- Very large catalogues require host filtering/windowing before passing items;
  virtualization is intentionally outside this first component.

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Row | `color.background.surface`, `color.border.subtle`, `radius.control` | compact item |
| Grabbed/drop | `color.accent.base`, `color.accent.focusRing` | movement |
| Hidden | `color.text.muted`, `color.background.panel` | secondary posture |
| Utilities | semantic chrome size/focus tokens | actions |
| Layout | `space.stack.*`, `space.inline.*` | rhythm |

## 9. Svelte Notes

- Reuse core reorder helpers; do not fork EditableList's machine.
- Rich action rows are purpose-built because EditableList's option semantics
  and add/remove workflow do not fit nested model actions.
- Key rows by opaque id.

## 10. GPUI Notes

- Implemented: `ModelCatalogueEditorSpec`
  (`packages/contracts/components/src/model_catalogue_editor.rs`),
  `poodle_render::model_catalogue_editor`, GPUI specimen
  `packages/gpui/preview/src/specimens/model_catalogue_editor_specimen.rs`.
- Pointer drag runs on the shared drag-and-drop substrate (architecture 011,
  spec 069): the reorder handle registers a `NodeDragSource` and every unlocked
  row a `NodeDropTarget`, and the GPUI `DragDropController` owns the session,
  hit testing, cancellation, and exactly-once cleanup. The subject kind is
  scoped to the catalogue instance and a row dropped onto itself is rejected.
  The explicit and keyboard grab/move route is unchanged and emits the same
  order payload.

### Native Binding

- Derivations, the reorder result, the focus-after-hide rule, and every
  announcement come from `poodle_headless::model_connection`. The Rust mirror
  of core's `listReorderKeyIntent` lives there as
  `model_catalogue_reorder_key_intent`.
- Transient interaction state is host state on the spec — `grabbed_id`,
  `drop_target_id`, `hidden_open`, and `live_message` — and the renderer asks
  for the next value through `on_grab_change`, `on_drop_target_change`,
  `on_hidden_open_change`, and `on_announce`.
- Keyboard grab and drop ride the backend's own activation path (Enter and
  Space), arrows ride `on_key`, and Escape rides `on_cancel`: the vocabulary
  has no other Escape channel for a plain control, and binding Space in both
  places would toggle the grab twice.
- Focus after a move or a hide is a request naming the destination element id;
  the backend performs the move. Hiding the last shown model also asks for the
  hidden section to be disclosed, so the destination exists. That destination
  is `Collapsible`'s own focusable trigger — the outer region it returns is not
  focusable — and the trigger is stamped with a focus patch so the backend
  creates a handle for it.
- `ModelCatalogueEditorHandlers::instance_id` is the backend-state scope.
  Semantic row ids stay readable; the scope lives on `runtime_id`, and focus
  destinations are named with `model_catalogue_handle_focus_id` /
  `model_catalogue_hidden_focus_id`.
- `leading` and `rowMeta` become host-composed nodes keyed by item id;
  `customAction` is a single header node.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] complete ordered-id payloads and visibility payloads match
- [ ] keyboard/button movement, guards, focus, and announcements match
- [ ] all catalogue postures remain distinct

### Tier 2: Visual Parity

- [ ] shown/hidden hierarchy, row density, and movement emphasis match

### Tier 3: Implementation Freedom

- [ ] pointer drag implementation may differ without changing outcomes

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| — | — | no open deltas; native completion landed in `g15.008` | — |

## 13. Approval And Adoption Notes

- contract status: `approved`
- approver: operator, 2026-08-14
- downstream adopter: Nucleus
- native completion: landed in `g15.008` (Rust declaration, `poodle-headless`
  behaviour mirror, `poodle-render` composition, GPUI specimen and mounted
  evidence)
- future follow-up: large-catalogue virtualization only after a concrete case
