# 067 Model Connection Management

Status: approved — web reference ready for dispatch
Updated: 2026-08-14
Consumers: Nucleus and other model-host applications
Related: `../contracts/components/model-picker.md`,
`../contracts/components/update-center.md`,
`../contracts/components/editable-list.md`

## Purpose

Define a small Poodle component family for adding, inspecting, enabling, and
curating configured model connections.

A connection is one configured route instance. It may represent:

- a hosted API or SDK
- an installed local harness
- a local model runtime or endpoint
- another configured instance of an already-listed provider

Visible product copy may still say "Providers". The component vocabulary uses
"connection" because one provider may expose several routes and several
configured instances.

## Authority Boundary

Poodle owns:

- controlled presentation components
- interaction and accessibility behavior
- safe display-state types
- list filtering, disclosure, and reorder mechanics
- Svelte and React reference implementations
- later active-runtime parity through the adopted conformance system

The consumer owns:

- provider, route, instance, and model truth
- discovery and machine-support probes
- credential collection, storage, OAuth launch, and revocation
- validation, installation, update, and refresh commands
- enabled state, ordering, visibility, defaults, and persistence
- mapping exact backend identities into Poodle display records

Poodle must not import Nucleus or Swallowtail. It must not accept credential
values as durable connection-summary data.

## Fixed Product Rules

- Keep enabled state separate from readiness. Turning a connection off does
  not revoke credentials, remove it, or change the last observed readiness.
- Keep exact configured instances distinct. Do not merge two Codex homes, two
  API accounts, or two routes from the same provider.
- Keep provider identity and route identity distinct in the host mapping.
- Keep unavailable connections visible with an honest reason.
- Treat update state as composed authority output. Do not derive it from
  connection readiness.
- Curating models changes their presentation in consumer pickers. It does not
  claim that a model or route is executable.
- A catalogue may be unavailable, empty, delayed, or negotiated only after a
  session starts. Those states are not interchangeable.
- Secrets, tokens, raw probe output, filesystem evidence, and opaque backend
  references never appear in closed-row props or specimens.

## Component Family

### `ModelConnectionPicker`

A searchable, grouped single-choice surface for exact connection routes.

Compose:

- `PickerShell`
- `TextInput` with search semantics
- `CardRadioGroup` or equivalent radio-card structure
- `Pill`, `StatusIndicator`, and `EmptyState`

Responsibilities:

- render host-ordered groups and options
- preserve exact opaque option ids
- show provider name, route label, short description, availability, and badges
- allow a host-supplied leading mark for provider branding
- support ready, checking, unavailable, unsupported, loading, error, empty,
  and no-results postures
- remain usable as an inline surface or inside a host-owned `Dialog`/`Drawer`

The picker does not choose a fallback route. When one provider has several
routes, each remains a distinct option. The host may group those options under
one provider heading.

Do not copy the fixed two-column logo grid from the reference screenshots. A
compact responsive card list scales better to Swallowtail's route count and
keeps route descriptions visible.

### `ModelConnectionSetup`

An adaptive controlled shell around route selection and route-specific setup.

Stages:

1. `choose` — render `ModelConnectionPicker`
2. `configure` — render the selected connection summary, host content,
   validation posture, and actions

There is no fixed Identity or Config step. The host content may contain:

- no fields plus an automatic local-harness check
- API-key or token fields
- an OAuth or device-code action and progress
- cloud identity guidance
- endpoint, binary, workspace, or model-path fields
- provider-specific advanced settings

Responsibilities:

- render consistent heading, back/cancel/add actions, pending state, and
  form-level feedback
- expose one configuration snippet/render prop
- accept host-owned `canSubmit`, `pending`, and safe error/success copy
- keep focus and announcements correct during stage changes and async checks

The host owns the outer overlay. The component works inline as well as inside
`Dialog` or `Drawer`.

### `ModelConnectionCard`

A controlled disclosure card for one configured connection.

Closed anatomy:

```text
[status] [provider mark] [instance/provider title + route/version]
         [safe access summary]
         [closed accessory] [disclosure] [enabled switch]
```

Open anatomy:

```text
[the same summary row]
[separator]
[host-owned settings content]
```

Responsibilities:

- keep disclosure and enabled `Switch` as separate focus targets
- show readiness through text and `StatusIndicator`, never colour alone
- show a sanitized access/auth summary
- allow multiple instances of the same provider to carry distinct labels
- provide a closed-accessory region intended for `UpdateCenter`
- provide an open details region for host-owned forms, access actions,
  diagnostics, and `ModelCatalogueEditor`
- support ready, checking, attention, unavailable, unknown, and error display
  postures without deriving backend policy

The whole row must not be one disclosure button because it contains the switch,
update trigger, and possible actions.

### `ModelCatalogueEditor`

A controlled curation surface for the models exposed through one configured
connection.

Responsibilities:

- render shown models as a keyboard- and pointer-reorderable list
- emit complete ordered-id updates
- hide a shown model without deleting backend catalogue truth
- render hidden models in a separate recoverable section
- restore hidden models to a host-defined insertion point
- expose model label, provider label where relevant, description, badges, and
  optional information action
- provide an optional custom-model action/region
- support loading, unavailable, empty, error, and session-negotiated states
- announce keyboard reorder and visibility changes

Move up/down actions remain available even if pointer drag is implemented.
Hidden rows are not assigned a meaningful display order.

This component does not own the default model or per-thread model options.
The consumer feeds the resulting visible ordered list into the existing
`ModelPicker`, whose capability axes already model route-specific options.

