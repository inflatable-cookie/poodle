/**
 * g15.047 — the Button pair comparator. Pure functions over verified
 * receipt/PNG pairs; no I/O, no browser, no process spawning, so the focused
 * tests can plant failures in memory and drive the exact production paths.
 *
 * Two policies, both from `policy.ts`:
 *
 * - `compareExactPair` (Svelte ↔ React): identical dimensions, zero
 *   logical-edge delta on every landmark, exactly equal role evidence, zero
 *   differing pixels (decoded RGBA byte equality).
 * - `compareRendererAwarePair` (Svelte ↔ GPUI): the fixed renderer-aware
 *   table — bounded geometry, bounded roles, pixelmatch with threshold 0.1,
 *   `includeAA: false`, at most 3% of the viewport.
 *
 * Channels are independent: a pixel pass never hides a geometry or role
 * failure, and the verdict reports each channel separately.
 */

import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";

import type { ButtonFixture, Landmark } from "../fixtures/button-visual-inventory.ts";
import { GEOMETRY, PIXELS, ROLES, type ChannelVerdict, type Finding, type PairVerdict } from "./policy.ts";
import type { ButtonCaptureReceipt, LandmarkBounds, ShadowLayer, Srgb } from "./receipt.ts";

export type CaptureEvidence = {
  receipt: ButtonCaptureReceipt;
  png: Buffer;
};

export type PairResult = {
  verdict: PairVerdict;
  /** Diff visualization for the pixels channel, when one was produced. */
  diffPng?: Buffer;
};

function channel(): ChannelVerdict {
  return { status: "pass", findings: [] };
}

function fail(verdict: ChannelVerdict, finding: Finding): void {
  verdict.status = "fail";
  verdict.findings.push(finding);
}

function assemble(fixture: string, pair: PairVerdict["pair"], channels: PairVerdict["channels"]): PairVerdict {
  const ok = Object.values(channels).every((entry) => entry.status === "pass");
  return { fixture, pair, channels, ok };
}

function edges(bounds: LandmarkBounds): { left: number; top: number; right: number; bottom: number } {
  return {
    left: bounds.x,
    top: bounds.y,
    right: bounds.x + bounds.width,
    bottom: bounds.y + bounds.height,
  };
}

function centre(bounds: LandmarkBounds): { x: number; y: number } {
  return { x: bounds.x + bounds.width / 2, y: bounds.y + bounds.height / 2 };
}

function fmt(value: number): string {
  return Number.isInteger(value) ? String(value) : value.toFixed(4);
}

function compareDimensions(a: ButtonCaptureReceipt, b: ButtonCaptureReceipt): ChannelVerdict {
  const verdict = channel();
  const da = a.deviceDimensions;
  const db = b.deviceDimensions;
  if (da.width !== db.width || da.height !== db.height) {
    fail(verdict, {
      channel: "dimensions",
      subject: "deviceDimensions",
      detail: `${a.runtime} ${da.width}x${da.height} vs ${b.runtime} ${db.width}x${db.height}`,
    });
  }
  return verdict;
}

function landmarkNames(receipt: ButtonCaptureReceipt): Landmark[] {
  return Object.keys(receipt.landmarks) as Landmark[];
}

function compareGeometryExact(a: ButtonCaptureReceipt, b: ButtonCaptureReceipt): ChannelVerdict {
  const verdict = channel();
  for (const name of landmarkNames(a)) {
    const ba = a.landmarks[name];
    const bb = b.landmarks[name];
    if (!ba || !bb) {
      fail(verdict, {
        channel: "geometry",
        subject: name,
        detail: `landmark present on one side only (${a.runtime}: ${Boolean(ba)}, ${b.runtime}: ${Boolean(bb)})`,
      });
      continue;
    }
    const ea = edges(ba);
    const eb = edges(bb);
    for (const edge of ["left", "top", "right", "bottom"] as const) {
      if (ea[edge] !== eb[edge]) {
        fail(verdict, {
          channel: "geometry",
          subject: name,
          detail: `${name}.${edge}: ${a.runtime} ${fmt(ea[edge])} vs ${b.runtime} ${fmt(eb[edge])} — exact policy allows zero logical-edge delta`,
        });
      }
    }
  }
  return verdict;
}

