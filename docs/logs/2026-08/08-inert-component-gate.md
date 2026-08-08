---
title: drift:events — a contract that declares events must have somewhere to send them
status: complete
owner: Poodle core
updated: 2026-08-08
tags: [log, gates, poodle-render, contracts]
---

## The Hole

`drift:clicks` was retired when its subject (`poodle-jetstream-components`) was
deleted, and the note recorded that `drift:handlers` covers what remained. That
was half true. `drift:handlers` catches a component that *accepts* a handler and
never uses it. Nothing caught the earlier failure: a component that accepts **no
handler at all**, while its contract documents `onConfirm`, `onDismiss`,
`onChange`.

That failure looks finished from the outside. An AlertDialog renders Confirm and
Cancel; a toast renders a dismiss; a field renders and takes text. None of it
can be wired, and no test notices, because nothing is broken — it was never
connected.

## The Rule

Deliberately coarse: **at least one** handler, not a name-by-name match.

The first attempt did match names, and reported 151 gaps across 72 components —
almost all of them naming differences (`onCheckedChange` is `on_change`; one
Rust callback often serves several documented events). A gate with a 151-entry
noise floor teaches people to ignore it. "Declares events, accepts nothing" has
no such ambiguity, and found 10.

Two refinements the real code forced:

- A component may take a `*Handlers` **bundle and forward it whole** —
  `ToastHost` hands its callbacks straight to `ToastStack` and so never spells
  `on_` at all. Three of the ten were this.
- Contracts with no poodle-render component yet are skipped: that is the
  migration's business, not this gate's.

## What It Found

Seven genuinely inert components. Four are legitimate and recorded in
`ACCEPTED` with the reason:

- `tooltip`, `hover-card`, `popover` — open state is a controlled prop on the
  Rust targets, so there is no transition for the component to report.
  (`popover`'s `onSurfaceGeometryChange` reports measured placement, which is
  the backend's, and the vocabulary has no channel for it.)
- `scroll-shell` — scroll position is backend-owned by the vocabulary's own
  division, and nothing reports it upward.

Three were real:

- **`embed-input` is fixed.** It composes the real `text_input`, so
  `onValueChange` was ten lines: an `EmbedInputHandlers` bundle and
  `text_input_with_change`. Its own module doc had claimed the event was a
  "host concern", which was wrong — the component owns the nested field, so it
  is the only layer that can see an edit. `onParse` genuinely is the host's: the
  host does the parsing and the spec arrives resolved.
- **`code-input` and `duration-input`** render slots and segments that take no
  keys. Making their events real means building the input behaviour first.

## Ratchet, Not Baseline

Those last two sit in `KNOWN_GAPS`, which is **debt, not acceptance** — the two
lists are separate on purpose, because calling unfinished work "accepted" is how
a gate stops meaning anything.

The gate reports them loudly, does not fail on them, and **fails if the list
grows**. It also fails if a listed component is fixed and not struck off, so the
list can only shrink.

Adding a token handler to silence it does not work either: `drift:handlers`
then fails, because a declared handler must be *read*. The two gates close each
other's loophole.

Both failure paths were tested rather than assumed — a fixed component left in
the list fails, and an unlisted inert component fails.

## Wiring

`drift:events`, in `ci:native` beside `drift:handlers`. The `drift:clicks`
retirement note is amended to say what was actually lost and what recovered it.

Green: `poodle-render` 119, `effigy drift:events`, `drift:handlers`,
`check:gpui`, `docs:lint`, `git diff --check`.
