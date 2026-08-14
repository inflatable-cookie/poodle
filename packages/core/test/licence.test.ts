import { describe, expect, test } from "bun:test";

import {
  LICENCE_ACCOUNT_FAILED_MESSAGE,
  LICENCE_FILE_REQUIRED_MESSAGE,
  LICENCE_FILE_UNREADABLE_MESSAGE,
  LICENCE_KEY_TOO_SHORT_MESSAGE,
  LICENCE_KEY_TYPO_MESSAGE,
  LICENCE_KEY_UNREADABLE_MESSAGE,
  LICENCE_MIRROR_FIELDS,
  LICENCE_MIRROR_VARIANT_FIELDS,
  LICENCE_UNNAMED_MACHINE,
  licenceFileContentsBase64,
  licenceKeyProblemMessage,
  licenceMachineLabel,
  licenceOtherSeats,
  licenceSeatRows,
  licenceStatusView,
  resolveLicenceSubmit,
  type LicenceKeyFormat,
  type LicenceKeyProblem,
  type LicenceStatusInput,
  type LicenceSubmitDraft,
  type LicenceUsability,
} from "../src/licence";

const base: LicenceStatusInput = {
  usability: { state: "active" },
  trustBasis: { kind: "offlineSignature" },
  useUntil: null,
  updateUntil: null,
  usable: true,
  attention: "none",
};

const USE_UNTIL_SECONDS = 1_800_000_000;
const UPDATE_UNTIL_SECONDS = 1_900_000_000;
const CHECKED_SECONDS = 1_750_000_000;
const EXPIRED_SECONDS = 1_700_000_000;

function view(overrides: Partial<LicenceStatusInput> = {}) {
  return licenceStatusView({ ...base, ...overrides });
}

/** Longhorn's real predicate: a check failure or a stray symbol is a typo; a
 *  truncation is not. Mirrored here so the tests exercise the shipped pairing
 *  rather than a convenient one. */
const keyFormat: LicenceKeyFormat = {
  parse(input) {
    const stripped = input.replace(/[-\s]/g, "");
    if (/[^A-Za-z0-9]/.test(stripped)) {
      return { ok: false, problem: { kind: "unexpectedSymbol", symbol: "!" } };
    }
    if (stripped.length < 20) {
      return { ok: false, problem: { kind: "tooShort", minimum: 20, actual: stripped.length } };
    }
    if (stripped.endsWith("X")) return { ok: false, problem: { kind: "checkFailed" } };
    return { ok: true, key: stripped.toUpperCase(), grouped: stripped.toUpperCase() };
  },
  isProbablyATypo(problem) {
    return problem.kind === "checkFailed" || problem.kind === "unexpectedSymbol";
  },
};

const VALID_KEY = "abcde-fghij-klmno-pqrst";

function draft(overrides: Partial<LicenceSubmitDraft> = {}): LicenceSubmitDraft {
  return {
    route: "key",
    key: VALID_KEY,
    token: null,
    fileContentsBase64: null,
    label: "",
    ...overrides,
  };
}

