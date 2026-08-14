# Native visual gate

Screenshots every GPUI preview component and diffs it against a machine-local
baseline. Capture workflow repaired under g14.002.

## Why this is shaped differently from `test/visual/`

The web gate diffs Svelte against React and needs no baselines: both emit the
same DOM from the same stylesheet, so any difference is a bug by construction.

The native targets have no twin. GPUI is a different renderer with its own
shell, font stack and compositor — a diff against Svelte would be all noise. So
this gate compares each component against **its own local reference image**. It
answers "did this edit move native rendering?", which is the question the
structural gates cannot ask.

## Capture stability

A capture is accepted only when **two consecutive attempts agree** within
`MAX_DIFF_RATIO`. The preview waits for its first render before screenshotting;
without the double-capture check, incomplete frames freeze into baselines.

If a component never settles, it belongs in the skip list in `config.ts`.

## Control size

The gate always passes `--control-size` to the preview. The preview accepts
`--control-size` as the canonical flag (`--size` remains a synonym). A focused
run with a non-default size must prove the preview received it — do not infer
size only from pixels.

```sh
bun test/native-visual/run.ts --slug=button --control-size=lg
```

## Baselines are local, not committed

Both `baselines/` directories are gitignored. They are transient reference
images. Do not add large PNG sets to Git.

## Modes

**Compare (default)** is read-only. A missing baseline fails and prints the
exact refresh command. Comparison never writes a new reference.

**Refresh** replaces the baseline after preserving the previous PNG as
`*.previous.png` and emits `test/native-visual/out/refresh-manifest-<axis>.json`
with slug, axis, dimensions, old/new hashes, paths, and reason.

```sh
effigy test:native-visual
effigy native-visual:refresh

bun test/native-visual/run.ts
bun test/native-visual/run.ts --slug=button,pagination
bun test/native-visual/run.ts --refresh --control-size=sm --reason='stale baseline reclassify'
```

`--update` is an alias for `--refresh`.

Diffs land in `test/native-visual/out/`; baselines live in
`packages/gpui/preview/baselines/`. Before, after, and diff evidence remain
available until review — refresh does not silently overwrite the only capture.

A bulk refresh is not proof that current rendering is correct. Reclassify stale
baselines through the explicit refresh flow and inspect the preserved previous
image plus the manifest.

## Local-only

The GPUI preview screenshots its own window — it finds itself by PID through a
CoreGraphics lookup and shells out to `screencapture`. That needs a live macOS
window-server session, so this is not in `ci:web` or `ci:native`.

## Skipped components

Anything inherently non-deterministic — `spinner` and `page-loading`
(animation), `time-ago` (wall-clock copy), `audio-player` and `video-player`
(media surfaces). They are listed in `config.ts` with reasons and named in the
run summary.
