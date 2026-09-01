# Value Track: Nested Menu Pointer Intent

Status: complete
Created: 2026-09-01
Updated: 2026-09-01
Priority: medium (informs a shared interaction rule; not a release blocker)

## Purpose

This dossier researches whether Poodle should preserve an open cascading submenu
while a mouse pointer travels diagonally from its parent item to the flyout. The
target failure is accidental sibling activation or submenu closure in the gap
between the two surfaces. The counter-failure is a sticky, wrong flyout that
survives an intentional move to a sibling or away from the menu.

The result is a conditional promotion candidate: compose the existing geometry
and dismissal ownership, extend the private web submenu pointer path, and add a
small placement-aware decision helper only after active-cohort parity is proved.
There is no implementation authorization in this dossier.

## Scope and authority

The governing card is
`docs/roadmaps/g16/044-nested-menu-pointer-intent-research.md`. Its writable
scope is this dossier and `PAPERCUTS.md` only for new execution friction. No
contracts, source, packages, roadmaps, consumers, or retained disposable
artifacts were changed for this research.

The repository authority read before research was:

- `AGENTS.md`, `README.md`, `docs/README.md`
- `docs/contracts/001-working-rules.md`, `docs/contracts/005-agent-local-paths.md`
- `docs/vision/001-poodle-vision.md`
- `docs/architecture/001-poodle-system-shape.md`,
  `docs/architecture/product-guardrails.md`
- `docs/roadmaps/README.md`, `docs/specs/README.md`
- `docs/policy/internal-writing-style.md`
- `PAPERCUTS.md`

The required research authority was also read: `docs/research/master-index.md`,
`docs/research/research-to-implementation-playbook.md`, the relevant existing
overlay, accessibility, and GPUI value tracks, and the value-track template.
`docs/architecture/system-architecture.md` and
`docs/contracts/contract-index.md` are not present in this checkout; the current
architecture document and contracts README were used as the documented
substitutes.

## Research questions and answers

| Question | Answer | Confidence |
|---|---|---|
| What geometry/timing reduces accidental close without creating a sticky wrong flyout? | A placement-aware corridor with a bounded grace cap is the best fit. A fixed close delay reduces misses only by waiting, and has no knowledge of the intended child. | Medium: established implementations plus a bounded analytical trace; no user study. |
| What about reverse direction, moving submenus, siblings, clipped geometry, and pointer exit? | The corridor must be derived from resolved placement and current child geometry, close on opposite-direction or unrelated-sibling movement, and fail closed when geometry is hidden/invalid or the cap expires. | Medium; these are explicit promotion cases, not currently tested Poodle behavior. |
| Can Menu, ContextMenu, and Menubar use one rule? | Menu and ContextMenu can share the nested-flyout law through their shared MenuSurface shape. Menubar currently owns immediate top-level sibling switching and explicitly excludes deeper cascading menus; it should not receive the rule until that contract changes. | High for the current contracts. |
| How should touch, pen, keyboard, AT, and reduced motion relate? | Pointer intent is a mouse/hover modality only. Keyboard opening, focus movement, ARIA state, touch/pen activation, and reduced-motion semantics must remain immediate and independent of the grace controller. | High from APG and current Poodle motion/accessibility contracts. |
| Which GPUI menus are Poodle-rendered versus OS-owned? | The inspected GPUI preview path is Poodle-rendered through `poodle-render` and the node backend. No OS-owned menu bridge was found in this repository. Contract text permits a future native-window delta, but that is not current implementation evidence. | High for the inspected checkout. |

## Current contract and consumer boundary

### Contract truth

The Menu contract defines `children` on `MenuItem`, recursive flyouts, parent
submenu semantics, hover/keyboard opening, sibling hover switching, and
placement-aware right-edge flipping (`docs/contracts/components/menu.md`,
`Submenus`, `Accessibility`, and `Known Deltas`). ContextMenu adopts the same
submenu behavior while adding a pointer or keyboard virtual anchor
(`docs/contracts/components/context-menu.md`, `Submenus`). Both contracts make
exact collision internals Tier 3 implementation freedom.

Menubar is different. Its contract explicitly puts cascading submenus beyond one
level out of scope, while requiring hover-to-switch between top-level triggers
(`docs/contracts/components/menubar.md`, `Scope`, `Pointer behavior`, and
`Known Deltas`). A shared helper therefore means one semantic law for components
that actually own a nested flyout, not one timer applied to every hoverable menu
item.

