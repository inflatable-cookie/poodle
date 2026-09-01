# Value track: agent citations and sources

Status: research complete; awaiting operator review
Created: 2026-09-01
Updated: 2026-09-01
Priority: medium (informs future agent-message composition)

## Executive summary

Research disposition: **compose, gated, not promoted**.

Poodle should not turn raw `AgentMessage` markdown into an automatic citation
parser. The current message path reparses streamed markdown, has no durable
inline-node identity, and deliberately keeps transcript announcements above
the message body. Numeric markers, token cadence, word reveal, and shimmer are
therefore the wrong ownership boundary.

The viable future shape is an authored, renderer-neutral composition that keeps
these concerns separate:

- **Compose:** an agent-citation composition owns the relationship between
  authored prose segments, inline reference marks, and a compact source list.
- **Add, conditionally:** a generic non-provider-specific reference mark and a
  source-list primitive can be added only after the composition proves that
  both are independently reusable. A single composite is the smallest first
  proof.
- **Extend, conditionally:** extend the existing message composition seam with
  an authored semantic-content input, not the raw markdown parser or
  `TranscriptMessage` record. No contract extension belongs in this research
  card.
- **Consumer-owned:** source identity, title, URL admission, fetching,
  verification, follow-up actions, availability truth, and placement policy.
- **Reject:** automatic extraction from markdown positions or numbers, source
  fetching, provider schemas, token-level announcements, word-reveal cadence,
  shimmer, and source-specific brand vocabulary.

The recommendation is not ready for promotion. Promotion requires operator
acceptance of stable identity, placement, inline-mark interaction, accessibility
ownership, link safety, licensing boundaries, and the first consumer proof.
Those unresolved decisions are recorded below; this lane does not ask the
operator to settle them.

## Scope and evidence discipline

The research follows the [g16.038 research card](../../roadmaps/g16/038-agent-citations-and-sources-research.md).
The local snapshot is Poodle commit `06a42e3cc36b865344d0bf9e3b5c81bbd1c0a32d`,
captured on 2026-09-01. The card's source leads were inspected from the
official routes and pinned public repositories. No source code, CSS, icon, or
proprietary text was copied into Poodle.

Evidence labels:

- **[LF] Local fact** — observed in the Poodle checkout or a read-only adjacent
  checkout at the captured revision.
- **[SF] Source fact** — observed in an official standard, documentation page,
  registry, or pinned public repository.
- **[WI] Worker inference** — a recommendation derived from the evidence; it
  is not a new Poodle contract.

Research is point-in-time evidence. The repository's current contracts and
architecture remain authoritative over this dossier.

## Research questions

### Composition boundary

- [x] Is a single composite or a pair of primitives the smaller reusable
  surface?
- [x] Can authored content carry stable source identity through stream,
  reparse, and settle without exposing parser implementation details?
- [x] Which source-record fields must remain consumer-owned?

### Runtime and interaction

- [x] Can the active Svelte, React, shared Rust, and GPUI cohort project one
  semantic surface?
- [x] What current focus, copy, selection, link, and live-region behavior must
  remain intact?
- [x] What is the smallest proof across dense, repeated, missing, unsafe-link,
  narrow, reduced-motion, and assistive-technology cases?

### Evidence and governance

- [x] What do the named Beautiful UI and AICSS examples actually implement?
- [x] What licensing limits apply to code, assets, and private/pro surfaces?
- [x] Which runtime, accessibility, licensing, and promotion gates must precede
  a contract change?

## Sources

### Primary local sources

