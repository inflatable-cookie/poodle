# g16.034 — Shared Motion Policy And Five-Family Pilot

Status: implemented — production repair complete; final evidence in execution log
Opened: 2026-09-01
Depends on: merged planning intake PR #121 and promoted architecture 012;
`g16.033` is complete
Governing refs: `../../architecture/012-semantic-motion-policy.md`,
`../../architecture/010-native-presentation-construction-context.md`,
`../../contracts/components/motion-policy-provider.md`,
`../../contracts/001-working-rules.md`
Decision evidence: PR #121, merged as `7f718dd42`

## Goal

Deliver one explicit full/reduced/frozen host motion policy across web core,
Svelte, React, shared Rust composition, and GPUI. Prove the common lifecycle on
five bounded families: disclosure, transient notification, Tabs selection,
discrete state, and loading/reveal.

This is one policy-substrate and pilot card. It is not a transition catalogue,
general animation framework, renderer-capability programme, or visual-parity
claim.

## Ordered Work

1. Add the closed `MotionPolicy` type and paired framework-free TypeScript/Rust
   laws: missing preference, restriction-only nesting, stable semantic keys,
   initial endpoint, interruption, reduced filtering, frozen endpoints, and
   terminal cleanup.
2. Implement `MotionPolicyProvider` in Svelte and React. Add the stable inherited
   web hook and ensure `UiPresentationProvider` scopes preserve motion.
3. Add `MotionPolicy` to native `RenderContext` and migrate every constructor,
   composite child-builder, preview facade, test helper, and in-repository
   consumer atomically. Add the construction-time motion provider without
   provider metadata in `Node`.
4. Implement role/lifecycle helpers over the existing property vocabulary.
   Retain explicit GPUI channels: generic opacity and SVG rotation. Name every
   translation/scale or disclosure-layout approximation; do not silently claim
   parity.
5. Migrate Accordion and Collapsible: initial endpoint, full clipped-height and
   indicator motion, proportional reversal, reduced/frozen immediate layout,
   and cleanup.
6. Migrate ToastStack: keyed enter/update/exit, immediate live/accessibility
   ownership, inert exit remnants, policy changes, no focus theft, and exact
   focused-dismissal fallback. Motion never owns expiry.
7. Replace Tabs' selected-item underline border with one measured indicator for
   `activeEdge="underline"`. Prove stable semantic retarget, first-layout snap,
   environmental remeasurement snap, horizontal/vertical geometry,
   reduced/frozen snap, and teardown. Outline/fill treatments stay unchanged.
8. Migrate Checkbox and same-slot IconButton state changes. Semantic and ARIA
   state commit immediately; full may use the accepted property budget,
   reduced uses opacity only, frozen paints the endpoint, and repeated targets
   do not restart.
9. Normalize Skeleton to the 1.6s opacity pulse and retain Spinner's rotation/
   opacity variants. Loops start after the first committed frame in full;
   reduced/frozen and `Skeleton.animated=false` schedule none. Wire the
   production GPUI preview through `Window::on_next_frame` and prove the
   mounted loading routes commit before starting full-mode loops. Prove the
   host-owned loading-to-content boundary.
10. Add the layered pilot evidence, update exact contracts/log/currentness, run
    the full headless boards, falsify every oracle row, and close the card.

## Acceptance

- A host can set one effective `full`, `reduced`, or `frozen` policy for Svelte,
  React, and native composition; absence resolves to full.
- A descendant can restrict but never re-enable motion. Presentation scopes do
  not reset policy.
- Semantic state, ARIA/accessibility state, focus, labels, announcements, and
  component timers remain immediate and mode-independent.
- Stable semantic owner + role + channel keys survive rebuilds without sibling
  collision. Repeated targets are inert; latest target wins; reversible binary
  phases use proportional remaining duration.
- Reduced schedules only explicitly allowed short opacity; frozen schedules no
  visual clocks and paints settled/canonical static frames.