function compareGeometryRendererAware(
  web: ButtonCaptureReceipt,
  gpui: ButtonCaptureReceipt,
): ChannelVerdict {
  const verdict = channel();
  for (const name of landmarkNames(web)) {
    const wa = web.landmarks[name];
    const ga = gpui.landmarks[name];
    if (!wa || !ga) {
      fail(verdict, {
        channel: "geometry",
        subject: name,
        detail: `landmark present on one side only (web: ${Boolean(wa)}, gpui: ${Boolean(ga)})`,
      });
      continue;
    }
    if (name === "root") {
      const we = edges(wa);
      const ge = edges(ga);
      for (const edge of ["left", "top", "right", "bottom"] as const) {
        const delta = Math.abs(we[edge] - ge[edge]);
        if (delta > GEOMETRY.rootEdge) {
          fail(verdict, {
            channel: "geometry",
            subject: "root",
            detail: `root.${edge}: web ${fmt(we[edge])} vs gpui ${fmt(ge[edge])} (delta ${fmt(delta)} > ${GEOMETRY.rootEdge} logical px)`,
          });
        }
      }
      continue;
    }
    // content / icon / spinner: centre and size, each axis bounded.
    const wc = centre(wa);
    const gc = centre(ga);
    const centreLimit = GEOMETRY.contentCentre;
    const sizeLimit = name === "content" ? GEOMETRY.contentExtent : GEOMETRY.contentSize;
    for (const axis of ["x", "y"] as const) {
      const delta = Math.abs(wc[axis] - gc[axis]);
      if (delta > centreLimit) {
        fail(verdict, {
          channel: "geometry",
          subject: name,
          detail: `${name} centre.${axis}: web ${fmt(wc[axis])} vs gpui ${fmt(gc[axis])} (delta ${fmt(delta)} > ${centreLimit} logical px)`,
        });
      }
    }
    for (const axis of ["width", "height"] as const) {
      const delta = Math.abs(wa[axis] - ga[axis]);
      if (delta > sizeLimit) {
        fail(verdict, {
          channel: "geometry",
          subject: name,
          detail: `${name} ${axis}: web ${fmt(wa[axis])} vs gpui ${fmt(ga[axis])} (delta ${fmt(delta)} > ${sizeLimit} logical px)`,
        });
      }
    }
  }
  return verdict;
}

function compareRolesExact(a: ButtonCaptureReceipt, b: ButtonCaptureReceipt): ChannelVerdict {
  const verdict = channel();
  const ja = JSON.stringify(a.roles);
  const jb = JSON.stringify(b.roles);
  if (ja !== jb) {
    // Name the diverging roles instead of dumping both documents.
    for (const role of ["fill", "border", "text", "shadow", "focus-ring"] as const) {
      const ra = JSON.stringify(a.roles[role]);
      const rb = JSON.stringify(b.roles[role]);
      if (ra !== rb) {
        fail(verdict, {
          channel: "roles",
          subject: role,
          detail: `${role}: ${a.runtime} ${ra} vs ${b.runtime} ${rb} — exact policy allows no role delta`,
        });
      }
    }
  }
  return verdict;
}

function checkColor(
  verdict: ChannelVerdict,
  subject: string,
  web: Srgb,
  gpui: Srgb,
): void {
  for (let index = 0; index < 4; index += 1) {
    const delta8 = Math.abs(web[index] - gpui[index]) * 255;
    if (delta8 > ROLES.colorChannel8Bit + 1e-9) {
      const name = ["r", "g", "b", "a"][index];
      fail(verdict, {
        channel: "roles",
        subject,
        detail: `${subject} ${name} channel: web ${fmt(web[index])} vs gpui ${fmt(gpui[index])} (delta ${fmt(delta8)} > ${ROLES.colorChannel8Bit} in 8-bit sRGB)`,
      });
    }
  }
}

function checkWidth(
  verdict: ChannelVerdict,
  subject: string,
  web: number,
  gpui: number,
): void {
  const delta = Math.abs(web - gpui);
  if (delta > ROLES.lineWidth) {
    fail(verdict, {
      channel: "roles",
      subject,
      detail: `${subject} width: web ${fmt(web)} vs gpui ${fmt(gpui)} (delta ${fmt(delta)} > ${ROLES.lineWidth} logical px)`,
    });
  }
}

