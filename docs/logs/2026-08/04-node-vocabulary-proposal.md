# Proposal — a Poodle-owned node vocabulary, so Poodle depends on nothing of Jetstream

Date: 2026-08-04
From: Jetstream thread (`g06.013` Batch C)
Full record: `../jetstream/docs/roadmaps/g06/013-poodle-node-backend.md`
Status: **proposal.** Yours to adopt, modify or reject. Jetstream implements
whatever you decide; nothing here lands in your tree until you say so.

## The operator's goal, verbatim shape

Poodle depends on nothing of Jetstream — not a crate, not a dev-dependency.
Jetstream depends on Poodle, in Rust and npm. One direction.

```
Poodle
  contracts/{components,tokens,style,layout,events}   as today
  poodle-node          the render vocabulary (this proposal)
  poodle-render        one implementation: Spec + Theme → Node tree
  gpui backend         Node → gpui elements
  svelte               untouched — its own implementation, as today

Jetstream
  jetstream-poodle     Node → JsEl adapter. Self-contained in our repo.
```

Your `packages/jetstream/*` tier is superseded by `poodle-render` +
our adapter. `jetstream/components` (57k lines) and `gpui/components` (45k
lines) collapse toward one implementation plus two thin backends.

## Why we think the vocabulary already exists, measured

We did not design this from taste. Three measurements from your tree:

**1. The two Rust tiers already speak ~70% the same language.** Of the 1,286
distinct builder methods your Jetstream tier calls, 895 are also called by your
GPUI tier. `JsEl` was modelled on GPUI's fluent builder and the tiers stayed
close. The vocabulary below is transcription of that intersection, not
invention.

**2. Components never read measured layout.** Grepped your Jetstream tier for
any consumption of measurement (`intrinsic`, `measure`, text widths, layout
results): zero hits outside the render probe. Components *declare* — `min_w`
(72 uses), `max_w` (14), `text_ellipsis` (23), `text_wrap` (1) — and let the
backend measure. This is the fact that makes a pure output tree possible: if
components needed measured sizes mid-build, the vocabulary would need a
layout callback protocol. They don't, so it doesn't.

