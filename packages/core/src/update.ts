/**
 * Update status — structural mirrors of the update authority's shapes, plus the
 * pure view derivation every renderer consumes.
 *
 * Poodle renders; Longhorn supplies behaviour. Poodle may not depend on
 * Longhorn, so the record types below are structural mirrors (ruling R2 of the
 * HistoryCentre precedent): Poodle never imports a Longhorn type and no manifest
 * gains one. Longhorn exports its generated field maps (`UPDATE_FIELDS`,
 * `UPDATE_VARIANT_FIELDS`) from `@inflatable-cookie/longhorn/update` so the
 * bridge can assert these mirrors against them and fail on drift — do not add
 * or remove fields.
 *
 * `updateStatusView` is the single display-copy authority. The five wrong
 * messages a naive rendering tells — a deferral styled as a failure, a null
 * download fraction drawn as zero, `aheadOfChannel` read as broken, a
 * managed-elsewhere install read as an error, and a signature rejection offered
 * a retry — are all decided here, once, so the Svelte and React renderers cannot
 * drift.
 */

// ── Structural mirrors of the authority's shapes ─────────────────────────

export type Channel = "production" | "beta" | "nightly";

export type OfferReason = "staged" | "belowMinimumVersion" | "userInitiated";

export type InstallManager =
  | "macAppStore"
  | "homebrewCask"
  | "flatpak"
  | "snap"
  | "appImage"
  | "nix"
  | "linuxDistribution";

export type UpdateAvailabilityProjection =
  | { state: "offer"; version: string; reason: OfferReason; notes: string | null }
  | { state: "upToDate" }
  | { state: "aheadOfChannel"; installed: string; channel: string }
  | { state: "withheldByRollout"; version: string }
  | { state: "managedElsewhere"; version: string; manager: InstallManager };

export type UpdateProgressProjection =
  | { state: "idle" }
  | { state: "downloading"; fraction: number | null }
  | { state: "verifying" }
  | { state: "readyToInstall"; version: string }
  | { state: "installing"; version: string };

export type DeferralCause =
  | { cause: "userPostponed" }
  | { cause: "workInFlight"; detail: string }
  | { cause: "installationNotWritable"; detail: string }
  | { cause: "externallyManaged"; manager: InstallManager; command: string | null }
  | { cause: "installFailed"; detail: string };

export type UpdateRejectionCode =
  | "staleAuthority"
  | "noOffer"
  | "unavailable"
  | "unreachable"
  | "signatureRejected"
  | "notWritable"
  | "installFailed";

// ── Controller reads ──────────────────────────────────────────────────────

/** Whether the update centre should surface at all, and how loud. Longhorn's
 *  predicate, never derived here. */
export type UpdatePresence = "hidden" | "quiet" | "attention";

export type UpdateControllerStatus =
  | { kind: "idle" }
  | { kind: "loading" }
  | { kind: "ready" }
  | { kind: "failed"; error: unknown };

export interface UpdateDeferral {
  version: string;
  cause: DeferralCause;
}

export interface UpdateAheadOfChannel {
  installed: string;
  channel: string;
}

// ── Display copy ─────────────────────────────────────────────────────────

export function installManagerLabel(manager: InstallManager): string {
  switch (manager) {
    case "macAppStore":
      return "Mac App Store";
    case "homebrewCask":
      return "Homebrew";
    case "flatpak":
      return "Flatpak";
    case "snap":
      return "Snap";
    case "appImage":
      return "AppImage";
    case "nix":
      return "Nix";
    case "linuxDistribution":
      return "your Linux distribution";
  }
}

export function updateRejectionMessage(code: UpdateRejectionCode): string {
  switch (code) {
    case "staleAuthority":
      return "The update service returned a stale result.";
    case "noOffer":
      return "No update is available right now.";
    case "unavailable":
      return "The update service is unavailable.";
    case "unreachable":
      return "The update service could not be reached.";
    // A signature rejection is not a network problem: the artifact did not come
    // from the signing key. Never phrase it as one, and never offer a retry.
    case "signatureRejected":
      return "The update failed its signature check and was not installed.";
    case "notWritable":
      return "The update could not be written to disk.";
    case "installFailed":
      return "The update could not be installed.";
  }
}

/**
 * The retry a rejection may offer. A signature rejection offers none — the
 * artifact did not come from the signing key, and re-trying will not fix that.
 * Install-side failures retry the install; check-side failures re-check.
 */
