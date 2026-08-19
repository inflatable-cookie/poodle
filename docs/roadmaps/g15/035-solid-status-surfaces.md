# g15.035 — Solid Tone Surfaces

Status: **complete** — PR #44 merged at `c24b19f4`; Pill's duplicate `fill`
axis is superseded by `g15.036`
Depends on: `g15.010` (complete active-cohort component closure)
Blocks: `g15.012`, `g15.013`
Parallel with: `g15.022` — no shared mutable implementation or specimen files
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/004-shared-control-types.md`,
`../../contracts/components/callout.md`,
`../../contracts/components/pill.md`,
`../../contracts/components/remediation-banner.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`

## Outcome

`Callout`, `RemediationBanner`, and `Pill` gain the same opt-in fill axis:

```ts
type ToneFill = "tint" | "solid";

fill?: ToneFill; // default: "tint"
```

`tint` preserves today's treatment. `solid` produces an opaque, high-contrast
tone-and-theme surface with primary foregrounds. Neutral is a real solid variant, not an
alias for info. The API, defaults, semantic output, and representative
specimens match in Svelte, React, renderer-neutral Rust, and GPUI.

This is additive public API on the pre-1.0 preview channel. It is not a new
component variant system and does not change announcement, dismissal, action,
layout, size, density, or Pill appearance behavior.

## Post-Merge Correction

PR #44 delivered this card as written. During closeout, the operator identified
that Pill already had a mutually exclusive appearance axis, making its new
`fill` prop duplicate public vocabulary. `g15.036` removes Pill `fill`, adds
`appearance="tint"` as the visual-preserving default, and assigns the opaque
treatment to the existing `appearance="solid"`. Callout and
RemediationBanner retain the `ToneFill` API delivered here. This section
records the supersession without rewriting the execution history below.

## Approved API And Visual Rule

- Add shared `ToneFill = "tint" | "solid"`; default `"tint"`.
- Add `fill` to all three components. Emit `data-fill` on web roots and carry
  the equivalent enum in all three Rust specs.
- `tint` retains the current fill, border, foreground, icon, spinner, action,
  and focus treatment. Do not use this card to restyle the existing variant.
- `solid` uses one shared tone-resolution rule:
  - neutral base: equal parts `color.text.secondary` and
    `color.background.surface`;
  - info/success/warning/danger base: the matching `color.status.*` token;
  - pending base: `color.accent.base`;
  - non-neutral solid background: an opaque sRGB mix of **40% tone base and
    60% `color.background.surface`**;
  - foreground: `color.text.primary` for title, message, icon, dismiss control,
    and pending spinner;
  - border: raw tone base for non-neutral; `color.border.strong` for neutral;
  - icon badge, where the component has one: a subtle primary-text overlay that
    keeps the existing badge geometry.
- Pill custom accents use the same formula with the custom accent as the tone
  base. Its dot and optional native remove affordance use primary foreground
  treatment in solid mode.
- Pill's existing `appearance="solid" | "subtle" | "badge"` axis is not
  renamed. It predates this fill axis: `appearance="solid"` remains the
  standard shell style, while `fill="solid"` requests an opaque tone fill.
  `fill="solid"` takes precedence over appearance-specific tint/opacity color
  recipes; badge typography remains, while subtle adds no opacity reduction in
  solid mode. This is an explicit precedence rule, not a silent fallback.
- Post-merge visual review replaced the original inverted 45/55 recipe. It
  washed dark themes toward white and light themes toward black. The 40/60
  tone/surface recipe keeps each theme on its own side of the contrast axis;
  current-theme evidence retains at least 4.7:1 normal-text contrast.
- Poodle `Button` compositions in either action area remain legible on the
  solid surface while honoring the action's supplied variant and disabled
  state. Use a local surface/foreground treatment; do not rewrite host action
  choices.
- Focus rings remain visible on every solid tone. Do not make focus depend on
  the tone matching the global accent.
- Accessibility roles, live-region behavior, labels, callbacks, and action
  payloads are unchanged.

## Measured Starting Point

- All three web components already share framework-neutral CSS with their
  paired runtime. Callout and RemediationBanner expose the same six
  `StatusTone` values; Pill exposes five `PillTone` values plus custom accent.
- `Callout` is generated-specimen-backed. Its authored display model and
  regenerated Svelte/React/Rust artifacts must carry `fill`; generated files
  are not edited by hand.
- `RemediationBanner` uses hand-written specimens and currently relies on the
  broad `WebParityCloseout` tests. This card gives it dedicated focused web
  evidence.
- `Pill` is generated-specimen-backed and already has focused paired web tests,
  a `PillSpec`, shared `poodle-render` composition, and a GPUI specimen. Its
  existing `appearance="solid"` name describes the standard shell, not the
  requested opaque color treatment; no breaking rename is authorised here.
- Shared Rust composition already resolves all three surfaces in `poodle-render`;
  GPUI consumes those nodes. No backend-only paint path is required.
- `CallOutSpec::default()` currently uses `StatusTone::Info`, while the
  contract and both web runtimes default to neutral. Correct it to neutral as
  part of this parity change.
- `RemediationBanner` already accepts `StatusTone::Neutral`, but its contract
  state table omits the neutral case. Make the contract and evidence complete.

## Delivery

### 1. Contract the shared axis first

- Define `ToneFill` once in `docs/contracts/004-shared-control-types.md`; name
  all three consumers and the TS and Rust authorities.
- Update all three component contracts before implementation. Record `fill`, its
  default, exact tint/solid token rules, neutral and pending behavior,
  foregrounds, composed actions, focus, runtime notes, and parity checklists.
- Remove Callout's redundant `StatusTone | "neutral"` wording: `StatusTone`
  already includes neutral. This is a documentation correction, not an API
  removal.
- Keep the component distinction intact: Callout remains contextual/passive;
  RemediationBanner remains action-primary and announcing by default; Pill
  remains non-interactive compact metadata.
- In Pill's contract, distinguish the legacy `appearance="solid"` label from
  the new fill axis and record the solid-over-appearance precedence exactly.

### 2. Deliver the web pair from shared CSS

- Export the shared TS type from both runtime type surfaces.
- Add the prop/default/data attribute to Svelte and React thin shells.
- Implement the visual treatment in `poodle-core/styles`, preserving the
  shared substrate. Do not duplicate framework-specific styles.
- Add recipe hooks only for real theme override seams. Hooks stay
  component-specific and use the established recipe-to-token fallback shape.
- Pending uses the shared Spinner with current/primary foreground in solid
  mode; tint mode remains accent.
- Prove all `6 tones × 2 fills`, defaults, attributes, semantics, actions,
  dismissal, disabled actions, and focus-visible behavior in focused paired
  tests. Add dedicated RemediationBanner test files instead of growing the
  unrelated closeout suite.
- Prove Pill's `5 tones × 2 fills`, custom accent, default, `data-fill`, dot,
  muted state, and `solid × appearance` precedence in its paired tests. Tint
  assertions must pin the existing CSS unchanged.

### 3. Deliver renderer-neutral Rust and GPUI

- Add `ToneFill` to the shared Rust type surface and add `fill` plus a builder
  to `CallOutSpec`, `RemediationBannerSpec`, and `PillSpec`.
- Default all three specs to `Tint`; also correct Callout's tone default to
  `Neutral`.
- Resolve the same 40/60 tone/surface sRGB solid fill, border, primary text/icon/spinner,
  action readability, and focus treatment in `poodle-render`.
- Add spec and render tests for the full matrices, default parity, token
  selection, contrast rule, callbacks, Pill appearance precedence, and
  unchanged accessibility output.
- Add representative solid examples to all three GPUI pages. Do not add a
  GPUI-only visual option or touch Jetstream preview integration.

### 4. Teach the option without rebuilding the catalogue

- Callout: extend the surviving display-specimen model with the presentation
  prop and regenerate its artifacts through `effigy ir:build`. Add one compact
  `Solid fills` group with representative neutral, warning, and danger
  examples; exhaustive tone coverage stays in tests.
- RemediationBanner: add a matching compact solid group to the hand-written
  Svelte, React, and GPUI specimens. Keep the existing realistic recovery
  examples and live action/dismiss behavior.
- Pill: extend the surviving display-specimen model with `fill`, regenerate
  its artifacts, and add one compact `Solid fills` group with representative
  neutral, success, warning, and custom-accent examples. Do not repeat its
  existing tones, sizes, or appearances as a matrix.
- Keep `Examples` human-centred. Do not add a fill matrix, repeat size/density
  rows, revive a Conformance tab, or extend the specimen model into behavior.
- The operator reviews all three changed pages live in Svelte and React before
  merge. Record the checkpoint honestly; the worker cannot self-approve it.

### 5. Record the release surface

The August batch log names:

- packages changed: `@inflatable-cookie/poodle-core`,
  `@inflatable-cookie/poodle-svelte`, `@inflatable-cookie/poodle-react`,
  `poodle-specs`, `poodle-render`, and preview-only consumers;
- public-entry-point impact: additive `ToneFill` plus `fill` on three
  components/specs; Callout's corrected Rust default;
- change class: additive public API plus a behavioral parity correction on the
  pre-1.0 preview channel;
- migration: none — tint remains the default;
- downstream re-check: consumers opt into `fill="solid"`; code relying on
  Rust Callout's incorrect implicit info tone must set `Info` explicitly.

## Acceptance

- [ ] One shared two-member type exists in both TS runtimes, Rust, and contract
      authority; all three components consume it without local restatement.
- [ ] Tint is the unchanged default in all three components and all active
      runtimes.
- [ ] Every solid tone uses the approved opaque fill/border/foreground rule;
      neutral and pending remain semantically distinct.
- [ ] Normal title/message/Pill text has at least 4.5:1 contrast for all twelve
      current themes; UI/icon boundaries meet the relevant 3:1 threshold.
- [ ] Solid action buttons, dismiss controls, spinners, disabled states, and
      focus rings remain readable and functional.
- [ ] Svelte and React behavior/API tests cover the full matrix and
      RemediationBanner has dedicated focused evidence; Pill covers custom
      accent and every appearance combination.
- [ ] Rust spec/render tests cover the same matrix and Callout defaults to
      neutral/tint.
- [ ] Pill tint behavior is unchanged; solid overrides appearance color
      recipes deterministically, badge typography survives, and custom accent
      stays contrast-safe.
- [ ] Svelte, React, and GPUI specimens teach representative solid usage
      without an exhaustive matrix.
- [ ] Generated specimen artifacts are regenerated from their authored model
      and `ir:check` is clean.
- [ ] The operator accepts all three changed web pages in the live paired previews.
- [ ] One batch log records change class, exact commands, contrast evidence,
      generated artifacts, and unresolved findings.

## Writable Scope

- `docs/contracts/004-shared-control-types.md`
- `docs/contracts/components/{callout,pill,remediation-banner}.md`
- Callout/Pill/RemediationBanner component shells, focused tests, and shared
  CSS in `packages/{svelte,react,core}`
- shared TS type surfaces and package-facing documentation generated from the
  component contracts where required by drift checks
- `packages/contracts/components/src/{types,call_out,pill,remediation_banner}.rs`
  and their exports/tests
- `packages/render/src/{callout,pill,remediation_banner}.rs` and focused tests
- the three named specimen pages across Svelte, React, and GPUI
- Callout and Pill authored display-specimen model entries, fixture, generated
  artifacts, and focused generator tests
- focused parity/contrast evidence for this exact three-component scope
- one August batch log
- append-only `PAPERCUTS.md` for new execution friction only

Do not edit the generation runway, dispatch ledger, another component's public
API, global status-tone values, token schema, catalogue shell/navigation,
Jetstream parity surfaces, release automation, or `.github/workflows/`.

## Validation

Run one coherent headless round after implementation:

- focused Svelte/React component tests for Callout, Pill, and RemediationBanner
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
- Pill needs a breaking `appearance` rename, an invalid-combination fallback,
  or a second component-specific fill type. Preserve the recorded precedence
  and report contrary evidence instead of widening the migration.
- The 40/60 tone/surface rule fails the contrast threshold in a current theme or cannot be
  reproduced by both shared CSS and `poodle-render`.
- Solid action readability requires changing Button's public API or global
  behavior rather than a local composition treatment.
- The change requires a new token family or changes global status-tone
  meaning; return the evidence to the orchestrator instead of widening scope.
- The generated Callout/Pill specimen path needs callbacks, conditions, or
  other behavior. Keep the scene presentation-only and report the boundary.
- A validation failure changes the API/visual plan rather than exposing an
  implementation defect inside this card.

## Continuation

Push one PR and stop for orchestrator review. Leave the paired live-preview
checkpoint open for the operator. This card may run beside `g15.022`; it must
land before `g15.012` and final certification. Do not absorb another specimen
family or advance the main runway from the worker.
