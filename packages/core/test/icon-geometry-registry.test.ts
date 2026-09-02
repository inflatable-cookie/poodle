import { describe, expect, test } from "bun:test";

import manifest from "../src/icons/morph-pairs.json";
import { ICON_GEOMETRY_REGISTRY } from "../src/icons/morph-pairs.generated";

const digestPattern = /^[a-f0-9]{64}$/;

describe("icon geometry registry", () => {
  test("keeps the accepted, candidate, and rejected states explicit", () => {
    expect(ICON_GEOMETRY_REGISTRY.schemaVersion).toBe(1);
    expect(ICON_GEOMETRY_REGISTRY.normalizerVersion).toBe("1.0.0");
    expect(ICON_GEOMETRY_REGISTRY.source).toEqual({
      package: "lucide-static",
      version: "1.31.0",
      manifest: "packages/core/src/icons/default-icons.json",
    });
    expect(ICON_GEOMETRY_REGISTRY.notice).toEqual({
      id: "lucide-static-isc-feather-mit",
      path: "packages/render/assets/icons/LICENSE.txt",
    });
    expect(ICON_GEOMETRY_REGISTRY.registryDigest).toMatch(digestPattern);

    const manifestIds = manifest.pairs.map((pair) => pair.id);
    const registryIds = ICON_GEOMETRY_REGISTRY.pairs.map((pair) => pair.id);
    expect(registryIds as string[]).toEqual(manifestIds);
    expect(new Set(registryIds).size).toBe(registryIds.length);
    expect(new Set(ICON_GEOMETRY_REGISTRY.pairs.map((pair) => pair.status))).toEqual(
      new Set(["accepted", "candidate", "rejected"]),
    );
    expect(ICON_GEOMETRY_REGISTRY.pairs.filter((pair) => pair.status === "accepted")).toHaveLength(5);
    expect(ICON_GEOMETRY_REGISTRY.pairs.filter((pair) => pair.status === "candidate")).toHaveLength(1);
    expect(ICON_GEOMETRY_REGISTRY.pairs.filter((pair) => pair.status === "rejected")).toHaveLength(6);
    const candidate = ICON_GEOMETRY_REGISTRY.pairs.find(
      (pair) => pair.status === "candidate",
    );
    expect(candidate?.id).toBe("circle-to-dot");
    expect(candidate?.geometryLeft).not.toBeNull();
    expect(candidate?.geometryRight).not.toBeNull();
    expect(candidate?.plan).not.toBeNull();

    for (const pair of ICON_GEOMETRY_REGISTRY.pairs) {
      expect(pair.qualityStatus).toBe(pair.status);
      expect(pair.qualityReviewer.length).toBeGreaterThan(0);
      expect(pair.qualityNotes.length).toBeGreaterThan(0);
      expect(pair.sourceDigestLeft).toMatch(digestPattern);
      expect(pair.sourceDigestRight).toMatch(digestPattern);
      expect(pair.assetDigestLeft).toMatch(digestPattern);
      expect(pair.assetDigestRight).toMatch(digestPattern);
      expect(pair.payloadBytes).toBeLessThanOrEqual(16 * 1024);
      if (pair.status !== "rejected") {
        expect(pair.geometryLeft).not.toBeNull();
        expect(pair.geometryRight).not.toBeNull();
        expect(pair.plan).not.toBeNull();
        expect(pair.derivedDigest).toMatch(digestPattern);
        expect(pair.payloadBytes).toBeGreaterThan(0);
      } else {
        expect(pair.rejectionReason).toEqual(expect.any(String));
      }
    }
  });

  test("emits compact, complete endpoint payloads", () => {
    const accepted = ICON_GEOMETRY_REGISTRY.pairs.filter(
      (pair) => pair.status === "accepted",
    );
    for (const pair of accepted) {
      for (const geometry of [pair.geometryLeft, pair.geometryRight]) {
        if (!geometry) throw new Error(`missing geometry for ${pair.id}`);
        expect(geometry.schemaVersion).toBe(1);
        expect(geometry.normalizerVersion).toBe("1.0.0");
        expect(geometry.canonical.contours.length).toBeGreaterThan(0);
        expect(geometry.sampled.contours).toHaveLength(
          geometry.canonical.contours.length,
        );
        expect(geometry.sampled.contours.every((contour) => contour.points.length === 64)).toBe(
          true,
        );
        expect(
          geometry.canonical.contours.every((contour) =>
            contour.segments.every((segment) => segment.length === 5),
          ),
        ).toBe(true);
      }
    }
  });

  test("records alias canonicalization in lineage", () => {
    const home = ICON_GEOMETRY_REGISTRY.pairs.find(
      (pair) => pair.id === "home-to-arrow-left",
    );
    expect(home?.authoredFrom).toBe("home");
    expect(home?.canonicalFrom).toBe("house");
    expect(home?.status).toBe("rejected");
    expect(home?.geometryLeft).toBeNull();
  });
});