export function updateRejectionRetry(code: UpdateRejectionCode): UpdateStatusAction | null {
  switch (code) {
    case "signatureRejected":
      return null;
    case "notWritable":
    case "installFailed":
      return { type: "install" };
    default:
      return { type: "check" };
  }
}

/** Accessible name for the update-centre trigger while a download runs. */
export function updateDownloadLabel(fraction: number | null): string {
  return fraction === null
    ? "Downloading update"
    : `Downloading update, ${Math.round(fraction * 100)}%`;
}

export function updateErrorMessage(error: unknown): string {
  if (typeof error === "string" && error.length > 0) {
    return error;
  }
  if (error instanceof Error && error.message.length > 0) {
    return error.message;
  }
  return "Something went wrong.";
}

// ── View model ────────────────────────────────────────────────────────────

export type UpdateStatusAction = { type: "check" | "install" | "defer" };

export type UpdateStatusNotice = {
  tone: "neutral" | "danger";
  message: string;
  retry: UpdateStatusAction | null;
};

export type UpdateStatusTone = "neutral" | "info" | "attention" | "danger";

export type UpdateStatusViewState =
  | "offer"
  | "upToDate"
  | "aheadOfChannel"
  | "withheldByRollout"
  | "managedElsewhere"
  | "downloading"
  | "verifying"
  | "readyToInstall"
  | "installing"
  | "checking"
  | "failed"
  | "rejected"
  | "idle";

export interface UpdateStatusView {
  /** Driving state, carried for `data-state` and tests. */
  state: UpdateStatusViewState;
  tone: UpdateStatusTone;
  title: string;
  body: string | null;
  /** A download bar descriptor. `fraction: null` is indeterminate, not zero. */
  progress: { fraction: number | null } | null;
  /** Secondary notice (deferral or rejection), already resolved to copy. */
  notice: UpdateStatusNotice | null;
  /** A leading spinner for text-only in-flight states. */
  busy: boolean;
  /** Commands to surface; labels are the renderer's concern. */
  actions: UpdateStatusAction[];
}

export interface UpdateStatusInput {
  status: UpdateControllerStatus;
  availability?: UpdateAvailabilityProjection;
  progress?: UpdateProgressProjection;
  deferral?: UpdateDeferral;
  lastRejection?: UpdateRejectionCode;
  aheadOfChannel?: UpdateAheadOfChannel;
  channel?: Channel;
  installedVersion?: string;
}

export function updateStatusView(input: UpdateStatusInput): UpdateStatusView {
  // Progress supersedes the availability line while an install is running.
  if (input.progress !== undefined && input.progress.state !== "idle") {
    return progressView(input.progress);
  }

  if (input.status.kind === "loading") {
    return {
      state: "checking",
      tone: "neutral",
      title: "Checking for updates…",
      body: null,
      progress: null,
      notice: null,
      busy: true,
      actions: [],
    };
  }

  if (input.status.kind === "failed") {
    return {
      state: "failed",
      tone: "danger",
      title: "Update check failed",
      body: updateErrorMessage(input.status.error),
      progress: null,
      notice: null,
      busy: false,
      actions: [{ type: "check" }],
    };
  }

  if (input.availability !== undefined) {
    return availabilityView(input.availability, input);
  }

  // A rejection with no availability to layer it over is the whole story.
  if (input.lastRejection !== undefined) {
    return rejectionView(input.lastRejection);
  }

  return {
    state: "idle",
    tone: "neutral",
    title: "Updates",
    body: null,
    progress: null,
    notice: null,
    busy: false,
    actions: [{ type: "check" }],
  };
}

function progressView(progress: UpdateProgressProjection): UpdateStatusView {
  switch (progress.state) {
    case "downloading":
      return {
        state: "downloading",
        tone: "info",
        title: "Downloading update…",
        body: null,
        progress: { fraction: progress.fraction },
        notice: null,
        busy: false,
        actions: [],
      };
    case "verifying":
      return {
        state: "verifying",
        tone: "info",
        title: "Verifying update…",
        body: null,
        progress: null,
        notice: null,
        busy: true,
        actions: [],
      };
    case "readyToInstall":
      return {
        state: "readyToInstall",
        tone: "attention",
        title: `Version ${progress.version} is ready to install`,
        body: null,
        progress: null,
        notice: null,
        busy: false,
        actions: [{ type: "install" }],
      };
    case "installing":
      return {
        state: "installing",
        tone: "info",
        title: `Installing ${progress.version}…`,
        body: null,
        progress: null,
        notice: null,
        busy: true,
        actions: [],
      };
    case "idle":
      // Unreachable: the caller guards `state !== "idle"`.
      return {
        state: "idle",
        tone: "neutral",
        title: "Updates",
        body: null,
        progress: null,
        notice: null,
        busy: false,
        actions: [],
      };
  }
}

