/**
 * g15.047 — focused comparator evidence. Every planted failure is built in
 * memory and driven through the production compare/verify functions; nothing
 * mutates the canonical inventory, a committed capture, or an expected asset.
 *
 * The card's planted set: missing capture, two-logical-pixel root shift,
 * missing icon/spinner landmark, changed role colour/shadow, PNG tamper, and
 * a pixel change beyond 3% — each must fail loudly through the production
 * path, and channels must stay independent (a pixel pass never lifts a
 * geometry or role failure).
 */

import { describe, expect, test } from "bun:test";

import { PNG } from "pngjs";

import {
  loadButtonVisualInventory,
  type ButtonFixture,
} from "../fixtures/button-visual-inventory.ts";
import {
  CaptureIntegrityError,
  captureSetProblems,
  isRecoverableTransportError,
  PreviewTransportError,
  repeatMismatchProblem,
} from "./capture-set.ts";
import {
  compareExactPair,
  compareRendererAwarePair,
  type CaptureEvidence,
} from "./compare.ts";
import { classifyKnownDelta } from "./policy.ts";
import {
  MIN_FOREGROUND_SAMPLES,
  parseButtonCaptureReceipt,
  ReceiptError,
  sha256Hex,
  verifyReceiptAgainstPng,
  type ButtonCaptureReceipt,
  type RuntimeName,
  type Srgb,
} from "./receipt.ts";

const inventory = loadButtonVisualInventory();
const fixtures = new Map(inventory.fixtures.map((fixture) => [fixture.name, fixture]));

function fixture(name: string): ButtonFixture {
  const found = fixtures.get(name);
  if (!found) throw new Error(`test setup: no fixture ${name}`);
  return found;
}

const WIDTH = 480; // 240x80 logical at 2x
const HEIGHT = 160;

function solidPng(r: number, g: number, b: number, a = 255): Buffer {
  const png = new PNG({ width: WIDTH, height: HEIGHT });
  for (let offset = 0; offset < png.data.length; offset += 4) {
    png.data[offset] = r;
    png.data[offset + 1] = g;
    png.data[offset + 2] = b;
    png.data[offset + 3] = a;
  }
  return PNG.sync.write(png);
}

function withWhiteRegion(png: Buffer, pixelCount: number): Buffer {
  const decoded = PNG.sync.read(png);
  for (let index = 0; index < pixelCount; index += 1) {
    const offset = index * 4;
    decoded.data[offset] = 255;
    decoded.data[offset + 1] = 255;
    decoded.data[offset + 2] = 255;
    decoded.data[offset + 3] = 255;
  }
  return PNG.sync.write(decoded);
}

const BLACK: Srgb = [0, 0, 0, 1];
const WHITE: Srgb = [1, 1, 1, 1];

function environmentFor(runtime: RuntimeName): ButtonCaptureReceipt["environment"] {
  return runtime === "gpui"
    ? {
        kind: "macos-window-server-nonactivating",
        os: "macos",
        arch: "arm64",
        gpuiSource: "crates.io",
        gpuiVersion: "0.2.2",
        foreground: {
          baseline: "com.example.editor",
          observed: ["com.example.editor"],
          samples: 24,
          verdict: "proved",
        },
      }
    : { kind: "chromium", version: "test-chromium" };
}

/** A valid synthetic receipt for the fixture, re-parsed through the verifier. */
function makeReceipt(
  target: ButtonFixture,
  runtime: RuntimeName,
  mutate?: (receipt: Record<string, unknown>) => void,
): ButtonCaptureReceipt {
  const landmarks: Record<string, unknown> = {
    root: { x: 16, y: 16, width: 80, height: 36 },
    content: { x: 42.5, y: 27, width: 21, height: 14 },
  };
  if (target.landmarks.includes("icon")) {
    landmarks.icon = { x: 28, y: 26, width: 16, height: 16 };
  }
  if (target.landmarks.includes("spinner")) {
    landmarks.spinner = { x: 28, y: 28, width: 12, height: 12 };
  }
  const raw: Record<string, unknown> = {
    schema: "poodle.button-visual-capture.v2",
    fixture: target.name,
    runtime,
    logicalViewport: { width: 240, height: 80 },
    scale: 2,
    deviceDimensions: { width: WIDTH, height: HEIGHT },
    pngSha256: "0".repeat(64),
    environment: environmentFor(runtime),
    landmarks,
    roles: {
      fill: { color: BLACK },
      border: { color: BLACK, width: 1 },
      text: { color: WHITE },
      shadow: { layers: [] },
      "focus-ring": { color: [0.2, 0.4, 0.9, 1], width: 2, status: "dormant" },
    },
  };
  mutate?.(raw);
  return parseButtonCaptureReceipt(raw, { fixture: target, runtime });
}

