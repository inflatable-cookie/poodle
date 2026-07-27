# Native visual gate

Screenshots every GPUI preview component and diffs it against a committed
baseline. Roadmap: `docs/roadmaps/g12/014`.

## Why this is shaped differently from `test/visual/`

The web gate diffs Svelte against React and needs no baselines: both emit the
same DOM from the same stylesheet, so any difference is a bug by construction.

The native targets have no twin. GPUI is a different renderer with its own
shell, font stack and compositor — a diff against Svelte would be all noise. So
this gate compares each component against **its own committed image**. It
answers "did this edit move native rendering?", which is the question the
structural gates cannot ask and the one that was open every time this repo
changed a native renderer.

A single capture is not reliable. The preview waits a fixed 1.5s for its first
render and screenshots whatever is on screen, which sometimes catches an
incomplete frame — one `segmented-control` baseline had the selected segment
barely painted, and sat there as the reference image until a fresh capture
disagreed with it.

So a capture is accepted only when **two consecutive attempts produce identical
bytes**. That costs 2x per component and is worth it: the alternative is raising
the tolerance until bad frames pass, which lets real regressions through beside
them.

If a component never settles, it is genuinely non-deterministic and belongs in
the skip list. `progress` was caught exactly that way.

## Baselines are local, not committed

Both `baselines/` directories are gitignored. They are transient reference
images, and committing them was costing real storage — the GPUI set alone is
103MB of full-window screenshots, and every rebaseline would add another
copy that git never reclaims.

A missing baseline is written on first run and reported as
`+ <slug> — baseline written (was missing)`, so a fresh clone self-populates.
The consequence to be aware of: **these gates compare against your machine's
last capture, not against a shared reference.** They answer "did my change move
the render?", which is what they were built for, and not "does this branch match
main". Both are local-only for other reasons anyway — GPUI needs a display,
Jetstream needs the sibling repo.

## Running

```sh
effigy test:native-visual        # diff against committed baselines
effigy native-visual:update      # (re)write baselines after an intended change
```

Or directly:

```sh
bun test/native-visual/run.ts
bun test/native-visual/run.ts --slug=button,pagination
bun test/native-visual/run.ts --update
```

Diffs land in `test/native-visual/out/`; baselines live in
`packages/gpui/preview/baselines/`.

## Local-only

The GPUI preview screenshots its own window — it finds itself by PID through a
CoreGraphics lookup and shells out to `screencapture`. That needs a live macOS
window-server session, so this is not in `ci:web` or `ci:native`. Same
constraint as `check:jetstream`, which needs the sibling runtime repo.

Run it before and after any change to `poodle-specs` accessors or the GPUI
component crate.

## When a baseline changes

A diff is not automatically a failure — an intended rendering change should
update the baseline. The rule is that the update lands **in the same commit as
the change that caused it**, with the reason in the message. A baseline updated
on its own is indistinguishable from a regression waved through.

## Skipped components

Anything inherently non-deterministic — `spinner` and `page-loading`
(animation), `time-ago` (wall-clock copy), `audio-player` and `video-player`
(media surfaces). They are listed in `config.ts` with reasons and named in the
run summary, rather than being quietly tolerated by a raised threshold.
