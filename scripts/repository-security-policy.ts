const fullRevision = /^[0-9a-f]{40}$/;

/**
 * Git repositories admitted into a Poodle Cargo graph, each pinned to one
 * reviewed full revision.
 *
 * Empty since g16.005: the five entries here existed only to carry the GPUI
 * fork, and Poodle's Rust graphs now resolve entirely from crates.io. An
 * empty allowlist means `validateCargoManifestSources` rejects EVERY Git
 * dependency, which is the fail-closed state this policy wants — matching
 * `deny.toml`'s `allow-git = []` with `unknown-git = "deny"`.
 *
 * Re-admitting a repository is a deliberate two-file change with a reviewed
 * revision, never an incidental one.
 */
export const approvedGitRevisions = new Map<string, string>([]);

/**
 * Crates whose source choice is a PUBLIC contract, not an internal one.
 *
 * `poodle-gpui-node-backend` is a public-intent package: a consumer that
 * depends on crates.io `gpui` must receive the same crate identity through
 * Poodle, or the two are different types and nothing can be passed across the
 * boundary. Published `v0.2.1` resolved `gpui` from a Git fork so an internal
 * capture tool could reach unpublished APIs, and Longhorn's prototypes could
 * not compile against it.
 *
 * These crates are therefore rejected from a Git source EVEN IF that
 * repository is otherwise approved above. Optional tooling does not get to
 * choose the crate identity a public runtime package exposes.
 */
export const registryOnlyCrates = ["gpui", "gpui_platform"];

const registryOnlyPattern = new RegExp(
  `^\\s*(?:${registryOnlyCrates.join("|")})\\s*=`,
);

const registryOnlyLockName = new RegExp(
  `^name\\s*=\\s*"(?:${registryOnlyCrates.join("|")})"$`,
);

function activeManifestLines(source: string): Array<{ line: string; number: number }> {
  return source.split(/\r?\n/).flatMap((line, index) => {
    if (line.trimStart().startsWith("#")) return [];
    return [{ line, number: index + 1 }];
  });
}

export function validateCargoManifestSources(
  filePath: string,
  source: string,
  // The allowlist is a parameter so the revision and mutable-reference rules
  // stay under test now that the production list is empty. Production callers
  // never pass it.
  approved: ReadonlyMap<string, string> = approvedGitRevisions,
): string[] {
  const errors: string[] = [];

  for (const { line, number } of activeManifestLines(source)) {
    for (const match of line.matchAll(/\bgit\s*=\s*"([^"]+)"/g)) {
      const url = match[1]!;

      if (registryOnlyPattern.test(line)) {
        errors.push(
          `${filePath}:${number}: ${line.trim().split(/\s*=/)[0]} must resolve from crates.io, not the Git source ${JSON.stringify(url)} — a public-intent package cannot expose a forked GPUI crate identity to consumers`,
        );
      }

      const revision = line.match(/\brev\s*=\s*"([^"]+)"/)?.[1];
      const expected = approved.get(url);

      if (!expected) {
        errors.push(
          `${filePath}:${number}: Git repository ${JSON.stringify(url)} is not approved`,
        );
      }

      if (!revision) {
        errors.push(
          `${filePath}:${number}: Git dependency ${JSON.stringify(url)} must use an immutable full rev`,
        );
        continue;
      }

      if (!fullRevision.test(revision)) {
        errors.push(
          `${filePath}:${number}: Git dependency ${JSON.stringify(url)} uses non-full revision ${JSON.stringify(revision)}`,
        );
      }

      if (expected && revision !== expected) {
        errors.push(
          `${filePath}:${number}: Git dependency ${JSON.stringify(url)} uses ${revision}; expected reviewed revision ${expected}`,
        );
      }
    }
  }

  return errors;
}

export function validateCargoLockSources(
  filePath: string,
  source: string,
  approved: ReadonlyMap<string, string> = approvedGitRevisions,
): string[] {
  const errors: string[] = [];

  // A lockfile entry is `[[package]] / name = / version = / source =`, so the
  // most recent `name =` line is the package the `source =` line belongs to.
  // Tracking it lets the registry-only rule name the offending crate rather
  // than reporting a bare URL.
  let currentName: string | null = null;

  for (const [index, line] of source.split(/\r?\n/).entries()) {
    const nameMatch = line.match(/^name\s*=\s*"([^"]+)"$/);
    if (nameMatch) {
      currentName = nameMatch[1]!;
      continue;
    }

    const sourceMatch = line.match(/^source\s*=\s*"git\+([^"]+)"$/);
    if (!sourceMatch) continue;

    if (currentName !== null && registryOnlyLockName.test(`name = "${currentName}"`)) {
      errors.push(
        `${filePath}:${index + 1}: ${currentName} resolves from a Git source; Poodle's active package graph must resolve it from crates.io so consumers receive the same crate identity`,
      );
    }

    const rawSource = sourceMatch[1]!;
    const parsed = rawSource.match(/^(.+?)\?rev=([^#]+)#(.+)$/);
    if (!parsed) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile source must use URL?rev=FULL_REV#FULL_REV, got ${JSON.stringify(rawSource)}`,
      );
      continue;
    }

    const [, url, revision, resolvedRevision] = parsed;
    const expected = approved.get(url);
    if (!expected) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile repository ${JSON.stringify(url)} is not approved`,
      );
    }

    if (!fullRevision.test(revision) || !fullRevision.test(resolvedRevision)) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile source for ${JSON.stringify(url)} must contain full immutable revisions`,
      );
    }

    if (revision !== resolvedRevision) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile source for ${JSON.stringify(url)} has mismatched requested and resolved revisions`,
      );
    }

    if (expected && (revision !== expected || resolvedRevision !== expected)) {
      errors.push(
        `${filePath}:${index + 1}: Git lockfile source for ${JSON.stringify(url)} is not the reviewed revision ${expected}`,
      );
    }
  }

  return errors;
}

const secretPatterns = [
  ["private key", /-----BEGIN (?:RSA |EC |OPENSSH |DSA |PGP )?PRIVATE KEY-----/],
  ["AWS access key", /(?:AKIA|ASIA)[A-Z0-9]{16}/],
  ["GitHub token", /gh[pousr]_[A-Za-z0-9]{30,}/],
  // Left word boundary: `mask-plus-...` and `task-backed-...` contain `sk-`
  // as an interior substring, not a token.
  ["OpenAI token", /\bsk-(?:proj-)?[A-Za-z0-9_-]{20,}/],
  ["Slack token", /xox[baprs]-[A-Za-z0-9-]{10,}/],
  ["Stripe live key", /[rs]k_live_[A-Za-z0-9]{16,}/],
  ["JWT", /eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}/],
  ["credential URL", /[A-Za-z][A-Za-z0-9+.-]*:\/\/[^/@\s]+:[^/@\s]+@/],
] as const;

export function secretPatternHits(source: string): string[] {
  const hits: string[] = [];
  for (const [label, pattern] of secretPatterns) {
    if (pattern.test(source)) hits.push(label);
  }
  return hits;
}