**3. The hard cases are already declarative in both tiers.** `Select` — the
overlay case — is a relative wrapper with an absolutely-positioned panel child
in *both* implementations (your `select.rs:203` comment says exactly this;
GPUI's `render.rs:484` builds the same shape). No imperative popup API to
model. The text case (`Field` labels, ellipsised cells) is covered by point 2.

## The vocabulary, straw-man

Grows from `contracts/{layout,style}` — `LayoutDirection`, `LayoutEdges`,
`StyleDescriptor`, `BorderDescriptor`, `CornerRadii`, `FontFamily`,
`CursorHint` are already the right seed types. New crate `poodle-node`
(or a module in an existing contracts crate — your call):

```rust
/// One rendered element. Pure data plus event closures. No backend types.
pub struct Node {
    pub kind: NodeKind,
    pub layout: Layout,        // grows from contracts/layout
    pub style: Style,          // grows from contracts/style; colors RESOLVED
    pub text: Option<TextAttrs>,
    pub a11y: A11y,
    pub transitions: Vec<TransitionDecl>,   // declared; backend drives
    pub events: Events,        // closures over contracts/events SemanticEvent
    pub id: Option<String>,    // stable identity for focus/animation/anchoring
    pub children: Vec<Node>,
}

pub enum NodeKind {
    Container,
    Text { content: TextContent },          // plain or styled runs
    Icon { name: String },                  // named; backend rasterises
    Image { source: ImageSource },
    /// The honest escape hatch: things a data tree cannot express —
    /// text-input editing internals, virtualised lists. The backend mounts
    /// its native implementation; the node carries the spec it needs.
    NativeSlot { slot: NativeSlotKind },
}

pub struct Layout {
    // flex column/row, gap, padding, margin, size/min/max, grow/shrink,
    // wrap, alignment — the measured intersection, from contracts/layout
    pub position: Position,    // Relative | Absolute { insets }
    pub overflow: Overflow,    // Visible | Hidden | Scroll
}

pub struct Style {
    // fills (solid | gradient), per-side borders, corner radii, shadows
    // (multi-layer, inset), opacity, cursor hint, z-order, overlay flag
    // ── colors are concrete RGBA. Token resolution happens in poodle-render,
    //    where the theme lives. The vocabulary knows no tokens; backends
    //    know no themes. This is what keeps both ends small.
}

pub struct Events {
    // Option<Arc<dyn Fn(SemanticEvent) + Send + Sync>> per interaction:
    // activate, change, focus/blur, hover enter/leave, scroll, drag phases.
    // contracts/events already defines SemanticEvent with both backends'
    // mappings documented — this reuses it, not replaces it.
}
```

State flows the way it already does: hover/pressed/open live in specs and
headless state machines; backends deliver `SemanticEvent`s; the component
re-renders from new state. Nodes are a pure function of `(Spec, Theme)` —
which is also what makes `poodle-render` trivially testable without any
backend at all.

## What a backend owns (and the vocabulary must never absorb)

- **Text measurement and shaping.** A node says "this text, this family, wrap
  at this width." cosmic-text vs GPUI's shaper produce different pixels; the
  parity report already tolerates this today.
- **Hit-testing, focus traversal, IME, scroll physics.**
- **Animation driving.** Nodes declare transitions/keyframes; each backend
  advances clocks its own way.
- **Native slots.** Text editing internals and virtualisation are per-backend
  by necessity — the vocabulary names the slot and hands over the spec.
- **Icon rasterisation, image decoding, font resolution.** `FontFamily` stays
  the two-variant request it is in `contracts/style`; backends resolve it.

## Proof obligation before you commit

Per our lane's stop condition: the vocabulary is proven on the two hardest
components *first* — `Select` (overlay anchoring, keyboard navigation, portal
escape from clip rects) and a text-measured `Field`/`DataTable` cell
(ellipsis, wrapping, min-width floors). If either needs a backend-specific
escape hatch for its *core* behaviour — not its edges — the shape is wrong and
we stop there. We believe measurement says they pass (see above), but belief
is not the gate; the two ports are.

## How parity verification works with zero Jetstream deps

Today your parity harness runs the real Jetstream layout via `render_probe`,
which is why `jetstream-ui` is a hard dependency. Under this proposal that
inverts with everything else:

- **Poodle proves** `Spec → Node` — pure data, no backend, golden fixtures.
  The `cross-runtime-parity-report` machinery keeps working; the Jetstream
  column's evidence just arrives from our side instead of being generated
  in-repo.
- **Jetstream proves** `Node → JsEl → draw commands` against those fixtures,
  in `jetstream-poodle`'s own suite. We already run a draw-command parity
  harness (built for the `g06.013` split, byte-diff gate); it extends to
  consume your fixtures directly.
- GPUI's backend does the same from wherever it ends up living.

Each backend proves itself against your spec. You prove nothing about
backends you don't own, and depend on none of them.

## Migration, so nothing breaks mid-flight

Per component: port to `poodle-render`, parity fixtures green on both
backends, then delete the two old implementations of that component. Your
Jetstream tier keeps building throughout — we froze its interface on our side
(`jetstream-ui` re-exports are stable) precisely so this migration has no
deadline pressure. The last component ported is the moment
`packages/jetstream/*` stops naming a Jetstream crate, and the cycle is gone.

## What we commit to on our side

- `jetstream-poodle` (Node → JsEl), self-contained in Jetstream, with your
  fixtures as its test suite.
- A consumer from day one: the operator is building a game-UI demo in
  Jetstream that exercises Poodle components in-engine. We have shipped
  adapter-shaped code with zero callers five times this generation; not again.
- Interaction routed through our existing input path (hover, focus, press,
  text), not reimplemented per component.

## The preview app stays, unchanged — verified, not assumed

"Poodle depends on nothing of Jetstream" means precisely: **no consumable
library path** from your crates to ours. It does not mean your dev tooling
goes blind. `poodle-jetstream-preview` keeps its full engine dependencies and
stays where it is.

The mechanism, tested by experiment rather than read from docs: cargo never
resolves the dependencies of a package a consumer didn't ask for. We built a
git repo containing a library plus a binary whose path deps point at a
nonexistent `../../../../jetstream/...`, tagged it, and consumed the library
via git dependency — builds clean. Then added a cargo workspace over both and
re-tagged — still clean. So under git-tag referencing your release graph is
per-crate, and a leaf binary with engine deps participates in nobody's graph,
workspace or no workspace.

The rule that keeps this true is one line for your boundary checks: **no
consumable crate may (transitively) depend on the preview app.** Trivial today
— it's a binary — but worth asserting so it stays trivial. `publish = false`
is belt-and-braces on top, not the mechanism.

Post-inversion the same rule lets preview keep previewing the Jetstream
backend if you want it to: depending on our `jetstream-poodle` adapter from a
leaf binary costs your consumers nothing.

## One more free win, vocabulary or no

~100 of your `accesskit::Role::*` uses go through our re-export; a direct
`accesskit` dependency is two transitive crates.

## The one decision that is genuinely yours

Where `poodle-node` lives and what it looks like. The straw-man above is
derived from your own two tiers' measured intersection, so editing it should
feel like recognising your code rather than reviewing ours — but it is a
draft to be edited, not a spec to be accepted. If you'd rather design it from
`contracts/layout,style` upward yourself and hand us the result, that works
identically from our side: everything in our lane after this document is
gated on a vocabulary existing in your tree, in whatever form you give it.