function availabilityView(
  availability: UpdateAvailabilityProjection,
  input: UpdateStatusInput,
): UpdateStatusView {
  const notice = noticeFor(input);

  switch (availability.state) {
    case "offer":
      return {
        state: "offer",
        tone: "attention",
        title: `Version ${availability.version} is available`,
        body: availability.notes,
        progress: null,
        notice,
        busy: false,
        actions: [{ type: "install" }, { type: "defer" }],
      };
    case "upToDate":
      return {
        state: "upToDate",
        tone: "neutral",
        title: "You're up to date",
        body: upToDateBody(input),
        progress: null,
        notice,
        busy: false,
        actions: [],
      };
    case "aheadOfChannel": {
      // Prefer the dedicated `aheadOfChannel` read; fall back to the
      // projection's own fields. Both versions must sit in the sentence: an
      // install ahead of its channel is correct behaviour, not a broken updater.
      const ahead = input.aheadOfChannel ?? {
        installed: availability.installed,
        channel: availability.channel,
      };
      return {
        state: "aheadOfChannel",
        tone: "neutral",
        title: "You're ahead of your channel",
        body: `Installed ${ahead.installed} · channel ${ahead.channel}`,
        progress: null,
        notice,
        busy: false,
        actions: [],
      };
    }
    case "withheldByRollout":
      return {
        state: "withheldByRollout",
        tone: "neutral",
        title: `Version ${availability.version} exists`,
        body: "Not staged to you yet.",
        progress: null,
        notice,
        busy: false,
        actions: [],
      };
    case "managedElsewhere":
      // Information, not an error: a version exists and a third party owns the
      // install. Say which version and who, and leave the action to them.
      return {
        state: "managedElsewhere",
        tone: "info",
        title: `Version ${availability.version} is available`,
        body: `Managed by ${installManagerLabel(availability.manager)}.`,
        progress: null,
        notice,
        busy: false,
        actions: [],
      };
  }
}

function upToDateBody(input: UpdateStatusInput): string | null {
  const parts: string[] = [];
  if (input.installedVersion) {
    parts.push(`Version ${input.installedVersion}`);
  }
  if (input.channel) {
    parts.push(`${input.channel} channel`);
  }
  return parts.length > 0 ? parts.join(" · ") : null;
}

function rejectionView(code: UpdateRejectionCode): UpdateStatusView {
  return {
    state: "rejected",
    tone: "danger",
    title: "Update unavailable",
    body: null,
    progress: null,
    notice: {
      tone: "danger",
      message: updateRejectionMessage(code),
      retry: updateRejectionRetry(code),
    },
    busy: false,
    actions: [],
  };
}

function noticeFor(input: UpdateStatusInput): UpdateStatusNotice | null {
  // A rejection is a fault; a deferral is a postponed command that succeeded.
  if (input.lastRejection !== undefined) {
    return {
      tone: "danger",
      message: updateRejectionMessage(input.lastRejection),
      retry: updateRejectionRetry(input.lastRejection),
    };
  }
  if (input.deferral !== undefined) {
    return deferralNotice(input.deferral.cause);
  }
  return null;
}

function deferralNotice(cause: DeferralCause): UpdateStatusNotice {
  switch (cause.cause) {
    case "userPostponed":
      return { tone: "neutral", message: "Update postponed.", retry: null };
    case "workInFlight":
      // The command succeeded and was deferred; the detail is the reason. Render
      // it plainly — this is not a failure — and offer to retry.
      return {
        tone: "neutral",
        message: `Install is on hold: ${cause.detail}`,
        retry: { type: "install" },
      };
    case "installationNotWritable":
      return {
        tone: "neutral",
        message: `Install location is not writable: ${cause.detail}`,
        retry: null,
      };
    case "externallyManaged": {
      const label = installManagerLabel(cause.manager);
      const message =
        cause.command !== null && cause.command.length > 0
          ? `${label} manages this install. Upgrade with: ${cause.command}`
          : `${label} manages this install.`;
      return { tone: "neutral", message, retry: null };
    }
    case "installFailed":
      return {
        tone: "neutral",
        message: `Install could not be applied: ${cause.detail}`,
        retry: { type: "install" },
      };
  }
}