There are two pre-existing contract ambiguities relevant to promotion:

1. The Menu and ContextMenu behavior-machine text says closing does not restore
   trigger focus, while their keyboard/accessibility tables require Escape and
   submenu close to restore focus. The pointer-intent work must not silently
   choose between those statements.
2. Menubar describes `MenuItem[]` children in its data shape but excludes nested
   cascading behavior, and the runtime types/renderers are flat. A later contract
   decision is required before treating Menubar as a recursive cohort.

### Consumer and runtime matrix

| Surface | Contract | Svelte | React | Rust/shared render | Current implication |
|---|---|---|---|---|---|
| Menu | Recursive `children` flyouts | Recursive `MenuSurface`; `onpointerenter` opens a child immediately and closes it on a leaf/sibling | `MenuItem` has no `children`; `MenuSurface` renders a flat `items` array and has no pointer-enter submenu path | `MenuEntry` has no child field; `render::menu` renders one flat panel | Svelte is the only live recursive implementation. |
| ContextMenu | Recursive child behavior; consumer may use `trigger=false` and a virtual point anchor | Reuses recursive `MenuSurface`; point/keyboard invocation owns the root anchor | Point anchor and triggerless invocation exist, but rows are flat and there is no child type/path | `ContextMenuSpec` wraps flat `MenuSpec`; renderer is flat | Nested behavior can share MenuSurface, but root target/origin remains consumer-owned. |
| Menubar | Top-level persistent bar; deeper cascading excluded | Portalled top-level overlay; immediate hover-to-switch; no nested child rendering | Same top-level behavior; immediate `onMouseEnter`; no child type | Trigger strip plus flat current menu; no nested rendering or pointer intent | Do not apply a corridor to intentional top-level switching. |
| GPUI preview | Native deltas are contract-permitted | N/A | N/A | Preview wrappers call `poodle_render::menu`, `context_menu`, and `menubar`; node backend maps the result | No current OS-owned exception or native submenu implementation to waive into. |

This is an active-cohort parity gap, not a reason to edit the contracts in this
research lane. The parity ledger records focused web and GPUI specimen evidence,
but no named mounted submenu regression, no React recursive implementation, and
no broad GPUI accessibility proof (`docs/roadmaps/g16/parity-evidence-ledger.md`,
Menu, ContextMenu, and Menubar rows and notes).

## Live runtime trace

### Web Menu and ContextMenu

The current Svelte flow is:

1. `Menu.svelte` owns the menu machine, root trigger, focus entry, and dismiss
   layer. The layer containment function covers the root and its portalled
   surface.
2. `MenuSurface.svelte` owns `highlightIndex`, the active child value, nested
   surface refs, and recursive child rendering.
3. `handleItemPointerEnter` immediately calls `openSubmenu` for a submenu parent,
   or clears the active child for a leaf. There is no pointer history, geometry
   corridor, open delay, or close grace.
4. The root surface uses `anchored.ts` and is portalled to the theme root. A
   nested surface has no anchor, stays in the parent item tree, and is positioned
   by the menu-surface CSS at the parent item edge.
5. `anchored.ts` and `surface-geometry.ts` already observe viewport rectangles;
   the observer reports independent opaque surface IDs and tracks movement,
   resize, visibility, and unmount. The public `onSurfaceGeometryChange` stream
   is web-only and host-neutral.
6. The current nested flip check only detects right-edge overflow. A promoted
   pointer rule must use resolved placement and current rectangle data rather
   than assume a right-facing child.

ContextMenu supplies the root pointer position through a virtual point anchor;
keyboard invocation derives an anchor from the trigger. Its triggerless mode
intentionally leaves the target and origin with the consumer. The root dismiss
layer contains the overlay rather than assuming the target is in the DOM. A
submenu intent helper must not change that ownership or turn the public geometry
callback into a control channel.

The React `AnchoredSurface` mirrors web anchoring and geometry observation, but
the current `MenuSurface.tsx` maps a flat item list. React ContextMenu has the
point-anchor and triggerless paths but no recursive child rendering. React
Menubar switches its top-level menu immediately on mouse enter.

