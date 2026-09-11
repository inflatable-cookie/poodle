# g18.014 — RichTextEditor Image Policy specimen proof

Status: merged
Merge: `72c7a9e5e6ce288780c7a2e3e44715949a035221` (PR #249) on 2026-09-11
Date: 2026-09-11
Branch: `ns-5b42e240-5e63-4bdd-8035-7f2285776c6e`
Card: `docs/roadmaps/g18/014-rich-text-image-policy-specimen-proof.md`
Handoff: `docs/handoffs/20260911-g18-014-rich-text-image-policy-specimen-proof.md`
Governing refs: `docs/contracts/components/rich-text-editor.md`,
`packages/svelte/preview/src/specimens/RichTextEditorSpecimen.svelte`,
`packages/react/preview/src/gallery/specimens/RichTextEditorSpecimen.tsx`,
`packages/svelte/preview/src/specimens/web-editor-documents.ts`
Base: `origin/main` at `bf1bbd900b0bab5bda5dec84fe30f5d18a6b07b3`
(g18.019 closeout; rebased after g18.013 merged as `1e11f59d0`)

## Outcome

The paired RichTextEditor Image Policy specimens now prove the optional image
contract visibly and deterministically. The shared dead `https://x.test/chart.png`
source is gone. Both previews seed a self-contained 96x48 PNG carried as a
`data:image/png` URL, render a host-visible image document readout (image count,
host request count, host change count, controlled JSON), and stand in for a
consumer media picker with an async `requestImage` that returns a visibly
distinct amber fixture after a 300 ms delay.

A click on Insert image therefore produces visible, inspectable evidence: one
additional decoded image, one host document change, one host request, and a host
count that goes 1 → 2. Images-off still mounts the ordinary standard document,
shows no image and no insertion command, and retains the image-enabled host
document across the remount with no callback echo and no silent node stripping.
No engine defect appeared once the fixture became loadable, so no engine,
schema, feature, command, or contract change was made.

## What changed

- `web-editor-documents.ts`:
  - `RICH_TEXT_IMAGE_SRC`/`RICH_TEXT_IMAGE_ALT` are now the seeded 96x48
    indigo-striped inline PNG fixture plus `RICH_TEXT_IMAGE_ALT` unchanged
    (`Revenue chart`).
  - New `RICH_TEXT_PICKED_IMAGE_SRC`/`RICH_TEXT_PICKED_IMAGE_ALT` (amber,
    `Revenue chart (host pick)`) — the fixture the specimen's host-owned
    `requestImage` returns, visibly distinct from the seeded one.
  - New `RICH_TEXT_IMAGE_REQUEST_DELAY_MS = 300`: the media-picker stand-in is
    asynchronous, which also gives the retained-selection proof a pending window.
  - `RICH_TEXT_IMAGE_DOCUMENT` gains one ordinary trailing paragraph
    (`Images stay host-owned: the consumer picks the source.`) so a planted
    selection has a deterministic insertion target and order.
  - New `countRichTextImages(document)` shared by both specimens.
- Paired specimens expose identical host feedback
  (`data-part="image-count"`, `"image-request-count"`, `"image-change-count"`,
  `"image-host-document"`) with identical classes and copy, toggling state
  (`imagesOn`, `imageRequests`, `imageChanges`) and the same fixture data.
- Paired preview tests:
  `packages/svelte/preview/test/g18-014-rich-text-image-policy-specimen-proof.test.ts`
  and `packages/react/preview/test/g18-014-rich-text-image-policy-specimen-proof.test.tsx`
  (5 cases each) pin the fixture integrity (PNG magic, no `http(s)` URL in the
  image document, two distinct fixtures), images-off truth, seeded load,
  insert-exactly-once with the callback's exact attributes, and the toggle
  remount with no echo.
- `test/rich-text-image-policy/probe.ts` plus
  `test:rich-text-image-policy{,-chromium,-webkit}` in `tasks/effigy.tasks.toml`:
  a headless Chromium + WebKit journey over the real public
  `#components/rich-text-editor` preview routes in both frameworks (36 checks
  per framework per engine).

## Validation

Recorded at the branch head on `ns-5b42e240-5e63-4bdd-8035-7f2285776c6e` after
rebasing onto `origin/main` `bf1bbd900`.

- `bunx vitest run --project svelte-preview --project react-preview`:
  25 files, 116 tests pass, including the 10 new g18.014 cases.
- `effigy test:rich-text-image-policy-chromium` and
  `effigy test:rich-text-image-policy-webkit`: 72/72 checks each, exit 0.
  Both frameworks prove the seeded and inserted fixtures decode (`naturalWidth`
  96 x 48, non-zero box), one click yields exactly one more visible image and
  exactly one host document change at the paragraph the caret was in when
  Insert image was pressed (the caret is moved to another paragraph while the
  request is pending), the toggle remount restores both images with no echo or
  re-request, and the run issues zero external requests and zero image
  requests at all.
- `effigy test:components`: 413 files, 4064 tests pass, exit 0.
- Focused component suites: Svelte `RichTextEditor.test.ts` 58 pass; React
  `RichText.test.tsx` 50 pass. These retain the existing engine-level
  cancellation, rejection, unmount, feature-disable, and unsafe-source
  fail-closed assertions that the specimen callback cannot express.
- `effigy test:a11y`: 183 pass, exit 0.
- `effigy svelte:build` and `effigy react:build`: exit 0.
- `effigy docs:check`: full sequence exit 0 (includes `gate:clean`, so the
  writer tasks left the tree clean).
- `effigy check:svelte-preview`: 0 errors, 6 pre-existing warnings in 4 files.
- `effigy check:react-preview` remains red repo-wide on the pre-existing
  string/`ControlSize` specimen backlog (265 errors, none in
  `RichTextEditorSpecimen.tsx` or `web-editor-documents.ts`; unchanged from the
  pre-branch count at this base), matching the g18.013 record.
- Visual parity (headless sweep tier with `--slug=rich-text-editor,rich-text-renderer`):
  4 pairs compared, 0 failing, at the exact head. The `RichTextRenderer`
  specimen shares the same document module, so its image now loads too.
- `git diff --check`: clean.

## Remaining limits

Web-admitted only; native/GPUI/Jetstream image parity is out of scope. The
request delay and the host feedback counters are specimen host behavior, not
Poodle API: Poodle still does not upload, browse, proxy, rewrite, persist, or
admit external image sources, and URL admission is unchanged. Cancellation and
unsafe-source refusal remain proven by the paired component suites rather than
by the specimen, whose callback is deliberately deterministic. The new browser
probe needs built `packages/core/dist` and the two preview dev servers, like
the existing `test:visual` and `test:*-browser` probes, so it stays out of
`ci:web`.

## Closeout

- Merge performed by the plugin as
  `72c7a9e5e6ce288780c7a2e3e44715949a035221` on 2026-09-11 (PR #249),
  with parents `2018876d10686f1dc2a36114720527a42a4cbe6d` (main) and
  `71f2d3104a6bfb54907521c11266cf18f678979c` (reviewed head).
- Accepted review: independent exact-head `ready_to_merge` approval of
  head `71f2d3104a6bfb54907521c11266cf18f678979c` by betterthanclay
  ([comment #5633674856](https://github.com/inflatable-cookie/poodle/pull/249#issuecomment-5633674856)).
  No blocking findings; two non-blocking notes (paired-but-different
  internal feedback class names, and the 300 ms delay plus host counters
  being specimen-host behavior rather than Poodle API), both safe to defer.
- Reviewed-head validation (reviewer ran at the exact head, tree left
  clean): Svelte + React preview suites 25 files / 116 tests pass
  (includes the 10 new g18.014 cases);
  `effigy test:rich-text-image-policy-chromium` and
  `effigy test:rich-text-image-policy-webkit` 72/72 checks each;
  `effigy test:components` 413 files / 4064 tests pass;
  `effigy test:a11y` exit 0; `effigy svelte:build` and
  `effigy react:build` exit 0; `effigy docs:check` exit 0;
  `effigy check:svelte-preview` 0 errors with 6 pre-existing warnings;
  `effigy check:react-preview` red on the pre-existing repo-wide
  string/`ControlSize` specimen backlog (265 errors, verified identical on
  `main`, none in the touched files); visual sweep tier 4 pairs compared,
  0 failing; `git diff --check` clean; CI `rust` and `web` pass on PR #249.
- Worker validation at the branch head is recorded above under Validation;
  the merged head adds no source over the reviewed head (merge only).
- Deferred: no release, tag, publish, Desktop, native, or retained-task work
  starts from this task. `g18.011` stays held until parallel g18.012 and
  g18.018→g18.020 converge on it; then operator acceptance and retained
  `g18.006`. `g18.009` stays held.

## Handoff

g18.011 stays held: this repair is accepted evidence only once reviewed and
merged. g18.006 and g18.009 are untouched. No release, tag, publish,
version, Desktop, or native work was performed.