function evidence(target: ButtonFixture, runtime: RuntimeName, png: Buffer, sha: string): CaptureEvidence {
  return {
    receipt: makeReceipt(target, runtime, (raw) => {
      raw.pngSha256 = sha;
    }),
    png,
  };
}

function hashOf(png: Buffer): string {
  return sha256Hex(png);
}

describe("receipt verification", () => {
  test("a valid receipt round-trips", () => {
    const receipt = makeReceipt(fixture("button/rest-secondary"), "svelte");
    expect(receipt.fixture).toBe("button/rest-secondary");
  });

  test("unknown fields are rejected", () => {
    expect(() =>
      makeReceipt(fixture("button/rest-secondary"), "react", (raw) => {
        raw.props = { variant: "secondary" };
      }),
    ).toThrow(ReceiptError);
  });

  test("a missing landmark is rejected by exact fixture name", () => {
    expect(() =>
      makeReceipt(fixture("button/state-loading"), "gpui", (raw) => {
        delete (raw.landmarks as Record<string, unknown>).spinner;
      }),
    ).toThrow(/landmarks is missing 'spinner'/);
  });

  test("an extra landmark is rejected by exact fixture name", () => {
    expect(() =>
      makeReceipt(fixture("button/rest-secondary"), "svelte", (raw) => {
        (raw.landmarks as Record<string, unknown>).icon = { x: 1, y: 1, width: 4, height: 4 };
      }),
    ).toThrow(/landmarks has unknown key 'icon'/);
  });

  test("a receipt naming another fixture or runtime is rejected", () => {
    const ghost = makeReceipt(fixture("button/variant-ghost"), "svelte");
    expect(() =>
      parseButtonCaptureReceipt(ghost, { fixture: fixture("button/rest-secondary"), runtime: "svelte" }),
    ).toThrow(/fixture must be 'button\/rest-secondary'/);
    expect(() =>
      parseButtonCaptureReceipt(ghost, { fixture: fixture("button/variant-ghost"), runtime: "react" }),
    ).toThrow(/runtime must be 'react'/);
  });

  test("a tampered PNG fails hash verification", () => {
    const target = fixture("button/rest-secondary");
    const png = solidPng(10, 20, 30);
    const sha = hashOf(png);
    const good = evidence(target, "svelte", png, sha);
    expect(verifyReceiptAgainstPng(good.receipt, png)).toEqual([]);

    const tampered = Buffer.concat([png, Buffer.from([0])]);
    const problems = verifyReceiptAgainstPng(good.receipt, tampered);
    expect(problems.some((problem) => problem.includes("does not match receipt"))).toBe(true);
  });

  test("a PNG with wrong dimensions is named", () => {
    const target = fixture("button/rest-secondary");
    const small = PNG.sync.write(new PNG({ width: 4, height: 4 }));
    const receipt = makeReceipt(target, "react", (raw) => {
      raw.pngSha256 = hashOf(small);
    });
    const problems = verifyReceiptAgainstPng(receipt, small);
    expect(problems.some((problem) => problem.includes("receipt declares 480x160"))).toBe(true);
  });
});

// ── g16.005: foreground evidence is read, not trusted ────────────────
//
// The GPUI capture is windowed, so every receipt carries the run's own proof
// that it left the foreground alone. A receipt is read on machines and at
// times far removed from the run that wrote it, so the verifier applies the
// same fail-closed rule the producer does rather than taking its word.