The shared dismiss stack is already the right boundary for outside interaction:
it resolves the innermost eligible layer and uses `layerContains` for portalled
surfaces. Pointer intent should decide whether the active branch remains open
while inside its corridor; it should not add a second dismissal stack or modify
outside/escape semantics.

### Rust and GPUI

The shared Rust component specs expose no recursive submenu data:
`MenuEntry` contains value, label, disabled/checked state, shortcut, and kind;
`MenubarEntry` contains a flat `Vec<MenuEntry>`. The shared renderers build flat
panels and hover styling but do not open nested surfaces or consume pointer
trajectory. ContextMenu forwards the flat MenuSpec.

The GPUI preview wrappers use those shared renderers and then map nodes through
the GPUI node backend. The current specimens for Menu, ContextMenu, Menubar, and
the tree consumer all demonstrate Poodle-rendered flat menus. The contract's
native-window menu note is a possible future delta, not a current OS-owned
consumer.

The node contract's `Interaction` includes activation, focus, context, dismiss,
drag, scrub, wheel, and key intents. Generic pointer movement/history is not a
Poodle node input. The GPUI backend listens to mouse movement for drag/scrub
interactions; the existing high-level `SemanticEvent::Hovered` is an entered/
left semantic event, not a coordinate/timestamp stream. Jetstream does not
consume that event today. A native implementation therefore needs a host-owned
hit-testing and sampling adapter before it can claim semantic parity.

### Accessibility and motion baseline

The contract requires a submenu parent to expose menu popup semantics,
`aria-haspopup`, and `aria-expanded`; keyboard ArrowRight/Enter/Space opens and
focuses the child; ArrowLeft/Escape closes and restores focus; leaves activate
and close. The WAI-ARIA APG supplies the same menubar/menu-button interaction
model. Pointer grace must never delay these semantic or focus transitions.

Poodle's motion policy requires semantics and focus to update immediately and
does not allow animation timers to own semantic state. Pointer intent is an
interaction decision, not a visual transition. Full, reduced, and frozen motion
policies must therefore produce the same semantic open/close result; reduced
motion may remove visual travel but cannot remove keyboard behavior or focus
restoration.

The current native accessibility contract records that GPUI 0.2.2 has no
accessibility API and that current GPUI preview accessibility observations are
window chrome, not proof of Poodle content semantics. This is a promotion gate,
not a reason to introduce a GPUI accessibility shim in this card.

## Durable external evidence

All external sources below were retrieved or pinned on 2026-09-01. The source
measurements are implementation/API observations, not user-study results.

### Primary sources and pins

