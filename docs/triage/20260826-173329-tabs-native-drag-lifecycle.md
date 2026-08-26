# Tabs Native Drag Lifecycle

Status: resolved — reusable payload lifecycle promoted into the Tabs contract
and `g16.006`
Captured: 2026-08-26
Source: post-`g16.004` parity-ledger selection

## Finding

Tabs is a high-value primitive and still lacks named mounted GPUI behaviour,
but its detailed contract is not currently executable as written on the native
substrate.

The contract requires pointer drag reorder, drag-source and drop-target visual
state, and an `onReorder` result. Svelte and React have the complete lifecycle.
Shared Rust can draw supplied drag/drop state and the node vocabulary can carry
a drag payload and drop zones, but `TabsHandlers` has no reorder callback and
the native drag-payload path does not report a general begin/move/end lifecycle
back to the component. The older delta-only `on_drag` gesture is a separate
backend path and cannot safely be combined with the payload/drop-zone path as
an implicit component workaround.

Post-`g16.005` inspection found that the reusable payload/drop seam is closer
than the first pass suggested, but not yet correct enough to consume:

- Tree and ModelCatalogueEditor already publish payload sources and drop zones;
- stock GPUI's `on_drag` callback supplies a real drag-start boundary and its
  root event path can observe release and Escape without a fork;
- the current backend sends hover to every zone on every drag move instead of
  hit-testing the zone first;
- the current drop callback discards the last computed edge and always reports
  `inside`;
- no source start/end or target-leave intent reaches shared composition, so a
  host cannot clear transient source/target visuals after cancellation.

This is a bounded defect in an existing reusable seam, not a reason to create a
Tabs-only gesture path or a new cross-runtime behavior architecture.

Keyboard reorder is also incomplete in shared Rust. It can be added with the
current key vocabulary, but doing that alone would make a mounted claim over a
known pointer-contract hole.

## Decision Needed

Choose one boundary before Tabs execution:

1. add a renderer-neutral drag-payload lifecycle callback to the node/backend
   seam, then implement the full contracted Tabs reorder result;
2. explicitly narrow native Tabs to keyboard reorder and record pointer drag as
   an accepted runtime delta; or
3. revise Tabs reorder semantics across all active runtimes.

The first option preserves the current contract and is preferred, but it must
be designed as a reusable node capability rather than a Tabs-only hidden
channel.

## Decision

Option 1 is selected. The active-cohort rule already requires the observable
Svelte contract to be ported unless an explicit product decision narrows it;
no such narrowing is justified here. `g16.006` completes the existing payload
lifecycle, corrects hit-testing and retained drop edges, then consumes that
seam for Tabs pointer reorder, keyboard reorder, close, and mounted GPUI proof.

The vocabulary remains semantic: payload id, lifecycle phase, target id, and
drop edge. No raw pointer coordinate or rendered geometry crosses into the
component. Stock crates.io GPUI remains the backend boundary; needing a fork
is a stop condition.

## Guardrail

Do not dispatch Tabs as a simple mounted-regression card. Promote the chosen
drag boundary into `docs/contracts/components/tabs.md` and a dedicated roadmap
before implementation. Do not use direct handler invocation, spec inspection,
or externally supplied visual state as evidence for the missing lifecycle.

## Promotion Route

Promoted on 2026-08-26 into `docs/contracts/components/tabs.md` and
`docs/roadmaps/g16/006-tabs-drag-keyboard-and-mounted-parity.md`. This note is
closed evidence. Tree and ModelCatalogueEditor remain regression consumers of
the corrected generic seam; their ledger cells do not move in this card.
