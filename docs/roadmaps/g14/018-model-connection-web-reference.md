# g14.018 — Model Connection Web Reference

Status: landed — merge `48c6ec37`
Posture: complete; operator review continues under g14.019
Depends on: approved spec 067 and the four model-connection contracts
Governing refs: `../../specs/067-model-connection-management.md`,
`../../contracts/components/model-connection-picker.md`,
`../../contracts/components/model-connection-setup.md`,
`../../contracts/components/model-connection-card.md`,
`../../contracts/components/model-catalogue-editor.md`

## Outcome

Deliver the Svelte and React reference suite for choosing, configuring,
inspecting, enabling, and curating model connections. Share display types,
pure transitions, filtering, reorder behavior, fixtures, and CSS where their
runtime allows. Leave native/conformance completion explicit under g14.020.

This card may run beside the g14 conformance pilot. It must not touch
`packages/core/src/conformance/**`, generated conformance artifacts, Rust,
GPUI, Jetstream, Effigy task definitions, or g14 evidence. The second PR to
merge must rebase shared indexes and preview registries.

## Goals

- [ ] Scale exact route selection beyond a small logo grid.
- [ ] Keep route-specific credentials, detection, and config host-rendered.
- [ ] Keep closed connection readiness, auth summary, update access,
      disclosure, and enable state distinct.
- [ ] Let operators reorder shown models and hide/restore models accessibly.
- [ ] Produce matching inspectable Svelte and React specimen pages.

## Execution Plan

### Batch A — Shared types and machinery

- [ ] Add a focused `packages/core/src/model-connection.ts` surface or a
      comparably small split. Export the approved display types only.
- [ ] Implement and test case-folded picker filtering across provider, route,
      description, group, and keywords while retaining host source order.
- [ ] Implement and test setup-stage guards and exact-id submit intent.
- [ ] Reuse existing single-select, disclosure, switch, reorder, and list-key
      machinery. Do not copy it into component-local helpers.
- [ ] Add pure shown-order and visibility request helpers for
      `ModelCatalogueEditor`. Hidden-item order has no meaning.
- [ ] Add one shared stylesheet family imported by both web packages.

Do not mirror Nucleus or Swallowtail wire types. Poodle receives presentation
records with opaque ids and safe labels. No type includes a credential,
credential reference, raw probe output, filesystem evidence, executable
handle, target, or account identifier.

### Batch B — Components

- [ ] Build `ModelConnectionPicker` in Svelte and React over PickerShell,
      TextInput, Badge, StatusIndicator, and radio-card semantics.
- [ ] Build `ModelConnectionSetup` as the approved adaptive choose/configure
      shell. It does not own Dialog/Drawer or provider form schemas.
- [ ] Build `ModelConnectionCard` with independent disclosure, closed
      accessory, actions, and Switch focus targets. Demonstrate UpdateCenter
      through the closed accessory.
- [ ] Build `ModelCatalogueEditor` with shown and hidden sections, pointer and
      keyboard reorder, explicit move buttons, hide/restore, focus following,
      and polite announcements.
- [ ] Keep props, defaults, callback timing, copy, and anatomy aligned between
      Svelte and React.

Do not add `ModelConnectionList`. Consumers stack cards through existing layout
components.

### Batch C — Specimens and packaging

- [ ] Add one standalone specimen page per new component in both previews.
- [ ] Share fixture data and pure derivation through core. Keep the temporary
      framework specimen wrappers structurally matched and small; g14.020
      replaces them with shared executable cases.
- [ ] Cover every specimen family named in spec 067, including multiple exact
      routes from one provider and two configured instances of one provider.
- [ ] Use inert credential placeholders only. Never include realistic secrets,
      account identifiers, paths from the worker machine, or provider output.
- [ ] Register contracts, exports, component docs, specimens, accessibility
      fixtures, parity checks, package entry points, and packed-install proof.
- [ ] Add one August batch log with public APIs, screenshots, focused evidence,
      and the remaining g14.020 work.

## Fixed Decisions

- Public component vocabulary is `ModelConnection*`; visible product copy is
  consumer-defined and may say "Providers".
- One option id identifies one exact route. No provider-level fallback or
  automatic route choice exists in Poodle.
- Setup starts at `choose`; routes with `requiresConfiguration=false` submit
  directly, while other routes enter `configure`. Provider-specific content is
  a snippet/render prop; no generic field schema or arbitrary stepper is added.
