# Working Rules

Status: active
Updated: 2026-10-04
Owner: Poodle core
Depends on: [Product Guardrails](../architecture/product-guardrails.md)

## Authority

- Architecture defines stable ownership and runtime boundaries.
- Specs define repository-wide normative rules.
- Component contracts define public component semantics.
- The Queue plan (lanes, lane documents and their order) states what matters
  next, and Queue holds leads, tasks, briefs, status and outcomes; Effigy tasks
  are command selectors. Neither is knowledge authority.
- Generated evidence under `docs/evidence/` records what was observed.

Execution status does not belong in contracts. When two documents conflict,
prefer the narrower current authority and repair the stale document.

## Contract-First Changes

- Update a component contract before changing observable inputs, defaults,
  states, events, keyboard behavior, accessibility, layout intent, or token use.
- Keep contracts renderer-neutral. Put framework and engine details in runtime
  notes only when they affect parity.
- Document intentional runtime differences and their reason.
- Do not infer parity from a preview specimen alone; validate contract behavior
  and relevant interaction evidence.

## Catalogue Specimens

- The Svelte and React preview headers follow the specimen Size selection
  (xs–xl) and ambient density, so every header control resolves the same
  shared ladder stop. This supersedes the g18.025 fixed-`md` header (operator
  ruling 2026-10-06).
- Component and portable-route counts are derived from one generated source
  that every gate consumes; adding a public component never needs a
  hand-bumped count in several files (operator ruling 2026-09-29).

- Every public component has an addressable, representative specimen in every
  runtime included by its admission. A staged web admission therefore requires
  both Svelte and React catalogue pages before the task is complete; excluding
  native parity does not exclude preview documentation. Distinct public editor
  and renderer exports each get their own page.
- Operator decision (Tom, 2026-09-10): the CodeEditor and RichTextEditor web
  admissions each need both Svelte and React catalogue pages because each is a
  paired web admission, not a single-framework component.
- Catalogue specimens are human-facing documentation. Their first job is to
  show what a component is for, what is available, and how it is normally
  composed.
- Keep `Examples` representative and curated. Do not replace it with an
  exhaustive case corpus or repeat size and density matrices already owned by
  dedicated tabs.
- Exhaustive fixtures, actions, and assertions belong in focused tests beside
  the component, not in the catalogue. The g14 pilot's shared case corpus and
  its projected `Conformance` tab are gone (`g14.008`, `g14.021`).
- A renderer-neutral specimen plan may share ordered tabs, sections, captions,
  and fixture references across runtimes. Runtime adapters still render real
  components and may own bounded presentation needed by their renderer.
- Axis tabs use the component's exact ordered public value domain. A prop named
  `size` does not imply the five-step `ControlSize` domain, and a prop named
  `density` does not justify a tab when its values have no observable effect.
  Every advertised axis value must render real evidence in every active
  runtime; an omitted, blank, collapsed, or fabricated row is a defect.
- Review specimens as documentation. A green test board does not make a
  specimen page useful, and an attractive specimen does not prove parity.

## Shared Implementation

- Put framework-free web state and interaction logic in `poodle-core`.
- Put shared web component styles in `poodle-core/styles`.
- Keep Svelte and React shells idiomatic and thin.
- Put shared native component composition in `poodle-render`.
- Keep GPUI and Jetstream backends limited to runtime interpretation, input,
  lifecycle, and drawing concerns.
- Extend the shared node vocabulary only for reusable rendering capabilities.
- Capability absence is declared with a reason, never inferred from a runtime
  being silent. A declared absence records debt; it does not count as parity
  or component completion.