describe("mirror field maps", () => {
  // These exist for one reason: a Longhorn-owned adapter test compares them
  // against its generated maps. Renaming a key or dropping a field silently
  // disarms that assertion, so the shape is pinned here too.
  test("mirrors the seat record and nothing else", () => {
    expect(Object.keys(LICENCE_MIRROR_FIELDS)).toEqual(["LicenceSeatProjection"]);
    expect(LICENCE_MIRROR_FIELDS.LicenceSeatProjection).toEqual([
      "machineId",
      "label",
      "thisMachine",
    ]);
  });

  test("mirrors the usability, trust-basis, and credential variants", () => {
    expect(Object.keys(LICENCE_MIRROR_VARIANT_FIELDS).sort()).toEqual([
      "LicenceCredentialProjection",
      "LicenceTrustBasisProjection",
      "LicenceUsabilityProjection",
    ]);
    expect(LICENCE_MIRROR_VARIANT_FIELDS.LicenceUsabilityProjection).toEqual({
      active: ["state"],
      inGrace: ["state", "until"],
      useWindowExpired: ["state", "at"],
      leaseLapsed: ["state", "at"],
      clockRefused: ["state"],
    });
    expect(LICENCE_MIRROR_VARIANT_FIELDS.LicenceTrustBasisProjection).toEqual({
      offlineSignature: ["kind"],
      remoteAssertion: ["kind", "checked"],
    });
    expect(LICENCE_MIRROR_VARIANT_FIELDS.LicenceCredentialProjection).toEqual({
      key: ["kind", "key"],
      accountToken: ["kind", "token"],
      licenceFile: ["kind", "contentsBase64"],
    });
  });

  test("every variant list carries its own discriminant", () => {
    for (const [type, variants] of Object.entries(LICENCE_MIRROR_VARIANT_FIELDS)) {
      const discriminant = type === "LicenceUsabilityProjection" ? "state" : "kind";
      for (const fields of Object.values(variants)) {
        expect(fields).toContain(discriminant);
      }
    }
  });
});

describe("licenceStatusView usability states", () => {
  const states: LicenceUsability[] = [
    { state: "active" },
    { state: "inGrace", until: USE_UNTIL_SECONDS },
    { state: "useWindowExpired", at: EXPIRED_SECONDS },
    { state: "leaseLapsed", at: EXPIRED_SECONDS },
    { state: "clockRefused" },
  ];

  test("every state renders a distinct title and state token", () => {
    const results = states.map((usability) => view({ usability }));
    expect(new Set(results.map((r) => r.state)).size).toBe(5);
    expect(results.map((r) => r.state)).toEqual([
      "active",
      "inGrace",
      "useWindowExpired",
      "leaseLapsed",
      "clockRefused",
    ]);
    expect(results.map((r) => r.title)).toEqual([
      "Licence active",
      "Licence active",
      "Use coverage ended",
      "Licence confirmation required",
      "Check this machine's clock",
    ]);
  });

  test("active reports covered use", () => {
    const result = view();
    expect(result.indicator).toBe("success");
    expect(result.tone).toBe("neutral");
    expect(result.body).toEqual({ text: "Use is currently covered.", timestamp: null });
    expect(result.detail).toBeNull();
  });

  test("inGrace is never warning or danger, in any tone channel", () => {
    for (const attention of ["none", "informational", "actionable"] as const) {
      const result = view({ usability: { state: "inGrace", until: USE_UNTIL_SECONDS }, attention });
      expect(result.tone === "warning" || result.tone === "danger").toBe(false);
      expect(result.indicator === "warning" || result.indicator === "danger").toBe(false);
    }
  });

  test("inGrace carries the continuation deadline as a quiet detail", () => {
    const result = view({ usability: { state: "inGrace", until: USE_UNTIL_SECONDS } });
    expect(result.detail).toEqual({ text: "Use continues until", timestamp: 1_800_000_000_000 });
    expect(result.body.text).not.toMatch(/fail|error|problem/i);
  });

  test("useWindowExpired says nothing about update coverage", () => {
    const result = view({ usability: { state: "useWindowExpired", at: EXPIRED_SECONDS } });
    expect(result.tone).toBe("danger");
    expect(result.body.timestamp).toBe(1_700_000_000_000);
    expect(`${result.title} ${result.body.text}`).not.toMatch(/update/i);
  });

  test("leaseLapsed never calls the licence expired", () => {
    const result = view({ usability: { state: "leaseLapsed", at: EXPIRED_SECONDS } });
    expect(result.tone).toBe("warning");
    expect(result.body.timestamp).toBe(1_700_000_000_000);
    expect(`${result.title} ${result.body.text}`).not.toMatch(/expir|invalid|revok/i);
  });

  test("clockRefused gives the clock remedy and no expiry or purchase copy", () => {
    const result = view({ usability: { state: "clockRefused" } });
    const copy = `${result.title} ${result.body.text}`;
    expect(copy).toMatch(/clock/i);
    expect(copy).toMatch(/set the clock/i);
    expect(copy).not.toMatch(/expir|invalid|revok|buy|purchase|renew|subscri/i);
    expect(result.tone).toBe("warning");
  });
});

