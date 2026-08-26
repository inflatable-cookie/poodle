# Tabs Native Drag Lifecycle

Status: open — decide before compiling a Tabs mounted-parity card
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

## Guardrail

Do not dispatch Tabs as a simple mounted-regression card. Promote the chosen
drag boundary into `docs/contracts/components/tabs.md` and a dedicated roadmap
before implementation. Do not use direct handler invocation, spec inspection,
or externally supplied visual state as evidence for the missing lifecycle.

## Promotion Route

Return here after a bounded control lane lands. Inspect Tree/EditList drag
consumers, choose the reusable lifecycle, then compile a Tabs semantic and
mounted-parity card.