| Source | Durable URL | Evidence used | License / reuse boundary |
|---|---|---|---|
| Rauno, Web Interface Guidelines | [pinned README at `81f523a5`](https://raw.githubusercontent.com/raunofreiberg/interfaces/81f523a5b469ba1ea877fef262588f3b4b65d31f/README.md); [guideline site](https://interfaces.rauno.me/) | Recommends a prediction cone for nested menus; also treats touch and accessibility as separate interaction concerns. | The pinned repository has no explicit `LICENSE` file or package license metadata. Treat as a reference and paraphrase only; do not copy prose, assets, or code without permission. |
| WAI-ARIA Authoring Practices, Menubar Pattern | [official pattern](https://www.w3.org/WAI/ARIA/apg/patterns/menubar/) | Parent menuitem state, submenu roles, arrow-key movement, Escape/focus behavior, and disabled/separator semantics. | [W3C Document License 2023](https://www.w3.org/copyright/document-license-2023/). Paraphrase here; any copied documentation needs the required URL, copyright, and status notices. |
| WAI-ARIA Authoring Practices, Menu Button Pattern | [official pattern](https://www.w3.org/WAI/ARIA/apg/patterns/menu-button/) | Menu-button focus and keyboard expectations applicable to Menu and ContextMenu roots. | Same W3C document-license boundary. |
| Floating UI | [pinned `safePolygon.ts` at `27629b7`](https://raw.githubusercontent.com/floating-ui/floating-ui/27629b74ba36ab8ceb2a968051927b9b69511a3b/packages/react/src/safePolygon.ts); [useHover docs](https://floating-ui.com/docs/usehover); [pinned MIT license](https://raw.githubusercontent.com/floating-ui/floating-ui/27629b74ba36ab8ceb2a968051927b9b69511a3b/LICENSE) | Dynamic reference/floating rectangles, pointer direction/speed, a polygon corridor, `requireIntent`, and a bounded pending timeout. The pinned implementation uses a 0.1 px/ms intent threshold, a 0.5 px default buffer, and a 40 ms safety timeout in its polygon path. | MIT. An adaptation of substantial source must preserve the copyright and permission notice. No dependency or source was imported. |
| Radix Primitives | [pinned menu source at `f7ecd5a`](https://raw.githubusercontent.com/radix-ui/primitives/f7ecd5ab16f5e1e820eb5786a1419a98a2d594ae5/packages/react/menu/src/menu.tsx); [dropdown-menu docs](https://www.radix-ui.com/primitives/docs/components/dropdown-menu); [pinned MIT license](https://raw.githubusercontent.com/radix-ui/primitives/f7ecd5ab16f5e1e820eb5786a1419a98a2d594ae/LICENSE) | Mouse-only pointer-grace path, pointer direction tracking, a 100 ms submenu-open timer, a dynamic grace polygon, and a 300 ms grace timer. Keyboard paths remain direct. | MIT. Preserve notices if code is adapted; do not copy the implementation verbatim. |
| Ant Design | [pinned Menu docs at `c046a8f`](https://raw.githubusercontent.com/ant-design/ant-design/c046a8f10eb0610c125af6c00a34c021308b8bec/components/menu/index.en-US.md); [pinned SubMenu source](https://raw.githubusercontent.com/ant-design/ant-design/c046a8f10eb0610c125af6c00a34c021308b8bec/components/menu/SubMenu.tsx); [pinned MIT license](https://raw.githubusercontent.com/ant-design/ant-design/c046a8f10eb0610c125af6c00a34c021308b8bec/LICENSE) | Public API exposes `subMenuOpenDelay` default `0` and `subMenuCloseDelay` default `0.1` seconds, plus hover/click trigger choice. This is a delay-only comparator, not evidence for a geometry-aware law. | MIT. Preserve notices if source is adapted; no source was imported. |

No secondary source was needed. The Rauno guideline is the research lead's
design reference, while APG and the pinned implementations provide the
normative/primary behavior and code evidence.

## Algorithm comparison

### Source-derived behavior measurements

| Candidate | Observed open/close behavior | Geometry awareness | Likely tradeoff |
|---|---|---|---|
| Current Poodle Svelte path | Immediate: parent pointer enter opens; leaf/sibling pointer enter clears. No timer. | None beyond the existing surface placement. | Zero pointer latency; diagonal travel through a gap can close the active branch before the child is reached. |
| Fixed close grace, Ant-like | Open `0 s`, close `0.1 s` in the public API. | None. | Easy to reason about and useful as a safety cap, but it waits equally for a child, a sibling, and an unrelated exit. |
| Directional polygon, Floating-like | `safePolygon` follows the current reference/floating rectangles, pointer direction/speed, and a bounded pending timeout; the pinned path uses `40 ms` for its final pending check and a `0.1 px/ms` intent threshold. | Yes; placement and current rectangles are inputs. | Best fit for diagonal travel; requires reliable pointer samples, collision-aware rectangles, and careful opposite-direction handling. |
| Pointer grace, Radix-like | Mouse-only direction tracking; `100 ms` submenu-open timer and `300 ms` polygon grace timer in the pinned source. | Yes; grace polygon is built from exit point and submenu rectangle. | Strong established pattern; more timing surface and a larger sticky-window risk if sibling/geometry cancellation is wrong. |

These values are not proposed Poodle defaults. They bound the design space and
show that “prediction cone” is not one settled algorithm: implementations vary
in open timing, speed thresholds, buffer, and grace cap.

### Bounded analytical trace

A deterministic inline model was run during research and no artifact was kept.
It is a behavior comparison, not a production benchmark or user study.

Assumptions:

- The active parent item is `[0,160] × [40,72]` and the visible child is
  `[168,368] × [24,160]`, leaving an 8 px gap.
- Six eligible target paths leave the parent diagonally and enter the child in
  `64, 96, 144, 64, 96, 144 ms`: upper/lower travel at fast/medium/slow
  speeds.
- Three non-target paths enter an unrelated sibling in `48, 64, 96 ms`.
- The fixed-delay comparator closes at `100 ms`; the directional comparator
  accepts a placement-aware corridor with a `300 ms` safety cap. The model
  treats a sibling entry as intentional switching and treats every target path
  as remaining inside its valid corridor until child entry.

| Comparator | Target paths accidentally closed before child | Target reach | Wrong active branch still open after sibling entry |
|---|---:|---:|---:|
| Immediate close | 6/6 (100%) | 0/6 | 0/3 |
| Fixed 100 ms close grace | 2/6 (33.3%) | 4/6 | 3/3 |
| Directional corridor + 300 ms cap | 0/6 | 6/6 | 0/3 |

The model gives the expected directional result: a corridor retains only paths
aimed at the current child, while a fixed delay retains every path for the same
period. It does not establish production rates. Promotion needs a real browser
fixture and a host probe using the measurement plan below, including flipped,
clipped, nested, and invalidated geometry.

## Proposed semantic law

This is the candidate rule for a future translation memo. It is deliberately a
private interaction law, not a new public pointer-history API.

1. Apply the law only while a visible submenu branch is active and only for a
   mouse/hover-capable pointer path. Keyboard, touch, and pen use the existing
   direct semantics.
2. On leaving the active parent item, snapshot the exit point and use the
   currently resolved child rectangle, parent item rectangle, and resolved
   placement. Construct a corridor toward the child's near edge. The corridor
   must work for right, left, top, bottom, RTL, and collision-flipped placement.
3. Keep the branch open while sampled pointer positions remain in the corridor
   or enter the active child. Opening/focus semantics still occur immediately
   when the child is entered.
4. Close or switch immediately when the pointer enters an unrelated sibling,
   travels in the opposite direction, leaves the corridor, or leaves the menu
   context. A sibling with its own child must become the new active branch under
   the existing sibling-hover semantics.
5. Use a bounded grace cap as a safety valve, not as a general dwell controller.
   Candidate values from the evidence are roughly `200–300 ms`; the cap is an
   unresolved promotion decision, not a Poodle token.
6. If the child rectangle is hidden, detached, clipped beyond the supported
   placement model, or invalidated by scroll/resize/reposition, fail closed or
   recompute from fresh geometry. Never retain a branch indefinitely because a
   rectangle stopped updating.
7. Keep dismissal, focus restoration, and semantic open state in their current
   owners. Pointer grace may defer visual branch closure during eligible mouse
   travel; it must not suppress an outside interaction or a keyboard event.

The helper should consume private surface rectangles and resolved placement
owned by the component runtime. It should not subscribe consumers to
`onSurfaceGeometryChange`, expose pointer samples, or let a consumer mutate the
menu state machine from a prediction cone.

## Recommendation by ownership category

### Extend

Extend the existing private `MenuSurface` pointer path once recursive submenu
parity is admitted. The Svelte `handleItemPointerEnter` path is the current
behavior seam. React needs the recursive item shape/rendering first; the Rust
spec/renderer needs a separate contract admission before it can be an active
cohort. Preserve immediate pointer entry into a child and direct sibling
switching.

### Add

Add, only in a later implementation lane, a small pure geometry decision helper
with inputs such as normalized rectangles, resolved placement, pointer samples,
and a bounded deadline. Keep it private to the implementation boundary until a
contract says otherwise. A pure helper can be shared by web runtimes and later
adapted to native coordinates without copying DOM APIs into Rust.

Do not add a pointer-intent event, pointer-history stream, public dwell prop, or
timing token to `menuTransition` in this research lane. Pointer intent depends
on geometry and input modality; the current transition machine owns semantic
open/close/action events and should not become a history controller by accident.

### Compose

Compose the rule with existing Poodle infrastructure:

- `anchored.ts` / `AnchoredSurface` and the shared overlay collision resolver for
  root placement and the resolved side;
- `surface-geometry.ts` / `observeOverlaySurfaceGeometry` for fresh root and
  nested rectangles;
- `layerContains` and the dismiss stack for portalled containment and outside
  dismissal;
- the existing recursive MenuSurface ownership for child state, focus, and
  leaf activation;
- `MotionPolicy` only for visuals, never for the pointer decision deadline.

No second geometry registry, portal path, dismiss stack, or native/browser
exception is needed.

### Component-local

- Menu owns the nested-flyout rule when it renders recursive children.
- ContextMenu can reuse that rule for nested children, while its trigger target,
  virtual anchor, invocation origin, and restoration target remain its existing
  consumer/overlay ownership boundary.
- Menubar keeps immediate top-level hover-to-switch. Its current contract has no
  recursive target for a corridor. If a future contract admits deeper Menubar
  flyouts, the shared nested helper can be applied only below the top-level
  trigger, with a separate proof that top-level switching remains immediate.

### Consumer-owned

Consumers continue to own the ContextMenu `trigger=false` target and any target
coordinate/origin data. A tree or canvas consumer must not implement its own
generic submenu dwell/history controller. The GPUI host owns native event
collection and hit testing, but the component contract owns the semantic result;
the adapter must prove that distinction rather than silently diverge.

### Reject

Reject the following designs for this card and for an eventual promotion unless
new evidence changes the boundary:

- importing Floating UI, Radix, or another implementation verbatim;
- copying Rauno prose, assets, or code without an explicit license grant;
- a consumer-owned delay/dwell controller or a public pointer-history API;
- one global delay token for Menu, ContextMenu, and Menubar;
- applying the corridor to Menubar's intentional top-level sibling switching;
- `blockPointerEvents`-style interception that makes unrelated controls
  unreachable;
- treating touch or pen as hover-capable for submenu intent;
- coupling semantic open/close to an animation or reduced-motion timer;
- adding a native/OS-owned exception without a semantic, accessibility, and
  licensing proof;
- changing `menuTransition` solely to carry pointer trajectory.

## Measurement and acceptance plan

### Metrics

For each active branch and eligible target path:

- **Accidental-close rate** = target-directed paths that close the active branch
  before entering the intended child / eligible target-directed paths.
- **Sticky-wrong-flyout rate** = paths that enter an unrelated sibling or leave
  the menu while the old branch remains open past the sibling hit or the agreed
  cap / non-target paths.
- **Time to child** = pointer exit from the parent item to child entry, split by
  accepted and rejected paths.
- **False hold** = duration the old branch remains open after the pointer leaves
  the corridor or enters a sibling.
- **Geometry invalidation rate** = paths whose active branch has no current valid
  rectangle after scroll, resize, flip, or clipping.

### Required matrix

The browser fixture and any native host probe must cover:

- right, left, top, bottom, RTL, and collision-flipped placement;
- slow, medium, fast, stop-and-turn, and reverse pointer movement;
- low/high item density, different item heights, and different parent/child
  offsets and gaps;
- viewport edges, clipping, scroll, resize, and a child that becomes hidden;
- depth 1, 2, and 3 recursive branches;
- direct diagonal child entry, sibling crossing, a sibling with a submenu,
  pointer leaving the entire menu, and pointer leaving the application window;
- mouse, touch, pen, keyboard, Escape, ArrowLeft/Right, Enter/Space, and focus
  restoration;
- full, reduced, and frozen motion policies.

Promotion should report the metrics per direction, speed, density, viewport
condition, and depth. Aggregate rates alone can hide a right-edge or nested-depth
regression.

## Explicit promotion gates

### Runtime gate

Promotion is not ready until all of the following are true:

- Svelte and React produce the same semantic outcomes for the recursive fixture,
  or the active cohort is explicitly narrowed in a contract decision. React
  currently lacks the recursive item type and renderer, so this evidence does
  not exist.
- The web implementation uses the actual resolved placement and fresh child
  rectangle after collision, flip, scroll, resize, and unmount. The current
  Svelte right-edge-only flip check is not sufficient evidence for all cases.
- The GPUI target has a named host-owned pointer sampling/hit-testing path and a
  headless probe showing equivalent open, switch, close, escape, and focus
  semantics. The current Poodle-rendered GPUI path has no generic pointer
  history input and no current recursive renderer.
- The helper never changes outside-dismissal ancestry, leaks timers after
  unmount, or retains an invalid branch after geometry disappears.
- No `*-windowed` conformance selector is used locally; native evidence uses the
  approved headless path or an explicitly hosted probe.

### Accessibility gate

- Parent items retain `role=menuitem`, `aria-haspopup="menu"`, and immediate
  `aria-expanded` updates; child surfaces retain menu role/name and the existing
  recursive relationships.
- ArrowRight/Enter/Space opens and focuses immediately; ArrowLeft/Escape closes
  and restores the documented parent/trigger focus. Pointer grace never delays,
  cancels, or re-targets keyboard/focus behavior.
- Pointerdown/click on a child or sibling remains actionable. No pointer-events
  blockade, invisible corridor overlay, or focus trap is introduced.
- Disabled items, separators, checked/radio semantics, announcements, and leaf
  activation are unchanged.
- Touch and pen do not receive hover opening or pointer grace; their activation
  behavior remains the existing contract.
- Full/reduced/frozen motion modes have identical semantic and AT results. The
  focus-restoration contradiction in the current Menu/ContextMenu contract is
  resolved by the contract owner before implementation, not by this helper.
- Web keyboard/AT and axe evidence exists for each recursive runtime. GPUI
  evidence names the host and AT path; current native accessibility artifacts
  are not sufficient proof.

### Licensing gate

- Every external behavior reference in the implementation record includes its
  pinned commit, URL, retrieval date, and license.
- No Rauno text, assets, or code is copied without permission and an explicit
  notice. W3C documentation is paraphrased unless the required document-license
  notices accompany copied material.
- Any adapted MIT source from Floating UI, Radix, or Ant Design carries the
  relevant copyright and permission notices. Prefer a clean-room Poodle helper
  derived from the behavior description and contracts.
- No dependency, vendored implementation, or license-file change is introduced
  as part of research promotion without a separate review.

### Promotion-decision gate

The following operator decisions are intentionally recorded here for a later
promotion review. This research lane does not ask the operator to settle them:

- exact open behavior and close cap: immediate child open, any open grace, and
  the final close cap within the evidence range of roughly `200–300 ms`;
- whether the corridor buffer/speed threshold is fixed, adaptive, or omitted;
- whether the helper is a private shared web utility, a runtime-neutral internal
  utility with a native adapter, or component-local until Rust parity exists;
- whether Menubar remains top-level-only or later admits deeper cascading
  flyouts, and whether its top-level hover switching is explicitly excluded;
- whether React and Rust recursive submenus enter the active cohort before this
  behavior can be called shared parity;
- what GPUI host owns pointer sampling and what evidence qualifies as equivalent
  semantics while GPUI accessibility remains unavailable;
- what collision/clipping policy applies when a child rect is hidden or cannot be
  represented by the corridor;
- which source adaptations, notices, and legal review are required if any
  external code is reused.

## Disposition

**Recommendation: compose + extend, conditional on promotion gates.**

The evidence supports a placement-aware directional corridor with a bounded
grace cap as the interaction shape most likely to reduce diagonal accidental
closure without accepting every hover path. Existing Poodle geometry,
recursive ownership, and dismissal infrastructure are sufficient boundaries for
a later implementation. The current active cohort is not broad enough to call
this a shared Menu/ContextMenu/Menubar behavior: only Svelte currently renders
recursive flyouts, Menubar excludes deeper nesting, and GPUI/React lack the
required recursive path and pointer input evidence.

This dossier therefore rejects implementation, contract edits, and a web-only
parity waiver. The next artifact, if the recorded promotion decisions and gates
are accepted, is a translation memo that fixes timing/geometry ownership and
defines the mounted Svelte, React, and GPUI evidence before source work begins.

## Related

- Research card: `docs/roadmaps/g16/044-nested-menu-pointer-intent-research.md`
- Working rules: `docs/contracts/001-working-rules.md`
- Menu contract: `docs/contracts/components/menu.md`
- ContextMenu contract: `docs/contracts/components/context-menu.md`
- Menubar contract: `docs/contracts/components/menubar.md`
- Anchored overlays: `docs/contracts/002-anchored-overlays.md`
- Native accessibility: `docs/contracts/003-native-accessibility.md`
- Research playbook: `docs/research/research-to-implementation-playbook.md`
- Parity evidence: `docs/roadmaps/g16/parity-evidence-ledger.md`

## Follow-up

Hold implementation until a promotion review accepts the timing, geometry
ownership, active-cohort boundary, runtime evidence plan, accessibility proof,
and licensing treatment recorded above. Then write the translation memo; do not
promote this value track directly into source changes.