function compareShadowLayers(
  verdict: ChannelVerdict,
  web: ShadowLayer[],
  gpui: ShadowLayer[],
): void {
  if (web.length !== gpui.length) {
    fail(verdict, {
      channel: "roles",
      subject: "shadow",
      detail: `shadow layer count: web ${web.length} vs gpui ${gpui.length} — exact match required`,
    });
    return;
  }
  for (const [index, wl] of web.entries()) {
    const gl = gpui[index];
    if (wl.inset !== gl.inset) {
      fail(verdict, {
        channel: "roles",
        subject: "shadow",
        detail: `shadow layer ${index} inset: web ${wl.inset} vs gpui ${gl.inset} — exact match required`,
      });
    }
    for (const key of ["offsetX", "offsetY", "blur", "spread"] as const) {
      const delta = Math.abs(wl[key] - gl[key]);
      if (delta > ROLES.shadowGeometry) {
        fail(verdict, {
          channel: "roles",
          subject: "shadow",
          detail: `shadow layer ${index} ${key}: web ${fmt(wl[key])} vs gpui ${fmt(gl[key])} (delta ${fmt(delta)} > ${ROLES.shadowGeometry} logical px)`,
        });
      }
    }
    checkColor(verdict, `shadow layer ${index}`, wl.color, gl.color);
  }
}

function compareRolesRendererAware(
  web: ButtonCaptureReceipt,
  gpui: ButtonCaptureReceipt,
): ChannelVerdict {
  const verdict = channel();
  checkColor(verdict, "fill", web.roles.fill.color, gpui.roles.fill.color);
  checkColor(verdict, "border", web.roles.border.color, gpui.roles.border.color);
  checkWidth(verdict, "border", web.roles.border.width, gpui.roles.border.width);
  checkColor(verdict, "text", web.roles.text.color, gpui.roles.text.color);

  const wRing = web.roles["focus-ring"];
  const gRing = gpui.roles["focus-ring"];
  if (wRing.color === null || gRing.color === null) {
    if (wRing.color !== null || gRing.color !== null) {
      fail(verdict, {
        channel: "roles",
        subject: "focus-ring",
        detail: `focus-ring colour declared on one side only (web: ${wRing.color === null ? "absent" : "declared"}, gpui: ${gRing.color === null ? "absent" : "declared"})`,
      });
    }
  } else {
    checkColor(verdict, "focus-ring", wRing.color, gRing.color);
  }
  if (wRing.width === null || gRing.width === null) {
    if (wRing.width !== null || gRing.width !== null) {
      fail(verdict, {
        channel: "roles",
        subject: "focus-ring",
        detail: `focus-ring width declared on one side only (web: ${wRing.width === null ? "absent" : fmt(wRing.width)}, gpui: ${gRing.width === null ? "absent" : fmt(gRing.width)})`,
      });
    }
  } else {
    checkWidth(verdict, "focus-ring", wRing.width, gRing.width);
  }

  compareShadowLayers(verdict, web.roles.shadow.layers, gpui.roles.shadow.layers);
  return verdict;
}

function decode(png: Buffer, label: string): PNG {
  try {
    return PNG.sync.read(png);
  } catch (error) {
    throw new Error(`${label} PNG does not decode: ${(error as Error).message}`);
  }
}