describe("foreground evidence (gpui)", () => {
  const mutateForeground = (
    change: (foreground: Record<string, unknown>) => void,
  ): (() => ButtonCaptureReceipt) => {
    return () =>
      makeReceipt(fixture("button/rest-secondary"), "gpui", (raw) => {
        change((raw.environment as Record<string, unknown>).foreground as Record<string, unknown>);
      });
  };

  test("a proved receipt round-trips", () => {
    const receipt = makeReceipt(fixture("button/rest-secondary"), "gpui");
    const foreground = (receipt.environment as { foreground: { verdict: string } }).foreground;
    expect(foreground.verdict).toBe("proved");
  });

  test("a null baseline is rejected — watching nothing is not proof", () => {
    expect(mutateForeground((f) => { f.baseline = null; })).toThrow(/baseline must name/);
  });

  test("an empty baseline is rejected", () => {
    expect(mutateForeground((f) => { f.baseline = ""; })).toThrow(/baseline must name/);
  });

  test("no observations is rejected", () => {
    expect(mutateForeground((f) => { f.observed = []; })).toThrow(/non-empty array/);
  });

  test("an observation other than the baseline is rejected", () => {
    expect(
      mutateForeground((f) => { f.observed = ["com.example.editor", "com.example.other"]; }),
    ).toThrow(/only the baseline/);
  });

  test("too few samples is rejected", () => {
    for (const samples of [0, 1, MIN_FOREGROUND_SAMPLES - 1]) {
      expect(mutateForeground((f) => { f.samples = samples; })).toThrow(/at least/);
    }
    expect(mutateForeground((f) => { f.samples = MIN_FOREGROUND_SAMPLES; })).not.toThrow();
  });

  test("any verdict but 'proved' is rejected", () => {
    for (const verdict of ["changed", "unprovable", "ok", true, null, undefined]) {
      expect(mutateForeground((f) => { f.verdict = verdict; })).toThrow(/verdict must be 'proved'/);
    }
  });

  test("the old boolean 'changed' shape no longer validates", () => {
    expect(
      mutateForeground((f) => {
        delete f.verdict;
        f.changed = false;
      }),
    ).toThrow();
  });

  test("a receipt with no foreground evidence at all is rejected", () => {
    expect(() =>
      makeReceipt(fixture("button/rest-secondary"), "gpui", (raw) => {
        delete (raw.environment as Record<string, unknown>).foreground;
      }),
    ).toThrow(ReceiptError);
  });
});

describe("capture set completeness and determinism", () => {
  test("the full 54-capture set is accepted", () => {
    const present = inventory.fixtures.flatMap((target) =>
      (["svelte", "react", "gpui"] as const).map((runtime) => ({ fixture: target.name, runtime })),
    );
    expect(captureSetProblems(inventory.fixtures, present)).toEqual([]);
  });

  test("a missing capture is named by fixture and runtime", () => {
    const present = inventory.fixtures
      .flatMap((target) =>
        (["svelte", "react", "gpui"] as const).map((runtime) => ({ fixture: target.name, runtime })),
      )
      .filter((entry) => !(entry.fixture === "button/tone-danger" && entry.runtime === "gpui"));
    const problems = captureSetProblems(inventory.fixtures, present);
    expect(problems).toEqual(["missing capture: button/tone-danger [gpui]"]);
  });

  test("an extra or duplicated capture is named", () => {
    const present = [
      { fixture: "button/rest-secondary", runtime: "svelte" as const },
      { fixture: "button/rest-secondary", runtime: "svelte" as const },
      { fixture: "button/not-a-fixture", runtime: "gpui" as const },
    ];
    const problems = captureSetProblems(inventory.fixtures, present);
    expect(problems.some((problem) => problem.startsWith("duplicated capture"))).toBe(true);
    expect(problems.some((problem) => problem.startsWith("extra capture"))).toBe(true);
  });

  test("differing repeat captures fail closed", () => {
    const id = { fixture: "button/state-loading", runtime: "react" as const };
    expect(repeatMismatchProblem(id, "aaa", "aaa")).toBeNull();
    expect(repeatMismatchProblem(id, "aaa", "bbb")).toMatch(/repeat captures differ/);
  });
});

describe("capture recovery boundary", () => {
  // The batch loop's single recovery decision, driven by the exact error
  // classes the production capture paths throw. A repeat-mismatch or receipt
  // failure is a CaptureIntegrityError and must never enter the retry branch;
  // only a pre-capture transport failure may.
  test("a repeat mismatch cannot enter the recovery branch", () => {
    const repeatMismatch = new CaptureIntegrityError(
      "repeat captures differ for button/state-loading [svelte]: aaa vs bbb",
    );
    expect(isRecoverableTransportError(repeatMismatch)).toBe(false);
  });

  test("a receipt verification failure cannot enter the recovery branch", () => {
    const receiptFailure = new CaptureIntegrityError(
      "web receipt does not verify for button/rest-secondary [react]",
    );
    expect(isRecoverableTransportError(receiptFailure)).toBe(false);
  });

  test("a host param rejection cannot enter the recovery branch", () => {
    expect(
      isRecoverableTransportError(new Error("fixture host rejected the params: missing 'label'")),
    ).toBe(false);
  });

  test("a pre-capture transport failure is the only recoverable class", () => {
    expect(isRecoverableTransportError(new PreviewTransportError("navigation timeout"))).toBe(true);
  });
});

