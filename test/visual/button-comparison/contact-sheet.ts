/**
 * g15.047 — contact sheet generation. The contact sheet is the operator's
 * review surface: every capture at native device scale, in canonical fixture
 * order, with its sibling runtimes, both diffs, and the per-channel verdicts
 * beside it. Raw captures and receipts stay on disk next to the sheet.
 *
 * The sheet is HTML rather than a composed PNG so every image stays at
 * readable native scale (480×160) with text labels — a flat grid PNG would
 * either lose the labels or shrink the captures.
 */

import type { ButtonFixture } from "../fixtures/button-visual-inventory.ts";
import { KNOWN_RENDERER_DELTAS, type Channel, type Finding, type PairVerdict } from "./policy.ts";
import { fixtureFileStem } from "./capture-gpui.ts";

export type FixtureReport = {
  fixture: ButtonFixture;
  pairs: PairVerdict[];
  /** Finding → known-delta id, for findings the current contract already decides. */
  knownDeltas: { pair: string; finding: Finding; deltaId: string }[];
};

function escapeHtml(text: string): string {
  return text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function channelStatus(pair: PairVerdict, channel: Channel, known: Set<Finding>): string {
  const verdict = pair.channels[channel];
  if (verdict.status === "pass") return "pass";
  return verdict.findings.every((finding) => known.has(finding)) ? "known-delta" : "FAIL";
}

function pairBlock(report: FixtureReport, pair: PairVerdict, stem: string): string {
  const known = new Set(
    report.knownDeltas.filter((entry) => entry.pair === pair.pair).map((entry) => entry.finding),
  );
  const rows = (["dimensions", "geometry", "roles", "pixels"] as const)
    .map((channelName) => {
      const status = channelStatus(pair, channelName, known);
      const findings = pair.channels[channelName].findings
        .map((finding) => {
          const delta = report.knownDeltas.find((entry) => entry.finding === finding);
          const citation = delta
            ? ` <em>(${escapeHtml(KNOWN_RENDERER_DELTAS.find((entry) => entry.id === delta.deltaId)?.citation ?? delta.deltaId)})</em>`
            : "";
          return `<li class="${delta ? "known" : "fail"}">${escapeHtml(finding.detail)}${citation}</li>`;
        })
        .join("");
      return `<tr><td>${channelName}</td><td class="${status === "FAIL" ? "fail" : status}">${status}</td></tr>${findings ? `<tr><td></td><td><ul>${findings}</ul></td></tr>` : ""}`;
    })
    .join("\n");
  return `
    <section class="pair">
      <h4>${pair.pair === "svelte-react" ? "Svelte ↔ React (exact)" : "Svelte ↔ GPUI (renderer-aware)"}</h4>
      <img src="diffs/${stem}--${pair.pair}.png" alt="${pair.pair} diff for ${report.fixture.name}" />
      <table>${rows}</table>
    </section>`;
}

/**
 * Render the full review sheet. `capturesDir`/`diffsDir` are paths relative to
 * the sheet's location used in `src` attributes.
 */
export function renderContactSheet(reports: FixtureReport[], environment: string[]): string {
  const sections = reports
    .map((report) => {
      const stem = fixtureFileStem(report.fixture.name);
      const captures = (["svelte", "react", "gpui"] as const)
        .map(
          (runtime) =>
            `<figure><img src="captures/${runtime}/${stem}.png" alt="${report.fixture.name} [${runtime}]" /><figcaption>${runtime}</figcaption></figure>`,
        )
        .join("\n");
      const pairs = report.pairs.map((pair) => pairBlock(report, pair, stem)).join("\n");
      return `
  <section class="fixture" id="${stem}">
    <h3><code>${escapeHtml(report.fixture.name)}</code> <span class="group">${escapeHtml(report.fixture.group)}</span></h3>
    <div class="captures">${captures}</div>
    <div class="pairs">${pairs}</div>
  </section>`;
    })
    .join("\n");

  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8" />
<title>g15.047 Button visual comparison — contact sheet</title>
<style>
  body { font: 13px/1.45 system-ui, sans-serif; margin: 24px; background: #111; color: #ddd; }
  h1 { font-size: 18px; }
  .fixture { border-top: 1px solid #333; padding-top: 12px; margin-top: 20px; }
  .group { color: #888; font-size: 12px; }
  .captures { display: flex; gap: 12px; }
  figure { margin: 0; }
  figcaption { text-align: center; color: #999; }
  img { image-rendering: pixelated; display: block; background: #000; }
  .pairs { display: flex; gap: 24px; margin-top: 12px; align-items: flex-start; }
  table { border-collapse: collapse; margin-top: 8px; }
  td { padding: 2px 10px 2px 0; vertical-align: top; }
  ul { margin: 0; padding-left: 18px; max-width: 460px; }
  .pass { color: #7d7; }
  .known-delta, .known { color: #dc4; }
  .fail, .FAIL { color: #e66; }
  td.fail { color: #e66; font-weight: 600; }
  .env { color: #888; font-size: 12px; }
</style>
</head>
<body>
<h1>g15.047 Button visual comparison — 18 fixtures × 3 runtimes</h1>
<p class="env">${environment.map(escapeHtml).join(" · ")}</p>
<p>All captures are same-run evidence, shown at native device scale (480×160 = 240×80 logical @ 2×),
in canonical inventory order. Diff images highlight pixels outside the fixed policy; they are
diagnostics, never baselines.</p>
${sections}
</body>
</html>
`;
}
