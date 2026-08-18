# g15.018 — Overloaded Examples Curation (parent)

Status: **planned parent — not dispatchable**
Consumes: `g15.011` partial screening baseline and its three approved pilots
Children: `g15.020`–`g15.025`
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`
(Section order, Section budget)

## Why This Is A Parent

This card defines the method for curating overloaded `Examples` views. It is
**not dispatchable**: its own stop condition forbids one PR redoing the
catalogue, and `g14.026` requires bounded family cards with explicit
operator-review checkpoints. The work is dispatched through the children.

`g15.011` proved the shape on Button, RangeSlider, and Tabs. The audit graded
**53 remaining pages** overloaded (10+ captioned examples) or long (7–9) where
the length is a prop cross-product rather than distinct things worth seeing.
The six children partition those 53 exactly: every page appears on one child
and no page appears on two.

## Children

Each child owns one family group, carries its own operator-review checkpoint,
and is dispatched separately.

| Card | Family group | Pages | Sequencing |
| --- | --- | ---: | --- |
| `g15.020` | Model connections + account lifecycle | 8 | none |
| `g15.021` | Application shell | 7 | none |
| `g15.022` | Audio & music | 11 | after `g15.017` splits `audio_specimens` |
| `g15.023` | Foundations: entry, content, status | 11 | after `g15.016` |
| `g15.024` | Agent & tools | 6 | after `g15.015` restores captions |
| `g15.025` | Collections, navigation, overlays, long tail | 10 | after `g15.016` |
| **Total** | | **53** | |

**Each child carries its own exact page list.** The counts above and the lists
in the children are one partition of the same 53 pages, taken from the audit at
this branch's head. A child re-measures its pages before starting — a
prerequisite card can change a page's example count — and records any change
rather than silently resizing its set.

## Method (every child follows this)

- Each page reaches the outline's 3–6 section budget, or states why it cannot.
- The first example answers "what is this and how do I normally use it?"
- Variants are distinct forms, not a prop cross-product. One tone row on one
  variant replaces a variant × tone grid.
- Removed examples are checked against the contract first: anything the
  contract covers and the page was the only evidence for either stays or is
  recorded as a coverage note, never silently dropped.
- Svelte and React stay identical; GPUI teaches the same set.

## Acceptance (every child)

- [ ] No page in the family exceeds 9 captioned examples.
- [ ] Every page's first example is a realistic default use.
- [ ] A named list of what was removed from each page, and why.
- [ ] Contract coverage unchanged, or its gaps recorded.
- [ ] **Operator review of the family's changed pages in the live Svelte and
      React previews before the child is called complete.** Unreviewed pages
      remain an explicit PR item.

## Stop Conditions

- Curation removes the only evidence for a contract behaviour without
  recording it.
- The card becomes a second attempt at an exhaustive reference view. An
  exhaustive view may be considered later; it never replaces `Examples`.
- A child grows past its family group.

## Writable Scope (children)

- the specimen files for that family, across Svelte, React, and GPUI
- one batch log per child

## Validation (children)

- focused preview tests, `effigy check:svelte`, `effigy react:build`,
  `effigy check:gpui`, `effigy docs:check`, `git diff --check`