function comparePixelsExact(a: CaptureEvidence, b: CaptureEvidence): { verdict: ChannelVerdict; diffPng?: Buffer } {
  const verdict = channel();
  const pa = decode(a.png, a.receipt.runtime);
  const pb = decode(b.png, b.receipt.runtime);
  if (pa.width !== pb.width || pa.height !== pb.height) {
    fail(verdict, {
      channel: "pixels",
      subject: "pixels",
      detail: `cannot compare pixels across different dimensions (${pa.width}x${pa.height} vs ${pb.width}x${pb.height})`,
    });
    return { verdict };
  }
  if (pa.data.equals(pb.data)) {
    verdict.metrics = { differingPixels: 0, totalPixels: pa.width * pa.height, diffRatio: 0 };
    return { verdict };
  }

  let differing = 0;
  for (let offset = 0; offset < pa.data.length; offset += 4) {
    if (
      pa.data[offset] !== pb.data[offset] ||
      pa.data[offset + 1] !== pb.data[offset + 1] ||
      pa.data[offset + 2] !== pb.data[offset + 2] ||
      pa.data[offset + 3] !== pb.data[offset + 3]
    ) {
      differing += 1;
    }
  }
  // The comparison itself is strict byte equality; the pixelmatch rendering is
  // only the visual aid the operator reviews beside the failure.
  const diff = new PNG({ width: pa.width, height: pa.height });
  pixelmatch(pa.data, pb.data, diff.data, pa.width, pa.height, {
    threshold: PIXELS.threshold,
    includeAA: PIXELS.includeAA,
  });
  verdict.metrics = {
    differingPixels: differing,
    totalPixels: pa.width * pa.height,
    diffRatio: differing / (pa.width * pa.height),
  };
  fail(verdict, {
    channel: "pixels",
    subject: "pixels",
    detail: `${differing} pixels differ — the exact policy allows zero`,
  });
  return { verdict, diffPng: PNG.sync.write(diff) };
}

function comparePixelsRendererAware(web: CaptureEvidence, gpui: CaptureEvidence): { verdict: ChannelVerdict; diffPng?: Buffer } {
  const verdict = channel();
  const pw = decode(web.png, "web");
  const pg = decode(gpui.png, "gpui");
  if (pw.width !== pg.width || pw.height !== pg.height) {
    fail(verdict, {
      channel: "pixels",
      subject: "pixels",
      detail: `cannot compare pixels across different dimensions (${pw.width}x${pw.height} vs ${pg.width}x${pg.height})`,
    });
    return { verdict };
  }
  const diff = new PNG({ width: pw.width, height: pw.height });
  const differing = pixelmatch(pw.data, pg.data, diff.data, pw.width, pw.height, {
    threshold: PIXELS.threshold,
    includeAA: PIXELS.includeAA,
  });
  const total = pw.width * pw.height;
  const ratio = differing / total;
  verdict.metrics = { differingPixels: differing, totalPixels: total, diffRatio: ratio };
  if (ratio > PIXELS.maxDiffRatio) {
    fail(verdict, {
      channel: "pixels",
      subject: "pixels",
      detail: `${differing}/${total} pixels differ (${(ratio * 100).toFixed(3)}% > ${PIXELS.maxDiffRatio * 100}% cap, pixelmatch threshold ${PIXELS.threshold}, includeAA ${PIXELS.includeAA})`,
    });
  }
  return { verdict, diffPng: PNG.sync.write(diff) };
}

/** Svelte ↔ React: the exact same-browser policy. `a`/`b` order is cosmetic. */
export function compareExactPair(
  fixture: ButtonFixture,
  a: CaptureEvidence,
  b: CaptureEvidence,
): PairResult {
  const dimensions = compareDimensions(a.receipt, b.receipt);
  const geometry = compareGeometryExact(a.receipt, b.receipt);
  const roles = compareRolesExact(a.receipt, b.receipt);
  const pixels = comparePixelsExact(a, b);
  const verdict = assemble(fixture.name, "svelte-react", {
    dimensions,
    geometry,
    roles,
    pixels: pixels.verdict,
  });
  return { verdict, diffPng: pixels.diffPng };
}

/** Svelte (web reference) ↔ GPUI: the fixed renderer-aware table. */
export function compareRendererAwarePair(
  fixture: ButtonFixture,
  web: CaptureEvidence,
  gpui: CaptureEvidence,
): PairResult {
  const dimensions = compareDimensions(web.receipt, gpui.receipt);
  const geometry = compareGeometryRendererAware(web.receipt, gpui.receipt);
  const roles = compareRolesRendererAware(web.receipt, gpui.receipt);
  const pixels = comparePixelsRendererAware(web, gpui);
  const verdict = assemble(fixture.name, "svelte-gpui", {
    dimensions,
    geometry,
    roles,
    pixels: pixels.verdict,
  });
  return { verdict, diffPng: pixels.diffPng };
}