describe("exact policy (svelte ↔ react)", () => {
  const target = fixture("button/rest-secondary");

  test("identical evidence passes every channel", () => {
    const png = solidPng(12, 34, 56);
    const a = evidence(target, "svelte", png, hashOf(png));
    const b = evidence(target, "react", png, hashOf(png));
    const { verdict } = compareExactPair(target, a, b);
    expect(verdict.ok).toBe(true);
    for (const channel of Object.values(verdict.channels)) {
      expect(channel.status).toBe("pass");
    }
  });

  test("a two-logical-pixel root shift fails geometry even with identical pixels", () => {
    const png = solidPng(12, 34, 56);
    const a = evidence(target, "svelte", png, hashOf(png));
    const b: CaptureEvidence = {
      receipt: makeReceipt(target, "react", (raw) => {
        (raw.landmarks as Record<string, { x: number }>).root.x = 18;
        raw.pngSha256 = hashOf(png);
      }),
      png,
    };
    const { verdict } = compareExactPair(target, a, b);
    expect(verdict.ok).toBe(false);
    expect(verdict.channels.geometry.status).toBe("fail");
    expect(verdict.channels.geometry.findings[0].detail).toContain("root.left");
    // Channel independence: pixels still pass; they cannot lift geometry.
    expect(verdict.channels.pixels.status).toBe("pass");
    expect(verdict.channels.roles.status).toBe("pass");
  });

  test("a changed role colour fails roles only", () => {
    const png = solidPng(12, 34, 56);
    const a = evidence(target, "svelte", png, hashOf(png));
    const b: CaptureEvidence = {
      receipt: makeReceipt(target, "react", (raw) => {
        (raw.roles as { fill: { color: Srgb } }).fill.color = [0.5, 0, 0, 1];
        raw.pngSha256 = hashOf(png);
      }),
      png,
    };
    const { verdict } = compareExactPair(target, a, b);
    expect(verdict.channels.roles.status).toBe("fail");
    expect(verdict.channels.roles.findings[0].subject).toBe("fill");
    expect(verdict.channels.pixels.status).toBe("pass");
  });

  test("one differing pixel fails the exact pixel channel", () => {
    const png = solidPng(12, 34, 56);
    const shifted = withWhiteRegion(png, 1);
    const a = evidence(target, "svelte", png, hashOf(png));
    const b = evidence(target, "react", shifted, hashOf(shifted));
    const { verdict, diffPng } = compareExactPair(target, a, b);
    expect(verdict.channels.pixels.status).toBe("fail");
    expect(verdict.channels.pixels.findings[0].detail).toContain("1 pixels differ");
    expect(diffPng).toBeDefined();
  });
});

