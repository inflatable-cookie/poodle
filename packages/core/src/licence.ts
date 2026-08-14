/**
 * Licence — structural mirrors of the licence authority's shapes, plus the pure
 * derivations every renderer consumes.
 *
 * Poodle renders; Longhorn supplies behaviour. Poodle may not depend on
 * Longhorn, so the record types below are structural mirrors (the update.ts
 * precedent): Poodle never imports a Longhorn type and no manifest gains one.
 * `LICENCE_MIRROR_FIELDS` / `LICENCE_MIRROR_VARIANT_FIELDS` are exported so a
 * Longhorn-owned adapter test can compare them against its generated
 * `LICENCE_FIELDS` / `LICENCE_VARIANT_FIELDS` and fail on drift. That downstream
 * assertion does not exist yet — these maps are the half of it Poodle owns.
 *
 * Three components read this file, and the display decisions they must not get
 * wrong are all made here, once:
 *
 *   - `inGrace` is not a fault. A renewal that has not landed yet is the
 *     seller's problem, not the customer's, so it never gets warning or danger
 *     treatment.
 *   - `clockRefused` is not an expiry. The remedy is the machine's clock, and
 *     the copy never mentions expiry, invalidity, revocation, or purchase.
 *   - Use coverage and update coverage are separate windows. Collapsing them
 *     tells a customer with lapsed updates that they cannot use what they own.
 *   - A key that fails its check symbol is a typing mistake, not a fake. The
 *     copy for the two is different on purpose.
 *   - A seat with no label is an unnamed machine, not a machine ID. Raw IDs
 *     never reach rendered or accessible text.
 *
 * Poodle owns no licence policy: `usable`, `attention`, and the seat list are
 * authority reads, and key parsing and account acquisition are injected.
 */

// ── Structural mirrors of the authority's shapes ─────────────────────────

export type LicenceUsability =
  | { state: "active" }
  | { state: "inGrace"; until: number }
  | { state: "useWindowExpired"; at: number }
  | { state: "leaseLapsed"; at: number }
  | { state: "clockRefused" };

export type LicenceTrustBasis =
  | { kind: "offlineSignature" }
  | { kind: "remoteAssertion"; checked: number };

/** Authority emphasis. Reported, never re-derived from the other reads. */
export type LicenceAttention = "none" | "informational" | "actionable";

export interface LicenceSeat {
  /** A random command identifier, not human identity. Never rendered. */
  machineId: string;
  label: string | null;
  thisMachine: boolean;
}

export type LicenceCredential =
  | { kind: "key"; key: string }
  | { kind: "accountToken"; token: string }
  | { kind: "licenceFile"; contentsBase64: string };

// ── Injected host behaviour ──────────────────────────────────────────────

export type LicenceKeyProblem =
  | { kind: "unexpectedSymbol"; symbol: string }
  | { kind: "tooShort"; minimum: number; actual: number }
  | { kind: "checkFailed" };

export type LicenceKeyResult =
  | { ok: true; key: string; grouped: string }
  | { ok: false; problem: LicenceKeyProblem };

/**
 * The host's key parser. Poodle neither imports nor reimplements it: lower
 * case, dashes, whitespace and I/L/O confusions are the parser's job, and
 * pre-normalizing here would decide the customer's input twice.
 */
export interface LicenceKeyFormat {
  parse(input: string): LicenceKeyResult;
  isProbablyATypo(problem: LicenceKeyProblem): boolean;
}

/**
 * The host's account flow. It owns the browser or login journey and returns the
 * resulting token; `null` is a cancellation. Poodle never renders a token field.
 */
export interface LicenceAccountTokenProvider {
  acquire(): Promise<string | null>;
}

// ── Mirror field maps (for the downstream Longhorn assertion) ────────────

/**
 * Record shapes Poodle actually mirrors, keyed by the authority's protocol type
 * name. Only the seat record is here: the status component reads flattened
 * controller values (`usable`, `attention`, `useUntil`, `updateUntil`), and
 * inventing a wire record for them would assert a shape that does not exist.
 */
export const LICENCE_MIRROR_FIELDS: Record<string, readonly string[]> = {
  LicenceSeatProjection: ["machineId", "label", "thisMachine"],
};

/**
 * Tagged-union shapes Poodle mirrors, keyed by protocol type then discriminant.
 * The discriminant itself is in each list, matching the authority's generated
 * map so an exact comparison downstream needs no reshaping.
 */
