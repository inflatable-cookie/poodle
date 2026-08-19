# g15.035 — Solid Status Surfaces

Status: **ready** — API and visual rules approved by the operator on 2026-08-19
Depends on: `g15.010` (complete active-cohort component closure)
Blocks: `g15.012`, `g15.013`
Parallel with: `g15.021` — no shared mutable implementation or specimen files
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/004-shared-control-types.md`,
`../../contracts/components/callout.md`,
`../../contracts/components/remediation-banner.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`

## Outcome

`Callout` and `RemediationBanner` gain the same opt-in fill axis:

```ts
type StatusSurfaceFill = "tint" | "solid";

fill?: StatusSurfaceFill; // default: "tint"
```

`tint` preserves today's treatment. `solid` produces an opaque, high-contrast
tone surface with inverse foregrounds. Neutral is a real solid variant, not an
alias for info. The API, defaults, semantic output, and representative
specimens match in Svelte, React, renderer-neutral Rust, and GPUI.

This is additive public API on the pre-1.0 preview channel. It is not a new
component variant system and does not change announcement, dismissal, action,
layout, size, or density behavior.

## Approved API And Visual Rule

- Add shared `StatusSurfaceFill = "tint" | "solid"`; default `"tint"`.
- Add `fill` to both components. Emit `data-fill` on web roots and carry the
  equivalent enum in both Rust specs.
- `tint` retains the current fill, border, foreground, icon, spinner, action,
  and focus treatment. Do not use this card to restyle the existing variant.
- `solid` uses one shared tone-resolution rule:
  - neutral base: `color.text.primary`;
  - info/success/warning/danger base: the matching `color.status.*` token;
  - pending base: `color.accent.base`;
  - non-neutral solid background: an opaque sRGB mix of **45% tone base and
    55% `color.text.primary`**;
  - neutral solid background: `color.text.primary` directly;
  - foreground: `color.text.inverse` for title, message, icon, dismiss control,
    and pending spinner;
  - border: raw tone base for non-neutral; `color.border.strong` for neutral;
  - icon badge, where the component has one: a subtle inverse overlay that
    keeps the existing badge geometry.
- The 45/55 rule is deliberate. A read-only check over all twelve current
  themes found its worst normal-text contrast against `color.text.inverse`
  above 5:1; a tone-heavier 50/50 mix fell just below 4.5:1 in Clay. Preserve
  the cross-runtime sRGB formula and add durable contrast evidence rather than
  replacing it with an unproved raw status fill.
- Poodle `Button` compositions in either action area remain legible on the
  solid surface while honoring the action's supplied variant and disabled
  state. Use a local surface/foreground treatment; do not rewrite host action
  choices.
- Focus rings remain visible on every solid tone. Do not make focus depend on
  the tone matching the global accent.
- Accessibility roles, live-region behavior, labels, callbacks, and action
  payloads are unchanged.

## Measured Starting Point

- Both web components already share framework-neutral CSS with their paired
  runtime and expose the same six `StatusTone` values.
- `Callout` is generated-specimen-backed. Its authored display model and
  regenerated Svelte/React/Rust artifacts must carry `fill`; generated files
  are not edited by hand.
- `RemediationBanner` uses hand-written specimens and currently relies on the
  broad `WebParityCloseout` tests. This card gives it dedicated focused web
  evidence.
- Shared Rust composition already resolves both surfaces in `poodle-render`;
  GPUI consumes those nodes. No backend-only paint path is required.
- `CallOutSpec::default()` currently uses `StatusTone::Info`, while the
  contract and both web runtimes default to neutral. Correct it to neutral as
  part of this parity change.
- `RemediationBanner` already accepts `StatusTone::Neutral`, but its contract
  state table omits the neutral case. Make the contract and evidence complete.

## Delivery

### 1. Contract the shared axis first

- Define `StatusSurfaceFill` once in
  `docs/contracts/004-shared-control-types.md`; name both consumers and the TS
  and Rust authorities.
- Update both component contracts before implementation. Record `fill`, its
  default, exact tint/solid token rules, neutral and pending behavior,
  foregrounds, composed actions, focus, runtime notes, and parity checklists.
- Remove Callout's redundant `StatusTone | "neutral"` wording: `StatusTone`
  already includes neutral. This is a documentation correction, not an API
  removal.
- Keep the component distinction intact: Callout remains contextual/passive;
  RemediationBanner remains action-primary and announcing by default.

### 2. Deliver the web pair from shared CSS

- Export the shared TS type from both runtime type surfaces.
- Add the prop/default/data attribute to Svelte and React thin shells.
- Implement the visual treatment in `poodle-core/styles`, preserving the
  shared substrate. Do not duplicate framework-specific styles.
- Add recipe hooks only for real theme override seams. Hooks stay
  component-specific and use the established recipe-to-token fallback shape.
- Pending uses the shared Spinner with current/inverse foreground in solid
  mode; tint mode remains accent.
- Prove all `6 tones × 2 fills`, defaults, attributes, semantics, actions,
  dismissal, disabled actions, and focus-visible behavior in focused paired
  tests. Add dedicated RemediationBanner test files instead of growing the
  unrelated closeout suite.

### 3. Deliver renderer-neutral Rust and GPUI

- Add `StatusSurfaceFill` to the shared Rust type surface and add `fill` plus a
  builder to `CallOutSpec` and `RemediationBannerSpec`.
- Default both specs to `Tint`; also correct Callout's tone default to
  `Neutral`.
- Resolve the same 45/55 sRGB solid fill, border, inverse text/icon/spinner,
  action readability, and focus treatment in `poodle-render`.
- Add spec and render tests for the full matrix, default parity, token
  selection, contrast rule, callbacks, and unchanged accessibility output.
- Add representative solid examples to both GPUI pages. Do not add a
  GPUI-only visual option or touch Jetstream preview integration.

### 4. Teach the option without rebuilding the catalogue

- Callout: extend the surviving display-specimen model with the presentation
  prop and regenerate its artifacts through `effigy ir:build`. Add one compact
  `Solid fills` group with representative neutral, warning, and danger
  examples; exhaustive tone coverage stays in tests.
- RemediationBanner: add a matching compact solid group to the hand-written
  Svelte, React, and GPUI specimens. Keep the existing realistic recovery
  examples and live action/dismiss behavior.
- Keep `Examples` human-centred. Do not add a fill matrix, repeat size/density
  rows, revive a Conformance tab, or extend the specimen model into behavior.
- The operator reviews both changed pages live in Svelte and React before
  merge. Record the checkpoint honestly; the worker cannot self-approve it.

### 5. Record the release surface

The August batch log names:

- packages changed: `@inflatable-cookie/poodle-core`,
  `@inflatable-cookie/poodle-svelte`, `@inflatable-cookie/poodle-react`,
  `poodle-specs`, `poodle-render`, and preview-only consumers;
- public-entry-point impact: additive `StatusSurfaceFill` plus `fill` on two
  components/specs; Callout's corrected Rust default;
- change class: additive public API plus a behavioral parity correction on the
  pre-1.0 preview channel;
- migration: none — tint remains the default;
- downstream re-check: consumers opt into `fill="solid"`; code relying on
  Rust Callout's incorrect implicit info tone must set `Info` explicitly.

## Acceptance

- [ ] One shared two-member type exists in both TS runtimes, Rust, and contract
      authority; both components consume it without local restatement.
- [ ] Tint is the unchanged default in both components and all active
      runtimes.
- [ ] Every solid tone uses the approved opaque fill/border/foreground rule;
      neutral and pending remain semantically distinct.
- [ ] Normal title/message text has at least 4.5:1 contrast for all twelve
      current themes; UI/icon boundaries meet the relevant 3:1 threshold.
- [ ] Solid action buttons, dismiss controls, spinners, disabled states, and
      focus rings remain readable and functional.
- [ ] Svelte and React behavior/API tests cover the full matrix and
      RemediationBanner has dedicated focused evidence.
- [ ] Rust spec/render tests cover the same matrix and Callout defaults to
      neutral/tint.
- [ ] Svelte, React, and GPUI specimens teach representative solid usage
      without an exhaustive matrix.
- [ ] Generated specimen artifacts are regenerated from their authored model
      and `ir:check` is clean.
- [ ] The operator accepts both changed web pages in the live paired previews.
- [ ] One batch log records change class, exact commands, contrast evidence,
      generated artifacts, and unresolved findings.

## Writable Scope

- `docs/contracts/004-shared-control-types.md`
- `docs/contracts/components/{callout,remediation-banner}.md`
- Callout/RemediationBanner component shells, focused tests, and shared CSS in
  `packages/{svelte,react,core}`
- shared TS type surfaces and package-facing documentation generated from the
  component contracts where required by drift checks
- `packages/contracts/components/src/{types,call_out,remediation_banner}.rs`
  and their exports/tests
- `packages/render/src/{callout,remediation_banner}.rs` and focused tests
- the two named specimen pages across Svelte, React, and GPUI
- Callout's authored display-specimen model, fixture, generated artifacts, and
  focused generator tests
- focused parity/contrast evidence for this exact two-component scope
- one August batch log
- append-only `PAPERCUTS.md` for new execution friction only

Do not edit the generation runway, dispatch ledger, another component's public
API, global status-tone values, token schema, catalogue shell/navigation,
Jetstream parity surfaces, release automation, or `.github/workflows/`.

## Validation

Run one coherent headless round after implementation:

- focused Svelte/React component tests for Callout and RemediationBanner
- focused Rust spec/render and contrast tests
- focused paired specimen/parity evidence
- `effigy ir:check`
- `effigy test:components`
- `effigy check:svelte`
- `effigy react:build`
- `effigy test:parity`
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy test:web-pack-install`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Never run a `*-windowed`, native-visual, conformance,
Jetstream, or release selector.

## Stop Conditions

- The approved shared axis cannot be expressed without a third value, a
  component-specific semantic fork, or a compatibility alias.
- The 45/55 rule fails the contrast threshold in a current theme or cannot be
  reproduced by both shared CSS and `poodle-render`.
- Solid action readability requires changing Button's public API or global
  behavior rather than a local composition treatment.
- The change requires a new token family or changes global status-tone
  meaning; return the evidence to the orchestrator instead of widening scope.
- The generated Callout specimen path needs callbacks, conditions, or other
  behavior. Keep the scene presentation-only and report the boundary.
- A validation failure changes the API/visual plan rather than exposing an
  implementation defect inside this card.

## Continuation

Push one PR and stop for orchestrator review. Leave the paired live-preview
checkpoint open for the operator. This card may run beside `g15.021`; it must
land before `g15.012` and final certification. Do not absorb another specimen
family or advance the main runway from the worker.