| Source | Evidence captured | Boundary |
| --- | --- | --- |
| [AgentMessage contract](../../contracts/components/agent-message.md) | [LF] Current input is one turn of raw markdown with streaming state, link target, size, density, and a link callback. The contract defines a normalized markdown model but no source records, citation marks, ranges, or source identity. | Normative for the current message surface; this dossier does not amend it. |
| [AgentTranscript contract](../../contracts/components/agent-transcript.md) | [LF] The viewport owns the append-only `role="log"` and polite announcement model. Messages carry raw markdown and stream state; transcript grouping, virtualization, and focus behavior are separate from message content. | Normative for transcript announcement, focus, and windowing ownership. |
| [TextLink contract](../../contracts/components/text-link.md) | [LF] Web-facing props already include `href`, `target`, `rel`, `ariaLabel`, disabled state, and click handling. Callers provide readable names; external-link icon policy is outside the contract. | A link foundation, not a URL-admission or source-record policy. |
| [Poodle system shape](../../architecture/001-poodle-system-shape.md) and [product guardrails](../../architecture/product-guardrails.md) | [LF] Poodle's active cohort is Svelte, React, shared Rust composition, and GPUI. Poodle owns generalized tokens, primitives, and composites; product-specific vocabulary and flows stay with consumers. | Prevents importing a provider's source schema or an app-specific citation workflow. |
| [Native accessibility contract](../../contracts/003-native-accessibility.md) | [LF] Jetstream has an AccessKit path but is deferred; GPUI 0.2.2 cannot currently map Poodle accessibility metadata into a native accessibility tree. GPUI proof is not full native AT conformance. | GPUI accessibility is a promotion blocker, not a claim to hide in a web-only result. |

### Primary implementation sources

| Source | Evidence captured |
| --- | --- |
| [`markdown-blocks.ts`](../../../packages/core/src/markdown-blocks.ts) and [markdown-block tests](../../../packages/core/test/markdown-blocks.test.ts) | [LF] `MdInline` has text, code, strong, emphasis, deletion, link, and hard-break cases. Links carry `href` and children only. Plain-text projection is used for aria, copy, and truncation measurement. There are no source IDs, source ranges, or durable node identities. |
| [`agent-transcript.ts`](../../../packages/core/src/agent-transcript.ts) and [transcript tests](../../../packages/core/test/agent-transcript.test.ts) | [LF] The shared headless model groups contiguous tool calls, keeps the first tool-call ID stable for a run, calculates virtualized block windows, and tracks pinned-bottom state. It has no citation/source records or citation announcement helper. |
| [Svelte `AgentMessage`](../../../packages/svelte/components/src/AgentMessage.svelte), [React `AgentMessage`](../../../packages/react/components/src/AgentMessage.tsx), and their [Svelte tests](../../../packages/svelte/components/test/AgentMessage.test.ts) / [React tests](../../../packages/react/components/test/AgentMessage.test.tsx) | [LF] Both web renderers derive blocks from the current markdown on each update. Svelte renders normalized inline runs; React uses indexed keys for recursive inline output. Both pass links to `TextLink`, render a hidden streaming caret, and have no source-mark, copy, focus, or message-level live-region behavior. |
| [Rust markdown](../../../packages/contracts/markdown/src/lib.rs), [Rust AgentMessage contract](../../../packages/contracts/components/src/agent_message.rs), and [Rust AgentMessage renderer](../../../packages/render/src/agent_message.rs) | [LF] The Rust markdown model also has links without source identity. The renderer flattens inline links and emphasis into text; its source comment says link callbacks wait for a future rich-inline-run model. The public Rust spec has no citation input. |
| [Rust AgentTranscript contract](../../../packages/contracts/components/src/agent_transcript.rs), [headless transcript](../../../packages/contracts/headless/src/agent_transcript.rs), and [renderer](../../../packages/render/src/agent_transcript.rs) | [LF] Native transcript rendering sets a log role and renders existing message/tool/status families. No citation/source composition or source-list semantics are present. |
| [`TextLink` renderer](../../../packages/render/src/text_link.rs), [node contract](../../../packages/contracts/node/src/lib.rs), and [GPUI node backend](../../../packages/gpui/node-backend/src/lib.rs) | [LF] Native `TextLink` currently builds styled text and activation metadata; `href`, target, and rel do not become native link navigation. The node vocabulary has no link or inline-reference kind, and `NodeA11y` has no live-region or announcement fields. GPUI's backend intentionally does not map `NodeA11y` into its current accessibility API. |
| [GPUI native proof](../../../packages/gpui/native-accessibility-proof.json) | [LF] A mounted proof must cover focus entry/recovery, keyboard traversal, and announcements before promotion. Web live-region behavior must be reproduced by native accessible-tree/announcement plumbing; the proof is not full native conformance. |

### Primary external sources

