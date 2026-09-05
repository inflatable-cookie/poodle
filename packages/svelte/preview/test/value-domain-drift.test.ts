import { describe, expect, it } from "vitest";

import {
  VALUE_DOMAIN_BASELINE,
  contractValueDomainDrift,
  unresolvedTypeKey,
  valueDomainFindingKey,
  valueDomainRatchet,
  type ValueDomainFinding,
} from "../scripts/contract-value-domain-drift.ts";

describe("value-domain drift ratchet", () => {
  it("matches the g16.107 inventory", () => {
    const ratchet = valueDomainRatchet(contractValueDomainDrift());
    expect(ratchet.fresh).toEqual([]);
    expect(ratchet.stale).toEqual([]);
    expect(ratchet.live).toHaveLength(VALUE_DOMAIN_BASELINE.size);
  });

  it("fails on a planted finding that is not in the baseline", () => {
    const live = contractValueDomainDrift();
    const planted: ValueDomainFinding = {
      slug: "button",
      prop: "plantedTone",
      side: "ts",
      classification: "impl-wider",
      contract: ["default"],
      impl: ["default", "planted"],
      onlyContract: [],
      onlyImpl: ["planted"],
    };
    const ratchet = valueDomainRatchet({
      findings: [...live.findings, planted],
      unresolved: live.unresolved,
    });
    expect(ratchet.fresh).toEqual([valueDomainFindingKey(planted)]);
  });

  it("fails when a baseline key disappears", () => {
    const live = contractValueDomainDrift();
    const dropped = live.findings[0];
    expect(dropped).toBeDefined();
    const ratchet = valueDomainRatchet({
      findings: live.findings.filter((finding) => finding !== dropped),
      unresolved: live.unresolved,
    });
    expect(ratchet.stale).toEqual([valueDomainFindingKey(dropped!)]);
  });

  it("keys unresolved types separately from enumerated drift", () => {
    expect(
      unresolvedTypeKey({
        slug: "dock-region",
        prop: "edge",
        typeName: "DockEdge",
        cell: "`DockEdge`",
      }),
    ).toBe("unresolved|dock-region|edge|DockEdge");
  });
});
