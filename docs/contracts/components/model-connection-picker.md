# ModelConnectionPicker

Status: approved
Updated: 2026-08-14
Governing spec: `../../specs/067-model-connection-management.md`

## 1. Purpose

- Component name: `ModelConnectionPicker`
- Layer: `composites`
- Summary: a searchable, grouped radio-card picker for one exact configured
  model route
- In scope: local text filtering, grouped options, exact opaque selection,
  option availability, async catalogue postures, provider-mark composition
- Out of scope: discovery, route fallback, credentials, setup forms, provider
  branding assets, persistence, or executable route selection

## 2. Anatomy

```text
[Root] <section>
  └── [PickerShell]
      ├── [Toolbar]
      │   └── [TextInput type="search"]
      ├── [Groups] <div>
      │   └── [Group] <section> *
      │       ├── [Group title] <h3>
      │       └── [Radio cards] <div role="radiogroup">
      │           └── [Option] <button role="radio"> *
      │               ├── [Leading snippet]
      │               ├── [Provider + route copy]
      │               └── [Compact availability]
      └── [Footer snippet]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | labelled picker region | inherited surface |
| Toolbar | yes | search field | control height, inline gap |
| Groups | ready only | host-ordered option groups | stack gap |
| Option | per visible option | exact route choice | surface, border, radius, focus |
| Leading | no | host-rendered provider mark, replaced by the selected check | icon size, tint surface |
| Availability | yes | text plus status indicator | status colour, secondary text |
| Footer | no | host actions or guidance | separator, stack gap |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `options` | `ModelConnectionOption[]` | `[]` | no | host-ordered exact route options |
| `value` | `string \| null \| undefined` | `undefined` | no | controlled selected option id |
| `defaultValue` | `string \| null` | `null` | no | **Web targets only** — uncontrolled initial selection; the native binding keeps the current value on the host (see Native Binding) |
| `query` | `string \| undefined` | `undefined` | no | controlled search text |
| `defaultQuery` | `string` | `""` | no | **Web targets only** — uncontrolled initial search text; see Native Binding |
| `state` | `PickerState` | `"ready"` | no | catalogue posture |
| `title` | `string` | `"Choose a connection"` | no | PickerShell heading |
| `description` | `string \| null` | `null` | no | supporting copy |
| `searchPlaceholder` | `string` | `"Search connections"` | no | query placeholder |
| `ariaLabel` | `string \| null` | `null` | no | root name; falls back to title |
| `isDisabled` | `boolean` | `false` | no | disables search and options |
| `variant` | `"inline" \| "popover" \| "modal"` | `"inline"` | no | forwarded to PickerShell |
| `onValueChange` | `((id: string) => void) \| null` | `null` | no | exact option selection request |
| `onQueryChange` | `((query: string) => void) \| null` | `null` | no | query change request |

```ts
type ModelConnectionAvailability =
  | "available" | "checking" | "unavailable" | "unsupported";

type PickerState = "ready" | "loading" | "error" | "empty" | "noResults";

type ModelConnectionOption = {
  id: string;
  providerLabel: string;
  routeLabel: string | null;
  description: string | null;
  group: string;
  keywords: string[];
  availability: ModelConnectionAvailability;
  availabilityLabel: string;
  isDisabled: boolean;
  requiresConfiguration: boolean;
};
```

### Snippets / Render Props

| Name | Input | Purpose |
|------|-------|---------|
| `leading` | `{ option }` | provider mark; default is a generic connection icon |
| `footer` | none | optional workflow guidance or actions |

### Controlled And Uncontrolled

`value` and `query` are independently controllable. Local filtering matches a
case-folded query against provider label, route label, description, group, and
keywords. The host controls ranking and source order by ordering `options`.

## 4. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | visible matches exist | grouped radio cards |
| loading | `state="loading"` | PickerShell loading posture |
| error | `state="error"` | error posture; no choices |
| empty | `state="empty"` or no source options | empty catalogue posture |
| no results | non-empty query has no matches | no-results posture |
| disabled | `isDisabled` | controls inert; selection retained |
| selected | option id equals value | accent border; checked indicator replaces provider mark |
| unavailable | option availability is unavailable/unsupported | reason shown; option disabled |

### Behavior Machine

Behavior classification: machine-backed.

Context: controllable `value` and `query`, source options, `isDisabled`.
Events: `SET_QUERY`, `SELECT`, `SET_VALUE`, `SET_OPTIONS`.
`SELECT` is inert for disabled, unavailable, or unsupported options. Other
selection and query transitions emit their matching callbacks. Filtering and
first-visible-group derivation are pure. Machinery dependencies: shared
single-select transition, radio roving focus, and id wiring.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onValueChange` | user selects an enabled available route | exact option id | never a provider label |
| `onQueryChange` | user edits or clears search | query string | fires after local state request |