- Abort, unmount, policy tightening, delayed exit, observer teardown, timer,
  rAF/WAAPI, and native clocks leave no live owner.
- Disclosure, ToastStack, Tabs, Checkbox/IconButton, and Skeleton/Spinner each
  pass the exact family oracle below in Svelte and React plus the shared Rust/
  GPUI evidence their mechanism requires.
- GPUI approximations are named and observable. Unsupported translation,
  scale, or layout never becomes a visual-parity claim.
- No Dialog state, icon morphing, shimmer, block slider, arbitrary easing,
  blur/path/filter/canvas/3D system, Jetstream admission, or permanent
  conformance authority enters the diff.
- No established parity-ledger cell moves. The additive public
  `MotionPolicyProvider` export is reflected by the live 176 public / 175
  portable denominator (with `MeterSurface` fixed web-only / n/a); this card
  adds semantic/lifecycle evidence and explicit capability gaps, not broad
  visual or accessibility completion.

## Review Oracle

| Invariant | Smallest adversarial counterexample | Required proof |
| --- | --- | --- |
| Policy is explicit and restriction-only | root reduced, nested presentation scope, child requests full | Svelte, React, TypeScript, Rust, and native construction all resolve reduced; deleting restriction or presentation preservation fails |
| Initial state is not invented motion | default-open disclosure, preloaded toast, selected tab, checked box, loading loop | one-shots paint endpoints; preloaded items do not enter; full loading loop starts only after the baseline frame |
| Latest semantic state owns motion | open → close reversal; tab A → B → C; checkbox repeated current value | proportional reversal, one retargeted indicator, and no restart/queue, proved in paired pure traces and mounted receipts |
| Reduced and frozen differ honestly | switch full → reduced → frozen during active one-shot and loop | disallowed properties drop; reduced may finish allowed opacity; frozen has zero clocks and latest endpoint/canonical loop frame |
| Cleanup is exact | unmount or tighten policy while observer, exit remnant, browser clock, or GPUI clock is live | zero live handles and no late callback/paint mutation in each runtime |
| Disclosure exception stays bounded | animate width or generic layout because height exists | only Accordion/Collapsible clipped block-axis height is admitted; reduced/frozen and native approximation remain exact |
| Toast semantics do not ride visual lifecycle | dismiss focused danger toast during enter, reorder, then expire | one announcement, immediate inert removal, deterministic focus fallback, unchanged expiry, no focus theft |
| Tabs indicator follows semantic selection, not environment | resize/font/orientation/overflow change during A → B | environmental change cancels and snaps latest endpoint; only stable-geometry selection animates |
| Discrete semantics precede paint | interrupt checkbox mixed/checked or IconButton pressed/loading swap | checked/pressed/busy/label state is immediate in both web shells and native; visual completion emits nothing |
| Loading loops obey policy | animated Skeleton and each Spinner variant under nested reduced/frozen | full-only pulse/rotation/phasing, static reduced/frozen frame, no loop after teardown, Spinner status semantics retained |
| Native gaps stay visible | translation/scale declaration reaches GPUI unsupported path | headless probe records named approximation or static endpoint and fails any silent exact-parity classification |

Before closeout, plant the smallest pre-fix behavior for every row, confirm the
named proof fails for the intended reason, restore, and rerun green. Commit the
real proof before planting so restoration cannot discard it.

## Evidence Shape

- Paired TypeScript/Rust trace tables own shared policy laws.
- Focused mounted Svelte and React cases own real component lifecycle and DOM
  semantics.
- Rust render assertions own effective policy, keys, declared properties,
  endpoints, loop declarations, and frozen absence.
- Headless GPUI regressions own supported channels, construction propagation,
  static endpoints, cleanup, and named approximations.
- One bounded browser probe owns disclosure geometry, Tabs measurement/resize,
  toast focus, and live-region non-reannouncement.
