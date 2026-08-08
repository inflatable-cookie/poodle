/**
 * Inert-component drift: a contract that declares events must be wirable.
 *
 * The counterpart to `dead-handler-drift`. That one catches a component which
 * *accepts* a handler and never uses it. This catches the earlier failure: a
 * component that accepts no handler at all, while its contract documents
 * `onConfirm`, `onDismiss`, `onChange`. The rendered control looks complete —
 * an AlertDialog with Confirm and Cancel, a toast with a dismiss — and no host
 * can make any of it do anything.
 *
 * This rule was previously covered from the other end by `drift:clicks`, which
 * drove a real pointer gesture into every Jetstream builder. Its subject was
 * deleted in `ee704699`, and nothing replaced the coverage.
 *
 * The rule is deliberately coarse: **at least one** handler, not a name-by-name
 * match. Contract event names and Rust handler names legitimately differ
 * (`onCheckedChange` is `on_change`; one Rust callback often serves several
 * documented events), so a strict mapping produces noise instead of signal.
 * "Declares events, accepts nothing" has no such ambiguity.
 */
import { readFileSync, readdirSync } from "node:fs";
import { basename, join } from "node:path";

const CONTRACTS = new URL("../../../../docs/contracts/components", import.meta.url).pathname;
const RENDER_SRC = new URL("../../../render/src", import.meta.url).pathname;

/**
 * Components that legitimately accept no handler, with the reason.
 *
 * These exist to be **closed**, not to grow. An entry is only defensible when
 * the host owns the state entirely and the component has nothing to report —
 * not when wiring it is merely unfinished.
 */
const ACCEPTED: Record<string, string> = {
  // Open state is a controlled prop on the Rust targets: the host decides
  // whether an overlay is showing and passes it in, so there is no transition
  // for the component to report back.
  tooltip: "open state is host-owned (`isOpen`); no native hover-intent timer to report from",
  "hover-card": "open state is host-owned (`isOpen`)",
  popover:
    "open state is host-owned (`isOpen`); `onSurfaceGeometryChange` reports measured placement, which is the backend's and has no vocabulary channel",
  // Scroll physics belong to the backend by the vocabulary's own division, and
  // nothing reports position back up yet.
  "scroll-shell": "scroll position is backend-owned; the vocabulary has no scroll-report channel",
};

/**
 * Debt, not acceptance: the events are real and the component should carry
 * them, but wiring them means building the input behaviour first.
 *
 * The gate reports these loudly and does not fail on them — and it fails
 * immediately if the list **grows**, which is what stops a codebase adopting a
 * gate from quietly acquiring more of exactly what the gate exists to catch.
 */
const KNOWN_GAPS: Record<string, string> = {
  // Empty, and worth keeping empty: both original entries (code-input's slots
  // and duration-input's segments) were closed rather than accepted.
};

function contractEvents(source: string): string[] {
  // Props tables list one prop per row: | `onFoo` | type | default | ...
  const rows = source.matchAll(/^\|\s*`(on[A-Z]\w*)`/gm);
  return [...new Set([...rows].map((m) => m[1]))].sort();
}

function main(): void {
  const failures: string[] = [];
  const accepted: string[] = [];
  const gaps: string[] = [];
  let checked = 0;
  let skipped = 0;

  for (const file of readdirSync(CONTRACTS).sort()) {
    if (!file.endsWith(".md")) continue;
    const slug = basename(file, ".md");
    const events = contractEvents(readFileSync(join(CONTRACTS, file), "utf8"));
    if (events.length === 0) continue;

    const renderPath = join(RENDER_SRC, `${slug.replace(/-/g, "_")}.rs`);
    let render: string;
    try {
      render = readFileSync(renderPath, "utf8");
    } catch {
      // No poodle-render component yet: that is the migration's business, not
      // this gate's.
      skipped += 1;
      continue;
    }

    checked += 1;
    // Either names handlers itself, or takes a `*Handlers` bundle and forwards
    // it whole — ToastHost hands its callbacks straight to ToastStack and so
    // never spells `on_` at all.
    if (/\bon_\w+/.test(render) || /\w*Handlers\b/.test(render)) continue;

    const detail = `${slug} — contract declares ${events.join(", ")} but the component accepts no handler`;
    if (slug in ACCEPTED) {
      accepted.push(`${detail} (accepted: ${ACCEPTED[slug]})`);
    } else if (slug in KNOWN_GAPS) {
      gaps.push(`${detail}\n      needs: ${KNOWN_GAPS[slug]}`);
    } else {
      failures.push(`  ${detail}`);
    }
  }

  console.log(
    `inert-component drift: checked ${checked}, skipped ${skipped} (no poodle-render component), ${accepted.length} accepted, ${gaps.length} known gaps`,
  );
  for (const note of accepted) console.log(`  - ${note}`);
  for (const note of gaps) console.log(`  ! ${note}`);

  // The ratchet: a fixed gap must be struck off, so the list can only shrink.
  const stale = Object.keys(KNOWN_GAPS).filter(
    (slug) => !gaps.some((g) => g.startsWith(slug)),
  );
  if (stale.length > 0) {
    console.error(
      `\nFAIL — ${stale.join(", ")} no longer inert; remove from KNOWN_GAPS so it cannot regress.`,
    );
    process.exit(1);
  }

  if (failures.length > 0) {
    console.error(
      `\nFAIL — ${failures.length} component(s) render events no host can receive:\n${failures.join("\n")}\n`,
    );
    console.error(
      "Add the handler to the component's *Handlers struct and attach it, or\nrecord it in ACCEPTED with the reason the host owns it entirely.",
    );
    process.exit(1);
  }
  console.log("\nOK — every contract that declares events has somewhere to send them.");
}

main();
