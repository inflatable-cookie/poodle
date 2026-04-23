# g06.011 — Composite Specs: Editing, Media, and Operational

Status: Completed
Updated: 2026-03-14

> Historical note: this milestone records the pre-`g10.004` split component
> spec crates. The live merged crate is now `poodle-specs`.

## Objective

Add 12 new composite spec structs for editing, media, and operational components.

## New Specs

| Spec | Category |
|------|----------|
| `AudioPlayerSpec` | Media |
| `VideoPlayerSpec` | Media |
| `MediaPickerSpec` | Media |
| `MarkdownEditorSpec` | Editing |
| `BlockEditorSpec` | Editing |
| `EmbedInputSpec` | Editing |
| `EmbedPreviewSpec` | Editing |
| `EmbedShellSpec` | Editing |
| `LogListSpec` | Operational |
| `PageLoadingSpec` | Operational |
| `StateTileSpec` | Operational |
| `ToastStackSpec` | Operational |

## Running Total

Composite specs: 16 (existing) + 12 (new) = **28**

## Verification

- [x] All 12 specs compile and are exported from `poodle-composites`
- [x] All 10 existing tests continue to pass