All external pages below were checked on 2026-09-01. Mutable demo routes are
paired with a pinned public-repository commit where available.

| Source | Evidence captured | Licensing/provenance boundary |
| --- | --- | --- |
| [Beautiful UI Streaming Text registry](https://www.beautifului.dev/r/streaming-text.json) and [Beautiful UI homepage](https://www.beautifului.dev/) | [SF] The example separates streamed tokens from source records. Tokens have a citation flag; sources have display name, domain, href, and image. A source footer opens from an action button and lists source links. The same example also includes fixed word timing and a completion hold. | Behavior observation only. The route is mutable; no code, CSS, or assets were copied. |
| [Beautiful UI pinned repository](https://github.com/slev12397/beautiful-ui/tree/63febac4c50a1f096d7a7360e6cda1f5cc87061c) and [license](https://www.beautifului.dev/license) | [SF] Public repository inspected at commit `63febac4c50a1f096d7a7360e6cda1f5cc87061c`; its stated license is MIT. The public README documents reduced-motion fallbacks and a rendering-layer boundary for real agent wiring. | MIT permits reuse with notice preservation, but each dependency and asset still needs its own inventory. The demo's third-party icon dependency is not automatically covered by the component's MIT notice. |
| [AICSS Inline Citations](https://www.aicss.dev/components/inline-citations) and [registry source](https://www.aicss.dev/r/inline-citations.json) | [SF] The example uses authored numeric markers and a separate reference list. Its public type has number, label, host, and URL. Known markers become links; unknown markers remain plain text. The compact footer and marker tooltip are useful density references. | Behavior observation only. No code, CSS, or copy was copied. |
| [AICSS pinned public repository](https://github.com/kvnkld/aicss/commit/4556a918fd8c9358d42d2b24a3866301b8ea10a2) and [license terms](https://www.aicss.dev/license) | [SF] Public repository inspected at commit `4556a918fd8c9358d42d2b24a3866301b8ea10a2`; the public package surface is MIT. The official license page says Pro components are private and separately licensed. | MIT applies to the public package surface only. Private/Pro components, website material, and unlisted assets are not a reuse source. |
| [WAI-ARIA `log`](https://www.w3.org/TR/wai-aria/#log) and [ARIA23](https://www.w3.org/WAI/standards-guidelines/aria/techniques/aria23) | [SF] A log is an append-only live region with polite sequential updates; new additions should be announced without moving focus. | Normative accessibility guidance for the transcript boundary. |
| [APG accessible names](https://www.w3.org/WAI/ARIA/apg/practices/names-and-descriptions/), [link pattern](https://www.w3.org/WAI/ARIA/apg/patterns/link/), [tooltip pattern](https://www.w3.org/WAI/ARIA/apg/patterns/tooltip/), and [disclosure pattern](https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/) | [SF] Interactive controls need useful names; native anchors are preferred for links; tooltips must be available on focus as well as hover and be associated with their trigger; disclosure buttons expose expanded state and keyboard behavior. | Normative interaction guidance used for the proposed gates, not a claim that either named demo meets every APG detail. |
| [OWASP XSS Prevention](https://cheatsheetseries.owasp.org/cheatsheets/Cross_Site_Scripting_Prevention_Cheat_Sheet.html), [MDN `javascript:` URLs](https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/javascript), [WHATWG links](https://html.spec.whatwg.org/dev/links.html), and [MDN `noopener`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel/noopener) | [SF] Untrusted URL values need canonicalization and scheme/allowlist validation. `javascript:` executes code; a new browsing context should be isolated from its opener. | Establishes the link-safety gate. It does not assign URL policy to Poodle or the consumer by itself. |
| [MDN `prefers-reduced-motion`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion) | [SF] Reduced-motion preferences should replace or reduce nonessential motion. | Establishes the motion gate; citation meaning must not depend on animation. |

## Current Poodle audit

### Contract and consumer boundary

The current surface has a clean ownership split:

1. `AgentMessage` accepts raw markdown for one turn and derives normalized
   blocks. It does not own source records, source fetching, follow-up actions,
   or message-level announcements.
2. `AgentTranscript` owns the append-only log, grouping, virtualization, scroll
   behavior, and focus policy. It should not infer citations from child text.
3. `TextLink` owns link presentation and native web link/button semantics. It
   does not own URL trust, redirects, source verification, or provider identity.
4. Core and Rust markdown models deliberately preserve semantic markdown blocks
   without parser-specific source ranges or node IDs. Their current link shape
   cannot associate an occurrence with a durable source record.
5. The Rust renderer and GPUI node path flatten inline content. They can prove
   text and handler construction, but they cannot currently prove native link
   semantics or a live-region announcement path.

### Streaming, reparsing, and stable identity

The live implementation makes automatic marker extraction unsafe:

- [LF] Svelte derives `blocksFromMarked(marked.lexer(markdown))` from the latest
  markdown. React derives the same shape with `useMemo` and renders recursive
  inline nodes with positional keys.
- [LF] The contract explicitly reparses every streamed append because a partial
  fenced block can change the interpretation of earlier text. Incremental
  parser identity is not available.
- [LF] `MdInline.Link` carries only `href` and child content in both web/core
  and Rust models. There are no source ranges, authored target IDs, or stable
  association keys.
- [WI] A marker derived from an index, number, href, title, normalized text, or
  DOM key can silently point at a different source after a stream update or
  reparse. Stable identity must arrive from the host as opaque IDs and survive
  content replacement explicitly.

### Announcement, focus, copy, and selection

- [LF] The transcript viewport is the current `role="log" aria-live="polite"`
  owner. The AgentMessage body is not a live region; the contract says the
  transcript announces a completed message at log level.
- [LF] Appending transcript content does not take focus. Expanding a tool or
  list preserves focus, and virtualization can remove blocks from the
  accessibility tree unless the host chooses the full-transcript mode.
- [LF] The current AgentMessage surfaces expose no copy action or citation
  selection policy. Core `markdownPlainText` is a projection used by aria,
  copy, and measurement, but it has no citation-aware output.
- [WI] Citation mutations must not create a second token-level live region or
  steal focus. A settled source list may be disclosed by an ordinary button;
  its rows become keyboard-reachable only when disclosed.

### Current consumers

- [LF] The in-repo Svelte and React preview specimens exercise AgentMessage
  with ordinary markdown, links, streaming caret, and size/density examples.
  They do not provide source records or a citation consumer.
- [LF] No current Poodle downstream consumer with a citation/source-record
  contract was found in the checked-out package or preview tree. The AgentMessage
  contract names Figmatic, Loophole, and future agent surfaces as downstream
  adopters, but the checked-in Poodle consumer evidence does not establish a
  citation vocabulary for them.
- [WI] The first proof must therefore use a host-authored fixture and must not
  pretend that the preview specimen is a production source model.

## Adjacent source-record shapes

These were read from adjacent products in read-only checkouts to test boundary
pressure, not as Poodle API proposals. Their fields and vocabulary must not be
imported.

| Adjacent model | Captured revision and shape | What it proves for Poodle | What must stay out of Poodle |
| --- | --- | --- | --- |
| Figmatic workspace provenance | Figmatic commit `7deb780ea44dafb38a649b8805106ff709706bfc`; `crates/figmatic-studio/src/workspace/sources.rs` models a source with IDs, file/name fields, origin path, registration, and conversion state. Screens point back to a source and can carry a stable logical-screen identity. `convert/identity.rs` uses source/project/kind/name identity. | A consumer may need stable local provenance across conversion and reparsing. Opaque IDs and explicit lifecycle are more durable than display text. | File paths, conversion IDs, logical-screen IDs, project IDs, and Figmatic's internal `Source`/`Screen` vocabulary. They are not web citations. |
| Bovine external reference | Bovine Accelerator commit `253785bd1a55048e8c784d3c270f54fe2e446c0f`; `system/examples/exam-questions/external-reference.example.json` separates a reference key, source type/label, source period/number, title, path, and an external resolution status. Its ACCA registry records URL/index metadata and whether verbatim documents may be stored. | A consumer may need a display title and source status even when the authoritative content is external or unavailable. Licensing and storage status can be part of consumer governance. | Exam module, question number, source folder, ACCA registry fields, verbatim-storage flags, and any assumption that Poodle can fetch or store the source. |

[WI] The cross-product comparison supports a narrow Poodle shape: opaque
association IDs plus display metadata and explicit availability, with the
consumer retaining provenance, lifecycle, storage, and policy fields.

## Named example findings

### Beautiful UI

[SF] The official Streaming Text registry uses a citation-bearing token flag and
a separate source array containing display name, domain, URL, and image. The
source footer is a disclosure-like action and renders source links. This is a
useful composition pattern: source identity is separate from the streamed text
payload.

[SF] The same registry couples citation marks to a word-by-word stream and
fixed timing. The pinned repository README describes a rendering layer that
can be wired to a real agent. The timing and hold are demo mechanics, not
source semantics.

[WI] Reuse the separation of authored/rendered content from source metadata;
reject the token cadence as a citation contract. A source mark must remain
correct if the content arrives in one settled update, many streamed updates,
or a replacement after reparsing.

### AICSS Inline Citations

[SF] The official component uses authored numeric markers and a separate
reference list. Its public type has a number, label, host, and URL. Known
markers become links; unknown markers remain plain text. The compact marker and
footer are useful references for dense layouts.

[SF] The inspected implementation uses positional numeric matching. It also
uses new-tab links with `rel="noreferrer"`. Its tooltip source does not provide
the complete APG-style trigger-to-tooltip association in the observed code,
and its numeric marker is not a durable identity across content edits.

[WI] Reuse the authored-reference idea and compact source-list density. Do not
reuse numeric position as identity, or treat the observed tooltip implementation
as the accessibility bar.

### Transferable versus non-transferable behavior

| Behavior | Disposition | Reason |
| --- | --- | --- |
| Source records separate from prose rendering | **Compose** | Fits Poodle's host-owned data and reusable-composition boundary. |
| Authored inline reference occurrence | **Add, gated** | A generic reference mark may be useful outside agent prose, but needs a stable target association and a non-fragmenting default interaction model. |
| Compact source list with disclosure | **Add, gated** | Useful as a generic list/composite if title, status, and link semantics are host-neutral. |
| Numeric labels | **Consumer-owned display choice** | A consumer can author `[1]`, a letter, or another label. Poodle must not renumber or infer identity from it. |
| Token flag tied to word reveal | **Reject** | Cadence is presentation/demo behavior and is unstable under stream replacement. |
| Provider-specific fields or source fetching | **Reject in Poodle** | Violates generalized component scope and makes trust/licensing policy implicit. |

## Candidate semantic shape

The following is a research fixture shape, not a proposed public contract. It
is intentionally authored and renderer-neutral; it is not `MdInline`, a
markdown AST, or a provider source schema.

```ts
type InlinePart =
  | { kind: "text"; value: string }
  | { kind: "reference"; targetId: string; label: string };

type ReferenceTarget = {
  id: string;
  title: string;
  href?: string | null;
  availability: "available" | "unavailable";
};

type CitedMessageContent = {
  messageId: string;
  blocks: readonly { inlines: readonly InlinePart[] }[];
  targets: readonly ReferenceTarget[];
};
```

Required invariants for any future contract discussion:

- `messageId` is a host-stable namespace/key. It remains unchanged across
  streaming, markdown replacement, reparse, and settle for the same message.
- `targetId` is opaque and stable for the cited target. It is not derived from
  numeric order, occurrence index, href, title, or normalized source text.
- `label` is an authored visible mark. Poodle does not renumber it. Repeated
  occurrences can share one `targetId`; the source list should render one row
  per target according to an explicit host policy.
- `title`, `href`, and availability are supplied by the consumer. Missing
  target records remain unresolved and must not be rebound to a nearby target.
- A source without an admitted safe URL is a named, noninteractive/unavailable
  item. Poodle must never turn an unvalidated string into an anchor by default.
- The paragraph-only `blocks` in this smallest fixture are deliberate. A
  production shape must either use an authored block surface aligned with the
  existing semantic block contract or a composition slot; it must not expose
  Marked or pulldown-cmark nodes as the cross-runtime API.
- If a stream removes a target, the old mark remains associated with its old
  `targetId` and becomes unavailable until the host supplies a new explicit
  association. It must not silently follow a changed array position.

The shape leaves open whether a source row is per message or grouped at a
higher transcript level. That is a placement decision, not a parser detail.

## Recommendation matrix

| Surface or behavior | Recommendation | Rationale and boundary |
| --- | --- | --- |
| Raw `AgentMessage` markdown props/parser | **Reject direct extension** | Current parsing is re-run on every stream update and has no stable source identity. Adding `sources`, marker regexes, or parser offsets would make citation correctness depend on implementation details. |
| Authored semantic message-content seam | **Extend, conditionally** | A future message composition may need an authored input/slot that can carry reference occurrences without exposing markdown parser types. It requires a new cross-runtime contract and proof; no change is made here. |
| `AgentTranscript` item/source fields | **Reject** | Transcript owns log, grouping, virtualization, scroll, and focus. It should receive a composed message surface or host block, not infer or store citation records. |
| Generic inline reference mark | **Add, gated** | Add only if the first proof shows independent reuse. Default behavior should not create a tab stop for every repeated mark. |
| Generic source list/disclosure | **Add, gated** | A compact list with explicit availability and native disclosure semantics is reusable if titles, URLs, and actions remain caller data. |
| Agent citations as a whole | **Compose** | Compose authored prose, marks, and source list. Reuse message typography and link primitives where semantics fit; do not make the existing raw markdown path citation-aware by inference. |
| Source IDs, title, href, verification, availability, actions | **Consumer-owned** | Consumers know provenance, trust, authorization, licensing, and follow-up behavior. Poodle renders supplied semantics and does not fetch or verify sources. |
| URL admission and redirect policy | **Consumer-owned, enforced at the boundary** | The consumer validates and admits URLs. Poodle must have a rendering invariant that unsafe/unadmitted values cannot become anchors. Exact allowlist and redirect behavior require operator acceptance. |
| Streaming cadence, word reveal, shimmer, citation timing | **Reject** | These are presentation mechanics. They can reorder or delay semantic information and are explicitly outside the card's citation meaning. |
| Automatic numeric/markdown extraction | **Reject** | Numeric position and parser ranges are unstable under stream/reparse and cannot satisfy the identity requirement. |
| Beautiful UI/AICSS provider vocabulary, source fetching, copied assets | **Reject** | Violates generalized scope and licensing/provenance boundaries. |

## Runtime feasibility

### Svelte and React

[WI] Web runtime feasibility is good after the semantic input exists, but the
current components cannot supply it. A first implementation would need:

- authored content rendering that preserves `messageId`, `targetId`, and
  occurrence order through updates;
- a source-list disclosure and source rows using existing token/style systems;
- native anchors only for consumer-admitted URLs, with explicit `target`/`rel`
  behavior;
- deterministic behavior for unresolved and unavailable targets; and
- tests that distinguish stream replacement from a fresh message.

Existing `TextLink` props are sufficient for basic web row semantics. They are
not sufficient as a complete trust policy. A future contract should not add
provider-specific `CitationLink` fields to `TextLink`.

### Shared Rust and GPUI

[LF] The shared Rust markdown model cannot carry the proposed association
without a separate authored semantic-content model. The current renderer
flattens inline runs into `Node::text`, and the node vocabulary has no link or
reference kind. The GPUI node backend also cannot currently project
`NodeA11y` into an accessible native tree.

[WI] The active cohort can share the proposed *semantic* input only after a
renderer-neutral Rust declaration is added and the web/native adapters agree on
marks, source rows, unavailable state, disclosure state, and focus intent. The
Rust model must not include a markdown parser implementation detail.

GPUI can provide a headless construction/interaction proof for node order,
stable IDs, labels, and activation handlers. It cannot presently provide a
full mounted native AT proof. Per the native accessibility contract, do not
schedule a GPUI accessibility claim until the upstream accessibility API is
available. Jetstream is deferred; if it is later admitted, its AccessKit-backed
semantics become a separate promotion gate rather than evidence that GPUI is
complete.

### Consumer boundary

No checked-in Poodle consumer currently supplies source records. The first
proof must be host-authored and must include at least one independent consumer
shape before promotion. The Figmatic and Bovine examples demonstrate why
consumer models should adapt into the semantic fixture instead of becoming
shared Poodle fields.

## Accessibility and interaction recommendation

The recommended default is a nonfragmenting reading path:

1. Keep the transcript's existing `role="log" aria-live="polite"` as the only
   message-stream announcement owner. Do not put a live region on each message,
   mark, or source list.
2. Render authored reference labels as part of the message's readable text.
   Do not `aria-hidden` a visible mark unless an equivalent accessible
   representation remains in the same reading path.
3. Use one native disclosure button for a per-message source list by default.
   It exposes `aria-expanded` and controls the source-list region. Enter/Space
   toggles it; focus stays on the button.
4. Make source rows native links only when a safe href was admitted. Source
   rows are the normal tab sequence for full titles and hosts. An unavailable
   row is named text with an explicit unavailable status, not a dead anchor.
5. Treat direct activation of each inline mark as an optional mode, not the
   default. If enabled, every occurrence is a focusable link with a concise
   accessible name and repeated marks become repeated tab stops. The operator
   must accept that reading and traversal tradeoff.
6. If a tooltip is used for a short mark label, it must work on keyboard focus
   as well as hover, have a real trigger-to-tooltip association, and not be the
   only place where source identity is available. Escape dismissal and focus
   recovery must be tested.
7. Preserve ordinary text selection. Do not intercept selection to make a
   citation action work. A future copy projection must explicitly decide
   whether copied text includes the authored mark, source title, or both.
8. Use visual ellipsis only as a density aid. The accessible name must retain
   the useful full title/status, and a narrow viewport must not remove the
   source's meaning.
9. No required information may depend on word reveal, shimmer, tooltip timing,
   or layout animation. With reduced motion, disclosure and state changes
   remain understandable without transition effects.

This recommendation keeps repeated source details out of the stream announcement
while preserving a keyboard path to the full source list. It also leaves the
direct-mark activation decision visible instead of hiding it in a component
default.

## Explicit gates

### Runtime gate

- Svelte and React render the same authored fixture and preserve IDs/order
  across initial, streaming replacement, and settled renders.
- Shared Rust vectors cover repeated references, duplicate target IDs,
  unresolved targets, unavailable targets, and safe/unsafe href states.
- Renderer dumps or headless assertions prove the same semantic order, labels,
  target associations, disclosure state, and activation intent.
- GPUI has a headless construction/interaction result before promotion. A full
  native AT result is required only when the upstream accessibility path exists;
  the current GPUI gap is recorded, not waived.
- No windowed conformance selector is used locally. Use the repository's
  approved headless path if a future implementation requires conformance.

### Accessibility gate

- Keyboard review covers disclosure toggle, source-row traversal, link
  activation, tooltip focus/Escape if present, focus recovery, and no focus
  steal when the message stream appends.
- Automated accessibility checks cover names, roles, states, relationships,
  and no nested/conflicting live region.
- Manual screen-reader review confirms one settled message announcement at the
  transcript log, readable mark context, source-list discovery, unavailable
  status, and no repeated-source announcement storm.
- Web results are not generalized to GPUI. A mounted native proof must cover
  the same focus and announcement obligations before native promotion.

### Link-safety gate

- The consumer validates and admits URL schemes and canonical forms before
  passing an href. At minimum, `javascript:` and other executable schemes do
  not reach an anchor; the exact allowed scheme set remains an operator
  decision.
- A missing, rejected, or unavailable URL renders as noninteractive text with
  status.
- New-tab behavior has an explicit opener-isolation policy, such as
  `rel="noopener noreferrer"`, rather than relying on a demo default.
- Poodle does not fetch, redirect, canonicalize, verify, or cache source pages.

### Licensing and provenance gate

- Any future implementation keeps a source inventory for code, CSS, icons,
  images, fonts, and copied text.
- MIT-covered public code may be reused only with required notices and after
  dependency-level licensing review.
- Beautiful UI's pinned public repo and AICSS's public package surface are
  evidence sources, not permission to copy private/pro components, site-only
  material, third-party icon packs, or unlicensed assets.
- Consumer source titles, excerpts, images, and URLs remain the consumer's
  content/licensing responsibility. Poodle renders metadata; it does not
  ingest source pages.

### Promotion gate

An operator must accept all of the following before a contract or package card
is opened:

- stable `messageId`/`targetId` identity and lifecycle semantics;
- source-list placement and disclosure default;
- whether inline marks are noninteractive or focusable links;
- ownership of transcript announcement, citation accessibility, copy/selection,
  and follow-up actions;
- consumer URL-admission and redirect policy;
- unavailable/missing-target presentation;
- active-runtime scope, including the GPUI accessibility limitation and any
  later Jetstream admission; and
- licensing/provenance review ownership.

The promotion proof must include the dense/repeated, missing/unavailable,
unsafe-link, narrow, reduced-motion, stream-replacement, keyboard, and AT
fixtures listed below. A preview screenshot or a named-demo resemblance is not
enough.

## Smallest proof

This is a validation plan for a future implementation card, not implementation
work in this research lane.

1. **Fixture:** one settled authored message with three targets, one repeated
   target, one unresolved mark, one unavailable target, one rejected/unsafe
   href, dense marks, and long titles. Include wide and narrow containers and a
   reduced-motion preference.
2. **Web semantics:** render the same fixture in Svelte and React. Check the
   transcript log boundary, disclosure button state, source-row names, link
   relationships, unavailable text, DOM association IDs, and ordinary text
   selection.
3. **Stream replacement:** render partial content, replace it with a reparse,
   and settle it. Assert that stable IDs keep the same targets, removed targets
   do not rebind, no token-level live region appears, and focus is not moved.
4. **Keyboard and AT:** test Tab/Enter/Space/Escape paths, keyboard-focused
   tooltip behavior if present, screen-reader reading of marks and source rows,
   and one polite settled-message announcement. Use automated checks as a
   screen, not as the complete result.
5. **Rust projection:** use the same authored fixture in a shared vector. Assert
   target association, repeated occurrence order, unavailable state, and that
   the public shape contains no markdown parser node or provider field.
6. **GPUI proof:** assert node construction, source-row order, labels, stable
   IDs, disclosure/activation intent, and focus intent headlessly. Mark native
   AT as pending until the documented GPUI accessibility dependency is
   available; do not call this proof full native conformance.
7. **Motion:** assert that essential source information and disclosure state
   remain available with reduced motion. Do not assert word cadence or shimmer;
   those are rejected behavior.

The smallest promotion-worthy result is a semantic cross-runtime proof plus one
real consumer adapter. A second independent consumer shape should be used
before treating the data model as generalized.

## Unresolved operator decisions

These are intentionally recorded, not escalated during the research lane:

- Is the first promoted surface one `AgentCitations` composite, or are
  `InlineReference` and `SourceList` independently useful enough to promote as
  primitives?
- Is the source list per message, grouped for a transcript, or a consumer slot
  outside the message body?
- Are inline marks visual/noninteractive by default, or direct links with a
  tab stop for every occurrence?
- Who owns the accessible mark-to-source relationship and the copied-text
  representation?
- Which URL schemes and redirect/authentication policies qualify as admitted
  links, and where is that validation enforced?
- Does an unavailable target keep a row, collapse into a message-level status,
  or remain only as an inline unavailable mark?
- What authored semantic-content shape supports headings, code, lists, and
  blockquotes without exposing markdown parser types?
- Which consumer supplies the first real adapter, and what independent second
  shape is sufficient evidence of generality?
- When, if ever, does Jetstream enter the active cohort, and what mounted native
  accessibility proof is required alongside the current GPUI construction
  proof?

## Related

- Research card: [g16.038](../../roadmaps/g16/038-agent-citations-and-sources-research.md)
- Component contract: [AgentMessage](../../contracts/components/agent-message.md)
- Component contract: [AgentTranscript](../../contracts/components/agent-transcript.md)
- Component contract: [TextLink](../../contracts/components/text-link.md)
- Native accessibility: [contract 003](../../contracts/003-native-accessibility.md)
- Architecture: [Poodle system shape](../../architecture/001-poodle-system-shape.md)

## Follow-up

Open an operator-approved implementation/promotion brief only after the
identity, placement, accessibility, URL, licensing, and first-consumer gates
above are accepted. That brief should begin with the fixture and proof shape,
not with a markdown marker parser.