export const LICENCE_MIRROR_VARIANT_FIELDS: Record<
  string,
  Record<string, readonly string[]>
> = {
  LicenceUsabilityProjection: {
    active: ["state"],
    inGrace: ["state", "until"],
    useWindowExpired: ["state", "at"],
    leaseLapsed: ["state", "at"],
    clockRefused: ["state"],
  },
  LicenceTrustBasisProjection: {
    offlineSignature: ["kind"],
    remoteAssertion: ["kind", "checked"],
  },
  LicenceCredentialProjection: {
    key: ["kind", "key"],
    accountToken: ["kind", "token"],
    licenceFile: ["kind", "contentsBase64"],
  },
};

// ── LicenceStatus view ───────────────────────────────────────────────────

export type LicenceStatusState = LicenceUsability["state"];

/** Copy carrying web milliseconds after the shared authority-seconds conversion. */
export interface LicenceTimedText {
  text: string;
  timestamp: number | null;
}

export interface LicenceCoverageRow {
  id: "use" | "update";
  term: string;
  /** Value text when there is no window; `null` when `timestamp` renders. */
  text: string | null;
  timestamp: number | null;
}

export interface LicenceTrustRow {
  term: string;
  text: string;
  timestamp: number | null;
}

/** Dot tone for the status indicator. */
export type LicenceStatusIndicator = "neutral" | "success" | "warning" | "danger";

/** Block tone for surface/border tokens. */
export type LicenceStatusTone = "neutral" | "info" | "warning" | "danger";

export interface LicenceStatusView {
  state: LicenceStatusState;
  indicator: LicenceStatusIndicator;
  tone: LicenceStatusTone;
  title: string;
  body: LicenceTimedText;
  /** The quiet `inGrace` line; `null` for every other state. */
  detail: LicenceTimedText | null;
  /** Use and update coverage, always both, always in this order. */
  coverage: LicenceCoverageRow[];
  trust: LicenceTrustRow;
  /** Echoed authority reads. Reported, never turned into permissions. */
  attention: LicenceAttention;
  usable: boolean;
}

export interface LicenceStatusInput {
  usability: LicenceUsability;
  trustBasis: LicenceTrustBasis;
  /** Authority timestamp in integer Unix seconds. */
  useUntil: number | null;
  /** Authority timestamp in integer Unix seconds. */
  updateUntil: number | null;
  usable: boolean;
  attention: LicenceAttention;
}

/** Convert one authority timestamp at the shared view boundary. */
export function licenceTimestampMilliseconds(timestampSeconds: number): number {
  return timestampSeconds * 1_000;
}

export function licenceStatusView(input: LicenceStatusInput): LicenceStatusView {
  const { title, body, detail, indicator } = usabilityCopy(input.usability);
  return {
    state: input.usability.state,
    indicator,
    tone: statusTone(input.usability.state, input.attention),
    title,
    body,
    detail,
    coverage: [coverageRow("use", input.useUntil), coverageRow("update", input.updateUntil)],
    trust: trustRow(input.trustBasis),
    attention: input.attention,
    usable: input.usable,
  };
}

function usabilityCopy(usability: LicenceUsability): {
  title: string;
  body: LicenceTimedText;
  detail: LicenceTimedText | null;
  indicator: LicenceStatusIndicator;
} {
  switch (usability.state) {
    case "active":
      return {
        title: "Licence active",
        body: { text: "Use is currently covered.", timestamp: null },
        detail: null,
        indicator: "success",
      };
    // The same title as `active`, because the same thing is true: use
    // continues. A pending renewal is the seller's outstanding work, and
    // saying so loudly would ask the customer to fix it.
    case "inGrace":
      return {
        title: "Licence active",
        body: { text: "A renewal is pending. Use continues in the meantime.", timestamp: null },
        detail: {
          text: "Use continues until",
          timestamp: licenceTimestampMilliseconds(usability.until),
        },
        indicator: "neutral",
      };
    // Use coverage only. Update coverage is a separate window with its own
    // row, and mentioning it here is how "you can still use it" turns into
    // "you have lost it".
    case "useWindowExpired":
      return {
        title: "Use coverage ended",
        body: {
          text: "This licence stopped covering use",
          timestamp: licenceTimestampMilliseconds(usability.at),
        },
        detail: null,
        indicator: "danger",
      };
    // Lapsed, not expired. The licence is intact; the confirmation is stale.
    case "leaseLapsed":
      return {
        title: "Licence confirmation required",
        body: {
          text: "The lease lapsed",
          timestamp: licenceTimestampMilliseconds(usability.at),
        },
        detail: null,
        indicator: "warning",
      };
    // A clock that moved backwards is a machine problem with a machine
    // remedy. Never expiry, invalidity, revocation, or purchase.
    case "clockRefused":
      return {
        title: "Check this machine's clock",
        body: {
          text: "This machine's clock moved backwards. Set the clock to the correct time, then check again.",
          timestamp: null,
        },
        detail: null,
        indicator: "warning",
      };
  }
}

