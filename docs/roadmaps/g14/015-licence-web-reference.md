# g14.015 — Licence Web Reference

Status: landed — merge `fd32d7d5`
Posture: complete
Review correction: g14.016 supersedes this card's equal-route activation model
with required `key | account` modes; this card remains the delivery record.
Depends on: approved component contracts for `LicenceStatus`,
`LicenceActivation`, and `LicenceSeats`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/licence-status.md`,
`../../contracts/components/licence-activation.md`,
`../../contracts/components/licence-seats.md`, Longhorn contract 019

## Outcome

Deliver the Svelte and React reference implementations for three
authority-agnostic licence components. Share types, display derivation, tests,
and CSS through `poodle-core`. Leave an explicit incomplete native/conformance
row for g14.017.

This card may run in parallel with g14.001 because it does not touch Button,
conformance kernel/codegen, Rust, native previews, or Effigy task definitions.
The second PR to merge must rebase over shared package indexes, preview
registries, and generated reports.

## Goals

- [ ] Render held licence state accurately without enforcing entitlement.
- [ ] Make key, account-token, and licence-file activation equal routes.
- [ ] Make release of other seats self-service without inventing machine
      identity.
- [ ] Keep Longhorn behaviour injected and the package graph Longhorn-free.
- [ ] Produce Svelte/React parity evidence and honest incomplete-native status.

## Execution Plan

### Batch A — Shared core contract

- [ ] Add `packages/core/src/licence.ts` with the exact structural types from
      the three component contracts.
- [ ] Export `LICENCE_MIRROR_FIELDS` and `LICENCE_MIRROR_VARIANT_FIELDS` for a
      Longhorn-owned adapter test to compare against its generated
      `LICENCE_FIELDS` / `LICENCE_VARIANT_FIELDS`.
- [ ] Add pure `licenceStatusView`, activation-submit/validation, and seat-row
      derivations. Copy and tone decisions live once here.
- [ ] Test every usability state, both trust bases, window combinations,
      typo-vs-too-short copy, route credentials, label normalization, and seat
      row rules.

No Poodle source or test imports Longhorn. Do not copy the Longhorn key parser
or normalization algorithm. `LicenceActivation` receives a `keyFormat` adapter
and an account-token provider whose functions the host supplies.

The mirror maps cover only shapes Poodle actually mirrors: seat fields plus
the usability, trust-basis, and credential variants. Do not invent a wire
`LicenceStatus` record for the flattened controller reads (`usable`,
`attention`, `useUntil`, `updateUntil`), and do not mirror unrelated HeldLicence
fields merely to make an exact-map comparison convenient. The downstream
Longhorn adapter owns the explicit name mapping and controller-view proof.

### Batch B — Web components

- [ ] Build `LicenceStatus` in Svelte and React over the shared view.
- [ ] Build `LicenceActivation` in Svelte and React with equal visible routes,
      injected key validation and account-token acquisition, single-file
      base64 reading, optional machine label, pending state, and one structural
      `onActivate` event. Account activation is a host flow, not a token field.
- [ ] Build `LicenceSeats` in Svelte and React with honest unnamed rows,
      current-machine marker, warning confirmation, and `onRelease` for other
      seats only.
- [ ] Add one shared `licence.css` or tightly related core stylesheet set.
      Reuse Poodle primitives; do not hand-roll Tabs, fields, upload controls,
      status indicators, buttons, or confirmation dialogs.
- [ ] Keep interface/default/callback parity exact between the two frameworks.

### Batch C — Specimens, tests, and packaging

- [ ] Add standalone Svelte and React specimens with matching structure and
      fixture data. This is pre-conformance duplication, kept deliberately
      small and replaced by g14.017 shared cases.
- [ ] Add component tests for all acceptance cases and accessible semantics.
- [ ] Register contracts, exports, component fixtures, specimens, package
      entry points, reports, and audit counts through the existing new-component
      checklist.
- [ ] Run packed-consumer proof so no source import points into a preview or
      outside the published package.
- [ ] Add one batch log with exact files, tests, known incomplete surfaces, and
      the public mirror-map names Longhorn must assert downstream.

## Fixed Decisions

- `LicenceCentre` is not built. Licence state changes too rarely for a durable
  shell indicator; actionable state already belongs in settings.
- `inGrace` is neutral. A failed renewal inside the lease is not customer
  action and never gets warning/error styling.
- `clockRefused` tells the operator to fix the clock. It never says expired,
  invalid, revoked, or buy/renew.
- Use and update coverage are separate visible rows.
- `usable` and `attention` are supplied reads. Components report them; they do
  not derive feature permissions.
- The key parser and typo predicate are injected through `keyFormat`; Poodle
  sends the raw accepted key and never normalizes it again.
- Account activation invokes an injected provider. Poodle never renders a
  token-paste field or owns the browser/account flow.
- All activation routes remain visible peers. The default selected tab does
  not make Key visually primary.
- Empty `seats` renders nothing. Raw or shortened machine IDs never render.
- Web reference is staged delivery, not component completion. Do not describe
  these components as parity-reviewed or four-runtime complete.

## Acceptance Criteria

- [ ] Each of the five usability states has a Svelte and React test and a
      specimen case.
- [ ] `inGrace` uses no warning/danger class, token role, icon, or announcement.
- [ ] `clockRefused` copy contains the clock remedy and no expiry/purchase copy.
- [ ] `useUntil` and `updateUntil` remain two visible labelled values.
- [ ] Changing `usable` never disables, hides, locks, or removes a feature
      control.
- [ ] Mistyped and too-short keys render distinct messages; typo copy never
      implies the key is invalid or not real.
- [ ] The injected parser receives lowercase, dashes, whitespace, and I/L/O
      input unchanged.
- [ ] Key, account token, and licence file are equally visible and keyboard
      reachable.
- [ ] Account activation invokes the injected provider; cancellation is quiet,
      and no account-token input is rendered.
- [ ] All routes emit exact credential shapes plus `label: string | null`.
- [ ] File bytes emit base64 without a data-URL prefix; secrets/contents never
      render or enter attributes/logs.
- [ ] Empty seats renders no seat UI; unnamed seats say `Unnamed machine`;
      this machine has no release action; every other seat is releasable.
- [ ] No rendered/accessibility string exposes a machine ID, hostname,
      platform, or last-seen value.
- [ ] No manifest, source, test, or generated artifact imports
      `@inflatable-cookie/longhorn` or points at the Longhorn checkout.
- [ ] Mirror field maps are exported for the downstream Longhorn assertion;
      the PR does not falsely claim that cross-repo gate already exists.
- [ ] Svelte/React public props, callbacks, copy, anatomy, CSS, specimens, and
      interaction tests agree.
- [ ] g14's estate continues to list native/conformance completion under
      g14.017.

## Stop Conditions

- The worker needs a licensing policy, entitlement name, purchase model,
  renewal URL, server response, or account-flow design not in the contracts.
- A route needs to be hidden, labelled advanced, or made secondary.
- Key validation would require importing or recreating Longhorn behaviour.
- Account activation would require Poodle to own browser/login behaviour or
  expose a token-paste field.
- The component would disable/hide another feature from `usable`, attention,
  entitlement, or seat state.
- The worker must edit conformance/codegen, `tasks/effigy.tasks.toml`, Rust,
  GPUI, Jetstream, Button, or another component's public contract.
- A framework needs different copy, prop meaning, callback payload, or fixture
  data to pass.

Stop with evidence and options. Do not widen the suite around the finding.

## Writable Scope

- the three licence component contracts only for discovered contradictions
- `packages/core/src/licence.ts`, its export, tests, and licence styles
- the three Svelte/React component files, indexes, and tests
- the three Svelte/React specimen files and existing registries
- component fixture/a11y/parity registrations and regenerated report artifacts
- package docs/exports needed for public install
- one August log and append-only `PAPERCUTS.md`

Do not edit `docs/roadmaps/**`, architecture/specs, dispatch, Effigy tasks,
codegen/conformance, Rust/native sources, release workflows, or Longhorn.

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

Record pre-existing failures separately. No baseline or exception-list edit may
turn a new licence failure green.

## Handoff

Open one PR. Include the core mirror/view API, public component API table,
specimen screenshots for both web runtimes, per-acceptance test mapping,
package-install result, and the exact remaining native/downstream assertions.
