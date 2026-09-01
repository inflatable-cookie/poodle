# Agent Citations Translation Memo

Status: ready for orchestrator intake; promotion held
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator
Source: [agent citations and sources dossier](../research/value-tracks/agent-citations-and-sources.md)
Research card: [g16.038](../roadmaps/g16/038-agent-citations-and-sources-research.md)
Promotion authority: orchestrator; merge is intake only

This packet translates the accepted citations research into one decision memo
for authored semantic content, inline reference marks, and a source list. It
keeps citation meaning outside raw `AgentMessage` markdown and outside
`AgentTranscript` records. No operator question is being reopened here.

## Settled Decisions Preserved

- Agent citations are a composition of authored prose, reference occurrences,
  and source rows.
- Source identity, provenance, verification, URL admission, redirects,
  authentication, availability truth, and follow-up actions stay with the
  consumer.
- Automatic numeric or markdown extraction, transcript-owned source records,
  streaming effects, word cadence, shimmer, source fetching, and provider
  vocabulary are rejected.
- No public `InlineReference` or `SourceList` primitive is admitted until the
  composition proves independent reuse.
- This packet is a translation recommendation, not a contract amendment,
  implementation card, or promotion decision.

## Translation Decision

### 1. Smallest candidate split

The first candidate is one per-message `AgentCitations` composition. Its
smallest proof is the dossier's paragraph/inline fixture: authored paragraphs,
inline text, and inline reference marks with one compact source list. It can
reuse `AgentMessage` typography, `TextLink`, and an ordinary disclosure control
where their existing semantics fit.

`InlineReference` and `SourceList` are composition roles first, not public
primitives. Admit either as a primitive only after it has a distinct second
use outside the agent-citation composition, with no agent/provider fields and
its own semantic proof. Current evidence does not meet that bar.

The citation seam may be extended only at an authored semantic-content
boundary. Do not add `sources`, marker regexes, parser offsets, or source
records to raw `AgentMessage` markdown or to `TranscriptMessage`. The
transcript receives a composed message surface; it does not infer citations
from child text.

### 2. Bounded citation fixture and identity

The shape below is the dossier's paragraph/inline planning fixture, not a
public TypeScript or Rust API. It carries citation identity through streaming
replacement without exposing `marked`, pulldown-cmark, or a provider schema:

```ts
type InlinePart =
  | { kind: "text"; value: string }
  | { kind: "reference"; targetId: string; label: string };

type ReferenceTarget = {
  id: string;
  title: string;
  href?: string | null; // present only when the host has admitted it
  availability: "available" | "unavailable";
};

type CitedMessageContent = {
  messageId: string;
  blocks: readonly { inlines: readonly InlinePart[] }[];
  targets: readonly ReferenceTarget[];
};
```

`blocks` is deliberately paragraph-only in this smallest proof. It does not
settle headings, code, lists, blockquotes, rules, or any other rich authored
content carrier.

Identity rules:

- `messageId` is stable for one message across partial content, stream
  replacement, reparsing, and settlement.
- `targetId` is opaque and stable. It is never derived from numeric order,
  occurrence index, URL, title, normalized source text, or a DOM key.
- `label` is authored visible content. Poodle never renumbers it. Repeated
  occurrences may share a target and collapse to one source row.
- Target definitions are unique within a snapshot. Duplicate definitions are a
  host/contract error; first-wins and last-wins behavior are not allowed.
- Replacing or reordering `targets` never retargets an old occurrence. A mark
  whose target disappears stays tied to its old `targetId` and becomes
  unresolved until the host supplies an explicit association.

### 3. Serial prerequisite: full authored semantic-content carrier

Production rich content needs a separate, serial architecture decision before
any implementation card opens. That decision must choose one route:

- an existing-contract-aligned authored semantic surface, derived from the
  current `AgentMessage`/shared semantic-block authority and extended only with
  the identity needed by authored references; or
- a documented composition-slot route whose authored content remains
  renderer-neutral and projects through the active runtime boundary.

