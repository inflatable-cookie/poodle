# 037 Focus Rings — Square Outlines On Rounded Controls

Status: ready
Milestone: side-quest (component styling, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-037-focus-ring-radius-audit`
Depends on: none
Governing refs: `docs/contracts/components/*.md` (§Token Usage / §Accessibility),
`PAPERCUTS.md` (2026-08-11 focus-ring entry, resolved 2026-08-12)

## Goal

A CSS `outline` follows its own element's `border-radius`. An element that
draws a focus outline and declares no radius renders a hard square ring, even
when everything around it is rounded.

HistoryCentre had three: the entry row, the picker row and the not-yet-loaded
row, all full-width, all square, fixed in `7c88908e`. A static scan says
**34 more elements across the corpus have the same shape**.

This card fixes those and gates the pattern. It is not the whole focus story —
see "What This Card Is Not".

## Fixed By Ruling (do not re-decide)

### R1 — The scan is a starting list, not a work order.

The 34 were found by a crude regex: "this file has a `:focus-visible` rule
declaring an outline, and the same base selector never declares
`border-radius` anywhere in this file". That has false positives. A radius can
arrive from a shared rule, a parent, a different selector spelling, or a
variant block the scan did not associate.

**Check each one in a browser before changing it.** A confirmed square ring on
a rounded control is a fix. A square ring on a genuinely square element is
correct and must be left alone — say which ones those were.

Reproduce the list with:

```sh
bun - <<'JS'
import { readdirSync, readFileSync } from "node:fs";
const dir = "packages/core/src/styles";
for (const f of readdirSync(dir).filter((n) => n.endsWith(".css"))) {
  const t = readFileSync(`${dir}/${f}`, "utf8");
  for (const m of t.matchAll(/([^{}]+)\{([^{}]*)\}/g)) {
    const sel = m[1].trim(), body = m[2];
    if (!sel.includes(":focus-visible") || !body.includes("outline:")) continue;
    if (body.split("outline:")[1].slice(0, 20).includes("none")) continue;
    const base = sel.split(":focus-visible")[0].split(",").pop().trim();
    if (!base.startsWith(".")) continue;
    if (!new RegExp(base.replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + "\\s*\\{[^}]*border-radius").test(t))
      console.log(`${f}: ${base}`);
  }
}
JS
```

### R2 — Match the component's own radius. Do not invent one.

Use the radius the component already uses for its own surfaces, normally
`--poodle-radius-control`. If a component genuinely has no radius vocabulary,
`--poodle-radius-control` is the default — but say so in the log rather than
silently picking.

Where the outline is inset (`outline-offset` negative), the radius belongs on
the element the outline is drawn on, not on an ancestor.

### R3 — Round the hover and selected fills with it.

HistoryCentre's entry row had a square hover block for exactly the same
reason. Where adding a radius for the ring leaves a differently-shaped fill on
the same element, they must agree. A rounded ring around a square fill is
worse than what was there before.

### R4 — Gate it.

Add a check that fails when an element declares a focus outline and no radius,
with a baseline holding any element that is square **by intent**. Same shape
as the other drift gates in `packages/svelte/preview/scripts/` — a standalone
selector plus a call from `lint-docs.ts`. Wire it as
`effigy docs:focus-ring-drift`.

An entry in the baseline needs a one-line reason. "Not looked at yet" is not a
reason; if it is unverified, verify it.

### R5 — Verify in a browser, and know the trap.

`element.focus()` does **not** trigger `:focus-visible` — it needs real
keyboard interaction. A Tab sweep will also miss anything behind a roving
tabindex, which is how HistoryCentre's rows evaded one. Force the pseudo-state
through CDP instead:

```js
const cdp = await page.context().newCDPSession(page);
await cdp.send("DOM.enable"); await cdp.send("CSS.enable");
const { root } = await cdp.send("DOM.getDocument");
const { nodeId } = await cdp.send("DOM.querySelector", { nodeId: root.nodeId, selector: SEL });
await cdp.send("CSS.forcePseudoState", { nodeId, forcedPseudoClasses: ["focus-visible", "focus"] });
```

Measure `borderRadius` and `outlineStyle` after forcing, and capture the ones
you changed.

## What This Card Is Not

Two larger findings stay out, and a follow-up card takes them:

- **79 of 160 component stylesheets draw no focus treatment at all** and
  inherit whatever the browser or host applies. Deciding which of those are
  focusable is a per-component judgement, not a mechanical sweep.
- **56 stylesheets draw a ring without suppressing the UA outline**, so a
  host's generic `button:focus-visible` stacks a second ring on top. Poodle's
  own shell hit this (`PAPERCUTS.md`, 2026-08-11) and needed a workaround.

Do not start either here. Note anything you learn about them in the log.

## Scope

### In scope

- `packages/core/src/styles/*.css` — radius fixes on confirmed square rings.
- The new gate and its wiring.
- Contract §Token Usage rows where a component's radius surface changes.

### Out of scope — stop conditions if reached

- Adding a focus treatment to a component that has none (that is the follow-up).
- Changing ring colour, width or offset. Radius only.
- Any component logic, markup, or Svelte/React file.
- `HistoryCenter` — already fixed in `7c88908e`.
- Refreshing visual baselines. A rounded ring **will** move pixels: if a
  baseline covers a component you changed, **stop and say so**. Classify the
  delta first; a baseline can be wrong rather than merely outdated.

## Required Tests

- The new gate fails on a planted square ring and passes clean (prove both).
- The gate's baseline is empty, or every entry carries a reason.
- For each component changed: a browser measurement showing radius non-zero
  and the outline present under forced `:focus-visible`.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **R1 is the rule that matters: verify before changing.** A blanket sweep
  over all 34 without looking is a failed card even if every gate passes.
- Run `effigy check:svelte`, `docs:lint`, `docs:callback-drift`.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-037-focus-ring-radius-audit`. Do not merge.

## Writable Paths

- `packages/core/src/styles/*.css`
- `packages/svelte/preview/scripts/**`
- `packages/svelte/preview/src/specimens/**` (only to expose a focus state
  that has no specimen)
- `tasks/effigy.tasks.toml`
- `docs/contracts/components/*.md`
- `docs/logs/2026-08/<DD>-g13-037-focus-ring-radius-audit.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ci:web`, `docs:lint`, `git diff --check`. All green.
2. Reproduce the scan (R1). Record the list in your log.
3. Verify each candidate in a browser with forced `:focus-visible` (R5).
   Split the list into *confirmed square on a rounded control*, *correctly
   square*, and *scan false positive*. Record all three.
4. Fix the confirmed ones (R2, R3).
5. Add the gate, baseline the intentional squares with reasons (R4).
6. Prove the gate: plant a square ring, watch it fail, restore.
7. Validate:
   ```sh
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:focus-ring-drift
   effigy docs:callback-drift
   effigy drift:recipes
   effigy svelte:surface-audit
   effigy ci:web
   git diff --check
   ```

## Acceptance Criteria

- [ ] Every candidate classified, with the three buckets named in the log.
- [ ] Confirmed square rings round on the component's own radius; fills agree.
- [ ] The gate fails on a planted regression and passes clean.
- [ ] Every baseline entry has a reason.
- [ ] All step-7 commands exit 0; no baseline refreshed.

## Stop Conditions

- A visual baseline moves.
- A component's radius is genuinely ambiguous — two surfaces disagree.
- The gate cannot separate "no radius declared" from "radius inherited" well
  enough to avoid a baseline larger than the fix list.

Stop with exact paths, commands, and the smallest unresolved question.