describe("licenceStatusView coverage rows", () => {
  const combinations: Array<[number | null, number | null]> = [
    [null, null],
    [USE_UNTIL_SECONDS, null],
    [null, USE_UNTIL_SECONDS],
    [USE_UNTIL_SECONDS, UPDATE_UNTIL_SECONDS],
  ];

  test("use and update always stay two separate rows", () => {
    for (const [useUntil, updateUntil] of combinations) {
      const result = view({ useUntil, updateUntil });
      expect(result.coverage.map((row) => row.id)).toEqual(["use", "update"]);
    }
  });

  test("null windows read as no end date, not as an expiry", () => {
    const result = view({ useUntil: null, updateUntil: null });
    expect(result.coverage[0]).toEqual({
      id: "use",
      term: "Use coverage",
      text: "No end date",
      timestamp: null,
    });
    expect(result.coverage[1]).toEqual({
      id: "update",
      term: "Update coverage",
      text: "No end date",
      timestamp: null,
    });
  });

  test("dated windows carry their timestamp and their own term", () => {
    const result = view({ useUntil: USE_UNTIL_SECONDS, updateUntil: UPDATE_UNTIL_SECONDS });
    expect(result.coverage[0]).toEqual({
      id: "use",
      term: "Use covered until",
      text: null,
      timestamp: 1_800_000_000_000,
    });
    expect(result.coverage[1]).toEqual({
      id: "update",
      term: "Updates covered until",
      text: null,
      timestamp: 1_900_000_000_000,
    });
  });
});

describe("licenceStatusView trust basis and authority reads", () => {
  test("both trust bases render distinctly", () => {
    expect(view({ trustBasis: { kind: "offlineSignature" } }).trust).toEqual({
      term: "Trust basis",
      text: "Verified on this machine",
      timestamp: null,
    });
    expect(view({ trustBasis: { kind: "remoteAssertion", checked: CHECKED_SECONDS } }).trust).toEqual({
      term: "Trust basis",
      text: "Confirmed with the server",
      timestamp: 1_750_000_000_000,
    });
  });

  test("usable is reported, not re-derived", () => {
    // The only difference a flipped `usable` may make. Nothing in the view
    // gates anything: no action list, no disabled flag, no hidden row.
    const on = view({ usable: true });
    const off = view({ usable: false });
    expect(on.usable).toBe(true);
    expect(off.usable).toBe(false);
    expect({ ...on, usable: null }).toEqual({ ...off, usable: null });
  });

  test("attention is echoed and only lifts a calm block to info", () => {
    expect(view({ attention: "none" }).tone).toBe("neutral");
    expect(view({ attention: "informational" }).tone).toBe("info");
    expect(view({ attention: "informational" }).attention).toBe("informational");
    // An actionable state keeps its own tone; attention cannot soften it.
    expect(
      view({ usability: { state: "useWindowExpired", at: 1 }, attention: "informational" }).tone,
    ).toBe("danger");
  });
});

describe("key problem copy", () => {
  test("typo problems never suggest the key is fake", () => {
    const problems: LicenceKeyProblem[] = [
      { kind: "checkFailed" },
      { kind: "unexpectedSymbol", symbol: "!" },
    ];
    for (const problem of problems) {
      const message = licenceKeyProblemMessage(problem, keyFormat);
      expect(message).toBe(LICENCE_KEY_TYPO_MESSAGE);
      expect(message).not.toMatch(/invalid|fake|not recognised|not recognized/i);
    }
  });

  test("too short is distinct from typo copy", () => {
    const message = licenceKeyProblemMessage({ kind: "tooShort", minimum: 20, actual: 4 }, keyFormat);
    expect(message).toBe(LICENCE_KEY_TOO_SHORT_MESSAGE);
    expect(message).not.toBe(LICENCE_KEY_TYPO_MESSAGE);
  });

  test("a host predicate that claims a truncation is a typo still gets the short message", () => {
    const eager: LicenceKeyFormat = { ...keyFormat, isProbablyATypo: () => true };
    expect(licenceKeyProblemMessage({ kind: "tooShort", minimum: 20, actual: 4 }, eager)).toBe(
      LICENCE_KEY_TOO_SHORT_MESSAGE,
    );
  });

  test("a problem the host claims for neither still avoids a verdict", () => {
    const silent: LicenceKeyFormat = { ...keyFormat, isProbablyATypo: () => false };
    const message = licenceKeyProblemMessage({ kind: "checkFailed" }, silent);
    expect(message).toBe(LICENCE_KEY_UNREADABLE_MESSAGE);
    expect(message).not.toMatch(/invalid|fake|not recognised|not recognized/i);
  });
});