This packet does not choose between those routes and does not define an
`AuthoredBlock` type. The full carrier is not settled, promotion-ready, or part
of the smallest citations seam. The paragraph/inline fixture above remains
the complete citation proof until that architecture decision is recorded in a
canonical destination. Citation identity, source-list, accessibility, URL,
and consumer-boundary decisions in this packet remain accepted inputs to that
later decision.

### 4. Source-list placement and ownership

The first placement is a source list inside the per-message composition,
following the authored message body and behind one disclosure button. It is
not a transcript-global registry. Per-transcript grouping or a consumer-owned
slot can be revisited only with evidence that message-local placement is not
reusable.

The disclosed list renders one row per distinct target in the authored/host
order. Repeated occurrences do not duplicate rows, and Poodle does not
renumber labels. A row receives the consumer's title, admitted URL state, and
availability. Provenance, verification receipts, redirect/authentication
state, storage/licensing state, and action callbacks remain consumer data.

The disclosure button is a native named control with `aria-expanded` and a
controlled source-list region. Enter and Space toggle it; focus stays on the
button. Source rows enter the normal tab sequence only when the list is
disclosed.

### 5. Inline-mark interaction

Inline marks are visual, noninteractive text by default. They do not create a
tab stop, intercept a click, or turn every repeated occurrence into another
navigation step. The authored label remains selectable and readable in the
message. The source list is the first keyboard route to source details.

Direct activation of every mark is outside the first promotion. If a later
consumer needs focusable mark links, that is a separate interaction decision
requiring a new proof for repeated tab stops, accessible names, focus return,
and source-list parity. A tooltip is not required for the first surface and
must never be the only source of identity; if added later, it must work on
keyboard focus and have a real trigger-to-tooltip relationship.

### 6. Mark-to-source accessibility

The composition owns the semantic association between each occurrence and its
target; the consumer owns the target's meaning and data. Renderers preserve
the stable `messageId`/`targetId` association and use deterministic occurrence
and row IDs derived from that namespace, never from render position.

The visible authored label stays in the message's accessible reading path. It
is not `aria-hidden`. The disclosed source row repeats that label with the
full title and availability status, so a reader can match the mark to its
source without a hover-only affordance or a description pointing at a hidden
collapsed row. The initial noninteractive mode does not create a direct ARIA
link from every mark to a collapsed row.

`AgentTranscript` remains the only message-stream announcement owner:

- keep `role="log" aria-live="polite"` at the transcript boundary;
- add no live region to a message, mark, or source list;
- do not announce token cadence or repeated-source details; and
- do not move focus when content appends or a source list opens.

Web automated checks and manual screen-reader review must cover names, roles,
expanded state, source-row relationships, unavailable status, one settled
message announcement, and absence of a nested/conflicting live region.

### 7. Unavailable and missing targets

- A target record with `availability: "unavailable"`, a missing admitted URL,
  or a rejected URL keeps its source row. The row is named text with an
  explicit unavailable status, never a dead anchor. Its inline marks remain
  the authored labels and are noninteractive.
- A reference with no matching target record remains an unresolved,
  noninteractive authored mark. The disclosed list shows one generic
  `Unresolved reference` row per distinct missing `targetId`, using the
  authored label as its only source-facing text and carrying no invented title,
  URL, or provenance. This row is a rendering of missing association state,
  not a synthetic source record.
- A stream removal does not make a mark follow the next target array position.
  It remains unresolved until an explicit host update restores the same
  association or supplies a new authored occurrence.
- Ellipsis may reduce visual density, but it must not remove the useful title,
  label, or status from the accessible name at a narrow width.

### 8. Copied text and selection

The first copy rule is ordinary selection. Do not intercept selection to make a
citation action work, and do not add a copy button to `AgentCitations`.

The message copy projection contains the authored prose and each authored mark
label in document order. If the host authors `[1]`, `[1]` is copied; Poodle
does not renumber or replace it. Generated source titles, URLs, availability
labels, and provenance are not appended to message copy. A user can select and
copy the disclosed source list separately. Any future explicit copy action
must declare a new projection rather than silently changing this rule.

### 9. URL rendering invariant