- Deterministic captures cover only static endpoints and canonical frozen loop
  frames. They do not prove reduced-motion behavior.

Do not create an exhaustive family × mode × runtime corpus or a new generated
interface.

Production repair evidence:

- `packages/core/test/motion-runtime.test.ts` proves natural clipped-height
  completion removes the exact live handle.
- `packages/svelte/components/src/disclosure-motion.ts` and
  `packages/react/components/src/disclosure-motion.ts` are wired into the
  actual Accordion/Collapsible paths; mounted rapid controlled reversal cases
  use a live clipped height and proportional remaining duration.
- `packages/svelte/components/test/ssr/ToastStackSsr.test.ts` proves authored
  preloaded ToastStack items are present and settled on the first/SSR paint.
- `production_loading_routes_commit_before_starting_full_mode_loops` in
  `packages/gpui/preview/src/specimen_probe.rs` mounts the real PreviewRoot
  Skeleton/Spinner routes and proves full-mode loops start only after the
  production first-frame commit path.
- The mounted React close receipt keeps controlled prop-driven content visible
  until the close remnant exists; the accepted PR #125 Tabs drag/drop handlers
  remain unchanged.

## Writable Scope

- `packages/core/` motion policy/lifecycle helpers, focused tests, and shared
  styles;
- paired Svelte/React provider shells and the eight named pilot components;
- shared Rust policy/context, component render paths, and focused tests;
- `poodle-node` animation declarations only where the accepted lifecycle needs
  an existing-channel helper or exact terminal representation;
- GPUI backend/probes only for accepted channels, cleanup, and named
  approximations;
- bounded browser probe fixtures and exact pilot specimens;
- `docs/parity/skeleton.md` for the promoted shimmer-to-pulse delta and its
  implementation closeout;
- architecture 012, MotionPolicyProvider and the eight named component
  contracts, this card, one execution log, front-door currentness, and
  `PAPERCUTS.md` for new execution friction.

Do not edit block sliders, icon morphing, shimmer, Dialog, package versions,
release workflows, or downstream consumers. Do not move established parity
evidence cells or change runtime admission. The additive provider export/row
is reflected by the current 176 public / 175 portable denominator; do not
admit Jetstream. A generated/static Jetstream catalogue route is registry
metadata only, not Jetstream admission or mounted parity.

## Validation

Use Effigy task discovery after worker startup. At minimum:

- focused TypeScript policy/lifecycle and Rust headless trace tests;
- focused Svelte and React provider plus family tests;
- focused `poodle-node`, `poodle-render`, and mounted GPUI regressions;
- the bounded headless browser motion probe in both supported engines;
- contract, prop, callback, capability, and generated-token drift selectors
  touched by the new provider/context;
- `effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, and
  `effigy docs:check`;
- one final headless `effigy qa`;
- `git diff --check origin/main...HEAD` and exact diff-scope/absence checks.

Never run `*-windowed`, release, tag, publication, workflow mutation, or sibling
repository commands.

## Stop Conditions

- A remaining public policy, component lifecycle, Tabs geometry, Toast focus,
  or approximation choice is not settled by the governing contracts.
- The implementation needs a general layout/path/filter/canvas animation
  system, a second policy channel, ambient preference lookup, or provider
  metadata in resolved nodes.
- Correctness requires Dialog, icon morphing, shimmer, block sliders,
  Jetstream, release, or consumer changes.
- A GPUI limitation cannot remain an explicit named approximation without
  misleading semantics.
- The evidence cannot falsify policy restriction, terminal cleanup, immediate
  semantics, or zero-clock frozen output.

## Continuation

On accepted merge, close `g16.034` and refresh the post-g16 frontier. The next
independent component lane is the additive block Slider/RangeSlider promotion.
Icon feasibility and AgentSubagent shimmer gates remain downstream consumers of
the landed motion policy and need their own accepted evidence before public API
work.