- Enabled state is independent of readiness, auth, update, and deletion.
- `UpdateCenter` is composed through the card's closed accessory. The card does
  not mirror UpdateCenter's authority props.
- Provider brand marks are host content. Poodle supplies a generic fallback,
  not a provider-logo registry.
- Model curation owns shown order and visibility only. It does not own a
  default, favourite, route support claim, or per-thread option default.
- Hidden models live in a separate recoverable section and have no order.
- The existing `ModelPicker` remains the per-thread model/options control.
- Web reference is staged delivery, not active-runtime completion.

## Acceptance Criteria

- [ ] Picker filtering is deterministic, retains source/group order, and
      selects only exact enabled available ids.
- [ ] Loading, error, empty, no-results, checking, unavailable, and unsupported
      picker postures are visibly distinct and accessible.
- [ ] Setup cannot continue without a selectable connection or submit without
      host `canSubmit`; direct routes skip configure and pending guards duplicate actions.
- [ ] Setup can render auto-detection, API key, OAuth, local endpoint, and
      validation fixtures without Poodle receiving credential values.
- [ ] Closed cards show safe readiness and access copy; UpdateCenter,
      disclosure, actions, and Switch are separate focus targets.
- [ ] Disabling a connection emits only enabled-state intent and does not alter
      readiness, open state, auth copy, or update state.
- [ ] Opening/closing preserves the documented focus behavior and host detail
      content never becomes part of summary props.
- [ ] Reorder emits the complete shown-id order through pointer, keyboard, and
      explicit move actions.
- [ ] Hide/restore emits only `{ id, visible }`; no backend model is deleted and
      the host controls restored position.
- [ ] Shown, hidden, loading, unavailable, empty, error, and
      session-negotiated model postures remain distinct.
- [ ] Reorder and visibility announcements name the model and outcome; focus
      follows moved/hidden rows as documented.
- [ ] Svelte and React public surfaces, tests, specimen cases, and shared CSS
      agree.
- [ ] Packed consumers import every component from public package entries.
- [ ] g14 estate and reports keep Rust/GPUI completion visibly open under 020.

## Stop Conditions

- A worker needs Swallowtail to pick a route, infer fallback, expose a secret,
  or flatten its readiness dimensions into Poodle policy.
- A worker needs Nucleus persistence, credential storage, discovery, OAuth,
  installation, refresh, or update authority inside Poodle.
- Provider-specific form fields are proposed as a Poodle schema or union.
- A fixed Driver/Identity/Config wizard or arbitrary step engine is proposed.
- Defaults, favourites, entitlement enforcement, or model option policy enter
  `ModelCatalogueEditor`.
- Reorder forks existing core machinery or pointer drag becomes the only move
  path.
- The work touches conformance/codegen, task manifests, Rust, GPUI, Jetstream,
  Nucleus, Swallowtail, Longhorn, release workflows, or another component's
  public contract.
- Svelte and React need different semantics or fixture truth.

Stop with evidence and options. Do not widen the suite around the finding.

## Writable Scope

- the four model-connection contracts only for discovered contradictions
- `packages/core/src/model-connection.ts` or a tightly bounded equivalent,
  its public export, focused tests, and shared CSS
- the four Svelte and four React component files, indexes, and focused tests
- the four matching Svelte and React specimen files and existing registries
- component fixture/a11y/parity registrations and regenerated web reports
- package docs/exports required for public installation
- one August batch log and append-only `PAPERCUTS.md`

Do not edit `docs/roadmaps/**`, architecture/specs, dispatch state,
`packages/core/src/conformance/**`, Effigy tasks, codegen, Rust/native sources,
external repositories, or release workflows.

## Validation

Use existing selectors only:

- `effigy test:core`
- `effigy test:components`
- `effigy test:parity`
- `effigy test:a11y`
- `effigy check:svelte`
- `effigy svelte:surface-audit`
- `effigy docs:contract-drift`
- `effigy docs:callback-drift`
- `effigy docs:react-specimen-drift`
- `effigy test:web-pack-install`
- `effigy ci:web`
- `effigy docs:check`
- `git diff --check`

Use `effigy test --plan` before changing validation shape. Record pre-existing
failures separately. Do not edit a baseline or exception list to make new work
green.

## Handoff

Open one PR. Include:

- shared core API and helper table
- Svelte/React public prop and callback comparison
- both-runtime screenshots for all four pages
- acceptance-to-test mapping
- package-install result
- exact g14.020 native/shared-case remainder

Do not edit roadmap status. The orchestrator reviews, merges, and records the
completion evidence.