The consumer validates and admits URLs before they reach the composition.
Poodle does not fetch, canonicalize, verify, redirect, authenticate, or cache a
source page. The consumer owns the exact allowed scheme set and redirect/
authentication policy; executable schemes such as `javascript:` must never be
admitted.

The renderer invariant is strict: an anchor exists only when the target is
available and carries a consumer-admitted safe `href`. A missing, rejected, or
unavailable URL renders noninteractive text with status. It must not be passed
to `TextLink`'s no-`href` button fallback, because that would turn an
unavailable source into an accidental control.

When a consumer requests a new browsing context, opener isolation is explicit
in its `target`/`rel` policy, including `noopener` (and `noreferrer` when the
consumer's policy requires it). Poodle does not infer trust from a title,
domain, redirect result, or demo default.

### 10. First consumer evidence

The first proof must use a host-authored fixture and one real downstream
consumer adapter. The in-repo preview specimen is useful for rendering checks,
but it is not consumer evidence and must not define a production source model.

The adapter must map its own source records into the generic authored shape
without importing product fields into Poodle. It must exercise at least one
settled message and a stream replacement, a repeated reference, an unavailable
target, a missing target, and an admitted versus rejected URL. The dossier's
Figmatic and Bovine comparisons explain why this adapter boundary matters;
their field names are not Poodle vocabulary.

A second independent consumer shape is required before treating the data model
as generalized or admitting either generic primitive. No checked-in Poodle
consumer currently supplies citation records, so this absence is an explicit
promotion hold, not permission to use a preview fixture as a substitute.

## Cohort Boundary

The bounded paragraph/inline citation proof covers the current Poodle
implementation cohort: Svelte, React, renderer-neutral shared Rust composition,
and GPUI through `poodle-node`. Parity means the same authored inputs, identity,
states, interaction intent, and token usage; it does not mean one parser or four
copied implementations. The full authored rich-content carrier has no settled
cohort contract until the serial architecture prerequisite is resolved.

- Svelte and React must render the same paragraph/inline fixture and preserve
  target associations through initial, streaming-replacement, and settled
  snapshots.
- Shared Rust vectors must cover the fixture's repeated,
  duplicate-definition, unresolved, unavailable, and safe/unsafe URL states
  without markdown-parser or provider fields in the public shape.
- `poodle-render` and headless GPUI proof must show paragraph/inline order,
  labels, stable IDs, source-row state, disclosure intent, activation intent,
  and focus intent.
- The current GPUI 0.2.2 boundary cannot provide a mounted native accessibility
  tree. Headless construction is evidence of structure/intent only; it is not
  mounted assistive-technology proof and must not be described as such.
- Jetstream remains a deferred backend, even though its separate AccessKit
  path is documented. No Jetstream result is used as active-cohort evidence or
  treated as passing until its admission runway is complete.

No windowed conformance selector is needed for this planning lane. If a future
implementation requires conformance, use the repository's approved headless
path.

## Promotion and Hold Oracles

### Promotion oracle

Promotion can open a bounded contract/package card only when every item below
is true. The full authored semantic-content carrier is a serial prerequisite,
not a decision this citation packet silently supplies:

1. A separate architecture decision has selected and canonically recorded
   either an existing-contract-aligned authored semantic surface or a
   composition-slot route for the full authored semantic-content carrier. No
   implementation card opens before that prerequisite is resolved; the
   citations proof remains bounded to paragraph/inline content.
2. The authored `messageId`/`targetId` lifecycle is explicit and survives
   stream replacement, reparse, removal, and settlement without positional
   rebinding.
3. Per-message source-list placement, noninteractive mark default, disclosure
   keyboard behavior, mark-to-source reading path, copy projection, and
   unavailable presentation are all fixed in the contract.
4. The consumer URL-admission boundary and new-context opener policy are
   documented; no unsafe value can reach an anchor.
5. The dense/repeated, missing/unavailable, unsafe-link, narrow, reduced-motion,
   stream-replacement, keyboard, ordinary-selection, and assistive-technology
   paragraph/inline fixtures pass in Svelte and React, with the same semantic
   cases in shared Rust and a headless GPUI construction/interaction proof.
6. One real consumer adapter passes the fixture. A second independent consumer
   shape exists before the data model is called generalized.
7. Each proposed public primitive has an independent reuse case. Otherwise
   `InlineReference` and `SourceList` remain internal roles of
   `AgentCitations`.
8. Licensing/provenance review accounts for code, CSS, icons, images, fonts,
   copied text, and consumer source metadata. Named demos remain evidence, not
   copy permission.

### Hold oracle

| Invariant | Smallest adversarial counterexample | Expected failure or stop | Required proof |
| --- | --- | --- | --- |
| Full carrier has an explicit architecture owner | The citation packet introduces an undefined `AuthoredBlock` for headings, code, lists, or other rich content | Stop; no production rich-content implementation card opens until the architecture decision chooses the existing-contract-aligned or composition-slot route | Separate architecture decision recorded before any implementation card |
| Identity is authored | A streamed reparse inserts text before `[1]` and a positional marker now points at source B | Stop; never infer a new target from position, number, URL, title, or DOM key | Svelte/React replacement trace plus shared vector |
| Removed targets do not rebind | Target A disappears and the next target array entry is B | Stop with the old mark unresolved; no silent retarget | Stable-ID stream fixture |
| Source rows are scoped | A transcript-level registry merges two messages that reuse a local target ID | Hold per-message composition; no global source table without a new namespace contract | Two-message isolation test |
| Marks do not fragment reading | Ten repeated marks create ten tab stops or token-level announcements | Hold interaction/accessibility result | Keyboard trace and transcript live-region audit |
| Source discovery is accessible | A visible mark is `aria-hidden`, or its only title is a hover tooltip | Hold; preserve the label in reading flow and disclose full status | DOM/role/name checks plus screen-reader review |
| Missing and unavailable are honest | A missing target receives a nearby title or an unavailable row becomes a link | Hold; retain unresolved/unavailable text and status | Missing/unavailable fixture and source-row assertions |
| Unsafe values cannot navigate | `javascript:` or a rejected URL reaches an anchor, or no-`href` `TextLink` becomes a button | Hold link-safety gate | Consumer admission test and rendered-node audit |
| Copy preserves authored meaning | Copy drops `[1]`, renumbers it, or appends an unrequested title/URL | Hold copy projection; ordinary selection remains intact | Selection/copy fixture in both web renderers |
| Transcript owns announcements | A message or source list adds a second live region, or append moves focus | Hold accessibility gate | Accessibility tree and focus trace |
| Runtime claims match evidence | Web checks pass and the result is called mounted GPUI AT parity, or Jetstream is called active | Hold wording and cohort claim | Rust/GPUI evidence review against contract 003 and architecture 001 |
| Public split is justified | `InlineReference` or `SourceList` has only the one agent-citation use | Keep it inside the composite; do not admit a primitive | Independent reuse inventory and focused proof |
| Consumer boundary is preserved | Poodle fetches, verifies, redirects, authenticates, or imports a provider field | Stop scope expansion | Changed-surface and export audit |
| First proof is real | Only a Poodle preview screenshot or synthetic specimen exists | Hold promotion; request a real host adapter | Downstream adapter receipt and fixture run |
| Licensing is clean | Demo CSS, icons, images, private/pro code, or copied source prose enters the package | Stop and route licensing review | Source inventory and notice/dependency review |

## Alternatives Not Selected

| Alternative | Reason |
| --- | --- |
| Parse `[1]`, markdown ranges, or rendered DOM positions automatically | Streaming markdown is reparsed and has no durable inline identity; position-based association can silently change source. |
| Add `sources` or citation fields to `AgentMessage` or `TranscriptMessage` | Message parsing and transcript log/window/focus ownership are the wrong boundary; source records would become coupled to transport and virtualization. |
| Make every inline mark a direct link by default | Repeated marks fragment reading and keyboard traversal; the source list provides one compact route. |
| Put source records in a transcript-global list first | It needs cross-message identity and grouping policy that the smallest per-message composition does not need. |
| Treat word reveal, token timing, or shimmer as citation state | These are presentation mechanics and are unstable under stream replacement. |
| Adopt Beautiful UI/AICSS provider fields, fetching, or demo assets | Their behavior is useful evidence, but provider vocabulary, source trust, licensing, and assets belong elsewhere. |
| Append source titles or URLs to every message copy | It changes ordinary selection and duplicates source-list content without a host request. |
| Promote from a preview-only Svelte specimen | A specimen cannot prove consumer ownership, cross-runtime semantics, or independent reuse. |
| Define a full `AuthoredBlock` in this packet | It silently creates a parallel rich-content model and makes the smallest citation seam depend on an unresolved architecture choice. |
| Extend the current GPUI or Jetstream boundary in this lane | GPUI lacks a mounted native accessibility tree; Jetstream remains a separately deferred backend. |

## Evidence Used

- [AgentMessage contract](../contracts/components/agent-message.md): raw
  markdown is parsed per update; current blocks and links carry no citation
  identity, and the message body is not a live region.
- [AgentTranscript contract](../contracts/components/agent-transcript.md): the
  transcript owns the append-only log, virtualization, focus, and polite
  announcement boundary; `TranscriptMessage` is not a source store.
- [TextLink contract](../contracts/components/text-link.md): native anchor
  semantics exist for admitted `href` values, while the no-`href` path is a
  button and therefore cannot be used as unavailable-source text by accident.
- [Poodle system shape](../architecture/001-poodle-system-shape.md),
  [product guardrails](../architecture/product-guardrails.md), and [working
  rules](../contracts/001-working-rules.md): generalized composites belong in
  Poodle; consumers own routing, fetching, authorization, provenance, and
  product vocabulary; active parity is semantic across Svelte, React, Rust,
  and GPUI.
- [Native accessibility contract](../contracts/003-native-accessibility.md):
  GPUI 0.2.2 cannot expose a mounted native accessibility tree; Jetstream's
  backend is separately deferred by the architecture.
- [g16.038 dossier](../research/value-tracks/agent-citations-and-sources.md):
  Beautiful UI and AICSS behavior observations, pinned public-repository and
  licensing limits, Figmatic/Bovine consumer-boundary comparisons, the bounded
  paragraph/inline stable-ID fixture, accessibility recommendation, link-safety
  gate, cohort feasibility, and smallest proof.
- [Beautiful UI Streaming Text](https://www.beautifului.dev/r/streaming-text.json),
  [AICSS Inline Citations](https://www.aicss.dev/components/inline-citations),
  WAI-ARIA/APG, OWASP, and MDN are behavior and safety evidence only. No code,
  CSS, icons, private/pro material, or source prose is copied.

## Proposed Canonical Destinations

| Meaning | Destination after separate intake and promotion |
| --- | --- |
| Bounded paragraph/inline citation fixture and identity/source semantics | A citation composition/contract surface limited to the dossier fixture; no full authored block carrier is implied |
| Full authored semantic-content carrier | A separate serial architecture decision choosing an existing-contract-aligned semantic surface or a composition-slot route; canonical architecture/contract destination must be selected before any implementation card |
| `AgentCitations` behavior after the prerequisite | Agent-message/composition contract, with the active Svelte/React/Rust/GPUI surface named explicitly |
| Transcript announcement, focus, and virtualization boundary | `AgentTranscript` contract; no transcript-owned source records |
| Admitted anchors and unavailable text | Composition contract over existing `TextLink`; URL admission and redirect/auth policy remain in the consumer |
| Stable paragraph/inline vectors and renderer proof | Shared core/Rust/headless vectors, `poodle-render`, `poodle-node`, and focused Svelte/React/GPUI evidence; rich-content vectors wait for the serial prerequisite |
| First consumer adapter | Owning downstream product; its source schema and provenance remain outside Poodle |
| Promotion sequencing and hold state | One future bounded roadmap card and completion log after the serial prerequisite and orchestrator intake; no implementation card is ready from this packet alone |

No contract, package, roadmap, consumer, or research file is changed by this
packet. The packet is the complete translation deliverable; its PR remains
unmerged pending orchestrator review and the explicit promotion gates above.