## 6. Accessibility

### Semantics

- Root is a labelled section.
- Each group has a visible heading and radiogroup label association.
- Options use radio semantics with checked and disabled state.
- Availability has compact visible text and exposes the supplied full reason
  to assistive technology; colour and provider marks are supplementary.

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | enters search, then selected/first enabled option |
| `Arrow keys` | move through enabled options within the complete picker |
| `Home` / `End` | first/last enabled option |
| `Space` / `Enter` | select focused option |

### Focus And Announcement

- Query changes announce result count politely.
- Async state changes use PickerShell status text.
- Selecting an option does not move focus.
- GPUI later maps group/radio state through native accessibility APIs.

## 7. Layout

- Options use one column in narrow containers and may use two when each card
  retains its provider, route, and compact status without truncation.
- Descriptions remain searchable and available to assistive technology but do
  not occupy a third visual line. Group-derived badges are not part of options.
- The result region scrolls; the search field stays outside that scroll port.
- Provider marks are fixed-size; copy columns use `min-width: 0`.
- The component stretches to its parent; overlay width belongs to the host.

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Option | `color.background.surface`, `color.border.subtle` | quiet card |
| Selected option | `color.accent.base`, `color.accent.focusRing` | selection |
| Copy | `color.text.primary`, `color.text.secondary` | hierarchy |
| Leading | `size.icon.*`, `radius.control` | provider mark lane |
| Groups | `space.stack.*`, `space.inline.*` | responsive rhythm |

## 9. Svelte Notes

- Compose existing PickerShell/TextInput/Pill/StatusIndicator primitives.
- Use a keyed `{#each}` by opaque option id.
- `leading` is a snippet receiving the full display option.

## 10. GPUI Notes

- Implemented: `ModelConnectionPickerSpec`
  (`packages/contracts/components/src/model_connection_picker.rs`),
  `poodle_render::model_connection_picker`, GPUI specimen
  `packages/gpui/preview/src/specimens/model_connection_picker_specimen.rs`.
- Exact id selection, radio semantics, local filtering, and roving focus are
  preserved. Brand marks remain consumer-rendered content.

### Native Binding

- The spec carries controlled display data only. `value` and `query` are the
  current values; the web `defaultValue`/`defaultQuery` seeds are **web
  targets only** because GPUI/AppState owns the current value and rerenders
  after `on_value_change` / `on_query_change`.
- Filtering, grouping, selectability, shell-state resolution, result
  announcements, and posture copy derive once through
  `poodle_headless::model_connection`.
- The `leading` snippet becomes host-composed nodes keyed by option id
  (`ModelConnectionPickerSlots::leading`); the generic mark is the fallback,
  not a provider catalogue.
- Natives label by object, so an option's accessible name states provider,
  route, description, and the supplied availability reason once, where the web
  composes the same content from descendant text plus a visually-hidden line.
- Enter and Space select through the backend's own activation path; `on_key`
  carries only the arrow/Home/End roving moves, so Space is never bound twice.
- `ModelConnectionPickerHandlers::instance_id` is the backend-state scope. The
  semantic `id` stays readable and unscoped; the scope lives on `runtime_id`,
  which is what GPUI keys focus handles and editing state by. Without it two
  pickers offering the same routes would share one handle per option id.

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] filtering inputs and output order match
- [ ] exact-id selection and disabled guards match
- [ ] keyboard, focus, status, and event timing match

### Tier 2: Visual Parity

- [ ] hierarchy, option states, responsive columns, and token roles match

### Tier 3: Implementation Freedom

- [ ] framework rendering details do not leak into option data

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| — | — | no open deltas; native completion landed in `g15.008` | — |

## 13. Approval And Adoption Notes

- contract status: `approved`
- approver: operator, 2026-08-14
- downstream adopter: Nucleus
- native completion: landed in `g15.008` (Rust declaration, `poodle-headless` behaviour mirror, `poodle-render` composition, GPUI specimen and mounted evidence)