- While the native crates have no release and no external consumer, changing
  their public Rust API (adding a spec field such as `aria_label:
  Option<String>` with a `with_*` builder, or giving a renderer a handlers
  argument such as `LogListHandlers`) is not a breaking migration. Update
  every in-repo caller in the same change, build specs through `new()` and
  `with_*`, and don't hide a contract input behind handlers or renderer-only
  arguments (planner ruling 2026-10-04, #83).

## Runtime Parity Authority

Poodle targets Svelte, React, GPUI, and Jetstream. The active completion cohort
is currently Svelte, React, and GPUI plus the renderer-neutral Rust declaration
and `poodle-node` output. Jetstream is a deferred backend integration until a
later admission runway proves its converter, input, accessibility, and preview
workflow against the same cases.

- **Svelte is the reference implementation.** Where runtimes disagree on what a
  component can do, Svelte is what the others are brought up to.
- A capability present in Svelte and absent from another active runtime is a
  **gap to port**, not an accepted delta. It remains a failing completion
  condition even when the absence is declared and explained.
- A capability present in another runtime and absent from Svelte is a
  **candidate for inclusion**, not an automatic port. Evaluate it, then either
  add it to Svelte and the contract, or record why it stays runtime-specific.
- The exception is genuinely runtime-owned behavior — focus, IME, portals,
  measurement, pointer capture, text systems, accessibility projection. Those
  are adapter capabilities and are expected to differ in mechanism while
  matching in observable result.
- Porting a capability includes documenting it. An undocumented capability is
  not "in Svelte and missing elsewhere"; it is drift on every side. The
  contract's props table is part of the port.
- Web-native attributes (for example `autocomplete`, `spellcheck`, `autofocus`)
  belong to the web runtimes and stay excluded from the portable Rust spec.
- React uncontrolled `default*` initializers and their change callbacks are
  the React form of Svelte `$bindable` initial values. They are framework
  idiom, recorded in the React drift baseline with that kind, not parity drift
  (operator decision 2026-09-04).
  Imperative escape hatches (for example `focus()`) are documented as methods,
  not props, and are expected in both web runtimes.

Contracts remain the semantic authority. This rule decides what *should* be
true when a contract is silent and the runtimes disagree; it does not let an
implementation override a contract that already speaks.

### Every component ships in the active cohort

A component is not exempt from an active runtime because of where it is
typically used. A titlebar control, a desktop-only affordance, or a dev-tool
surface still implements Svelte, React, shared Rust composition, and GPUI.
"It's only used on the web" is not a reason to skip the Rust target.

Jetstream deferral is program-wide, not a per-component exception and not a
parity claim. Components must keep renderer-neutral specs, cases, and node
output so Jetstream can consume the same authority later. Reports must label
Jetstream deferred until its admission gate passes; they must not report it as
passing, complete, or an accepted absence.

One exception (Tom, 2026-10-07): the Jetstream preview must compile and open
a component by name, because the poodle-lab side-by-side specimen view hosts
it. That repair is not admission; parity reporting stays deferred.

Distinguish two things that sound alike:

- **Active-cohort component parity is required.** Every component has a
  contract, Svelte and React implementations, a `<Name>Spec`, a
  `poodle-render` implementation, and a GPUI specimen. Jetstream preview
  admission is deferred as one backend program rather than waived component by
  component.
- **Web-platform prop parity is not.** Native attributes like `autocomplete`,
  `autofocus` and `spellcheck`, imperative escape hatches, and DOM-node props
  stay web-only and out of the portable spec. `WEB_ONLY` in
  `contract-spec-drift.ts` is the sanctioned register: each prop is keyed
  under `"*"` (every component) or a slug, and each entry carries its reason.

A capability that genuinely cannot cross — a CSS selector, a DOM element
reference — is a documented delta with its rationale, not a silent omission.
Where a web capability has no native equivalent, the native target implements
the *observable result* by its own means, or the contract records why it
cannot.

### CodeEditor staged web admission

Operator decision 2026-09-10: `CodeEditor` may ship first as a complete
TypeScript/web pair over CodeMirror 6. Svelte and React remain one admission;
neither may ship alone. Its Rust spec, `poodle-render` composition, and GPUI
implementation are a named future admission and do not block the web package.

This is a component-specific staged admission, not a general web-only escape:

- the contract and evidence say `web-admitted`, never parity-complete;
- CodeMirror types, transactions, themes, and extensions stay private;
- no native placeholder, fake construction route, or accepted-absence receipt
  improves GPUI counts;
- native work later implements the same public semantics by native means;
- existing portable components touched by the task, including `Tabs`, still
  satisfy the normal active-cohort rule.

### RichTextEditor staged web admission

Operator decision 2026-09-10: `RichTextEditor` and `RichTextRenderer` may ship
first as one complete TypeScript/web pair over TipTap 3 and ProseMirror. Svelte
and React remain one admission; neither may ship alone. Rust, `poodle-render`,
and GPUI are named future work and do not block the web package.

This exception preserves upstream document authority and a configurable seam:

- ProseMirror document JSON and schema semantics are authoritative; Poodle does
  not define a parallel rich-text document model;
- TipTap editor instances, transactions, plugins, commands, and arbitrary
  extensions remain private;
- Poodle exposes curated composable feature modules, with tables in the
  standard set and images optional per project; embeds are deferred;
- the read-only renderer ships in the same admission and uses the identical
  document and feature configuration;
- no native placeholder or accepted-absence record improves GPUI counts.

## Component Ownership

- Poodle owns reusable primitives, composites, and general workstation shells.
- Applications own routing, persistence, data fetching, authorization, domain
  vocabulary, and workflow orchestration.
- App-specific DAW or product widgets remain in their owning repositories.
- Underlay integrations preserve Underlay-owned public APIs behind adapters and
  token bridges.

## Tokens and Presentation

- Change token meaning in the canonical DTCG schema and regenerate every
  target.
- Components consume semantic tokens rather than hardcoded theme values.
- Theme, density, control size, and contrast remain independent axes.
- Use `typography="inherit"` when an inline text-like primitive should follow
  parent typography. Shell geometry should scale proportionally where the
  runtime supports it.

## Svelte Surface

- Prefer Svelte 5 runes for new or substantially changed internals.
- Prefer callback props and snippets for new public composition surfaces.
- Add compatibility aliases only for a documented downstream migration need.
- Treat Bits Svelte as an implementation detail, never as contract authority.

## Accessibility

- Non-interactive layout primitives remain accessibility-neutral by default.
- Semantic regions, labels, focus behavior, keyboard operation, dismissal, and
  announcements must be explicit in the contract.
- Native implementations preserve equivalent semantics where runtime support
  exists and document unsupported capabilities where it does not.
- An adapter may translate API shape but must not silently drop accessibility
  behavior.

## Focus Visibility

- A focus ring, and any composite "focus treatment" (border, fill, shadow),
  is for keyboard interaction. Pointer-driven focus paints none of it. Simple
  controls rely on `:focus-visible`; composite containers gate on the
  document input-modality attribute
  (`data-poodle-input-modality`, installed by the components that need it).
  Native backends must apply the same rule from a keyboard-origin signal
  (operator decision 2026-09-07; `g17.002` for web).

## Release Certification

A release run that fails is a process failure, not a discovery (operator
rule, 2026-09-04). Npm archives and native crates share one lockstep candidate
and immutable tag. Before that tag exists, the exact merged candidate must
have green required PR CI, one green `release.yml` candidate run that uploads
the certified core/Svelte archives, and the native gates recorded in
[`release.md`](release.md). The matching tag is then published from those
exact archives in one second run; there is no tag dry run or publish-time
rebuild. The npm workflow has a ten-minute hard ceiling and never runs
aggregate, Rust, native, GPUI or Jetstream gates. Native consumers use that
same tag by git. A red or over-budget run stops the lane and returns to
planning; the tag is retracted only when nothing was published from it. See
spec 071.

## Papercuts

Small, recurring friction worth fixing later is filed in Queue, not in this
repository (operator ruling 2026-09-26). From the Queue plugin root
(`~/Dev/projects/paseo-northstar-queue`), run
`node bin/queue-cli.mjs papercut.add payload.json` with
`repository: {origin: "inflatable-cookie/poodle", path}`, `title`, `happened`
and `impact`, plus optional `area` and `fix`. Record it and continue the task.
The planner promotes a papercut into the plan or a brief, or closes it as
completed or deprecated.

## Test-Environment Traps

Known behaviour of the tools Poodle tests with. Work around them as follows;
where the repository already provides the workaround, the trap links to it:

- **Svelte:** a prop named `state` collides with the `$state` rune at runtime
  (`store_invalid_shape`). Alias it in `$props()`, for example
  `state: catalogueState = "ready"`.
- **happy-dom:** `@media` rules are evaluated only when a stylesheet is
  parsed, so resizing the window later updates `matchMedia` but not the
  cascade. Set the width before the stylesheet loads, or assert responsive
  layout in a browser.
- **Testing Library:** `fireEvent.click` dispatches only `click`. Overlay
  dismissal listens on document `mousedown`, so a pointer-commit regression
  dispatches `mousedown` on the real target, then `click`.
- **TypeScript `tsc`:** with `FORCE_COLOR=1` (Paseo's agent default) it
  colourises diagnostics, so a plain-string `includes()` of
  `error TS2339: ...` fails. Strip ANSI at the comparison.
- **Vite `--strictPort`:** if a configured preview port is taken by a process
  that is not a healthy preview, the spawn exits and a waiter that only
  probes HTTP will poll the squatter's 404s until timeout. Check the listen
  table before spawn, name the occupant, and print the child output on death.
- **Playwright preview harnesses:** a page degrades after ~15–20 SPA
  navigations as vite client state accumulates, a preview started earlier in a
  batch can die mid-run, and an unhandled native file chooser or dialog can
  wedge the page. Use the shared `captureSession` in
  [`test/visual/session.ts`](../../../test/visual/session.ts): it absorbs
  choosers and dialogs, recycles the page on a fixed cadence, restarts a dead
  preview, and enforces a per-page deadline. Boot previews through
  `startPreviews()` so the fixed port is bound with `--strictPort`.
- **WebKit:** some ports are restricted (for example 4190: "Not allowed to use
  restricted network port"). Browser probes pick a free port outside that
  list, and run one Playwright engine per process.
- **GPUI headless platform:** a view renders several times per
  `window.draw`, so interactive nodes in mounted regressions declare explicit
  ids, or press and release land on different elements. `Frame::clear` never
  clears `debug_bounds` in gpui 0.2.2, so a probe that discovers elements
  across routes uses a fresh window per route. Hit testing clips at the window
  viewport, not at the driver's 160x60 mount box (a press below the box but on
  screen still dispatches), so `HeadlessDriver::pointer_activate_id` fails with
  the element named when its center is off screen instead of pressing the
  mount-box guess. `TestAppContext::run_until_parked` can park the test
  thread forever; every `HeadlessDriver` wait (idle, element, settle,
  open/close) takes a wall-clock deadline and panics naming what it waited
  for. `run_headless` arms a per-test deadline from the libtest thread name.
  Bounds and headroom live beside the constants in
  [`packages/gpui/preview/src/headless_driver.rs`](../../../packages/gpui/preview/src/headless_driver.rs).

## Validation

Use Effigy as the command surface. Match proof cost to the delivery stage:

1. **Worker loop:** run, once, the narrow selectors that exercise the
   changed paths, a compile or type check of what changed, any
   generator/checker pair needed by changed evidence, and `docs:lint` when
   docs or evidence changed. Never run a whole suite (`effigy qa`, `ci:web`,
   `ci:rust`, `docs:check`, `ci:fresh`) or a release gate per task (Tom,
   2026-09-30). Every brief names its targeted checks as acceptance. If they
   prove insufficient, stop and explain the missing proof instead of
   improvising a broader board.
2. **Exact-head PR proof:** push the stable head and use the repository's
   required CI lanes for broad web/Rust coverage. Do not repeat an equivalent
   broad local board merely to restate green CI unless the task changes that
   board or its reproducibility. The worker reports `ready_for_review` as soon
   as the clean PR head and local task proof exist, then stops. Queue
   coordination observes CI asynchronously; workers never run sleep/poll loops
   for GitHub checks. Queue runs no per-task validation command (Tom,
   2026-09-30: per-task full QA wedged the machine and filled the disk).
   Tasks merge on targeted checks, exact-head review and required PR CI.
3. **Milestone proof:** the planner runs `effigy ci:fresh` on `main` at
   release points and after a major chunk of work, through Queue
   (`project.qa.run` with `{"repository": "inflatable-cookie/poodle",
   "baseBranch": "main"}`, then `project.qa.get`; the repository's Queue `qa`
   setting is `effigy ci:fresh`, one-hour timeout). It runs
   `bun install --frozen-lockfile` then `ci` (`ci:web` and `ci:rust`) with
   the Bun pinned in `package.json` `packageManager`, fetched through `bunx`
   when the host Bun differs, because Bun versions lay out `node_modules`
   differently and that changes declaration emit. It must leave
   `git status --porcelain` empty. Native, windowed and release gates stay
   out. The planner also runs `effigy check:gpui` locally on macOS, which carries the
   macOS-only GPUI preview unit tests (`test:gpui-preview`) that the Ubuntu
   PR lanes can't compile (planner ruling 2026-09-30). A red milestone run is
   triaged into papercuts or tasks before the release goes on.
4. **Release proof:** the npm candidate task runs the bounded npm artifact gate
   once after the candidate is complete and stable. Required PR CI owns source
   behavior; candidate mode owns archive certification; publish mode verifies
   and ships those same bytes. Do not stack a local aggregate board, candidate
   run, tag dry run and rebuilt publish proof.

The GPUI window-capture monitor may record unrelated foreground-app changes
during a background run, but the capture process must never become frontmost
and its window must remain unfocused, non-key, and inactive. Tom's 2026-09-08
decision permits ordinary desktop use during a long capture batch without
weakening the proof that capture itself did not activate.

A broad selector subsumes the narrower selectors in its task graph. Do not run
`docs:check`, `ci:web`, `qa`, and release gates serially to restate the same
proof. After review feedback, rerun the affected leaf selector only. Reviewers
read the diff, run the same targeted checks and exercise the behaviour; they
add focused adversarial proof for a finding, never a complete local board.

Aggregate selectors must emit live child progress and elapsed time. A full
headless board hard-stops after fifteen minutes and an individual silent child
after five unless its contract declares a smaller bound. An over-budget run is
a measured blocker: stop its owned process tree and report it rather than
waiting or retrying. Release and validation infrastructure work may run one
capped baseline board and one final board; all intermediate checks stay on
focused leaves.

`docs:lint` is the default local check for execution notes and generated
evidence. Use `docs:check` when the task changes public documentation,
documentation generation, or the docs gate itself. Generated evidence must
describe the current implementation and must not be edited by hand.
`ir:check` and `catalogue:check` run on `ci:rust` (they need cargo; they stay
off `ci:web`), and so does `check:rustfmt`: it drives rustfmt over each
crate's authored `.rs` files directly, because stable rustfmt has no per-path
exclusion and recurses into `#[path]` child modules — plain `cargo fmt` would
reformat generated Rust that must stay byte-identical to its generator. Cheap
`docs:check` leaves that need no Rust
(`check:gpui-census`, `docs:react-prop-drift`, `docs:value-domain-drift`) also
run on `ci:web`, so a stale GPUI census or a new prop-domain drift cannot land
with required CI green. `docs:snippet-check` and `docs:build` stay on
`docs:check` (network and a Vite build). `test:codegen-stamp` is a focused
planted restamp and is not on that board: a second codegen compile would stack
on `ci:rust`, and
`ir:check` is the committed-tree gate. The parity ledger's `Updated` date is
derived from Nucleus V1 lab-run evidence; reproduction compares the body, not
that header line.