/**
 * Block tone. The state decides it; `informational` attention may only lift a
 * calm block to info. It can never make `inGrace` a warning — that is the whole
 * point of the state being neutral.
 */
function statusTone(state: LicenceStatusState, attention: LicenceAttention): LicenceStatusTone {
  switch (state) {
    case "useWindowExpired":
      return "danger";
    case "leaseLapsed":
    case "clockRefused":
      return "warning";
    default:
      return attention === "informational" ? "info" : "neutral";
  }
}

function coverageRow(id: "use" | "update", until: number | null): LicenceCoverageRow {
  if (id === "use") {
    return until === null
      ? { id, term: "Use coverage", text: "No end date", timestamp: null }
      : {
          id,
          term: "Use covered until",
          text: null,
          timestamp: licenceTimestampMilliseconds(until),
        };
  }
  return until === null
    ? { id, term: "Update coverage", text: "No end date", timestamp: null }
    : {
        id,
        term: "Updates covered until",
        text: null,
        timestamp: licenceTimestampMilliseconds(until),
      };
}

function trustRow(basis: LicenceTrustBasis): LicenceTrustRow {
  return basis.kind === "offlineSignature"
    ? { term: "Trust basis", text: "Verified on this machine", timestamp: null }
    : {
        term: "Trust basis",
        text: "Confirmed with the server",
        timestamp: licenceTimestampMilliseconds(basis.checked),
      };
}

// ── LicenceActivation resolution ─────────────────────────────────────────

export type LicenceActivationRoute = "key" | "accountToken" | "licenceFile";

export interface LicenceRouteDescriptor {
  value: LicenceActivationRoute;
  label: string;
}

/**
 * The three routes, in one order, as equal peers. Both renderers build their
 * tab strip from this so neither can quietly promote one route or bury another
 * behind an "advanced" disclosure.
 */
export const LICENCE_ROUTES: readonly LicenceRouteDescriptor[] = [
  { value: "key", label: "Key" },
  { value: "accountToken", label: "Account" },
  { value: "licenceFile", label: "Licence file" },
];

/** A mistyped key. Never `invalid`, `fake`, or `not recognised`. */
export const LICENCE_KEY_TYPO_MESSAGE = "Check the key for a typing mistake.";
/** Distinct from the typo message: a truncation is not a transposition. */
export const LICENCE_KEY_TOO_SHORT_MESSAGE = "This key is too short.";
/** A host predicate that claims neither. Still not a verdict on the key. */
export const LICENCE_KEY_UNREADABLE_MESSAGE = "This key could not be read.";
export const LICENCE_FILE_REQUIRED_MESSAGE = "Choose a licence file to continue.";
export const LICENCE_FILE_UNREADABLE_MESSAGE = "That file could not be read.";
/** Generic by design: a failed account flow never leaks token material. */
export const LICENCE_ACCOUNT_FAILED_MESSAGE = "Account activation could not be completed.";

/**
 * Local copy for a rejected key.
 *
 * `tooShort` is checked before the typo predicate. The contract demands the two
 * messages stay distinct, and a host predicate that answered `true` for a
 * truncation would otherwise collapse them — Longhorn's never does, so the
 * order costs nothing and closes the case where an unusual host is wired up.
 */
export function licenceKeyProblemMessage(
  problem: LicenceKeyProblem,
  keyFormat: LicenceKeyFormat,
): string {
  if (problem.kind === "tooShort") return LICENCE_KEY_TOO_SHORT_MESSAGE;
  if (keyFormat.isProbablyATypo(problem)) return LICENCE_KEY_TYPO_MESSAGE;
  return LICENCE_KEY_UNREADABLE_MESSAGE;
}

/** The optional machine label: trimmed, and empty means absent. */
export function licenceMachineLabel(raw: string): string | null {
  const trimmed = raw.trim();
  return trimmed.length > 0 ? trimmed : null;
}

/**
 * Base64 from whatever the browser handed back. `FileReader.readAsDataURL`
 * returns `data:<mime>;base64,<payload>`; the authority wants the payload, so
 * the prefix is stripped exactly once here rather than in two renderers.
 */