describe("resolveLicenceSubmit", () => {
  test("key submission requires the key-mode adapter", () => {
    expect(() => resolveLicenceSubmit(draft(), null)).toThrow(
      "A key-format adapter is required for key activation.",
    );
  });

  test("a valid key emits exactly as typed", () => {
    const raw = "abcde-fghij-klmno-pqrst";
    const result = resolveLicenceSubmit(draft({ key: raw }), keyFormat);
    expect(result).toEqual({ outcome: "emit", credential: { kind: "key", key: raw }, label: null });
  });

  test("the injected parser receives lowercase, dashes, whitespace and I/L/O unchanged", () => {
    const seen: string[] = [];
    const recording: LicenceKeyFormat = {
      parse(input) {
        seen.push(input);
        return keyFormat.parse(input);
      },
      isProbablyATypo: keyFormat.isProbablyATypo,
    };
    const raw = "  abcde-fghij klmno-pqrsI lO  ";
    resolveLicenceSubmit(draft({ key: raw }), recording);
    expect(seen).toEqual([raw]);
  });

  test("a rejected key does not emit and does not round-trip", () => {
    expect(resolveLicenceSubmit(draft({ key: "abc" }), keyFormat)).toEqual({
      outcome: "reject",
      message: LICENCE_KEY_TOO_SHORT_MESSAGE,
    });
    expect(resolveLicenceSubmit(draft({ key: "abcde-fghij-klmno-pqrsX" }), keyFormat)).toEqual({
      outcome: "reject",
      message: LICENCE_KEY_TYPO_MESSAGE,
    });
  });

  test("an acquired account token emits its exact shape", () => {
    const result = resolveLicenceSubmit(
      draft({ route: "accountToken", token: "tok_123" }),
      keyFormat,
    );
    expect(result).toEqual({
      outcome: "emit",
      credential: { kind: "accountToken", token: "tok_123" },
      label: null,
    });
  });

  test("a cancelled account flow is quiet", () => {
    expect(resolveLicenceSubmit(draft({ route: "accountToken", token: null }), keyFormat)).toEqual({
      outcome: "quiet",
    });
  });

  test("a file emits base64 and an empty file route asks for one", () => {
    expect(
      resolveLicenceSubmit(
        draft({ route: "licenceFile", fileContentsBase64: "QUJD" }),
        keyFormat,
      ),
    ).toEqual({
      outcome: "emit",
      credential: { kind: "licenceFile", contentsBase64: "QUJD" },
      label: null,
    });
    expect(resolveLicenceSubmit(draft({ route: "licenceFile" }), keyFormat)).toEqual({
      outcome: "reject",
      message: LICENCE_FILE_REQUIRED_MESSAGE,
    });
  });

  test("every route carries the same optional label", () => {
    const drafts: LicenceSubmitDraft[] = [
      draft({ label: "  Studio Mac  " }),
      draft({ route: "accountToken", token: "tok", label: "  Studio Mac  " }),
      draft({ route: "licenceFile", fileContentsBase64: "QUJD", label: "  Studio Mac  " }),
    ];
    for (const candidate of drafts) {
      const result = resolveLicenceSubmit(candidate, keyFormat);
      expect(result.outcome).toBe("emit");
      expect(result.outcome === "emit" && result.label).toBe("Studio Mac");
    }
  });

  test("a blank label is null, never an empty string", () => {
    expect(licenceMachineLabel("")).toBeNull();
    expect(licenceMachineLabel("   ")).toBeNull();
    expect(licenceMachineLabel(" Studio Mac ")).toBe("Studio Mac");
    const result = resolveLicenceSubmit(draft({ label: "   " }), keyFormat);
    expect(result.outcome === "emit" && result.label).toBeNull();
  });

  test("failure copy never carries credential material", () => {
    for (const message of [LICENCE_FILE_UNREADABLE_MESSAGE, LICENCE_ACCOUNT_FAILED_MESSAGE]) {
      expect(message).not.toMatch(/token|key|base64/i);
    }
  });
});