### No `ModelConnectionList`

Do not add a list shell only to stack cards. Consumers compose
`ModelConnectionCard` with `Stack`, `ListContainer`, a settings page, or their
own shell.

## Provisional Display Shapes

These shapes are contract input, not mirrors of a backend wire type.

```ts
type ModelConnectionOption = {
  id: string;
  providerLabel: string;
  routeLabel: string | null;
  description: string | null;
  group: string;
  keywords: string[];
  badges: { label: string; tone?: PillTone }[];
  availability:
    | "available"
    | "checking"
    | "unavailable"
    | "unsupported";
  availabilityLabel: string;
  isDisabled: boolean;
};

type ModelConnectionSummary = {
  id: string;
  title: string;
  providerLabel: string;
  routeLabel: string | null;
  version: string | null;
  accessSummary: string | null;
  readiness:
    | "ready"
    | "checking"
    | "attention"
    | "unavailable"
    | "unknown"
    | "error";
  readinessLabel: string;
  enabled: boolean;
};

type ModelCatalogueItem = {
  id: string;
  label: string;
  providerLabel: string | null;
  description: string | null;
  badges: { label: string; tone?: PillTone }[];
  visible: boolean;
  isDisabled: boolean;
};

type ModelCatalogueState =
  | "ready"
  | "loading"
  | "unavailable"
  | "empty"
  | "error"
  | "sessionNegotiated";
```

Provider marks remain component snippets/render props. Poodle's icon set cannot
be the authority for provider brand assets.

## Reference Flow

```text
Host Add action
  -> host Dialog/Drawer
  -> ModelConnectionSetup: choose
       -> ModelConnectionPicker
  -> ModelConnectionSetup: configure
       -> host credential/config/detection content
       -> host verifies and persists
  -> configured settings list
       -> ModelConnectionCard
       -> open details
            -> host settings/access actions
            -> ModelCatalogueEditor

New thread/composer
  -> existing ModelPicker
       -> host supplies visible ordered models for selected connection
```

## Visual Direction

- Quiet bordered cards rather than one undifferentiated settings slab.
- Strong provider mark and title; subdued route, version, and access copy.
- One-line closed rows where space permits, two-line summaries at normal
  settings widths, stacked controls only at narrow widths.
- Expanded content stays inside the same card with a subtle separator and
  inset sections.
- The route picker uses search plus host-defined groups. One column at narrow
  widths; two columns only when cards retain useful descriptions.
- Model rows use compact spacing, visible reorder affordance, and subdued
  utility actions. Hidden models live in their own collapsed section.
- Unavailable and disabled states remain readable, not washed out to the point
  of looking absent.

## Accessibility

- Exact connection choice uses radio-group semantics.
- Search result count and async detection changes use polite announcements.
- Setup errors identify the affected field or action and do not steal focus.
- Connection disclosure exposes `aria-expanded` and a labelled region.
- Disclosure, update trigger, secondary actions, and enabled switch are
  independent keyboard stops with non-overlapping hit targets.
- Reorder supports keyboard grab/move/drop and explicit move buttons.
- Reorder and hide/restore actions announce the model label and resulting
  position or visibility.
- Status always has visible or accessible text; colour and brand marks are
  supplementary.

## Specimen Matrix

### Picker

- many providers grouped across hosted, installed, and local-runtime routes
- one provider with several exact routes
- available, checking, unavailable, and unsupported options
- query with results and no results
- loading, error, and empty catalogue
- narrow and wide layouts

### Setup

- auto-detected local harness: found
- auto-detected local harness: missing
- API key form
- external OAuth pending and complete
- local endpoint configuration
- validation failure and retry
- async submit lock

All credential specimens use inert placeholders. No realistic token or account
identifier belongs in repository fixtures.

### Connection card

- ready and enabled
- ready and disabled
- checking
- needs attention
- unavailable with reason
- two instances of one provider
- closed `UpdateCenter` trigger
- open route-specific form and access actions
- narrow summary wrapping

### Model catalogue

- visible models reordered
- shown and hidden models
- duplicate display labels with distinct opaque ids
- mixed provider labels from one gateway
- custom-model action
- loading, unavailable, empty, error, and session-negotiated states
- keyboard move, hide, and restore announcements

## Delivery Shape

### Reference tranche

- approve four component contracts first
- add pure display and reorder helpers to `poodle-core`
- share one stylesheet family between Svelte and React
- implement matching Svelte and React components
- add matching specimen pages and focused interaction tests
- keep Rust, GPUI, Jetstream, Swallowtail, Nucleus, and Longhorn out of scope

### Active-runtime tranche

- starts only after g14.008 adopts the conformance pipeline
- authors shared cases/specimen structure once
- adds Rust declarations, shared renderer/node support, and GPUI execution
- keeps Jetstream deferred under the current g14 backend policy

The web reference may run beside the g14 pilot only through an explicit roadmap
exception equivalent to the licence reference lane. It must not touch
`packages/core/src/conformance/**`, Rust, native packages, or g14 evidence.

## Decisions

- Public component vocabulary: `ModelConnection*`. Product copy remains
  consumer-defined and may say "Providers".
- Setup: adaptive `choose` then `configure`; no generic stepper.
- Model settings: ordering and visibility only. Defaults, favourites, and
  option defaults remain consumer concerns.
- Delivery: dispatch the bounded Svelte/React web reference now. Native
  completion remains blocked until g14.008 adopts the pipeline.

## Promotion Record

The four component contracts were approved on 2026-08-14. Implementation stays
one complete worker handoff rather than component-by-component cards.
