# Desktop CodeEditor intake

Date: 2026-09-10
Source repository: `bovine-accelerator-desktop`
Source commit: `7c7f6f13864512189ba53949fd65534140c57597`
Source authority: Desktop contract 034 and g02.058

## Decision

Accepted `CodeEditor` as a new reusable Poodle component contract. Expanding
`MarkdownEditor` would mix Markdown presentation with exact general-text
editing. `Code`, `CodeInput`, and multiline `TextInput` also do not own syntax,
editor search, diagnostics, or bounded editor viewport behavior.

The contract is controlled, exact-text, renderer-neutral, and engine-neutral.
It includes semantic edits, a closed language domain, line numbers, local
search, attached diagnostics, keyboard/focus rules, read-only/disabled states,
explicit plain performance mode, and a 2 MiB UTF-8 compatibility envelope.
Desktop retains every source, draft, save, review, and recovery decision.

Tabs needs one adjacent contract amendment. `closable=false` prevents closing
Details but does not prevent global reorder. `TabItem.pinned="start"` fixes the
general seam: pinned tabs cannot move, and unpinned tabs cannot cross them.

## Package posture

No released package or accepted source pin contains this surface. The intake
does not authorize implementation, release, or Desktop adoption. The initial
implementation isolates its editor dependency behind a dedicated `./editor`
package subpath. Desktop g02.058 remains gated until that separately approved
work is merged and released or explicitly source-pinned.

## Operator follow-up

The operator confirmed CodeMirror 6 on 2026-09-10 and prioritized complete
TypeScript/web support. Svelte and React ship together; TypeScript syntax is in
the first language set. GPUI and shared Rust are a future admission and do not
block the web package. The component remains labelled web-admitted rather than
parity-complete.

The public contract now uses UTF-16 offsets and exact replacement edits,
matching CodeMirror without leaking its transactions. It drops guessed
edit-origin labels. CodeMirror stays private behind dedicated `./editor`
entries, with no arbitrary extension surface.