describe("licenceFileContentsBase64", () => {
  test("strips the data-URL prefix", () => {
    expect(licenceFileContentsBase64("data:application/octet-stream;base64,QUJD")).toBe("QUJD");
    expect(licenceFileContentsBase64("data:text/plain;base64,QUJD")).toBe("QUJD");
  });

  test("leaves bare base64 alone", () => {
    expect(licenceFileContentsBase64("QUJD")).toBe("QUJD");
  });

  test("never leaves a prefix behind", () => {
    expect(licenceFileContentsBase64("data:application/octet-stream;base64,QUJD")).not.toContain(
      "data:",
    );
    expect(licenceFileContentsBase64("data:application/octet-stream;base64,QUJD")).not.toContain(
      "base64,",
    );
  });
});

describe("licenceSeatRows", () => {
  const seats = [
    { machineId: "m-1", label: "Studio Mac", thisMachine: true },
    { machineId: "m-2", label: "Laptop", thisMachine: false },
    { machineId: "m-3", label: null, thisMachine: false },
    { machineId: "m-4", label: "   ", thisMachine: false },
  ];

  test("empty seats derive no rows", () => {
    expect(licenceSeatRows([])).toEqual([]);
    expect(licenceOtherSeats([])).toEqual([]);
  });

  test("labelled rows show their label verbatim", () => {
    const rows = licenceSeatRows(seats);
    expect(rows[0].displayLabel).toBe("Studio Mac");
    expect(rows[1].displayLabel).toBe("Laptop");
    expect(rows[0].named).toBe(true);
  });

  test("unnamed and whitespace-only labels read as Unnamed machine", () => {
    const rows = licenceSeatRows(seats);
    expect(rows[2].displayLabel).toBe(LICENCE_UNNAMED_MACHINE);
    expect(rows[3].displayLabel).toBe(LICENCE_UNNAMED_MACHINE);
    expect(rows[2].named).toBe(false);
    expect(rows[3].named).toBe(false);
  });

  test("this machine is not releasable and every other seat is", () => {
    const rows = licenceSeatRows(seats);
    expect(rows[0].thisMachine).toBe(true);
    expect(rows[0].releasable).toBe(false);
    expect(rows.slice(1).every((row) => row.releasable)).toBe(true);
    expect(licenceOtherSeats(seats).map((seat) => seat.machineId)).toEqual(["m-2", "m-3", "m-4"]);
  });

  test("no derived string exposes a machine ID", () => {
    for (const row of licenceSeatRows(seats)) {
      for (const text of [row.displayLabel, row.releaseName, row.confirmBody]) {
        expect(text).not.toContain(row.machineId);
      }
    }
  });

  test("release names stay honest for unnamed rows", () => {
    const rows = licenceSeatRows(seats);
    expect(rows[1].releaseName).toBe("Release Laptop");
    expect(rows[2].releaseName).toBe("Release unnamed machine");
    expect(licenceSeatRows(seats, null, "Remove")[1].releaseName).toBe("Remove Laptop");
  });

  test("pending affects only the matching row", () => {
    const rows = licenceSeatRows(seats, "m-2");
    expect(rows.map((row) => row.pending)).toEqual([false, true, false, false]);
    expect(licenceSeatRows(seats, null).every((row) => !row.pending)).toBe(true);
  });
});