describe("renderer-aware policy (svelte ↔ gpui)", () => {
  const target = fixture("button/rest-secondary");

  function gpuiEvidence(png: Buffer, mutate?: (raw: Record<string, unknown>) => void): CaptureEvidence {
    return {
      receipt: makeReceipt(target, "gpui", (raw) => {
        raw.pngSha256 = hashOf(png);
        mutate?.(raw);
      }),
      png,
    };
  }

  test("identical evidence passes", () => {
    const png = solidPng(12, 34, 56);
    const web = evidence(target, "svelte", png, hashOf(png));
    const { verdict } = compareRendererAwarePair(target, web, gpuiEvidence(png));
    expect(verdict.ok).toBe(true);
  });

  test("root edges inside 0.5 logical px pass; beyond they fail", () => {
    const png = solidPng(12, 34, 56);
    const web = evidence(target, "svelte", png, hashOf(png));
    const inside = compareRendererAwarePair(
      target,
      web,
      gpuiEvidence(png, (raw) => {
        (raw.landmarks as Record<string, { x: number }>).root.x = 16.5;
      }),
    );
    expect(inside.verdict.channels.geometry.status).toBe("pass");

    const beyond = compareRendererAwarePair(
      target,
      web,
      gpuiEvidence(png, (raw) => {
        (raw.landmarks as Record<string, { x: number }>).root.x = 18;
      }),
    );
    expect(beyond.verdict.channels.geometry.status).toBe("fail");
    expect(beyond.verdict.channels.geometry.findings[0].detail).toContain("root.left");
  });

  test("content width beyond 2 logical px fails; icon centre beyond 1 fails", () => {
    const withIcon = fixture("button/content-leading-icon");
    const png = solidPng(12, 34, 56);
    const web: CaptureEvidence = {
      receipt: makeReceipt(withIcon, "svelte", (raw) => {
        raw.pngSha256 = hashOf(png);
      }),
      png,
    };
    const wideContent = compareRendererAwarePair(target, web, {
      receipt: makeReceipt(withIcon, "gpui", (raw) => {
        raw.pngSha256 = hashOf(png);
        (raw.landmarks as Record<string, { width: number; x: number }>).content.width = 24;
        (raw.landmarks as Record<string, { width: number; x: number }>).content.x = 41;
      }),
      png,
    });
    expect(wideContent.verdict.channels.geometry.status).toBe("fail");
    expect(wideContent.verdict.channels.geometry.findings[0].detail).toContain("content width");

    const offCentreIcon = compareRendererAwarePair(target, web, {
      receipt: makeReceipt(withIcon, "gpui", (raw) => {
        raw.pngSha256 = hashOf(png);
        (raw.landmarks as Record<string, { x: number }>).icon.x = 30;
      }),
      png,
    });
    expect(offCentreIcon.verdict.channels.geometry.status).toBe("fail");
    expect(offCentreIcon.verdict.channels.geometry.findings[0].detail).toContain("icon centre.x");
  });

  test("a role colour beyond 1/255 fails roles", () => {
    const png = solidPng(12, 34, 56);
    const web = evidence(target, "svelte", png, hashOf(png));
    const { verdict } = compareRendererAwarePair(
      target,
      web,
      gpuiEvidence(png, (raw) => {
        (raw.roles as { text: { color: Srgb } }).text.color = [1, 1, 1, 1 - 2 / 255];
      }),
    );
    expect(verdict.channels.roles.status).toBe("fail");
    expect(verdict.channels.roles.findings[0].subject).toBe("text");
    expect(verdict.channels.pixels.status).toBe("pass");
  });

  test("web shadow vs no gpui shadow is a roles failure and classifies as the contract delta", () => {
    const png = solidPng(12, 34, 56);
    const web: CaptureEvidence = {
      receipt: makeReceipt(target, "svelte", (raw) => {
        raw.pngSha256 = hashOf(png);
        (raw.roles as { shadow: { layers: unknown[] } }).shadow.layers = [
          { inset: true, offsetX: 0, offsetY: 1, blur: 0, spread: 0, color: [1, 1, 1, 0.08] },
        ];
      }),
      png,
    };
    const gpui = gpuiEvidence(png); // zero shadow layers
    const { verdict } = compareRendererAwarePair(target, web, gpui);
    expect(verdict.channels.roles.status).toBe("fail");
    const finding = verdict.channels.roles.findings.find((entry) => entry.subject === "shadow");
    expect(finding?.detail).toContain("layer count");
    expect(
      classifyKnownDelta(finding!, {
        webShadowLayers: web.receipt.roles.shadow.layers.length,
        gpuiShadowLayers: gpui.receipt.roles.shadow.layers.length,
      }),
    ).toBe("gpui-omits-box-shadow");
  });

  test("a shadow geometry delta between two non-empty layer sets stays a plain failure", () => {
    const png = solidPng(12, 34, 56);
    const oneLayer = (raw: Record<string, unknown>, blur: number) => {
      (raw.roles as { shadow: { layers: unknown[] } }).shadow.layers = [
        { inset: true, offsetX: 0, offsetY: 1, blur, spread: 0, color: [1, 1, 1, 0.08] },
      ];
    };
    const web: CaptureEvidence = {
      receipt: makeReceipt(target, "svelte", (raw) => {
        raw.pngSha256 = hashOf(png);
        oneLayer(raw, 0);
      }),
      png,
    };
    const gpui = gpuiEvidence(png, (raw) => oneLayer(raw, 2));
    const { verdict } = compareRendererAwarePair(target, web, gpui);
    const finding = verdict.channels.roles.findings.find((entry) => entry.subject === "shadow");
    expect(finding).toBeDefined();
    expect(
      classifyKnownDelta(finding!, {
        webShadowLayers: 1,
        gpuiShadowLayers: 1,
      }),
    ).toBeNull();
  });

  test("a pixel change beyond 3% fails; a smaller one passes", () => {
    const total = WIDTH * HEIGHT;
    const png = solidPng(12, 34, 56);
    const web = evidence(target, "svelte", png, hashOf(png));

    const beyond = withWhiteRegion(png, Math.ceil(total * 0.04));
    const over = compareRendererAwarePair(target, web, gpuiEvidence(beyond));
    expect(over.verdict.channels.pixels.status).toBe("fail");
    expect(over.verdict.channels.pixels.findings[0].detail).toContain("cap");
    expect(over.diffPng).toBeDefined();

    const within = withWhiteRegion(png, Math.floor(total * 0.01));
    const under = compareRendererAwarePair(target, web, gpuiEvidence(within));
    expect(under.verdict.channels.pixels.status).toBe("pass");
  });
});