export function licenceFileContentsBase64(read: string): string {
  if (!read.startsWith("data:")) return read;
  const comma = read.indexOf(",");
  return comma < 0 ? read : read.slice(comma + 1);
}

export interface LicenceSubmitDraft {
  route: LicenceActivationRoute;
  /** The key exactly as typed. Never pre-normalized. */
  key: string;
  /** A token the injected provider returned; `null` when it cancelled. */
  token: string | null;
  /** File payload, already free of any data-URL prefix. */
  fileContentsBase64: string | null;
  /** The machine-label field's raw text. */
  label: string;
}

export type LicenceSubmitResolution =
  /** Emit the credential; the host runs the activation command. */
  | { outcome: "emit"; credential: LicenceCredential; label: string | null }
  /** Show a local message and emit nothing. */
  | { outcome: "reject"; message: string }
  /** Say nothing and emit nothing — a cancelled account flow. */
  | { outcome: "quiet" };

/**
 * The single submit decision, shared by both renderers.
 *
 * The accepted key emitted is the raw one. `parse` having said yes is the whole
 * of Poodle's business with it: re-emitting the parser's normalized form would
 * be Poodle deciding the credential's spelling, which is the authority's job.
 */
export function resolveLicenceSubmit(
  draft: LicenceSubmitDraft,
  keyFormat: LicenceKeyFormat,
): LicenceSubmitResolution {
  const label = licenceMachineLabel(draft.label);

  switch (draft.route) {
    case "key": {
      const result = keyFormat.parse(draft.key);
      if (!result.ok) {
        return { outcome: "reject", message: licenceKeyProblemMessage(result.problem, keyFormat) };
      }
      return { outcome: "emit", credential: { kind: "key", key: draft.key }, label };
    }
    case "accountToken":
      // A cancellation is a decision, not a failure. Nothing is said about it.
      if (draft.token === null) return { outcome: "quiet" };
      return { outcome: "emit", credential: { kind: "accountToken", token: draft.token }, label };
    case "licenceFile":
      if (draft.fileContentsBase64 === null) {
        return { outcome: "reject", message: LICENCE_FILE_REQUIRED_MESSAGE };
      }
      return {
        outcome: "emit",
        credential: { kind: "licenceFile", contentsBase64: draft.fileContentsBase64 },
        label,
      };
  }
}

// ── LicenceSeats rows ────────────────────────────────────────────────────

export const LICENCE_UNNAMED_MACHINE = "Unnamed machine";
export const LICENCE_THIS_MACHINE = "This machine";
export const LICENCE_RELEASE_CONFIRM_TITLE = "Release this seat?";

export interface LicenceSeatRow {
  /** Carried for the callback and list keys only. Never rendered. */
  machineId: string;
  /** The supplied label, or `Unnamed machine`. Never an ID. */
  displayLabel: string;
  named: boolean;
  thisMachine: boolean;
  /** Only other machines are releasable; this one is where you are. */
  releasable: boolean;
  pending: boolean;
  /** Accessible name for the row's release control. */
  releaseName: string;
  /** Confirmation body — the same honest name the row shows. */
  confirmBody: string;
}

/** Seats other than this machine — the only rows that can be released. */
export function licenceOtherSeats(seats: readonly LicenceSeat[]): LicenceSeat[] {
  return seats.filter((seat) => !seat.thisMachine);
}

/**
 * One row per seat.
 *
 * Several unnamed rows look alike, and they stay that way. Shortening a machine
 * ID, or inventing a hostname to tell them apart, would put identity Poodle was
 * never given in front of the customer.
 */
export function licenceSeatRows(
  seats: readonly LicenceSeat[],
  pendingMachineId: string | null = null,
  releaseLabel = "Release",
): LicenceSeatRow[] {
  return seats.map((seat) => {
    const named = seat.label !== null && seat.label.trim().length > 0;
    const displayLabel = named ? (seat.label as string) : LICENCE_UNNAMED_MACHINE;
    return {
      machineId: seat.machineId,
      displayLabel,
      named,
      thisMachine: seat.thisMachine,
      releasable: !seat.thisMachine,
      pending: pendingMachineId !== null && pendingMachineId === seat.machineId,
      releaseName: named
        ? `${releaseLabel} ${displayLabel}`
        : `${releaseLabel} unnamed machine`,
      confirmBody: displayLabel,
    };
  });
}
