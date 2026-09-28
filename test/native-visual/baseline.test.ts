import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { PNG } from "pngjs";

import { applyBaseline } from "./baseline";

function pngBuffer(red: number, green: number, blue: number): Buffer {
  const image = new PNG({ width: 2, height: 2 });
  for (let i = 0; i < image.data.length; i += 4) {
    image.data[i] = red;
    image.data[i + 1] = green;
    image.data[i + 2] = blue;
    image.data[i + 3] = 255;
  }
  return PNG.sync.write(image);
}

describe("native visual baseline compare", () => {
  test("a brand-new specimen with no baseline fails instead of reporting no diff", () => {
    const root = mkdtempSync(join(tmpdir(), "poodle-visual-baseline-"));
    const baselinePath = join(root, "planted.png");
    const shot = pngBuffer(255, 0, 0);
    const result = applyBaseline({
      file: "planted.png",
      shot,
      baselinePath,
      update: false,
    });
    expect(result.status).toBe("missing");
    expect(result).toMatchObject({ detail: expect.stringContaining("no baseline — planted.png") });
    expect(existsSync(baselinePath)).toBe(false);
  });

  test("an explicit update writes a missing baseline", () => {
    const root = mkdtempSync(join(tmpdir(), "poodle-visual-baseline-"));
    const baselinePath = join(root, "planted.png");
    const shot = pngBuffer(0, 255, 0);
    const result = applyBaseline({
      file: "planted.png",
      shot,
      baselinePath,
      update: true,
    });
    expect(result).toEqual({ status: "written", first: true });
    expect(readFileSync(baselinePath).equals(shot)).toBe(true);
  });

  test("a matching baseline passes", () => {
    const root = mkdtempSync(join(tmpdir(), "poodle-visual-baseline-"));
    mkdirSync(root, { recursive: true });
    const baselinePath = join(root, "planted.png");
    const shot = pngBuffer(0, 0, 255);
    writeFileSync(baselinePath, shot);
    expect(
      applyBaseline({
        file: "planted.png",
        shot,
        baselinePath,
        update: false,
      }),
    ).toEqual({ status: "ok" });
  });
});
