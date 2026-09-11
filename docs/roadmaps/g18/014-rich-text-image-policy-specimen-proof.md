# 014 — Rich-text image-policy specimen proof

Status: ready behind g18.013 — operator-confirmed specimen repair
Owner: Poodle web quality
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/rich-text-editor.md`,
`../../../packages/svelte/preview/src/specimens/RichTextEditorSpecimen.svelte`,
`../../../packages/react/preview/src/gallery/specimens/RichTextEditorSpecimen.tsx`,
`../../../packages/svelte/preview/src/specimens/web-editor-documents.ts`
Depends on: g18.013 Queue task `e3a0e8cb-287c-447d-90c3-c226bca3d763`
complete and merged

## Outcome

Make the paired RichTextEditor Image Policy specimens visibly and
deterministically prove the optional image contract. “Images on” shows a seeded
image without network access. “Insert image” invokes the host-owned request,
adds exactly one visible image at the retained selection, and updates an
inspectable host document/count. “Images off” shows neither image content nor
an insertion command.

Repair the smallest engine defect only if a planted end-to-end test proves the
request/insertion path itself is broken after the fixture becomes loadable.

## Ready-State Rubric

- [x] Both specimens provide `requestImage` and an image-enabled document.
- [x] The seeded document and callback both use
  `https://x.test/chart.png`, a non-resolving test URL, so rendered output and
  insertion appear inert to a human.
- [x] The operator reproduced the failure and approved repair.
- [x] g18.013 already owns the same paired specimen files and must merge first
  to avoid concurrent edits.
- [x] g18.011 remains Queue-held until this visible proof is accepted.

## Decisions

- Use a deterministic repository-owned preview asset or self-contained safe
  raster data URL. The specimen must not depend on DNS, an external service,
  mutable remote content, or permissive development networking.
- The seeded image and inserted image must be visibly distinct or carry an
  explicit insertion marker/count so a click cannot look like a no-op.
- Show compact host feedback beside the specimen: image count and the current
  controlled document, or an equivalent inspectable projection. Do not require
  DevTools to understand whether the callback ran.
- Keep asset choice host-owned. The specimen’s `requestImage` is a deterministic
  stand-in for a consumer media picker; Poodle still does not upload, browse,
  proxy, rewrite, or persist images.
- Toggle transitions remain truthful. Images-off mounts a document valid for
  the standard schema; images-on restores the image-enabled controlled
  document without callback echo or silent node stripping.
- Svelte and React use the same fixture data and observable behavior.

## Dispatch manifest

- **State:** dependency-queued behind g18.013; serial before held g18.011 and
  retained g18.006; g18.009 remains held
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** shared web-editor specimen documents/assets; paired
  Svelte/React RichTextEditor specimens; focused preview/browser tests and test
  helpers; RichTextEditor image request/insertion engine path only if the
  visible fixture proves a real bounded engine defect; one g18.014 log
- **Reserved closeout surfaces:** public rich-text schema/feature/command
  domains; toolbar presentation owned by g18.013; renderer behavior; g18 README,
  index and dispatch; g18.006/g18.009 and g18.010–g18.013 task/workspace/PR
  state; versions, release/tag/publication, Desktop, native/GPUI/Jetstream
- **Worker:** paired-preview/browser worker comfortable with controlled
  ProseMirror documents, deterministic assets, asynchronous host callbacks and
  visual assertions
- **Excluded:** media picker/upload UI; external network fixtures; image
  resizing/cropping; schema redesign; embeds; unrelated editor UX; release
- **Escalation:** Chatterbox if the failure requires a public image API/schema
  change, security-policy weakening, external service, or work outside the
  bounded request/insertion path

## Work

1. Bind the current failure in both previews: images-on contains an `<img>`
   whose `src` cannot load, and Insert image cannot produce visible evidence.
2. Replace the dead URL with one deterministic same-repository or safe raster
   fixture shared by both frameworks. Prove it loads with non-zero rendered
   geometry and the expected alt text.
3. Add visible controlled-host feedback. Toggle images on/off/on and prove the
   configured documents remain valid without callback echo or data loss.
4. Place a selection, invoke Insert image, settle the async request, and assert
   exactly one document change and one additional visible image with the
   callback’s exact attributes. Repeat after remount.
5. Prove images-off omits Insert image and rejects no ordinary standard
   document. Prove cancellation and rejected/unsafe results remain inert in
   focused tests without weakening the existing fail-closed policy.
6. If step 4 fails with a loadable fixture, repair only the demonstrated engine
   request/insertion defect and bind the regression in both wrappers.
7. Run focused preview/component/browser checks, both preview builds, relevant
   accessibility checks, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Seeded image is visible | DOM contains an `<img>` with zero geometry or failed network state | loaded image, expected alt text and non-zero geometry in both previews |
| Demo is deterministic | image depends on `x.test`, the public internet or mutable remote content | offline-capable fixture and zero external image requests |
| Insert image visibly works | callback runs but broken image makes the click appear inert | before/after document and rendered-image counts increase by exactly one |
| Controlled state is honest | editor mutates internally but host document stays unchanged | one `onChange` with exact inserted attrs and matching visible readout |
| Selection is retained | async callback inserts at an unrelated fallback location | planted selection and exact surrounding document order |
| Feature gate is real | images-off still shows Insert image or carries an image node | paired off/on DOM, toolbar and document assertions |
| Toggle does not corrupt data | off/on silently strips the image-enabled document | preserved image document and zero callback echo across remounts |
| Failure stays closed | cancelled or unsafe source inserts a partial/broken node | focused cancellation/rejection assertions with unchanged count/document |
| Frameworks agree | only Svelte displays or inserts the fixture | shared data plus paired browser journeys |
| Sweep remains gated | g18.011 starts with an inert Image Policy specimen | merged repair before Queue hold release |

## Stop conditions

- Stop if the repair requires changing the public image node shape or enabling
  embeds.
- Stop before adding a network-backed asset or weakening URL admission.
- Stop before release, Desktop, native, or retained-task mutations.

## Evidence

Inspection on 2026-09-11 found the intended host-owned callback in both
specimens and the insertion path in both engines. The shared seeded document
and callback return the same non-resolving `https://x.test/chart.png`; this
explains why neither the initial image nor a newly inserted one is visible.

## Next task

After g18.010, g18.013, and this repair merge, Chatterbox releases held g18.011
for the complete three-surface acceptance sweep. Keep g18.006 blocked and
g18.009 held.
